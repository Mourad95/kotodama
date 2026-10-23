"""Pure-logic tests for export_mobile.py (no model needed)."""
import json
import sys
from pathlib import Path

import numpy as np
import pytest

sys.path.insert(0, str(Path(__file__).parent))
import export_mobile as em  # noqa: E402


def test_softmax_matches_sklearn_multinomial():
    from sklearn.linear_model import LogisticRegression

    rng = np.random.default_rng(0)
    X = rng.normal(size=(len(em.LABELS) * 10, em.EMBEDDING_DIM))
    y = np.repeat(em.LABELS, 10)
    clf = LogisticRegression(C=10, max_iter=2000).fit(X, y)
    head = em.head_to_json(clf.classes_.tolist(), clf.coef_, clf.intercept_)
    for x in rng.normal(size=(5, em.EMBEDDING_DIM)):
        probs = em.head_probabilities(head, x)
        ref = dict(zip(clf.classes_, clf.predict_proba([x])[0]))
        assert all(abs(probs[k] - ref[k]) < 1e-9 for k in ref)
        assert abs(sum(probs.values()) - 1) < 1e-9


def test_head_serialisation_round_trip():
    coef = np.random.default_rng(1).normal(size=(len(em.LABELS), em.EMBEDDING_DIM))
    head = em.head_to_json(em.LABELS, coef, np.zeros(len(em.LABELS)))
    back = json.loads(json.dumps(head))
    assert back["labels"] == em.LABELS
    assert np.allclose(back["weights"], coef) and back["bias"] == [0.0] * len(em.LABELS)


def test_head_shape_is_checked():
    with pytest.raises(ValueError):
        em.head_to_json(em.LABELS, np.zeros((6, em.EMBEDDING_DIM)), np.zeros(len(em.LABELS)))


def test_manifest_contract(tmp_path):
    proto = tmp_path / "p.json"
    proto.write_text("{}")
    m = em.build_manifest(em.LABELS, {"b": "2", "a": "1"}, proto, {"dev_v1": {"global_accuracy": 1.0}})
    assert m["schema"] == 1 and m["embedding_dim"] == 768 and m["threshold"] == 0.2
    assert m["prompt"] == "task: classification | query: "
    assert m["labels"] == em.LABELS and list(m["files"]) == ["a", "b"]
    assert m["prototypes"] == {"name": "p.json", "sha256": em.sha256(proto)}


def test_export_uses_editable_eight_category_dataset_by_default(tmp_path, monkeypatch):
    """Run the actual export with a tiny deterministic embedder instead of model weights."""
    import hashlib
    from types import SimpleNamespace

    class FakeEmbedder:
        def __init__(self, model, onnx):
            self.prompts = {em.PROMPT_NAME: em.PROMPT}

        def encode(self, texts, **kwargs):
            vectors = []
            for text in texts:
                seed = int.from_bytes(hashlib.sha256(text.encode()).digest()[:8], "little")
                vector = np.random.default_rng(seed).normal(size=em.EMBEDDING_DIM)
                vectors.append(vector / np.linalg.norm(vector))
            return np.stack(vectors)

    model = tmp_path / "model"
    model.mkdir()
    (model / "model_q4.onnx").write_bytes(b"test graph boundary")
    (model / "tokenizer.json").write_text("{}")
    monkeypatch.setitem(sys.modules, "ort_embedder", SimpleNamespace(OrtEmbedder=FakeEmbedder))
    out = tmp_path / "bundle"
    monkeypatch.setattr(sys, "argv", ["export_mobile.py", "--model", str(model), "--onnx", "model_q4.onnx", "--out", str(out)])
    em.main()
    manifest = json.loads((out / "manifest.json").read_text())
    head = json.loads((out / "emotion_head.json").read_text())
    assert set(head["labels"]) == set(em.LABELS)
    assert manifest["training_count"] == len(em.training_rows(em.read_dataset(), em.read_dev(em.BENCH / "dev_v1.csv")))
    assert manifest["training_dataset"] == {"name": "dataset_emotions.xlsx", "sha256": em.sha256(em.DEFAULT_DATASET)}
    assert "prototypes" not in manifest
    assert "genance" not in manifest["metrics"]["dev_v1"]["evaluated_labels"]
    parity = json.loads((out / "parity.json").read_text())
    assert all(set(row["probabilities"]) == set(em.LABELS) for row in parity["messages"])
