"""Tests for curate_examples.py (pure logic, no data needed)."""
import json
from pathlib import Path

import curate_examples as ce


def write(tmp_path: Path, name: str, body: str) -> Path:
    p = tmp_path / name
    p.write_text(body, encoding="utf-8")
    return p


def run(tmp_path, body, base=(), reference=()):
    return ce.curate([write(tmp_path, "a.csv", body)], list(base), list(reference))


def reasons(report):
    return [r[2] for r in report.rejected]


def test_normalise_squeezes_letters_accents_and_punctuation():
    assert ce.normalise("TRAAAAP bien !!!") == ce.normalise("traap bien") == "traap bien"
    assert ce.normalise("Élève, ça va ?") == "eleve ca va"


def test_valid_rows_are_accepted(tmp_path):
    r = run(tmp_path, 'text,label\n"trop content, wesh",joie\nje suis au bus,calme\n')
    assert r.accepted["joie"] == ["trop content, wesh"] and r.accepted["calme"] == ["je suis au bus"]
    assert not r.rejected


def test_malformed_unknown_empty_and_long_rows_are_rejected(tmp_path):
    long = "a " * 200
    r = run(tmp_path, f'text,label\nsans label\nok,inconnue\n"",joie\n"{long}",joie\n')
    assert len(r.rejected) == 4
    assert "malformed" in reasons(r)[0] and "unknown label" in reasons(r)[1]
    assert reasons(r)[2] == "empty text" and "longer" in reasons(r)[3]
    assert not any(r.accepted.values())


def test_bad_header_rejects_the_file(tmp_path):
    r = run(tmp_path, "texte,emotion\nx,joie\n")
    assert "header" in reasons(r)[0] and not any(r.accepted.values())


def test_personal_data_is_rejected(tmp_path):
    body = 'text,label\nappelle moi au 0612345678,calme\nmon mail a@b.fr,calme\nva sur https://x.io,calme\n"suis @pseudo",calme\n'
    r = run(tmp_path, body)
    assert len(r.rejected) == 4 and all("personal data" in x for x in reasons(r))


def test_duplicates_near_duplicates_and_leaks_are_rejected(tmp_path):
    body = (
        "text,label\n"
        "trop content de te voir,joie\n"
        "TROP content de te voir !!,joie\n"          # duplicate after normalisation
        "trop contente de te voir demain,joie\n"     # near-duplicate of the base example
        "on mange a quelle heure,calme\n"            # identical to a measurement text
        "j'ai un truc de fou a te dire,suspense\n"   # fine
    )
    r = run(tmp_path, body, base=["trop content de te voir demain"], reference=["On mange à quelle heure ?"])
    assert r.accepted["suspense"] == ["j'ai un truc de fou a te dire"]
    assert [x for x in reasons(r) if "duplicate" in x or "measurement" in x]
    assert len(r.rejected) == 3 and not r.accepted["calme"]
    assert r.accepted["joie"] == ["trop content de te voir"]


def test_merge_keeps_the_base_untouched(tmp_path):
    base = {l: {"description": l, "examples": ["x"]} for l in ce.LABELS}
    r = run(tmp_path, "text,label\nnouveau message,joie\n")
    merged = ce.merge(base, r)
    assert merged["joie"]["examples"] == ["x", "nouveau message"] and base["joie"]["examples"] == ["x"]
    json.dumps(merged)
