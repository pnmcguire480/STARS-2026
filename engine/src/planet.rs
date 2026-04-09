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

/// Maximum population a planet can sustain at a given habitability.
///
/// Returns `Colonists` where `units()` = hundreds of people. A 100%
/// hab world holds 1,000,000 people = `Colonists(10000)`. Hab is
/// floored at 5% for this calculation (even a 1% green world can
/// hold 50,000 people). Negative or zero hab returns 0 capacity.
///
/// The result is floored to the nearest 100 people (matching the
/// `Colonists` granularity). See `docs/FORMULAS.md` F-5.
///
/// # Arguments
///
/// - `hab_pct` — the planet's habitability for the owner race (output
///   of [`habitability`]).
#[must_use]
pub fn max_population(hab_pct: i32) -> crate::types::Colonists {
    use crate::types::Colonists;

    if hab_pct <= 0 {
        return Colonists::new(0);
    }

    // Floor hab at 5% for capacity calculation per canon.
    let effective_hab = hab_pct.max(5);

    // 1,000,000 people at 100% hab = Colonists(10000).
    // At effective_hab%: 10000 * effective_hab / 100.
    // Integer division floors to nearest 100 people automatically.
    #[allow(clippy::cast_sign_loss)]
    let units = (10000_i32 * effective_hab / 100) as u32;
    Colonists::new(units)
}

/// Population growth for one turn (FR-5).
///
/// Returns the **signed delta** in `Colonists` units (hundreds of
/// people). Positive = growth, negative = deaths.
///
/// # Cases
///
/// 1. **Normal growth (hab > 0, pop ≤ capacity):**
///    `growth = pop * growth_rate * hab_pct / 10000`, with crowding
///    penalty applied when pop > 25% of capacity.
/// 2. **Crowding penalty (pop > 25% of capacity):**
///    `factor = (16/9) * (1 - ratio)^2` where ratio = pop/capacity.
///    Uses f64 (safe cross-target per Atom B). At 100% capacity,
///    factor = 0 (no growth). At 25% capacity, factor = 1.0.
/// 3. **Hostile worlds (hab < 0):**
///    `deaths = pop * hab_pct / 1000` (`hab_pct` is negative, so this
///    returns a negative delta). E.g. -10 hab ≈ 1% death per turn.
/// 4. **Overcrowding (pop > capacity):**
///    `dieoff = clamp((ratio - 1) * 0.04, 0, 0.12)`, then
///    `deaths = -pop * dieoff`. Caps at 12% death per turn.
///
/// **Rounding:** truncation (floor toward zero) per Patrick decision
/// 2026-04-09, matching Stars! 1995 canon. See `docs/FORMULAS.md` F-5.
///
/// # Arguments
///
/// - `pop` — current planet population.
/// - `hab_pct` — habitability for the owner race (-45 to +100).
/// - `growth_rate` — the race's chosen growth rate (integer, e.g. 15
///   for 15%). Set during race creation.
#[must_use]
#[allow(clippy::cast_precision_loss)] // pop units ≤ ~100_000 (10M people / 100), well within f64 mantissa
pub fn population_growth(pop: crate::types::Colonists, hab_pct: i32, growth_rate: u32) -> i64 {
    let pop_units = i64::from(pop.units());

    if pop_units == 0 {
        return 0;
    }

    // ── Hostile worlds: population dies ──────────────────────────
    if hab_pct < 0 {
        // deaths = pop * hab_pct / 1000 (hab_pct is negative → negative result)
        return pop_units * i64::from(hab_pct) / 1000;
    }

    let max_pop = max_population(hab_pct);
    let max_units = i64::from(max_pop.units());

    // Guard: if max_pop is 0 (hab_pct == 0), no growth possible.
    if max_units == 0 {
        return 0;
    }

    // ── Overcrowding: population dies ───────────────────────────
    if pop_units > max_units {
        let ratio = pop_units as f64 / max_units as f64;
        let dieoff = ((ratio - 1.0) * 0.04).clamp(0.0, 0.12);
        // Negative delta (deaths). Truncate toward zero.
        #[allow(clippy::cast_possible_truncation)]
        let deaths = -(pop_units as f64 * dieoff) as i64;
        return deaths;
    }

    // ── Normal growth ───────────────────────────────────────────
    let mut growth = pop_units * i64::from(growth_rate) * i64::from(hab_pct) / 10000;

    // ── Crowding penalty (above 25% capacity) ───────────────────
    let ratio = pop_units as f64 / max_units as f64;
    if ratio > 0.25 {
        let crowding_factor = (16.0 / 9.0) * (1.0 - ratio).powi(2);
        #[allow(clippy::cast_possible_truncation)]
        let crowded = (growth as f64 * crowding_factor) as i64;
        growth = crowded;
    }

    growth
}

/// Resources generated per turn on a planet (FR-6).
///
/// Returns the total resource output as an integer. Resources come
/// from two sources: colonist labor and factory output.
///
/// ```text
/// colonist_resources = population / (pop_efficiency * 100)
/// max_operable       = num_factories_per_10k * population / 10000
/// operable           = min(built_factories, max_operable)
/// factory_resources  = ceil(operable * factory_output / 10)
/// total              = colonist_resources + factory_resources
/// ```
///
/// See `docs/FORMULAS.md` F-6 for the full derivation.
///
/// # Arguments
///
/// - `pop` — current planet population.
/// - `factories` — number of factories built on the planet.
/// - `pop_efficiency` — colonists per resource (default 10 = 1 resource
///   per 1,000 people). Race-configurable.
/// - `factory_output` — output per factory in tenths (default 10 = 1
///   resource per factory). Race-configurable (1–15).
/// - `num_factories_per_10k` — max operable factories per 10,000
///   colonists (default 10). Race-configurable (5–25).
#[must_use]
pub fn resource_output(
    pop: crate::types::Colonists,
    factories: u32,
    pop_efficiency: u32,
    factory_output: u32,
    num_factories_per_10k: u32,
) -> u32 {
    let pop_units = pop.units();

    // Colonist resources: pop_units is in hundreds, so population =
    // pop_units * 100. Resources = population / (pop_efficiency * 100)
    // = pop_units * 100 / (pop_efficiency * 100) = pop_units / pop_efficiency.
    let colonist_resources = if pop_efficiency == 0 {
        0
    } else {
        pop_units / pop_efficiency
    };

    // Max operable factories: num_factories_per_10k per 10,000 colonists.
    // population = pop_units * 100, so max_operable =
    // num_factories_per_10k * pop_units * 100 / 10000
    // = num_factories_per_10k * pop_units / 100.
    let max_operable = num_factories_per_10k * pop_units / 100;
    let operable = factories.min(max_operable);

    // Factory resources: ceil(operable * factory_output / 10).
    let factory_resources = (operable * factory_output).div_ceil(10);

    colonist_resources + factory_resources
}

/// Minerals extracted per turn for a single mineral type (FR-7).
///
/// Returns the kT of minerals extracted. Does NOT modify concentration
/// (depletion is a separate concern tracked via mine-years in the turn
/// engine).
///
/// ```text
/// max_operable = num_mines_per_10k * population / 10000
/// operable     = min(built_mines, max_operable)
/// output       = concentration * operable * mine_output / 1000
/// ```
///
/// See `docs/FORMULAS.md` F-7 for the full derivation.
///
/// # Arguments
///
/// - `pop` — current planet population.
/// - `mines` — number of mines built on the planet.
/// - `concentration` — current mineral concentration (1–100).
/// - `mine_output` — output per mine per year in thousandths (default
///   10, so at concentration 100: 100 × 1 × 10/1000 = 1 kT/mine/yr).
///   Race-configurable.
/// - `num_mines_per_10k` — max operable mines per 10,000 colonists
///   (default 10). Race-configurable.
#[must_use]
pub fn mineral_extraction(
    pop: crate::types::Colonists,
    mines: u32,
    concentration: u32,
    mine_output: u32,
    num_mines_per_10k: u32,
) -> u32 {
    let pop_units = pop.units();

    // Max operable mines: same formula as factories.
    let max_operable = num_mines_per_10k * pop_units / 100;
    let operable = mines.min(max_operable);

    // Output: concentration * operable * mine_output / 1000.
    concentration * operable * mine_output / 1000
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

    // ─── C.3 tests: max_population ─────────────────────────────────

    #[test]
    fn max_pop_100_percent_is_one_million() {
        let cap = max_population(100);
        assert_eq!(cap.units(), 10000); // 10000 * 100 people = 1,000,000
    }

    #[test]
    fn max_pop_50_percent_is_500k() {
        let cap = max_population(50);
        assert_eq!(cap.units(), 5000);
    }

    #[test]
    fn max_pop_negative_hab_is_zero() {
        assert_eq!(max_population(-10).units(), 0);
        assert_eq!(max_population(-45).units(), 0);
    }

    #[test]
    fn max_pop_zero_hab_is_zero() {
        assert_eq!(max_population(0).units(), 0);
    }

    #[test]
    fn max_pop_low_hab_floored_at_5_percent() {
        // 1% and 4% both floor to 5% for capacity: 10000 * 5 / 100 = 500
        assert_eq!(max_population(1).units(), 500);
        assert_eq!(max_population(4).units(), 500);
        assert_eq!(max_population(5).units(), 500);
    }

    // ─── C.4 + C.5 tests: population_growth ────────────────────────

    use crate::types::Colonists;

    #[test]
    fn zero_pop_zero_growth() {
        assert_eq!(population_growth(Colonists::new(0), 100, 15), 0);
    }

    #[test]
    fn normal_growth_small_pop() {
        // 1000 people (units=10), 100% hab, 15% growth rate.
        // growth = 10 * 15 * 100 / 10000 = 1 (truncated).
        let delta = population_growth(Colonists::new(10), 100, 15);
        assert_eq!(delta, 1);
    }

    #[test]
    fn normal_growth_larger_pop() {
        // 100,000 people (units=1000), 80% hab, 15% growth.
        // uncrowded = 1000 * 15 * 80 / 10000 = 120.
        // capacity = max_pop(80) = 8000. ratio = 1000/8000 = 0.125 < 0.25.
        // No crowding penalty. Growth = 120.
        let delta = population_growth(Colonists::new(1000), 80, 15);
        assert_eq!(delta, 120);
    }

    #[test]
    fn crowding_reduces_growth() {
        // 500,000 people (units=5000), 100% hab, 15% growth.
        // uncrowded = 5000 * 15 * 100 / 10000 = 750.
        // capacity = 10000. ratio = 0.5 > 0.25 → crowding.
        // factor = (16/9) * (1 - 0.5)^2 = (16/9) * 0.25 = 0.4444
        // growth = 750 * 0.4444 = 333 (truncated).
        let delta = population_growth(Colonists::new(5000), 100, 15);
        assert!(
            delta > 0 && delta < 750,
            "crowded growth should be positive but less than uncrowded 750, got {delta}"
        );
    }

    #[test]
    fn at_capacity_zero_growth() {
        // 1,000,000 people (units=10000), 100% hab, 15% growth.
        // ratio = 1.0 → crowding factor = (16/9) * 0 = 0. Growth = 0.
        let delta = population_growth(Colonists::new(10000), 100, 15);
        assert_eq!(delta, 0);
    }

    #[test]
    fn hostile_world_kills_population() {
        // 100,000 people (units=1000), -10 hab.
        // deaths = 1000 * -10 / 1000 = -10.
        let delta = population_growth(Colonists::new(1000), -10, 15);
        assert_eq!(delta, -10);
    }

    #[test]
    fn max_hostile_kills_fast() {
        // -45 hab: deaths = 1000 * -45 / 1000 = -45.
        let delta = population_growth(Colonists::new(1000), -45, 15);
        assert_eq!(delta, -45);
    }

    #[test]
    fn overcrowding_kills_proportionally() {
        // 2,000,000 people (units=20000) on a 100% hab world (cap=10000).
        // ratio = 2.0. dieoff = (2.0-1.0)*0.04 = 0.04.
        // deaths = -(20000 * 0.04) = -800.
        let delta = population_growth(Colonists::new(20000), 100, 15);
        assert!(
            delta < 0,
            "overcrowded planet should have negative growth, got {delta}"
        );
        assert!(
            delta >= -2400, // max 12% of 20000 = 2400
            "overcrowding death should be capped, got {delta}"
        );
    }

    #[test]
    fn overcrowding_death_capped_at_12_percent() {
        // Massively overcrowded: 5x capacity.
        // dieoff = (5.0-1.0)*0.04 = 0.16 → clamped to 0.12.
        // deaths = -(50000 * 0.12) = -6000.
        let delta = population_growth(Colonists::new(50000), 100, 15);
        #[allow(clippy::cast_possible_truncation)]
        let max_death = -(50000.0 * 0.12) as i64;
        assert_eq!(delta, max_death);
    }

    #[test]
    fn growth_is_deterministic() {
        let a = population_growth(Colonists::new(3000), 75, 15);
        let b = population_growth(Colonists::new(3000), 75, 15);
        assert_eq!(a, b);
    }

    // ─── D.1 tests: resource_output ────────────────────────────────

    #[test]
    fn resource_output_empty_planet() {
        assert_eq!(resource_output(Colonists::new(0), 100, 10, 10, 10), 0);
    }

    #[test]
    fn resource_output_colonists_only_no_factories() {
        // 100,000 people (units=1000), no factories.
        // colonist_resources = 1000 / 10 = 100.
        assert_eq!(resource_output(Colonists::new(1000), 0, 10, 10, 10), 100);
    }

    #[test]
    fn resource_output_default_race_constants() {
        // 100,000 people (units=1000), 50 factories.
        // colonist = 1000 / 10 = 100.
        // max_operable = 10 * 1000 / 100 = 100. operable = min(50, 100) = 50.
        // factory = ceil(50 * 10 / 10) = 50.
        // total = 100 + 50 = 150.
        assert_eq!(resource_output(Colonists::new(1000), 50, 10, 10, 10), 150);
    }

    #[test]
    fn resource_output_factory_limit_applies() {
        // 10,000 people (units=100), 200 factories.
        // colonist = 100 / 10 = 10.
        // max_operable = 10 * 100 / 100 = 10. operable = min(200, 10) = 10.
        // factory = ceil(10 * 10 / 10) = 10.
        // total = 10 + 10 = 20.
        assert_eq!(resource_output(Colonists::new(100), 200, 10, 10, 10), 20);
    }

    #[test]
    fn resource_output_high_factory_output() {
        // 100,000 people (units=1000), 50 factories, factory_output=15.
        // colonist = 1000 / 10 = 100.
        // max_operable = 10 * 1000 / 100 = 100. operable = 50.
        // factory = ceil(50 * 15 / 10) = ceil(75) = 75.
        // total = 175.
        assert_eq!(resource_output(Colonists::new(1000), 50, 10, 15, 10), 175);
    }

    #[test]
    fn resource_output_factory_ceil_rounding() {
        // 10,000 people (units=100), 3 factories, factory_output=10.
        // factory = ceil(3 * 10 / 10) = ceil(3.0) = 3.
        // 7 factories: ceil(7 * 10 / 10) = 7.
        assert_eq!(resource_output(Colonists::new(100), 3, 10, 10, 10), 13);
        // Now with factory_output=7: ceil(3 * 7 / 10) = ceil(2.1) = 3.
        assert_eq!(resource_output(Colonists::new(100), 3, 10, 7, 10), 13);
    }

    // ─── D.2 tests: mineral_extraction ─────────────────────────────

    #[test]
    fn mineral_extraction_empty_planet() {
        assert_eq!(mineral_extraction(Colonists::new(0), 10, 50, 10, 10), 0);
    }

    #[test]
    fn mineral_extraction_no_mines() {
        assert_eq!(mineral_extraction(Colonists::new(1000), 0, 50, 10, 10), 0);
    }

    #[test]
    fn mineral_extraction_default_race_at_full_concentration() {
        // 100,000 people (units=1000), 100 mines, concentration=100.
        // max_operable = 10 * 1000 / 100 = 100. operable = 100.
        // output = 100 * 100 * 10 / 1000 = 100.
        assert_eq!(
            mineral_extraction(Colonists::new(1000), 100, 100, 10, 10),
            100
        );
    }

    #[test]
    fn mineral_extraction_half_concentration() {
        // Same as above but concentration=50.
        // output = 50 * 100 * 10 / 1000 = 50.
        assert_eq!(
            mineral_extraction(Colonists::new(1000), 100, 50, 10, 10),
            50
        );
    }

    #[test]
    fn mineral_extraction_mine_limit_applies() {
        // 10,000 people (units=100), 200 mines, concentration=100.
        // max_operable = 10 * 100 / 100 = 10. operable = 10.
        // output = 100 * 10 * 10 / 1000 = 10.
        assert_eq!(
            mineral_extraction(Colonists::new(100), 200, 100, 10, 10),
            10
        );
    }

    #[test]
    fn mineral_extraction_low_concentration() {
        // concentration=1 (minimum floor). 100 operable mines.
        // output = 1 * 100 * 10 / 1000 = 1.
        assert_eq!(mineral_extraction(Colonists::new(1000), 100, 1, 10, 10), 1);
    }

    #[test]
    fn mineral_extraction_deterministic() {
        let a = mineral_extraction(Colonists::new(500), 50, 75, 10, 10);
        let b = mineral_extraction(Colonists::new(500), 50, 75, 10, 10);
        assert_eq!(a, b);
    }
}
