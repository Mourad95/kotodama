# Résultats benchmark émotions — dev_v1.csv

70 messages dont 6 avec fautes. Un message vaut 1.4 points au global et 16.7 points dans la colonne « Avec fautes ».

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |
|---|---|---|---|---|---|---|---|---|---|
| gemma2-q4-fewshot | 94.3 % | 97.1 % | 93.9 % | 95.2 % | 83.3 % | 95.3 % | 59.9 ms | 119.7 Mo | choix typé (fewshot), ONNX model_q4.onnx |
| gemma2-q4-proto | 88.6 % | 91.4 % | 87.8 % | 90.5 % | 83.3 % | 89.1 % | 47.1 ms | 119.7 Mo | choix typé (proto), ONNX model_q4.onnx |

## Rappel par émotion

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|---|---|---|---|---|---|---|
| gemma2-q4-proto | 72.7 % | 70.0 % | 100.0 % | 100.0 % | 90.0 % | 100.0 % | 88.9 % |
| gemma2-q4-fewshot | 81.8 % | 90.0 % | 100.0 % | 100.0 % | 90.0 % | 100.0 % | 100.0 % |

## Confiance (choix typé)

Précision tolérante sur les messages dont la confiance dépasse le seuil, et part des messages concernés.

| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |
|---|---|---|---|---|---|
| gemma2-q4-proto | 0.13 | 100.0 % sur 13 % | 100.0 % sur 6 % | 100.0 % sur 4 % | 100.0 % sur 1 % |
| gemma2-q4-fewshot | 0.71 | 98.5 % sur 96 % | 98.5 % sur 93 % | 98.4 % sur 90 % | 100.0 % sur 81 % |

## Erreurs : gemma2-q4-proto (6)

- « mdrrr t'es trop drôle » → attendu **joie**, prédit **colere**
- « jsuis grave content pour toi » → attendu **joie**, prédit **amour**
- « tu m'as encore laissé en vu, sérieux » → attendu **colere**, prédit **suspense**
- « ça marche, pas de souci » → attendu **calme**, prédit **energie**
- « tu ma soulé avec tes histoire » → attendu **colere**, prédit **suspense**
- « I'M SO DONE WITH HIM » → attendu **colere**, prédit **tristesse**

## Erreurs : gemma2-q4-fewshot (2)

- « jsuis grave content pour toi » → attendu **joie**, prédit **amour**
- « tu ma soulé avec tes histoire » → attendu **colere**, prédit **suspense**
