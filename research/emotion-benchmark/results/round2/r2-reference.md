# Résultats benchmark émotions — test_v2.csv

301 messages dont 70 avec fautes. Un message vaut 0.3 points au global et 1.4 points dans la colonne « Avec fautes ».

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |
|---|---|---|---|---|---|---|---|---|---|
| tabularisai | 56.8 % | 58.5 % | 55.1 % | 60.0 % | 50.0 % | 58.9 % | 8.3 ms | 1129 Mo | référence, labels figés |

## Rappel par émotion

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|---|---|---|---|---|---|---|
| tabularisai | 55.8 % | 60.5 % | 0.0 % | 46.5 % | 48.8 % | 93.0 % | 93.0 % |

## Erreurs : tabularisai (125)

- « ahaha t'es vraiment un clown » → attendu **joie**, prédit **colere**
- « on a fini premiers au cross du collège 🏃 » → attendu **joie**, prédit **calme**
- « il neige !!! pas cours demain » → attendu **joie**, prédit **calme**
- « le prof a annulé l'interro mdr » → attendu **joie**, prédit **calme**
- « je viens de voir mon bulletin, que des félicitations » → attendu **joie**, prédit **suspense**
- « jpp de cette vidéo de chat » → attendu **joie**, prédit **colere**
- « j'ai eu un 19 en physique, je rêve » → attendu **joie**, prédit **suspense**
- « t'avais promis et t'es pas venu, merci bien » → attendu **colere**, prédit **amour**
- « ma sœur a lu mon journal, je vais la tuer » → attendu **colere**, prédit **suspense**
- « j'ai le seum contre l'arbitre, il était nul » → attendu **colere**, prédit **suspense**
- « STOP avec tes messages » → attendu **colere**, prédit **amour**
- « je supporte plus son attitude » → attendu **colere**, prédit **amour**
- « grave relou le bus qui part sans nous » → attendu **colere**, prédit **tristesse**
- « laisse-moi tranquille » → attendu **colere**, prédit **calme**
- « il a balancé mon secret à toute la classe » → attendu **colere**, prédit **suspense**
- « ce soir je révise jusqu'à minuit, rien ne m'arrête » → attendu **energie**, prédit **calme**
- « on va leur montrer qui on est au match » → attendu **energie**, prédit **calme**
- « j'ai décidé, je m'inscris au marathon des jeunes » → attendu **energie**, prédit **calme**
- « demain 7h footing, qui vient ? » → attendu **energie**, prédit **calme**
- « cette fois je vise le 20 » → attendu **energie**, prédit **calme**
- « on est prêts, ils vont rien comprendre » → attendu **energie**, prédit **suspense**
- « à fond pour la compèt de danse » → attendu **energie**, prédit **colere**
- « allez l'équipe on y croit jusqu'au bout 🔥 » → attendu **energie**, prédit **colere**
- « je me remets au piano, une heure par jour » → attendu **energie**, prédit **calme**
- « je vais tout casser au contrôle de demain » → attendu **energie**, prédit **suspense**
- « on lâche rien les gars » → attendu **energie**, prédit **calme**
- « motivée à mort pour le stage » → attendu **energie**, prédit **colere**
- « ce match on le gagne, point » → attendu **energie**, prédit **calme**
- « full concentration pour les exams 💯 » → attendu **energie**, prédit **calme**
- « je m'entraîne tous les soirs pour les sélections » → attendu **energie**, prédit **calme**
- « go les bleus, on va les éclater » → attendu **energie**, prédit **amour**
- « cette semaine je change tout, nouvelles habitudes » → attendu **energie**, prédit **calme**
- « chaud pour la rando de 20 km » → attendu **energie**, prédit **calme**
- « on attaque le projet ce soir, je suis à bloc » → attendu **energie**, prédit **calme**
- « j'ai la niaque pour le tournoi » → attendu **energie**, prédit **suspense**
- « regarde ta boîte aux lettres 👀 » → attendu **suspense**, prédit **calme**
- « y a quelqu'un qui m'a laissé un mot dans mon casier » → attendu **suspense**, prédit **colere**
- « devine ce que j'ai trouvé dans le grenier » → attendu **suspense**, prédit **calme**
- « pourquoi la directrice a appelé mes parents ? » → attendu **suspense**, prédit **tristesse**
- « tu sais qui est le nouveau ? » → attendu **suspense**, prédit **calme**
- « y a une voiture bizarre garée devant chez moi depuis ce matin » → attendu **suspense**, prédit **calme**
- « je peux pas en parler par message, appelle » → attendu **suspense**, prédit **calme**
- « il s'est passé un truc de fou à la cantine » → attendu **suspense**, prédit **calme**
- « on a reçu une lettre sans nom d'expéditeur » → attendu **suspense**, prédit **calme**
- « ne dis rien à personne mais... » → attendu **suspense**, prédit **colere**
- « la porte du gymnase était ouverte en pleine nuit » → attendu **suspense**, prédit **calme**
- « le prof a dit qu'il avait une annonce importante » → attendu **suspense**, prédit **calme**
- « mon grand-père est parti cette nuit » → attendu **tristesse**, prédit **suspense**
- « j'ai pas été retenue pour le spectacle » → attendu **tristesse**, prédit **calme**
- « tout le monde était invité sauf moi » → attendu **tristesse**, prédit **calme**
- « mon meilleur ami change de collège » → attendu **tristesse**, prédit **suspense**
- « ma chienne est malade, le véto est pas optimiste » → attendu **tristesse**, prédit **suspense**
- « mon vélo a été volé, c'était un cadeau de ma mamie » → attendu **tristesse**, prédit **amour**
- « je vais devoir redoubler » → attendu **tristesse**, prédit **suspense**
- « j'ai fait une crise d'angoisse avant l'oral » → attendu **tristesse**, prédit **suspense**
- « j'ai cassé l'écran du téléphone que je venais d'avoir » → attendu **tristesse**, prédit **calme**
- « câlin virtuel 🤗 » → attendu **amour**, prédit **joie**
- « joyeux anniversaire à la meilleure du monde 🎂 » → attendu **amour**, prédit **colere**
- « vu, merci » → attendu **calme**, prédit **amour**
- « jsuis trop contante jai eu mon brevet » → attendu **joie**, prédit **calme**
- « sa ma fait trop plaisir ton message » → attendu **joie**, prédit **amour**
- « trop bien les vacance qui arrive » → attendu **joie**, prédit **tristesse**
- « jai eu 16 en math je suis choquer » → attendu **joie**, prédit **suspense**
- « on va a la mer cette été yesss » → attendu **joie**, prédit **calme**
- « c abusé il ma pas attendu » → attendu **colere**, prédit **calme**
- « arete de m'enerver » → attendu **colere**, prédit **calme**
- « il ce moque de moi devant tout le monde » → attendu **colere**, prédit **tristesse**
- « demain on va tout dechirer au match » → attendu **energie**, prédit **calme**
- « je suis motivé a fond pour les controle » → attendu **energie**, prédit **calme**
- « on lache rien jusqu'a la fin » → attendu **energie**, prédit **tristesse**
- « je vais m'entrainé tout les jours » → attendu **energie**, prédit **calme**
- « chaud pour le tournois de samedi » → attendu **energie**, prédit **calme**
- « cette anné je donne tout » → attendu **energie**, prédit **amour**
- « on y vas les gars on va gagné » → attendu **energie**, prédit **calme**
- « tu c pas ce que jai vu » → attendu **suspense**, prédit **colere**
- « ya un truc bizzare dans mon jardin » → attendu **suspense**, prédit **colere**
- « jai une nouvelle de ouf a tannoncer » → attendu **suspense**, prédit **calme**
- « tu devinera jamais qui ma ecrit » → attendu **suspense**, prédit **tristesse**
- « pourquoi le prof a garder mon cahier » → attendu **suspense**, prédit **colere**
- « il c passer quoi hier soir ? » → attendu **suspense**, prédit **calme**
- « jai rater mon examen je suis degouter » → attendu **tristesse**, prédit **suspense**
- « personne veut jouer avec moi a la recré » → attendu **tristesse**, prédit **colere**
- « tu me manque trop » → attendu **tristesse**, prédit **amour**
- « ont a perdu la finale » → attendu **tristesse**, prédit **calme**
- « we're going to the beach this weekend 🌊 » → attendu **joie**, prédit **calme**
- « school's cancelled tomorrow because of the snow » → attendu **joie**, prédit **calme**
- « haha you're the funniest person I know » → attendu **joie**, prédit **amour**
- « best birthday ever, thank you guys 🎉 » → attendu **joie**, prédit **amour**
- « WHY DOES NOBODY EVER LISTEN TO ME » → attendu **colere**, prédit **tristesse**
- « he read my messages without asking » → attendu **colere**, prédit **calme**
- « I'm so annoyed, the bus left without me again » → attendu **colere**, prédit **tristesse**
- « you said you'd come and you didn't » → attendu **colere**, prédit **calme**
- « leave me alone » → attendu **colere**, prédit **suspense**
- « ugh my brother broke my headphones » → attendu **colere**, prédit **tristesse**
- « tomorrow we're winning, no doubt » → attendu **energie**, prédit **calme**
- « studying all night, nothing can stop me » → attendu **energie**, prédit **amour**
- « let's gooo, game day 🔥 » → attendu **energie**, prédit **calme**
- « I'm signing up for the half marathon » → attendu **energie**, prédit **calme**
- « we've got this team, all in » → attendu **energie**, prédit **amour**
- « morning run at 6, who's in? » → attendu **energie**, prédit **calme**
- « this year I'm going for straight A's » → attendu **energie**, prédit **calme**
- « can't wait to get on stage tonight!! » → attendu **energie**, prédit **amour**
- « time to lock in for finals 💯 » → attendu **energie**, prédit **calme**
- « we're gonna destroy them on Saturday » → attendu **energie**, prédit **suspense**
- « training every day until tryouts » → attendu **energie**, prédit **calme**
- « full send on the project tonight » → attendu **energie**, prédit **calme**
- « I have something big to tell you... » → attendu **suspense**, prédit **calme**
- « why did the principal call my parents? » → attendu **suspense**, prédit **tristesse**
- « the teacher said she has a big announcement » → attendu **suspense**, prédit **calme**
- « I didn't make the team » → attendu **tristesse**, prédit **suspense**
- « everyone was invited except me » → attendu **tristesse**, prédit **calme**
- « my best friend is moving to another state » → attendu **tristesse**, prédit **suspense**
- « my parents keep fighting » → attendu **tristesse**, prédit **amour**
- « happy birthday to the best sister ever 🎂 » → attendu **amour**, prédit **colere**
- « going to bed, see you tomorrow » → attendu **calme**, prédit **amour**
- « that was so funy lmaoo » → attendu **joie**, prédit **colere**
- « were going to disneyland next weak!! » → attendu **joie**, prédit **colere**
- « were gonna win tommorow lets go » → attendu **energie**, prédit **colere**
- « im so pumped for the game tonite » → attendu **energie**, prédit **amour**
- « no excuses, practise every day » → attendu **energie**, prédit **calme**
- « guess who texted me lol » → attendu **suspense**, prédit **calme**
- « theres something wierd going on at school » → attendu **suspense**, prédit **colere**
- « my cat dyed this morning » → attendu **tristesse**, prédit **suspense**
- « i miss you alot » → attendu **tristesse**, prédit **amour**
- « im omw be there in 5 » → attendu **calme**, prédit **suspense**
