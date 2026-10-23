# Résultats benchmark émotions — test_v2.csv

301 messages dont 70 avec fautes. Un message vaut 0.3 points au global et 1.4 points dans la colonne « Avec fautes ».

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |
|---|---|---|---|---|---|---|---|---|---|
| gemma2-q4-fewshot | 82.4 % | 83.7 % | 80.1 % | 86.7 % | 81.4 % | 82.7 % | 50.6 ms | 119.7 Mo | choix typé (fewshot), ONNX model_q4.onnx |
| gemma2-q4-proto | 75.7 % | 77.1 % | 73.0 % | 81.0 % | 74.3 % | 76.2 % | 45.4 ms | 119.7 Mo | choix typé (proto), ONNX model_q4.onnx |

## Rappel par émotion

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|---|---|---|---|---|---|---|
| gemma2-q4-proto | 62.8 % | 72.1 % | 81.4 % | 88.4 % | 67.4 % | 83.7 % | 74.4 % |
| gemma2-q4-fewshot | 76.7 % | 79.1 % | 88.4 % | 86.0 % | 79.1 % | 90.7 % | 76.7 % |

## Confiance (choix typé)

Précision tolérante sur les messages dont la confiance dépasse le seuil, et part des messages concernés.

| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |
|---|---|---|---|---|---|
| gemma2-q4-proto | 0.14 | 98.3 % sur 19 % | 100.0 % sur 6 % | 100.0 % sur 3 % | 100.0 % sur 1 % |
| gemma2-q4-fewshot | 0.44 | 88.7 % sur 85 % | 93.3 % sur 69 % | 95.6 % sur 53 % | 98.2 % sur 37 % |

## Erreurs : gemma2-q4-proto (69)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **suspense**
- « jpp de cette vidéo de chat » → attendu **joie**, prédit **colere**
- « j'ai eu un 19 en physique, je rêve » → attendu **joie**, prédit **energie**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **suspense**
- « je supporte plus son attitude » → attendu **colere**, prédit **energie**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **suspense**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « on lâche rien les gars » → attendu **energie**, prédit **colere**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « il s'est passé un truc de fou à la cantine » → attendu **suspense**, prédit **colere**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « mon grand-père est parti cette nuit » → attendu **tristesse**, prédit **calme**
- « mon meilleur ami change de collège » → attendu **tristesse**, prédit **suspense**
- « j'ai encore eu une mauvaise note, je suis nul » → attendu **tristesse**, prédit **colere**
- « mon vélo a été volé, c'était un cadeau de ma mamie » → attendu **tristesse**, prédit **suspense**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **suspense**
- « merci d'avoir été là hier, ça m'a fait du bien » → attendu **amour**, prédit **joie**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **suspense**
- « t'es mon rayon de soleil » → attendu **amour**, prédit **suspense**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je suis fou d'elle » → attendu **amour**, prédit **colere**
- « t'as le numéro de Tom ? » → attendu **calme**, prédit **suspense**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « je prends mon goûter et j'arrive » → attendu **calme**, prédit **energie**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **suspense**
- « ok je regarde et je te dis » → attendu **calme**, prédit **suspense**
- « j'ai mis les photos sur le drive » → attendu **calme**, prédit **joie**
- « sa ma fait trop plaisir ton message » → attendu **joie**, prédit **suspense**
- « mdr t trop marant » → attendu **joie**, prédit **colere**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **suspense**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « arete de m'enerver » → attendu **colere**, prédit **energie**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **suspense**
- « on y vas les gars on va gagné » → attendu **energie**, prédit **joie**
- « jai une nouvelle de ouf a tannoncer » → attendu **suspense**, prédit **colere**
- « jai rater mon examen je suis degouter » → attendu **tristesse**, prédit **suspense**
- « ont a perdu la finale » → attendu **tristesse**, prédit **suspense**
- « merci detre la pour moi » → attendu **amour**, prédit **colere**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « jarrive dans 5 min » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « my mom said I can get a puppy!!! » → attendu **joie**, prédit **suspense**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « let's gooo, game day 🔥 » → attendu **energie**, prédit **joie**
- « morning run at 6, who's in? » → attendu **energie**, prédit **calme**
- « full send on the project tonight » → attendu **energie**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **colere**
- « everyone was invited except me » → attendu **tristesse**, prédit **suspense**
- « I failed the test again, I'm so dumb » → attendu **tristesse**, prédit **colere**
- « going to bed, see you tomorrow » → attendu **calme**, prédit **energie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **suspense**
- « my cat dyed this morning » → attendu **tristesse**, prédit **suspense**
- « i miss you alot » → attendu **tristesse**, prédit **amour**

## Erreurs : gemma2-q4-fewshot (49)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **suspense**
- « j'ai eu un 19 en physique, je rêve » → attendu **joie**, prédit **energie**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **suspense**
- « je supporte plus son attitude » → attendu **colere**, prédit **energie**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **suspense**
- « on lâche rien les gars » → attendu **energie**, prédit **colere**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « il s'est passé un truc de fou à la cantine » → attendu **suspense**, prédit **colere**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « mon grand-père est parti cette nuit » → attendu **tristesse**, prédit **calme**
- « mon vélo a été volé, c'était un cadeau de ma mamie » → attendu **tristesse**, prédit **suspense**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **suspense**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **joie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « je prends mon goûter et j'arrive » → attendu **calme**, prédit **energie**
- « on se met à côté en techno ? » → attendu **calme**, prédit **energie**
- « je vais me coucher, à demain » → attendu **calme**, prédit **energie**
- « ok je regarde et je te dis » → attendu **calme**, prédit **suspense**
- « j'ai mis les photos sur le drive » → attendu **calme**, prédit **joie**
- « mdr t trop marant » → attendu **joie**, prédit **colere**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **tristesse**
- « on y vas les gars on va gagné » → attendu **energie**, prédit **joie**
- « jai une nouvelle de ouf a tannoncer » → attendu **suspense**, prédit **colere**
- « jai rater mon examen je suis degouter » → attendu **tristesse**, prédit **energie**
- « merci detre la pour moi » → attendu **amour**, prédit **colere**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « jarrive dans 5 min » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « my mom said I can get a puppy!!! » → attendu **joie**, prédit **suspense**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « morning run at 6, who's in? » → attendu **energie**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **colere**
- « going to bed, see you tomorrow » → attendu **calme**, prédit **energie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **suspense**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**
- « i miss you alot » → attendu **tristesse**, prédit **amour**
