# Résultats benchmark émotions — test_v2.csv

301 messages dont 70 avec fautes. Un message vaut 0.3 points au global et 1.4 points dans la colonne « Avec fautes ».

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |
|---|---|---|---|---|---|---|---|---|---|
| gemma2-q4-fewshot | 83.7 % | 85.4 % | 82.1 % | 86.7 % | 84.3 % | 83.5 % | 52.1 ms | 119.7 Mo | choix typé (fewshot), ONNX model_q4.onnx |
| gemma2-q4-proto | 78.1 % | 79.1 % | 76.0 % | 81.9 % | 77.1 % | 78.4 % | 44.0 ms | 119.7 Mo | choix typé (proto), ONNX model_q4.onnx |

## Rappel par émotion

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|---|---|---|---|---|---|---|
| gemma2-q4-proto | 72.1 % | 58.1 % | 93.0 % | 81.4 % | 72.1 % | 88.4 % | 81.4 % |
| gemma2-q4-fewshot | 81.4 % | 74.4 % | 90.7 % | 88.4 % | 79.1 % | 88.4 % | 83.7 % |

## Confiance (choix typé)

Précision tolérante sur les messages dont la confiance dépasse le seuil, et part des messages concernés.

| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |
|---|---|---|---|---|---|
| gemma2-q4-proto | 0.1 | 100.0 % sur 8 % | 100.0 % sur 4 % | 100.0 % sur 1 % | 100.0 % sur 0 % |
| gemma2-q4-fewshot | 0.61 | 87.5 % sur 95 % | 89.4 % sur 88 % | 92.2 % sur 77 % | 94.1 % sur 67 % |

## Erreurs : gemma2-q4-proto (63)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **suspense**
- « il a encore pris mon chargeur sans demander » → attendu **colere**, prédit **suspense**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **joie**
- « je supporte plus son attitude » → attendu **colere**, prédit **energie**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « elle m'a traitée devant tout le monde » → attendu **colere**, prédit **suspense**
- « c'est la troisième fois qu'on me vole ma trousse, j'en peux plus » → attendu **colere**, prédit **tristesse**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **suspense**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « il s'est passé un truc de fou à la cantine » → attendu **suspense**, prédit **colere**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **calme**
- « ne dis rien à personne mais... » → attendu **suspense**, prédit **colere**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « mon grand-père est parti cette nuit » → attendu **tristesse**, prédit **calme**
- « mon meilleur ami change de collège » → attendu **tristesse**, prédit **calme**
- « mon vélo a été volé, c'était un cadeau de ma mamie » → attendu **tristesse**, prédit **suspense**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **suspense**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **joie**
- « t'es mon rayon de soleil » → attendu **amour**, prédit **suspense**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « t'as le numéro de Tom ? » → attendu **calme**, prédit **suspense**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **energie**
- « ok je regarde et je te dis » → attendu **calme**, prédit **suspense**
- « mdr t trop marant » → attendu **joie**, prédit **colere**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « il ma piquer mon stylo encore » → attendu **colere**, prédit **suspense**
- « c abusé il ma pas attendu » → attendu **colere**, prédit **suspense**
- « arete de m'enerver » → attendu **colere**, prédit **energie**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **suspense**
- « jai une nouvelle de ouf a tannoncer » → attendu **suspense**, prédit **colere**
- « jai rater mon examen je suis degouter » → attendu **tristesse**, prédit **joie**
- « ont a perdu la finale » → attendu **tristesse**, prédit **suspense**
- « merci detre la pour moi » → attendu **amour**, prédit **colere**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « my mom said I can get a puppy!!! » → attendu **joie**, prédit **suspense**
- « we came first in the science fair » → attendu **joie**, prédit **energie**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « you said you'd come and you didn't » → attendu **colere**, prédit **calme**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « the ref was a joke, we got robbed » → attendu **colere**, prédit **tristesse**
- « morning run at 6, who's in? » → attendu **energie**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **colere**
- « the teacher said she has a big announcement » → attendu **suspense**, prédit **calme**
- « everyone was invited except me » → attendu **tristesse**, prédit **suspense**
- « going to bed, see you tomorrow » → attendu **calme**, prédit **energie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **suspense**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**
- « i miss you alot » → attendu **tristesse**, prédit **amour**

## Erreurs : gemma2-q4-fewshot (44)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **tristesse**
- « j'ai la haine, ils m'ont pas sélectionné alors que je suis meilleur » → attendu **colere**, prédit **tristesse**
- « ma sœur a lu mon journal, je vais la tuer » → attendu **colere**, prédit **suspense**
- « je supporte plus son attitude » → attendu **colere**, prédit **amour**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « c'est la troisième fois qu'on me vole ma trousse, j'en peux plus » → attendu **colere**, prédit **tristesse**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **suspense**
- « je me remets au piano, une heure par jour » → attendu **energie**, prédit **tristesse**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **calme**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « mon grand-père est parti cette nuit » → attendu **tristesse**, prédit **calme**
- « mon vélo a été volé, c'était un cadeau de ma mamie » → attendu **tristesse**, prédit **suspense**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **joie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je suis fou d'elle » → attendu **amour**, prédit **colere**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « on se met à côté en techno ? » → attendu **calme**, prédit **energie**
- « ok je regarde et je te dis » → attendu **calme**, prédit **suspense**
- « jai une nouvelle de ouf a tannoncer » → attendu **suspense**, prédit **colere**
- « jai rater mon examen je suis degouter » → attendu **tristesse**, prédit **energie**
- « merci detre la pour moi » → attendu **amour**, prédit **colere**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « jarrive dans 5 min » → attendu **calme**, prédit **energie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **calme**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « we came first in the science fair » → attendu **joie**, prédit **energie**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « you said you'd come and you didn't » → attendu **colere**, prédit **calme**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « morning run at 6, who's in? » → attendu **energie**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **colere**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **suspense**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**
- « i miss you alot » → attendu **tristesse**, prédit **amour**
