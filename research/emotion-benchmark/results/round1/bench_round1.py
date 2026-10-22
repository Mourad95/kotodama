"""Benchmark des détecteurs d'émotion pour Texte Vivant.

Compare plusieurs modèles sur dataset.csv (messages d'ados FR/EN annotés
avec les 7 émotions de l'app) et écrit results.md.

Usage :
    python bench.py                      # tous les modèles disponibles
    python bench.py --only minilm-proto  # un seul candidat
    python bench.py --skip zeroshot      # en exclure

Principe « choix typé » : chaque candidat local reçoit les émotions décrites dans
prototypes.json et renvoie un choix + des probabilités + une confiance.
"""
from __future__ import annotations

import argparse
import csv
import os
import statistics
import time
from collections import Counter, defaultdict
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).parent
LABELS = ["joie", "colere", "energie", "suspense", "tristesse", "amour", "calme"]

# Définitions communes : utilisées par le zero-shot.
DEFINITIONS = {
    "joie": "happiness, fun, laughing, good news, pride in a success",
    "colere": "anger, annoyance, frustration, feeling something is unfair",
    "energie": "motivation, hype, determination, ready to fight or win",
    "suspense": "curiosity, teasing a secret, surprise, mystery or worry about what happened",
    "tristesse": "sadness, loss, disappointment, loneliness, missing someone",
    "amour": "love, affection, tenderness, gratitude toward someone close",
    "calme": "neutral, practical or logistical message with no strong emotion",
}

# ---------------------------------------------------------------- mappings
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


# ---------------------------------------------------------------- runners
def dir_size_mb(path: str) -> float:
    total = sum(p.stat().st_size for p in Path(path).rglob("*") if p.is_file())
    return total / 1e6


def hf_classifier(model_id: str, mapping: dict[str, str]):
    from huggingface_hub import snapshot_download
    from transformers import pipeline

    local = snapshot_download(model_id, allow_patterns=["*.json", "*.safetensors", "*.bin", "*.model", "*.txt"])
    clf = pipeline("text-classification", model=local, top_k=None, truncation=True)

    def run(text: str) -> str:
        scores = clf(text)[0]
        agg: dict[str, float] = defaultdict(float)
        for s in scores:
            lab = s["label"].lower()
            if lab in mapping:
                agg[mapping[lab]] += s["score"]
        return max(agg, key=agg.get) if agg else "calme"

    return run, {"taille_mo": round(dir_size_mb(local)), "remarque": "local"}


def zeroshot():
    model_id = "MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7"
    from huggingface_hub import snapshot_download
    from transformers import pipeline

    local = snapshot_download(model_id, allow_patterns=["*.json", "*.safetensors", "*.bin", "*.model", "*.txt"])
    clf = pipeline("zero-shot-classification", model=local)
    hyp = {DEFINITIONS[k]: k for k in LABELS}

    def run(text: str) -> str:
        out = clf(text, candidate_labels=list(hyp), hypothesis_template="This message expresses {}.")
        return hyp[out["labels"][0]]

    return run, {"taille_mo": round(dir_size_mb(local)), "remarque": "local, labels libres"}




def embeddings(model_id: str, mode: str, query_prefix: str = "", prompt_name: str | None = None):
    """Choix typé local : un modèle d'embeddings + prototypes.json (description + exemples).

    mode="proto"  : chaque émotion = moyenne des vecteurs de sa description et de ses exemples,
                    on prend la plus proche (aucun entraînement).
    mode="fewshot": petite régression logistique entraînée sur les exemples (style SetFit sans
                    fine-tuning du modèle), quelques secondes de calcul.
    Les exemples de prototypes.json sont distincts de ceux de dataset.csv.
    """
    import json

    import numpy as np
    from huggingface_hub import snapshot_download
    from sentence_transformers import SentenceTransformer

    local = snapshot_download(model_id, ignore_patterns=["onnx/*", "openvino/*", "*.onnx", "*.h5", "*.msgpack", "*.ot"])
    model = SentenceTransformer(local)
    protos = json.loads((HERE / "prototypes.json").read_text(encoding="utf-8"))

    def enc(texts):
        texts = [query_prefix + t for t in texts]
        kw = {"normalize_embeddings": True}
        if prompt_name and prompt_name in getattr(model, "prompts", {}):
            kw["prompt_name"] = prompt_name
        return model.encode(texts, **kw)

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

    return run, {"taille_mo": round(dir_size_mb(local)), "remarque": f"local, embeddings ({mode})"}


CANDIDATES = {
    "tabularisai": lambda: hf_classifier("tabularisai/multilingual-emotion-classification", MAP_TABULARIS),
    "goemotions": lambda: hf_classifier("SamLowe/roberta-base-go_emotions", MAP_GOEMO),
    "xlmemo": lambda: hf_classifier("MilaNLProc/xlm-emo-t", MAP_XLMEMO),
    "zeroshot": zeroshot,
    # Choix typé local : embeddings + descriptions/exemples (prototypes.json)
    "gemma-proto": lambda: embeddings("google/embeddinggemma-300m", "proto", prompt_name="Classification"),
    "gemma-fewshot": lambda: embeddings("google/embeddinggemma-300m", "fewshot", prompt_name="Classification"),
    "minilm-proto": lambda: embeddings("sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2", "proto"),
    "minilm-fewshot": lambda: embeddings("sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2", "fewshot"),
    "e5small-proto": lambda: embeddings("intfloat/multilingual-e5-small", "proto", query_prefix="query: "),
    "e5small-fewshot": lambda: embeddings("intfloat/multilingual-e5-small", "fewshot", query_prefix="query: "),
}


# ---------------------------------------------------------------- eval
def confidence(probs: dict) -> float:
    """1 si toute la probabilité est sur une émotion, 0 si elle est répartie uniformément."""
    import math

    ent = -sum(p * math.log(p) for p in probs.values() if p > 0)
    return 1 - ent / math.log(len(probs))


def evaluate(name, run, meta, rows, workers):
    run(rows[0]["text"])  # chauffe
    lat, preds, confs = [], [None] * len(rows), [None] * len(rows)

    def one(i):
        t0 = time.perf_counter()
        p = run(rows[i]["text"])
        return i, p, (time.perf_counter() - t0) * 1000

    with ThreadPoolExecutor(max_workers=workers) as ex:
        for i, p, ms in ex.map(one, range(len(rows))):
            if isinstance(p, tuple):
                p, probs = p
                confs[i] = confidence(probs)
            preds[i] = p
            lat.append(ms)

    def acc(filt):
        sel = [i for i, r in enumerate(rows) if filt(r)]
        return round(100 * sum(preds[i] == rows[i]["label"] for i in sel) / len(sel)) if sel else None

    recall = {
        lab: acc(lambda r, lab=lab: r["label"] == lab) for lab in LABELS
    }
    errors = [(rows[i]["text"], rows[i]["label"], preds[i]) for i in range(len(rows)) if preds[i] != rows[i]["label"]]

    # Seuils de confiance : sous le seuil, l'app retomberait sur « calme » (animation neutre).
    seuils = {}
    if all(c is not None for c in confs):
        for t in (0.2, 0.3, 0.4, 0.5):
            sel = [i for i in range(len(rows)) if confs[i] >= t]
            seuils[str(t)] = {
                "couverture": round(100 * len(sel) / len(rows)),
                "precision": round(100 * sum(preds[i] == rows[i]["label"] for i in sel) / len(sel)) if sel else None,
            }
    return {
        "nom": name,
        "global": acc(lambda r: True),
        "fr": acc(lambda r: r["lang"] == "fr"),
        "en": acc(lambda r: r["lang"] == "en"),
        "fautes": acc(lambda r: r["typo"] == "1"),
        "latence_ms": round(statistics.median(lat)),
        "rappel": recall,
        "erreurs": errors,
        "confiance_moy": round(statistics.mean(confs), 2) if seuils else None,
        "seuils": seuils,
        **meta,
    }


def report(results):
    out = ["# Résultats benchmark émotions\n",
           "| Modèle | Global | FR | EN | Avec fautes | Latence médiane | Taille | Remarque |",
           "|---|---|---|---|---|---|---|---|"]
    for r in sorted(results, key=lambda r: -r["global"]):
        out.append(f"| {r['nom']} | {r['global']} % | {r['fr']} % | {r['en']} % | {r['fautes']} % | "
                   f"{r['latence_ms']} ms | {r['taille_mo']} Mo | {r['remarque']} |")
    out += ["\n## Rappel par émotion\n", "| Modèle | " + " | ".join(LABELS) + " |",
            "|---|" + "---|" * len(LABELS)]
    for r in results:
        out.append(f"| {r['nom']} | " + " | ".join(f"{r['rappel'][l]} %" for l in LABELS) + " |")
    with_conf = [r for r in results if r["seuils"]]
    if with_conf:
        out += ["\n## Confiance (candidats « choix typé »)\n",
                "Précision sur les messages dont la confiance dépasse le seuil, et part des messages concernés.\n",
                "| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |", "|---|---|---|---|---|---|"]
        for r in with_conf:
            cells = [f"{v['precision']} % sur {v['couverture']} %" for v in r["seuils"].values()]
            out.append(f"| {r['nom']} | {r['confiance_moy']} | " + " | ".join(cells) + " |")
    for r in results:
        out.append(f"\n## Erreurs : {r['nom']}\n")
        for text, gold, pred in r["erreurs"]:
            out.append(f"- « {text} » → attendu **{gold}**, prédit **{pred}**")
    return "\n".join(out) + "\n"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--only", nargs="*")
    ap.add_argument("--skip", nargs="*", default=[])
    ap.add_argument("--workers", type=int, default=1, help="appels parallèles")
    ap.add_argument("--out", default="results", help="nom des fichiers de sortie (sans extension)")
    args = ap.parse_args()

    rows = list(csv.DictReader(open(HERE / "dataset.csv", encoding="utf-8")))
    names = args.only or [n for n in CANDIDATES if n not in args.skip]
    results = []
    for name in names:
        print(f"→ {name}")
        try:
            run, meta = CANDIDATES[name]()
        except Exception as e:  # noqa: BLE001
            print(f"  ignoré : {e}")
            continue
        r = evaluate(name, run, meta, rows, args.workers)
        print(f"  global {r['global']} %  FR {r['fr']} %  EN {r['en']} %  {r['latence_ms']} ms")
        results.append(r)

    if results:
        import json
        import platform

        (HERE / f"{args.out}.md").write_text(report(results), encoding="utf-8")
        env = {"python": platform.python_version(), "machine": platform.machine(),
               "system": platform.platform(), "processor": platform.processor()}
        (HERE / f"{args.out}.json").write_text(
            json.dumps({"env": env, "results": results}, ensure_ascii=False, indent=2), encoding="utf-8")
        print(f"\nRapport : {args.out}.md et {args.out}.json")


if __name__ == "__main__":
    main()
