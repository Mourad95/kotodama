# Guide d'annotation des émotions

Une seule règle de base : **l'émotion dominante que l'expéditeur veut transmettre**, telle
qu'un ami la ressentirait en lisant le message. Pas l'émotion du sujet évoqué.

| Émotion | Quand l'utiliser | Pièges |
|---|---|---|
| `joie` | bonne nouvelle, rire, fierté, plaisir d'un moment | « trop bien demain » tourné vers l'action → `energie` |
| `colere` | agacement, injustice, reproche, ordre sec (« laisse-moi », « stop ») | une déception douce sans reproche → `tristesse` |
| `energie` | motivation, défi, préparation, « on va gagner » | la joie d'avoir déjà gagné → `joie` |
| `suspense` | secret, rumeur, mystère, question qui intrigue, surprise annoncée | une question pratique → `calme` |
| `tristesse` | perte, deuil, échec, rejet, solitude, peur pour quelqu'un | le manque tendre adressé à la personne → `amour` possible en `alt` |
| `amour` | affection, tendresse, remerciement chaleureux, compliment à un proche | un « merci » purement pratique → `calme` |
| `calme` | logistique, horaires, devoirs, info neutre | — |

## Colonne `alt`

Quand deux émotions sont vraiment défendables, la principale va dans `label` et la seconde
dans `alt`. Le benchmark calcule une précision **stricte** (seulement `label`) et **tolérante**
(`label` ou `alt`). Ne pas abuser de `alt` : moins de 10 % des messages.

## Fautes

`typo = 1` quand le message contient au moins une vraie faute d'orthographe ou de grammaire
(pas seulement de l'abréviation volontaire comme « mdr » ou « jsp »).
