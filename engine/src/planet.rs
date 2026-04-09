//! Planet-level formulas for STARS 2026.
//!
//! This module contains **pure functions** over the type vocabulary in
//! `types.rs`. It does not own the `Planet` struct (that lives in
//! `types.rs` as a data record) and does not mutate planet state. The
//! turn engine calls these functions to compute habitability, growth,
//! and (in future atoms) resource generation and mineral extraction.
//!
//! # Atom C scope (FR-4 + FR-5)
//!
//! - **FR-4:** `habitability` — canonical Stars! 1995 hab formula,
//!   sourced from `craig-stars/cs/race.go` and the autohost wiki
//!   "Guts of the Habitability Formula". See `docs/FORMULAS.md` F-4.
//! - **FR-5:** `population_growth` — canonical Stars! 1995 growth
//!   formula with crowding, hostile death, and overcrowding. See
//!   `docs/FORMULAS.md` F-5.
//!
//! # Design decisions
//!
//! - **Free functions, not methods on `Planet`.** Matches the
//!   `galaxy.rs` pattern where `generate_galaxy` takes `&GameSettings`
//!   rather than being a method on `GameSettings`. The Rust council
//!   was unanimous.
//! - **f64 for the crowding factor.** The formula `(16/9)(1-ratio)^2`
//!   is naturally fractional. Atom B proved f64 is cross-target
//!   byte-identical, so we use it here and truncate at the final
//!   `Colonists` conversion.
//! - **Truncation (floor) for fractional colonists.** Patrick decision
//!   2026-04-09, matching Stars! 1995 canon. Documented in
//!   `docs/FORMULAS.md` F-5.

use crate::types::{EnvClick, HabAxis};

/// Compute the habitability contribution of a single environment axis.
///
/// Returns `(planet_value_points, ideality_factor, red_value)` where:
/// - `planet_value_points`: 0–10000 (squared distance-from-ideal score;
///   10000 = perfect, 0 = edge of tolerance). Only meaningful when the
///   planet is inside the race's tolerance range (i.e. `red_value == 0`).
/// - `ideality_factor`: starts at the caller's accumulated ideality and
///   is reduced when the planet is in the outer half of the tolerance
///   range on one side of center. Passed through unchanged when immune
///   or when the planet is in the inner half.
/// - `red_value`: 0 if the planet is green (inside tolerance or immune);
///   1–15 if the planet is red (outside tolerance), capped at 15 per
///   axis per the canonical formula.
///
/// See `docs/FORMULAS.md` F-4 for the full derivation and source.
///
/// # Panics
///
/// Does not panic. All arithmetic is bounded by the `EnvClick` range
/// (0–100) and the `HabAxis` validation at construction time.
#[must_use]
pub fn hab_value_one_axis(
    env_value: EnvClick,
    axis: &HabAxis,
    ideality_in: i32,
) -> (i32, i32, i32) {
    match axis {
        HabAxis::Immune => {
            // Immune axes contribute max points and never reduce ideality.
            (10000, ideality_in, 0)
        }
        HabAxis::Range { min, max } => {
            let hab_center = (min + max) / 2;
            let (hab_radius, tmp) = if hab_center > env_value {
                (hab_center - min, hab_center - env_value)
            } else {
                (max - hab_center, env_value - hab_center)
            };

            if env_value < *min || env_value > *max {
                // Planet is outside the tolerance range — hostile (red).
                let distance = if env_value < *min {
                    min - env_value
                } else {
                    env_value - max
                };
                let red = distance.min(15);
                (0, ideality_in, red)
            } else {
                // Planet is inside the tolerance range — green.
                // Guard against zero-width ranges (min == max == center).
                let from_ideal = if hab_radius == 0 {
                    100
                } else {
                    100 - (tmp * 100 / hab_radius)
                };
                let points = from_ideal * from_ideal;

                // Ideality penalty: when the planet is in the outer half
                // of the tolerance range on one side of center, reduce
                // the ideality scalar proportionally.
                let poor_planet_mod = (tmp * 2) - hab_radius;
                let ideality = if poor_planet_mod > 0 {
                    ideality_in * (hab_radius * 2 - poor_planet_mod) / (hab_radius * 2)
                } else {
                    ideality_in
                };

                (points, ideality, 0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── C.1 tests: hab_value_one_axis ─────────────────────────────

    #[test]
    fn immune_axis_returns_max_points() {
        let (pts, ideality, red) = hab_value_one_axis(50, &HabAxis::Immune, 10000);
        assert_eq!(pts, 10000);
        assert_eq!(ideality, 10000);
        assert_eq!(red, 0);
    }

    #[test]
    fn immune_axis_ignores_extreme_env_value() {
        // Even at env=0 or env=100, immune is always perfect.
        let (pts, _, red) = hab_value_one_axis(0, &HabAxis::Immune, 10000);
        assert_eq!(pts, 10000);
        assert_eq!(red, 0);
        let (pts, _, red) = hab_value_one_axis(100, &HabAxis::Immune, 10000);
        assert_eq!(pts, 10000);
        assert_eq!(red, 0);
    }

    #[test]
    fn centered_value_returns_max_points() {
        // Race tolerates 20-80, center=50. Planet at exactly 50.
        let axis = HabAxis::range(20, 80).unwrap();
        let (pts, ideality, red) = hab_value_one_axis(50, &axis, 10000);
        assert_eq!(pts, 10000); // fromIdeal = 100, 100^2 = 10000
        assert_eq!(ideality, 10000); // no penalty at center
        assert_eq!(red, 0);
    }

    #[test]
    fn edge_of_range_returns_zero_points() {
        // Race tolerates 20-80, center=50. Planet at exactly 20 (min edge).
        let axis = HabAxis::range(20, 80).unwrap();
        let (pts, _, red) = hab_value_one_axis(20, &axis, 10000);
        assert_eq!(pts, 0); // fromIdeal = 0, 0^2 = 0
        assert_eq!(red, 0); // still inside range
    }

    #[test]
    fn outside_range_returns_red_capped_at_15() {
        let axis = HabAxis::range(20, 80).unwrap();
        // 1 click outside → red = 1
        let (_, _, red) = hab_value_one_axis(19, &axis, 10000);
        assert_eq!(red, 1);
        // 15 clicks outside → red = 15
        let (_, _, red) = hab_value_one_axis(5, &axis, 10000);
        assert_eq!(red, 15);
        // 20 clicks outside → still capped at 15
        let (_, _, red) = hab_value_one_axis(0, &axis, 10000);
        assert_eq!(red, 15);
    }

    #[test]
    fn outside_range_high_side() {
        let axis = HabAxis::range(20, 80).unwrap();
        let (_, _, red) = hab_value_one_axis(81, &axis, 10000);
        assert_eq!(red, 1);
        let (_, _, red) = hab_value_one_axis(95, &axis, 10000);
        assert_eq!(red, 15);
        let (_, _, red) = hab_value_one_axis(100, &axis, 10000);
        assert_eq!(red, 15);
    }

    #[test]
    fn poor_planet_mod_reduces_ideality() {
        // Race tolerates 40-60, center=50, radius=10.
        // Planet at 55: tmp=5, poorPlanetMod = 10 - 10 = 0 → no penalty.
        let axis = HabAxis::range(40, 60).unwrap();
        let (_, ideality, _) = hab_value_one_axis(55, &axis, 10000);
        assert_eq!(ideality, 10000);

        // Planet at 58: tmp=8, poorPlanetMod = 16 - 10 = 6 → penalty.
        let (_, ideality, _) = hab_value_one_axis(58, &axis, 10000);
        assert!(
            ideality < 10000,
            "ideality should be reduced at 58, got {ideality}"
        );
    }

    #[test]
    fn zero_width_range_at_center() {
        // Edge case: min == max. Planet must be exactly at that value.
        let axis = HabAxis::range(50, 50).unwrap();
        let (pts, _, red) = hab_value_one_axis(50, &axis, 10000);
        assert_eq!(pts, 10000); // guard clause: hab_radius=0 → fromIdeal=100
        assert_eq!(red, 0);
        // One click off → red
        let (_, _, red) = hab_value_one_axis(51, &axis, 10000);
        assert_eq!(red, 1);
    }
}
