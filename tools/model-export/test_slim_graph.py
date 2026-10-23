"""Tests for slim_graph.py on a tiny synthetic graph (no model needed)."""
import sys
from pathlib import Path

import numpy as np
import onnx
import onnxruntime as ort
from onnx import TensorProto as T
from onnx import helper as h
from onnx import numpy_helper as nh

sys.path.insert(0, str(Path(__file__).parent))
import slim_graph as sg  # noqa: E402

DIM = 8


def _model() -> onnx.ModelProto:
    f = lambda name, v: nh.from_array(np.array(v, np.float32), name)
    w = np.random.default_rng(0).uniform(0.5, 1.5, DIM).astype(np.float32)
    nodes = [
        # RMSNorm with a scale
        h.make_node("Pow", ["x", "two"], ["p"], name="p"),
        h.make_node("ReduceMean", ["p", "axes"], ["m"], name="m", keepdims=1),
        h.make_node("Add", ["m", "eps"], ["a"], name="a"),
        h.make_node("Pow", ["a", "mhalf"], ["r"], name="r"),
        h.make_node("Mul", ["x", "r"], ["n"], name="n"),
        h.make_node("Mul", ["n", "w"], ["y"], name="y"),
        # tanh GELU on y
        h.make_node("Mul", ["y", "y"], ["sq"], name="sq"),
        h.make_node("Mul", ["y", "sq"], ["cube"], name="cube"),
        h.make_node("Mul", ["c1", "cube"], ["k1"], name="k1"),
        h.make_node("Add", ["y", "k1"], ["s"], name="s"),
        h.make_node("Mul", ["c2", "s"], ["k2"], name="k2"),
        h.make_node("Tanh", ["k2"], ["t"], name="t"),
        h.make_node("Add", ["one", "t"], ["u"], name="u"),
        h.make_node("Mul", ["y", "u"], ["v"], name="v"),
        h.make_node("Mul", ["half", "v"], ["out"], name="out"),
    ]
    inits = [f("two", 2.0), f("eps", 1e-6), f("mhalf", -0.5), f("c1", 0.044715), f("c2", 0.7978845608),
             f("one", 1.0), f("half", 0.5), nh.from_array(w, "w"), nh.from_array(np.array([-1], np.int64), "axes")]
    graph = h.make_graph(nodes, "g", [h.make_tensor_value_info("x", T.FLOAT, ["b", "s", DIM])],
                         [h.make_tensor_value_info("out", T.FLOAT, ["b", "s", DIM])], inits)
    return h.make_model(graph, opset_imports=[h.make_opsetid("", 21)], ir_version=10)


def _run(model: onnx.ModelProto, x: np.ndarray) -> np.ndarray:
    return ort.InferenceSession(model.SerializeToString(), providers=["CPUExecutionProvider"]).run(None, {"x": x})[0]


def test_fusion_replaces_both_patterns_and_keeps_the_result():
    model = _model()
    x = np.random.default_rng(1).normal(size=(2, 5, DIM)).astype(np.float32) * 3
    expected = _run(model, x)
    stats = sg.fuse(model)
    assert stats == {"rmsnorm": 1, "gelu": 1}
    assert [n.op_type for n in model.graph.node] == ["SimplifiedLayerNormalization", "FastGelu"]
    assert np.abs(_run(model, x) - expected).max() < 1e-4


def test_unscaled_rmsnorm_gets_a_vector_of_ones():
    model = _model()
    # Drop the scale multiplication and the GELU: the norm output feeds the graph output.
    nodes = [n for n in model.graph.node if n.name in ("p", "m", "a", "r", "n")]
    nodes[-1].output[0] = "out"
    graph = h.make_graph(nodes, "g", list(model.graph.input), list(model.graph.output), list(model.graph.initializer))
    plain = h.make_model(graph, opset_imports=list(model.opset_import), ir_version=10)
    x = np.random.default_rng(2).normal(size=(1, 3, DIM)).astype(np.float32)
    expected = _run(plain, x)
    plain = onnx.load_from_string(plain.SerializeToString())
    plain.graph.value_info.add().CopyFrom(h.make_tensor_value_info("x", T.FLOAT, ["b", "s", DIM]))
    assert sg.fuse(plain) == {"rmsnorm": 1}
    assert np.abs(_run(plain, x) - expected).max() < 1e-5


def test_unrelated_graphs_are_left_alone():
    graph = h.make_graph([h.make_node("Relu", ["x"], ["out"], name="r")], "g",
                         [h.make_tensor_value_info("x", T.FLOAT, [1])], [h.make_tensor_value_info("out", T.FLOAT, [1])])
    model = h.make_model(graph, opset_imports=[h.make_opsetid("", 21)], ir_version=10)
    assert sg.fuse(model) == {}
    assert [n.op_type for n in model.graph.node] == ["Relu"]
