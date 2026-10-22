# Résultats benchmark émotions — test_v2.csv

301 messages dont 70 avec fautes. Un message vaut 0.3 points au global et 1.4 points dans la colonne « Avec fautes ».

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |
|---|---|---|---|---|---|---|---|---|---|
| gemma2-fewshot | 82.1 % | 83.1 % | 82.1 % | 81.9 % | 82.9 % | 81.8 % | 123.9 ms | 1084.0 Mo | choix typé (fewshot), PyTorch |
| gemma2-proto | 81.7 % | 83.1 % | 80.6 % | 83.8 % | 81.4 % | 81.8 % | 129.4 ms | 1084.0 Mo | choix typé (proto), PyTorch |
| gemma2-fewshot-512 | 81.4 % | 82.4 % | 81.1 % | 81.9 % | 80.0 % | 81.8 % | 120.1 ms | 1084.0 Mo | choix typé (fewshot), PyTorch, 512 dims |
| gemma2-fewshot-256 | 81.4 % | 82.7 % | 80.6 % | 82.9 % | 81.4 % | 81.4 % | 124.1 ms | 1084.0 Mo | choix typé (fewshot), PyTorch, 256 dims |

## Rappel par émotion

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|---|---|---|---|---|---|---|
| gemma2-proto | 69.8 % | 81.4 % | 88.4 % | 88.4 % | 74.4 % | 90.7 % | 79.1 % |
| gemma2-fewshot | 76.7 % | 72.1 % | 88.4 % | 83.7 % | 81.4 % | 90.7 % | 81.4 % |
| gemma2-fewshot-512 | 74.4 % | 76.7 % | 88.4 % | 83.7 % | 76.7 % | 90.7 % | 79.1 % |
| gemma2-fewshot-256 | 74.4 % | 72.1 % | 90.7 % | 83.7 % | 79.1 % | 90.7 % | 79.1 % |

## Confiance (choix typé)

Précision tolérante sur les messages dont la confiance dépasse le seuil, et part des messages concernés.

| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |
|---|---|---|---|---|---|
| gemma2-proto | 0.12 | 100.0 % sur 14 % | 100.0 % sur 6 % | 100.0 % sur 2 % | 100.0 % sur 0 % |
| gemma2-fewshot | 0.3 | 93.0 % sur 66 % | 97.7 % sur 44 % | 100.0 % sur 26 % | 100.0 % sur 14 % |
| gemma2-fewshot-512 | 0.3 | 92.0 % sur 67 % | 97.1 % sur 45 % | 100.0 % sur 26 % | 100.0 % sur 14 % |
| gemma2-fewshot-256 | 0.3 | 91.0 % sur 67 % | 96.9 % sur 44 % | 100.0 % sur 25 % | 100.0 % sur 16 % |

## Erreurs : gemma2-proto (51)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **suspense**
- « je supporte plus son attitude » → attendu **colere**, prédit **energie**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **suspense**
- « on lâche rien les gars » → attendu **energie**, prédit **colere**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « chaud pour la rando de 20 km » → attendu **energie**, prédit **calme**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **colere**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **calme**
- « mon grand-père est parti cette nuit » → attendu **tristesse**, prédit **calme**
- « mon meilleur ami change de collège » → attendu **tristesse**, prédit **suspense**
- « mon vélo a été volé, c'était un cadeau de ma mamie » → attendu **tristesse**, prédit **suspense**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **suspense**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **suspense**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **energie**
- « ok je regarde et je te dis » → attendu **calme**, prédit **suspense**
- « j'ai mis les photos sur le drive » → attendu **calme**, prédit **joie**
- « mdr t trop marant » → attendu **joie**, prédit **colere**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **suspense**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « arete de m'enerver » → attendu **colere**, prédit **energie**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « ont a perdu la finale » → attendu **tristesse**, prédit **suspense**
- « merci detre la pour moi » → attendu **amour**, prédit **calme**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « jarrive dans 5 min » → attendu **calme**, prédit **energie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « my mom said I can get a puppy!!! » → attendu **joie**, prédit **suspense**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **tristesse**
- « he read my messages without asking » → attendu **colere**, prédit **calme**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « morning run at 6, who's in? » → attendu **energie**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « my parents keep fighting » → attendu **tristesse**, prédit **colere**
- « I failed the test again, I'm so dumb » → attendu **tristesse**, prédit **colere**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **suspense**

## Erreurs : gemma2-fewshot (51)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **amour**
- « ma sœur a lu mon journal, je vais la tuer » → attendu **colere**, prédit **joie**
- « je supporte plus son attitude » → attendu **colere**, prédit **energie**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **suspense**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « chaud pour la rando de 20 km » → attendu **energie**, prédit **calme**
- « regarde ta boîte aux lettres 👀 » → attendu **suspense**, prédit **calme**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **colere**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **calme**
- « mon vélo a été volé, c'était un cadeau de ma mamie » → attendu **tristesse**, prédit **suspense**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **suspense**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **joie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **energie**
- « j'ai mis les photos sur le drive » → attendu **calme**, prédit **joie**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « arete de m'enerver » → attendu **colere**, prédit **energie**
- « cette anné je donne tout » → attendu **energie**, prédit **joie**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « ont a perdu la finale » → attendu **tristesse**, prédit **suspense**
- « merci detre la pour moi » → attendu **amour**, prédit **calme**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « jarrive dans 5 min » → attendu **calme**, prédit **energie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **calme**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « my mom said I can get a puppy!!! » → attendu **joie**, prédit **suspense**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **tristesse**
- « he read my messages without asking » → attendu **colere**, prédit **calme**
- « you said you'd come and you didn't » → attendu **colere**, prédit **calme**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « the ref was a joke, we got robbed » → attendu **colere**, prédit **tristesse**
- « morning run at 6, who's in? » → attendu **energie**, prédit **calme**
- « someone left a note in my locker » → attendu **suspense**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « my parents keep fighting » → attendu **tristesse**, prédit **colere**
- « I failed the test again, I'm so dumb » → attendu **tristesse**, prédit **colere**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **suspense**

## Erreurs : gemma2-fewshot-512 (53)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **amour**
- « je supporte plus son attitude » → attendu **colere**, prédit **energie**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **suspense**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « chaud pour la rando de 20 km » → attendu **energie**, prédit **calme**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **colere**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **calme**
- « mon vélo a été volé, c'était un cadeau de ma mamie » → attendu **tristesse**, prédit **suspense**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
- « j'ai fait une crise d'angoisse avant l'oral » → attendu **tristesse**, prédit **suspense**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **suspense**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **joie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **energie**
- « ok je regarde et je te dis » → attendu **calme**, prédit **suspense**
- « j'ai mis les photos sur le drive » → attendu **calme**, prédit **joie**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **suspense**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « arete de m'enerver » → attendu **colere**, prédit **energie**
- « cette anné je donne tout » → attendu **energie**, prédit **joie**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « personne veut jouer avec moi a la recré » → attendu **tristesse**, prédit **energie**
- « ont a perdu la finale » → attendu **tristesse**, prédit **suspense**
- « merci detre la pour moi » → attendu **amour**, prédit **calme**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « jarrive dans 5 min » → attendu **calme**, prédit **energie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **calme**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « my mom said I can get a puppy!!! » → attendu **joie**, prédit **suspense**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **tristesse**
- « he read my messages without asking » → attendu **colere**, prédit **calme**
- « you said you'd come and you didn't » → attendu **colere**, prédit **calme**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « morning run at 6, who's in? » → attendu **energie**, prédit **calme**
- « who posted that in the group chat?? » → attendu **suspense**, prédit **colere**
- « someone left a note in my locker » → attendu **suspense**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « my parents keep fighting » → attendu **tristesse**, prédit **colere**
- « I failed the test again, I'm so dumb » → attendu **tristesse**, prédit **colere**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **suspense**

## Erreurs : gemma2-fewshot-256 (52)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **calme**
- « ma sœur a lu mon journal, je vais la tuer » → attendu **colere**, prédit **joie**
- « je supporte plus son attitude » → attendu **colere**, prédit **energie**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « c'est la troisième fois qu'on me vole ma trousse, j'en peux plus » → attendu **colere**, prédit **tristesse**
- « motivée à mort pour le stage » → attendu **energie**, prédit **joie**
- « chaud pour la rando de 20 km » → attendu **energie**, prédit **calme**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **colere**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **calme**
- « mon vélo a été volé, c'était un cadeau de ma mamie » → attendu **tristesse**, prédit **suspense**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **calme**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **suspense**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **joie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **suspense**
- « on se met à côté en techno ? » → attendu **calme**, prédit **energie**
- « je vais me coucher, à demain » → attendu **calme**, prédit **energie**
- « ok je regarde et je te dis » → attendu **calme**, prédit **suspense**
- « j'ai mis les photos sur le drive » → attendu **calme**, prédit **joie**
- « on a gagnez le match de hand !! » → attendu **joie**, prédit **energie**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « arete de m'enerver » → attendu **colere**, prédit **energie**
- « tu c pas ce que jai vu » → attendu **suspense**, prédit **colere**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « personne veut jouer avec moi a la recré » → attendu **tristesse**, prédit **energie**
- « ont a perdu la finale » → attendu **tristesse**, prédit **suspense**
- « merci detre la pour moi » → attendu **amour**, prédit **calme**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **calme**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « my mom said I can get a puppy!!! » → attendu **joie**, prédit **suspense**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **tristesse**
- « he read my messages without asking » → attendu **colere**, prédit **calme**
- « you said you'd come and you didn't » → attendu **colere**, prédit **calme**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « the ref was a joke, we got robbed » → attendu **colere**, prédit **tristesse**
- « morning run at 6, who's in? » → attendu **energie**, prédit **calme**
- « someone left a note in my locker » → attendu **suspense**, prédit **calme**
- « call me, I can't say it over text » → attendu **suspense**, prédit **calme**
- « I failed the test again, I'm so dumb » → attendu **tristesse**, prédit **colere**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « my cat dyed this morning » → attendu **tristesse**, prédit **suspense**
