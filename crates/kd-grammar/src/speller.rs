//! French spelling over a Hunspell dictionary (ADR-0009, ADR-0025).
//!
//! The dictionary (`grammar/fr/hunspell/`, MPL-2.0) is read by the `spellbook` crate. Words of the
//! allow-list (`grammar/fr/allowlist.txt`) are never reported, and neither are emoji, numbers
//! ("18h"), links, e-mail addresses, @mentions and #hashtags. Elongated words ("troooop",
//! "mdrrr") are accepted when their shortened form is.

use std::collections::HashSet;

use kd_core::Language;

use crate::{GrammarChecker, Issue};

/// Rule identifier of the issues reported by [`FrenchSpeller`].
pub const SPELLING_RULE: &str = "spelling";

/// Longest message checked, in characters; the rest is ignored.
const MAX_CHECKED_CHARS: usize = 2000;

/// Most suggestions returned for one word.
const MAX_SUGGESTIONS: usize = 4;

/// Elided prefixes accepted before an apostrophe ("j'arrive", "qu'elle").
const ELISIONS: [&str; 13] = [
    "c", "d", "j", "l", "m", "n", "s", "t", "qu", "jusqu", "lorsqu", "puisqu", "quoiqu",
];

/// The dictionary files could not be read.
#[derive(Debug, thiserror::Error)]
#[error("invalid Hunspell dictionary: {0}")]
pub struct SpellerError(String);

/// French spell checker: reports the words the dictionary does not know.
pub struct FrenchSpeller {
    dictionary: spellbook::Dictionary,
    allowed: HashSet<String>,
}

impl std::fmt::Debug for FrenchSpeller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FrenchSpeller")
            .field("allowed", &self.allowed.len())
            .finish_non_exhaustive()
    }
}

impl FrenchSpeller {
    /// Builds the checker from the contents of `fr.aff`, `fr.dic` and the allow-list (one word per
    /// line, `#` starts a comment line).
    pub fn new(affixes: &str, words: &str, allowlist: &str) -> Result<Self, SpellerError> {
        let dictionary =
            spellbook::Dictionary::new(affixes, words).map_err(|e| SpellerError(e.to_string()))?;
        let allowed = allowlist
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(|l| normalize_apostrophes(&l.to_lowercase()))
            .collect();
        Ok(Self {
            dictionary,
            allowed,
        })
    }

    /// Whether `word` is spelled correctly (or deliberately, for slang).
    pub fn is_correct(&self, word: &str) -> bool {
        let word = normalize_apostrophes(word);
        self.knows(&word) || elongation_variants(&word).iter().any(|w| self.knows(w))
    }

    /// Spelling suggestions for `word`, best first. Slow (tens of milliseconds): call it only when
    /// the user asks, off the main thread.
    pub fn suggest(&self, word: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        let word = normalize_apostrophes(word);
        let shortened = elongation_variants(&word);
        let word = shortened.first().unwrap_or(&word);
        self.dictionary.suggest(word, &mut suggestions);
        suggestions.truncate(MAX_SUGGESTIONS);
        suggestions
    }

    fn knows(&self, word: &str) -> bool {
        if self.allowed.contains(&word.to_lowercase()) || self.in_dictionary(word) {
            return true;
        }
        // "dis-moi", "peut-être": every part known.
        if word.contains('-') {
            return word
                .split('-')
                .all(|part| part.is_empty() || self.knows(part));
        }
        // "j'arrive": a known elision before a known word.
        if let Some((prefix, rest)) = word.rsplit_once('\'') {
            let prefix = prefix.to_lowercase();
            let prefix = prefix.rsplit('\'').next().unwrap_or(&prefix);
            return ELISIONS.contains(&prefix) && !rest.is_empty() && self.knows(rest);
        }
        false
    }

    fn in_dictionary(&self, word: &str) -> bool {
        self.dictionary.check(word)
            || (word.contains('\'') && self.dictionary.check(&word.replace('\'', "’")))
    }
}

impl GrammarChecker for FrenchSpeller {
    fn language(&self) -> Language {
        Language::Fr
    }

    /// Unknown words, without suggestions (see [`FrenchSpeller::suggest`]).
    fn check(&self, text: &str) -> Vec<Issue> {
        words(text)
            .filter(|&(start, end)| !self.is_correct(&text[start..end]))
            .map(|(start, end)| Issue {
                start,
                end,
                rule_id: SPELLING_RULE.into(),
                message: "unknown word".into(),
                suggestions: Vec::new(),
            })
            .collect()
    }
}

fn normalize_apostrophes(word: &str) -> String {
    word.replace('’', "'")
}

/// The word with every run of three or more identical letters shortened to one, then to two
/// letters ("troooop" gives "trop" and "troop"). Empty when the word has no such run.
fn elongation_variants(word: &str) -> Vec<String> {
    let mut runs: Vec<(char, usize)> = Vec::new();
    for c in word.chars() {
        match runs.last_mut() {
            Some((last, n)) if *last == c => *n += 1,
            _ => runs.push((c, 1)),
        }
    }
    if !runs.iter().any(|&(c, n)| n >= 3 && c.is_alphabetic()) {
        return Vec::new();
    }
    [1, 2]
        .into_iter()
        .map(|keep| {
            runs.iter()
                .flat_map(|&(c, n)| {
                    let n = if n >= 3 && c.is_alphabetic() { keep } else { n };
                    std::iter::repeat_n(c, n)
                })
                .collect()
        })
        .collect()
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric()
}

fn is_joiner(c: char) -> bool {
    matches!(c, '\'' | '’' | '-')
}

/// Byte spans of the words to check in `text`: letters joined by apostrophes or hyphens, skipping
/// numbers, links, addresses, mentions and hashtags.
fn words(text: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    let limit = text
        .char_indices()
        .nth(MAX_CHECKED_CHARS)
        .map_or(text.len(), |(i, _)| i);
    let text = &text[..limit];
    let mut spans = Vec::new();
    let mut chunk_start = None;
    for (i, c) in text
        .char_indices()
        .chain(std::iter::once((text.len(), ' ')))
    {
        match (c.is_whitespace(), chunk_start) {
            (false, None) => chunk_start = Some(i),
            (true, Some(start)) => {
                words_in_chunk(text, start, i, &mut spans);
                chunk_start = None;
            }
            _ => {}
        }
    }
    spans.into_iter()
}

fn words_in_chunk(text: &str, start: usize, end: usize, spans: &mut Vec<(usize, usize)>) {
    let chunk = &text[start..end];
    let lower = chunk.to_lowercase();
    if chunk.contains("://")
        || lower.starts_with("www.")
        || chunk.contains('@')
        || chunk.starts_with('#')
    {
        return;
    }
    let chars: Vec<(usize, char)> = chunk.char_indices().collect();
    let mut i = 0;
    while i < chars.len() {
        if !is_word_char(chars[i].1) {
            i += 1;
            continue;
        }
        let first = i;
        let mut last = i;
        i += 1;
        while i < chars.len() {
            let c = chars[i].1;
            if is_word_char(c) {
                last = i;
            } else if !(is_joiner(c) && chars.get(i + 1).is_some_and(|&(_, n)| is_word_char(n))) {
                break;
            }
            i += 1;
        }
        let word_end = chars[last].0 + chars[last].1.len_utf8();
        let word = &chunk[chars[first].0..word_end];
        if !word.chars().any(|c| c.is_numeric()) {
            spans.push((start + chars[first].0, start + word_end));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;

    use super::*;

    fn speller() -> &'static FrenchSpeller {
        static SPELLER: OnceLock<FrenchSpeller> = OnceLock::new();
        SPELLER.get_or_init(|| {
            FrenchSpeller::new(
                include_str!("../../../grammar/fr/hunspell/fr.aff"),
                include_str!("../../../grammar/fr/hunspell/fr.dic"),
                include_str!("../../../grammar/fr/allowlist.txt"),
            )
            .unwrap()
        })
    }

    fn unknown(text: &str) -> Vec<&str> {
        speller()
            .check(text)
            .into_iter()
            .map(|i| &text[i.start..i.end])
            .collect::<Vec<_>>()
    }

    #[test]
    fn reports_misspelled_words_only() {
        assert_eq!(
            unknown("Je suis trop contnet de te voir demin"),
            ["contnet", "demin"]
        );
        assert!(unknown("Ça va ? On se voit à l'école demain.").is_empty());
    }

    #[test]
    fn accepts_both_spellings_and_apostrophes() {
        assert!(unknown("un événement, un évènement, aujourd’hui, aujourd'hui").is_empty());
        assert!(unknown("j'arrive, qu'elle vienne, c'est jusqu'à lundi").is_empty());
        assert!(unknown("dis-moi, peut-être, rendez-vous").is_empty());
    }

    #[test]
    fn slang_from_the_allowlist_is_never_reported() {
        assert!(unknown("mdr tkt jsp stp bcp pk jtm ptdr rdv ok oké chuis jsuis").is_empty());
        assert!(unknown("MDR wesh LOL").is_empty());
    }

    #[test]
    fn elongated_words_count_as_their_short_form() {
        assert!(unknown("troooop bieeeen mdrrrr ouiiiii").is_empty());
        assert_eq!(unknown("contnnnet"), ["contnnnet"]);
    }

    #[test]
    fn skips_emoji_numbers_links_and_mentions() {
        assert!(
            unknown("😂 on se voit à 18h 👍 https://exemple.fr/xyzq @mathou #tropbiennn 2e")
                .is_empty()
        );
    }

    #[test]
    fn spans_are_byte_offsets_on_char_boundaries() {
        let text = "« Ça » erreurr, ét bienn";
        let issues = speller().check(text);
        assert_eq!(issues.len(), 3);
        assert_eq!(&text[issues[0].start..issues[0].end], "erreurr");
        assert_eq!(issues[0].rule_id, SPELLING_RULE);
    }

    #[test]
    fn suggests_the_right_word() {
        assert!(speller().suggest("demin").contains(&"demain".to_string()));
        assert!(speller()
            .suggest("controle")
            .contains(&"contrôle".to_string()));
        assert!(speller()
            .suggest("contnnnet")
            .contains(&"content".to_string()));
    }

    #[test]
    fn elongation_variants_shorten_runs_of_three_or_more() {
        assert_eq!(elongation_variants("troooop"), ["trop", "troop"]);
        assert_eq!(elongation_variants("mdrrr"), ["mdr", "mdrr"]);
        assert!(elongation_variants("bonne").is_empty());
    }
}
