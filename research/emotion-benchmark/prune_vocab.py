"""Élagage du vocabulaire d'un modèle d'embeddings (sentence-transformers) au français + anglais.

Les modèles multilingues embarquent ~250 000 morceaux de mots pour des dizaines de langues.
La table d'embeddings correspondante représente l'essentiel de leur poids. On ne garde que les
morceaux réellement utilisés par un corpus FR+EN (+ argot, emojis, caractères isolés de secours),
on découpe la table d'embeddings en conséquence, puis on vérifie que les embeddings produits
restent identiques à ceux du modèle d'origine.

Formats de tokenizer gérés : Unigram (XLM-R : MiniLM multilingue, E5) et BPE avec byte fallback
(Gemma / EmbeddingGemma).

Usage :
    python prune_vocab.py --model sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2 \
        --out pruned/minilm --wiki 20000
    python prune_vocab.py --model google/embeddinggemma-300m --out pruned/gemma --wiki 20000

Le corpus d'élagage ne doit JAMAIS contenir test_v2.csv (voir PROTOCOLE.md).
"""
from __future__ import annotations

import argparse
import copy
import csv
import json
import shutil
import unicodedata
from collections import Counter
from pathlib import Path

HERE = Path(__file__).parent
SLOW_TOKENIZER_FILES = ["sentencepiece.bpe.model", "tokenizer.model", "spiece.model"]


# ------------------------------------------------------------------ corpus
def corpus_from_files(paths: list[str]) -> list[str]:
    lines: list[str] = []
    for p in paths:
        p = Path(p)
        if p.suffix == ".csv":
            lines += [r["text"] for r in csv.DictReader(open(p, encoding="utf-8"))]
        elif p.suffix == ".json":
            data = json.loads(p.read_text(encoding="utf-8"))
            for v in data.values():
                lines += [v["description"], *v["examples"]]
        else:
            lines += [ln.strip() for ln in p.read_text(encoding="utf-8").splitlines() if ln.strip()]
    return lines


def corpus_from_wikipedia(n_articles: int, langs=("fr", "en"), max_chars=4000) -> list[str]:
    from datasets import load_dataset

    lines: list[str] = []
    for lang in langs:
        ds = load_dataset("wikimedia/wikipedia", f"20231101.{lang}", split="train", streaming=True)
        for i, art in enumerate(ds):
            if i >= n_articles:
                break
            text = art["text"][:max_chars]
            lines += [ln for ln in text.split("\n") if ln.strip()]
    return lines


# ------------------------------------------------------------------ token selection
FR_EN_CHARS = set("abcdefghijklmnopqrstuvwxyzàâäæçéèêëîïôöœùûüÿ")


def is_fr_en(line: str) -> bool:
    """Heuristique : toutes les lettres appartiennent à l'alphabet français/anglais."""
    letters = [c for c in line.lower() if c.isalpha()]
    return bool(letters) and all(c in FR_EN_CHARS for c in letters)

def fallback_chars(corpus: list[str]) -> set[str]:
    """Caractères à garder comme morceaux isolés, pour que tout mot FR/EN reste découpable.
    corpus = messages courts uniquement (pas Wikipédia, qui contient tous les alphabets du monde)."""
    chars = set("".join(corpus))
    for cp in range(0x20, 0x250):  # ASCII + Latin-1 + Latin étendu (accents, œ, ß…)
        chars.add(chr(cp))
    chars |= set("’‘“”«»…–—€£•°✓✔❤♥☺☹")
    for lo, hi in ((0x1F300, 0x1FAFF), (0x2600, 0x27BF), (0xFE00, 0xFE0F), (0x200D, 0x200D)):  # emojis
        chars |= {chr(cp) for cp in range(lo, hi + 1)}
    return {c for c in chars if not unicodedata.category(c).startswith("C") or c == "\u200d"}


def used_ids(tokenizer, corpus: list[str], batch=1000) -> Counter:
    counts: Counter = Counter()
    for i in range(0, len(corpus), batch):
        for enc in tokenizer.encode_batch(corpus[i:i + batch]):
            counts.update(enc.ids)
    return counts


def single_char_ids(tj: dict, chars: set[str]) -> set[int]:
    model = tj["model"]
    keep = set()
    if model["type"] == "Unigram":
        for i, (piece, _score) in enumerate(model["vocab"]):
            core = piece[1:] if piece.startswith("▁") else piece
            if piece == "▁" or (len(core) == 1 and core in chars):
                keep.add(i)
    elif model["type"] == "BPE":
        for piece, i in model["vocab"].items():
            core = piece[1:] if piece.startswith("▁") else piece
            if piece == "▁" or (len(core) == 1 and core in chars) or (piece.startswith("<0x") and len(piece) == 6):
                keep.add(i)
    return keep


def special_ids(tj: dict) -> set[int]:
    return {t["id"] for t in tj.get("added_tokens", [])}


def bpe_merges(model: dict) -> list[tuple[str, str]]:
    out = []
    for m in model.get("merges", []):
        a, b = (m.split(" ", 1) if isinstance(m, str) else m)
        out.append((a, b))
    return out


def bpe_closure(tj: dict, keep: set[int]) -> set[int]:
    """En BPE, un morceau gardé n'est atteignable que si les morceaux qui le composent le sont aussi."""
    model = tj["model"]
    vocab = model["vocab"]
    inv = {i: p for p, i in vocab.items()}
    produced: dict[str, tuple[str, str]] = {}
    for a, b in bpe_merges(model):
        produced.setdefault(a + b, (a, b))
    stack = [inv[i] for i in keep if i in inv]
    kept = {inv[i] for i in keep if i in inv}
    while stack:
        p = stack.pop()
        if p in produced:
            for part in produced[p]:
                if part not in kept and part in vocab:
                    kept.add(part)
                    stack.append(part)
    return keep | {vocab[p] for p in kept}


# ------------------------------------------------------------------ tokenizer.json rewrite
def remap_ids_in(obj, old2new: dict[int, int]):
    """Remappe les ids de tokens spéciaux dans post_processor (TemplateProcessing, RobertaProcessing…)."""
    if isinstance(obj, dict):
        for k, v in obj.items():
            if k == "ids" and isinstance(v, list):
                obj[k] = [old2new[i] for i in v]
            elif k in ("sep", "cls") and isinstance(v, list) and len(v) == 2 and isinstance(v[1], int):
                obj[k] = [v[0], old2new[v[1]]]
            else:
                remap_ids_in(v, old2new)
    elif isinstance(obj, list):
        for v in obj:
            remap_ids_in(v, old2new)


def prune_tokenizer_json(tj: dict, keep: set[int]) -> tuple[dict, dict[int, int]]:
    tj = copy.deepcopy(tj)
    model = tj["model"]
    keep_sorted = sorted(keep)
    old2new = {old: new for new, old in enumerate(keep_sorted)}

    if model["type"] == "Unigram":
        model["vocab"] = [model["vocab"][i] for i in keep_sorted]
        if model.get("unk_id") is not None:
            model["unk_id"] = old2new[model["unk_id"]]
    elif model["type"] == "BPE":
        inv = {i: p for p, i in model["vocab"].items()}
        kept_pieces = {inv[i] for i in keep_sorted if i in inv}
        model["vocab"] = {inv[i]: old2new[i] for i in keep_sorted if i in inv}
        merges = [(a, b) for a, b in bpe_merges(model) if a in kept_pieces and b in kept_pieces and a + b in kept_pieces]
        as_strings = bool(tj["model"].get("merges")) and isinstance(tj["model"]["merges"][0], str)
        model["merges"] = [f"{a} {b}" for a, b in merges] if as_strings else [[a, b] for a, b in merges]
    else:
        raise ValueError(f"Type de tokenizer non géré : {model['type']}")

    for t in tj.get("added_tokens", []):
        t["id"] = old2new[t["id"]]
    if tj.get("post_processor"):
        remap_ids_in(tj["post_processor"], old2new)
    return tj, old2new


def prune_tokenizer_config(cfg: dict, old2new: dict[int, int]) -> dict:
    cfg = copy.deepcopy(cfg)
    if "added_tokens_decoder" in cfg:
        cfg["added_tokens_decoder"] = {
            str(old2new[int(k)]): v for k, v in cfg["added_tokens_decoder"].items() if int(k) in old2new
        }
    return cfg


def select_keep(tj: dict, tokenizer, corpus: list[str], min_count: int,
                wiki: list[str] | None = None, wiki_min_count: int = 3) -> set[int]:
    """corpus : messages courts (argot, prototypes, dev) → tout morceau vu au moins min_count fois.
    wiki : texte encyclopédique → seuls les morceaux vus au moins wiki_min_count fois (élimine noms
    propres rares et mots étrangers, qui gonflent le vocabulaire sans servir à des messages d'ados).
    Les caractères isolés de secours viennent des messages courts + latin + emojis, pas de Wikipédia."""
    counts = used_ids(tokenizer, corpus)
    keep = {i for i, c in counts.items() if c >= min_count}
    if wiki:
        keep |= {i for i, c in used_ids(tokenizer, wiki).items() if c >= wiki_min_count}
    keep |= special_ids(tj)
    keep |= single_char_ids(tj, fallback_chars(corpus))
    if tj["model"]["type"] == "BPE":
        keep = bpe_closure(tj, keep)
    return keep


# ------------------------------------------------------------------ model surgery
def prune_model(model_id: str, out: Path, corpus: list[str], min_count: int,
                wiki: list[str] | None = None, wiki_min_count: int = 3):
    import torch
    from sentence_transformers import SentenceTransformer
    from tokenizers import Tokenizer

    st = SentenceTransformer(model_id, device="cpu")  # CPU : évite l'échec MPS + safetensors sur Mac
    tmp = out.with_name(out.name + "_orig")
    if tmp.exists():
        shutil.rmtree(tmp)
    st.save(str(tmp))

    tj = json.loads((tmp / "tokenizer.json").read_text(encoding="utf-8"))
    tokenizer = Tokenizer.from_file(str(tmp / "tokenizer.json"))
    old_vocab = tokenizer.get_vocab_size(with_added_tokens=True)
    keep = select_keep(tj, tokenizer, corpus, min_count, wiki, wiki_min_count)
    # Certains tokenizers déclarent plus d'ids que la table d'embeddings n'a de lignes
    # (EmbeddingGemma : 262 145 ids pour 262 144 lignes) : on écarte ces ids orphelins.
    n_rows = st[0].auto_model.get_input_embeddings().weight.shape[0]
    orphans = {i for i in keep if i >= n_rows}
    keep -= orphans
    tj["added_tokens"] = [t for t in tj.get("added_tokens", []) if t["id"] < n_rows]
    new_tj, old2new = prune_tokenizer_json(tj, keep)
    keep_sorted = sorted(keep)
    print(f"Vocabulaire : {old_vocab} → {len(keep_sorted)} morceaux ({100 * len(keep_sorted) / old_vocab:.1f} %)")

    auto = st[0].auto_model
    emb = auto.get_input_embeddings()
    params_before = sum(p.numel() for p in st.parameters())
    idx = torch.tensor(keep_sorted, dtype=torch.long)
    emb.weight = torch.nn.Parameter(emb.weight.data[idx].clone())
    emb.num_embeddings = len(keep_sorted)
    if getattr(emb, "padding_idx", None) is not None:
        emb.padding_idx = old2new[emb.padding_idx]
    cfg = auto.config
    for holder in (cfg, getattr(cfg, "text_config", None)):
        if holder is None:
            continue
        if hasattr(holder, "vocab_size"):
            holder.vocab_size = len(keep_sorted)
        for attr in ("pad_token_id", "bos_token_id", "eos_token_id", "unk_token_id"):
            v = getattr(holder, attr, None)
            if isinstance(v, int) and v in old2new:
                setattr(holder, attr, old2new[v])
            elif isinstance(v, list):
                setattr(holder, attr, [old2new[x] for x in v if x in old2new])
    params_after = sum(p.numel() for p in st.parameters())

    if out.exists():
        shutil.rmtree(out)
    st.save(str(out))
    (out / "tokenizer.json").write_text(json.dumps(new_tj, ensure_ascii=False), encoding="utf-8")
    tc_path = out / "tokenizer_config.json"
    if tc_path.exists():
        tc = json.loads(tc_path.read_text(encoding="utf-8"))
        tc_path.write_text(json.dumps(prune_tokenizer_config(tc, old2new), ensure_ascii=False, indent=2),
                           encoding="utf-8")
    for name in SLOW_TOKENIZER_FILES:
        (out / name).unlink(missing_ok=True)
    (out / "pruning.json").write_text(json.dumps({
        "source": model_id, "vocab_avant": old_vocab, "vocab_apres": len(keep_sorted),
        "params_avant": params_before, "params_apres": params_after, "corpus_lignes": len(corpus),
        "wiki_lignes": len(wiki or []), "min_count": min_count, "wiki_min_count": wiki_min_count,
    }, indent=2), encoding="utf-8")
    print(f"Paramètres : {params_before / 1e6:.1f} M → {params_after / 1e6:.1f} M")
    return tmp


def validate(orig_dir: Path, pruned_dir: Path, texts: list[str]) -> dict:
    """Compare tokens et embeddings du modèle d'origine et du modèle élagué."""
    import numpy as np
    from sentence_transformers import SentenceTransformer
    from tokenizers import Tokenizer

    t_old = Tokenizer.from_file(str(orig_dir / "tokenizer.json"))
    t_new = Tokenizer.from_file(str(pruned_dir / "tokenizer.json"))
    same_tokens = sum(a.tokens == b.tokens for a, b in zip(t_old.encode_batch(texts), t_new.encode_batch(texts)))

    m_old = SentenceTransformer(str(orig_dir), device="cpu")
    m_new = SentenceTransformer(str(pruned_dir), device="cpu")
    e_old = m_old.encode(texts, normalize_embeddings=True, batch_size=64)
    e_new = m_new.encode(texts, normalize_embeddings=True, batch_size=64)
    cos = (e_old * e_new).sum(axis=1)
    res = {
        "textes": len(texts),
        "tokens_identiques_pct": round(100 * same_tokens / len(texts), 2),
        "cosinus_moyen": round(float(cos.mean()), 6),
        "cosinus_min": round(float(cos.min()), 6),
        "texte_le_plus_eloigne": texts[int(np.argmin(cos))],
    }
    return res


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--model", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--wiki", type=int, default=20000, help="articles Wikipédia par langue (0 = aucun)")
    ap.add_argument("--corpus", nargs="*", default=[str(HERE / "slang_lexicon.txt"), str(HERE / "dev_v1.csv"),
                                                     str(HERE / "prototypes.json")])
    ap.add_argument("--min-count", type=int, default=1, help="seuil pour les messages courts")
    ap.add_argument("--wiki-min-count", type=int, default=3, help="seuil pour Wikipédia")
    ap.add_argument("--validate", nargs="*", default=[str(HERE / "dev_v1.csv"), str(HERE / "prototypes.json"),
                                                       str(HERE / "slang_lexicon.txt")])
    args = ap.parse_args()

    for p in args.corpus + args.validate:
        if Path(p).name.startswith("test_v2"):
            raise SystemExit("Interdit : le jeu de test ne doit pas servir à l'élagage ni à sa validation.")

    corpus = corpus_from_files(args.corpus)
    held_out: list[str] = []
    if args.wiki:
        print(f"Téléchargement de {args.wiki} articles Wikipédia FR et EN…")
        wiki = corpus_from_wikipedia(args.wiki)
        # 2 000 lignes mises de côté : jamais vues par l'élagage, elles mesurent la généralisation.
        step = max(1, len(wiki) // 2000)
        held_idx = set(range(0, len(wiki), step))
        held_out = [ln for i, ln in enumerate(wiki) if i in held_idx][:2000]
        wiki = [ln for i, ln in enumerate(wiki) if i not in held_idx]
    else:
        wiki = []
    print(f"Corpus : {len(corpus)} lignes courtes + {len(wiki)} lignes Wikipédia")

    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    orig_dir = prune_model(args.model, out, corpus, args.min_count, wiki, args.wiki_min_count)

    report = {"messages_courts": validate(orig_dir, out, corpus_from_files(args.validate))}
    if held_out:
        # Le critère porte sur le FR/EN : les lignes en écriture non latine ou pleines de
        # caractères étrangers (toponymes turcs, translittérations…) sont mesurées à part.
        fr_en = [ln for ln in held_out if is_fr_en(ln)]
        report["wikipedia_inedit_fr_en"] = validate(orig_dir, out, fr_en)
        report["wikipedia_inedit_tout"] = validate(orig_dir, out, held_out)
    meta = json.loads((out / "pruning.json").read_text(encoding="utf-8"))
    meta["validation"] = report
    (out / "pruning.json").write_text(json.dumps(meta, ensure_ascii=False, indent=2), encoding="utf-8")
    print(json.dumps(report, ensure_ascii=False, indent=2))
    critere = [report["messages_courts"]] + ([report["wikipedia_inedit_fr_en"]] if held_out else [])
    if min(r["cosinus_min"] for r in critere) < 0.99 or min(r["cosinus_moyen"] for r in critere) < 0.999:
        print("⚠️ Certains embeddings ont changé : vérifier le corpus d'élagage (voir texte_le_plus_eloigne).")


if __name__ == "__main__":
    main()
