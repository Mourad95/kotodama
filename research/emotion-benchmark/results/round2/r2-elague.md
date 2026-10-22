# Résultats benchmark émotions — test_v2.csv

301 messages dont 70 avec fautes. Un message vaut 0.3 points au global et 1.4 points dans la colonne « Avec fautes ».

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |
|---|---|---|---|---|---|---|---|---|---|
| gemma-m3-fewshot | 85.0 % | 86.0 % | 82.1 % | 90.5 % | 80.0 % | 86.6 % | 33.5 ms | 760.0 Mo | choix typé (fewshot), PyTorch |
| gemma-m10-fewshot | 83.7 % | 85.0 % | 80.6 % | 89.5 % | 77.1 % | 85.7 % | 31.7 ms | 684.1 Mo | choix typé (fewshot), PyTorch |
| gemma-m3-proto | 83.4 % | 84.7 % | 81.1 % | 87.6 % | 78.6 % | 84.8 % | 32.5 ms | 760.0 Mo | choix typé (proto), PyTorch |
| gemma-m10-proto | 80.7 % | 82.4 % | 78.6 % | 84.8 % | 74.3 % | 82.7 % | 31.8 ms | 684.1 Mo | choix typé (proto), PyTorch |
| minilm-m3-proto | 68.4 % | 69.4 % | 66.3 % | 72.4 % | 72.9 % | 67.1 % | 7.8 ms | 154.9 Mo | choix typé (proto), PyTorch |
| minilm-m3-fewshot | 67.8 % | 68.4 % | 66.3 % | 70.5 % | 75.7 % | 65.4 % | 8.2 ms | 154.9 Mo | choix typé (fewshot), PyTorch |

## Rappel par émotion

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|---|---|---|---|---|---|---|
| gemma-m3-proto | 67.4 % | 81.4 % | 90.7 % | 90.7 % | 83.7 % | 86.0 % | 83.7 % |
| gemma-m3-fewshot | 81.4 % | 79.1 % | 86.0 % | 90.7 % | 86.0 % | 90.7 % | 81.4 % |
| gemma-m10-proto | 67.4 % | 69.8 % | 88.4 % | 90.7 % | 83.7 % | 86.0 % | 79.1 % |
| gemma-m10-fewshot | 76.7 % | 76.7 % | 88.4 % | 88.4 % | 86.0 % | 88.4 % | 81.4 % |
| minilm-m3-proto | 48.8 % | 67.4 % | 88.4 % | 60.5 % | 74.4 % | 83.7 % | 55.8 % |
| minilm-m3-fewshot | 53.5 % | 65.1 % | 88.4 % | 62.8 % | 74.4 % | 76.7 % | 53.5 % |

## Confiance (choix typé)

Précision tolérante sur les messages dont la confiance dépasse le seuil, et part des messages concernés.

| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |
|---|---|---|---|---|---|
| gemma-m3-proto | 0.47 | 88.3 % sur 88 % | 91.4 % sur 73 % | 94.8 % sur 57 % | 96.0 % sur 42 % |
| gemma-m3-fewshot | 0.44 | 90.2 % sur 85 % | 92.9 % sur 70 % | 95.6 % sur 53 % | 99.1 % sur 37 % |
| gemma-m10-proto | 0.43 | 88.2 % sur 84 % | 92.4 % sur 66 % | 95.4 % sur 50 % | 99.0 % sur 35 % |
| gemma-m10-fewshot | 0.42 | 90.5 % sur 81 % | 92.6 % sur 67 % | 95.3 % sur 50 % | 99.1 % sur 35 % |
| minilm-m3-proto | 0.48 | 71.6 % sur 91 % | 77.0 % sur 74 % | 82.7 % sur 61 % | 83.2 % sur 46 % |
| minilm-m3-fewshot | 0.37 | 76.2 % sur 81 % | 83.0 % sur 57 % | 85.7 % sur 40 % | 88.4 % sur 23 % |

## Erreurs : gemma-m3-proto (46)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « jpp de cette vidéo de chat » → attendu **joie**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « je supporte plus son attitude » → attendu **colere**, prédit **amour**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « laisse-moi tranquille » → attendu **colere**, prédit **calme**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
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

## Erreurs : gemma-m3-fewshot (42)

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
- « chaud pour la rando de 20 km » → attendu **energie**, prédit **calme**
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

## Erreurs : gemma-m10-proto (53)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « jpp de cette vidéo de chat » → attendu **joie**, prédit **suspense**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **calme**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « je supporte plus son attitude » → attendu **colere**, prédit **amour**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « elle m'a traitée devant tout le monde » → attendu **colere**, prédit **suspense**
- « laisse-moi tranquille » → attendu **colere**, prédit **calme**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
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
- « cette anné je donne tout » → attendu **energie**, prédit **calme**
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
- « did u finish the homework » → attendu **calme**, prédit **suspense**

## Erreurs : gemma-m10-fewshot (45)

- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « elle m'a répondu !!! je suis aux anges » → attendu **joie**, prédit **amour**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **tristesse**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « elle m'a traitée devant tout le monde » → attendu **colere**, prédit **suspense**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « je me remets au piano, une heure par jour » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **tristesse**
- « tu crois qu'ils vont dire les résultats aujourd'hui ? » → attendu **suspense**, prédit **calme**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « j'ai fait une crise d'angoisse avant l'oral » → attendu **tristesse**, prédit **suspense**
- « t'es mon rayon de soleil » → attendu **amour**, prédit **calme**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **suspense**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **tristesse**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « jen ai marre de ces devoir » → attendu **colere**, prédit **tristesse**
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
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **tristesse**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **joie**

## Erreurs : minilm-m3-proto (92)

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
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **tristesse**
- « haha you're the funniest person I know » → attendu **joie**, prédit **colere**
- « finally beat that boss level » → attendu **joie**, prédit **energie**
- « best birthday ever, thank you guys 🎉 » → attendu **joie**, prédit **amour**
- « we came first in the science fair » → attendu **joie**, prédit **energie**
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

## Erreurs : minilm-m3-fewshot (95)

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
- « je suis chez mamie ce week-end » → attendu **calme**, prédit **energie**
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
- « ya un truc bizzare dans mon jardin » → attendu **suspense**, prédit **amour**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **calme**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **amour**
- « gros bisous mon coeur » → attendu **amour**, prédit **colere**
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
