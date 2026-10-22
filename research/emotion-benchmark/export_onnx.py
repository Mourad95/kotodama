"""Export a sentence-transformers model to one self-contained ONNX graph, then quantize it to 4 bits.

Unlike quantize_onnx.py (optimum, transformers < 5), this script traces the full sentence-embedding
pipeline (transformer + pooling + normalize) with torch.onnx, so it also works for models that need
transformers >= 5 (EmbeddingGemma 2). Output contract: inputs `input_ids`, `attention_mask`
(int64, batch x seq), output `sentence_embedding` — the graph the Rust engine loads.

Usage:
    python export_onnx.py --model pruned/gemma2-m3

Writes <model>/onnx/model.onnx (FP32), <model>/onnx/model_q4.onnx (4-bit blocks: MatMul and the
embedding Gather) and <model>/onnx_report.json (sizes, latency, cosine vs PyTorch). Evaluate with:
    python bench.py --only none --local name=<model>@onnx/model_q4.onnx
"""
from __future__ import annotations

import argparse
import csv
import json
import statistics
import time
from pathlib import Path

import numpy as np

from ort_embedder import OrtEmbedder
from prune_vocab import load_st

HERE = Path(__file__).parent
OPSET = 18


def export(st, dst: Path):
    import torch

    class Wrapper(torch.nn.Module):
        def __init__(self, model):
            super().__init__()
            self.model = model

        def forward(self, input_ids, attention_mask):
            return self.model({"input_ids": input_ids, "attention_mask": attention_mask})["sentence_embedding"]

    tok = st.tokenizer(["task: classification | query: trop bien ce soir"], return_tensors="pt")
    dst.parent.mkdir(parents=True, exist_ok=True)
    with torch.no_grad():
        torch.onnx.export(Wrapper(st).eval(), (tok["input_ids"], tok["attention_mask"]), str(dst),
                          input_names=["input_ids", "attention_mask"], output_names=["sentence_embedding"],
                          dynamic_axes={"input_ids": {0: "batch", 1: "seq"}, "attention_mask": {0: "batch", 1: "seq"},
                                        "sentence_embedding": {0: "batch"}},
                          opset_version=OPSET, dynamo=False)


def quantize_q4(src: Path, dst: Path) -> str:
    import onnx
    from onnxruntime.quantization.matmul_nbits_quantizer import MatMulNBitsQuantizer

    model = onnx.load(str(src))
    q = MatMulNBitsQuantizer(model, block_size=32, is_symmetric=True, accuracy_level=4,
                             op_types_to_quantize=("MatMul", "Gather"),
                             quant_axes=(("MatMul", 0), ("Gather", 1)))
    q.process()
    q.model.save_model_to_file(str(dst), use_external_data_format=q.model.model.ByteSize() > 1.9e9)
    return "MatMul + Gather (embeddings)"


def size_mb(p: Path) -> float:
    data = Path(str(p) + "_data")
    return round((p.stat().st_size + (data.stat().st_size if data.exists() else 0)) / 1e6, 1)


def latency_ms(model, texts: list[str], prompt_name: str | None, warmup=5) -> float:
    for t in texts[:warmup]:
        model.encode([t], prompt_name=prompt_name)
    lat = []
    for t in texts:
        t0 = time.perf_counter()
        model.encode([t], prompt_name=prompt_name)
        lat.append((time.perf_counter() - t0) * 1000)
    return round(statistics.median(lat), 2)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--model", required=True, help="local sentence-transformers folder (e.g. pruned/gemma2-m3)")
    ap.add_argument("--n", type=int, default=200, help="dev_v1 messages used for parity and latency")
    args = ap.parse_args()

    path = Path(args.model)
    st = load_st(str(path))
    prompt_name = "Classification" if "Classification" in (st.prompts or {}) else None
    texts = [r["text"] for r in csv.DictReader(open(HERE / "dev_v1.csv", encoding="utf-8"))][:args.n]

    fp32_file, q4_file = path / "onnx/model.onnx", path / "onnx/model_q4.onnx"
    export(st, fp32_file)
    note = quantize_q4(fp32_file, q4_file)

    ref = st.encode(texts, prompt_name=prompt_name, normalize_embeddings=True)
    report = {"model": str(path), "opset": OPSET, "prompt": prompt_name, "parity_texts": len(texts),
              "pytorch_latency_ms": latency_ms(st, texts, prompt_name)}
    for tag, f in (("fp32", fp32_file), ("q4", q4_file)):
        m = OrtEmbedder(path, f"onnx/{f.name}")
        cos = (ref * m.encode(texts, prompt_name=prompt_name)).sum(axis=1)
        report |= {f"{tag}_size_mb": size_mb(f), f"{tag}_latency_ms": latency_ms(m, texts, prompt_name),
                   f"{tag}_cos_vs_pytorch_mean": round(float(cos.mean()), 5),
                   f"{tag}_cos_vs_pytorch_min": round(float(cos.min()), 5)}
    report["q4_scope"] = note
    (path / "onnx_report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))
    if report["fp32_cos_vs_pytorch_min"] < 0.999:
        print("WARNING: the FP32 ONNX graph does not match PyTorch; check the export before quantizing.")


if __name__ == "__main__":
    np.set_printoptions(precision=4)
    main()
