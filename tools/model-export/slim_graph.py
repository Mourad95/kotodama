"""Shrink the Q4 ONNX graph so ONNX Runtime opens it quickly on a phone (ADR-0013).

ONNX Runtime plans memory for every node when a session is created, and that step grows faster
than linearly with the node count: the exported graph (8000 nodes) took 7 s to open on a laptop and
9 s on a mid-range phone. No session option avoids it, so the graph itself is reduced:

1. ONNX Runtime folds constants offline (basic level only: nothing hardware specific),
2. every RMSNorm (Pow, ReduceMean, Add, Pow, Mul, Mul) becomes one `SimplifiedLayerNormalization`,
3. every tanh GELU (nine nodes) becomes one `FastGelu`,
4. the weights go to an external data file, which ONNX Runtime memory-maps instead of copying.

The graph computes the same function: the result is checked against the input graph
(cosine similarity of the embeddings) and the script fails below `--min-cosine`.

    python slim_graph.py --src pruned/gemma2-m3/onnx/model_q4.onnx --dst pruned/gemma2-m3/slim/model_q4.onnx
"""
from __future__ import annotations

import argparse
import collections
import tempfile
import time
from pathlib import Path

import numpy as np
import onnx
import onnxruntime as ort
from onnx import helper as h
from onnx import numpy_helper as nh

GELU_C1, GELU_C2 = 0.044715, 0.7978845608


def _scalar(name, inits, producers):
    """Value of a constant input (initializer or Constant node), or None."""
    if name in inits:
        return nh.to_array(inits[name])
    node = producers.get(name)
    if node is not None and node.op_type == "Constant":
        return nh.to_array(node.attribute[0].t)
    return None


def _close(value, target, tol) -> bool:
    return value is not None and value.size == 1 and abs(float(value) - target) <= tol


def fuse(model: onnx.ModelProto) -> dict[str, int]:
    """Replace RMSNorm and tanh-GELU sub-graphs in place. Returns how many of each were fused."""
    g = model.graph
    producers = {o: n for n in g.node for o in n.output}
    consumers = collections.defaultdict(list)
    for n in g.node:
        for i in n.input:
            consumers[i].append(n)
    inits = {i.name: i for i in g.initializer}
    inferred = onnx.shape_inference.infer_shapes(model, data_prop=True)
    last_dim = {}
    for vi in list(inferred.graph.value_info) + list(inferred.graph.input):
        dims = vi.type.tensor_type.shape.dim
        if len(dims) and dims[-1].HasField("dim_value"):
            last_dim[vi.name] = dims[-1].dim_value

    def only(name):
        """The single consumer of `name`, or None."""
        return consumers[name][0] if len(consumers[name]) == 1 else None

    def const_inputs(node):
        return [v for v in (_scalar(i, inits, producers) for i in node.input) if v is not None]

    removed, replacement, ones, stats = set(), {}, [], collections.Counter()

    for n in g.node:
        chain = _rmsnorm(n, only, const_inputs, inits, producers, last_dim)
        if chain:
            nodes, x, scale, eps = chain
            if any(c.name in removed for c in nodes):
                continue
            if scale is None:
                scale = f"ones_{len(ones)}"
                ones.append((last_dim[x], scale))
            removed.update(c.name for c in nodes)
            replacement[nodes[-1].name] = h.make_node(
                "SimplifiedLayerNormalization", [x, scale], [nodes[-1].output[0]],
                name=f"rmsnorm_{stats['rmsnorm']}", axis=-1, epsilon=eps, stash_type=1)
            stats["rmsnorm"] += 1
            continue
        chain = _gelu(n, only, const_inputs)
        if chain:
            nodes, x = chain
            if any(c.name in removed for c in nodes):
                continue
            removed.update(c.name for c in nodes)
            replacement[nodes[-1].name] = h.make_node(
                "FastGelu", [x], [nodes[-1].output[0]], name=f"gelu_{stats['gelu']}", domain="com.microsoft")
            stats["gelu"] += 1

    kept = [replacement.get(n.name, n) for n in g.node if n.name in replacement or n.name not in removed]
    del g.node[:]
    g.node.extend(kept)
    for dim, name in ones:
        g.initializer.append(nh.from_array(np.ones(dim, np.float32), name))
    if stats["gelu"] and not any(o.domain == "com.microsoft" for o in model.opset_import):
        model.opset_import.append(h.make_opsetid("com.microsoft", 1))
    return dict(stats)


def _rmsnorm(pow2, only, const_inputs, inits, producers, last_dim):
    """x -> Pow(2) -> ReduceMean -> Add(eps) -> Pow(-0.5) -> Mul(x) [-> Mul(scale)]. None if no match."""
    if pow2.op_type != "Pow" or not _close(_scalar(pow2.input[1], inits, producers), 2.0, 0):
        return None
    x = pow2.input[0]
    mean = only(pow2.output[0])
    if mean is None or mean.op_type != "ReduceMean":
        return None
    add = only(mean.output[0])
    if add is None or add.op_type != "Add" or len(const_inputs(add)) != 1:
        return None
    rsqrt = only(add.output[0])
    if rsqrt is None or rsqrt.op_type != "Pow" or not _close(_scalar(rsqrt.input[1], inits, producers), -0.5, 0):
        return None
    norm = only(rsqrt.output[0])
    if norm is None or norm.op_type != "Mul" or x not in norm.input:
        return None
    nodes, scale = [pow2, mean, add, rsqrt, norm], None
    weighted = only(norm.output[0])
    if weighted is not None and weighted.op_type == "Mul":
        other = next(i for i in weighted.input if i != norm.output[0])
        if _scalar(other, inits, producers) is not None:
            nodes.append(weighted)
            scale = other
    if scale is None and x not in last_dim:
        return None
    return nodes, x, scale, float(const_inputs(add)[0])


def _gelu(sq, only, const_inputs):
    """g*g -> *g -> *0.044715 -> +g -> *0.79788 -> Tanh -> +1 -> *g -> *0.5. None if no match."""
    if sq.op_type != "Mul" or sq.input[0] != sq.input[1]:
        return None
    x = sq.input[0]
    cube = only(sq.output[0])
    if cube is None or cube.op_type != "Mul" or set(cube.input) != {x, sq.output[0]}:
        return None
    k1 = only(cube.output[0])
    if k1 is None or k1.op_type != "Mul" or not any(_close(c, GELU_C1, 1e-6) for c in const_inputs(k1)):
        return None
    add = only(k1.output[0])
    if add is None or add.op_type != "Add" or x not in add.input:
        return None
    k2 = only(add.output[0])
    if k2 is None or k2.op_type != "Mul" or not any(_close(c, GELU_C2, 1e-5) for c in const_inputs(k2)):
        return None
    tanh = only(k2.output[0])
    if tanh is None or tanh.op_type != "Tanh":
        return None
    plus1 = only(tanh.output[0])
    if plus1 is None or plus1.op_type != "Add" or not any(_close(c, 1.0, 0) for c in const_inputs(plus1)):
        return None
    gate = only(plus1.output[0])
    if gate is None or gate.op_type != "Mul" or x not in gate.input:
        return None
    half = only(gate.output[0])
    if half is None or half.op_type != "Mul" or not any(_close(c, 0.5, 0) for c in const_inputs(half)):
        return None
    return [sq, cube, k1, add, k2, tanh, plus1, gate, half], x


def _session(path: Path) -> tuple[ort.InferenceSession, float]:
    options = ort.SessionOptions()
    options.graph_optimization_level = ort.GraphOptimizationLevel.ORT_DISABLE_ALL
    start = time.perf_counter()
    session = ort.InferenceSession(str(path), options, providers=["CPUExecutionProvider"])
    return session, time.perf_counter() - start


def min_cosine(reference: Path, candidate: Path) -> tuple[float, float, float]:
    """Smallest cosine similarity of the two graphs' embeddings, and both load times."""
    ref, t_ref = _session(reference)
    new, t_new = _session(candidate)
    worst = 1.0
    for length in (3, 10, 40, 130, 400):
        for batch in (1, 2):
            ids = np.array([[2, *range(50, 50 + length)]] * batch, dtype=np.int64)
            feed = {"input_ids": ids, "attention_mask": np.ones_like(ids)}
            a, b = ref.run(None, feed)[0], new.run(None, feed)[0]
            worst = min(worst, float((a * b).sum(axis=1).min()))
    return worst, t_ref, t_new


def slim(src: Path, dst: Path, min_cos: float = 0.995) -> dict:
    """Write the slim graph (and its `<dst name>_data` file) and verify it against `src`."""
    dst.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory() as tmp:
        folded = Path(tmp) / "folded.onnx"
        options = ort.SessionOptions()
        options.graph_optimization_level = ort.GraphOptimizationLevel.ORT_ENABLE_BASIC
        options.optimized_model_filepath = str(folded)
        ort.InferenceSession(str(src), options, providers=["CPUExecutionProvider"])
        model = onnx.load(str(folded))
    nodes_before = len(model.graph.node)
    stats = fuse(model)
    data = dst.name + "_data"
    for stale in (dst, dst.with_name(data)):
        stale.unlink(missing_ok=True)
    onnx.save_model(model, str(dst), save_as_external_data=True, all_tensors_to_one_file=True,
                    location=data, size_threshold=1024)
    cosine, t_src, t_dst = min_cosine(src, dst)
    report = {"fused": stats, "nodes": [nodes_before, len(model.graph.node)], "min_cosine": round(cosine, 6),
              "open_s": [round(t_src, 2), round(t_dst, 2)]}
    if cosine < min_cos:
        raise SystemExit(f"slim graph differs from the source (min cosine {cosine:.5f} < {min_cos}): {report}")
    return report


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--src", type=Path, required=True, help="Q4 graph (model_q4.onnx)")
    ap.add_argument("--dst", type=Path, required=True, help="slim graph; weights go to <dst>_data")
    ap.add_argument("--min-cosine", type=float, default=0.995)
    args = ap.parse_args()
    print(slim(args.src, args.dst, args.min_cosine))


if __name__ == "__main__":
    main()
