//! Public facade of the Rust core, exposed to Swift and Kotlin through UniFFI (ADR-0002).
//!
//! This is the **only** surface the native shells see. Keep it tiny: a handful of functions with
//! plain data types. Everything behind it can change without touching Swift or Kotlin.
//!
//! Milestone M0 exposes version and theme validation only, to prove the binding toolchain.
//! `analyze` (emotion + grammar + intensity) arrives in M1/M2.

uniffi::setup_scaffolding!();

/// Version of the Rust core, shown in the app's debug screen.
#[uniffi::export]
pub fn core_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Animation intensity level (1 to 3) for a number of writing errors.
#[uniffi::export]
pub fn intensity_level(errors: u32) -> u8 {
    lt_core::Intensity::from_error_count(errors as usize).level()
}

/// Validates a theme file. Returns `None` if valid, or the error message.
#[uniffi::export]
pub fn validate_theme(json: String) -> Option<String> {
    lt_fx::Theme::from_json(&json).err().map(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_works() {
        assert!(!core_version().is_empty());
        assert_eq!(intensity_level(0), 3);
        assert_eq!(intensity_level(2), 1);
        assert!(validate_theme(include_str!("../../../themes/anime/theme.json").into()).is_none());
        assert!(validate_theme("{}".into()).is_some());
    }
}
