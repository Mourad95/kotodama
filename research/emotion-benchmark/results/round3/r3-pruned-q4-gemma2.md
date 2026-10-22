# Résultats benchmark émotions — test_v2.csv

301 messages dont 70 avec fautes. Un message vaut 0.3 points au global et 1.4 points dans la colonne « Avec fautes ».

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |
|---|---|---|---|---|---|---|---|---|---|
| gemma2-m3-q4-fewshot | 80.4 % | 81.7 % | 78.6 % | 83.8 % | 81.4 % | 80.1 % | 54.3 ms | 119.7 Mo | choix typé (fewshot), ONNX model_q4.onnx |
| gemma2-m3-q4-proto | 79.1 % | 80.4 % | 76.5 % | 83.8 % | 80.0 % | 78.8 % | 51.4 ms | 119.7 Mo | choix typé (proto), ONNX model_q4.onnx |

## Rappel par émotion

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|---|---|---|---|---|---|---|
| gemma2-m3-q4-proto | 72.1 % | 74.4 % | 86.0 % | 81.4 % | 65.1 % | 88.4 % | 86.0 % |
| gemma2-m3-q4-fewshot | 81.4 % | 74.4 % | 86.0 % | 76.7 % | 74.4 % | 90.7 % | 79.1 % |

## Confiance (choix typé)

Précision tolérante sur les messages dont la confiance dépasse le seuil, et part des messages concernés.

| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |
|---|---|---|---|---|---|
| gemma2-m3-q4-proto | 0.12 | 100.0 % sur 13 % | 100.0 % sur 5 % | 100.0 % sur 1 % | 100.0 % sur 0 % |
| gemma2-m3-q4-fewshot | 0.3 | 90.6 % sur 67 % | 98.5 % sur 44 % | 98.8 % sur 27 % | 100.0 % sur 15 % |

## Erreurs : gemma2-m3-q4-proto (59)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **suspense**
- « jpp de cette vidéo de chat » → attendu **joie**, prédit **colere**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **suspense**
- « je supporte plus son attitude » → attendu **colere**, prédit **energie**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « elle m'a traitée devant tout le monde » → attendu **colere**, prédit **suspense**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **suspense**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « on lâche rien les gars » → attendu **energie**, prédit **colere**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « chaud pour la rando de 20 km » → attendu **energie**, prédit **calme**
- « regarde ta boîte aux lettres 👀 » → attendu **suspense**, prédit **calme**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « il s'est passé un truc de fou à la cantine » → attendu **suspense**, prédit **colere**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **calme**
- « ne dis rien à personne mais... » → attendu **suspense**, prédit **colere**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « le prof a dit qu'il avait une annonce importante » → attendu **suspense**, prédit **calme**
- « mon grand-père est parti cette nuit » → attendu **tristesse**, prédit **calme**
- « mon meilleur ami change de collège » → attendu **tristesse**, prédit **suspense**
- « j'ai encore eu une mauvaise note, je suis nul » → attendu **tristesse**, prédit **colere**
- « mon vélo a été volé, c'était un cadeau de ma mamie » → attendu **tristesse**, prédit **suspense**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **suspense**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **suspense**
- « t'es mon rayon de soleil » → attendu **amour**, prédit **suspense**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « ok je regarde et je te dis » → attendu **calme**, prédit **suspense**
- « j'ai mis les photos sur le drive » → attendu **calme**, prédit **joie**
- « mdr t trop marant » → attendu **joie**, prédit **colere**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « c abusé il ma pas attendu » → attendu **colere**, prédit **suspense**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **suspense**
- « jai rater mon examen je suis degouter » → attendu **tristesse**, prédit **energie**
- « ont a perdu la finale » → attendu **tristesse**, prédit **suspense**
- « merci detre la pour moi » → attendu **amour**, prédit **colere**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « jarrive dans 5 min » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « my mom said I can get a puppy!!! » → attendu **joie**, prédit **suspense**
- « he read my messages without asking » → attendu **colere**, prédit **calme**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « morning run at 6, who's in? » → attendu **energie**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **colere**
- « my best friend is moving to another state » → attendu **tristesse**, prédit **suspense**
- « my parents keep fighting » → attendu **tristesse**, prédit **colere**
- « I failed the test again, I'm so dumb » → attendu **tristesse**, prédit **colere**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **suspense**
- « my cat dyed this morning » → attendu **tristesse**, prédit **suspense**
- « i miss you alot » → attendu **tristesse**, prédit **amour**

## Erreurs : gemma2-m3-q4-fewshot (55)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **tristesse**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **amour**
- « je supporte plus son attitude » → attendu **colere**, prédit **energie**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « elle m'a traitée devant tout le monde » → attendu **colere**, prédit **suspense**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **suspense**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « chaud pour la rando de 20 km » → attendu **energie**, prédit **calme**
- « regarde ta boîte aux lettres 👀 » → attendu **suspense**, prédit **calme**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « il s'est passé un truc de fou à la cantine » → attendu **suspense**, prédit **colere**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **calme**
- « ne dis rien à personne mais... » → attendu **suspense**, prédit **colere**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « mon grand-père est parti cette nuit » → attendu **tristesse**, prédit **calme**
- « j'ai encore eu une mauvaise note, je suis nul » → attendu **tristesse**, prédit **colere**
- « mon vélo a été volé, c'était un cadeau de ma mamie » → attendu **tristesse**, prédit **suspense**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **suspense**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **suspense**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « je vais me coucher, à demain » → attendu **calme**, prédit **energie**
- « ok je regarde et je te dis » → attendu **calme**, prédit **suspense**
- « j'ai mis les photos sur le drive » → attendu **calme**, prédit **joie**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **tristesse**
- « cette anné je donne tout » → attendu **energie**, prédit **joie**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « jai rater mon examen je suis degouter » → attendu **tristesse**, prédit **energie**
- « merci detre la pour moi » → attendu **amour**, prédit **colere**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « jarrive dans 5 min » → attendu **calme**, prédit **energie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « my mom said I can get a puppy!!! » → attendu **joie**, prédit **suspense**
- « he read my messages without asking » → attendu **colere**, prédit **calme**
- « you said you'd come and you didn't » → attendu **colere**, prédit **calme**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « the ref was a joke, we got robbed » → attendu **colere**, prédit **tristesse**
- « morning run at 6, who's in? » → attendu **energie**, prédit **calme**
- « someone left a note in my locker » → attendu **suspense**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « I failed the test again, I'm so dumb » → attendu **tristesse**, prédit **colere**
- « going to bed, see you tomorrow » → attendu **calme**, prédit **energie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **suspense**
- « my cat dyed this morning » → attendu **tristesse**, prédit **suspense**
- « i miss you alot » → attendu **tristesse**, prédit **amour**
