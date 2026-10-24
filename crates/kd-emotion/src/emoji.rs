//! Emoji-first emotion (ADR-0028).
//!
//! An emoji states its emotion more reliably than the model reads a sentence: 🔥 and 💪 are
//! energy, 😭 is sadness. [`EmojiMap`] maps emoji to emotions from `data/emotions/emoji.json`;
//! [`EmojiMap::emotion`] picks the main emotion of a message from its emoji.

use std::collections::HashMap;

use kd_core::Emotion;

use crate::EmotionError;

/// Variation selector 16 (emoji presentation), ignored when matching.
const VS16: char = '\u{FE0F}';

/// Emoji to emotion map, as shipped in `data/emotions/emoji.json`.
#[derive(Debug, Clone)]
pub struct EmojiMap {
    /// `(emoji without U+FE0F, emotion)`, longest emoji first so sequences win over prefixes.
    entries: Vec<(String, Emotion)>,
}

impl EmojiMap {
    /// The map built into the engine (`data/emotions/emoji.json`).
    pub fn builtin() -> Self {
        Self::from_json(include_str!("../../../data/emotions/emoji.json"))
            .expect("data/emotions/emoji.json is valid")
    }

    /// Parses `{ "<emotion id>": ["<emoji>", ...], ... }`.
    ///
    /// Rejects unknown emotion ids, empty emoji and an emoji listed under two emotions.
    pub fn from_json(json: &str) -> Result<Self, EmotionError> {
        let raw: HashMap<String, Vec<String>> =
            serde_json::from_str(json).map_err(|e| EmotionError::InvalidEmoji(e.to_string()))?;
        let mut seen: HashMap<String, Emotion> = HashMap::new();
        for (id, emoji) in &raw {
            let emotion = Emotion::from_id(id)
                .ok_or_else(|| EmotionError::InvalidEmoji(format!("unknown emotion `{id}`")))?;
            for e in emoji {
                let key = strip_vs16(e);
                if key.is_empty() {
                    return Err(EmotionError::InvalidEmoji(format!("empty emoji in `{id}`")));
                }
                if let Some(other) = seen.insert(key, emotion) {
                    if other != emotion {
                        return Err(EmotionError::InvalidEmoji(format!(
                            "`{e}` is listed under `{}` and `{id}`",
                            other.id()
                        )));
                    }
                }
            }
        }
        let mut entries: Vec<(String, Emotion)> = seen.into_iter().collect();
        entries.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(&b.0)));
        Ok(Self { entries })
    }

    /// The main emotion of `text` from its emoji, or `None` if it has no known emoji.
    ///
    /// The emotion with the most emoji wins; a tie goes to the emotion whose emoji comes first.
    /// Skin tones and U+FE0F do not change the meaning of an emoji.
    pub fn emotion(&self, text: &str) -> Option<Emotion> {
        let text = strip_vs16(text);
        // (emotion, count, first position)
        let mut found: Vec<(Emotion, usize, usize)> = Vec::new();
        let mut i = 0;
        while i < text.len() {
            let rest = &text[i..];
            match self
                .entries
                .iter()
                .find(|(e, _)| rest.starts_with(e.as_str()))
            {
                Some((e, emotion)) => {
                    match found.iter_mut().find(|f| f.0 == *emotion) {
                        Some(f) => f.1 += 1,
                        None => found.push((*emotion, 1, i)),
                    }
                    i += e.len();
                }
                None => i += rest.chars().next().map_or(1, char::len_utf8),
            }
        }
        found
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.2.cmp(&a.2)))
            .map(|(emotion, _, _)| emotion)
    }
}

fn strip_vs16(s: &str) -> String {
    s.chars().filter(|&c| c != VS16).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_map_is_valid_and_covers_every_animated_emotion() {
        let map = EmojiMap::builtin();
        for emotion in Emotion::ALL {
            let covered = map.entries.iter().any(|(_, e)| *e == emotion);
            assert_eq!(covered, emotion != Emotion::Neutral, "{}", emotion.id());
        }
    }

    #[test]
    fn emoji_reveal_the_main_emotion() {
        let map = EmojiMap::builtin();
        assert_eq!(map.emotion("on y va 🔥💪"), Some(Emotion::Energy));
        assert_eq!(map.emotion("je t'aime ❤️"), Some(Emotion::Love));
        assert_eq!(map.emotion("je t'aime ❤"), Some(Emotion::Love));
        assert_eq!(map.emotion("il est parti 😭"), Some(Emotion::Sadness));
        assert_eq!(map.emotion("qui a fait ça 🕵"), Some(Emotion::Suspense));
        assert_eq!(map.emotion("bonjour à tous"), None);
        assert_eq!(map.emotion("ok 👍"), None);
    }

    #[test]
    fn skin_tones_do_not_hide_an_emoji() {
        let map = EmojiMap::builtin();
        assert_eq!(map.emotion("allez 💪🏽"), Some(Emotion::Energy));
    }

    #[test]
    fn most_frequent_emotion_wins_and_ties_go_to_the_first() {
        let map = EmojiMap::builtin();
        assert_eq!(map.emotion("😭 mais 🔥🔥"), Some(Emotion::Energy));
        assert_eq!(map.emotion("😂 puis 😡"), Some(Emotion::Joy));
        assert_eq!(map.emotion("😡 puis 😂"), Some(Emotion::Anger));
    }

    #[test]
    fn invalid_maps_are_rejected() {
        assert!(EmojiMap::from_json(r#"{"bonheur": ["😀"]}"#).is_err());
        assert!(EmojiMap::from_json(r#"{"joie": [""]}"#).is_err());
        assert!(EmojiMap::from_json(r#"{"joie": ["😀"], "amour": ["😀"]}"#).is_err());
        assert!(EmojiMap::from_json(r#"{"joie": "😀"}"#).is_err());
    }
}
