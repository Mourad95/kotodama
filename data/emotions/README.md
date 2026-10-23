# Dataset emotions

`dataset_emotions.xlsx` is the editable source of truth for training the emotion head.
It was supplied by the owner on 2026-10-23 as `dataset_emotions_8_categories_800.xlsx`.
The workbook is copied byte-for-byte; its data and formatting are unchanged.
Initial content: 800 distinct French messages, 100 per category.

## Enrich the dataset

1. Open `dataset_emotions.xlsx` in Excel or another spreadsheet editor.
2. Append rows to the relevant category sheet, keeping the `text` and `label` headers.
3. Use the sheet's exact label: `joie`, `colere`, `energie`, `suspense`, `tristesse`, `amour`,
   `calme`, or `genance`. Keep each message paired with one unambiguous label.
4. Save and validate from the repository root:

   ```bash
   python tools/model-export/dataset_emotions.py --out data/emotions/dataset_emotions.csv
   ```

The eight category sheets are authoritative. `Tous_les_messages` is the original combined
snapshot and is ignored during import. Adding rows there alone does not add training examples.
The importer reads appended worksheet rows beyond the original 100 examples. Identical
messages are counted once. Contradictory labels, formulas, empty messages and missing
categories are rejected. Message text, spelling, accents, slang and emoji are preserved.

`dataset_emotions.csv` is a reproducible convenience export, not a second editable source.
The model exporter reads the workbook directly on every run, so a stale CSV cannot silently
prevent new examples from reaching training.

## Train and export

```bash
python tools/model-export/export_mobile.py --model /path/to/pruned-model --onnx slim/model_q4.onnx
```

The default source is this workbook. `--dataset /path/to/file.xlsx` or `--dataset /path/to/file.csv`
selects another validated eight-category source. `--prototypes /path/to/prototypes.json` explicitly
restores the legacy seven-category pipeline.

The manifest records the dataset filename, SHA-256 and actual training count. Messages shared
with `dev_v1.csv` are excluded before training. The frozen research files remain unchanged;
`test_v2.csv` is never used by the training importer. The legacy development set does not
measure embarrassment recall: eight-category quality needs a separately labelled evaluation set.
Adding examples changes the next trained head, not an already installed app's weights.

## Provenance

User-supplied training material; authorship and generation method have not been independently
verified. This is not a new collection of private conversations. Do not add real minors'
conversations without satisfying the project's data-consent gate (docs/legal.md L23).
The prior research protocols, datasets and reported benchmark results remain historical records.
