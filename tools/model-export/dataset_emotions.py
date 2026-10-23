"""Read and validate the editable Dataset emotions workbook, without spreadsheet dependencies.

The eight category sheets are authoritative. Tous_les_messages is an original convenience
snapshot and is never concatenated with them. Workbook cells are data, never instructions.
"""
from __future__ import annotations

import argparse
from collections import Counter
import csv
from pathlib import Path, PurePosixPath
import re
import unicodedata
import xml.etree.ElementTree as ET
import zipfile

REPO = Path(__file__).resolve().parents[2]
DEFAULT_DATASET = REPO / "data" / "emotions" / "dataset_emotions.xlsx"
# Read only to keep its messages out of training (a leak guard), never to tune or measure anything.
TEST_SET = REPO / "research" / "emotion-benchmark" / "test_v2.csv"
LABELS = ("joie", "colere", "energie", "suspense", "tristesse", "amour", "calme", "genance")
NS = {"s": "http://schemas.openxmlformats.org/spreadsheetml/2006/main"}
REL_NS = "http://schemas.openxmlformats.org/officeDocument/2006/relationships"


def text_key(text: str) -> str:
    """Compare duplicate messages without changing the text used for training."""
    return " ".join(unicodedata.normalize("NFKC", text).casefold().replace("’", "'").split())


def leak_key(text: str) -> str:
    """Compare held-out messages loosely: punctuation and spacing differences do not hide a leak."""
    letters = "".join(c if c.isalnum() or c == "'" else " " for c in text_key(text))
    return " ".join(letters.split())


def held_out_test_texts(path: Path = TEST_SET) -> set[str]:
    """Leak keys of the frozen test set, used only to exclude its messages from training."""
    with path.open(encoding="utf-8", newline="") as stream:
        return {leak_key(row["text"]) for row in csv.DictReader(stream)}


def _sheet_rows(archive: zipfile.ZipFile, path: str, strings: list[str]) -> list[dict[str, str]]:
    root = ET.fromstring(archive.read(path))
    rows: list[dict[str, str]] = []
    for row in root.findall("s:sheetData/s:row", NS):
        values: dict[str, str] = {}
        for cell in row.findall("s:c", NS):
            reference = cell.get("r", "")
            column = re.sub(r"\d", "", reference)
            if cell.find("s:f", NS) is not None:
                raise ValueError(f"{path} {reference}: formula cells are not training text")
            kind = cell.get("t")
            value = cell.find("s:v", NS)
            if kind == "s" and value is not None:
                content = strings[int(value.text or "0")]
            elif kind == "inlineStr":
                content = "".join(node.text or "" for node in cell.findall("s:is//s:t", NS))
            else:
                content = value.text or "" if value is not None else ""
            values[column] = content
        if any(v.strip() for v in values.values()):
            rows.append(values)
    if not rows:
        raise ValueError(f"{path}: missing text,label header")
    headers = {col: value.strip() for col, value in rows[0].items()}
    if not {"text", "label"} <= set(headers.values()):
        raise ValueError(f"{path}: expected text,label columns")
    if len(headers.values()) != len(set(headers.values())):
        raise ValueError(f"{path}: duplicate column headers")
    return [{headers[col]: value for col, value in row.items() if col in headers} for row in rows[1:]]


def _read_xlsx(path: Path) -> list[dict[str, str]]:
    with zipfile.ZipFile(path) as archive:
        strings = []
        if "xl/sharedStrings.xml" in archive.namelist():
            root = ET.fromstring(archive.read("xl/sharedStrings.xml"))
            strings = ["".join(n.text or "" for n in item.findall(".//s:t", NS)) for item in root.findall("s:si", NS)]
        relations = ET.fromstring(archive.read("xl/_rels/workbook.xml.rels"))
        targets = {r.attrib["Id"]: r.attrib["Target"] for r in relations}
        workbook = ET.fromstring(archive.read("xl/workbook.xml"))
        sheets = {sheet.attrib["name"]: sheet.attrib[f"{{{REL_NS}}}id"]
                  for sheet in workbook.findall("s:sheets/s:sheet", NS)}
        missing = set(LABELS) - sheets.keys()
        if missing:
            raise ValueError(f"missing category sheets: {', '.join(sorted(missing))}")
        records = []
        for label in LABELS:
            target = targets[sheets[label]]
            target = target.lstrip("/") if target.startswith("/") else str(PurePosixPath("xl") / target)
            for number, row in enumerate(_sheet_rows(archive, target, strings), 2):
                if row.get("label", "").strip() != label:
                    raise ValueError(f"{label} row {number}: label must match the category sheet")
                records.append(row)
        return records


def read_dataset(path: Path = DEFAULT_DATASET) -> list[dict[str, str]]:
    """Load validated unique rows. Reject blank messages, missing categories and ambiguous labels."""
    if path.suffix.lower() == ".xlsx":
        rows = _read_xlsx(path)
    elif path.suffix.lower() == ".csv":
        with path.open(encoding="utf-8-sig", newline="") as stream:
            reader = csv.DictReader(stream)
            if not {"text", "label"} <= set(reader.fieldnames or []):
                raise ValueError("expected text,label columns")
            rows = list(reader)
    else:
        raise ValueError("dataset must be an .xlsx or .csv file")
    unique: dict[str, dict[str, str]] = {}
    for number, row in enumerate(rows, 2):
        text, label = (row.get("text") or "").strip(), (row.get("label") or "").strip()
        if not text or label not in LABELS:
            raise ValueError(f"row {number}: nonempty text and a known label are required")
        key = text_key(text)
        if key in unique and unique[key]["label"] != label:
            raise ValueError(f"row {number}: the same message has conflicting labels")
        unique.setdefault(key, {"text": text, "label": label})
    missing = set(LABELS) - {r["label"] for r in unique.values()}
    if missing:
        raise ValueError(f"missing categories: {', '.join(sorted(missing))}")
    return list(unique.values())


def training_rows(rows: list[dict[str, str]], dev: list[dict[str, str]]) -> list[dict[str, str]]:
    """Keep development and frozen test messages out of training.

    The test set is only read for this exclusion; nothing is tuned or measured against it.
    """
    held_out = {leak_key(row["text"]) for row in dev} | held_out_test_texts()
    result = [row for row in rows if leak_key(row["text"]) not in held_out]
    if set(LABELS) - {row["label"] for row in result}:
        raise ValueError("a category is empty after excluding held-out messages")
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, default=DEFAULT_DATASET)
    parser.add_argument("--out", type=Path, help="optional reproducible text,label CSV export")
    args = parser.parse_args()
    rows = read_dataset(args.source)
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        with args.out.open("w", encoding="utf-8", newline="") as stream:
            writer = csv.DictWriter(stream, fieldnames=["text", "label"])
            writer.writeheader()
            writer.writerows(rows)
    counts = Counter(row["label"] for row in rows)
    print(f"Dataset emotions: {len(rows)} unique messages")
    for label in LABELS:
        print(f"  {label}: {counts[label]}")


if __name__ == "__main__":
    main()
