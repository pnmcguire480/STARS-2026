//! STARS 2026 game engine.
//!
//! Pure, deterministic game logic. No I/O, no time, no global state.
//! Compiles to both `wasm32-unknown-unknown` (browser) and native targets
//! (multiplayer server). Same source, byte-identical results.
//!
//! See `ARCHITECTURE.md` for the determinism contract.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

// Modules grow one sniff-tested atom at a time, per SNIFFTEST.md.
// Phase 1 order: types → rng → galaxy → race → planet → tech → ship
//              → fleet → combat → scanner → turn → ai

pub mod data;
pub mod galaxy;
pub mod rng;
pub mod types;

pub use types::GameError;

#[cfg(test)]
mod tests {
    #[test]
    fn engine_crate_compiles() {
        // Sentinel test so `cargo test` has something to run before Phase 1.
        assert_eq!(2 + 2, 4);
    }
}
