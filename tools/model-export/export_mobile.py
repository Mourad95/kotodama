"""Build the model bundle (M1): ONNX graph + tokenizer + few-shot head + parity + manifest.

Chain: (optional) export_onnx -> embed Dataset emotions with OrtEmbedder -> train the multinomial
logistic-regression head exactly as research/emotion-benchmark/bench.py ("fewshot" mode) ->
write the bundle. The head scores softmax(W.e + b), which equals sklearn predict_proba.

Never tune anything on test_v2.csv: only dev_v1.csv is used (parity messages and metrics).
"""
from __future__ import annotations

import argparse
import csv
import hashlib
import json
import shutil
import subprocess
import sys
from pathlib import Path

import numpy as np

from dataset_emotions import DEFAULT_DATASET, LABELS as DATASET_LABELS, read_dataset, training_rows

REPO = Path(__file__).resolve().parents[2]
BENCH = REPO / "research" / "emotion-benchmark"
LEGACY_LABELS = ["joie", "colere", "energie", "suspense", "tristesse", "amour", "calme"]
LABELS = list(DATASET_LABELS)
PROMPT_NAME = "Classification"
PROMPT = "task: classification | query: "
THRESHOLD = 0.2
EMBEDDING_DIM = 768
SOURCE_MODEL = "google/embeddinggemma-2"
LICENCE = "Apache-2.0"
EDGE_CASES = [
    "ok",
    "😂😂😂",
    "❤️🥰",
    "Thanks a lot, you really made my day, I am so happy right now!",
    "I miss you so much, the house feels empty since you left.",
    " ".join(["Franchement je ne sais pas quoi faire, tout s'enchaîne trop vite"] * 5)[:300],
]


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def softmax(z: np.ndarray) -> np.ndarray:
    """Row-wise softmax of logits (shape (n, k) or (k,))."""
    z = z - z.max(axis=-1, keepdims=True)
    e = np.exp(z)
    return e / e.sum(axis=-1, keepdims=True)


def head_to_json(labels: list[str], coef: np.ndarray, intercept: np.ndarray) -> dict:
    """Serialise the head: labels in the order of the weight rows."""
    if coef.shape != (len(labels), EMBEDDING_DIM) or intercept.shape != (len(labels),):
        raise ValueError(f"unexpected head shape {coef.shape} / {intercept.shape}")
    return {"labels": list(labels), "weights": coef.astype(float).tolist(), "bias": intercept.astype(float).tolist()}


def head_probabilities(head: dict, embedding: np.ndarray) -> dict[str, float]:
    """Probabilities for one embedding from the serialised head (what the Rust side computes)."""
    logits = np.asarray(head["weights"], dtype=np.float64) @ np.asarray(embedding, dtype=np.float64)
    probs = softmax(logits + np.asarray(head["bias"], dtype=np.float64))
    return dict(zip(head["labels"], probs.tolist()))


def build_manifest(labels: list[str], files: dict[str, str], prototypes: Path, metrics: dict,
                   *, source_kind: str = "prototypes", training_count: int | None = None) -> dict:
    """Assemble manifest.json (schema 1)."""
    return {
        "schema": 1,
        "source_model": SOURCE_MODEL,
        "licence": LICENCE,
        "prompt": PROMPT,
        "threshold": THRESHOLD,
        "embedding_dim": EMBEDDING_DIM,
        "labels": list(labels),
        "files": dict(sorted(files.items())),
        source_kind: {"name": prototypes.name, "sha256": sha256(prototypes)},
        **({"training_count": training_count} if training_count is not None else {}),
        "metrics": metrics,
    }


def read_dev(path: Path) -> list[dict]:
    with path.open(encoding="utf-8", newline="") as f:
        return list(csv.DictReader(f))


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--model", type=Path, default=BENCH / "pruned" / "gemma2-m3", help="pruned model folder")
    ap.add_argument("--onnx", default="onnx/model_q4.onnx", help="Q4 graph, relative to --model")
    ap.add_argument("--export", action="store_true", help="run export_onnx.py on --model first")
    sources = ap.add_mutually_exclusive_group()
    sources.add_argument("--dataset", type=Path, help="editable Dataset emotions (.xlsx or .csv); the default training source")
    sources.add_argument("--prototypes", type=Path, help="explicit legacy seven-category prototype source")
    ap.add_argument("--out", type=Path, default=REPO / "bundle")
    args = ap.parse_args()

    # Validate the training source before loading a large model or creating any output.
    dev = read_dev(BENCH / "dev_v1.csv")
    if args.prototypes:
        source = args.prototypes
        protos = json.loads(source.read_text(encoding="utf-8"))
        labels = LEGACY_LABELS
        rows = [{"text": text, "label": label} for label in labels
                for text in [protos[label]["description"], *protos[label]["examples"]]]
        source_kind = "prototypes"
    else:
        source = args.dataset or DEFAULT_DATASET
        all_rows = read_dataset(source)
        rows = training_rows(all_rows, dev)
        labels = LABELS
        source_kind = "training_dataset"
        print(f"Dataset emotions: {len(rows)} training messages; {len(all_rows) - len(rows)} held-out (dev_v1, test_v2) overlaps excluded")

    sys.path.insert(0, str(BENCH))
    if args.export:
        subprocess.run([sys.executable, str(BENCH / "export_onnx.py"), "--model", str(args.model)],
                       check=True, cwd=BENCH)
    from sklearn.linear_model import LogisticRegression

    from ort_embedder import OrtEmbedder

    emb = OrtEmbedder(args.model, args.onnx)
    if emb.prompts.get(PROMPT_NAME) != PROMPT:
        raise SystemExit(f"model prompt {PROMPT_NAME!r} is not {PROMPT!r}")

    def enc(texts: list[str]) -> np.ndarray:
        return emb.encode(texts, prompt_name=PROMPT_NAME, normalize_embeddings=True)

    X = enc([row["text"] for row in rows])
    y = [row["label"] for row in rows]
    clf = LogisticRegression(max_iter=2000, C=10).fit(np.stack(X), y)
    # Class order of the weights is sklearn's (alphabetical); the labels list records it.
    head = head_to_json(clf.classes_.tolist(), clf.coef_, clf.intercept_)
    if sorted(head["labels"]) != sorted(labels):
        raise SystemExit("unexpected class set")

    # Numerical check: serialised head == predict_proba.
    sample = enc([next(row["text"] for row in rows if row["label"] == label) for label in labels])
    ref = clf.predict_proba(sample)
    mine = np.stack([[head_probabilities(head, e)[l] for l in clf.classes_] for e in sample])
    assert np.abs(ref - mine).max() < 1e-6, f"head softmax differs from predict_proba: {np.abs(ref - mine).max()}"

    # dev_v1 metrics and parity messages (never test_v2).
    dev_probs = [head_probabilities(head, e) for e in enc([r["text"] for r in dev])]
    hits = [max(p, key=p.get) == r["label"] for p, r in zip(dev_probs, dev)]
    accuracy = round(100 * sum(hits) / len(hits), 1)
    messages = [{"text": r["text"], "probabilities": p}
                for i, (r, p) in enumerate(zip(dev, dev_probs)) if i % 7 < 4]
    edge_probs = [head_probabilities(head, e) for e in enc(EDGE_CASES)]
    messages += [{"text": t, "probabilities": p} for t, p in zip(EDGE_CASES, edge_probs)]

    out = args.out
    out.mkdir(parents=True, exist_ok=True)
    src = args.model / args.onnx
    copies = {"model_q4.onnx": src, "tokenizer.json": args.model / "tokenizer.json"}
    data = src.with_name(src.name + "_data")
    if data.exists():
        copies["model_q4.onnx_data"] = data
    for name, path in copies.items():
        shutil.copyfile(path, out / name)

    write = lambda name, obj: (out / name).write_text(json.dumps(obj, ensure_ascii=False, indent=1) + "\n",
                                                      encoding="utf-8")
    write("emotion_head.json", head)
    write("parity.json", {"prompt": PROMPT, "messages": messages})
    names = [*copies, "emotion_head.json", "parity.json"]
    files = {n: sha256(out / n) for n in names}
    metrics = {"dev_v1": {"n": len(dev), "global_accuracy": accuracy,
                          "evaluated_labels": sorted({row["label"] for row in dev}),
                          "note": "Legacy development set; does not measure embarrassment recall."}}
    write("manifest.json", build_manifest(head["labels"], files, source, metrics,
                                                source_kind=source_kind, training_count=len(rows)))
    print(f"bundle written to {out}: dev_v1 accuracy {accuracy}% ({len(dev)} messages), "
          f"{len(messages)} parity messages")


if __name__ == "__main__":
    main()
