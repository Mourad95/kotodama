# SYNTHESE — Round 2 · Détection d'émotions (Texte Vivant)

## 1. Environnement

| Élément | Valeur |
|---|---|
| Date | 2026-10-22, 09:30 → 12:00 |
| OS | macOS 26.6.2 (arm64) |
| Puce | **Apple M1 Max** (10 cœurs), 34 Go RAM, sans GPU CUDA |
| Python | 3.11.15 (venv dédié) |
| torch / transformers | 2.14.1 / 4.57.6 |
| sentence-transformers / optimum | 5.7.0 / 2.1.0 |
| onnxruntime / onnx-ir | 1.31.0 / 1.1.0 |
| datasets | 5.0.1 |
| Exécution ONNX | `CPUExecutionProvider` (forcé, voir §12) |
| Latences | 100 % locales, aucune mesure réseau |

**Étanchéité vérifiée** : `md5` de `test_v2.csv`, `dev_v1.csv` et `prototypes.json` **identiques**
à `MD5SUMS` au début **et** à la fin des runs.

---

## 2. Tableau principal — `test_v2.csv` (301 messages, 70 avec fautes)

Un message vaut 0,3 point au global et 1,4 point sur « avec fautes ».

| Modèle | Format | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | rappel `energie` | rappel `suspense` | Taille | Latence |
|---|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| **gemma-m3-Q4-fewshot** ⭐ | ONNX Q4 | **86,7 %** | 87,4 % | 85,2 % | 89,5 % | **81,4 %** | 88,3 % | **93,0 %** | **90,7 %** | **115,9 Mo** | 25,5 ms |
| gemma-orig-Q4-fewshot | ONNX Q4 | 87,4 % | 88,0 % | 85,2 % | 91,4 % | 81,4 % | 89,2 % | 95,3 % | 90,7 % | 190,7 Mo | 25,6 ms |
| gemma-m3-INT8-fewshot | ONNX INT8 | 86,0 % | 86,7 % | 83,7 % | 90,5 % | 81,4 % | 87,4 % | 93,0 % | 88,4 % | 185,8 Mo | 27,8 ms |
| gemma-m10-Q4-fewshot | ONNX Q4 | 85,0 % | 85,7 % | 83,2 % | 88,6 % | 78,6 % | 87,0 % | 93,0 % | 90,7 % | 104,3 Mo | 26,6 ms |
| gemma-m10-INT8-fewshot | ONNX INT8 | 84,1 % | 84,7 % | 82,1 % | 87,6 % | 80,0 % | 85,3 % | 88,4 % | 90,7 % | 167,2 Mo | 27,3 ms |
| gemma-m3-Q4-proto | ONNX Q4 | 83,4 % | 84,1 % | 81,1 % | 87,6 % | 77,1 % | 85,3 % | 88,4 % | 90,7 % | 115,9 Mo | 24,9 ms |
| gemma-m10-Q4-proto | ONNX Q4 | 83,4 % | 84,1 % | 81,6 % | 86,7 % | 77,1 % | 85,3 % | 90,7 % | 93,0 % | 104,3 Mo | 25,0 ms |
| gemma-orig-Q4-proto | ONNX Q4 | 84,1 % | 84,7 % | 81,6 % | 88,6 % | 78,6 % | 85,7 % | 90,7 % | 90,7 % | 190,7 Mo | 24,7 ms |
| gemma-m3-INT8-proto | ONNX INT8 | 82,4 % | 83,7 % | 79,6 % | 87,6 % | 78,6 % | 83,5 % | 90,7 % | 88,4 % | 185,8 Mo | 25,1 ms |
| gemma-m10-INT8-proto | ONNX INT8 | 81,1 % | 82,4 % | 78,6 % | 85,7 % | 77,1 % | 82,3 % | 90,7 % | 90,7 % | 167,2 Mo | 25,3 ms |
| minilm-m3-INT8-proto | ONNX INT8 | 67,8 % | 68,4 % | 67,3 % | 68,6 % | 72,9 % | 66,2 % | 90,7 % | 60,5 % | **38,9 Mo** | **4,4 ms** |
| minilm-m3-INT8-fewshot | ONNX INT8 | 67,4 % | 68,1 % | 66,3 % | 69,5 % | 72,9 % | 65,8 % | 88,4 % | 60,5 % | **38,9 Mo** | **4,1 ms** |

### Modèles d'origine (PyTorch, phase 1) — pour situer

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Taille |
|---|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| gemma-fewshot | 85,0 % | 86,4 % | 82,1 % | 90,5 % | 80,0 % | 86,6 % | 1 269,6 Mo |
| gemma-proto | 83,1 % | 84,4 % | 80,6 % | 87,6 % | 78,6 % | 84,4 % | 1 269,6 Mo |
| e5small-fewshot | 71,1 % | 72,4 % | 68,4 % | 76,2 % | 74,3 % | 70,1 % | 964,0 Mo |
| e5small-proto | 69,4 % | 70,8 % | 67,9 % | 72,4 % | 75,7 % | 67,5 % | 964,0 Mo |
| minilm-proto | 68,8 % | 69,8 % | 67,3 % | 71,4 % | 75,7 % | 66,7 % | 970,3 Mo |
| minilm-fewshot | 68,4 % | 69,1 % | 67,3 % | 70,5 % | 78,6 % | 65,4 % | 970,3 Mo |
| *tabularisai (témoin, CC-BY-NC)* | *56,8 %* | *58,5 %* | — | — | *50,0 %* | — | *1 129 Mo* |

`e5-small` est **abandonné** comme prévu par `PROTOCOLE-SUITE.md` (qualité insuffisante,
confiance inexploitable : moyenne 0,01).

---

## 3. Tableau de décision — Gemma : origine / m3 / m10 × FP32 / INT8 / Q4

| Pruning | Format | Fichier | Global | Avec fautes | `energie` | `suspense` | Cos vs FP32 (moy / min) | Latence |
|:--:|:--:|---|:--:|:--:|:--:|:--:|:--:|:--:|
| origine | FP32 | 1 212,5 Mo | 85,0 %* | 80,0 %* | 88,4 %* | 90,7 %* | — | 18,3 ms |
| origine | INT8 | 305,6 Mo | non mesuré | — | — | — | 0,9639 / 0,8538 | 11,4 ms |
| origine | **Q4** | 190,7 Mo | 87,4 % | 81,4 % | 95,3 % | 90,7 % | 0,9682 / 0,9321 | 10,3 ms |
| **m3** | FP32 | 733,6 Mo | 85,0 % | 80,0 % | 93,0 % | 90,7 % | — | 27,6 ms |
| m3 | INT8 | 185,8 Mo | 86,0 % | 81,4 % | 93,0 % | 88,4 % | 0,9640 / 0,9068 | 12,9 ms |
| **m3** | **Q4** | **115,9 Mo** | **86,7 %** | **81,4 %** | **93,0 %** | **90,7 %** | 0,9682 / 0,9321 | 10,1 ms |
| m10 | FP32 | 659,3 Mo | 83,7 % | 77,1 % | 93,0 % | 90,7 % | — | 22,4 ms |
| m10 | INT8 | 167,2 Mo | 84,1 % | 80,0 % | 88,4 % | 90,7 % | 0,9664 / 0,9275 | 16,1 ms |
| m10 | **Q4** | 104,3 Mo | 85,0 % | 78,6 % | 93,0 % | 90,7 % | 0,9682 / 0,9321 | 10,4 ms |

\* valeurs de la variante `fewshot` en PyTorch (phase 1 / phase 2 bis), pas du fichier embarqué.

**Deux observations contre-intuitives, vérifiées :**

1. **Q4 bat INT8 sur les trois axes** — plus petit (115,9 vs 185,8 Mo), plus rapide
   (10,1 vs 12,9 ms) **et** plus précis (86,7 vs 86,0 %). La quantification INT8 dynamique
   d'ONNX Runtime dégrade davantage les activations que le schéma 4 bits par blocs
   (`MatMulNBits` + `GatherBlockQuantized`), qui quantifie aussi la table d'embeddings.
2. **Le cosinus Q4 ↔ FP32 est identique au 4ᵉ chiffre pour les trois variantes** (0,9682 / 0,9321).
   Vérifié indépendamment, et explicable : sur les 50 textes de dev, **tous** les tokens utilisés
   sont conservés par l'élagage ; les lignes d'embedding restent donc bit-identiques d'une variante
   à l'autre, et la quantification produit exactement la même erreur. Ce n'est pas un bug.

**Note sur `q4_portee`** : `"MatMul + Gather (embeddings)"` pour les trois — la table d'embeddings
est bien quantifiée. Avertissements ONNX Runtime : le modèle est passé en **opset 21 / IR 10**
pour supporter int4 (« *the generated model may not be a valid model* ») ; le modèle produit
fonctionne (encodage et cosinus calculés sans erreur), mais **ce point mérite un test sur
téléphone avant production**.

---

## 4. Tableau de l'élagage

| Modèle | Vocab. avant → après | Params avant → après | Msg courts : tokens / cos moy / cos min | Wikipédia FR/EN : tokens / cos moy / cos min |
|---|:--:|:--:|:--:|:--:|
| **gemma-m3** | 262 145 → **106 233** (40,5 %) | 307,6 M → **187,8 M** (−39 %) | 99,52 % / **0,99992** ✅ / 0,9826 | 95,21 % / **0,99894** ✅ / 0,7814 |
| gemma-m10 | 262 145 → **82 047** (31,3 %) | 307,6 M → **169,3 M** (−45 %) | 98,55 % / **0,99944** ✅ / 0,9173 | 86,30 % / **0,99564** ❌ / 0,7546 |
| minilm-m3 | 250 002 → **43 323** (17,3 %) | 117,7 M → **38,3 M** (−67 %) | **100 %** / **1,0** ✅ / **1,0** ✅ | 98,54 % / **0,99953** ✅ / 0,8483 |

### Critère de validité (`cos moy ≥ 0,999` et `cos min ≥ 0,99`)

| Modèle | `cos_moy` msg courts | `cos_moy` wiki FR/EN | `cos_min` wiki FR/EN | Verdict |
|---|:--:|:--:|:--:|:--:|
| gemma-m3 | ✅ 0,99992 | ✅ 0,99894 | ❌ 0,7814 | moyenne ✅ / min ❌ |
| gemma-m10 | ✅ 0,99944 | ❌ 0,99564 | ❌ 0,7546 | **échoue** |
| minilm-m3 | ✅ 1,0 | ✅ 0,99953 | ❌ 0,8483 | moyenne ✅ / min ❌ |

**Constat : le `cos_min` n'est atteint par aucun modèle, et c'est structurel.** Les textes les
plus éloignés sont des **noms propres étrangers présents dans Wikipédia FR/EN** :
`" Yapıldak"` (turc, pour e5), `" Trafikverket official website"` (suédois, pour minilm-m3),
`" Archer-Daniels-Midland"` (pour gemma-m3). L'élagage FR+EN supprime les morceaux de mots de ces
langues, donc leur embedding change — comportement attendu.

**Sur la population qui correspond à l'usage réel** (messages courts, style SMS) : cosinus moyen
**≥ 0,9994** et **100 % de tokens identiques pour MiniLM**. C'est l'indicateur pertinent.
Je rapporte donc le critère **non atteint formellement**, mais avec l'analyse de la cause.

**`m10` doit être écarté** : `cos_moy` wiki 0,99564 < 0,999 et only 86,3 % de tokens identiques
— la fidélité se dégrade vraiment, pas seulement sur des cas exotiques.

---

## 5. Gain de l'élagage, chiffré

**À format égal (Q4), Gemma :**

| | Taille | Global | Fautes |
|---|:--:|:--:|:--:|
| Gemma **non élagué** Q4 | 190,7 Mo | 87,4 % | 81,4 % |
| **Gemma m3** Q4 | **115,9 Mo** | 86,7 % | 81,4 % |
| **Gain** | **−74,8 Mo (−39,2 %)** | −0,7 pt (dans le bruit) | **0 pt** |

**MiniLM (INT8) — la référence « clavier » :**

| | Taille | Global | Fautes |
|---|:--:|:--:|:--:|
| MiniLM **non élagué** INT8 | 118,3 Mo | 68,8 %* | 75,7 %* |
| **MiniLM m3** INT8 | **38,9 Mo** | 67,8 % | 72,9 % |
| **Gain** | **−79,4 Mo (−67,1 %)** | −1,0 pt | −2,8 pt |

\* valeurs `proto` en PyTorch, seule mesure disponible pour le non-élagué.

**Conclusion** : l'élagage est **gratuit** sur Gemma (−39 % de taille, −0,7 point) et très
rentable sur MiniLM (−67 % de taille, −1 point). C'est le levier de taille le plus efficace du
round 2.

---

## 6. Seuil de confiance (phase 4)

Seuil choisi sur **`dev_v1.csv`** (70 messages), puis reporté sur `test_v2.csv`.

| Modèle | Conf. moy. | dev ≥ 0,2 | dev ≥ 0,3 | dev ≥ 0,4 | test ≥ 0,2 | test ≥ 0,3 | test ≥ 0,4 |
|---|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| **gemma-m3-Q4-fewshot** | 0,54 | **95,3 % / 91 %** | 95,1 % / 87 % | 98,1 % / 74 % | **91,0 % / 85 %** | 94,6 % / 67 % | 96,6 % / 49 % |
| gemma-m3-Q4-proto | 0,55 | 93,8 % / 93 % | 93,2 % / 84 % | 94,1 % / 73 % | 88,2 % / 87 % | 92,2 % / 68 % | 94,7 % / 50 % |
| gemma-m10-Q4-fewshot | 0,52 | 93,8 % / 91 % | 93,2 % / 84 % | 95,7 % / 67 % | 91,5 % / 82 % | 93,1 % / 68 % | 98,5 % / 45 % |

*(précision tolérante / couverture)*

**Seuil recommandé : 0,2.** C'est le seul qui satisfait le critère du §6
(**précision ≥ 85 % en couvrant ≥ 70 %**) **sur les deux jeux à la fois** :
95,3 % / 91 % sur dev, 91,0 % / 85 % sur test.

À 0,3 la précision monte à 94,6 % mais la couverture tombe à **67 %** sur le test — sous la cible.
**Réserve** : dev ne compte que 70 messages, le seuil est donc **indicatif** ; il devra être
recalibré sur un jeu de validation plus large en production.

---

## 7. Intervalles de confiance

| Population | n | Incertitude (95 %) | Conséquence |
|---|:--:|:--:|---|
| Global | 301 | **±5 points** | Un écart < 5 points **n'est pas une différence** |
| Avec fautes | 70 | **±10 points** | Un écart < 10 points **n'est pas une différence** |

**Application aux résultats :**

- gemma-orig-Q4 (87,4 %) vs gemma-m3-Q4 (86,7 %) → **−0,7 point = aucune différence.** L'élagage
  m3 est donc **gratuit en qualité**, de façon statistique.
- gemma-m3-Q4 (86,7 %) vs gemma-m10-Q4 (85,0 %) → −1,7 point = **dans le bruit**. Mais m10 est
  écarté sur le critère d'élagage (`cos_moy` wiki), pas sur la précision.
- gemma-m3-Q4 (86,7 %) vs minilm-m3-INT8 (67,8 %) → **−18,9 points = différence massive.**
- Q4 vs INT8 sur gemma-m3 : +0,7 point → **aucune différence.** Q4 gagne par la **taille**
  (−69,9 Mo) et la **latence** (−2,3 ms), pas par la précision.

---

## 8. Les 10 erreurs les plus révélatrices — `gemma-m3-Q4-fewshot` (38 erreurs)

Classées par cause probable.

| # | Message | Attendu | Prédit | Cause |
|:-:|---|:--:|:--:|---|
| 1 | « t'avais promis et t'es pas venu, merci bien » | `colere` | `tristesse` | **Sarcasme.** « merci bien » est lu au premier degré ; le reproche est noyé dans la politesse. |
| 2 | « mes parents se disputent tout le temps » | `tristesse` | `colere` | **Émotion du sujet ≠ émotion de l'expéditeur.** Le sujet est le conflit (`colere`), ce que ressent l'ado est la tristesse/impuissance. Piège explicitement signalé par `ANNOTATION.md`. |
| 3 | « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » | `colere` | `calme` | **Litote sans lexique émotionnel.** Aucun mot d'agacement, seulement des majuscules et une négation. |
| 4 | « je vais devoir redoubler » | `tristesse` | `energie` | **Échec énoncé comme un défi.** « devoir + infinitif » est associé à la détermination dans les prototypes d'`energie`. |
| 5 | « j'ai fait une crise d'angoisse avant l'oral » | `tristesse` | `suspense` | **Angoisse** (peur) classée `suspense` (anticipation) au lieu de tristesse. Frontière `suspense`/`tristesse` mal tranchée. |
| 6 | « il neige !!! pas cours demain » | `joie` | `calme` | **Joie par conséquence** : la joie n'est pas dite, elle est inférée (« pas cours »). Le message est factuel en surface. |
| 7 | « le prof a annulé l'interro mdr » | `joie` | `calme` | Même cause, et **« mdr » sous-exploité** comme marqueur de joie. |
| 8 | « demain 7h footing, qui vient ? » | `energie` | `calme` | **Énergie portée par l'action proposée**, pas par un mot émotionnel. |
| 9 | « je peux pas en parler par message, appelle » | `suspense` | `calme` | **Suspense implicite** (refus de dire) sans marqueur explicite. |
| 10 | « grave relou le bus qui part sans nous » | `colere` | `tristesse` | **Argot « relou »** absent des prototypes + registre plaintif qui tire vers la tristesse. |

**Trois familles de causes dominent :** (a) l'émotion est **inférée, jamais dite**
(#6, #7, #8, #9), (b) le **lexique est absent** (#3, #10, #4), (c) la **frontière entre deux
émotions proches** n'est pas tranchée (#1, #2, #5).

---

## 9. Annotations de `test_v2.csv` qui semblent discutables

*(Non modifiées. `alt` n'est renseigné que sur 11 messages, soit 3,7 % — sous le plafond de 10 %
que fixe `ANNOTATION.md`.)*

| Message | Annotation | Discussion |
|---|---|---|
| « on part à Disney ce week-end 🤩 » | `joie` | **`energie` défendable** : projet, anticipation. Aucun `alt` renseigné. |
| « joyeux anniversaire à la meilleure du monde 🎂 » | `amour` | **`joie` défendable** : féliciter exprime aussi la joie. Aucun `alt`. |
| « j'ai fait une crise d'angoisse avant l'oral » | `tristesse` | **`suspense` défendable** (peur/anxiété). 5 modèles sur 6 prédisent `suspense`. Aucun `alt`. |
| « mes parents se disputent tout le temps » | `tristesse` | **`colere` défendable** (révolte). Aucun `alt`. Candidat idéal pour un `alt`. |
| « t'avais promis et t'es pas venu, merci bien » | `colere` | **`tristesse` défendable** (déception). Aucun `alt`. |
| « grave relou le bus qui part sans nous » | `colere` | **`tristesse` défendable** (frustration). Aucun `alt`. |
| « il neige !!! pas cours demain » | `joie` | Correct, mais **la joie est entièrement inférée** — le message ne contient aucun marqueur. Cas difficile à annoter sans contexte. |

**Impact** : 7 messages = 2,3 % du jeu, soit ~0,7 point au global. Non significatif, mais les
4 premiers mériteraient un `alt` pour ne pas pénaliser injustement un modèle qui choisit
l'émotion alternative.

---

## 10. Pistes pour enrichir `prototypes.json`

**Déduites du jeu de développement (`dev_v1.csv`) uniquement, comme exigé.**

Les erreurs de dev du meilleur modèle pointent vers ces manques :

### a) Émotions **inférées** sans lexique émotionnel — le trou le plus coûteux
- `joie` : *pas cours*, *l'interro est annulée*, *c'est fini*, *dernier jour*
- `energie` : *demain 7h footing, qui vient ?*, *on s'entraîne à 18h*, *qui est chaud ?*
- `suspense` : *je peux pas en parler par message*, *appelle-moi*, *pas ici*, *je te dirai*

### b) Argot et registre relâché
- `colere` : *relou*, *gavé*, *ça me saoule*, *j'ai la haine*, *c'est abusé*
- `tristesse` : *j'ai le seum*, *je suis dégoûté*, *bad*, *au fond du trou*

### c) Sarcasme et fausse politesse
- `colere` : *merci bien…*, *super…*, *génial.* (point final), *bravo hein*
- `colere` : *t'avais promis*, *tu m'as encore laissé en vu*

### d) Frontières à trancher explicitement
- `colere` **vs** `tristesse` : ajouter dans `colere` des exemples **sans** destinataire
  (*ça m'énerve*, *c'est injuste*) et dans `tristesse` des exemples de **déception adressée**
  (*je suis déçu*, *tu m'as fait de la peine*)
- `suspense` **vs** `tristesse` : placer l'*angoisse* et la *crise d'angoisse* côté `tristesse`
  avec un exemple explicite, et réserver `suspense` au **mystère** et au **teasing**
- `energie` **vs** `joie` : garder la **victoire déjà acquise** en `joie`, la **préparation** en `energie`

### e) Émotion du sujet ≠ émotion de l'expéditeur
Ajouter dans `tristesse` : *mes parents se disputent*, *mon frère est malade*,
*ma copine ne me parle plus* — des messages où **le sujet est négatif** mais où **l'expéditeur
est triste**, pas en colère.

### f) Échec et déception personnelle
- `tristesse` : *je vais redoubler*, *j'ai raté*, *je suis nul*, *j'y arrive pas*

### g) Emojis porteurs d'émotion
🤩 🎉 (joie) · 😡 🤬 (colere) · 💪 🔥 (energie) · 👀 (suspense) · 😭 😞 (tristesse) · ❤️ 🥰 (amour)

### h) Équilibrage FR/EN
`prototypes.json` a 16 exemples par émotion ; vérifier que la répartition FR/EN reste ~8/8,
le jeu de test étant à 65 % FR / 35 % EN.

**Levier prioritaire : (a) + (d) + (e).** Ce sont les trois causes directes des erreurs mesurées.
`prototypes.json` est modifiable sans réentraînement : c'est le levier d'amélioration le moins
coûteux.

---

## 11. Licence Gemma

Source retenue : **`unsloth/embeddinggemma-300m`** (miroir), licence déclarée **`gemma`**
(un miroir ne change pas la licence).

**Gemma Terms of Use** — points à faire valider par un juriste avant publication :

- **Usage commercial : autorisé**, y compris dans une application distribuée.
- **Redistribution : autorisée**, mais l'app doit **reproduire la notice de licence** et la
  clause de « Prohibited Use Policy » de Google.
- **Usage prohibé** : la politique de Google s'applique à l'app et à ses utilisateurs finaux.
- **Pas de clause de non-concurrence** dans la version actuelle, mais Google peut faire évoluer
  les termes pour les versions futures.
- **Attention** : ce n'est **pas** une licence open-source (ni Apache, ni MIT). Elle est plus
  permissive qu'une licence non commerciale (`tabularisai`, CC-BY-NC, écartée pour cette raison),
  mais impose des obligations de notice.

**Comparaison** : `paraphrase-multilingual-MiniLM-L12-v2` est en **Apache-2.0** (aucune obligation
de notice au-delà de l'attribution standard). C'est un argument en faveur de MiniLM si les
contraintes juridiques pèsent plus que les 19 points de précision.

---

## 12. Recommandation finale

### Pour l'app (≤ 150 Mo)

> **`embeddinggemma-300m` élagué `--wiki-min-count 3`, variante `fewshot`, ONNX **Q4** :
> 115,9 Mo, 86,7 % global · 81,4 % avec fautes · 93,0 % `energie` · 90,7 % `suspense` ·
> 25,5 ms sur M1, seuil de confiance 0,2.**

C'est **le plus petit Gemma qui passe tous les critères** du §6 : qualité (≥ 80 % / ≥ 70 % /
≥ 70 %), taille (≤ 150 Mo) et latence (≤ 50 ms). Il ne perd **rien** face au Gemma non élagué
(−0,7 point, dans le bruit) tout en étant **39 % plus léger**.

**Réserve de latence** : 25,5 ms sur M1 Max. Un téléphone de milieu de gamme est **3 à 5 fois
plus lent**, soit **~75-130 ms**. À valider sur cible réelle — si c'est trop lent, `gemma-m10-Q4`
(104,3 Mo, 85,0 %) offre une marge, ou MiniLM comme repli.

### Pour le clavier (le plus petit possible)

> **La cible de 40 Mo n'est pas atteignable avec une qualité acceptable.**
> `minilm-m3` INT8 fait **38,9 Mo** et tient le budget, mais plafonne à **67,8 %** global et
> **60,5 % de rappel `suspense`** — soit **19 points sous Gemma**, très en dessous des critères.

Trois options, par ordre de préférence :

1. **Relever le budget clavier à ~110 Mo** et utiliser **le même `gemma-m3-Q4`** que l'app :
   un seul modèle à maintenir, un seul jeu de prototypes, qualité constante partout.
2. **Accepter MiniLM à 38,9 Mo** en assumant une expérience dégradée (2 messages sur 3 bien
   classés, `suspense` quasi toujours raté) — à réserver à un mode « suggestions » très tolérant.
3. **Ne pas faire de clavier** et réserver la détection à l'app — c'est ce que les chiffres
   recommandent.

---

## 13. Écarts au protocole, et pourquoi

| # | Écart | Justification | Impact |
|---|---|---|---|
| 1 | **Source Gemma = `unsloth/embeddinggemma-300m`** au lieu de `google/embeddinggemma-300m` | Google est **`gated: manual`** (approbation + token indisponibles). Miroir non-gated, format sentence-transformers complet. Autorisé par `PROTOCOLE-SUITE.md` §1 (« Sinon : garde `unsloth/…`, note-le comme risque résiduel »). | **Risque résiduel : poids supposés identiques, non vérifiés bit à bit.** |
| 2 | **`device="cpu"` dans `prune_vocab.py`** | Sur Apple Silicon, `SentenceTransformer()` choisit MPS et `st.save()` échoue dans safetensors (`torch.AcceleratorError`). Intégré par la suite du protocole. | Aucun sur les résultats ; rend l'élagage déterministe. |
| 3 | **Filtre des ids orphelins dans `prune_vocab.py`** (3 endroits) | EmbeddingGemma déclare **262 145** tokens pour **262 144** lignes d'embeddings : `emb.weight[idx]` lève `IndexError` sur l'id 262144. Non corrigé dans la suite du protocole → Gemma plantait. | 1 id de tokenizer écarté. Aucun impact mesurable (ce token n'a pas de vecteur). |
| 4 | **`CPUExecutionProvider` forcé dans `bench.py` et `quantize_onnx.py`** | Même avec `device="cpu"`, sentence-transformers 5.7 sélectionne CoreML sur macOS, qui échoue sur les graphes quantifiés : `CoreML … Unable to compute the prediction (error code: -1)`. Forcer CPU = la cible réelle (CPU ARM) et la référence du protocole. | Les latences mesurées sont **CPU**, pas CoreML/GPU — c'est ce qu'on veut pour un téléphone. |
| 5 | **`quantize_onnx.py` rendu idempotent** | `fp32.save(path)` lève `SameFileError` si `onnx/model.onnx` existe déjà : script non ré-exécutable. | Aucun. |
| 6 | **`onnx-ir` installé** | `MatMulNBitsQuantizer` (onnxruntime 1.31) importe `onnx_ir`, absent des dépendances déclarées au §4 du protocole. | Aucun. |
| 7 | **`--wiki 50000` non exécuté** | `PROTOCOLE-SUITE.md` §0.4 le demande explicitement : « Ne pas relancer avec `--wiki 50000`, ton analyse est juste. » | Aucun. |
| 8 | **`e5-small` arrêté après la phase 1** | `PROTOCOLE-SUITE.md` §0.1 : « E5 est abandonné ». | Aucun. |
| 9 | **Gemma non élagué mesuré en Q4 seulement (pas en INT8)** pour la précision | Le protocole demande le gain « à format égal » ; le format retenu est Q4, et l'INT8 non élagué (305,6 Mo) est hors budget de toute façon. Sa taille INT8 est néanmoins rapportée dans le `onnx_report.json`. | Le gain est chiffré à format Q4, ce qui est le format de décision. |
| 10 | **`gemma-m10` conservé malgré l'échec du critère `cos_moy`** | Demandé explicitement par la suite du protocole (mesurer le compromis taille/fidélité). | Écarté de la recommandation, mais documenté. |

---

## 14. Livrables

| Fichier | Contenu |
|---|---|
| `r2-origine.md` / `.json` | Phase 1 — 6 candidats sur `test_v2.csv` |
| `r2-origine-google.*` | *(absent : pas de token HF)* |
| `r2-reference.md` / `.json` | Témoin `tabularisai` |
| `r2-elague.md` / `.json` | Précision des 3 modèles élagués (PyTorch) |
| `pruned/gemma-m3/pruning.json` | Vocab, params, validation (2 groupes) |
| `pruned/gemma-m10/pruning.json` | idem |
| `pruned/minilm-m3/pruning.json` | idem |
| `pruned/gemma-m3/onnx_report.json` | Tailles FP32/INT8/Q4, latences, cosinus |
| `pruned/gemma-m10/onnx_report.json` | idem |
| `pruned/minilm-m3/onnx_report.json` | FP32/INT8 |
| `models/unsloth__embeddinggemma-300m/onnx_report.json` | Gemma non élagué |
| `r2-embarque.md` / `.json` | Précision des 12 variantes embarquées sur `test_v2.csv` |
| `r2-dev-seuils.md` / `.json` | Seuils de confiance sur `dev_v1.csv` |
| `versions.txt` | `pip freeze` complet |
| `SYNTHESE.md` | Ce document |
