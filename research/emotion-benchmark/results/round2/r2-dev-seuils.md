# Résultats benchmark émotions — dev_v1.csv

70 messages dont 6 avec fautes. Un message vaut 1.4 points au global et 16.7 points dans la colonne « Avec fautes ».

| Modèle | Global | Tolérant | FR | EN | Avec fautes | Sans fautes | Latence | Taille | Remarque |
|---|---|---|---|---|---|---|---|---|---|
| gemma-m3-q4-proto | 90.0 % | 91.4 % | 87.8 % | 95.2 % | 83.3 % | 90.6 % | 22.8 ms | 115.9 Mo | choix typé (proto), ONNX model_q4.onnx |
| gemma-m3-q4-fewshot | 90.0 % | 91.4 % | 85.7 % | 100.0 % | 66.7 % | 92.2 % | 24.6 ms | 115.9 Mo | choix typé (fewshot), ONNX model_q4.onnx |
| gemma-m10-q4-fewshot | 90.0 % | 91.4 % | 87.8 % | 95.2 % | 83.3 % | 90.6 % | 24.1 ms | 104.3 Mo | choix typé (fewshot), ONNX model_q4.onnx |
| gemma-m10-q4-proto | 88.6 % | 90.0 % | 87.8 % | 90.5 % | 83.3 % | 89.1 % | 24.7 ms | 104.3 Mo | choix typé (proto), ONNX model_q4.onnx |

## Rappel par émotion

| Modèle | joie | colere | energie | suspense | tristesse | amour | calme |
|---|---|---|---|---|---|---|---|
| gemma-m3-q4-proto | 90.9 % | 70.0 % | 90.0 % | 90.0 % | 90.0 % | 100.0 % | 100.0 % |
| gemma-m3-q4-fewshot | 81.8 % | 70.0 % | 100.0 % | 90.0 % | 90.0 % | 100.0 % | 100.0 % |
| gemma-m10-q4-proto | 81.8 % | 70.0 % | 90.0 % | 90.0 % | 90.0 % | 100.0 % | 100.0 % |
| gemma-m10-q4-fewshot | 81.8 % | 70.0 % | 90.0 % | 100.0 % | 90.0 % | 100.0 % | 100.0 % |

## Confiance (choix typé)

Précision tolérante sur les messages dont la confiance dépasse le seuil, et part des messages concernés.

| Modèle | Confiance moy. | ≥ 0.2 | ≥ 0.3 | ≥ 0.4 | ≥ 0.5 |
|---|---|---|---|---|---|
| gemma-m3-q4-proto | 0.55 | 93.8 % sur 93 % | 93.2 % sur 84 % | 94.1 % sur 73 % | 97.4 % sur 56 % |
| gemma-m3-q4-fewshot | 0.54 | 95.3 % sur 91 % | 95.1 % sur 87 % | 98.1 % sur 74 % | 100.0 % sur 53 % |
| gemma-m10-q4-proto | 0.51 | 93.5 % sur 89 % | 94.6 % sur 80 % | 95.7 % sur 66 % | 97.1 % sur 49 % |
| gemma-m10-q4-fewshot | 0.52 | 93.8 % sur 91 % | 93.2 % sur 84 % | 95.7 % sur 67 % | 97.1 % sur 50 % |

## Erreurs : gemma-m3-q4-proto (6)

- « J'EN AI MARRE DE CE PROF » → attendu **colere**, prédit **calme**
- « jsuis trop vénère là » → attendu **colere**, prédit **amour**
- « tu vas jamais croire ce qu'elle a fait » → attendu **suspense**, prédit **colere**
- « personne m'a invitée à son anniv » → attendu **tristesse**, prédit **calme**
- « tu ma soulé avec tes histoire » → attendu **colere**, prédit **suspense**
- « I'm so hyped for the game » → attendu **energie**, prédit **joie**

## Erreurs : gemma-m3-q4-fewshot (6)

- « jsuis grave content pour toi » → attendu **joie**, prédit **amour**
- « J'EN AI MARRE DE CE PROF » → attendu **colere**, prédit **calme**
- « jsuis trop vénère là » → attendu **colere**, prédit **amour**
- « personne m'a invitée à son anniv » → attendu **tristesse**, prédit **suspense**
- « tu ma soulé avec tes histoire » → attendu **colere**, prédit **suspense**
- « tu sais pas ce qui c'est passer a la récré » → attendu **suspense**, prédit **calme**

## Erreurs : gemma-m10-q4-proto (7)

- « J'EN AI MARRE DE CE PROF » → attendu **colere**, prédit **calme**
- « jsuis trop vénère là » → attendu **colere**, prédit **amour**
- « tu vas jamais croire ce qu'elle a fait » → attendu **suspense**, prédit **colere**
- « personne m'a invitée à son anniv » → attendu **tristesse**, prédit **suspense**
- « tu ma soulé avec tes histoire » → attendu **colere**, prédit **suspense**
- « lmao that's hilarious » → attendu **joie**, prédit **colere**
- « I'm so hyped for the game » → attendu **energie**, prédit **joie**

## Erreurs : gemma-m10-q4-fewshot (6)

- « jsuis grave content pour toi » → attendu **joie**, prédit **amour**
- « J'EN AI MARRE DE CE PROF » → attendu **colere**, prédit **calme**
- « jsuis trop vénère là » → attendu **colere**, prédit **amour**
- « personne m'a invitée à son anniv » → attendu **tristesse**, prédit **suspense**
- « tu ma soulé avec tes histoire » → attendu **colere**, prédit **suspense**
- « I'm so hyped for the game » → attendu **energie**, prédit **joie**
