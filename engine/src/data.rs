//! Data loader — the `data/*.json` on-ramp.
//!
//! Loads data files bundled with the engine at compile time via
//! `include_str!`, parses them with `serde_json`, and returns plain
//! owned data to the caller. This is the **first** use of the
//! `data/*.json` loader pattern in STARS 2026; the shape chosen here
//! sets the precedent for the deferred PRT/LRT JSON registry atom
//! (see `BACKLOG.md`).
//!
//! # Design constraints
//!
//! - **Determinism:** `include_str!` embeds the file contents into the
//!   binary at compile time. There is no filesystem access at runtime,
//!   which keeps the wasm32 build happy and eliminates the "file missing
//!   on the server" failure mode entirely.
//! - **DLC-as-JSON promise (SPEC.md):** the JSON format is the primary
//!   source of truth. A future DLC mechanism can swap `include_str!` for
//!   a runtime load without any caller having to change.
//! - **Argument passing over globals:** loaders return owned `Vec`s (or
//!   equivalent) that callers thread through as arguments. This is the
//!   Patrick decision from 2026-04-08 (P1-1 resolution): keep the
//!   `seed → galaxy` mapping pure so a given seed produces the same
//!   galaxy *for a given name list*, with the name list being part of
//!   the explicit inputs rather than an implicit global.

use crate::types::GameError;

/// The embedded JSON blob for `data/star_names.json`, resolved at
/// compile time. Path is relative to this source file.
const STAR_NAMES_JSON: &str = include_str!("../../data/star_names.json");

/// On-disk schema for `data/star_names.json`. The wrapping struct
/// exists so the JSON file can grow metadata fields (schema version,
/// source attribution) without breaking the loader.
#[derive(serde::Deserialize)]
struct StarNamesFile {
    names: Vec<String>,
}

/// Load the canonical star-name list from `data/star_names.json`.
///
/// The returned `Vec<String>` is intended to be passed into
/// [`crate::galaxy::generate_galaxy`] as an explicit argument. Do not
/// cache this globally — the whole point of the P1-1 migration is
/// that the name list is an input to the generator, not a side effect.
///
/// # Errors
///
/// Returns [`GameError::GalaxyGenerationFailed`] if the embedded JSON
/// fails to parse. In practice this is unreachable: the JSON is
/// embedded at compile time and a parse error would be caught by the
/// `load_star_names_parses` test below on every `cargo test` run. The
/// `Result` exists so the failure mode is visible at the call site
/// rather than papered over with `.unwrap()`.
pub fn load_star_names() -> Result<Vec<String>, GameError> {
    let parsed: StarNamesFile = serde_json::from_str(STAR_NAMES_JSON)
        .map_err(|_| GameError::GalaxyGenerationFailed("star_names.json parse error"))?;
    if parsed.names.is_empty() {
        return Err(GameError::GalaxyGenerationFailed(
            "star_names.json contains empty names list",
        ));
    }
    Ok(parsed.names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_star_names_parses() {
        let names = load_star_names().expect("bundled star_names.json must parse");
        assert!(!names.is_empty(), "loaded name list must not be empty");
    }

    #[test]
    fn load_star_names_contains_canon_anchors() {
        // Tripwire: if a contributor accidentally empties the file or
        // ships a schema change that drops the names array, these
        // canonical anchors will catch it before the fingerprint does.
        let names = load_star_names().unwrap();
        assert!(names.iter().any(|n| n == "Vega"));
        assert!(names.iter().any(|n| n == "Sirius"));
        assert!(names.iter().any(|n| n == "Rigel"));
    }

    #[test]
    fn load_star_names_deterministic() {
        // Two loads must return byte-identical vectors — `include_str!`
        // is compile-time so this is guaranteed, but the test pins the
        // contract for future refactors that might switch to runtime
        // loading.
        let a = load_star_names().unwrap();
        let b = load_star_names().unwrap();
        assert_eq!(a, b);
    }
}
