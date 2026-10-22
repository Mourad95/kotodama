# Résultats benchmark émotions — test_v2.csv

301 messages dont 70 avec fautes. Un message vaut 0.3 points au global et 1.4 points dans la colonne « Avec fautes ».

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |
|---|---|---|---|---|---|---|---|---|---|
| gemma-fewshot | 85.0 % | 86.4 % | 82.1 % | 90.5 % | 80.0 % | 86.6 % | 31.1 ms | 1269.6 Mo | choix typé (fewshot), PyTorch |
| gemma-proto | 83.1 % | 84.4 % | 80.6 % | 87.6 % | 78.6 % | 84.4 % | 30.8 ms | 1269.6 Mo | choix typé (proto), PyTorch |
| e5small-fewshot | 71.1 % | 72.4 % | 68.4 % | 76.2 % | 74.3 % | 70.1 % | 7.6 ms | 964.0 Mo | choix typé (fewshot), PyTorch |
| e5small-proto | 69.4 % | 70.8 % | 67.9 % | 72.4 % | 75.7 % | 67.5 % | 7.5 ms | 964.0 Mo | choix typé (proto), PyTorch |
| minilm-proto | 68.8 % | 69.8 % | 67.3 % | 71.4 % | 75.7 % | 66.7 % | 7.3 ms | 970.3 Mo | choix typé (proto), PyTorch |
| minilm-fewshot | 68.4 % | 69.1 % | 67.3 % | 70.5 % | 78.6 % | 65.4 % | 7.6 ms | 970.3 Mo | choix typé (fewshot), PyTorch |

## Rappel par émotion

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|---|---|---|---|---|---|---|
| gemma-proto | 67.4 % | 83.7 % | 88.4 % | 90.7 % | 83.7 % | 86.0 % | 81.4 % |
| gemma-fewshot | 81.4 % | 76.7 % | 88.4 % | 90.7 % | 86.0 % | 90.7 % | 81.4 % |
| minilm-proto | 48.8 % | 65.1 % | 88.4 % | 62.8 % | 74.4 % | 86.0 % | 55.8 % |
| minilm-fewshot | 53.5 % | 65.1 % | 88.4 % | 65.1 % | 74.4 % | 79.1 % | 53.5 % |
| e5small-proto | 62.8 % | 55.8 % | 72.1 % | 65.1 % | 79.1 % | 81.4 % | 69.8 % |
| e5small-fewshot | 65.1 % | 53.5 % | 86.0 % | 67.4 % | 74.4 % | 81.4 % | 69.8 % |

## Confiance (choix typé)

Précision tolérante sur les messages dont la confiance dépasse le seuil, et part des messages concernés.

| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |
|---|---|---|---|---|---|
| gemma-proto | 0.47 | 87.7 % sur 89 % | 91.0 % sur 74 % | 94.3 % sur 58 % | 96.2 % sur 44 % |
| gemma-fewshot | 0.44 | 90.6 % sur 85 % | 92.9 % sur 70 % | 95.7 % sur 54 % | 99.1 % sur 38 % |
| minilm-proto | 0.48 | 71.9 % sur 91 % | 77.0 % sur 74 % | 82.2 % sur 61 % | 82.6 % sur 46 % |
| minilm-fewshot | 0.37 | 76.3 % sur 81 % | 83.0 % sur 57 % | 85.7 % sur 40 % | 89.7 % sur 23 % |
| e5small-proto | 0.01 | None % sur 0 % | None % sur 0 % | None % sur 0 % | None % sur 0 % |
| e5small-fewshot | 0.12 | 97.9 % sur 16 % | 100.0 % sur 6 % | 100.0 % sur 1 % | None % sur 0 % |

## Erreurs : gemma-proto (47)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « jpp de cette vidéo de chat » → attendu **joie**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « je supporte plus son attitude » → attendu **colere**, prédit **amour**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « je me remets au piano, une heure par jour » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « j'ai fait une crise d'angoisse avant l'oral » → attendu **tristesse**, prédit **suspense**
- « merci d'avoir été là hier, ça m'a fait du bien » → attendu **amour**, prédit **joie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je kiffe trop être avec toi » → attendu **amour**, prédit **colere**
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
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **calme**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « je suis trop contant de t'avoir comme ami » → attendu **amour**, prédit **tristesse**
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

## Erreurs : gemma-fewshot (41)

- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « elle m'a traitée devant tout le monde » → attendu **colere**, prédit **suspense**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « je me remets au piano, une heure par jour » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « tu crois qu'ils vont dire les résultats aujourd'hui ? » → attendu **suspense**, prédit **calme**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « j'ai fait une crise d'angoisse avant l'oral » → attendu **tristesse**, prédit **suspense**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **suspense**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **tristesse**
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
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **calme**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « you said you'd come and you didn't » → attendu **colere**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **tristesse**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**

## Erreurs : minilm-proto (91)

- « on a eu la meilleure note de la classe pour l'exposé ! » → attendu **joie**, prédit **calme**
- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **calme**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **colere**
- « jpp de cette vidéo de chat » → attendu **joie**, prédit **suspense**
- « elle m'a répondu !!! je suis aux anges » → attendu **joie**, prédit **suspense**
- « haha la tête du prof quand il a vu le gâteau » → attendu **joie**, prédit **calme**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **tristesse**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **calme**
- « j'ai la haine, ils m'ont pas sélectionné alors que je suis meilleur » → attendu **colere**, prédit **tristesse**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **amour**
- « je supporte plus son attitude » → attendu **colere**, prédit **amour**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **calme**
- « elle m'a traitée devant tout le monde » → attendu **colere**, prédit **amour**
- « ce soir je révise jusqu'à minuit, rien ne m'arrête » → attendu **energie**, prédit **calme**
- « cette fois je vise le 20 » → attendu **energie**, prédit **joie**
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
- « merci d'avoir été là hier, ça m'a fait du bien » → attendu **amour**, prédit **joie**
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
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **amour**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « jarrive dans 5 min » → attendu **calme**, prédit **energie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **energie**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **tristesse**
- « haha you're the funniest person I know » → attendu **joie**, prédit **colere**
- « finally beat that boss level » → attendu **joie**, prédit **energie**
- « best birthday ever, thank you guys 🎉 » → attendu **joie**, prédit **amour**
- « we came first in the science fair » → attendu **joie**, prédit **energie**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **tristesse**
- « he read my messages without asking » → attendu **colere**, prédit **tristesse**
- « I'm so annoyed, the bus left without me again » → attendu **colere**, prédit **tristesse**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « leave me alone » → attendu **colere**, prédit **tristesse**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « studying all night, nothing can stop me » → attendu **energie**, prédit **calme**
- « full send on the project tonight » → attendu **energie**, prédit **calme**
- « I have something big to tell you... » → attendu **suspense**, prédit **amour**
- « why did the principal call my parents? » → attendu **suspense**, prédit **tristesse**
- « someone left a note in my locker » → attendu **suspense**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « the teacher said she has a big announcement » → attendu **suspense**, prédit **calme**
- « my parents keep fighting » → attendu **tristesse**, prédit **colere**
- « what time do you finish tomorrow? » → attendu **calme**, prédit **energie**
- « I'm at my grandma's this weekend » → attendu **calme**, prédit **joie**
- « is it 8 or 9 tomorrow? » → attendu **calme**, prédit **energie**
- « I uploaded the pics to the drive » → attendu **calme**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **suspense**
- « wat time is practise » → attendu **calme**, prédit **energie**
- « im omw be there in 5 » → attendu **calme**, prédit **joie**

## Erreurs : minilm-fewshot (93)

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
- « haha la tête du prof quand il a vu le gâteau » → attendu **joie**, prédit **colere**
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
- « t'es trop adorable quand tu fais ça » → attendu **amour**, prédit **joie**
- « je suis trop fière d'être ta sœur » → attendu **amour**, prédit **joie**
- « câlin virtuel 🤗 » → attendu **amour**, prédit **calme**
- « il est trop mignon avec moi » → attendu **amour**, prédit **joie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je kiffe trop être avec toi » → attendu **amour**, prédit **joie**
- « je suis fou d'elle » → attendu **amour**, prédit **colere**
- « je suis chez mamie ce week-end » → attendu **calme**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « je prends mon goûter et j'arrive » → attendu **calme**, prédit **energie**
- « c'est quelle page pour l'exercice ? » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « ma mère passe me chercher à 17h30 » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **energie**
- « je serai pas là mardi, rdv chez le dentiste » → attendu **calme**, prédit **tristesse**
- « je vais me coucher, à demain » → attendu **calme**, prédit **energie**
- « j'ai mis les photos sur le drive » → attendu **calme**, prédit **joie**
- « sa ma fait trop plaisir ton message » → attendu **joie**, prédit **amour**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **tristesse**
- « arete de m'enerver » → attendu **colere**, prédit **energie**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **tristesse**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **calme**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **amour**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « jarrive dans 5 min » → attendu **calme**, prédit **energie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **energie**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **tristesse**
- « haha you're the funniest person I know » → attendu **joie**, prédit **colere**
- « finally beat that boss level » → attendu **joie**, prédit **energie**
- « we came first in the science fair » → attendu **joie**, prédit **energie**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **tristesse**
- « he read my messages without asking » → attendu **colere**, prédit **calme**
- « I'm so annoyed, the bus left without me again » → attendu **colere**, prédit **tristesse**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « leave me alone » → attendu **colere**, prédit **tristesse**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « studying all night, nothing can stop me » → attendu **energie**, prédit **calme**
- « full send on the project tonight » → attendu **energie**, prédit **calme**
- « there's a weird car parked outside my house » → attendu **suspense**, prédit **tristesse**
- « why did the principal call my parents? » → attendu **suspense**, prédit **colere**
- « someone left a note in my locker » → attendu **suspense**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « my best friend is moving to another state » → attendu **tristesse**, prédit **suspense**
- « my parents keep fighting » → attendu **tristesse**, prédit **colere**
- « goodnight sweetheart » → attendu **amour**, prédit **calme**
- « I'm at my grandma's this weekend » → attendu **calme**, prédit **joie**
- « my mom picks me up at 5:30 » → attendu **calme**, prédit **suspense**
- « is it 8 or 9 tomorrow? » → attendu **calme**, prédit **energie**
- « I uploaded the pics to the drive » → attendu **calme**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « were gonna win tommorow lets go » → attendu **energie**, prédit **joie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **suspense**
- « wat time is practise » → attendu **calme**, prédit **energie**
- « im omw be there in 5 » → attendu **calme**, prédit **suspense**

## Erreurs : e5small-proto (88)

- « trop fière de moi j'ai fini le livre en anglais » → attendu **joie**, prédit **tristesse**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **tristesse**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « elle m'a répondu !!! je suis aux anges » → attendu **joie**, prédit **amour**
- « il a encore pris mon chargeur sans demander » → attendu **colere**, prédit **tristesse**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **tristesse**
- « j'ai la haine, ils m'ont pas sélectionné alors que je suis meilleur » → attendu **colere**, prédit **tristesse**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **amour**
- « ma sœur a lu mon journal, je vais la tuer » → attendu **colere**, prédit **tristesse**
- « j'ai le seum contre l'arbitre, il était nul » → attendu **colere**, prédit **tristesse**
- « je supporte plus son attitude » → attendu **colere**, prédit **calme**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « laisse-moi tranquille » → attendu **colere**, prédit **amour**
- « c'est la troisième fois qu'on me vole ma trousse, j'en peux plus » → attendu **colere**, prédit **joie**
- « ce soir je révise jusqu'à minuit, rien ne m'arrête » → attendu **energie**, prédit **tristesse**
- « j'ai décidé, je m'inscris au marathon des jeunes » → attendu **energie**, prédit **joie**
- « je me remets au piano, une heure par jour » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « full concentration pour les exams 💯 » → attendu **energie**, prédit **calme**
- « chaud pour la rando de 20 km » → attendu **energie**, prédit **joie**
- « regarde ta boîte aux lettres 👀 » → attendu **suspense**, prédit **calme**
- « y a quelqu'un qui m'a laissé un mot dans mon casier » → attendu **suspense**, prédit **tristesse**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **tristesse**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **tristesse**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « mon meilleur ami change de collège » → attendu **tristesse**, prédit **joie**
- « on a perdu la finale, on était si près » → attendu **tristesse**, prédit **joie**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **suspense**
- « t'es trop adorable quand tu fais ça » → attendu **amour**, prédit **joie**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **tristesse**
- « je suis trop fière d'être ta sœur » → attendu **amour**, prédit **tristesse**
- « il est trop mignon avec moi » → attendu **amour**, prédit **colere**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je kiffe trop être avec toi » → attendu **amour**, prédit **colere**
- « je suis fou d'elle » → attendu **amour**, prédit **colere**
- « je suis chez mamie ce week-end » → attendu **calme**, prédit **tristesse**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « je prends mon goûter et j'arrive » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « ma mère passe me chercher à 17h30 » → attendu **calme**, prédit **tristesse**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « je serai pas là mardi, rdv chez le dentiste » → attendu **calme**, prédit **tristesse**
- « je vais me coucher, à demain » → attendu **calme**, prédit **energie**
- « mdr t trop marant » → attendu **joie**, prédit **colere**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « arete de m'enerver » → attendu **colere**, prédit **calme**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **tristesse**
- « cette anné je donne tout » → attendu **energie**, prédit **amour**
- « ya un truc bizzare dans mon jardin » → attendu **suspense**, prédit **joie**
- « tu devinera jamais qui ma ecrit » → attendu **suspense**, prédit **colere**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « il c passer quoi hier soir ? » → attendu **suspense**, prédit **calme**
- « jai rater mon examen je suis degouter » → attendu **tristesse**, prédit **calme**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **amour**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « got an A on my history test, can't believe it » → attendu **joie**, prédit **suspense**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **tristesse**
- « haha you're the funniest person I know » → attendu **joie**, prédit **amour**
- « finally beat that boss level » → attendu **joie**, prédit **energie**
- « best birthday ever, thank you guys 🎉 » → attendu **joie**, prédit **amour**
- « we came first in the science fair » → attendu **joie**, prédit **energie**
- « I'm so annoyed, the bus left without me again » → attendu **colere**, prédit **tristesse**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « leave me alone » → attendu **colere**, prédit **tristesse**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « I can't stand how he talks to me » → attendu **colere**, prédit **tristesse**
- « I can't believe you lied to me » → attendu **colere**, prédit **suspense**
- « studying all night, nothing can stop me » → attendu **energie**, prédit **colere**
- « full send on the project tonight » → attendu **energie**, prédit **calme**
- « who posted that in the group chat?? » → attendu **suspense**, prédit **calme**
- « why did the principal call my parents? » → attendu **suspense**, prédit **tristesse**
- « someone left a note in my locker » → attendu **suspense**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **colere**
- « we lost the final, so close » → attendu **tristesse**, prédit **energie**
- « I'm at my grandma's this weekend » → attendu **calme**, prédit **tristesse**
- « going to bed, see you tomorrow » → attendu **calme**, prédit **energie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « theres something wierd going on at school » → attendu **suspense**, prédit **colere**
- « i miss you alot » → attendu **tristesse**, prédit **amour**
- « im omw be there in 5 » → attendu **calme**, prédit **joie**

## Erreurs : e5small-fewshot (83)

- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **tristesse**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « elle m'a répondu !!! je suis aux anges » → attendu **joie**, prédit **amour**
- « j'ai eu un 19 en physique, je rêve » → attendu **joie**, prédit **energie**
- « il a encore pris mon chargeur sans demander » → attendu **colere**, prédit **tristesse**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **tristesse**
- « j'ai la haine, ils m'ont pas sélectionné alors que je suis meilleur » → attendu **colere**, prédit **tristesse**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **amour**
- « ma sœur a lu mon journal, je vais la tuer » → attendu **colere**, prédit **tristesse**
- « j'ai le seum contre l'arbitre, il était nul » → attendu **colere**, prédit **tristesse**
- « je supporte plus son attitude » → attendu **colere**, prédit **amour**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « laisse-moi tranquille » → attendu **colere**, prédit **amour**
- « c'est la troisième fois qu'on me vole ma trousse, j'en peux plus » → attendu **colere**, prédit **joie**
- « je me remets au piano, une heure par jour » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « regarde ta boîte aux lettres 👀 » → attendu **suspense**, prédit **calme**
- « y a quelqu'un qui m'a laissé un mot dans mon casier » → attendu **suspense**, prédit **tristesse**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **tristesse**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **tristesse**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « mon meilleur ami change de collège » → attendu **tristesse**, prédit **amour**
- « on a perdu la finale, on était si près » → attendu **tristesse**, prédit **joie**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **suspense**
- « t'es trop adorable quand tu fais ça » → attendu **amour**, prédit **joie**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **tristesse**
- « je suis trop fière d'être ta sœur » → attendu **amour**, prédit **tristesse**
- « il est trop mignon avec moi » → attendu **amour**, prédit **colere**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je kiffe trop être avec toi » → attendu **amour**, prédit **joie**
- « je suis fou d'elle » → attendu **amour**, prédit **colere**
- « je suis chez mamie ce week-end » → attendu **calme**, prédit **tristesse**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « je prends mon goûter et j'arrive » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « ma mère passe me chercher à 17h30 » → attendu **calme**, prédit **tristesse**
- « je serai pas là mardi, rdv chez le dentiste » → attendu **calme**, prédit **tristesse**
- « il faut ramener une calculatrice » → attendu **calme**, prédit **energie**
- « je vais me coucher, à demain » → attendu **calme**, prédit **energie**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **energie**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « c abusé il ma pas attendu » → attendu **colere**, prédit **tristesse**
- « arete de m'enerver » → attendu **colere**, prédit **tristesse**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **tristesse**
- « cette anné je donne tout » → attendu **energie**, prédit **amour**
- « ya un truc bizzare dans mon jardin » → attendu **suspense**, prédit **joie**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « il c passer quoi hier soir ? » → attendu **suspense**, prédit **calme**
- « jai rater mon examen je suis degouter » → attendu **tristesse**, prédit **energie**
- « personne veut jouer avec moi a la recré » → attendu **tristesse**, prédit **energie**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **amour**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **energie**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « got an A on my history test, can't believe it » → attendu **joie**, prédit **suspense**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **tristesse**
- « finally beat that boss level » → attendu **joie**, prédit **energie**
- « best birthday ever, thank you guys 🎉 » → attendu **joie**, prédit **amour**
- « we came first in the science fair » → attendu **joie**, prédit **energie**
- « I'm so annoyed, the bus left without me again » → attendu **colere**, prédit **tristesse**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « leave me alone » → attendu **colere**, prédit **tristesse**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « the ref was a joke, we got robbed » → attendu **colere**, prédit **joie**
- « I can't believe you lied to me » → attendu **colere**, prédit **suspense**
- « full send on the project tonight » → attendu **energie**, prédit **calme**
- « who posted that in the group chat?? » → attendu **suspense**, prédit **calme**
- « why did the principal call my parents? » → attendu **suspense**, prédit **tristesse**
- « someone left a note in my locker » → attendu **suspense**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **colere**
- « we lost the final, so close » → attendu **tristesse**, prédit **energie**
- « I'm at my grandma's this weekend » → attendu **calme**, prédit **tristesse**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « theres something wierd going on at school » → attendu **suspense**, prédit **colere**
- « i miss you alot » → attendu **tristesse**, prédit **amour**
- « im omw be there in 5 » → attendu **calme**, prédit **energie**
