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

use crate::types::{EnvClick, Environment, HabAxis, HabRanges};

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

/// Calculate planet habitability for a given race (FR-4).
///
/// Returns an integer in the range **-45 to +100**:
/// - **100** = perfect homeworld (all three axes centered in tolerance)
/// - **0** = marginally habitable (at the edge of tolerance)
/// - **Negative** = hostile (population dies each turn; see FR-5)
/// - **-45** = maximally hostile (all three axes at max red distance)
///
/// The formula applies [`hab_value_one_axis`] to each of the three
/// environment axes, then combines the results:
/// - If any axis is red (outside tolerance), the total is the **negative
///   sum** of all red distances (capped at -45).
/// - If all axes are green (inside tolerance or immune), the total is
///   `floor(sqrt(sum_of_points / 3) + 0.9) * ideality / 10000`.
///
/// See `docs/FORMULAS.md` F-4 for the full derivation and source
/// (craig-stars `GetPlanetHabitability`, autohost wiki "Guts of
/// Habitability").
///
/// # Arguments
///
/// - `env` — the planet's environment reading (gravity, temperature,
///   radiation in 0–100 clicks).
/// - `hab_ranges` — the race's tolerance ranges for each axis.
#[must_use]
pub fn habitability(env: &Environment, hab_ranges: &HabRanges) -> i32 {
    let mut total_points: i32 = 0;
    let mut ideality: i32 = 10000;
    let mut total_red: i32 = 0;

    // Process each axis, threading ideality through.
    let axes: [(EnvClick, &HabAxis); 3] = [
        (env.gravity, &hab_ranges.gravity),
        (env.temperature, &hab_ranges.temperature),
        (env.radiation, &hab_ranges.radiation),
    ];

    for (env_val, axis) in axes {
        let (points, new_ideality, red) = hab_value_one_axis(env_val, axis, ideality);
        total_points += points;
        ideality = new_ideality;
        total_red += red;
    }

    if total_red > 0 {
        return -total_red;
    }

    // All axes green: combine via sqrt and ideality scalar.
    // f64 is safe cross-target (proven in Atom B).
    let points_f = f64::from(total_points);
    let raw = (points_f / 3.0).sqrt() + 0.9;

    // Truncate to integer, then scale by ideality.
    #[allow(clippy::cast_possible_truncation)]
    let base = raw as i32;
    base * ideality / 10000
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

    // ─── C.2 tests: habitability (combined) ────────────────────────

    /// Helper to build `HabRanges` from three `(min, max)` tuples.
    fn ranges(g: (i32, i32), t: (i32, i32), r: (i32, i32)) -> HabRanges {
        HabRanges {
            gravity: HabAxis::range(g.0, g.1).unwrap(),
            temperature: HabAxis::range(t.0, t.1).unwrap(),
            radiation: HabAxis::range(r.0, r.1).unwrap(),
        }
    }

    #[test]
    fn perfect_homeworld_returns_100() {
        // All three axes centered in a wide range.
        let env = Environment {
            gravity: 50,
            temperature: 50,
            radiation: 50,
        };
        let hab = ranges((0, 100), (0, 100), (0, 100));
        assert_eq!(habitability(&env, &hab), 100);
    }

    #[test]
    fn all_immune_returns_100() {
        let env = Environment {
            gravity: 0,
            temperature: 100,
            radiation: 50,
        };
        let hab = HabRanges {
            gravity: HabAxis::Immune,
            temperature: HabAxis::Immune,
            radiation: HabAxis::Immune,
        };
        assert_eq!(habitability(&env, &hab), 100);
    }

    #[test]
    fn single_axis_red_returns_negative() {
        // Gravity is fine, temperature is fine, radiation is way off.
        let env = Environment {
            gravity: 50,
            temperature: 50,
            radiation: 95,
        };
        let hab = ranges((0, 100), (0, 100), (40, 60));
        let result = habitability(&env, &hab);
        assert!(
            result < 0,
            "single red axis should produce negative hab, got {result}"
        );
    }

    #[test]
    fn triple_red_returns_minus_45() {
        // All three axes maximally hostile (15+ clicks outside each).
        let env = Environment {
            gravity: 0,
            temperature: 0,
            radiation: 0,
        };
        let hab = ranges((50, 80), (50, 80), (50, 80));
        let result = habitability(&env, &hab);
        assert_eq!(result, -45, "triple max-red should be -45, got {result}");
    }

    #[test]
    fn partial_hab_is_between_0_and_100() {
        // Planet is inside all ranges but not centered.
        let env = Environment {
            gravity: 30,
            temperature: 70,
            radiation: 50,
        };
        let hab = ranges((20, 80), (20, 80), (20, 80));
        let result = habitability(&env, &hab);
        assert!(
            result > 0 && result < 100,
            "partial hab should be 0 < h < 100, got {result}"
        );
    }

    #[test]
    fn immune_axis_boosts_partial_hab() {
        // Two normal axes (not centered) + one immune axis.
        let env = Environment {
            gravity: 30,
            temperature: 70,
            radiation: 50,
        };
        let hab_no_immune = ranges((20, 80), (20, 80), (20, 80));
        let hab_one_immune = HabRanges {
            gravity: HabAxis::range(20, 80).unwrap(),
            temperature: HabAxis::range(20, 80).unwrap(),
            radiation: HabAxis::Immune,
        };
        let normal = habitability(&env, &hab_no_immune);
        let boosted = habitability(&env, &hab_one_immune);
        assert!(
            boosted >= normal,
            "immune axis should help: normal={normal}, boosted={boosted}"
        );
    }

    #[test]
    fn habitability_is_deterministic() {
        let env = Environment {
            gravity: 42,
            temperature: 73,
            radiation: 15,
        };
        let hab = ranges((10, 90), (20, 80), (5, 50));
        let a = habitability(&env, &hab);
        let b = habitability(&env, &hab);
        assert_eq!(a, b);
    }
}
