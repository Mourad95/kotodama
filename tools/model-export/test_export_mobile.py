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
    X = rng.normal(size=(70, em.EMBEDDING_DIM))
    y = np.repeat(em.LABELS, 10)
    clf = LogisticRegression(C=10, max_iter=2000).fit(X, y)
    head = em.head_to_json(clf.classes_.tolist(), clf.coef_, clf.intercept_)
    for x in rng.normal(size=(5, em.EMBEDDING_DIM)):
        probs = em.head_probabilities(head, x)
        ref = dict(zip(clf.classes_, clf.predict_proba([x])[0]))
        assert all(abs(probs[k] - ref[k]) < 1e-9 for k in ref)
        assert abs(sum(probs.values()) - 1) < 1e-9


def test_head_serialisation_round_trip():
    coef = np.random.default_rng(1).normal(size=(7, em.EMBEDDING_DIM))
    head = em.head_to_json(em.LABELS, coef, np.zeros(7))
    back = json.loads(json.dumps(head))
    assert back["labels"] == em.LABELS
    assert np.allclose(back["weights"], coef) and back["bias"] == [0.0] * 7


def test_head_shape_is_checked():
    with pytest.raises(ValueError):
        em.head_to_json(em.LABELS, np.zeros((6, em.EMBEDDING_DIM)), np.zeros(7))


def test_manifest_contract(tmp_path):
    proto = tmp_path / "p.json"
    proto.write_text("{}")
    m = em.build_manifest(em.LABELS, {"b": "2", "a": "1"}, proto, {"dev_v1": {"global_accuracy": 1.0}})
    assert m["schema"] == 1 and m["embedding_dim"] == 768 and m["threshold"] == 0.2
    assert m["prompt"] == "task: classification | query: "
    assert m["labels"] == em.LABELS and list(m["files"]) == ["a", "b"]
    assert m["prototypes"] == {"name": "p.json", "sha256": em.sha256(proto)}
