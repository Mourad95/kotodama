# Résultats benchmark émotions — dev_v1.csv

70 messages dont 6 avec fautes. Un message vaut 1.4 points au global et 16.7 points dans la colonne « Avec fautes ».

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |
|---|---|---|---|---|---|---|---|---|---|
| gemma2-q4-fewshot | 88.6 % | 91.4 % | 85.7 % | 95.2 % | 83.3 % | 89.1 % | 46.9 ms | 119.7 Mo | choix typé (fewshot), ONNX model_q4.onnx |
| gemma2-q4-proto | 84.3 % | 87.1 % | 81.6 % | 90.5 % | 83.3 % | 84.4 % | 48.1 ms | 119.7 Mo | choix typé (proto), ONNX model_q4.onnx |

## Rappel par émotion

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|---|---|---|---|---|---|---|
| gemma2-q4-proto | 72.7 % | 80.0 % | 90.0 % | 90.0 % | 90.0 % | 100.0 % | 66.7 % |
| gemma2-q4-fewshot | 81.8 % | 90.0 % | 100.0 % | 90.0 % | 90.0 % | 100.0 % | 66.7 % |

## Confiance (choix typé)

Précision tolérante sur les messages dont la confiance dépasse le seuil, et part des messages concernés.

| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |
|---|---|---|---|---|---|
| gemma2-q4-proto | 0.18 | 95.5 % sur 31 % | 100.0 % sur 11 % | 100.0 % sur 6 % | 100.0 % sur 4 % |
| gemma2-q4-fewshot | 0.55 | 95.2 % sur 89 % | 94.7 % sur 81 % | 98.1 % sur 76 % | 100.0 % sur 60 % |

## Erreurs : gemma2-q4-proto (9)

- « mdrrr t'es trop drôle » → attendu **joie**, prédit **colere**
- « jsuis grave content pour toi » → attendu **joie**, prédit **amour**
- « tu m'as encore laissé en vu, sérieux » → attendu **colere**, prédit **suspense**
- « tu vas jamais croire ce qu'elle a fait » → attendu **suspense**, prédit **colere**
- « ok à toute » → attendu **calme**, prédit **energie**
- « ça marche, pas de souci » → attendu **calme**, prédit **energie**
- « j'arrive dans 5 min » → attendu **calme**, prédit **energie**
- « tu ma soulé avec tes histoire » → attendu **colere**, prédit **suspense**
- « I'm so hyped for the game » → attendu **energie**, prédit **joie**

## Erreurs : gemma2-q4-fewshot (6)

- « jsuis grave content pour toi » → attendu **joie**, prédit **amour**
- « tu vas jamais croire ce qu'elle a fait » → attendu **suspense**, prédit **colere**
- « je rentre vers 18h » → attendu **calme**, prédit **energie**
- « ça marche, pas de souci » → attendu **calme**, prédit **joie**
- « j'arrive dans 5 min » → attendu **calme**, prédit **energie**
- « tu ma soulé avec tes histoire » → attendu **colere**, prédit **suspense**
