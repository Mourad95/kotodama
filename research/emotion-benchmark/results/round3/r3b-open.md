# Résultats benchmark émotions — test_v2.csv

301 messages dont 70 avec fautes. Un message vaut 0.3 points au global et 1.4 points dans la colonne « Avec fautes ».

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |
|---|---|---|---|---|---|---|---|---|---|
| bgem3-fewshot | 79.7 % | 80.7 % | 79.6 % | 80.0 % | 81.4 % | 79.2 % | 133.3 ms | 2297.6 Mo | choix typé (fewshot), PyTorch |
| qwen3-fewshot | 79.4 % | 80.4 % | 77.6 % | 82.9 % | 77.1 % | 80.1 % | 511.2 ms | 1207.5 Mo | choix typé (fewshot), PyTorch |
| e5base-fewshot | 76.4 % | 77.4 % | 73.5 % | 81.9 % | 81.4 % | 74.9 % | 46.7 ms | 2246.8 Mo | choix typé (fewshot), PyTorch |
| granite-fewshot | 72.4 % | 73.4 % | 71.4 % | 74.3 % | 82.9 % | 69.3 % | 80.1 ms | 1683.1 Mo | choix typé (fewshot), PyTorch |
| granite-small-fewshot | 69.8 % | 70.4 % | 69.9 % | 69.5 % | 82.9 % | 65.8 % | 17.7 ms | 998.9 Mo | choix typé (fewshot), PyTorch |

## Rappel par émotion

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|---|---|---|---|---|---|---|
| e5base-fewshot | 69.8 % | 60.5 % | 88.4 % | 79.1 % | 83.7 % | 86.0 % | 67.4 % |
| granite-small-fewshot | 72.1 % | 55.8 % | 86.0 % | 74.4 % | 72.1 % | 74.4 % | 53.5 % |
| granite-fewshot | 69.8 % | 48.8 % | 90.7 % | 79.1 % | 74.4 % | 81.4 % | 62.8 % |
| bgem3-fewshot | 79.1 % | 60.5 % | 93.0 % | 79.1 % | 74.4 % | 90.7 % | 81.4 % |
| qwen3-fewshot | 62.8 % | 79.1 % | 79.1 % | 79.1 % | 81.4 % | 83.7 % | 90.7 % |

## Confiance (choix typé)

Précision tolérante sur les messages dont la confiance dépasse le seuil, et part des messages concernés.

| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |
|---|---|---|---|---|---|
| e5base-fewshot | 0.13 | 98.1 % sur 17 % | 100.0 % sur 6 % | 100.0 % sur 2 % | None % sur 0 % |
| granite-small-fewshot | 0.23 | 87.2 % sur 47 % | 89.5 % sur 25 % | 94.4 % sur 12 % | 95.2 % sur 7 % |
| granite-fewshot | 0.25 | 84.1 % sur 56 % | 95.2 % sur 28 % | 95.7 % sur 16 % | 94.7 % sur 6 % |
| bgem3-fewshot | 0.29 | 87.1 % sur 64 % | 93.6 % sur 42 % | 95.9 % sur 25 % | 100.0 % sur 12 % |
| qwen3-fewshot | 0.4 | 86.0 % sur 78 % | 88.9 % sur 63 % | 91.8 % sur 49 % | 96.2 % sur 35 % |

## Erreurs : e5base-fewshot (68)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « elle m'a répondu !!! je suis aux anges » → attendu **joie**, prédit **amour**
- « j'ai eu un 19 en physique, je rêve » → attendu **joie**, prédit **energie**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **tristesse**
- « j'ai la haine, ils m'ont pas sélectionné alors que je suis meilleur » → attendu **colere**, prédit **tristesse**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **amour**
- « ma sœur a lu mon journal, je vais la tuer » → attendu **colere**, prédit **tristesse**
- « j'ai le seum contre l'arbitre, il était nul » → attendu **colere**, prédit **tristesse**
- « pourquoi tu fais exprès de m'ignorer ? » → attendu **colere**, prédit **suspense**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « elle m'a traitée devant tout le monde » → attendu **colere**, prédit **amour**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « je me remets au piano, une heure par jour » → attendu **energie**, prédit **calme**
- « regarde ta boîte aux lettres 👀 » → attendu **suspense**, prédit **calme**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **calme**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **tristesse**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « mon meilleur ami change de collège » → attendu **tristesse**, prédit **joie**
- « on a perdu la finale, on était si près » → attendu **tristesse**, prédit **energie**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **tristesse**
- « il est trop mignon avec moi » → attendu **amour**, prédit **joie**
- « je kiffe trop être avec toi » → attendu **amour**, prédit **colere**
- « je suis fou d'elle » → attendu **amour**, prédit **colere**
- « je suis chez mamie ce week-end » → attendu **calme**, prédit **energie**
- « t'as le numéro de Tom ? » → attendu **calme**, prédit **suspense**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « ma mère passe me chercher à 17h30 » → attendu **calme**, prédit **tristesse**
- « je serai pas là mardi, rdv chez le dentiste » → attendu **calme**, prédit **tristesse**
- « il faut ramener une calculatrice » → attendu **calme**, prédit **energie**
- « je vais me coucher, à demain » → attendu **calme**, prédit **energie**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **energie**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « arete de m'enerver » → attendu **colere**, prédit **energie**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **tristesse**
- « on lache rien jusqu'a la fin » → attendu **energie**, prédit **colere**
- « ya un truc bizzare dans mon jardin » → attendu **suspense**, prédit **joie**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **calme**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **amour**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **colere**
- « je suis trop contant de t'avoir comme ami » → attendu **amour**, prédit **tristesse**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « je rentre a pied ce soir » → attendu **calme**, prédit **energie**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « best birthday ever, thank you guys 🎉 » → attendu **joie**, prédit **amour**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **tristesse**
- « I'm so annoyed, the bus left without me again » → attendu **colere**, prédit **tristesse**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « leave me alone » → attendu **colere**, prédit **tristesse**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « I can't believe you lied to me » → attendu **colere**, prédit **suspense**
- « full send on the project tonight » → attendu **energie**, prédit **calme**
- « why did the principal call my parents? » → attendu **suspense**, prédit **tristesse**
- « someone left a note in my locker » → attendu **suspense**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **colere**
- « my best friend is moving to another state » → attendu **tristesse**, prédit **joie**
- « we lost the final, so close » → attendu **tristesse**, prédit **joie**
- « I'm at my grandma's this weekend » → attendu **calme**, prédit **joie**
- « my mom picks me up at 5:30 » → attendu **calme**, prédit **tristesse**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « im omw be there in 5 » → attendu **calme**, prédit **energie**

## Erreurs : granite-small-fewshot (89)

- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « je viens de voir mon bulletin, que des félicitations » → attendu **joie**, prédit **suspense**
- « elle m'a répondu !!! je suis aux anges » → attendu **joie**, prédit **amour**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **tristesse**
- « arrête de raconter des trucs sur moi » → attendu **colere**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **amour**
- « ma sœur a lu mon journal, je vais la tuer » → attendu **colere**, prédit **suspense**
- « ils se foutent de nous avec ce devoir de 10 pages » → attendu **colere**, prédit **energie**
- « pourquoi tu fais exprès de m'ignorer ? » → attendu **colere**, prédit **suspense**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **calme**
- « elle m'a traitée devant tout le monde » → attendu **colere**, prédit **tristesse**
- « laisse-moi tranquille » → attendu **colere**, prédit **tristesse**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **joie**
- « ce soir je révise jusqu'à minuit, rien ne m'arrête » → attendu **energie**, prédit **colere**
- « ce match on le gagne, point » → attendu **energie**, prédit **joie**
- « regarde ta boîte aux lettres 👀 » → attendu **suspense**, prédit **calme**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **tristesse**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **calme**
- « c'est qui ce numéro qui m'appelle depuis tout à l'heure ? » → attendu **suspense**, prédit **calme**
- « tu crois qu'ils vont dire les résultats aujourd'hui ? » → attendu **suspense**, prédit **calme**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « mon meilleur ami change de collège » → attendu **tristesse**, prédit **joie**
- « on a perdu la finale, on était si près » → attendu **tristesse**, prédit **joie**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « mon vélo a été volé, c'était un cadeau de ma mamie » → attendu **tristesse**, prédit **joie**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **calme**
- « merci d'avoir été là hier, ça m'a fait du bien » → attendu **amour**, prédit **joie**
- « bonne nuit mon cœur » → attendu **amour**, prédit **tristesse**
- « t'es trop adorable quand tu fais ça » → attendu **amour**, prédit **joie**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **tristesse**
- « câlin virtuel 🤗 » → attendu **amour**, prédit **joie**
- « il est trop mignon avec moi » → attendu **amour**, prédit **colere**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je suis chez mamie ce week-end » → attendu **calme**, prédit **tristesse**
- « t'as le numéro de Tom ? » → attendu **calme**, prédit **amour**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « je prends mon goûter et j'arrive » → attendu **calme**, prédit **energie**
- « c'est quelle page pour l'exercice ? » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « t'as fini le chapitre 3 ? » → attendu **calme**, prédit **joie**
- « je serai pas là mardi, rdv chez le dentiste » → attendu **calme**, prédit **tristesse**
- « il faut ramener une calculatrice » → attendu **calme**, prédit **energie**
- « je vais me coucher, à demain » → attendu **calme**, prédit **energie**
- « j'ai mis les photos sur le drive » → attendu **calme**, prédit **tristesse**
- « arete de m'enerver » → attendu **colere**, prédit **amour**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **tristesse**
- « cette anné je donne tout » → attendu **energie**, prédit **colere**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « personne veut jouer avec moi a la recré » → attendu **tristesse**, prédit **suspense**
- « tu me manque trop » → attendu **tristesse**, prédit **amour**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **amour**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « je rentre a pied ce soir » → attendu **calme**, prédit **energie**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « got an A on my history test, can't believe it » → attendu **joie**, prédit **suspense**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **energie**
- « finally beat that boss level » → attendu **joie**, prédit **energie**
- « best birthday ever, thank you guys 🎉 » → attendu **joie**, prédit **amour**
- « we came first in the science fair » → attendu **joie**, prédit **calme**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « stop talking behind my back » → attendu **colere**, prédit **tristesse**
- « I'm so annoyed, the bus left without me again » → attendu **colere**, prédit **tristesse**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « leave me alone » → attendu **colere**, prédit **tristesse**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « studying all night, nothing can stop me » → attendu **energie**, prédit **colere**
- « why did the principal call my parents? » → attendu **suspense**, prédit **tristesse**
- « someone left a note in my locker » → attendu **suspense**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **colere**
- « my best friend is moving to another state » → attendu **tristesse**, prédit **joie**
- « we lost the final, so close » → attendu **tristesse**, prédit **joie**
- « you're so cute when you do that » → attendu **amour**, prédit **joie**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **joie**
- « can't stop thinking about you » → attendu **amour**, prédit **colere**
- « what time do you finish tomorrow? » → attendu **calme**, prédit **energie**
- « I'm at my grandma's this weekend » → attendu **calme**, prédit **joie**
- « do you have Tom's number? » → attendu **calme**, prédit **suspense**
- « my mom picks me up at 5:30 » → attendu **calme**, prédit **tristesse**
- « going to bed, see you tomorrow » → attendu **calme**, prédit **energie**
- « I uploaded the pics to the drive » → attendu **calme**, prédit **tristesse**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « i miss you alot » → attendu **tristesse**, prédit **amour**

## Erreurs : granite-fewshot (80)

- « trop fière de moi j'ai fini le livre en anglais » → attendu **joie**, prédit **tristesse**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « je viens de voir mon bulletin, que des félicitations » → attendu **joie**, prédit **suspense**
- « elle m'a répondu !!! je suis aux anges » → attendu **joie**, prédit **suspense**
- « haha la tête du prof quand il a vu le gâteau » → attendu **joie**, prédit **suspense**
- « mes cousins arrivent ce soir, trop content » → attendu **joie**, prédit **tristesse**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **suspense**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **tristesse**
- « j'ai la haine, ils m'ont pas sélectionné alors que je suis meilleur » → attendu **colere**, prédit **tristesse**
- « arrête de raconter des trucs sur moi » → attendu **colere**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **amour**
- « ma sœur a lu mon journal, je vais la tuer » → attendu **colere**, prédit **suspense**
- « ils se foutent de nous avec ce devoir de 10 pages » → attendu **colere**, prédit **energie**
- « je supporte plus son attitude » → attendu **colere**, prédit **amour**
- « pourquoi tu fais exprès de m'ignorer ? » → attendu **colere**, prédit **suspense**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « laisse-moi tranquille » → attendu **colere**, prédit **tristesse**
- « ce soir je révise jusqu'à minuit, rien ne m'arrête » → attendu **energie**, prédit **tristesse**
- « regarde ta boîte aux lettres 👀 » → attendu **suspense**, prédit **calme**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **tristesse**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **calme**
- « tu crois qu'ils vont dire les résultats aujourd'hui ? » → attendu **suspense**, prédit **calme**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « mon meilleur ami change de collège » → attendu **tristesse**, prédit **joie**
- « on a perdu la finale, on était si près » → attendu **tristesse**, prédit **joie**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « mon vélo a été volé, c'était un cadeau de ma mamie » → attendu **tristesse**, prédit **joie**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **calme**
- « merci d'avoir été là hier, ça m'a fait du bien » → attendu **amour**, prédit **joie**
- « t'es trop adorable quand tu fais ça » → attendu **amour**, prédit **joie**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **tristesse**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je suis chez mamie ce week-end » → attendu **calme**, prédit **tristesse**
- « t'as le numéro de Tom ? » → attendu **calme**, prédit **suspense**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « je serai pas là mardi, rdv chez le dentiste » → attendu **calme**, prédit **tristesse**
- « il faut ramener une calculatrice » → attendu **calme**, prédit **energie**
- « je vais me coucher, à demain » → attendu **calme**, prédit **energie**
- « j'ai mis les photos sur le drive » → attendu **calme**, prédit **tristesse**
- « c abusé il ma pas attendu » → attendu **colere**, prédit **suspense**
- « arete de m'enerver » → attendu **colere**, prédit **amour**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **tristesse**
- « cette anné je donne tout » → attendu **energie**, prédit **colere**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « personne veut jouer avec moi a la recré » → attendu **tristesse**, prédit **suspense**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **joie**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « jarrive dans 5 min » → attendu **calme**, prédit **energie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « je rentre a pied ce soir » → attendu **calme**, prédit **energie**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « got an A on my history test, can't believe it » → attendu **joie**, prédit **suspense**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **energie**
- « haha you're the funniest person I know » → attendu **joie**, prédit **suspense**
- « finally beat that boss level » → attendu **joie**, prédit **energie**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **suspense**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « stop talking behind my back » → attendu **colere**, prédit **tristesse**
- « I'm so annoyed, the bus left without me again » → attendu **colere**, prédit **tristesse**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « leave me alone » → attendu **colere**, prédit **tristesse**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « studying all night, nothing can stop me » → attendu **energie**, prédit **colere**
- « why did the principal call my parents? » → attendu **suspense**, prédit **tristesse**
- « someone left a note in my locker » → attendu **suspense**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **colere**
- « we lost the final, so close » → attendu **tristesse**, prédit **joie**
- « you're so cute when you do that » → attendu **amour**, prédit **joie**
- « I'm so lucky to have you » → attendu **amour**, prédit **joie**
- « can't stop thinking about you » → attendu **amour**, prédit **colere**
- « I'm at my grandma's this weekend » → attendu **calme**, prédit **joie**
- « don't forget the permission slip » → attendu **calme**, prédit **tristesse**
- « do you have Tom's number? » → attendu **calme**, prédit **suspense**
- « I uploaded the pics to the drive » → attendu **calme**, prédit **tristesse**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « i miss you alot » → attendu **tristesse**, prédit **amour**

## Erreurs : bgem3-fewshot (58)

- « il neige !!! pas cours demain » → attendu **joie**, prédit **tristesse**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **tristesse**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « PERSONNE NE M'ÉCOUTE DANS CETTE MAISON » → attendu **colere**, prédit **tristesse**
- « j'ai la haine, ils m'ont pas sélectionné alors que je suis meilleur » → attendu **colere**, prédit **tristesse**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **suspense**
- « j'ai le seum contre l'arbitre, il était nul » → attendu **colere**, prédit **tristesse**
- « je supporte plus son attitude » → attendu **colere**, prédit **energie**
- « pourquoi tu fais exprès de m'ignorer ? » → attendu **colere**, prédit **tristesse**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « chaud pour la rando de 20 km » → attendu **energie**, prédit **joie**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **tristesse**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **tristesse**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **tristesse**
- « tu crois qu'ils vont dire les résultats aujourd'hui ? » → attendu **suspense**, prédit **calme**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « mon meilleur ami change de collège » → attendu **tristesse**, prédit **joie**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je me sens vide en ce moment » → attendu **tristesse**, prédit **calme**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « il est trop mignon avec moi » → attendu **amour**, prédit **joie**
- « je suis fou d'elle » → attendu **amour**, prédit **colere**
- « je suis chez mamie ce week-end » → attendu **calme**, prédit **tristesse**
- « vu, merci » → attendu **calme**, prédit **amour**
- « ma mère passe me chercher à 17h30 » → attendu **calme**, prédit **tristesse**
- « je serai pas là mardi, rdv chez le dentiste » → attendu **calme**, prédit **tristesse**
- « c abusé il ma pas attendu » → attendu **colere**, prédit **suspense**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **tristesse**
- « ya un truc bizzare dans mon jardin » → attendu **suspense**, prédit **joie**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « jai rater mon examen je suis degouter » → attendu **tristesse**, prédit **joie**
- « personne veut jouer avec moi a la recré » → attendu **tristesse**, prédit **energie**
- « tu me manque trop » → attendu **tristesse**, prédit **amour**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **joie**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « je suis trop contant de t'avoir comme ami » → attendu **amour**, prédit **joie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « lol that meme killed me » → attendu **joie**, prédit **colere**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **tristesse**
- « best birthday ever, thank you guys 🎉 » → attendu **joie**, prédit **amour**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **tristesse**
- « he read my messages without asking » → attendu **colere**, prédit **suspense**
- « I'm so annoyed, the bus left without me again » → attendu **colere**, prédit **tristesse**
- « you said you'd come and you didn't » → attendu **colere**, prédit **suspense**
- « leave me alone » → attendu **colere**, prédit **tristesse**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « the ref was a joke, we got robbed » → attendu **colere**, prédit **joie**
- « I can't believe you lied to me » → attendu **colere**, prédit **suspense**
- « morning run at 6, who's in? » → attendu **energie**, prédit **calme**
- « why did the principal call my parents? » → attendu **suspense**, prédit **tristesse**
- « call me, I can't say it over text » → attendu **suspense**, prédit **tristesse**
- « I'm at my grandma's this weekend » → attendu **calme**, prédit **tristesse**
- « I uploaded the pics to the drive » → attendu **calme**, prédit **joie**
- « we have gym first period » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **tristesse**
- « i miss you alot » → attendu **tristesse**, prédit **amour**

## Erreurs : qwen3-fewshot (59)

- « mon père a dit oui pour le concert !!! » → attendu **joie**, prédit **calme**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **tristesse**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **tristesse**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **calme**
- « elle m'a répondu !!! je suis aux anges » → attendu **joie**, prédit **amour**
- « j'ai eu un 19 en physique, je rêve » → attendu **joie**, prédit **energie**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **amour**
- « je supporte plus son attitude » → attendu **colere**, prédit **amour**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « laisse-moi tranquille » → attendu **colere**, prédit **calme**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **suspense**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « on est prêts, ils vont rien comprendre » → attendu **energie**, prédit **suspense**
- « on lâche rien les gars » → attendu **energie**, prédit **colere**
- « ce match on le gagne, point » → attendu **energie**, prédit **joie**
- « regarde ta boîte aux lettres 👀 » → attendu **suspense**, prédit **calme**
- « y a quelqu'un qui m'a laissé un mot dans mon casier » → attendu **suspense**, prédit **tristesse**
- « il s'est passé un truc de fou à la cantine » → attendu **suspense**, prédit **colere**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **calme**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **colere**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « j'ai fait une crise d'angoisse avant l'oral » → attendu **tristesse**, prédit **suspense**
- « je suis trop fière d'être ta sœur » → attendu **amour**, prédit **joie**
- « câlin virtuel 🤗 » → attendu **amour**, prédit **calme**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « je suis fou d'elle » → attendu **amour**, prédit **joie**
- « on a sport en 1re heure » → attendu **calme**, prédit **energie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « mdr t trop marant » → attendu **joie**, prédit **calme**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **suspense**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **calme**
- « jen ai marre de ces devoir » → attendu **colere**, prédit **tristesse**
- « on lache rien jusqu'a la fin » → attendu **energie**, prédit **tristesse**
- « on y vas les gars on va gagné » → attendu **energie**, prédit **joie**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **calme**
- « jai rater mon examen je suis degouter » → attendu **tristesse**, prédit **energie**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **amour**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « I got into the art program!! » → attendu **joie**, prédit **calme**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **energie**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **tristesse**
- « best birthday ever, thank you guys 🎉 » → attendu **joie**, prédit **amour**
- « you said you'd come and you didn't » → attendu **colere**, prédit **calme**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **suspense**
- « morning run at 6, who's in? » → attendu **energie**, prédit **calme**
- « full send on the project tonight » → attendu **energie**, prédit **calme**
- « who posted that in the group chat?? » → attendu **suspense**, prédit **calme**
- « someone left a note in my locker » → attendu **suspense**, prédit **calme**
- « the teacher said she has a big announcement » → attendu **suspense**, prédit **calme**
- « my parents keep fighting » → attendu **tristesse**, prédit **colere**
- « goodnight sweetheart » → attendu **amour**, prédit **calme**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **joie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **tristesse**
- « im so pumped for the game tonite » → attendu **energie**, prédit **joie**
- « guess who texted me lol » → attendu **suspense**, prédit **calme**
- « my cat dyed this morning » → attendu **tristesse**, prédit **suspense**
