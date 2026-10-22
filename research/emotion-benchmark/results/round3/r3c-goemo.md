# Résultats benchmark émotions — test_v2.csv

301 messages dont 70 avec fautes. Un message vaut 0.3 points au global et 1.4 points dans la colonne « Avec fautes ».

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |
|---|---|---|---|---|---|---|---|---|---|
| goemo-fewshot | 51.5 % | 52.8 % | 49.5 % | 55.2 % | 51.4 % | 51.5 % | 38.8 ms | 712.5 Mo | choix typé (fewshot), PyTorch |

## Rappel par émotion

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|---|---|---|---|---|---|---|
| goemo-fewshot | 48.8 % | 51.2 % | 39.5 % | 46.5 % | 53.5 % | 65.1 % | 55.8 % |

## Confiance (choix typé)

Précision tolérante sur les messages dont la confiance dépasse le seuil, et part des messages concernés.

| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |
|---|---|---|---|---|---|
| goemo-fewshot | 0.42 | 58.8 % sur 85 % | 62.1 % sur 66 % | 66.4 % sur 50 % | 73.5 % sur 34 % |

## Erreurs : goemo-fewshot (142)

- « mon père a dit oui pour le concert !!! » → attendu **joie**, prédit **calme**
- « trop fière de moi j'ai fini le livre en anglais » → attendu **joie**, prédit **amour**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **energie**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **energie**
- « je viens de voir mon bulletin, que des félicitations » → attendu **joie**, prédit **amour**
- « on part à Disney ce week-end 🤩 » → attendu **joie**, prédit **energie**
- « jpp de cette vidéo de chat » → attendu **joie**, prédit **calme**
- « j'ai eu un 19 en physique, je rêve » → attendu **joie**, prédit **energie**
- « il a encore pris mon chargeur sans demander » → attendu **colere**, prédit **energie**
- « c'est quoi ton problème avec moi ? » → attendu **colere**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **amour**
- « on m'a encore accusé à tort » → attendu **colere**, prédit **tristesse**
- « je supporte plus son attitude » → attendu **colere**, prédit **calme**
- « pourquoi tu fais exprès de m'ignorer ? » → attendu **colere**, prédit **suspense**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « laisse-moi tranquille » → attendu **colere**, prédit **energie**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **amour**
- « on va leur montrer qui on est au match » → attendu **energie**, prédit **suspense**
- « j'ai décidé, je m'inscris au marathon des jeunes » → attendu **energie**, prédit **calme**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « cette fois je vise le 20 » → attendu **energie**, prédit **calme**
- « on est prêts, ils vont rien comprendre » → attendu **energie**, prédit **calme**
- « trop hâte d'être sur scène samedi !! » → attendu **energie**, prédit **suspense**
- « à fond pour la compèt de danse » → attendu **energie**, prédit **calme**
- « je vais tout casser au contrôle de demain » → attendu **energie**, prédit **colere**
- « on lâche rien les gars » → attendu **energie**, prédit **colere**
- « motivée à mort pour le stage » → attendu **energie**, prédit **tristesse**
- « ce match on le gagne, point » → attendu **energie**, prédit **joie**
- « full concentration pour les exams 💯 » → attendu **energie**, prédit **calme**
- « je m'entraîne tous les soirs pour les sélections » → attendu **energie**, prédit **joie**
- « cette semaine je change tout, nouvelles habitudes » → attendu **energie**, prédit **joie**
- « j'ai la niaque pour le tournoi » → attendu **energie**, prédit **calme**
- « j'ai un truc énorme à te dire... » → attendu **suspense**, prédit **calme**
- « qui a posté ça sur le groupe ?? » → attendu **suspense**, prédit **calme**
- « regarde ta boîte aux lettres 👀 » → attendu **suspense**, prédit **energie**
- « y a quelqu'un qui m'a laissé un mot dans mon casier » → attendu **suspense**, prédit **colere**
- « y a une voiture bizarre garée devant chez moi depuis ce matin » → attendu **suspense**, prédit **energie**
- « il s'est passé un truc de fou à la cantine » → attendu **suspense**, prédit **joie**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **tristesse**
- « ne dis rien à personne mais... » → attendu **suspense**, prédit **calme**
- « tu crois qu'ils vont dire les résultats aujourd'hui ? » → attendu **suspense**, prédit **calme**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **energie**
- « surprise demain, je te dis rien 🤫 » → attendu **suspense**, prédit **energie**
- « le prof a dit qu'il avait une annonce importante » → attendu **suspense**, prédit **calme**
- « mon grand-père est parti cette nuit » → attendu **tristesse**, prédit **calme**
- « tout le monde était invité sauf moi » → attendu **tristesse**, prédit **joie**
- « j'ai l'impression que personne me comprend » → attendu **tristesse**, prédit **joie**
- « j'ai encore eu une mauvaise note, je suis nul » → attendu **tristesse**, prédit **colere**
- « mes parents se disputent tout le temps » → attendu **tristesse**, prédit **energie**
- « je me sens vide en ce moment » → attendu **tristesse**, prédit **joie**
- « mon vélo a été volé, c'était un cadeau de ma mamie » → attendu **tristesse**, prédit **joie**
- « on s'est séparés hier soir 💔 » → attendu **tristesse**, prédit **suspense**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **energie**
- « personne ne m'a souhaité mon anniversaire » → attendu **tristesse**, prédit **colere**
- « il m'a bloquée partout » → attendu **tristesse**, prédit **energie**
- « c'est la dernière fois qu'on se voit avant longtemps » → attendu **tristesse**, prédit **suspense**
- « mes copines c'est ma deuxième famille » → attendu **amour**, prédit **joie**
- « câlin virtuel 🤗 » → attendu **amour**, prédit **calme**
- « t'es mon rayon de soleil » → attendu **amour**, prédit **calme**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **joie**
- « on est inséparables toi et moi » → attendu **amour**, prédit **colere**
- « je kiffe trop être avec toi » → attendu **amour**, prédit **colere**
- « je suis fou d'elle » → attendu **amour**, prédit **energie**
- « tu finis à quelle heure demain ? » → attendu **calme**, prédit **suspense**
- « on a sport en 1re heure » → attendu **calme**, prédit **suspense**
- « je prends mon goûter et j'arrive » → attendu **calme**, prédit **joie**
- « vu, merci » → attendu **calme**, prédit **amour**
- « ma mère passe me chercher à 17h30 » → attendu **calme**, prédit **suspense**
- « je suis dans le bus, j'arrive dans 10 min » → attendu **calme**, prédit **energie**
- « je vais me coucher, à demain » → attendu **calme**, prédit **energie**
- « on mange à la cantine ou dehors ? » → attendu **calme**, prédit **suspense**
- « trop bien les vacance qui arrive » → attendu **joie**, prédit **energie**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **amour**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **energie**
- « jen ai marre de ces devoir » → attendu **colere**, prédit **tristesse**
- « ma soeur a pris mes affaire sans demandé » → attendu **colere**, prédit **tristesse**
- « demain on va tout dechirer au match » → attendu **energie**, prédit **tristesse**
- « on lache rien jusqu'a la fin » → attendu **energie**, prédit **joie**
- « cette anné je donne tout » → attendu **energie**, prédit **amour**
- « on y vas les gars on va gagné » → attendu **energie**, prédit **joie**
- « tu c pas ce que jai vu » → attendu **suspense**, prédit **colere**
- « ya un truc bizzare dans mon jardin » → attendu **suspense**, prédit **energie**
- « qui ta dit sa ?? » → attendu **suspense**, prédit **calme**
- « jai une nouvelle de ouf a tannoncer » → attendu **suspense**, prédit **joie**
- « jai rater mon examen je suis degouter » → attendu **tristesse**, prédit **colere**
- « personne veut jouer avec moi a la recré » → attendu **tristesse**, prédit **colere**
- « ma meilleur amie part en angleterre » → attendu **tristesse**, prédit **amour**
- « je pense a toi tout le temp » → attendu **amour**, prédit **calme**
- « gros bisous mon coeur » → attendu **amour**, prédit **colere**
- « t'es trop chou quand tu rigole » → attendu **amour**, prédit **joie**
- « je suis trop contant de t'avoir comme ami » → attendu **amour**, prédit **tristesse**
- « tu fini a quel heure ? » → attendu **calme**, prédit **suspense**
- « jarrive dans 5 min » → attendu **calme**, prédit **joie**
- « c'est quel exercice pour demain » → attendu **calme**, prédit **energie**
- « je rentre a pied ce soir » → attendu **calme**, prédit **suspense**
- « on a quoi en premiere heure » → attendu **calme**, prédit **suspense**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **suspense**
- « got an A on my history test, can't believe it » → attendu **joie**, prédit **tristesse**
- « omg the concert was amazing » → attendu **joie**, prédit **amour**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **tristesse**
- « my mom said I can get a puppy!!! » → attendu **joie**, prédit **energie**
- « finally beat that boss level » → attendu **joie**, prédit **energie**
- « best birthday ever, thank you guys 🎉 » → attendu **joie**, prédit **amour**
- « we came first in the science fair » → attendu **joie**, prédit **suspense**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **suspense**
- « he read my messages without asking » → attendu **colere**, prédit **calme**
- « she posted that pic of me on purpose » → attendu **colere**, prédit **calme**
- « leave me alone » → attendu **colere**, prédit **tristesse**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « the ref was a joke, we got robbed » → attendu **colere**, prédit **joie**
- « I can't stand how he talks to me » → attendu **colere**, prédit **suspense**
- « studying all night, nothing can stop me » → attendu **energie**, prédit **colere**
- « we've got this team, all in » → attendu **energie**, prédit **calme**
- « morning run at 6, who's in? » → attendu **energie**, prédit **suspense**
- « can't wait to get on stage tonight!! » → attendu **energie**, prédit **suspense**
- « we're gonna destroy them on Saturday » → attendu **energie**, prédit **colere**
- « who posted that in the group chat?? » → attendu **suspense**, prédit **calme**
- « why did the principal call my parents? » → attendu **suspense**, prédit **calme**
- « someone left a note in my locker » → attendu **suspense**, prédit **calme**
- « surprise tomorrow, not telling you anything 🤫 » → attendu **suspense**, prédit **energie**
- « the teacher said she has a big announcement » → attendu **suspense**, prédit **energie**
- « everyone was invited except me » → attendu **tristesse**, prédit **joie**
- « my best friend is moving to another state » → attendu **tristesse**, prédit **colere**
- « my parents keep fighting » → attendu **tristesse**, prédit **amour**
- « I failed the test again, I'm so dumb » → attendu **tristesse**, prédit **colere**
- « you're so cute when you do that » → attendu **amour**, prédit **joie**
- « I'll always have your back, promise » → attendu **amour**, prédit **colere**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **joie**
- « can't stop thinking about you » → attendu **amour**, prédit **colere**
- « I'm at my grandma's this weekend » → attendu **calme**, prédit **amour**
- « don't forget the permission slip » → attendu **calme**, prédit **suspense**
- « going to bed, see you tomorrow » → attendu **calme**, prédit **energie**
- « ok I'll check and let you know » → attendu **calme**, prédit **energie**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **energie**
- « thats so unfare » → attendu **colere**, prédit **tristesse**
- « were gonna win tommorow lets go » → attendu **energie**, prédit **suspense**
- « no excuses, practise every day » → attendu **energie**, prédit **amour**
- « guess who texted me lol » → attendu **suspense**, prédit **joie**
- « theres something wierd going on at school » → attendu **suspense**, prédit **colere**
- « my cat dyed this morning » → attendu **tristesse**, prédit **colere**
- « wat time is practise » → attendu **calme**, prédit **suspense**
- « im omw be there in 5 » → attendu **calme**, prédit **joie**
