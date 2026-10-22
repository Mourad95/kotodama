# Protocole de test — round 2 — détection d'émotions (Texte Vivant)

Ce document s'adresse à l'agent qui exécute le benchmark. Suis les étapes dans l'ordre et rends
les livrables listés au §9. En cas de doute, privilégie la rigueur à la vitesse et documente.

## 1. Contexte et objectif

Le round 1 (70 messages) a validé l'architecture **« choix typé »** : un modèle d'embeddings
reçoit les 7 émotions décrites dans `prototypes.json` (description + exemples) et renvoie un choix,
des probabilités et une confiance. Il a dépassé les classifieurs figés d'environ 30 points.

Trois limites du round 1 motivent ce round 2 :
1. **Trop peu de messages** : 6 messages avec fautes (1 message = 17 points) ne permettaient pas de conclure.
2. **Pas d'élagage du vocabulaire** : les modèles multilingues gardaient ~250 000 morceaux de mots pour
   des dizaines de langues, qui pèsent l'essentiel du modèle. Les tailles mesurées étaient donc surévaluées.
3. **Risque de surapprentissage** : les pistes d'amélioration venaient des erreurs sur le jeu de test.

Questions à trancher :
1. Après élagage FR+EN, quelle taille réelle pour **EmbeddingGemma** et **MiniLM** (INT8) ?
2. L'élagage préserve-t-il les embeddings (cosinus ≈ 1) et la précision ?
3. Sur un jeu de test plus grand et indépendant, quel modèle et quelle variante (`proto` / `fewshot`) gagnent ?
4. Quel seuil de confiance retenir ?

## 2. Contenu du dossier

| Fichier | Rôle | Modifiable ? |
|---|---|---|
| `test_v2.csv` | **jeu de test** : 301 messages (196 FR / 105 EN), 70 avec fautes, colonne `alt` | **Non. Ne jamais l'ouvrir pour ajuster quoi que ce soit.** |
| `dev_v1.csv` | jeu du round 1, devenu **jeu de développement** (70 messages) | Non |
| `prototypes.json` | 16 exemples par émotion (FR + EN, argot, majuscules, fautes) | Non pendant les phases 1 à 5 |
| `slang_lexicon.txt` | lexique argot / SMS / emojis pour le corpus d'élagage | Non |
| `ANNOTATION.md` | guide d'annotation (règles, colonne `alt`, fautes) | — |
| `bench.py` | benchmark (précision stricte et tolérante, confiance, latence, taille) | Correctifs de compatibilité seulement, à signaler |
| `prune_vocab.py` | élagage du vocabulaire FR+EN + validation automatique | Idem |
| `quantize_onnx.py` | export ONNX + INT8 + mesure taille/latence | Idem |
| `data_src/` | sources texte du jeu de test | — |

### Règles d'étanchéité (absolues)

- `test_v2.csv` ne sert **qu'à mesurer**. Jamais pour élaguer, régler un seuil, ni enrichir les prototypes.
- Ne jamais copier un message de `test_v2.csv` dans `prototypes.json` ou dans un corpus.
- Les réglages (seuil de confiance, choix de variante) se décident sur `dev_v1.csv` **ou** sont rapportés
  sur le test avec la mention « choisi a posteriori ».
- `prune_vocab.py` refuse tout fichier dont le nom commence par `test_v2`. Ne pas contourner.

Vérifier les empreintes au début et à la fin avec `md5sum -c MD5SUMS`
(macOS : comparer `md5 test_v2.csv dev_v1.csv prototypes.json` au fichier `MD5SUMS`). Elles doivent être identiques.

## 3. Prérequis

- Python 3.10+, environ 15 Go de disque libre, connexion internet.
- Compte Hugging Face avec la licence d'**EmbeddingGemma** acceptée sur
  https://huggingface.co/google/embeddinggemma-300m (modèle « gated »).

## 4. Installation

```bash
cd emotion-bench
python3 -m venv .venv
source .venv/bin/activate            # Windows : .venv\Scripts\activate
pip install --upgrade pip
pip install torch transformers "sentence-transformers[onnx]" onnxruntime "optimum[onnxruntime]" \
            scikit-learn sentencepiece protobuf huggingface_hub datasets tokenizers
huggingface-cli login
```

Contrôle : `python -c "import torch, sentence_transformers, onnxruntime, datasets, sklearn; print('ok')"`.
Noter les versions exactes (`pip freeze > versions.txt`) : c'est un livrable.

## 5. Phase 1 — Référence sur le nouveau jeu de test (modèles d'origine)

```bash
python bench.py --only gemma-proto gemma-fewshot minilm-proto minilm-fewshot e5small-proto e5small-fewshot --out r2-origine
python bench.py --only tabularisai --out r2-reference
```

`tabularisai` sert de témoin (licence non commerciale, donc jamais retenu).
Les autres références du round 1 (goemotions, xlmemo, zeroshot) ne sont plus nécessaires.

## 6. Phase 2 — Élagage du vocabulaire FR + EN

```bash
python prune_vocab.py --model sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2 --out pruned/minilm --wiki 20000
python prune_vocab.py --model google/embeddinggemma-300m --out pruned/gemma --wiki 20000
python prune_vocab.py --model intfloat/multilingual-e5-small --out pruned/e5small --wiki 20000
```

Chaque commande télécharge 20 000 articles Wikipédia FR et 20 000 EN (en streaming), ajoute le
lexique d'argot, `dev_v1.csv` et les prototypes, garde les morceaux de mots utilisés, découpe la
table d'embeddings puis **valide** : le fichier `pruned/<nom>/pruning.json` contient, pour des
messages courts et pour 2 000 lignes Wikipédia jamais vues par l'élagage, le pourcentage de textes
tokenisés à l'identique et le cosinus entre embeddings d'origine et élagués.

**Critère de validité de l'élagage** : cosinus moyen ≥ 0,999 et minimum ≥ 0,99 sur les deux groupes.
Sinon : relancer avec `--wiki 50000`, et rapporter les deux résultats.

Si l'élagage d'un modèle échoue (structure de tokenizer inattendue, erreur de chargement…),
noter l'erreur complète et passer au suivant. Ne pas modifier le modèle à la main.

Puis mesurer la précision des modèles élagués (PyTorch) :

```bash
python bench.py --only none --local minilm-elague=pruned/minilm gemma-elague=pruned/gemma e5-elague=pruned/e5small --out r2-elague
```

Attendu : précision identique aux modèles d'origine à ±1 point. Un écart plus grand est une anomalie à investiguer.

## 7. Phase 3 — ONNX INT8 et mesure embarquée

Pour chaque modèle élagué valide :

```bash
python quantize_onnx.py --model pruned/minilm --config arm64
python quantize_onnx.py --model pruned/gemma --config arm64
python quantize_onnx.py --model pruned/e5small --config arm64
```

(`arm64` correspond aux puces des téléphones et des Mac M1/M2/M3. Sur une machine x86 de test,
utiliser `--config avx2` et le noter.) Chaque commande écrit `pruned/<nom>/onnx_report.json` :
tailles FP32 et INT8, latences, et cosinus FP32 ↔ INT8.

Puis la précision des versions INT8 :

```bash
python bench.py --only none --out r2-int8 --local \
  minilm-int8=pruned/minilm@onnx/model_qint8_arm64.onnx \
  gemma-int8=pruned/gemma@onnx/model_qint8_arm64.onnx \
  e5-int8=pruned/e5small@onnx/model_qint8_arm64.onnx
```

**Pour comparaison**, quantifier aussi MiniLM **non élagué** (même commande avec
`--model sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2`) afin de chiffrer le gain de l'élagage.

Si l'export ONNX de Gemma échoue : noter l'erreur exacte, rapporter la taille FP32 élaguée et une
estimation INT8 (nombre de paramètres × 1 octet), et indiquer que la mesure reste à faire.

## 8. Phase 4 — Seuil de confiance (sur le jeu de développement)

Pour les 2 meilleures combinaisons de la phase 3 :

```bash
python bench.py --dataset dev_v1.csv --only none --out r2-dev-seuils --local <mêmes specs que ci-dessus>
```

Choisir le seuil sur `dev_v1.csv` (meilleur compromis : précision ≥ 85 % en couvrant le plus de messages),
**puis** rapporter ce que donne ce même seuil sur `test_v2.csv` (section « Confiance » de `r2-int8.md`).
Préciser que le jeu de dev est petit (70 messages) : le seuil est indicatif.

## 9. Livrables

1. Tous les `r2-*.md`, `r2-*.json`, les `pruned/*/pruning.json` et `pruned/*/onnx_report.json`
   (pas les poids des modèles).
2. `versions.txt` (`pip freeze`) et les empreintes md5 de début et de fin.
3. **`SYNTHESE.md`** contenant :
   - environnement : OS, puce, Python, date ;
   - **tableau principal** (jeu `test_v2.csv`) pour chaque combinaison modèle × élagage × format :
     Global, Tolérant, FR, EN, Avec fautes, Sans fautes, rappel `energie`, rappel `suspense`,
     taille, latence, licence ;
   - tableau de l'élagage : vocabulaire avant/après, paramètres avant/après, cosinus moyen et minimum,
     % de tokens identiques (messages courts et Wikipédia inédit) ;
   - gain de l'élagage sur MiniLM (INT8 non élagué vs INT8 élagué) ;
   - seuil de confiance retenu sur dev et son effet sur le test ;
   - **intervalles de confiance** : avec 301 messages, l'incertitude est d'environ ±5 points au global ;
     avec 70 messages fautifs, d'environ ±10 points. Un écart inférieur à ces marges n'est pas une différence ;
   - les 10 erreurs les plus révélatrices du meilleur candidat, classées par cause probable
     (argot, faute, sarcasme, émotion mixte, annotation discutable…) ;
   - annotations de `test_v2.csv` qui semblent discutables (lister, ne pas modifier) ;
   - pistes pour enrichir `prototypes.json`, **déduites du jeu de dev uniquement** ;
   - recommandation finale en une phrase pour l'app (≤ 150 Mo) et pour le clavier (taille la plus petite possible),
     avec les chiffres qui la justifient ;
   - tout écart à ce protocole et pourquoi.

## 10. Problèmes courants

| Symptôme | Action |
|---|---|
| `401` / `gated repo` sur EmbeddingGemma | Accepter la licence, refaire `huggingface-cli login` |
| `datasets` ne trouve pas `wikimedia/wikipedia` | Vérifier la version de `datasets` ; sinon utiliser `--wiki 0` et le signaler (validation plus faible) |
| Élagage : `KeyError` sur un id | Noter la trace complète, passer au modèle suivant |
| `export_dynamic_quantized_onnx_model` introuvable | `pip install -U "sentence-transformers[onnx]"` |
| Export ONNX de Gemma impossible | Voir fin du §7 |
| Mémoire insuffisante | Lancer les candidats un par un |

## 11. Critères de décision (pour information)

- **Qualité** : Global ≥ 80 %, Avec fautes ≥ 70 %, rappel `energie` et `suspense` ≥ 70 % sur `test_v2.csv`.
- **Élagage valide** : cosinus moyen ≥ 0,999, précision identique à ±1 point.
- **App** : INT8 ≤ 150 Mo. **Clavier** : le plus petit modèle qui passe le seuil de qualité, cible ≤ 40 Mo.
- **Confiance** : au seuil retenu, précision ≥ 85 % sur au moins 70 % des messages.
- En cas d'égalité dans les marges d'incertitude : le plus petit modèle gagne, puis la licence la plus libre.
