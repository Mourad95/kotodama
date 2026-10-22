# Résultats benchmark émotions — test_v2.csv

301 messages dont 70 avec fautes. Un message vaut 0.3 points au global et 1.4 points dans la colonne « Avec fautes ».

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |
|---|---|---|---|---|---|---|---|---|---|
| bgem3-m3-q4-fewshot | 80.7 % | 81.7 % | 80.1 % | 81.9 % | 81.4 % | 80.5 % | 37.8 ms | 224.0 Mo | choix typé (fewshot), ONNX model_q4.onnx |
| bgem3-m3-q4-proto | 78.4 % | 78.7 % | 77.6 % | 80.0 % | 78.6 % | 78.4 % | 37.9 ms | 224.0 Mo | choix typé (proto), ONNX model_q4.onnx |

## Rappel par émotion

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|---|---|---|---|---|---|---|
| bgem3-m3-q4-proto | 72.1 % | 60.5 % | 90.7 % | 76.7 % | 81.4 % | 90.7 % | 76.7 % |
| bgem3-m3-q4-fewshot | 74.4 % | 60.5 % | 95.3 % | 83.7 % | 83.7 % | 93.0 % | 74.4 % |

## Confiance (choix typé)

Précision tolérante sur les messages dont la confiance dépasse le seuil, et part des messages concernés.

| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |
|---|---|---|---|---|---|
| bgem3-m3-q4-proto | 0.17 | 87.2 % sur 31 % | 97.4 % sur 13 % | 100.0 % sur 4 % | 100.0 % sur 1 % |
| bgem3-m3-q4-fewshot | 0.28 | 89.0 % sur 60 % | 92.8 % sur 37 % | 97.1 % sur 23 % | 100.0 % sur 11 % |

## Erreurs : bgem3-m3-q4-proto (64)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **tristesse**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **tristesse**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **tristesse**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **tristesse**
- « j'ai la haine, ils m'ont pas sélectionné alors que je suis meilleur » → attendu **colere**, prédit **tristesse**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **suspense**
- « j'ai le seum contre l'arbitre, il était nul » → attendu **colere**, prédit **tristesse**
- « je supporte plus son attitude » → attendu **colere**, prédit **energie**
- « pourquoi tu fais exprès de m'ignorer ? » → attendu **colere**, prédit **tristesse**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « laisse-moi tranquille » → attendu **colere**, prédit **calme**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « chaud pour la rando de 20 km » → attendu **energie**, prédit **joie**
- « regarde ta boîte aux lettres 👀 » → attendu **suspense**, prédit **calme**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **tristesse**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **tristesse**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je me sens vide en ce moment » → attendu **tristesse**, prédit **calme**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « câlin virtuel 🤗 » → attendu **amour**, prédit **joie**
- « il est trop mignon avec moi » → attendu **amour**, prédit **colere**
- « je suis fou d'elle » → attendu **amour**, prédit **colere**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « ma mère passe me chercher à 17h30 » → attendu **calme**, prédit **tristesse**
- « on se met à côté en techno ? » → attendu **calme**, prédit **tristesse**
- « je serai pas là mardi, rdv chez le dentiste » → attendu **calme**, prédit **tristesse**
- « je vais me coucher, à demain » → attendu **calme**, prédit **energie**
- « c abusé il ma pas attendu » → attendu **colere**, prédit **suspense**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **tristesse**
- « cette anné je donne tout » → attendu **energie**, prédit **colere**
- « ya un truc bizzare dans mon jardin » → attendu **suspense**, prédit **calme**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « jai rater mon examen je suis degouter » → attendu **tristesse**, prédit **joie**
- « personne veut jouer avec moi a la recré » → attendu **tristesse**, prédit **energie**
- « tu me manque trop » → attendu **tristesse**, prédit **colere**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **amour**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **tristesse**
- « haha you're the funniest person I know » → attendu **joie**, prédit **amour**
- « finally beat that boss level » → attendu **joie**, prédit **energie**
- « best birthday ever, thank you guys 🎉 » → attendu **joie**, prédit **amour**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « I'm so annoyed, the bus left without me again » → attendu **colere**, prédit **tristesse**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « leave me alone » → attendu **colere**, prédit **tristesse**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « the ref was a joke, we got robbed » → attendu **colere**, prédit **joie**
- « there's a weird car parked outside my house » → attendu **suspense**, prédit **tristesse**
- « why did the principal call my parents? » → attendu **suspense**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **tristesse**
- « I uploaded the pics to the drive » → attendu **calme**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **tristesse**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « i miss you alot » → attendu **tristesse**, prédit **amour**
- « im omw be there in 5 » → attendu **calme**, prédit **energie**

## Erreurs : bgem3-m3-q4-fewshot (55)

- « il neige !!! pas cours demain » → attendu **joie**, prédit **tristesse**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **tristesse**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **tristesse**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **tristesse**
- « j'ai la haine, ils m'ont pas sélectionné alors que je suis meilleur » → attendu **colere**, prédit **tristesse**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **suspense**
- « j'ai le seum contre l'arbitre, il était nul » → attendu **colere**, prédit **tristesse**
- « je supporte plus son attitude » → attendu **colere**, prédit **energie**
- « pourquoi tu fais exprès de m'ignorer ? » → attendu **colere**, prédit **tristesse**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **tristesse**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « je me sens vide en ce moment » → attendu **tristesse**, prédit **calme**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « il est trop mignon avec moi » → attendu **amour**, prédit **colere**
- « je suis fou d'elle » → attendu **amour**, prédit **colere**
- « je suis chez mamie ce week-end » → attendu **calme**, prédit **tristesse**
- « vu, merci » → attendu **calme**, prédit **amour**
- « ma mère passe me chercher à 17h30 » → attendu **calme**, prédit **tristesse**
- « on se met à côté en techno ? » → attendu **calme**, prédit **tristesse**
- « je serai pas là mardi, rdv chez le dentiste » → attendu **calme**, prédit **tristesse**
- « je vais me coucher, à demain » → attendu **calme**, prédit **energie**
- « c abusé il ma pas attendu » → attendu **colere**, prédit **suspense**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **tristesse**
- « cette anné je donne tout » → attendu **energie**, prédit **colere**
- « ya un truc bizzare dans mon jardin » → attendu **suspense**, prédit **calme**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « jai rater mon examen je suis degouter » → attendu **tristesse**, prédit **joie**
- « personne veut jouer avec moi a la recré » → attendu **tristesse**, prédit **energie**
- « tu me manque trop » → attendu **tristesse**, prédit **amour**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **amour**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « got an A on my history test, can't believe it » → attendu **joie**, prédit **suspense**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **tristesse**
- « finally beat that boss level » → attendu **joie**, prédit **energie**
- « best birthday ever, thank you guys 🎉 » → attendu **joie**, prédit **amour**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « I'm so annoyed, the bus left without me again » → attendu **colere**, prédit **tristesse**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « leave me alone » → attendu **colere**, prédit **tristesse**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « the ref was a joke, we got robbed » → attendu **colere**, prédit **joie**
- « why did the principal call my parents? » → attendu **suspense**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **tristesse**
- « I'm at my grandma's this weekend » → attendu **calme**, prédit **tristesse**
- « I uploaded the pics to the drive » → attendu **calme**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **tristesse**
- « im omw be there in 5 » → attendu **calme**, prédit **energie**
