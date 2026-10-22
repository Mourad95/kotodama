"""Benchmark des détecteurs d'émotion pour Texte Vivant (round 2).

Compare des moteurs sur un jeu de messages d'ados annotés avec les 7 émotions de l'app
et écrit <out>.md et <out>.json.

Principe « choix typé » : chaque candidat reçoit les émotions décrites dans prototypes.json
(description + exemples) et renvoie un choix + des probabilités + une confiance.

Usage :
    python bench.py --only minilm-proto minilm-fewshot              # candidats prédéfinis
    python bench.py --local minilm-elague=pruned/minilm --only none # modèle local (proto + fewshot)
    python bench.py --local minilm-int8=pruned/minilm@onnx/model_qint8_arm64.onnx --only none
    python bench.py --dataset dev_v1.csv --only minilm-proto        # jeu de développement
"""
from __future__ import annotations

import argparse
import csv
import json
import math
import platform
import statistics
import time
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).parent
LABELS = ["joie", "colere", "energie", "suspense", "tristesse", "amour", "calme"]

# Définitions pour le zero-shot (référence).
DEFINITIONS = {
    "joie": "happiness, fun, laughing, good news, pride in a success",
    "colere": "anger, annoyance, frustration, feeling something is unfair",
    "energie": "motivation, hype, determination, ready to fight or win",
    "suspense": "curiosity, teasing a secret, surprise, mystery or worry about what happened",
    "tristesse": "sadness, loss, disappointment, loneliness, missing someone",
    "amour": "love, affection, tenderness, gratitude toward someone close",
    "calme": "neutral, practical or logistical message with no strong emotion",
}

MAP_TABULARIS = {
    "anger": "colere", "contempt": "colere", "disgust": "colere", "frustration": "colere",
    "joy": "joie", "gratitude": "amour", "love": "amour", "sadness": "tristesse",
    "fear": "suspense", "surprise": "suspense", "neutral": "calme",
}
MAP_GOEMO = {
    "admiration": "joie", "amusement": "joie", "anger": "colere", "annoyance": "colere",
    "approval": "calme", "caring": "amour", "confusion": "suspense", "curiosity": "suspense",
    "desire": "amour", "disappointment": "tristesse", "disapproval": "colere", "disgust": "colere",
    "embarrassment": "tristesse", "excitement": "energie", "fear": "suspense", "gratitude": "amour",
    "grief": "tristesse", "joy": "joie", "love": "amour", "nervousness": "suspense",
    "optimism": "energie", "pride": "energie", "realization": "suspense", "relief": "calme",
    "remorse": "tristesse", "sadness": "tristesse", "surprise": "suspense", "neutral": "calme",
}
MAP_XLMEMO = {"anger": "colere", "fear": "suspense", "joy": "joie", "sadness": "tristesse"}

PROTOTYPES_PATH = HERE / "prototypes.json"


# ---------------------------------------------------------------- utilitaires
def size_mb(path: Path) -> float:
    path = Path(path)
    if path.is_file():
        extra = Path(str(path) + "_data")
        return (path.stat().st_size + (extra.stat().st_size if extra.exists() else 0)) / 1e6
    skip = {"onnx", "openvino"}
    return sum(p.stat().st_size for p in path.rglob("*")
               if p.is_file() and not skip & set(p.relative_to(path).parts)) / 1e6


def hub_dir(model_id: str, patterns=None, ignore=None) -> Path:
    from huggingface_hub import snapshot_download

    return Path(snapshot_download(model_id, allow_patterns=patterns, ignore_patterns=ignore))


# ---------------------------------------------------------------- références (labels figés)
def hf_classifier(model_id: str, mapping: dict[str, str]):
    from transformers import pipeline

    local = hub_dir(model_id, patterns=["*.json", "*.safetensors", "*.bin", "*.model", "*.txt"])
    clf = pipeline("text-classification", model=str(local), top_k=None, truncation=True)

    def run(text: str) -> str:
        agg: dict[str, float] = defaultdict(float)
        for s in clf(text)[0]:
            lab = s["label"].lower()
            if lab in mapping:
                agg[mapping[lab]] += s["score"]
        return max(agg, key=agg.get) if agg else "calme"

    return run, {"taille_mo": round(size_mb(local)), "remarque": "référence, labels figés"}


def zeroshot():
    from transformers import pipeline

    local = hub_dir("MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7",
                    patterns=["*.json", "*.safetensors", "*.bin", "*.model", "*.txt"])
    clf = pipeline("zero-shot-classification", model=str(local))
    hyp = {DEFINITIONS[k]: k for k in LABELS}

    def run(text: str) -> str:
        out = clf(text, candidate_labels=list(hyp), hypothesis_template="This message expresses {}.")
        return hyp[out["labels"][0]]

    return run, {"taille_mo": round(size_mb(local)), "remarque": "référence, zero-shot"}


# ---------------------------------------------------------------- choix typé
def embeddings(source: str, mode: str, onnx_file: str | None = None, truncate_dim: int | None = None,
               prefix: str | None = None):
    """Choix typé local : modèle d'embeddings + prototypes.json.

    source       : id Hugging Face ou dossier local (ex. modèle élagué).
    mode         : "proto" (émotion la plus proche, aucun entraînement)
                   ou "fewshot" (régression logistique apprise sur les exemples).
    onnx_file    : chemin relatif d'un fichier ONNX dans le dossier (ex. INT8) → backend ONNX Runtime.
    truncate_dim : troncature Matryoshka (ex. 512, 256), renormalisée ensuite.
    prefix       : fixed text prepended to every input (model's documented instruction format).
    """
    import numpy as np
    from sentence_transformers import SentenceTransformer

    from ort_embedder import OrtEmbedder, is_self_contained

    local = Path(source) if Path(source).exists() else hub_dir(
        source, ignore=["onnx/*", "openvino/*", "*.onnx", "*.h5", "*.msgpack", "*.ot"])
    if onnx_file and is_self_contained(local, onnx_file):
        # export_onnx.py graph (pooling + normalize inside): tokenizer + ONNX Runtime only.
        model = OrtEmbedder(local, onnx_file)
        taille = size_mb(local / onnx_file)
    elif onnx_file:
        model = SentenceTransformer(str(local), backend="onnx", device="cpu", truncate_dim=truncate_dim,
                                    model_kwargs={"file_name": onnx_file, "provider": "CPUExecutionProvider"})
        taille = size_mb(local / onnx_file)
    elif is_gemma2(local):
        # Round 3: text tower only (vision/audio encoders not loaded), float32 on CPU (model card:
        # bfloat16 is slow on CPU, float16 is unsafe).
        import torch

        model = SentenceTransformer(str(local), device="cpu", truncate_dim=truncate_dim,
                                    config_kwargs={"vision_config": None, "audio_config": None},
                                    model_kwargs={"dtype": torch.float32})
        taille = round(sum(p.numel() for p in model.parameters()) * 4 / 1e6, 1)  # text tower, FP32
    else:
        model = SentenceTransformer(str(local), truncate_dim=truncate_dim)
        taille = size_mb(local)

    if prefix is None:
        prefix = "query: " if "e5" in source.lower() else ""
    prompt_name = "Classification" if "Classification" in (getattr(model, "prompts", None) or {}) else None
    protos = json.loads(PROTOTYPES_PATH.read_text(encoding="utf-8"))

    def enc(texts):
        kw = {"normalize_embeddings": True}
        if prompt_name:
            kw["prompt_name"] = prompt_name
        return model.encode([prefix + t for t in texts], **kw)

    if mode == "proto":
        centers = []
        for lab in LABELS:
            vecs = enc([protos[lab]["description"], *protos[lab]["examples"]])
            c = vecs.mean(axis=0)
            centers.append(c / np.linalg.norm(c))
        centers = np.stack(centers)
        temperature = 20.0  # écarte les similarités cosinus pour obtenir des probabilités lisibles

        def run(text: str):
            sims = centers @ enc([text])[0]
            e = np.exp((sims - sims.max()) * temperature)
            probs = e / e.sum()
            return LABELS[int(np.argmax(probs))], dict(zip(LABELS, probs.tolist()))
    else:
        from sklearn.linear_model import LogisticRegression

        X, y = [], []
        for lab in LABELS:
            items = [protos[lab]["description"], *protos[lab]["examples"]]
            X.extend(enc(items))
            y.extend([lab] * len(items))
        head = LogisticRegression(max_iter=2000, C=10).fit(np.stack(X), y)

        def run(text: str):
            probs = head.predict_proba(enc([text]))[0]
            d = dict(zip(head.classes_.tolist(), probs.tolist()))
            return max(d, key=d.get), d

    backend = f"ONNX {Path(onnx_file).name}" if onnx_file else "PyTorch"
    dims = f", {truncate_dim} dims" if truncate_dim else ""
    return run, {"taille_mo": round(taille, 1), "remarque": f"choix typé ({mode}), {backend}{dims}"}


def is_gemma2(local: Path) -> bool:
    """True for an EmbeddingGemma 2 checkpoint (multimodal config: load the text tower only)."""
    cfg = local / "config.json"
    return cfg.exists() and json.loads(cfg.read_text(encoding="utf-8")).get("model_type") == "embedding_gemma2"


GEMMA = "google/embeddinggemma-300m"
MINILM = "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2"
E5 = "intfloat/multilingual-e5-small"
GEMMA2 = "google/embeddinggemma-2"  # Apache-2.0 (ADR-0010); text-only use, round 3
# Round 3b: fully open alternatives (OSI licence, no extra use policy).
E5_BASE = "intfloat/multilingual-e5-base"  # MIT
GRANITE = "ibm-granite/granite-embedding-278m-multilingual"  # Apache-2.0
GRANITE_S = "ibm-granite/granite-embedding-107m-multilingual"  # Apache-2.0
BGE_M3 = "BAAI/bge-m3"  # MIT
QWEN3 = "Qwen/Qwen3-Embedding-0.6B"  # Apache-2.0
# Qwen3-Embedding expects a one-line task instruction (model card format); fixed, not tuned.
GOEMO = "AnasAlokla/multilingual_go_emotions"  # MIT, mBERT fine-tuned on GoEmotions (28 labels)
QWEN3_PREFIX = "Instruct: Classify the emotion expressed in the message\nQuery: "

CANDIDATES = {
    "gemma-proto": lambda: embeddings(GEMMA, "proto"),
    "gemma-fewshot": lambda: embeddings(GEMMA, "fewshot"),
    "gemma2-proto": lambda: embeddings(GEMMA2, "proto"),
    "gemma2-fewshot": lambda: embeddings(GEMMA2, "fewshot"),
    "gemma2-fewshot-512": lambda: embeddings(GEMMA2, "fewshot", truncate_dim=512),
    "gemma2-fewshot-256": lambda: embeddings(GEMMA2, "fewshot", truncate_dim=256),
    "e5base-fewshot": lambda: embeddings(E5_BASE, "fewshot"),
    "granite-fewshot": lambda: embeddings(GRANITE, "fewshot"),
    "granite-small-fewshot": lambda: embeddings(GRANITE_S, "fewshot"),
    "bgem3-fewshot": lambda: embeddings(BGE_M3, "fewshot"),
    "goemo-fewshot": lambda: embeddings(GOEMO, "fewshot"),
    "qwen3-fewshot": lambda: embeddings(QWEN3, "fewshot", prefix=QWEN3_PREFIX),
    "minilm-proto": lambda: embeddings(MINILM, "proto"),
    "minilm-fewshot": lambda: embeddings(MINILM, "fewshot"),
    "e5small-proto": lambda: embeddings(E5, "proto"),
    "e5small-fewshot": lambda: embeddings(E5, "fewshot"),
    "tabularisai": lambda: hf_classifier("tabularisai/multilingual-emotion-classification", MAP_TABULARIS),
    "goemotions": lambda: hf_classifier("SamLowe/roberta-base-go_emotions", MAP_GOEMO),
    "xlmemo": lambda: hf_classifier("MilaNLProc/xlm-emo-t", MAP_XLMEMO),
    "zeroshot": zeroshot,
}


def add_local(spec: str) -> list[str]:
    """--local nom=dossier[@fichier.onnx] → ajoute nom-proto et nom-fewshot."""
    name, _, rest = spec.partition("=")
    path, _, onnx_file = rest.partition("@")
    for mode in ("proto", "fewshot"):
        CANDIDATES[f"{name}-{mode}"] = (lambda p=path, m=mode, o=onnx_file or None: embeddings(p, m, o))
    return [f"{name}-proto", f"{name}-fewshot"]


# ---------------------------------------------------------------- évaluation
def confidence(probs: dict) -> float:
    """1 si toute la probabilité est sur une émotion, 0 si elle est répartie uniformément."""
    ent = -sum(p * math.log(p) for p in probs.values() if p > 0)
    return 1 - ent / math.log(len(probs))


def evaluate(name, run, meta, rows):
    run(rows[0]["text"])  # chauffe
    lat, preds, confs = [], [], []
    for r in rows:
        t0 = time.perf_counter()
        p = run(r["text"])
        lat.append((time.perf_counter() - t0) * 1000)
        if isinstance(p, tuple):
            p, probs = p
            confs.append(confidence(probs))
        else:
            confs.append(None)
        preds.append(p)

    ok = [p == r["label"] for p, r in zip(preds, rows)]
    ok_tol = [p == r["label"] or bool(r.get("alt")) and p == r["alt"] for p, r in zip(preds, rows)]

    def acc(filt, hits=ok):
        sel = [i for i, r in enumerate(rows) if filt(r)]
        return round(100 * sum(hits[i] for i in sel) / len(sel), 1) if sel else None

    seuils = {}
    if all(c is not None for c in confs):
        for t in (0.2, 0.3, 0.4, 0.5):
            sel = [i for i in range(len(rows)) if confs[i] >= t]
            seuils[str(t)] = {
                "couverture": round(100 * len(sel) / len(rows)),
                "precision": round(100 * sum(ok_tol[i] for i in sel) / len(sel), 1) if sel else None,
            }
    return {
        "nom": name,
        "n": len(rows),
        "global": acc(lambda r: True),
        "global_tolerant": acc(lambda r: True, ok_tol),
        "fr": acc(lambda r: r["lang"] == "fr"),
        "en": acc(lambda r: r["lang"] == "en"),
        "fautes": acc(lambda r: r["typo"] == "1"),
        "sans_fautes": acc(lambda r: r["typo"] == "0"),
        "n_fautes": sum(r["typo"] == "1" for r in rows),
        "latence_ms": round(statistics.median(lat), 1),
        "rappel": {lab: acc(lambda r, lab=lab: r["label"] == lab) for lab in LABELS},
        "confiance_moy": round(statistics.mean(confs), 2) if seuils else None,
        "seuils": seuils,
        "erreurs": [(r["text"], r["label"], p) for p, r, h in zip(preds, rows, ok_tol) if not h],
        **meta,
    }


def report(results, dataset):
    n, nf = results[0]["n"], results[0]["n_fautes"]
    out = [f"# Résultats benchmark émotions — {dataset}\n",
           f"{n} messages dont {nf} avec fautes. Un message vaut {100 / n:.1f} points au global "
           f"et {100 / max(nf, 1):.1f} points dans la colonne « Avec fautes ».\n",
           "| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |",
           "|---|---|---|---|---|---|---|---|---|---|"]
    for r in sorted(results, key=lambda r: -r["global"]):
        out.append(f"| {r['nom']} | {r['global']} % | {r['global_tolerant']} % | {r['fr']} % | {r['en']} % | "
                   f"{r['fautes']} % | {r['sans_fautes']} % | {r['latence_ms']} ms | {r['taille_mo']} Mo | "
                   f"{r['remarque']} |")
    out += ["\n## Rappel par émotion\n", "| Modèle | " + " | ".join(LABELS) + " |",
            "|---|" + "---|" * len(LABELS)]
    for r in results:
        out.append(f"| {r['nom']} | " + " | ".join(f"{r['rappel'][l]} %" for l in LABELS) + " |")
    with_conf = [r for r in results if r["seuils"]]
    if with_conf:
        out += ["\n## Confiance (choix typé)\n",
                "Précision tolérante sur les messages dont la confiance dépasse le seuil, et part des messages concernés.\n",
                "| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |", "|---|---|---|---|---|---|"]
        for r in with_conf:
            cells = [f"{v['precision']} % sur {v['couverture']} %" for v in r["seuils"].values()]
            out.append(f"| {r['nom']} | {r['confiance_moy']} | " + " | ".join(cells) + " |")
    for r in results:
        out.append(f"\n## Erreurs : {r['nom']} ({len(r['erreurs'])})\n")
        for text, gold, pred in r["erreurs"]:
            out.append(f"- « {text} » → attendu **{gold}**, prédit **{pred}**")
    return "\n".join(out) + "\n"


def main():
    global PROTOTYPES_PATH
    ap = argparse.ArgumentParser()
    ap.add_argument("--only", nargs="*", help="candidats prédéfinis à lancer (« none » = aucun)")
    ap.add_argument("--skip", nargs="*", default=[])
    ap.add_argument("--local", nargs="*", default=[], help="nom=dossier[@onnx/fichier.onnx]")
    ap.add_argument("--dataset", default="test_v2.csv")
    ap.add_argument("--prototypes", default="prototypes.json")
    ap.add_argument("--out", default="results")
    args = ap.parse_args()

    PROTOTYPES_PATH = HERE / args.prototypes
    rows = list(csv.DictReader(open(HERE / args.dataset, encoding="utf-8")))
    for r in rows:
        r["alt"] = r.get("alt") or ""

    local_names = [n for spec in args.local for n in add_local(spec)]
    if args.only == ["none"]:
        names = local_names
    elif args.only:
        names = args.only + local_names
    else:
        names = [n for n in CANDIDATES if n not in args.skip and n not in local_names] + local_names

    results = []
    for name in names:
        print(f"→ {name}")
        try:
            run, meta = CANDIDATES[name]()
        except Exception as e:  # noqa: BLE001
            print(f"  ignoré : {type(e).__name__}: {e}")
            continue
        r = evaluate(name, run, meta, rows)
        print(f"  global {r['global']} %  tolérant {r['global_tolerant']} %  fautes {r['fautes']} %  "
              f"{r['latence_ms']} ms  {r['taille_mo']} Mo")
        results.append(r)

    if results:
        env = {"python": platform.python_version(), "machine": platform.machine(),
               "system": platform.platform(), "processor": platform.processor(),
               "dataset": args.dataset, "prototypes": args.prototypes}
        (HERE / f"{args.out}.md").write_text(report(results, args.dataset), encoding="utf-8")
        (HERE / f"{args.out}.json").write_text(
            json.dumps({"env": env, "results": results}, ensure_ascii=False, indent=2), encoding="utf-8")
        print(f"\nRapport : {args.out}.md et {args.out}.json")


if __name__ == "__main__":
    main()
