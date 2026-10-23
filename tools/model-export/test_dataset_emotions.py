"""Regression coverage for workbook enrichment, source validation and training isolation."""
import csv
import sys
from pathlib import Path
import xml.etree.ElementTree as ET
import zipfile

import pytest

sys.path.insert(0, str(Path(__file__).parent))
from dataset_emotions import DEFAULT_DATASET, LABELS, NS, read_dataset, training_rows


def test_workbook_reads_category_sheets_once_and_matches_csv():
    rows = read_dataset()
    assert rows
    assert {row["label"] for row in rows} == set(LABELS)
    assert rows == read_dataset(DEFAULT_DATASET.with_suffix(".csv"))


def test_appended_category_row_is_used_even_when_summary_is_stale(tmp_path):
    out = tmp_path / "enriched.xlsx"
    with zipfile.ZipFile(DEFAULT_DATASET) as source, zipfile.ZipFile(out, "w") as target:
        for name in source.namelist():
            data = source.read(name)
            if name == "xl/worksheets/sheet2.xml":
                root = ET.fromstring(data)
                rows = root.find("s:sheetData", NS)
                next_row = 1 + max(int(row.attrib["r"]) for row in rows)
                row = ET.SubElement(rows, f"{{{NS['s']}}}row", {"r": str(next_row)})
                for col, text in [("A", "Mon nouveau message embarrassé."), ("B", "genance")]:
                    cell = ET.SubElement(row, f"{{{NS['s']}}}c", {"r": f"{col}{next_row}", "t": "inlineStr"})
                    inline = ET.SubElement(cell, f"{{{NS['s']}}}is")
                    ET.SubElement(inline, f"{{{NS['s']}}}t").text = text
                data = ET.tostring(root)
            target.writestr(name, data)
    rows = read_dataset(out)
    baseline = read_dataset()
    assert len(rows) == len(baseline) + 1
    assert sum(r["label"] == "genance" for r in rows) == sum(r["label"] == "genance" for r in baseline) + 1


def write_csv(path, rows):
    with path.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=["text", "label"])
        writer.writeheader()
        writer.writerows(rows)


def test_duplicates_are_not_weighted_twice_and_conflicting_labels_fail(tmp_path):
    rows = read_dataset()
    path = tmp_path / "dataset.csv"
    write_csv(path, rows + [rows[0]])
    assert len(read_dataset(path)) == len(rows)
    write_csv(path, rows + [{**rows[0], "label": "colere"}])
    with pytest.raises(ValueError, match="conflicting labels"):
        read_dataset(path)


def test_missing_classes_and_blank_text_are_rejected(tmp_path):
    rows = read_dataset()
    path = tmp_path / "dataset.csv"
    write_csv(path, [r for r in rows if r["label"] != "genance"])
    with pytest.raises(ValueError, match="missing categories"):
        read_dataset(path)
    write_csv(path, rows + [{"text": " ", "label": "joie"}])
    with pytest.raises(ValueError, match="nonempty text"):
        read_dataset(path)


def test_development_messages_are_excluded_before_training():
    rows = read_dataset()
    filtered = training_rows(rows, [{"text": rows[0]["text"].upper()}])
    assert len(filtered) == len(rows) - 1
    assert rows[0] not in filtered
