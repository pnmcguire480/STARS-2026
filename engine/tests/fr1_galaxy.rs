//! FR-1 acceptance test — procedural galaxy generation.
//!
//! From SPEC.md (amended 2026-04-08, Atom A.1):
//!
//! > **FR-1**: Generate a procedural galaxy from a seed (24–100 stars
//! > for v0.1, "Tiny" size). See "Deviations from 1995 canon" — Tiny
//! > floor is 24 per canon, not 32.
//!
//! This integration test pins the FR-1 contract end-to-end against the
//! public `generate_galaxy` API. It is the canonical user-facing
//! verification that the Atom 2 sub-atoms compose into a working
//! galaxy generator: same seed → same galaxy, star count in the FR-1
//! envelope, and no rejection-sampler explosions across a wide
//! cross-section of seeds.
//!
//! **History (resolved):**
//!
//! Atom 2.4 shipped a symmetric `[-10, +10]%` jitter which could
//! under-shoot the then-SPEC floor of 32; Atom 2.9 P0-fixed this by
//! flipping to asymmetric `[0, +20]%`. Atom A (2026-04-08) then
//! amended SPEC FR-1 from 32 to 24 (per canon, SPEC D-2) and dropped
//! `GalaxySize::Tiny.target_stars()` from 32 to 24. The asymmetric
//! jitter shape is preserved; the floor is now 24. This test asserts
//! the new SPEC envelope `[24, 100]` directly.

use stars2026_engine::data::load_star_names;
use stars2026_engine::galaxy::generate_galaxy;
use stars2026_engine::types::{AiDifficulty, GalaxyDensity, GalaxySize, GameSettings};

fn names() -> Vec<String> {
    load_star_names().expect("bundled star_names.json must parse")
}

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
/// produce a star count inside the SPEC FR-1 envelope `[24, 100]`
/// (amended 2026-04-08), and the placement pipeline must never
/// return an error.
#[test]
fn fr1_tiny_normal_envelope_holds_across_100_seeds() {
    let star_names = names();
    for seed in 0..100u64 {
        let settings = tiny_normal_settings(seed);
        let galaxy = generate_galaxy(&settings, &star_names)
            .unwrap_or_else(|e| panic!("FR-1 seed {seed} failed to generate: {e}"));
        let n = galaxy.stars.len();
        assert!(
            (24..=100).contains(&n),
            "FR-1 envelope violation: seed {seed} produced {n} stars (expected 24..=100 per SPEC FR-1)"
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
    let star_names = names();
    let settings = tiny_normal_settings(0xFEED_FACE);
    let g1 = generate_galaxy(&settings, &star_names).expect("first generation");
    let g2 = generate_galaxy(&settings, &star_names).expect("second generation");
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
/// per-seed range produced by Tiny+Normal across 200 seeds. Originally
/// documented the Atom 2.4 deferred-question case; now preserved as a
/// tripwire that the jitter shape hasn't drifted. Asserts only that
/// the observed range is narrow (max - min ≤ 10).
#[test]
fn fr1_tiny_normal_observed_range_is_narrow() {
    let star_names = names();
    let mut min = u32::MAX;
    let mut max = 0u32;
    for seed in 0..200u64 {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "galaxy.stars.len() bounded by FR-1 envelope (≤100)"
        )]
        let n = generate_galaxy(&tiny_normal_settings(seed), &star_names)
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

/// **A.11 — P1-8 resolution.** Huge+Packed saturation stress test.
///
/// The Crucible Assumption Auditor flagged that `STAR_PLACEMENT_ATTEMPTS`
/// (the per-star retry budget in `place_one_star`) was only exercised
/// by the Tiny+Normal FR-1 envelope test across 100 seeds. The actual
/// saturation case — the densest density on the largest map — was
/// never stressed, which is precisely where the rejection sampler is
/// most likely to exhaust its budget if a future tweak to
/// `min_star_distance` (see docs/FORMULAS.md F-1) pushes the packing
/// constraint past its limit.
///
/// This test generates a Huge+Packed galaxy across 20 seeds and
/// asserts success on every one. 20 is enough to catch the failure
/// mode without ballooning test runtime (a single Huge+Packed
/// generation is ~600 stars × ~100 rejection draws per star in the
/// worst case, so ~60k RNG calls per seed).
#[test]
fn fr1_huge_packed_saturation_holds_across_20_seeds() {
    let star_names = names();
    for seed in 0..20u64 {
        let settings = GameSettings {
            galaxy_size: GalaxySize::Huge,
            density: GalaxyDensity::Packed,
            player_count: 1,
            starting_year: 2400,
            victory_conditions: vec![],
            victory_requirements_met: 1,
            ai_difficulty: AiDifficulty::Standard,
            random_seed: seed,
        };
        let galaxy = generate_galaxy(&settings, &star_names).unwrap_or_else(|e| {
            panic!("Huge+Packed saturation: seed {seed} exhausted rejection budget: {e}")
        });
        assert!(
            galaxy.stars.len() >= 600,
            "Huge+Packed seed {seed} produced only {} stars (expected >= 600 base)",
            galaxy.stars.len()
        );
    }
}

/// FR-1 cross-density smoke test: a Tiny galaxy at every density tier
/// must successfully generate. This catches the failure mode where a
/// future tweak to `min_star_distance` accidentally pushes the
/// rejection sampler past its budget for one density.
#[test]
fn fr1_tiny_all_density_tiers_succeed() {
    let star_names = names();
    for density in [
        GalaxyDensity::Sparse,
        GalaxyDensity::Normal,
        GalaxyDensity::Dense,
        GalaxyDensity::Packed,
    ] {
        let mut settings = tiny_normal_settings(0xDEAD_BEEF);
        settings.density = density;
        let galaxy = generate_galaxy(&settings, &star_names)
            .unwrap_or_else(|e| panic!("FR-1 density {density:?} failed: {e}"));
        assert!(!galaxy.stars.is_empty(), "FR-1 density {density:?} empty");
    }
}
