# Emotion annotation guide

One rule: label the **dominant emotion the sender wants to convey**, as a friend reading the message
would feel it — not the emotion of the topic.

| Emotion | Use when | Pitfalls |
|---|---|---|
| `joie` | good news, laughter, pride, enjoying a moment | "can't wait for tomorrow" aimed at action → `energie` |
| `colere` | annoyance, unfairness, blame, curt orders ("leave me alone", "stop") | mild disappointment without blame → `tristesse` |
| `energie` | motivation, challenge, preparation, "we're gonna win" | joy about a win already achieved → `joie` |
| `suspense` | secret, rumour, mystery, intriguing question, announced surprise | a practical question → `calme` |
| `tristesse` | loss, grief, failure, rejection, loneliness, fear for someone | tender "I miss you" addressed to the person → `amour` possible as `alt` |
| `amour` | affection, tenderness, warm thanks, compliment to someone close | a purely practical "thanks" → `calme` |
| `calme` | logistics, schedules, homework, neutral information | — |

## `alt` column

When two emotions are genuinely defensible, the main one goes to `label` and the second to `alt`.
The benchmark reports **strict** accuracy (`label` only) and **lenient** accuracy (`label` or `alt`).
Keep `alt` under 10 % of messages.

## Errors

`typo = 1` when the message contains at least one real spelling or grammar error — not deliberate
abbreviations such as "mdr" or "jsp".
