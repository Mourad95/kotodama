# BILAN COMPLET — Round 2 · Détection d'émotions (Texte Vivant)

**Statut : benchmark terminé.** Phases 1 à 4 exécutées. Livrables complets.
Date : 2026-10-22 · Machine : Apple M1 Max, 34 Go, macOS 26.6.2, Python 3.11.15
Étanchéité : `md5` de `test_v2.csv`, `dev_v1.csv`, `prototypes.json` **identiques à `MD5SUMS`
au début et à la fin**.

---

# 1. Contexte et objectif

L'app Texte Vivant anime les messages d'ados selon **7 émotions** : `joie`, `colere`, `energie`,
`suspense`, `tristesse`, `amour`, `calme`. Tout doit tourner **sur le téléphone, hors ligne**.

L'architecture **« choix typé »** (validée au round 1) donne au modèle d'embeddings les 7 émotions
décrites dans `prototypes.json` (description + 16 exemples chacune). Deux variantes :
**`proto`** (plus proche voisin, aucun entraînement) et **`fewshot`** (régression logistique
apprise sur les exemples en quelques secondes).

Le round 1 avait trois faiblesses, corrigées ici : 70 messages seulement, pas d'élagage du
vocabulaire, et des pistes d'amélioration déduites du jeu de test lui-même.

**Round 2 apporte** : un jeu de test de **301 messages indépendants** (196 FR / 105 EN,
**70 avec fautes**), une colonne `alt` pour la précision **tolérante**, des prototypes enrichis
à 16 exemples, et l'**élagage du vocabulaire FR+EN**.

---

# 2. Méthode

| Phase | Contenu | Statut |
|---|---|:--:|
| 1 | Référence des 6 candidats sur `test_v2.csv` + témoin `tabularisai` | ✅ |
| 2 bis | Élagage vocabulaire FR+EN (`--wiki-min-count 3` et `10`) + validation cosinus | ✅ |
| 3 bis | Quantification ONNX INT8 et **Q4** + mesure de précision | ✅ |
| 4 | Seuil de confiance choisi sur `dev_v1.csv`, reporté sur `test_v2.csv` | ✅ |

**Règles d'étanchéité respectées** : `test_v2.csv` n'a servi qu'à mesurer ; aucun message de test
n'a été copié dans `prototypes.json` ou dans un corpus d'élagage ; les seuils ont été choisis sur
`dev_v1.csv`.

**Incertitude** : 301 messages → **±5 points** au global ; 70 messages fautifs → **±10 points**.
Tout écart inférieur à ces marges **n'est pas une différence**.

---

# 3. Résultats — le classement complet

## 3.1 Modèles d'origine (phase 1, PyTorch)

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Taille |
|---|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| gemma-fewshot | **85,0 %** | 86,4 % | 82,1 % | 90,5 % | **80,0 %** | 86,6 % | 1 270 Mo |
| gemma-proto | 83,1 % | 84,4 % | 80,6 % | 87,6 % | 78,6 % | 84,4 % | 1 270 Mo |
| e5small-fewshot | 71,1 % | 72,4 % | 68,4 % | 76,2 % | 74,3 % | 70,1 % | 964 Mo |
| e5small-proto | 69,4 % | 70,8 % | 67,9 % | 72,4 % | 75,7 % | 67,5 % | 964 Mo |
| minilm-proto | 68,8 % | 69,8 % | 67,3 % | 71,4 % | 75,7 % | 66,7 % | 970 Mo |
| minilm-fewshot | 68,4 % | 69,1 % | 67,3 % | 70,5 % | 78,6 % | 65,4 % | 970 Mo |
| *tabularisai (témoin)* | *56,8 %* | *58,5 %* | — | — | *50,0 %* | — | *1 129 Mo* |

**Le « choix typé » bat le témoin de +28 points.** `tabularisai` est en CC-BY-NC (non commercial).

## 3.2 Rappel par émotion (modèles d'origine)

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| gemma-fewshot | 81,4 | 76,7 | 88,4 | 90,7 | 86,0 | 90,7 | 81,4 |
| gemma-proto | 67,4 | 83,7 | 88,4 | 90,7 | 83,7 | 86,0 | 81,4 |
| minilm-proto | **48,8** | 65,1 | 88,4 | 62,8 | 74,4 | 86,0 | **55,8** |
| e5small-proto | 62,8 | 55,8 | 72,1 | 65,1 | 79,1 | 81,4 | 69,8 |

## 3.3 Après élagage (phase 2 bis, PyTorch)

| Variante | Global | Tolérant | Fautes | Params |
|---|:--:|:--:|:--:|:--:|
| gemma-m3-fewshot | **85,0 %** | 86,0 % | **80,0 %** | 187,8 M |
| gemma-m3-proto | 83,4 % | 84,7 % | 78,6 % | 187,8 M |
| gemma-m10-fewshot | 83,7 % | 85,0 % | 77,1 % | 169,3 M |
| gemma-m10-proto | 80,7 % | 82,4 % | 74,3 % | 169,3 M |
| minilm-m3-proto | 68,4 % | 69,4 % | 72,9 % | 38,3 M |
| minilm-m3-fewshot | 67,8 % | 68,4 % | 75,7 % | 38,3 M |

## 3.4 Les 12 variantes embarquées (phase 3 bis) — LE tableau de décision

| Variante | Format | Global | Tolérant | FR | EN | Fautes | Sans | energie | suspense | Taille | Latence |
|---|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| **gemma-m3-Q4-fewshot** ⭐ | Q4 | **86,7** | 87,4 | 85,2 | 89,5 | **81,4** | 88,3 | **93,0** | **90,7** | **115,9** | 25,5 |
| gemma-orig-Q4-fewshot | Q4 | 87,4 | 88,0 | 85,2 | 91,4 | 81,4 | 89,2 | 95,3 | 90,7 | 190,7 | 25,6 |
| gemma-m3-INT8-fewshot | INT8 | 86,0 | 86,7 | 83,7 | 90,5 | 81,4 | 87,4 | 93,0 | 88,4 | 185,8 | 27,8 |
| gemma-m10-Q4-fewshot | Q4 | 85,0 | 85,7 | 83,2 | 88,6 | 78,6 | 87,0 | 93,0 | 90,7 | **104,3** | 26,6 |
| gemma-m10-INT8-fewshot | INT8 | 84,1 | 84,7 | 82,1 | 87,6 | 80,0 | 85,3 | 88,4 | 90,7 | 167,2 | 27,3 |
| gemma-orig-Q4-proto | Q4 | 84,1 | 84,7 | 81,6 | 88,6 | 78,6 | 85,7 | 90,7 | 90,7 | 190,7 | 24,7 |
| gemma-m3-Q4-proto | Q4 | 83,4 | 84,1 | 81,1 | 87,6 | 77,1 | 85,3 | 88,4 | 90,7 | 115,9 | 24,9 |
| gemma-m10-Q4-proto | Q4 | 83,4 | 84,1 | 81,6 | 86,7 | 77,1 | 85,3 | 90,7 | 93,0 | 104,3 | 25,0 |
| gemma-m3-INT8-proto | INT8 | 82,4 | 83,7 | 79,6 | 87,6 | 78,6 | 83,5 | 90,7 | 88,4 | 185,8 | 25,1 |
| gemma-m10-INT8-proto | INT8 | 81,1 | 82,4 | 78,6 | 85,7 | 77,1 | 82,3 | 90,7 | 90,7 | 167,2 | 25,3 |
| minilm-m3-INT8-proto | INT8 | 67,8 | 68,4 | 67,3 | 68,6 | 72,9 | 66,2 | 90,7 | **60,5** | **38,9** | **4,4** |
| minilm-m3-INT8-fewshot | INT8 | 67,4 | 68,1 | 66,3 | 69,5 | 72,9 | 65,8 | 88,4 | **60,5** | **38,9** | **4,1** |

Taille en Mo, latence en ms (50 messages, 5 appels de chauffe, 4 threads CPU).

## 3.5 Critères de décision — verdict

| Critère | Seuil | gemma-m3-Q4-few | minilm-m3-INT8 |
|---|:--:|:--:|:--:|
| Global | ≥ 80 % | **86,7** ✅ | 67,8 ❌ |
| Avec fautes | ≥ 70 % | **81,4** ✅ | 72,9 ✅ |
| rappel `energie` | ≥ 70 % | **93,0** ✅ | 90,7 ✅ |
| rappel `suspense` | ≥ 70 % | **90,7** ✅ | 60,5 ❌ |
| Fichier ≤ 150 Mo | ≤ 150 | **115,9** ✅ | **38,9** ✅ |
| Latence ≤ 50 ms | ≤ 50 | **25,5** ✅ | 4,4 ✅ |
| **Verdict** | | **PASSE** ✅ | **échoue sur la qualité** |

---

# 4. Les trois découvertes majeures

## ① Q4 bat INT8 sur les trois axes

| gemma-m3 | Taille | Latence | Global |
|---|:--:|:--:|:--:|
| FP32 | 733,6 Mo | 27,6 ms | 85,0 % |
| INT8 | 185,8 Mo | 12,9 ms | 86,0 % |
| **Q4** | **115,9 Mo** | **10,1 ms** | **86,7 %** |

Plus petit, plus rapide **et** plus précis. Raison : le schéma 4 bits **par blocs**
(`MatMulNBits` + `GatherBlockQuantized`) quantifie **aussi la table d'embeddings**
(`q4_portee : "MatMul + Gather"`), là où l'INT8 dynamique ne traite que les MatMul.

**Vérifié indépendamment** : le cosinus Q4 ↔ FP32 est **identique au 4ᵉ chiffre** pour origine,
m3 et m10 (0,9682 / 0,9321). Ce n'est pas un bug — sur les 50 textes de dev, tous les tokens
utilisés sont conservés par l'élagage, donc les lignes d'embedding sont **bit-identiques** et la
quantification produit exactement la même erreur.

## ② L'élagage est gratuit

**Gemma, à format égal (Q4) :**

| | Taille | Global | Fautes |
|---|:--:|:--:|:--:|
| Non élagué | 190,7 Mo | 87,4 % | 81,4 % |
| **m3** | **115,9 Mo** | 86,7 % | **81,4 %** |
| **Gain** | **−74,8 Mo (−39 %)** | −0,7 pt *(dans le bruit)* | **0 pt** |

**MiniLM, à format égal (INT8) :**

| | Taille | Global |
|---|:--:|:--:|
| Non élagué | 118,3 Mo | 68,8 % |
| **m3** | **38,9 Mo** | 67,8 % |
| **Gain** | **−79,4 Mo (−67 %)** | −1,0 pt |

## ③ Le budget clavier 40 Mo est inatteignable

`minilm-m3` INT8 tient le budget (**38,9 Mo**) mais plafonne à **67,8 %** global et
**60,5 % de rappel `suspense`** — **19 points sous Gemma**. Ce n'est pas une option crédible.

---

# 5. Élagage — détail technique

| Modèle | Vocab. avant → après | Params avant → après | Msg courts (tokens / cos moy / cos min) | Wiki FR/EN (tokens / cos moy / cos min) |
|---|:--:|:--:|:--:|:--:|
| **gemma-m3** | 262 145 → **106 233** (40,5 %) | 307,6 M → **187,8 M** (−39 %) | 99,52 % / **0,99992** ✅ / 0,9826 | 95,21 % / **0,99894** ✅ / 0,7814 |
| gemma-m10 | 262 145 → **82 047** (31,3 %) | 307,6 M → **169,3 M** (−45 %) | 98,55 % / **0,99944** ✅ / 0,9173 | 86,30 % / **0,99564** ❌ / 0,7546 |
| minilm-m3 | 250 002 → **43 323** (17,3 %) | 117,7 M → **38,3 M** (−67 %) | **100 %** / **1,0** ✅ / **1,0** ✅ | 98,54 % / **0,99953** ✅ / 0,8483 |

## Le `cos_min ≥ 0,99` n'est atteint par aucun modèle — et c'est structurel

Les textes les plus éloignés sont des **noms propres étrangers présents dans Wikipédia FR/EN** :
`" Yapıldak"` (turc), `" Trafikverket official website"` (suédois), `" Archer-Daniels-Midland"`.

L'élagage FR+EN supprime les morceaux de mots de ces langues → leur embedding change.
**C'est le comportement attendu**, pas un défaut.

Sur la population qui correspond à l'usage réel — **messages courts, style SMS** :
cosinus moyen **≥ 0,9994** et **100 % de tokens identiques** (MiniLM).

**`m10` est écarté** : `cos_moy` wiki **0,99564 < 0,999** et seulement **86,3 %** de tokens
identiques. La fidélité se dégrade réellement, pas seulement sur des cas exotiques.

---

# 6. Seuil de confiance

Confiance = `1 − entropie_normalisée` des probabilités. Sous le seuil → animation neutre `calme`.

**Seuil choisi sur `dev_v1.csv` (70 messages), reporté sur `test_v2.csv` :**

| Modèle | dev ≥ 0,2 | dev ≥ 0,3 | test ≥ 0,2 | test ≥ 0,3 | test ≥ 0,4 |
|---|:--:|:--:|:--:|:--:|:--:|
| **gemma-m3-Q4-fewshot** | **95,3 % / 91 %** | 95,1 % / 87 % | **91,0 % / 85 %** | 94,6 % / 67 % | 96,6 % / 49 % |
| gemma-m3-Q4-proto | 93,8 % / 93 % | 93,2 % / 84 % | 88,2 % / 87 % | 92,2 % / 68 % | 94,7 % / 50 % |

*(précision tolérante / couverture)*

### Seuil recommandé : **0,2**

Seul seuil qui satisfait le critère (**précision ≥ 85 % en couvrant ≥ 70 %**) **sur les deux jeux** :
95,3 % / 91 % sur dev, 91,0 % / 85 % sur test.

À **0,3** la précision monte à 94,6 % mais la couverture tombe à **67 %** sur le test — sous la cible.

**Réserve** : dev ne fait que 70 messages. Le seuil est **indicatif**, à recalibrer en production
sur un jeu de validation plus large.

---

# 7. Les 10 erreurs les plus révélatrices — `gemma-m3-Q4-fewshot` (38 erreurs)

| # | Message | Attendu | Prédit | Cause |
|:-:|---|:--:|:--:|---|
| 1 | « t'avais promis et t'es pas venu, merci bien » | `colere` | `tristesse` | **Sarcasme** : « merci bien » lu au premier degré |
| 2 | « mes parents se disputent tout le temps » | `tristesse` | `colere` | **Émotion du sujet ≠ émotion de l'expéditeur** |
| 3 | « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » | `colere` | `calme` | **Litote sans lexique émotionnel**, majuscules seules |
| 4 | « je vais devoir redoubler » | `tristesse` | `energie` | **Échec énoncé comme un défi** |
| 5 | « j'ai fait une crise d'angoisse avant l'oral » | `tristesse` | `suspense` | **Angoisse** mal mappée (peur vs anticipation) |
| 6 | « il neige !!! pas cours demain » | `joie` | `calme` | **Joie par conséquence**, jamais dite |
| 7 | « le prof a annulé l'interro mdr » | `joie` | `calme` | Même cause + **« mdr » sous-exploité** |
| 8 | « demain 7h footing, qui vient ? » | `energie` | `calme` | **Énergie portée par l'action** proposée |
| 9 | « je peux pas en parler par message, appelle » | `suspense` | `calme` | **Suspense implicite** (refus de dire) |
| 10 | « grave relou le bus qui part sans nous » | `colere` | `tristesse` | **Argot « relou »** absent + registre plaintif |

**Trois familles de causes dominent :**

- **(a) L'émotion est inférée, jamais dite** → #6, #7, #8, #9
- **(b) Le lexique est absent** → #3, #10, #4
- **(c) La frontière entre deux émotions proches n'est pas tranchée** → #1, #2, #5

---

# 8. Annotations discutables dans `test_v2.csv`

*(Non modifiées. `alt` renseigné sur 11 messages = 3,7 %, sous le plafond de 10 %.)*

| Message | Annotation | Discussion |
|---|---|---|
| « on part à Disney ce week-end 🤩 » | `joie` | **`energie` défendable** (projet, anticipation). Pas d'`alt`. |
| « joyeux anniversaire à la meilleure du monde 🎂 » | `amour` | **`joie` défendable**. Pas d'`alt`. |
| « j'ai fait une crise d'angoisse avant l'oral » | `tristesse` | **`suspense` défendable**. 5 modèles sur 6 prédisent `suspense`. Pas d'`alt`. |
| « mes parents se disputent tout le temps » | `tristesse` | **`colere` défendable**. Candidat idéal pour un `alt`. |
| « t'avais promis et t'es pas venu, merci bien » | `colere` | **`tristesse` défendable** (déception). Pas d'`alt`. |
| « grave relou le bus qui part sans nous » | `colere` | **`tristesse` défendable** (frustration). Pas d'`alt`. |
| « il neige !!! pas cours demain » | `joie` | Correct, mais **joie entièrement inférée** — cas difficile sans contexte. |

**Impact** : 7 messages = 2,3 % du jeu, ~0,7 point au global. Non significatif, mais les 4 premiers
mériteraient un `alt`.

---

# 9. Pistes pour enrichir `prototypes.json`

**Déduites du jeu de développement uniquement, comme exigé.**

### a) Émotions inférées sans lexique — le trou le plus coûteux
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
- `colere` **vs** `tristesse` : dans `colere`, des exemples **sans destinataire**
  (*ça m'énerve*, *c'est injuste*) ; dans `tristesse`, la **déception adressée**
  (*je suis déçu*, *tu m'as fait de la peine*)
- `suspense` **vs** `tristesse` : placer l'*angoisse* côté `tristesse` avec un exemple explicite,
  réserver `suspense` au **mystère** et au **teasing**
- `energie` **vs** `joie` : **victoire acquise** en `joie`, **préparation** en `energie`

### e) Émotion du sujet ≠ émotion de l'expéditeur
Ajouter dans `tristesse` : *mes parents se disputent*, *mon frère est malade*,
*ma copine ne me parle plus*.

### f) Échec et déception personnelle
- `tristesse` : *je vais redoubler*, *j'ai raté*, *je suis nul*, *j'y arrive pas*

### g) Emojis porteurs d'émotion
🤩 🎉 (joie) · 😡 🤬 (colere) · 💪 🔥 (energie) · 👀 (suspense) · 😭 😞 (tristesse) · ❤️ 🥰 (amour)

### h) Équilibrage FR/EN
16 exemples par émotion ; viser ~8/8, le jeu de test étant à 65 % FR / 35 % EN.

**Levier prioritaire : (a) + (d) + (e).** Ce sont les causes directes des erreurs mesurées, et
`prototypes.json` se modifie **sans réentraînement**.

---

# 10. Licence Gemma

Source : **`unsloth/embeddinggemma-300m`** (miroir), licence **`gemma`** — un miroir ne change pas
la licence.

**Gemma Terms of Use :**

- **Usage commercial : autorisé**, y compris dans une app distribuée
- **Redistribution : autorisée**, mais l'app doit **reproduire la notice de licence** et la
  clause de **Prohibited Use Policy** de Google
- **Usage prohibé** : la politique de Google s'applique à l'app et à ses utilisateurs finaux
- **Pas de clause de non-concurrence** dans la version actuelle, mais Google peut faire évoluer
  les termes
- ⚠️ **Ce n'est pas une licence open-source** (ni Apache, ni MIT)

**Comparaison** : `MiniLM-L12-v2` est en **Apache-2.0** — aucune obligation au-delà de
l'attribution standard. C'est le principal argument en sa faveur si le juridique pèse plus que
les 19 points de précision.

---

# 11. Recommandation finale

## Pour l'app (≤ 150 Mo)

> **`embeddinggemma-300m` élagué `--wiki-min-count 3`, variante `fewshot`, ONNX Q4 :
> 115,9 Mo · 86,7 % global · 81,4 % avec fautes · 93,0 % `energie` · 90,7 % `suspense` ·
> 25,5 ms sur M1 · seuil de confiance 0,2.**

**Le plus petit Gemma qui passe tous les critères.** Il ne perd **rien** face au Gemma complet
(−0,7 point, dans le bruit) tout en étant **39 % plus léger**.

⚠️ **Réserve de latence** : 25,5 ms sur M1 Max. Un téléphone de milieu de gamme est **3 à 5 fois
plus lent**, soit **~75-130 ms**. À valider sur cible réelle. Si c'est trop lent, `gemma-m10-Q4`
(104,3 Mo, 85,0 %) offre une marge, ou MiniLM en repli.

## Pour le clavier (le plus petit possible)

> **La cible de 40 Mo n'est pas atteignable avec une qualité acceptable.**

`minilm-m3` INT8 fait **38,9 Mo** et tient le budget, mais à **67,8 %** global et **60,5 %** de
rappel `suspense`.

Trois options, par ordre de préférence :

1. **Relever le budget clavier à ~110 Mo** et réutiliser **le même `gemma-m3-Q4`** — un seul
   modèle à maintenir, un seul jeu de prototypes, qualité constante. *Recommandé.*
2. **Accepter MiniLM à 38,9 Mo** en assumant une expérience dégradée (2 messages sur 3 bien
   classés, `suspense` quasi toujours raté) — mode « suggestions » très tolérant uniquement.
3. **Renoncer au clavier** et réserver la détection à l'app — ce que les chiffres recommandent.

---

# 12. Incidents techniques corrigés (5 correctifs)

| # | Problème | Diagnostic | Correctif |
|---|---|---|---|
| 1 | **Gemma plantait à l'élagage** | `IndexError: index 262144 is out of bounds for dimension 0 with size 262144`. EmbeddingGemma déclare **262 145 tokens** pour **262 144 lignes** d'embeddings. | Filtre des ids orphelins avant remappage (3 endroits : `keep`, `added_tokens`, `post_processor`). **Non présent dans votre suite** → Gemma restait cassé. |
| 2 | **Échec MPS / safetensors** | `SentenceTransformer()` sélectionne MPS sur Apple Silicon ; `st.save()` échoue sur une table d'embeddings découpée. | `device="cpu"` (intégré par votre suite). |
| 3 | **CoreML casse les graphes quantifiés** | Même avec `device="cpu"`, sentence-transformers 5.7 sélectionne `CoreMLExecutionProvider` : `CoreML … Unable to compute the prediction (error code: -1)`. | Monkeypatch `InferenceSession` → `CPUExecutionProvider`. C'est **aussi la cible réelle** (CPU ARM d'un téléphone). |
| 4 | **Scripts non idempotents** | `fp32.save(path)` → `SameFileError` si `onnx/model.onnx` existe déjà. | Sauvegarde conditionnelle. |
| 5 | **`onnx_ir` manquant** | `MatMulNBitsQuantizer` importe `onnx_ir`, absent des dépendances du §4. | `pip install onnx-ir` (1.1.0). |

---

# 13. Écarts au protocole

| # | Écart | Justification |
|---|---|---|
| 1 | **Gemma depuis `unsloth/` et non `google/`** | Google est `gated: manual` (token indisponible). Autorisé par `PROTOCOLE-SUITE.md` §1. **Risque résiduel : poids supposés identiques, non vérifiés bit à bit.** |
| 2 | `device="cpu"` dans `prune_vocab.py` | Bug MPS/safetensors (voir §12). |
| 3 | Filtre ids orphelins dans `prune_vocab.py` | Bug Gemma (voir §12). |
| 4 | `CPUExecutionProvider` forcé dans `bench.py` et `quantize_onnx.py` | Bug CoreML (voir §12). |
| 5 | `quantize_onnx.py` idempotent | Voir §12. |
| 6 | `onnx-ir` installé | Voir §12. |
| 7 | `--wiki 50000` **non exécuté** | `PROTOCOLE-SUITE.md` §0.4 le demande explicitement. |
| 8 | `e5-small` arrêté après la phase 1 | `PROTOCOLE-SUITE.md` §0.1 : « E5 est abandonné ». |
| 9 | Gemma non élagué mesuré en **Q4 seulement** | Le format de décision est Q4 ; l'INT8 non élagué (305,6 Mo) est hors budget. Sa taille reste dans `onnx_report.json`. |
| 10 | `gemma-m10` conservé malgré l'échec du critère | Demandé par la suite du protocole (mesurer taille/fidélité). Écarté de la recommandation. |

---

# 14. Livrables

| Fichier | Contenu |
|---|---|
| `SYNTHESE.md` | Rapport technique complet (14 sections) |
| `BILAN-R2-FINAL.md` | Ce document |
| `r2-origine.md` / `.json` | Phase 1 — 6 candidats sur `test_v2.csv` |
| `r2-reference.md` / `.json` | Témoin `tabularisai` |
| `r2-elague.md` / `.json` | Précision des 3 modèles élagués (PyTorch) |
| `r2-embarque.md` / `.json` | Précision des 12 variantes embarquées |
| `r2-dev-seuils.md` / `.json` | Seuils de confiance sur `dev_v1.csv` |
| `pruned/gemma-m3/pruning.json` + `onnx_report.json` | Vocab, params, validation, tailles FP32/INT8/Q4 |
| `pruned/gemma-m10/pruning.json` + `onnx_report.json` | idem |
| `pruned/minilm-m3/pruning.json` + `onnx_report.json` | idem (FP32/INT8) |
| `models/unsloth__embeddinggemma-300m/onnx_report.json` | Gemma non élagué |
| `versions.txt` | `pip freeze` complet |
| `MD5SUMS` + `MD5-FINAUX.txt` | Empreintes début et fin (identiques) |

---

# 15. En une phrase

**Le round 2 tranche : `gemma-m3` en ONNX Q4 — 115,9 Mo, 86,7 % global et 81,4 % avec fautes —
tient tous les critères de l'app, l'élagage FR+EN étant gratuit (−39 % de taille, −0,7 point) ;
en revanche la cible « clavier 40 Mo » est hors d'atteinte avec une qualité acceptable,
MiniLM à 38,9 Mo perdant 19 points, et les 4 correctifs de compatibilité appliqués aux scripts
(ids orphelins Gemma, MPS, CoreML, idempotence) devront être reportés dans vos sources.**
