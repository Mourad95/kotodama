# Résultats benchmark émotions — test_v2.csv

301 messages dont 70 avec fautes. Un message vaut 0.3 points au global et 1.4 points dans la colonne « Avec fautes ».

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |
|---|---|---|---|---|---|---|---|---|---|
| gemma-orig-q4-fewshot | 87.4 % | 88.0 % | 85.2 % | 91.4 % | 81.4 % | 89.2 % | 25.6 ms | 190.7 Mo | choix typé (fewshot), ONNX model_q4.onnx |
| gemma-m3-q4-fewshot | 86.7 % | 87.4 % | 85.2 % | 89.5 % | 81.4 % | 88.3 % | 25.5 ms | 115.9 Mo | choix typé (fewshot), ONNX model_q4.onnx |
| gemma-m3-int8-fewshot | 86.0 % | 86.7 % | 83.7 % | 90.5 % | 81.4 % | 87.4 % | 27.8 ms | 185.8 Mo | choix typé (fewshot), ONNX model_qint8_arm64.onnx |
| gemma-m10-q4-fewshot | 85.0 % | 85.7 % | 83.2 % | 88.6 % | 78.6 % | 87.0 % | 26.6 ms | 104.3 Mo | choix typé (fewshot), ONNX model_q4.onnx |
| gemma-m10-int8-fewshot | 84.1 % | 84.7 % | 82.1 % | 87.6 % | 80.0 % | 85.3 % | 27.3 ms | 167.2 Mo | choix typé (fewshot), ONNX model_qint8_arm64.onnx |
| gemma-orig-q4-proto | 84.1 % | 84.7 % | 81.6 % | 88.6 % | 78.6 % | 85.7 % | 24.7 ms | 190.7 Mo | choix typé (proto), ONNX model_q4.onnx |
| gemma-m3-q4-proto | 83.4 % | 84.1 % | 81.1 % | 87.6 % | 77.1 % | 85.3 % | 24.9 ms | 115.9 Mo | choix typé (proto), ONNX model_q4.onnx |
| gemma-m10-q4-proto | 83.4 % | 84.1 % | 81.6 % | 86.7 % | 77.1 % | 85.3 % | 25.0 ms | 104.3 Mo | choix typé (proto), ONNX model_q4.onnx |
| gemma-m3-int8-proto | 82.4 % | 83.7 % | 79.6 % | 87.6 % | 78.6 % | 83.5 % | 25.1 ms | 185.8 Mo | choix typé (proto), ONNX model_qint8_arm64.onnx |
| gemma-m10-int8-proto | 81.1 % | 82.4 % | 78.6 % | 85.7 % | 77.1 % | 82.3 % | 25.3 ms | 167.2 Mo | choix typé (proto), ONNX model_qint8_arm64.onnx |
| minilm-m3-int8-proto | 67.8 % | 68.4 % | 67.3 % | 68.6 % | 72.9 % | 66.2 % | 4.4 ms | 38.9 Mo | choix typé (proto), ONNX model_qint8_arm64.onnx |
| minilm-m3-int8-fewshot | 67.4 % | 68.1 % | 66.3 % | 69.5 % | 72.9 % | 65.8 % | 4.1 ms | 38.9 Mo | choix typé (fewshot), ONNX model_qint8_arm64.onnx |

## Rappel par émotion

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|---|---|---|---|---|---|---|
| gemma-m3-int8-proto | 67.4 % | 79.1 % | 90.7 % | 88.4 % | 86.0 % | 86.0 % | 79.1 % |
| gemma-m3-int8-fewshot | 83.7 % | 79.1 % | 93.0 % | 88.4 % | 88.4 % | 90.7 % | 79.1 % |
| gemma-m3-q4-proto | 69.8 % | 79.1 % | 88.4 % | 90.7 % | 83.7 % | 88.4 % | 83.7 % |
| gemma-m3-q4-fewshot | 83.7 % | 81.4 % | 93.0 % | 90.7 % | 86.0 % | 93.0 % | 79.1 % |
| gemma-m10-int8-proto | 65.1 % | 69.8 % | 90.7 % | 90.7 % | 81.4 % | 88.4 % | 81.4 % |
| gemma-m10-int8-fewshot | 79.1 % | 74.4 % | 88.4 % | 90.7 % | 88.4 % | 90.7 % | 76.7 % |
| gemma-m10-q4-proto | 67.4 % | 72.1 % | 90.7 % | 93.0 % | 83.7 % | 90.7 % | 86.0 % |
| gemma-m10-q4-fewshot | 76.7 % | 79.1 % | 93.0 % | 90.7 % | 86.0 % | 90.7 % | 79.1 % |
| minilm-m3-int8-proto | 44.2 % | 67.4 % | 90.7 % | 60.5 % | 74.4 % | 83.7 % | 53.5 % |
| minilm-m3-int8-fewshot | 53.5 % | 62.8 % | 88.4 % | 60.5 % | 74.4 % | 74.4 % | 58.1 % |
| gemma-orig-q4-proto | 69.8 % | 81.4 % | 90.7 % | 90.7 % | 83.7 % | 88.4 % | 83.7 % |
| gemma-orig-q4-fewshot | 83.7 % | 83.7 % | 95.3 % | 90.7 % | 86.0 % | 93.0 % | 79.1 % |

## Confiance (choix typé)

Précision tolérante sur les messages dont la confiance dépasse le seuil, et part des messages concernés.

| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |
|---|---|---|---|---|---|
| gemma-m3-int8-proto | 0.45 | 87.9 % sur 85 % | 91.8 % sur 69 % | 93.8 % sur 54 % | 97.5 % sur 39 % |
| gemma-m3-int8-fewshot | 0.43 | 90.8 % sur 83 % | 93.3 % sur 69 % | 95.7 % sur 53 % | 98.3 % sur 40 % |
| gemma-m3-q4-proto | 0.44 | 88.2 % sur 87 % | 92.2 % sur 68 % | 94.7 % sur 50 % | 98.1 % sur 36 % |
| gemma-m3-q4-fewshot | 0.43 | 91.0 % sur 85 % | 94.6 % sur 67 % | 96.6 % sur 49 % | 99.1 % sur 37 % |
| gemma-m10-int8-proto | 0.41 | 89.2 % sur 83 % | 91.7 % sur 64 % | 95.9 % sur 48 % | 97.9 % sur 32 % |
| gemma-m10-int8-fewshot | 0.41 | 90.1 % sur 81 % | 92.9 % sur 65 % | 96.6 % sur 49 % | 99.0 % sur 34 % |
| gemma-m10-q4-proto | 0.4 | 90.0 % sur 80 % | 92.4 % sur 61 % | 97.0 % sur 44 % | 98.9 % sur 30 % |
| gemma-m10-q4-fewshot | 0.41 | 91.5 % sur 82 % | 93.1 % sur 68 % | 98.5 % sur 45 % | 99.0 % sur 33 % |
| minilm-m3-int8-proto | 0.48 | 71.4 % sur 89 % | 78.4 % sur 74 % | 83.1 % sur 59 % | 85.6 % sur 44 % |
| minilm-m3-int8-fewshot | 0.37 | 77.1 % sur 81 % | 84.1 % sur 56 % | 87.5 % sur 40 % | 89.9 % sur 23 % |
| gemma-orig-q4-proto | 0.45 | 88.6 % sur 88 % | 91.8 % sur 69 % | 94.8 % sur 51 % | 97.3 % sur 37 % |
| gemma-orig-q4-fewshot | 0.43 | 91.8 % sur 85 % | 95.1 % sur 68 % | 95.9 % sur 49 % | 99.1 % sur 36 % |

## Erreurs : gemma-m3-int8-proto (49)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **colere**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « jpp de cette vidéo de chat » → attendu **joie**, prédit **suspense**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « laisse-moi tranquille » → attendu **colere**, prédit **calme**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **suspense**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « tu crois qu'ils vont dire les résultats aujourd'hui ? » → attendu **suspense**, prédit **calme**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « merci d'avoir été là hier, ça m'a fait du bien » → attendu **amour**, prédit **joie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je kiffe trop être avec toi » → attendu **amour**, prédit **colere**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **suspense**
- « il faut ramener une calculatrice » → attendu **calme**, prédit **colere**
- « mdr t trop marant » → attendu **joie**, prédit **colere**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **tristesse**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « arete de m'enerver » → attendu **colere**, prédit **calme**
- « tu devinera jamais qui ma ecrit » → attendu **suspense**, prédit **colere**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **calme**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « je suis trop contant de t'avoir comme ami » → attendu **amour**, prédit **tristesse**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « je rentre a pied ce soir » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **energie**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « you said you'd come and you didn't » → attendu **colere**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « my parents keep fighting » → attendu **tristesse**, prédit **colere**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**

## Erreurs : gemma-m3-int8-fewshot (40)

- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « elle m'a traitée devant tout le monde » → attendu **colere**, prédit **suspense**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **colere**
- « tu crois qu'ils vont dire les résultats aujourd'hui ? » → attendu **suspense**, prédit **calme**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **suspense**
- « il faut ramener une calculatrice » → attendu **calme**, prédit **colere**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « arete de m'enerver » → attendu **colere**, prédit **amour**
- « tu devinera jamais qui ma ecrit » → attendu **suspense**, prédit **colere**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **joie**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « je suis trop contant de t'avoir comme ami » → attendu **amour**, prédit **tristesse**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « je rentre a pied ce soir » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « you said you'd come and you didn't » → attendu **colere**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **tristesse**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**

## Erreurs : gemma-m3-q4-proto (48)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « jpp de cette vidéo de chat » → attendu **joie**, prédit **calme**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **calme**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « laisse-moi tranquille » → attendu **colere**, prédit **calme**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « je me remets au piano, une heure par jour » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
- « j'ai fait une crise d'angoisse avant l'oral » → attendu **tristesse**, prédit **suspense**
- « merci d'avoir été là hier, ça m'a fait du bien » → attendu **amour**, prédit **joie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je suis fou d'elle » → attendu **amour**, prédit **colere**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **suspense**
- « mdr t trop marant » → attendu **joie**, prédit **colere**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **tristesse**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « jen ai marre de ces devoir » → attendu **colere**, prédit **tristesse**
- « arete de m'enerver » → attendu **colere**, prédit **calme**
- « cette anné je donne tout » → attendu **energie**, prédit **calme**
- « tu devinera jamais qui ma ecrit » → attendu **suspense**, prédit **colere**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **calme**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « je rentre a pied ce soir » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **suspense**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « you said you'd come and you didn't » → attendu **colere**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « my parents keep fighting » → attendu **tristesse**, prédit **colere**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**

## Erreurs : gemma-m3-q4-fewshot (38)

- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **calme**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « j'ai fait une crise d'angoisse avant l'oral » → attendu **tristesse**, prédit **suspense**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **suspense**
- « il faut ramener une calculatrice » → attendu **calme**, prédit **colere**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **tristesse**
- « jen ai marre de ces devoir » → attendu **colere**, prédit **tristesse**
- « arete de m'enerver » → attendu **colere**, prédit **amour**
- « tu devinera jamais qui ma ecrit » → attendu **suspense**, prédit **colere**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **joie**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « je rentre a pied ce soir » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **calme**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **suspense**
- « you said you'd come and you didn't » → attendu **colere**, prédit **tristesse**
- « I'm signing up for the half marathon » → attendu **energie**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **tristesse**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**

## Erreurs : gemma-m10-int8-proto (53)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « jpp de cette vidéo de chat » → attendu **joie**, prédit **suspense**
- « haha la tête du prof quand il a vu le gâteau » → attendu **joie**, prédit **colere**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **calme**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « je supporte plus son attitude » → attendu **colere**, prédit **amour**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « elle m'a traitée devant tout le monde » → attendu **colere**, prédit **suspense**
- « laisse-moi tranquille » → attendu **colere**, prédit **calme**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **suspense**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « j'ai fait une crise d'angoisse avant l'oral » → attendu **tristesse**, prédit **suspense**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je kiffe trop être avec toi » → attendu **amour**, prédit **colere**
- « je suis fou d'elle » → attendu **amour**, prédit **colere**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **suspense**
- « mdr t trop marant » → attendu **joie**, prédit **colere**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **tristesse**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « arete de m'enerver » → attendu **colere**, prédit **calme**
- « tu devinera jamais qui ma ecrit » → attendu **suspense**, prédit **colere**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « personne veut jouer avec moi a la recré » → attendu **tristesse**, prédit **calme**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **calme**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « je rentre a pied ce soir » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **suspense**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « you said you'd come and you didn't » → attendu **colere**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « my parents keep fighting » → attendu **tristesse**, prédit **colere**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **tristesse**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**

## Erreurs : gemma-m10-int8-fewshot (46)

- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « elle m'a répondu !!! je suis aux anges » → attendu **joie**, prédit **amour**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **energie**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « elle m'a traitée devant tout le monde » → attendu **colere**, prédit **suspense**
- « laisse-moi tranquille » → attendu **colere**, prédit **calme**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **suspense**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **tristesse**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je kiffe trop être avec toi » → attendu **amour**, prédit **colere**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **suspense**
- « il faut ramener une calculatrice » → attendu **calme**, prédit **colere**
- « ok je regarde et je te dis » → attendu **calme**, prédit **suspense**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « jen ai marre de ces devoir » → attendu **colere**, prédit **tristesse**
- « arete de m'enerver » → attendu **colere**, prédit **amour**
- « tu devinera jamais qui ma ecrit » → attendu **suspense**, prédit **colere**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **joie**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « je rentre a pied ce soir » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « you said you'd come and you didn't » → attendu **colere**, prédit **tristesse**
- « I'm signing up for the half marathon » → attendu **energie**, prédit **calme**
- « morning run at 6, who's in? » → attendu **energie**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **tristesse**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**

## Erreurs : gemma-m10-q4-proto (48)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « haha la tête du prof quand il a vu le gâteau » → attendu **joie**, prédit **colere**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **energie**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « je supporte plus son attitude » → attendu **colere**, prédit **amour**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « laisse-moi tranquille » → attendu **colere**, prédit **calme**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
- « j'ai fait une crise d'angoisse avant l'oral » → attendu **tristesse**, prédit **suspense**
- « merci d'avoir été là hier, ça m'a fait du bien » → attendu **amour**, prédit **joie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **suspense**
- « trop bien les vacance qui arrive » → attendu **joie**, prédit **calme**
- « mdr t trop marant » → attendu **joie**, prédit **colere**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **tristesse**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « jen ai marre de ces devoir » → attendu **colere**, prédit **tristesse**
- « arete de m'enerver » → attendu **colere**, prédit **calme**
- « cette anné je donne tout » → attendu **energie**, prédit **calme**
- « tu devinera jamais qui ma ecrit » → attendu **suspense**, prédit **colere**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **calme**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **suspense**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « you said you'd come and you didn't » → attendu **colere**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « my parents keep fighting » → attendu **tristesse**, prédit **colere**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **tristesse**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**

## Erreurs : gemma-m10-q4-fewshot (43)

- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « elle m'a répondu !!! je suis aux anges » → attendu **joie**, prédit **amour**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **energie**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « elle m'a traitée devant tout le monde » → attendu **colere**, prédit **suspense**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **tristesse**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
- « j'ai fait une crise d'angoisse avant l'oral » → attendu **tristesse**, prédit **suspense**
- « t'es mon rayon de soleil » → attendu **amour**, prédit **calme**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **suspense**
- « il faut ramener une calculatrice » → attendu **calme**, prédit **colere**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **tristesse**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « jen ai marre de ces devoir » → attendu **colere**, prédit **tristesse**
- « arete de m'enerver » → attendu **colere**, prédit **amour**
- « tu devinera jamais qui ma ecrit » → attendu **suspense**, prédit **colere**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **joie**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « je rentre a pied ce soir » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **calme**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « you said you'd come and you didn't » → attendu **colere**, prédit **tristesse**
- « I'm signing up for the half marathon » → attendu **energie**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **tristesse**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**

## Erreurs : minilm-m3-int8-proto (95)

- « on a eu la meilleure note de la classe pour l'exposé ! » → attendu **joie**, prédit **calme**
- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **amour**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **calme**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **colere**
- « je viens de voir mon bulletin, que des félicitations » → attendu **joie**, prédit **amour**
- « jpp de cette vidéo de chat » → attendu **joie**, prédit **suspense**
- « elle m'a répondu !!! je suis aux anges » → attendu **joie**, prédit **suspense**
- « trop bien la soirée pyjama hier » → attendu **joie**, prédit **calme**
- « haha la tête du prof quand il a vu le gâteau » → attendu **joie**, prédit **colere**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **calme**
- « j'ai la haine, ils m'ont pas sélectionné alors que je suis meilleur » → attendu **colere**, prédit **tristesse**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **amour**
- « je supporte plus son attitude » → attendu **colere**, prédit **amour**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **calme**
- « elle m'a traitée devant tout le monde » → attendu **colere**, prédit **amour**
- « ce soir je révise jusqu'à minuit, rien ne m'arrête » → attendu **energie**, prédit **calme**
- « j'ai un truc énorme à te dire... » → attendu **suspense**, prédit **amour**
- « regarde ta boîte aux lettres 👀 » → attendu **suspense**, prédit **calme**
- « y a quelqu'un qui m'a laissé un mot dans mon casier » → attendu **suspense**, prédit **colere**
- « t'es assise ? parce que ce que je vais te dire... » → attendu **suspense**, prédit **calme**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **tristesse**
- « tu crois qu'ils vont dire les résultats aujourd'hui ? » → attendu **suspense**, prédit **energie**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « le prof a dit qu'il avait une annonce importante » → attendu **suspense**, prédit **calme**
- « mon meilleur ami change de collège » → attendu **tristesse**, prédit **colere**
- « ma chienne est malade, le véto est pas optimiste » → attendu **tristesse**, prédit **suspense**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je me sens vide en ce moment » → attendu **tristesse**, prédit **joie**
- « on s'est séparés hier soir 💔 » → attendu **tristesse**, prédit **suspense**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **suspense**
- « câlin virtuel 🤗 » → attendu **amour**, prédit **calme**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je kiffe trop être avec toi » → attendu **amour**, prédit **colere**
- « je suis fou d'elle » → attendu **amour**, prédit **colere**
- « tu finis à quelle heure demain ? » → attendu **calme**, prédit **energie**
- « je suis chez mamie ce week-end » → attendu **calme**, prédit **energie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « c'est quelle page pour l'exercice ? » → attendu **calme**, prédit **energie**
- « on se met à côté en techno ? » → attendu **calme**, prédit **energie**
- « je serai pas là mardi, rdv chez le dentiste » → attendu **calme**, prédit **tristesse**
- « je vais me coucher, à demain » → attendu **calme**, prédit **energie**
- « j'ai mis les photos sur le drive » → attendu **calme**, prédit **joie**
- « demain c'est 8h ou 9h ? » → attendu **calme**, prédit **energie**
- « jsuis trop contante jai eu mon brevet » → attendu **joie**, prédit **colere**
- « sa ma fait trop plaisir ton message » → attendu **joie**, prédit **amour**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **colere**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « arete de m'enerver » → attendu **colere**, prédit **calme**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **tristesse**
- « cette anné je donne tout » → attendu **energie**, prédit **joie**
- « ya un truc bizzare dans mon jardin » → attendu **suspense**, prédit **amour**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **amour**
- « gros bisous mon coeur » → attendu **amour**, prédit **colere**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « jarrive dans 5 min » → attendu **calme**, prédit **energie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **energie**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « got an A on my history test, can't believe it » → attendu **joie**, prédit **colere**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **tristesse**
- « haha you're the funniest person I know » → attendu **joie**, prédit **colere**
- « finally beat that boss level » → attendu **joie**, prédit **energie**
- « best birthday ever, thank you guys 🎉 » → attendu **joie**, prédit **amour**
- « we came first in the science fair » → attendu **joie**, prédit **energie**
- « he read my messages without asking » → attendu **colere**, prédit **tristesse**
- « I'm so annoyed, the bus left without me again » → attendu **colere**, prédit **tristesse**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « leave me alone » → attendu **colere**, prédit **tristesse**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « studying all night, nothing can stop me » → attendu **energie**, prédit **calme**
- « full send on the project tonight » → attendu **energie**, prédit **calme**
- « I have something big to tell you... » → attendu **suspense**, prédit **amour**
- « why did the principal call my parents? » → attendu **suspense**, prédit **tristesse**
- « someone left a note in my locker » → attendu **suspense**, prédit **colere**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « the teacher said she has a big announcement » → attendu **suspense**, prédit **calme**
- « my parents keep fighting » → attendu **tristesse**, prédit **colere**
- « thank you for being there yesterday, it meant a lot » → attendu **amour**, prédit **joie**
- « what time do you finish tomorrow? » → attendu **calme**, prédit **energie**
- « I'm at my grandma's this weekend » → attendu **calme**, prédit **joie**
- « don't forget the permission slip » → attendu **calme**, prédit **suspense**
- « is it 8 or 9 tomorrow? » → attendu **calme**, prédit **energie**
- « I uploaded the pics to the drive » → attendu **calme**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **suspense**
- « wat time is practise » → attendu **calme**, prédit **energie**
- « im omw be there in 5 » → attendu **calme**, prédit **joie**

## Erreurs : minilm-m3-int8-fewshot (96)

- « on a eu la meilleure note de la classe pour l'exposé ! » → attendu **joie**, prédit **calme**
- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **colere**
- « j'ai enfin réussi à passer le niveau 50 » → attendu **joie**, prédit **energie**
- « je viens de voir mon bulletin, que des félicitations » → attendu **joie**, prédit **calme**
- « jpp de cette vidéo de chat » → attendu **joie**, prédit **suspense**
- « elle m'a répondu !!! je suis aux anges » → attendu **joie**, prédit **suspense**
- « trop bien la soirée pyjama hier » → attendu **joie**, prédit **calme**
- « j'ai eu un 19 en physique, je rêve » → attendu **joie**, prédit **calme**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **tristesse**
- « j'ai la haine, ils m'ont pas sélectionné alors que je suis meilleur » → attendu **colere**, prédit **tristesse**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **calme**
- « ma sœur a lu mon journal, je vais la tuer » → attendu **colere**, prédit **suspense**
- « je supporte plus son attitude » → attendu **colere**, prédit **amour**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **calme**
- « ce soir je révise jusqu'à minuit, rien ne m'arrête » → attendu **energie**, prédit **calme**
- « je me remets au piano, une heure par jour » → attendu **energie**, prédit **calme**
- « j'ai un truc énorme à te dire... » → attendu **suspense**, prédit **amour**
- « qui a posté ça sur le groupe ?? » → attendu **suspense**, prédit **joie**
- « regarde ta boîte aux lettres 👀 » → attendu **suspense**, prédit **calme**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **tristesse**
- « y a une voiture bizarre garée devant chez moi depuis ce matin » → attendu **suspense**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **tristesse**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **calme**
- « tu crois qu'ils vont dire les résultats aujourd'hui ? » → attendu **suspense**, prédit **energie**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « le prof a dit qu'il avait une annonce importante » → attendu **suspense**, prédit **calme**
- « mon meilleur ami change de collège » → attendu **tristesse**, prédit **colere**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je me sens vide en ce moment » → attendu **tristesse**, prédit **calme**
- « on s'est séparés hier soir 💔 » → attendu **tristesse**, prédit **suspense**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **suspense**
- « merci d'avoir été là hier, ça m'a fait du bien » → attendu **amour**, prédit **calme**
- « t'es trop adorable quand tu fais ça » → attendu **amour**, prédit **joie**
- « je suis trop fière d'être ta sœur » → attendu **amour**, prédit **joie**
- « câlin virtuel 🤗 » → attendu **amour**, prédit **calme**
- « il est trop mignon avec moi » → attendu **amour**, prédit **joie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je kiffe trop être avec toi » → attendu **amour**, prédit **joie**
- « je suis fou d'elle » → attendu **amour**, prédit **colere**
- « je suis chez mamie ce week-end » → attendu **calme**, prédit **energie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « ma mère passe me chercher à 17h30 » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **energie**
- « je serai pas là mardi, rdv chez le dentiste » → attendu **calme**, prédit **tristesse**
- « je vais me coucher, à demain » → attendu **calme**, prédit **energie**
- « j'ai mis les photos sur le drive » → attendu **calme**, prédit **joie**
- « sa ma fait trop plaisir ton message » → attendu **joie**, prédit **amour**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **tristesse**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « tu ma menti sa se fait pas » → attendu **colere**, prédit **suspense**
- « arete de m'enerver » → attendu **colere**, prédit **energie**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **tristesse**
- « ya un truc bizzare dans mon jardin » → attendu **suspense**, prédit **amour**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **calme**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **suspense**
- « gros bisous mon coeur » → attendu **amour**, prédit **colere**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « jarrive dans 5 min » → attendu **calme**, prédit **energie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **energie**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « haha you're the funniest person I know » → attendu **joie**, prédit **colere**
- « finally beat that boss level » → attendu **joie**, prédit **energie**
- « we came first in the science fair » → attendu **joie**, prédit **energie**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **tristesse**
- « he read my messages without asking » → attendu **colere**, prédit **calme**
- « I'm so annoyed, the bus left without me again » → attendu **colere**, prédit **tristesse**
- « you said you'd come and you didn't » → attendu **colere**, prédit **calme**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « leave me alone » → attendu **colere**, prédit **tristesse**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « studying all night, nothing can stop me » → attendu **energie**, prédit **calme**
- « full send on the project tonight » → attendu **energie**, prédit **calme**
- « there's a weird car parked outside my house » → attendu **suspense**, prédit **tristesse**
- « why did the principal call my parents? » → attendu **suspense**, prédit **colere**
- « someone left a note in my locker » → attendu **suspense**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « the teacher said she has a big announcement » → attendu **suspense**, prédit **calme**
- « my best friend is moving to another state » → attendu **tristesse**, prédit **suspense**
- « my parents keep fighting » → attendu **tristesse**, prédit **colere**
- « goodnight sweetheart » → attendu **amour**, prédit **calme**
- « I'm at my grandma's this weekend » → attendu **calme**, prédit **joie**
- « my mom picks me up at 5:30 » → attendu **calme**, prédit **suspense**
- « is it 8 or 9 tomorrow? » → attendu **calme**, prédit **suspense**
- « I uploaded the pics to the drive » → attendu **calme**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « were gonna win tommorow lets go » → attendu **energie**, prédit **joie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **suspense**
- « wat time is practise » → attendu **calme**, prédit **energie**
- « im omw be there in 5 » → attendu **calme**, prédit **suspense**

## Erreurs : gemma-orig-q4-proto (46)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « jpp de cette vidéo de chat » → attendu **joie**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « laisse-moi tranquille » → attendu **colere**, prédit **calme**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « je me remets au piano, une heure par jour » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
- « j'ai fait une crise d'angoisse avant l'oral » → attendu **tristesse**, prédit **suspense**
- « merci d'avoir été là hier, ça m'a fait du bien » → attendu **amour**, prédit **joie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je suis fou d'elle » → attendu **amour**, prédit **colere**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **suspense**
- « mdr t trop marant » → attendu **joie**, prédit **colere**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **tristesse**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « jen ai marre de ces devoir » → attendu **colere**, prédit **tristesse**
- « arete de m'enerver » → attendu **colere**, prédit **calme**
- « tu devinera jamais qui ma ecrit » → attendu **suspense**, prédit **colere**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **calme**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « je rentre a pied ce soir » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « you said you'd come and you didn't » → attendu **colere**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « my parents keep fighting » → attendu **tristesse**, prédit **colere**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**

## Erreurs : gemma-orig-q4-fewshot (36)

- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **tristesse**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « j'ai fait une crise d'angoisse avant l'oral » → attendu **tristesse**, prédit **suspense**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **suspense**
- « il faut ramener une calculatrice » → attendu **calme**, prédit **colere**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **tristesse**
- « jen ai marre de ces devoir » → attendu **colere**, prédit **tristesse**
- « arete de m'enerver » → attendu **colere**, prédit **amour**
- « tu devinera jamais qui ma ecrit » → attendu **suspense**, prédit **colere**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **joie**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « je rentre a pied ce soir » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **calme**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « you said you'd come and you didn't » → attendu **colere**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **tristesse**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**
