//! Grammar and spelling checks (ADR-0006).
//!
//! One provider per language behind the [`GrammarChecker`] trait: an in-house checker for French
//! (ADR-0009) and Harper for English arrive in milestone M4. The scoring that
//! turns issues into animation intensity lives in `kd-core`.

use kd_core::{Intensity, Language};
use serde::{Deserialize, Serialize};

/// One spelling or grammar issue in a message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Issue {
    /// Start of the faulty span, in bytes from the start of the message.
    pub start: usize,
    /// End of the faulty span (exclusive), in bytes.
    pub end: usize,
    /// Provider rule identifier, used to group statistics ("a / à", "ça / sa"…).
    pub rule_id: String,
    /// Short explanation shown to the user.
    pub message: String,
    /// Replacement suggestions, best first.
    pub suggestions: Vec<String>,
}

/// A grammar provider for one language.
pub trait GrammarChecker: Send + Sync {
    /// Language handled by this provider.
    fn language(&self) -> Language;

    /// Returns the issues found in `text`, ordered by position.
    ///
    /// Deliberate abbreviations and slang ("mdr", "jsp") must not be reported.
    fn check(&self, text: &str) -> Vec<Issue>;
}

/// Provider that reports nothing. Used until real providers are integrated, and in tests.
#[derive(Debug, Clone, Copy)]
pub struct NoopChecker(pub Language);

impl GrammarChecker for NoopChecker {
    fn language(&self) -> Language {
        self.0
    }

    fn check(&self, _text: &str) -> Vec<Issue> {
        Vec::new()
    }
}

/// Animation intensity for a message, from its issues.
pub fn intensity(issues: &[Issue]) -> Intensity {
    Intensity::from_error_count(issues.len())
}

/// Applies one suggestion of one issue to `text`.
///
/// Returns `None` if the span is out of bounds, not on character boundaries, or the
/// suggestion index does not exist.
pub fn apply_fix(text: &str, issue: &Issue, suggestion: usize) -> Option<String> {
    let replacement = issue.suggestions.get(suggestion)?;
    if issue.start > issue.end
        || issue.end > text.len()
        || !text.is_char_boundary(issue.start)
        || !text.is_char_boundary(issue.end)
    {
        return None;
    }
    Some(format!(
        "{}{}{}",
        &text[..issue.start],
        replacement,
        &text[issue.end..]
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn issue(start: usize, end: usize, fix: &str) -> Issue {
        Issue {
            start,
            end,
            rule_id: "test".into(),
            message: "test".into(),
            suggestions: vec![fix.into()],
        }
    }

    #[test]
    fn noop_reports_nothing() {
        let checker = NoopChecker(Language::Fr);
        assert!(checker.check("sa va ?").is_empty());
        assert_eq!(checker.language(), Language::Fr);
    }

    #[test]
    fn fixing_errors_raises_intensity() {
        let text = "sa va a 18h";
        let issues = vec![issue(0, 2, "ça"), issue(6, 7, "à")];
        assert_eq!(intensity(&issues), Intensity::Low);

        // Fix from the end so earlier byte offsets stay valid.
        let fixed = apply_fix(text, &issues[1], 0).unwrap();
        let fixed = apply_fix(&fixed, &issues[0], 0).unwrap();
        assert_eq!(fixed, "ça va à 18h");
        assert_eq!(intensity(&[]), Intensity::Full);
    }

    #[test]
    fn apply_fix_rejects_bad_spans() {
        assert!(apply_fix("abc", &issue(2, 9, "x"), 0).is_none());
        assert!(apply_fix("é", &issue(1, 2, "x"), 0).is_none());
        assert!(apply_fix("abc", &issue(0, 1, "x"), 3).is_none());
    }
}
