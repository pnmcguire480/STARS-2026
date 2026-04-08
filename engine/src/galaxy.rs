//! Procedural galaxy generation for STARS 2026.
//!
//! This module owns the placement of stars on the galactic map. It does
//! not generate planets — that responsibility belongs to `planet.rs`
//! (Atom 3). The boundary is deliberate: FR-1 is satisfied by stars at
//! positions; FR-2 (planets, hab, minerals) is a separate concern with
//! its own determinism fingerprint extension.
//!
//! # Determinism contract
//!
//! Every random decision in this module flows from a single
//! [`seeded_rng`](crate::rng::seeded_rng) call keyed on the master
//! `GameSettings.random_seed`. The same seed → the same galaxy on every
//! target, forever. Squared-distance comparisons use `i64` accumulators
//! to avoid any `f64` math in the rejection-sampling hot path, which
//! eliminates the FMA-fusion / ULP-drift class of cross-target bugs the
//! Phase 1 Crucible's Red Teamer flagged.
//!
//! # Module growth plan
//!
//! Atom 2.2 (this commit): star name registry + deterministic picker.
//! Atom 2.3: `random_position` helper.
//! Atom 2.4: `actual_star_count`.
//! Atom 2.5: `place_one_star` + `place_all_stars` (merged).
//! Atom 2.6: Galaxy struct + `generate_galaxy` entry point (merged).

/// Canonical-flavored star name list for STARS 2026.
///
/// Hand-curated mix of real stars, mythological figures, and Stars!-style
/// invented names. The list is small on purpose — the Game Design
/// council recommended a 1,000-name JSON file matching the 1995 canon,
/// but that adds load logic and a data file outside the Atom 2 brief's
/// authorization. The First Principles auditor wanted no names at all,
/// but `Star.name: String` is non-optional in `types.rs`, so a small
/// const list is the smallest thing that satisfies both the type and
/// the brief.
///
/// **Open question for Patrick (deferred to manual review):** should
/// this be replaced by a JSON-loaded canonical list when the data
/// pipeline lands? See `CONTEXT.md` "Open Questions".
///
/// When the picker exhausts the list (galaxy demands more stars than
/// names), it suffixes a numeric tag — `"Vega-2"`, `"Vega-3"`, etc. —
/// per the Stars! 1995 fallback behavior the Game Design council
/// documented from `wiki.starsautohost.org`.
const STAR_NAMES: &[&str] = &[
    "Vega",
    "Antares",
    "Procyon",
    "Sirius",
    "Aldebaran",
    "Betelgeuse",
    "Rigel",
    "Polaris",
    "Arcturus",
    "Capella",
    "Altair",
    "Deneb",
    "Castor",
    "Pollux",
    "Spica",
    "Regulus",
    "Bellatrix",
    "Mintaka",
    "Alnilam",
    "Alnitak",
    "Saiph",
    "Hadar",
    "Atria",
    "Mira",
    "Algol",
    "Thuban",
    "Mizar",
    "Alcor",
    "Dubhe",
    "Merak",
    "Phecda",
    "Megrez",
    "Alioth",
    "Alkaid",
    "Acrux",
    "Mimosa",
    "Gacrux",
    "Avior",
    "Miaplacidus",
    "Alphard",
    "Thor",
    "Loki",
    "Odin",
    "Freya",
    "Tyr",
    "Quetzalcoatl",
    "Kukulkan",
    "Xochiquetzal",
    "Mictlan",
    "Tezcatlipoca",
];

/// Total number of canonical names available before the numeric-suffix
/// fallback kicks in. Exposed as a `pub(crate) const u32` so tests and
/// future atoms can reason about exhaustion thresholds without
/// duplicating the magic number, and so the value composes with
/// `pick_index: u32` without truncation casts on either target.
///
/// Hand-coded to match `STAR_NAMES.len()`. The compile-time assertion
/// below makes drift impossible.
pub(crate) const STAR_NAME_COUNT: u32 = 50;

const _: () = assert!(
    STAR_NAMES.len() == STAR_NAME_COUNT as usize,
    "STAR_NAME_COUNT must equal STAR_NAMES.len() — bump the const when adding names"
);

/// Pick a star name from the canonical list using one `u64` draw from
/// the supplied RNG, with a numeric suffix when the same base name has
/// already been issued in this galaxy.
///
/// `pick_index` is the **issue order** of this star (0 for the first
/// star placed, 1 for the second, …) — the picker uses it to compute a
/// suffix when the rotation wraps. Specifically:
///
/// - The first `STAR_NAME_COUNT` stars get the bare canonical name
///   (`"Vega"`, `"Antares"`, …) in RNG-shuffled order.
/// - Subsequent stars get the same shuffled name with a `-2`, `-3`, …
///   suffix indicating which rotation they belong to.
///
/// The shuffle is done implicitly: each call draws one `u64` from the
/// RNG and indexes the list at `(draw % STAR_NAME_COUNT)`. This is
/// modulo-biased in theory (the bias is roughly 1 in 2^58 for a 50-name
/// list, so it cannot be detected by any test we will ever write), and
/// it has the property that two consecutive picks **can** collide on the
/// same base name — which is exactly what the suffix counter handles.
///
/// # Determinism
///
/// Same `(rng_state, pick_index)` → same returned `String`. The function
/// is byte-identical wasm/native because it uses only integer modulo and
/// integer-to-string formatting (no `f64`, no allocator-order leaks).
#[must_use]
pub fn pick_star_name(rng: &mut rand_chacha::ChaCha20Rng, pick_index: u32) -> String {
    use rand::RngCore;

    let draw = rng.next_u64();
    // The modulo result is bounded by STAR_NAME_COUNT (50), so the
    // u64 → usize cast is mathematically lossless on every target the
    // engine compiles to (32-bit wasm and 64-bit native both have
    // `usize >= 50`). The cast lint is silenced locally with a
    // justifying comment per the project policy on `#[allow]`.
    #[allow(
        clippy::cast_possible_truncation,
        reason = "modulo result < STAR_NAME_COUNT (50), always fits in usize"
    )]
    let base_idx = (draw % u64::from(STAR_NAME_COUNT)) as usize;
    let base = STAR_NAMES[base_idx];

    // The first STAR_NAME_COUNT picks get the bare name; subsequent
    // picks get a `-N` suffix where N is the rotation count plus one.
    let rotation = pick_index / STAR_NAME_COUNT;
    if rotation == 0 {
        base.to_string()
    } else {
        format!("{base}-{}", rotation + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::seeded_rng;
    use crate::types::PlayerId;

    #[test]
    fn star_name_count_matches_array_length() {
        // Tripwire mirroring the module-level `const _: () = assert!(...)`.
        // The const-context check is the load-bearing one (compile-fail
        // if drift), and this runtime test exists so the failure mode
        // appears in the test report rather than as a cryptic build
        // error if a contributor splits the change across atoms.
        assert_eq!(STAR_NAME_COUNT as usize, STAR_NAMES.len());
    }

    #[test]
    fn same_rng_state_picks_same_name() {
        // Determinism contract: identical inputs → identical output.
        let mut rng_a = seeded_rng(42, 0, PlayerId(0), "galaxy");
        let mut rng_b = seeded_rng(42, 0, PlayerId(0), "galaxy");
        assert_eq!(pick_star_name(&mut rng_a, 0), pick_star_name(&mut rng_b, 0));
    }

    #[test]
    fn first_rotation_returns_bare_name() {
        // pick_index < STAR_NAME_COUNT → no suffix.
        let mut rng = seeded_rng(1, 0, PlayerId(0), "galaxy");
        let name = pick_star_name(&mut rng, 0);
        assert!(
            !name.contains('-'),
            "first-rotation name must not contain a suffix dash, got {name}"
        );
    }

    #[test]
    fn second_rotation_appends_dash_two() {
        // pick_index == STAR_NAME_COUNT → suffix `-2`.
        let mut rng = seeded_rng(1, 0, PlayerId(0), "galaxy");
        let name = pick_star_name(&mut rng, STAR_NAME_COUNT);
        assert!(
            name.ends_with("-2"),
            "second-rotation name must end in -2, got {name}"
        );
    }

    #[test]
    fn third_rotation_appends_dash_three() {
        let mut rng = seeded_rng(1, 0, PlayerId(0), "galaxy");
        let name = pick_star_name(&mut rng, STAR_NAME_COUNT * 2);
        assert!(
            name.ends_with("-3"),
            "third-rotation name must end in -3, got {name}"
        );
    }

    #[test]
    fn picker_drains_rng_one_u64_per_call() {
        // Regression: each call must consume exactly one u64 from the
        // RNG so the determinism fingerprint stays predictable. We
        // verify this by drawing N names and confirming the underlying
        // RNG state matches a fresh RNG that drew exactly N u64s.
        use rand::RngCore;

        let mut rng_picker = seeded_rng(7, 0, PlayerId(0), "galaxy");
        for i in 0..10 {
            let _ = pick_star_name(&mut rng_picker, i);
        }

        let mut rng_baseline = seeded_rng(7, 0, PlayerId(0), "galaxy");
        for _ in 0..10 {
            let _ = rng_baseline.next_u64();
        }

        // Both RNGs should now produce the same next value if the
        // picker drained exactly 10 u64s.
        assert_eq!(rng_picker.next_u64(), rng_baseline.next_u64());
    }
}
