"""Export ONNX + quantification INT8 d'un modèle sentence-transformers, puis mesure taille et latence.

Usage :
    python quantize_onnx.py --model pruned/minilm
    python quantize_onnx.py --model pruned/gemma --config arm64

Produit <model>/onnx/model.onnx (FP32), <model>/onnx/model_qint8_<config>.onnx (INT8) et,
avec --q4, <model>/onnx/model_q4.onnx (poids 4 bits par blocs : couches MatMul et table
d'embeddings), puis écrit <model>/onnx_report.json. Les candidats s'évaluent ensuite avec :
    python bench.py --only none --local nom=<model>@onnx/model_qint8_<config>.onnx
    python bench.py --only none --local nom=<model>@onnx/model_q4.onnx
"""
from __future__ import annotations

import argparse
import csv
import json
import statistics
import time
from pathlib import Path

HERE = Path(__file__).parent


def latency_ms(model, texts: list[str], warmup=5) -> float:
    for t in texts[:warmup]:
        model.encode([t])
    lat = []
    for t in texts:
        t0 = time.perf_counter()
        model.encode([t])
        lat.append((time.perf_counter() - t0) * 1000)
    return round(statistics.median(lat), 2)


def quantize_q4(path: Path, fp32, texts, e32, mb) -> dict:
    """Quantification 4 bits par blocs (MatMulNBits + GatherBlockQuantized d'ONNX Runtime).
    Si la table d'embeddings (Gather) ne peut pas être quantifiée, seules les MatMul le sont."""
    import onnx
    from onnxruntime.quantization.matmul_nbits_quantizer import MatMulNBitsQuantizer
    from sentence_transformers import SentenceTransformer

    src, dst = path / "onnx/model.onnx", path / "onnx/model_q4.onnx"
    model = onnx.load(str(src))
    note = "MatMul + Gather (embeddings)"
    try:
        q = MatMulNBitsQuantizer(model, block_size=32, is_symmetric=True, accuracy_level=4,
                                 op_types_to_quantize=("MatMul", "Gather"),
                                 quant_axes=(("MatMul", 0), ("Gather", 1)))
        q.process()
    except TypeError:
        model = onnx.load(str(src))
        q = MatMulNBitsQuantizer(model, block_size=32, is_symmetric=True, accuracy_level=4)
        q.process()
        note = "MatMul seulement (cette version d'ONNX Runtime ne quantifie pas les Gather)"
    big = q.model.model.ByteSize() > 1.9e9
    q.model.save_model_to_file(str(dst), use_external_data_format=big)

    q4 = SentenceTransformer(str(path), backend="onnx", device="cpu",
                             model_kwargs={"file_name": "onnx/model_q4.onnx", "provider": "CPUExecutionProvider"})
    e4 = q4.encode(texts, normalize_embeddings=True)
    cos = (e32 * e4).sum(axis=1)
    return {
        "taille_q4_mo": mb(dst),
        "latence_q4_ms": latency_ms(q4, texts),
        "cosinus_fp32_q4_moyen": round(float(cos.mean()), 4),
        "cosinus_fp32_q4_min": round(float(cos.min()), 4),
        "fichier_q4": "onnx/model_q4.onnx",
        "q4_portee": note,
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--model", required=True, help="dossier ou id d'un modèle sentence-transformers")
    ap.add_argument("--config", default="arm64", choices=["arm64", "avx2", "avx512", "avx512_vnni"],
                    help="arm64 = processeurs de téléphones et Mac M1/M2/M3")
    ap.add_argument("--q4", action="store_true", help="produire aussi une version 4 bits")
    args = ap.parse_args()

    from sentence_transformers import SentenceTransformer, export_dynamic_quantized_onnx_model

    texts = [r["text"] for r in csv.DictReader(open(HERE / "dev_v1.csv", encoding="utf-8"))][:50]
    path = Path(args.model)
    if not path.exists():  # id Hugging Face : on enregistre d'abord une copie locale
        path = HERE / "models" / args.model.replace("/", "__")
        SentenceTransformer(args.model, device="cpu").save(str(path))
    args.model = str(path)

    # CPU forcé : sur macOS, CoreML échoue sur les graphes quantifiés ; le CPU ARM est la cible réelle.
    fp32 = SentenceTransformer(args.model, backend="onnx", device="cpu", model_kwargs={"provider": "CPUExecutionProvider"})
    if not (path / "onnx/model.onnx").exists():
        fp32.save(str(path))
    export_dynamic_quantized_onnx_model(fp32, args.config, str(path))
    qname = f"onnx/model_qint8_{args.config}.onnx"
    int8 = SentenceTransformer(args.model, backend="onnx", device="cpu", model_kwargs={"file_name": qname, "provider": "CPUExecutionProvider"})

    e32 = fp32.encode(texts, normalize_embeddings=True)
    e8 = int8.encode(texts, normalize_embeddings=True)
    cos = (e32 * e8).sum(axis=1)

    def mb(p: Path) -> float:
        total = p.stat().st_size
        data = Path(str(p) + "_data")  # gros modèles : poids externes
        if data.exists():
            total += data.stat().st_size
        return round(total / 1e6, 1)

    report = {
        "modele": args.model,
        "config": args.config,
        "taille_fp32_mo": mb(path / "onnx/model.onnx"),
        "taille_int8_mo": mb(path / qname),
        "latence_fp32_ms": latency_ms(fp32, texts),
        "latence_int8_ms": latency_ms(int8, texts),
        "cosinus_fp32_int8_moyen": round(float(cos.mean()), 4),
        "cosinus_fp32_int8_min": round(float(cos.min()), 4),
        "fichier_int8": qname,
    }
    if args.q4:
        report.update(quantize_q4(path, fp32, texts, e32, mb))
    (path / "onnx_report.json").write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding="utf-8")
    print(json.dumps(report, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
