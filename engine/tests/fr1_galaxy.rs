//! FR-1 acceptance test — procedural galaxy generation.
//!
//! From SPEC.md:
//!
//! > **FR-1**: Generate a procedural galaxy from a seed (32–100 stars
//! > for v0.1, "Tiny" size).
//!
//! This integration test pins the FR-1 contract end-to-end against the
//! public `generate_galaxy` API. It is the canonical user-facing
//! verification that the eight Atom 2 sub-atoms compose into a working
//! galaxy generator: same seed → same galaxy, star count in the FR-1
//! envelope, and no rejection-sampler explosions across a wide
//! cross-section of seeds.
//!
//! **Calibration note (resolved by Atom 2.9):**
//!
//! The original Atom 2.4 jitter was symmetric `[-10, +10]` percent,
//! which produced worst-case 29 stars on Tiny+Normal — below SPEC
//! FR-1's stated floor of 32. The Atom 2 closing Crucible flagged
//! this as a P0 SPEC violation. Atom 2.9 changed the jitter to
//! `[0, +20]` so `actual_star_count` is always `>= base *
//! density_scale / 100`. For Tiny+Normal that bottoms out at exactly
//! 32, hitting the SPEC FR-1 floor on every seed. This test asserts
//! the SPEC envelope `[32, 100]` directly.

use stars2026_engine::galaxy::generate_galaxy;
use stars2026_engine::types::{AiDifficulty, GalaxyDensity, GalaxySize, GameSettings};

fn tiny_normal_settings(seed: u64) -> GameSettings {
    GameSettings {
        galaxy_size: GalaxySize::Tiny,
        density: GalaxyDensity::Normal,
        player_count: 1,
        starting_year: 2400,
        victory_conditions: vec![],
        victory_requirements_met: 1,
        ai_difficulty: AiDifficulty::Standard,
        random_seed: seed,
    }
}

/// FR-1 envelope: every Tiny+Normal galaxy across 100 seeds must
/// produce a star count inside the SPEC FR-1 envelope `[32, 100]`,
/// and the placement pipeline must never return an error.
#[test]
fn fr1_tiny_normal_envelope_holds_across_100_seeds() {
    for seed in 0..100u64 {
        let settings = tiny_normal_settings(seed);
        let galaxy = generate_galaxy(&settings)
            .unwrap_or_else(|e| panic!("FR-1 seed {seed} failed to generate: {e}"));
        let n = galaxy.stars.len();
        assert!(
            (32..=100).contains(&n),
            "FR-1 envelope violation: seed {seed} produced {n} stars (expected 32..=100 per SPEC FR-1)"
        );
    }
}

/// FR-1 determinism: two consecutive `generate_galaxy` calls with the
/// same `GameSettings` must produce byte-equal `Galaxy`s. This is the
/// per-call replay contract that powers IndexedDB save/load (FR-16),
/// the multiplayer determinism gate (FR-31), and the cross-target
/// fingerprint (`engine/tests/determinism.rs`).
#[test]
fn fr1_determinism_same_seed_same_galaxy() {
    let settings = tiny_normal_settings(0xFEED_FACE);
    let g1 = generate_galaxy(&settings).expect("first generation");
    let g2 = generate_galaxy(&settings).expect("second generation");
    assert_eq!(g1.stars.len(), g2.stars.len(), "star count drift");
    for (i, (a, b)) in g1.stars.iter().zip(g2.stars.iter()).enumerate() {
        assert_eq!(a.id, b.id, "star {i} id drift");
        assert_eq!(a.name, b.name, "star {i} name drift");
        assert_eq!(
            a.position.x.to_bits(),
            b.position.x.to_bits(),
            "star {i} x bit drift"
        );
        assert_eq!(
            a.position.y.to_bits(),
            b.position.y.to_bits(),
            "star {i} y bit drift"
        );
    }
}

/// Calibration observation (NOT a hard FR-1 assertion): the actual
/// per-seed range produced by Tiny+Normal across 200 seeds. Documents
/// the deferred-question case from Atom 2.4 — the worst-case lower
/// bound is currently below the SPEC's stated FR-1 floor of 32. The
/// test asserts only that the observed range is *narrow enough* to
/// inform Patrick's morning calibration decision (max - min ≤ 10).
#[test]
fn fr1_tiny_normal_observed_range_is_narrow() {
    let mut min = u32::MAX;
    let mut max = 0u32;
    for seed in 0..200u64 {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "galaxy.stars.len() bounded by FR-1 envelope (≤100)"
        )]
        let n = generate_galaxy(&tiny_normal_settings(seed))
            .unwrap()
            .stars
            .len() as u32;
        if n < min {
            min = n;
        }
        if n > max {
            max = n;
        }
    }
    // The observation: print the actual range and assert it's narrow.
    eprintln!("FR-1 calibration observation: Tiny+Normal range = [{min}, {max}]");
    assert!(
        max - min <= 10,
        "FR-1 calibration: range [{min}, {max}] wider than expected"
    );
}

/// FR-1 cross-density smoke test: a Tiny galaxy at every density tier
/// must successfully generate. This catches the failure mode where a
/// future tweak to `min_star_distance` accidentally pushes the
/// rejection sampler past its budget for one density.
#[test]
fn fr1_tiny_all_density_tiers_succeed() {
    for density in [
        GalaxyDensity::Sparse,
        GalaxyDensity::Normal,
        GalaxyDensity::Dense,
        GalaxyDensity::Packed,
    ] {
        let mut settings = tiny_normal_settings(0xDEAD_BEEF);
        settings.density = density;
        let galaxy = generate_galaxy(&settings)
            .unwrap_or_else(|e| panic!("FR-1 density {density:?} failed: {e}"));
        assert!(!galaxy.stars.is_empty(), "FR-1 density {density:?} empty");
    }
}
