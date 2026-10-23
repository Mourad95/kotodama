"""Check and merge generated emotion examples into a new prototypes file.

    python curate_examples.py generated/raw/*.csv                     # report only
    python curate_examples.py generated/raw/*.csv --write prototypes_v3.json

Each CSV has the header `text,label` (labels: the 7 emotion ids). A row is rejected when it is malformed,
too long, carries personal data (phone number, e-mail, link, @handle), or duplicates (exactly or nearly)
another accepted row, an example of the base prototypes, or a text of dev_v1.csv / test_v2.csv. The last two
files are read ONLY to drop leaked duplicates; nothing is tuned on them (CLAUDE.md rule 4).

The base file (default prototypes_v2.json) is never modified: the output is a new file with the same
structure (descriptions kept, accepted examples appended).
"""
from __future__ import annotations

import argparse
import csv
import difflib
import json
import re
import sys
import unicodedata
from dataclasses import dataclass, field
from pathlib import Path

HERE = Path(__file__).parent
LABELS = ["joie", "colere", "energie", "suspense", "tristesse", "amour", "calme"]
MAX_CHARS = 300
NEAR_DUPLICATE = 0.88
TARGET_PER_LABEL = 150
MIN_SLANG_SHARE = 0.5
# Markers of teen slang, verlan and messaging abbreviations (whole words, accents and case ignored).
SLANG = re.compile(
    r"\b(mdr+|ptdr+|jpp|jsp|jsuis|chui|tkt|tfk|tmtc|wesh|wsh|wallah|wlh|frr+|frero|frere|reuf|reus|sah|"
    r"relou|chelou|zarbi|ouf|venere?|vnr|saoul\w*|gave|kiff\w*|askip|bg|boloss|seum|flemme|daron\w*|"
    r"meuf|keum|teuf|tarpin|abuse|oklm|bail|ptn|pk|pq|bcp|dsl|stp|svp|vsy|tjrs|tt|jtm|jtd|cv|"
    r"chaud\w*|ghost\w*|crush|lache rien|niaque|hess|pls|bad|gener|chanme|taff|squatt\w*|sheesh|"
    r"fdp|ntm|miskine|dead|ouais|nan|waouh|wow|yes+|ahah\w*|haha\w*|lol|omg|omgg+|bro)\b"
    r"|trop |de ouf|ca passe|a fond|j'en peux plus|jpense|jvais|jveux|jsais|j'suis|t'es|y'a|ya ",
    re.I,
)
PERSONAL = [
    (re.compile(r"\d{6,}"), "long number"),
    (re.compile(r"\S+@\S+\.\S+"), "e-mail"),
    (re.compile(r"https?://|www\.", re.I), "link"),
    (re.compile(r"(?<!\w)@\w{2,}"), "@handle"),
]


def has_slang(text: str) -> bool:
    """True if the message carries a slang, verlan or abbreviation marker (accents and case ignored)."""
    plain = "".join(c for c in unicodedata.normalize("NFKD", text) if not unicodedata.combining(c))
    return bool(SLANG.search(plain))


def normalise(text: str) -> str:
    """Key for duplicate detection: lower case, no accents, repeated letters squeezed, letters and digits only."""
    t = unicodedata.normalize("NFKD", text.casefold())
    t = "".join(c for c in t if not unicodedata.combining(c))
    t = re.sub(r"(.)\1{2,}", r"\1\1", t)
    t = re.sub(r"[^\w\s]|_", " ", t)
    return re.sub(r"\s+", " ", t).strip()


@dataclass
class Report:
    accepted: dict[str, list[str]] = field(default_factory=lambda: {l: [] for l in LABELS})
    rejected: list[tuple[str, int, str, str]] = field(default_factory=list)  # file, line, reason, text


def read_texts(path: Path) -> list[str]:
    with path.open(encoding="utf-8", newline="") as f:
        return [r["text"] for r in csv.DictReader(f) if r.get("text")]


def similar(key: str, known: list[str]) -> str | None:
    for other in known:
        if key == other:
            return "duplicate"
        if abs(len(key) - len(other)) <= max(6, len(key) // 3) and \
                difflib.SequenceMatcher(None, key, other).ratio() >= NEAR_DUPLICATE:
            return "near-duplicate"
    return None


def read_exclusions(path: Path | None) -> dict[str, str]:
    """`text<TAB>reason` lines (blank lines and `#` comments ignored) -> {normalised text: reason}."""
    if path is None or not path.exists():
        return {}
    out = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.strip() and not line.startswith("#"):
            text, _, reason = line.partition("\t")
            out[normalise(text)] = reason.strip() or "excluded by hand"
    return out


def curate(files: list[Path], base_examples: list[str], reference_texts: list[str],
           exclusions: dict[str, str] | None = None) -> Report:
    exclusions = exclusions or {}
    report = Report()
    known = [normalise(t) for t in base_examples]
    reference = [normalise(t) for t in reference_texts]
    for path in files:
        with path.open(encoding="utf-8", newline="") as f:
            rows = list(csv.reader(f))
        if not rows or [c.strip() for c in rows[0]] != ["text", "label"]:
            report.rejected.append((path.name, 1, "header is not `text,label`", ""))
            continue
        for number, row in enumerate(rows[1:], start=2):
            text = row[0].strip() if row else ""
            def reject(reason: str) -> None:
                report.rejected.append((path.name, number, reason, text))
            if len(row) != 2:
                reject("malformed row (expected text,label)")
            elif row[1].strip() not in LABELS:
                reject(f"unknown label {row[1].strip()!r}")
            elif not text:
                reject("empty text")
            elif len(text) > MAX_CHARS:
                reject(f"longer than {MAX_CHARS} characters")
            else:
                label, key = row[1].strip(), normalise(text)
                personal = next((why for rx, why in PERSONAL if rx.search(text)), None)
                dup = similar(key, known)
                leak = similar(key, reference)
                if personal:
                    reject(f"personal data ({personal})")
                elif not key:
                    reject("no letters or digits")
                elif key in exclusions:
                    reject(f"excluded: {exclusions[key]}")
                elif dup:
                    reject(f"{dup} of an accepted or base example")
                elif leak:
                    reject(f"{leak} of a measurement text (dev_v1 / test_v2)")
                else:
                    report.accepted[label].append(text)
                    known.append(key)
    return report


def merge(base: dict, report: Report) -> dict:
    merged = json.loads(json.dumps(base))
    for label, texts in report.accepted.items():
        merged[label]["examples"].extend(texts)
    return merged


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("files", nargs="+", type=Path)
    ap.add_argument("--base", type=Path, default=HERE / "prototypes_v2.json")
    ap.add_argument("--exclude", type=Path, default=HERE / "generated" / "exclude.tsv",
                    help="`text<TAB>reason` lines of messages to drop (mislabelled, outdated...)")
    ap.add_argument("--write", type=Path, help="write the merged prototypes to this new file")
    args = ap.parse_args()

    base = json.loads(args.base.read_text(encoding="utf-8"))
    base_examples = [e for l in LABELS for e in [base[l]["description"], *base[l]["examples"]]]
    reference = read_texts(HERE / "dev_v1.csv") + read_texts(HERE / "test_v2.csv")
    report = curate(args.files, base_examples, reference, read_exclusions(args.exclude))

    print("accepted per label (base examples + new; slang share must be >= %d %%):" % (MIN_SLANG_SHARE * 100))
    low = []
    for l in LABELS:
        new = report.accepted[l]
        share = sum(map(has_slang, new)) / len(new) if new else 1.0
        flag = "  <-- too little slang" if share < MIN_SLANG_SHARE else ""
        if flag:
            low.append(l)
        print(f"  {l:9s} +{len(new):3d}  -> {len(base[l]['examples']) + len(new):3d}   slang {share:4.0%}{flag}")
    print(f"rejected: {len(report.rejected)}")
    for file, line, reason, text in report.rejected:
        print(f"  {file}:{line}: {reason}: {text[:70]!r}")
    if low and args.write:
        sys.exit(f"not writing: too little slang in {', '.join(low)}")
    if args.write:
        if args.write.resolve() == args.base.resolve():
            sys.exit("refusing to overwrite the base prototypes file")
        args.write.write_text(json.dumps(merge(base, report), ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
        print(f"written: {args.write}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
