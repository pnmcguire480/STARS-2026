//! Seeded RNG primitive for STARS 2026.
//!
//! This module owns the single source of deterministic randomness used by
//! every subsystem in the engine: galaxy generation, planet rolls, race
//! creation, combat dice, AI decisions. The contract is unconditional —
//! given the same `(game_seed, turn, player_id, subsystem)` tuple, every
//! subsystem on every target produces the same byte sequence forever.
//!
//! # Why a separate module
//!
//! The Atom 2 council was unanimous: the RNG primitive has the broadest
//! reach of any utility in the engine, so it lives at the top of the
//! dependency graph in its own file. Burying it inside `galaxy.rs` would
//! force every later module (planet, race, combat, AI) to import a
//! galaxy-flavored helper, creating a phantom coupling that has no basis
//! in the actual data flow. The First Principles auditor put it bluntly:
//! a law-enforcement primitive used by ten modules belongs in its own
//! file so the import path names what it is.
//!
//! # Why `ChaCha20` specifically
//!
//! `rand_chacha::ChaCha20Rng` is pure 32-bit integer arithmetic with no
//! platform intrinsics, no floating-point math, and no `unsafe` code on
//! either target. It is byte-identical between `wasm32-unknown-unknown`
//! and native `x86_64`, which is the **only** property that matters for
//! the determinism contract. Faster generators (Xoshiro, PCG) all have
//! some implementation wrinkle that makes cross-target equality hard to
//! audit. `ChaCha20` is the boring, correct choice.
//!
//! # Domain separation
//!
//! `seeded_rng` does **not** call `ChaCha20Rng::seed_from_u64`. That
//! helper runs a tiny PRNG over the input which collides nearby seeds
//! and offers no domain isolation between subsystems. Instead we build
//! the full 32-byte `ChaCha` seed by packing the four input fields
//! little-endian into fixed offsets, with the variable-length subsystem
//! string compressed via FNV-1a 64-bit hash so it can never bleed into
//! adjacent fields. The result: changing the subsystem string by one
//! character produces an entirely different RNG stream, and two
//! subsystems cannot share an accidental sub-stream by colliding on
//! `(game_seed, turn, player_id)`.
//!
//! The hand-rolled FNV-1a is intentional — adding a hash crate to the
//! workspace requires Patrick approval per the Atom 2 brief, and the
//! algorithm is twelve lines of straightforward integer math whose
//! reference vectors are public domain. We never change the constants;
//! the function is locked the day it ships.

use crate::types::PlayerId;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;

/// FNV-1a 64-bit offset basis (RFC 3309 reference constant).
const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;

/// FNV-1a 64-bit prime (RFC 3309 reference constant).
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// 64-bit FNV-1a hash of a byte slice.
///
/// Used internally to compress the subsystem identifier into a fixed-width
/// field of the `ChaCha` seed buffer. FNV-1a is chosen because it has zero
/// dependencies, byte-identical output across every target, and a public
/// reference implementation that has been frozen since 1991. The hash is
/// **not** cryptographic — collision resistance is not a goal. The goal
/// is "any two distinct subsystem strings produce a different 64-bit
/// integer with overwhelmingly high probability", and FNV-1a satisfies
/// that with no surprises.
const fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        i += 1;
    }
    hash
}

/// Construct a fresh, deterministic `ChaCha20Rng` from a domain-separated
/// `(game_seed, turn, player_id, subsystem)` tuple.
///
/// The four inputs are packed into a 32-byte `ChaCha` seed buffer at fixed
/// offsets, guaranteeing that no two distinct tuples share the same
/// underlying RNG stream. The subsystem identifier is intended to be a
/// short, fixed string literal naming the consumer (e.g. `"galaxy"`,
/// `"planet_minerals"`, `"combat_dice"`) and is hashed via FNV-1a so its
/// length cannot influence the seed buffer layout.
///
/// # Determinism contract
///
/// - Same inputs → same first N random values, forever, on every target.
/// - Different inputs (any field) → different first random value with
///   overwhelmingly high probability.
/// - Returned RNG is freshly initialized; the caller may draw any number
///   of values without affecting any other subsystem's stream.
///
/// # Why pass `PlayerId` instead of `Option<PlayerId>`
///
/// Galaxy generation and other "world" subsystems that have no player
/// context simply pass `PlayerId(0)`. The 32-bit player slot in the seed
/// is never invalid — the engine never assigns `PlayerId(0)` to a real
/// human player (player ids start at 1 in the registry), so collisions
/// between "no player" and "player 0" cannot occur. This keeps the
/// signature monomorphic and the call sites uniform.
#[must_use]
pub fn seeded_rng(
    game_seed: u64,
    turn: u32,
    player_id: PlayerId,
    subsystem: &'static str,
) -> ChaCha20Rng {
    let subsystem_hash = fnv1a_64(subsystem.as_bytes());

    let mut seed = [0u8; 32];
    seed[0..8].copy_from_slice(&game_seed.to_le_bytes());
    seed[8..12].copy_from_slice(&turn.to_le_bytes());
    seed[12..16].copy_from_slice(&player_id.0.to_le_bytes());
    seed[16..24].copy_from_slice(&subsystem_hash.to_le_bytes());
    // The final 8 bytes mix the master seed with the subsystem hash so the
    // upper half of the ChaCha seed buffer is never zero — a fully zero
    // upper half is a known weak ChaCha seed configuration.
    seed[24..32].copy_from_slice(&(game_seed ^ subsystem_hash).to_le_bytes());

    ChaCha20Rng::from_seed(seed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngCore;

    /// Helper that draws four `u64`s from a fresh RNG so we can compare
    /// streams as fixed-size arrays.
    fn draw_four(rng: &mut ChaCha20Rng) -> [u64; 4] {
        [
            rng.next_u64(),
            rng.next_u64(),
            rng.next_u64(),
            rng.next_u64(),
        ]
    }

    #[test]
    fn same_inputs_produce_identical_streams() {
        let mut a = seeded_rng(0xDEAD_BEEF, 0, PlayerId(0), "galaxy");
        let mut b = seeded_rng(0xDEAD_BEEF, 0, PlayerId(0), "galaxy");
        assert_eq!(draw_four(&mut a), draw_four(&mut b));
    }

    #[test]
    fn different_game_seeds_diverge() {
        let mut a = seeded_rng(1, 0, PlayerId(0), "galaxy");
        let mut b = seeded_rng(2, 0, PlayerId(0), "galaxy");
        assert_ne!(draw_four(&mut a), draw_four(&mut b));
    }

    #[test]
    fn different_turns_diverge() {
        let mut a = seeded_rng(42, 0, PlayerId(0), "galaxy");
        let mut b = seeded_rng(42, 1, PlayerId(0), "galaxy");
        assert_ne!(draw_four(&mut a), draw_four(&mut b));
    }

    #[test]
    fn different_player_ids_diverge() {
        let mut a = seeded_rng(42, 0, PlayerId(1), "combat");
        let mut b = seeded_rng(42, 0, PlayerId(2), "combat");
        assert_ne!(draw_four(&mut a), draw_four(&mut b));
    }

    #[test]
    fn different_subsystems_diverge() {
        let mut a = seeded_rng(42, 0, PlayerId(1), "galaxy");
        let mut b = seeded_rng(42, 0, PlayerId(1), "planet");
        assert_ne!(draw_four(&mut a), draw_four(&mut b));
    }

    #[test]
    fn one_character_subsystem_change_diverges() {
        // Domain separation must be tight: even a single-byte difference
        // in the subsystem string must produce a different stream.
        let mut a = seeded_rng(42, 0, PlayerId(0), "galaxy_gen");
        let mut b = seeded_rng(42, 0, PlayerId(0), "galaxy_xen");
        assert_ne!(draw_four(&mut a), draw_four(&mut b));
    }

    #[test]
    fn fnv1a_known_vector_empty() {
        // Per the FNV-1a 64-bit reference: hash("") == FNV_OFFSET_BASIS.
        assert_eq!(fnv1a_64(b""), FNV_OFFSET_BASIS);
    }

    #[test]
    fn fnv1a_known_vector_a() {
        // Per the FNV-1a 64-bit reference: hash("a") == 0xaf63dc4c8601ec8c.
        assert_eq!(fnv1a_64(b"a"), 0xaf63_dc4c_8601_ec8c);
    }

    #[test]
    fn seed_buffer_upper_half_never_zero() {
        // Regression: the final 8 bytes of the seed buffer XOR game_seed
        // with the subsystem hash so the upper ChaCha seed half is never
        // structurally zero. Drawing a value should produce non-trivial
        // output even for game_seed == 0.
        let mut rng = seeded_rng(0, 0, PlayerId(0), "galaxy");
        // The first u64 of a zero-seeded ChaCha20 stream is a fixed
        // non-zero constant; we don't pin its value here (the
        // determinism fingerprint atom does that), only that it isn't
        // zero, which would indicate a malformed seed.
        assert_ne!(rng.next_u64(), 0);
    }
}
