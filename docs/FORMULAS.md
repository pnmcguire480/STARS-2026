---
id: FORMULAS
title: "Game formula derivations and citations"
status: stub — growing by atom
date: 2026-04-08
---

# FORMULAS.md

> **Purpose:** every game formula used by the STARS 2026 engine must be
> either cited to a public reference (`starsfaq.com`, `wiki.starsautohost.org`,
> or the 1995 canonical *Stars!* documentation) or explicitly marked as a
> STARS 2026 design decision with a reviewable rationale.
>
> **CLAUDE.md rule 8:** "Cite every game formula to starsfaq.com /
> wiki.starsautohost.org / docs/FORMULAS.md." This file is the backstop
> for formulas that don't have a clean upstream citation — each entry
> must name its origin (canon lift vs. council decision) and, if it's a
> council decision, the reasoning.

---

## Conventions

Each entry follows this shape:

- **Formula ID** — stable identifier referenced from code comments.
- **Applies to** — the code symbol(s) that implement this formula.
- **Source** — one of:
  - **CANON** — lifted directly from a cited public reference.
  - **COUNCIL** — a STARS 2026 design decision made by an agent
    council, pending a canon cross-check or derivation.
  - **PENDING** — placeholder; a council must revisit before the code
    ships to v0.1.
- **Value** — the actual numeric constants or closed-form expression.
- **Rationale / citation** — URL, quote, or council summary.
- **Cross-check by** — the atom or review pass that will promote this
  entry from COUNCIL → CANON (or reject it).

---

## Entries

### F-1 — `min_star_distance` (per-density star spacing)

- **Applies to:** `engine/src/galaxy.rs::min_star_distance`
- **Source:** **COUNCIL** (Atom 2, Game Design + Performance Engineer)
- **Value:**

  | Density | Minimum spacing (light-years) |
  |---|---|
  | Sparse  | 30 |
  | Normal  | 25 |
  | Dense   | 20 |
  | Packed  | 15 |

- **Rationale:** The Game Design council noted that *Stars!* 1995 canon
  places individual stars at roughly 3–8 light-years apart on its
  internal grid. STARS 2026 uses larger map dimensions (per
  `GalaxySize::map_dimension`) and the raw 3–8 values would leave stars
  visually touching at the browser zoom levels SPEC.md mandates. The
  council authorized scaling the canon values up modestly to preserve
  the relative ordering across density tiers while keeping the
  rejection sampler tractable for Huge+Packed galaxies (the saturation
  case — see `engine/tests/fr1_galaxy.rs` and Atom A.11).
- **Cross-check by:** a future "formulas audit" atom before v0.1 ships.
  The Crucible P1-4 finding (Devil's Advocate, 2026-04-08) flagged the
  lack of derivation citation and pushed for either a canon reference
  or a documented design decision; this entry is the documented design
  decision. If a canon reference surfaces in the starsfaq.com or
  autohost wiki sources, this entry is promoted to CANON.

### F-2 — `GalaxySize::target_stars` (base star count per size)

- **Applies to:** `engine/src/types.rs::GalaxySize::target_stars`
- **Source:** Mixed.
  - **CANON** for Tiny = 24 (SPEC D-2, Stars! 1995 reference).
  - **COUNCIL** for Small = 70, Medium = 150, Large = 300, Huge = 600
    (Atom 1, `types.rs` Phase 1 Task 1 council).
- **Rationale:** Tiny was amended from a non-canonical 32 draft to the
  canon value of 24 in Atom A.1 (commit `860ead4`). The other sizes
  retain their Phase 1 council values pending a canon cross-check.
- **Cross-check by:** tech / content council when `tech.rs` and
  `planet.rs` integration lands; the actual *Stars!* 1995 values for
  Small/Medium/Large/Huge should be verified against `starsfaq.com`
  before v0.1 ships.

### F-3 — Jitter shape for `actual_star_count`

- **Applies to:** `engine/src/galaxy.rs::actual_star_count`
- **Source:** **COUNCIL** (Atom 2.9 P0 fix + Atom A.4 re-derivation)
- **Value:** asymmetric `[0%, +20%]` jitter applied to
  `base * density_scale / 100`, where `density_scale ∈ {75, 100, 130, 160}`
  for `{Sparse, Normal, Dense, Packed}` respectively.
- **Rationale:** The closing Crucible for Atom 2 flagged an earlier
  symmetric `[-10, +10]%` jitter as a SPEC FR-1 floor violation. The
  asymmetric form guarantees the generator never returns fewer stars
  than `base * density_scale / 100`, which for Tiny+Normal bottoms out
  at the SPEC floor on every seed. The specific `+20%` ceiling was
  chosen so the Tiny+Packed upper bound sits comfortably below the
  SPEC FR-1 ceiling of 100 stars.
- **Cross-check by:** none pending — this is a STARS 2026 design
  decision, not a canon formula. The jitter shape has no *Stars!* 1995
  equivalent; the original game used fixed counts with no per-seed
  variation.

---

### F-4 — Habitability formula (FR-4)

- **Applies to:** `engine/src/planet.rs::habitability` (Atom C)
- **Source:** **CANON** — reverse-engineered from Stars! 1995, verified
  against `craig-stars/cs/race.go::GetPlanetHabitability` and the
  autohost wiki "Guts of the Habitability Formula" article.
- **Algorithm:**

  **Per-axis (gravity, temperature, radiation):**

  If the race is **immune** to this axis: contribute a fixed **10000**
  points to `planetValuePoints` and leave `ideality` unchanged.

  If the planet value falls **within** the race's `[habLow, habHigh]`:
  ```
  habCenter = (habLow + habHigh) / 2
  habRadius = (habCenter > habValue) ? (habCenter - habLow) : (habHigh - habCenter)
  tmp       = abs(habValue - habCenter)
  fromIdeal = 100 - (tmp * 100 / habRadius)
  planetValuePoints += fromIdeal * fromIdeal    // max 10000 per axis
  poorPlanetMod = (tmp * 2) - habRadius
  if poorPlanetMod > 0:
      ideality = ideality * (habRadius * 2 - poorPlanetMod) / (habRadius * 2)
  ```

  If the planet value falls **outside** `[habLow, habHigh]` (hostile):
  ```
  habRed = distance from nearest edge of [habLow, habHigh]
  habRed = min(habRed, 15)     // capped at 15 per axis
  redValue += habRed
  ```

  **Combining axes:**
  ```
  if redValue != 0:
      return -redValue           // range: -1 to -45 (3 axes × 15)
  result = floor(sqrt(planetValuePoints / 3.0) + 0.9)
  result = result * ideality / 10000
  return result                  // range: 0 to 100
  ```

  `ideality` starts at 10000 (100%) and is reduced per green axis when
  the planet is in the outer half of the tolerance range (the
  `poorPlanetMod` penalty). Immune axes always contribute max points
  and never reduce ideality.

- **Output range:** -45 (maximally hostile, all 3 axes at max red) to
  +100 (perfect homeworld, all 3 axes centered).
- **Cross-check by:** Atom C test suite pins against known
  craig-stars reference values.

### F-5 — Population growth formula (FR-5)

- **Applies to:** `engine/src/planet.rs::population_growth` (Atom C)
- **Source:** **CANON** — reverse-engineered from Stars! 1995, verified
  against `craig-stars/cs/planet.go::GetGrowthAmount` and the autohost
  wiki "Guts of population growth" article (Jason Cawley / Bill Butler).
- **Algorithm:**

  **Max population (planet capacity):**
  ```
  maxPop = floor_to_100(1_000_000 * max(habValue, 5) / 100)
  ```
  A 100% hab world holds 1M; hab floored at 5% for capacity calc.

  **Normal growth (hab > 0, pop ≤ maxPop):**
  ```
  popGrowth = pop * growthRate * habValue / 10000
  ```
  `growthRate` is the race's chosen rate (integer, e.g. 15 for 15%).

  **Crowding penalty (pop > 25% of maxPop):**
  ```
  capacityRatio = pop / maxPop
  if capacityRatio > 0.25:
      crowdingFactor = (16/9) * (1 - capacityRatio)^2
      popGrowth = popGrowth * crowdingFactor
  ```
  At 25% capacity: factor = 1.0 (no penalty). At 100%: factor = 0.

  **Hostile worlds (hab < 0):**
  ```
  deathAmount = pop * habValue / 1000    // habValue is negative
  ```
  E.g. -10 hab kills ~1% per turn; -45 hab kills ~4.5% per turn.

  **Overcrowding (pop > maxPop):**
  ```
  dieoffPercent = clamp((capacityRatio - 1) * 0.04, 0, 0.12)
  deathAmount = pop * -dieoffPercent
  ```
  At 200% capacity: 4% die/turn. Caps at 12% die/turn.

  **Rounding:** STARS 2026 uses **truncation (floor)** per Patrick
  decision 2026-04-09. This is a deliberate deviation from
  craig-stars which uses round-half-up. `maxPop` is floored to the
  nearest 100 (matching the `Colonists(u32)` granularity).

- **Cross-check by:** Atom C test suite pins against known
  craig-stars reference values (adjusted for truncation rounding).

### F-6 — Resource generation formula (FR-6)

- **Applies to:** `engine/src/planet.rs::resource_output` (Atom D)
- **Source:** **CANON** — sourced from `craig-stars/cs/planet.go`
  (`ComputeResourcesPerYear`) and `craig-stars/cs/race.go` (`NewRace`).
- **Algorithm:**

  ```
  colonist_resources = population / (pop_efficiency * 100)
  max_operable       = num_factories_per_10k * population / 10000
  operable           = min(built_factories, max_operable)
  factory_resources  = ceil(operable * factory_output / 10)
  total_resources    = colonist_resources + factory_resources
  ```

  **Default race constants:**
  - `pop_efficiency` = 10 (1 resource per 1,000 colonists)
  - `factory_output` = 10 (each factory produces 10/10 = 1 resource/yr)
  - `num_factories_per_10k` = 10 (operable per 10,000 colonists)

  Race design allows customizing `factory_output` (1–15),
  `num_factories_per_10k` (5–25), and `factory_cost` (5–25). AR races
  skip factories entirely (not in v0.1 scope).

  **Note:** hab% does NOT directly multiply resource output (SPEC FR-6
  amended 2026-04-09 to match canon). Hab affects resources indirectly
  via population growth rate and planet capacity.

- **Rounding:** `colonist_resources` truncates (integer division).
  `factory_resources` uses ceiling (`ceil`).
- **Cross-check by:** Atom D test suite.

### F-7 — Mineral extraction formula (FR-7)

- **Applies to:** `engine/src/planet.rs::mineral_extraction` (Atom D)
- **Source:** **CANON** — sourced from `craig-stars/cs/planet.go`
  (`getMineralOutput`) and `craig-stars/cs/rules.go`.
- **Algorithm:**

  **Extraction per mineral type per turn:**
  ```
  max_operable = num_mines_per_10k * population / 10000
  operable     = min(built_mines, max_operable)
  output       = concentration * operable * mine_output / 1000
  ```

  **Default race constants:**
  - `mine_output` = 10 (at concentration 100: each mine yields 1 kT/yr)
  - `num_mines_per_10k` = 10 (operable per 10,000 colonists)

  **Concentration depletion** (cumulative mine-years model):
  ```
  mine_years_to_rollover = mineral_decay_factor / (concentration^2)
  mineral_decay_factor   = 1,500,000
  ```
  When accumulated `mine_years > mine_years_to_rollover`, concentration
  drops by `floor(mine_years / mine_years_to_rollover)`. Minimum
  concentration = **1** (never reaches 0). Homeworld minimum = **30**.

  **Note:** mine-years accumulation is turn-engine state. Atom D
  implements the per-turn extraction formula and the depletion threshold
  calculation as pure functions; the actual mine-years tracking lands
  when the turn engine (FR-15) ships.

- **Rounding:** extraction output truncates (integer division).
- **Cross-check by:** Atom D test suite.

## Entries pending (no stub yet)

- **F-8** — `GalaxySize::map_dimension` (map size in light-years per
  galaxy size) — Phase 1 Task 1 council value, pending canon cross-check.
- **F-9** — Tactical combat resolution order — FR-14, pending.

---

## How to add an entry

1. Pick the next free ID (F-N).
2. Fill in every field. **"TBD" is not an acceptable value for Source
   or Rationale** — if you don't know, the entry is not ready to ship.
3. Reference the entry from a Rust doc-comment in the implementing
   file: `/// See docs/FORMULAS.md entry F-N.`
4. If the entry is COUNCIL, open a follow-up ticket (in `BACKLOG.md`)
   to cross-check against canon before v0.1.
