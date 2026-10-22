# Protocole round 2 — suite (après le bilan intermédiaire)

À l'agent : merci pour le bilan intermédiaire, rigoureux et bien documenté. Ce document
**remplace les phases 2 à 4** de `PROTOCOLE.md`. Tout le reste (règles d'étanchéité, md5,
livrables, critères de qualité) reste valable.

## 0. Ce qui change et pourquoi

1. **Gemma est le seul candidat qui passe la qualité** (85,0 % global, 80,0 % avec fautes).
   La suite se concentre sur lui. MiniLM est gardé uniquement comme point de comparaison de taille.
   **E5 est abandonné** (qualité insuffisante, confiance inexploitable à 0,01).
2. **Les 1 270 Mo de Gemma sont du FP32** (308 M paramètres × 4 octets). Le budget de 150 Mo vise
   la version quantifiée. L'écart réel à combler est donc bien plus petit que « 8,5× ».
3. **Nouveaux scripts** (remplacent les tiens, ton correctif `device="cpu"` est intégré) :
   - `prune_vocab.py` : Wikipédia n'est plus pris « dès 1 occurrence » mais à partir de
     **3 occurrences** (`--wiki-min-count`), ce qui élimine noms propres rares et mots étrangers ;
     les caractères isolés de secours ne viennent plus de Wikipédia (qui contient tous les alphabets).
     La validation sépare **Wikipédia FR/EN** (critère) et **Wikipédia tout** (information).
   - `quantize_onnx.py` : nouvelle option `--q4` (poids 4 bits par blocs, y compris la table
     d'embeddings), via ONNX Runtime (`MatMulNBits` + `GatherBlockQuantized`).
4. **Ne pas relancer avec `--wiki 50000`** : ton analyse est juste, le `cos_min` venait d'un
   toponyme turc. Le nouveau critère ne porte que sur le FR/EN.

## 1. Source de Gemma

Mourad va accepter la licence de `google/embeddinggemma-300m` et te fournir un token.
- **Si tu as le token** : utilise `google/embeddinggemma-300m` et refais `gemma-proto` / `gemma-fewshot`
  de la phase 1 avec ce modèle (`--out r2-origine-google`), pour vérifier l'égalité avec le miroir.
- **Sinon** : garde `unsloth/embeddinggemma-300m`, et note-le comme risque résiduel.

Dans les deux cas, la **licence Gemma** s'applique (un miroir ne la change pas) : relève dans la
synthèse ses conditions d'usage commercial et de redistribution dans une app.

Dans les commandes ci-dessous, `GEMMA` désigne la source retenue.

## 2. Phase 2 bis — Élagage

```bash
python prune_vocab.py --model GEMMA --out pruned/gemma-m3 --wiki 20000 --wiki-min-count 3
python prune_vocab.py --model GEMMA --out pruned/gemma-m10 --wiki 20000 --wiki-min-count 10
python prune_vocab.py --model sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2 \
       --out pruned/minilm-m3 --wiki 20000 --wiki-min-count 3
```

Deux seuils pour Gemma afin de mesurer le compromis taille / fidélité.

**Critère de validité** (dans `pruning.json`) : sur `messages_courts` **et** `wikipedia_inedit_fr_en`,
cosinus moyen ≥ 0,999 et minimum ≥ 0,99. Le groupe `wikipedia_inedit_tout` est informatif.
Le script affiche un avertissement si le critère n'est pas atteint.

Puis la précision des versions élaguées (PyTorch) :

```bash
python bench.py --only none --out r2-elague --local \
  gemma-m3=pruned/gemma-m3 gemma-m10=pruned/gemma-m10 minilm-m3=pruned/minilm-m3
```

Attendu : à ±1 point des modèles d'origine (`r2-origine`).

## 3. Phase 3 bis — Quantification

```bash
python quantize_onnx.py --model pruned/gemma-m3 --config arm64 --q4
python quantize_onnx.py --model pruned/gemma-m10 --config arm64 --q4
python quantize_onnx.py --model GEMMA --config arm64 --q4          # Gemma NON élagué, pour chiffrer le gain
python quantize_onnx.py --model pruned/minilm-m3 --config arm64
```

Chaque `onnx_report.json` donne tailles, latences et cosinus FP32 ↔ INT8 ↔ Q4.
Si `q4_portee` indique « MatMul seulement », le noter (la table d'embeddings reste alors en FP32).

Puis la précision de toutes les versions embarquées :

```bash
python bench.py --only none --out r2-embarque --local \
  gemma-m3-int8=pruned/gemma-m3@onnx/model_qint8_arm64.onnx \
  gemma-m3-q4=pruned/gemma-m3@onnx/model_q4.onnx \
  gemma-m10-int8=pruned/gemma-m10@onnx/model_qint8_arm64.onnx \
  gemma-m10-q4=pruned/gemma-m10@onnx/model_q4.onnx \
  minilm-m3-int8=pruned/minilm-m3@onnx/model_qint8_arm64.onnx
```

(Ajouter `gemma-orig-q4=<dossier models/…>@onnx/model_q4.onnx` pour le Gemma non élagué :
le dossier exact est indiqué dans son `onnx_report.json`.)

**Si l'export ONNX de Gemma échoue** : noter l'erreur complète, essayer une fois avec
`pip install -U optimum "optimum[onnxruntime]"`, puis s'arrêter et rapporter. Ne pas passer par
Ollama/GGUF pour la décision : le format retenu pour l'app est ONNX Runtime.

## 4. Phase 4 — Seuil de confiance

Sur le meilleur Gemma embarqué (qualité ≥ critères, taille ≤ 150 Mo) :

```bash
python bench.py --dataset dev_v1.csv --only none --out r2-dev-seuils --local <spec du meilleur>
```

Choisir le seuil sur `dev_v1.csv`, puis rapporter son effet sur `test_v2.csv` (section
« Confiance » de `r2-embarque.md`).

## 5. Synthèse finale (`SYNTHESE.md`)

En plus des éléments du §9 de `PROTOCOLE.md` :

- **Tableau de décision** : pour chaque variante Gemma (origine / m3 / m10 × FP32 / INT8 / Q4) :
  Global, Avec fautes, rappel `energie` / `suspense`, taille du fichier, latence, cosinus vs FP32.
- **Le plus petit Gemma qui passe les critères** et sa taille exacte.
- Le gain chiffré de l'élagage (non élagué vs élagué, à format égal).
- MiniLM-m3 INT8 : taille et qualité, comme référence pour un futur clavier.
- La licence Gemma (§1).

## 6. Critères de décision (inchangés sauf précision)

- Qualité sur `test_v2.csv` : Global ≥ 80 %, Avec fautes ≥ 70 %, `energie` et `suspense` ≥ 70 %.
- **App : fichier modèle ≤ 150 Mo**, latence ≤ 50 ms sur le M1 (un téléphone milieu de gamme
  sera 3 à 5 fois plus lent : le noter).
- Entre deux variantes dans la marge d'incertitude (±5 points global, ±10 points fautes) :
  la plus petite gagne.
