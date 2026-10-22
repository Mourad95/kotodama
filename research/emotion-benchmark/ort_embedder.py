"""Sentence embeddings from a self-contained ONNX graph (written by export_onnx.py).

The graph takes `input_ids` and `attention_mask` and returns `sentence_embedding` (pooled and
normalized), so the runtime only needs a tokenizer and ONNX Runtime — the same contract as the
Rust engine. No optimum, no transformers.
"""
from __future__ import annotations

import json
from pathlib import Path

import numpy as np


class OrtEmbedder:
    """Minimal `encode` compatible with the subset of SentenceTransformer that bench.py uses."""

    def __init__(self, model_dir: Path, onnx_file: str, threads: int = 0):
        import onnxruntime as ort
        from tokenizers import Tokenizer

        opts = ort.SessionOptions()
        if threads:
            opts.intra_op_num_threads = threads
        self.session = ort.InferenceSession(str(model_dir / onnx_file), opts, providers=["CPUExecutionProvider"])
        self.tokenizer = Tokenizer.from_file(str(model_dir / "tokenizer.json"))
        cfg = model_dir / "config_sentence_transformers.json"
        self.prompts = json.loads(cfg.read_text(encoding="utf-8")).get("prompts", {}) if cfg.exists() else {}

    def encode(self, texts: list[str], prompt_name: str | None = None, normalize_embeddings: bool = True,
               **_ignored) -> np.ndarray:
        prompt = self.prompts.get(prompt_name, "") if prompt_name else ""
        out = []
        for text in texts:  # one message at a time, as on the phone
            enc = self.tokenizer.encode(prompt + text)
            feeds = {"input_ids": np.array([enc.ids], dtype=np.int64),
                     "attention_mask": np.array([enc.attention_mask], dtype=np.int64)}
            out.append(self.session.run(["sentence_embedding"], feeds)[0][0])
        emb = np.stack(out).astype(np.float32)
        if normalize_embeddings:
            emb /= np.linalg.norm(emb, axis=1, keepdims=True)
        return emb


def is_self_contained(model_dir: Path, onnx_file: str) -> bool:
    """True when the ONNX file outputs `sentence_embedding` directly (export_onnx.py format)."""
    import onnx

    m = onnx.load(str(model_dir / onnx_file), load_external_data=False)
    return any(o.name == "sentence_embedding" for o in m.graph.output)
