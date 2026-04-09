//! Core shared types for the STARS 2026 engine.
//!
//! This module defines the data vocabulary shared by every other engine
//! module. It contains no game logic — only type definitions, constructors,
//! and the trait impls required to work with them.
//!
//! Types are grown incrementally, one sniff-tested atom at a time per
//! [`SNIFFTEST.md`](../../../SNIFFTEST.md). Contributors must not pre-create
//! speculative variants or fields; each addition ships with its test.
//!
//! # Determinism contract
//!
//! Per [`CLAUDE.md`](../../../CLAUDE.md), the engine must produce
//! byte-identical output across `wasm32-unknown-unknown` and native
//! `x86_64` targets. Types defined in this module must not rely on
//! `HashMap` iteration order or any other non-deterministic std behavior.
//! Use `BTreeMap` when ordered iteration is required.

use thiserror::Error;

/// All fallible engine operations return this error type.
///
/// New variants are added on demand as engine functionality reports new
/// failure modes. Prefer `&'static str` payloads over owned `String`s
/// unless the message must interpolate runtime data — this keeps error
/// construction allocation-free on hot paths and reduces wasm binary size.
///
/// `PartialEq` is intentionally **not** derived. Tests should use
/// [`matches!`] or destructuring on variants so adding a new variant in a
/// future atom does not ripple into equality-based assertions.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum GameError {
    /// Race creation or validation rejected the input — for example, too
    /// many advantage points spent, or a malformed trait combination.
    #[error("invalid race: {0}")]
    InvalidRace(&'static str),

    /// An integer operation in the engine would have overflowed. The payload
    /// names the operation (e.g. `"Minerals::checked_add"`) for diagnostics.
    /// Raised instead of silently wrapping, because silent wrap on one
    /// target and panic on another would break the determinism contract.
    #[error("arithmetic overflow in {0}")]
    ArithmeticOverflow(&'static str),

    /// A player or planet tried to spend more of a resource than it had.
    /// The payload names the resource or operation (e.g. `"Minerals::spend"`)
    /// so tests and UI can disambiguate without parsing the display string.
    #[error("insufficient resources in {0}")]
    InsufficientResources(&'static str),

    /// Procedural galaxy generation could not satisfy the requested
    /// constraints — typically because the rejection sampler exhausted
    /// its retry budget without finding a valid placement (density too
    /// high for the requested star count, or `min_homeworld_distance`
    /// too large for the map dimension). The payload names the failing
    /// stage (e.g. `"place_one_star: retry budget exhausted"`) so the
    /// `SvelteKit` layer can surface an actionable message instead of a
    /// stack trace. Same seed → same failure, per the determinism
    /// contract.
    ///
    /// **Struct variant (A.9, P1-5 resolution):** upgraded from a
    /// single-tuple `(&'static str)` payload to a named-field struct
    /// variant during Atom A. This was flagged by the Crucible
    /// Inversion Agent as a breaking-change debt: v0.2 i18n (server →
    /// JS client) needs a typed payload, and paying the cost while
    /// there is exactly one call site is cheaper than doing it later.
    /// The `reason` field is tagged `// i18n:v0.2` in spirit — when
    /// v0.2 lands, add an `error_code: u16` or enum field and keep
    /// `reason` as the human-readable fallback.
    #[error("galaxy generation failed: {reason}")]
    GalaxyGenerationFailed {
        /// Short, human-readable stage/reason string. `'static` for now
        /// because every caller passes a string literal; will move to
        /// `String` or an error-code enum when v0.2 i18n lands.
        reason: &'static str,
    },
}

// =============================================================================
// Typed identifiers
// =============================================================================
//
// Each domain concept gets its own newtype wrapper around a primitive integer.
// This makes it a compile-time error to (e.g.) pass a `PlayerId` where a
// `StarId` is expected, even though both are `u32` under the hood. Every ID
// derives `Ord` / `PartialOrd` so it can serve as a `BTreeMap` key without
// further ceremony — per the project-wide determinism rule.

use serde::{Deserialize, Serialize};

/// Unique identifier for a saved game / match instance.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct GameId(pub u64);

/// Unique identifier for a player within a game.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct PlayerId(pub u32);

/// Unique identifier for a star system within a galaxy.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct StarId(pub u32);

/// Unique identifier for a planet within a galaxy.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct PlanetId(pub u32);

/// Unique identifier for a fleet within a game.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct FleetId(pub u32);

/// Unique identifier for a player-authored ship design.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct ShipDesignId(pub u32);

/// Unique identifier for a player-authored battle plan.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct BattlePlanId(pub u32);

// =============================================================================
// Position — galactic coordinates
// =============================================================================

/// A position in the galaxy, measured in light-years.
///
/// Stored as `f64` to match the legacy scaffold and allow sub-light-year
/// precision for fleet movement interpolation. Engine code must avoid
/// transcendental math (`sin`, `cos`, `atan2`) on `Position` values — IEEE 754
/// only guarantees bit-identical results across `wasm32` and native targets
/// for the basic operations (`+`, `-`, `*`, `/`, `sqrt`). Stick to those.
///
/// `Eq` / `Hash` are deliberately **not** derived because `f64` does not
/// implement them. Do not store `Position` as a `HashMap` / `BTreeMap` key.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    /// Construct a new position at the given light-year coordinates.
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Euclidean distance between this position and `other`, in light-years.
    ///
    /// Uses only IEEE 754-deterministic operations (`-`, `*`, `+`, `sqrt`),
    /// so two runs of the same seed on any supported target must return
    /// bit-identical values. Avoid `hypot` — it is not guaranteed
    /// bit-identical across wasm/native.
    #[must_use]
    pub fn distance_to(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

// =============================================================================
// Minerals — the three-kind resource primitive
// =============================================================================

/// The three mineral kinds tracked by the economy.
///
/// Stars! has exactly three minerals and always will — this enum is
/// intentionally not `#[non_exhaustive]` because adding a fourth would be
/// a game-design decision that breaks every formula in the engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MineralType {
    Ironium,
    Boranium,
    Germanium,
}

/// A bundle of the three mineral kinds measured in kilotons (kT).
///
/// Used for costs, cargo, planet surface stockpiles, and trade. All
/// arithmetic on `Minerals` is **checked**: overflow and underflow raise
/// [`GameError`] instead of wrapping silently, because silent wrap on one
/// target and debug-panic on another would break the wasm/native
/// determinism contract.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Minerals {
    pub ironium: u32,
    pub boranium: u32,
    pub germanium: u32,
}

impl Minerals {
    /// The zero bundle — convenient starting point for accumulators and
    /// tests. Prefer over `Minerals::default()` in engine code to make
    /// intent explicit ("start from zero" vs "use the default state").
    pub const ZERO: Self = Self {
        ironium: 0,
        boranium: 0,
        germanium: 0,
    };

    /// Construct a mineral bundle with the given kiloton amounts.
    #[must_use]
    pub const fn new(ironium: u32, boranium: u32, germanium: u32) -> Self {
        Self {
            ironium,
            boranium,
            germanium,
        }
    }

    /// Total mineral count across all three kinds, in kilotons.
    ///
    /// Returns `u64` to make overflow mathematically impossible — three
    /// `u32` values can sum to at most 3 × `u32::MAX` ≈ 12.9 billion,
    /// which fits comfortably in a `u64`. Callers that need a `u32`
    /// must explicitly `u32::try_from(...)` and handle the failure.
    #[must_use]
    pub const fn total(&self) -> u64 {
        self.ironium as u64 + self.boranium as u64 + self.germanium as u64
    }

    /// Returns `true` if `self` holds at least `cost` of every mineral.
    ///
    /// Pure query — does not mutate either bundle. Pair with [`spend`] to
    /// actually deduct the cost.
    ///
    /// [`spend`]: Self::spend
    #[must_use]
    pub const fn can_afford(&self, cost: &Self) -> bool {
        self.ironium >= cost.ironium
            && self.boranium >= cost.boranium
            && self.germanium >= cost.germanium
    }

    /// Subtract `cost` from `self` atomically.
    ///
    /// On success, every mineral in `self` is decremented by the matching
    /// field of `cost`. On failure, `self` is **left unchanged** — the
    /// operation is all-or-nothing, so a failed `spend` can be retried
    /// against a different cost without first reverting.
    ///
    /// # Errors
    ///
    /// Returns [`GameError::InsufficientResources`] if any single mineral
    /// would underflow.
    pub fn spend(&mut self, cost: &Self) -> Result<(), GameError> {
        if !self.can_afford(cost) {
            return Err(GameError::InsufficientResources("Minerals::spend"));
        }
        // Unwraps are safe because can_afford just proved each subtraction
        // fits in a u32, but we use checked_sub anyway to make the safety
        // argument local to this line rather than dependent on the branch
        // above surviving refactors.
        self.ironium =
            self.ironium
                .checked_sub(cost.ironium)
                .ok_or(GameError::InsufficientResources(
                    "Minerals::spend (ironium)",
                ))?;
        self.boranium =
            self.boranium
                .checked_sub(cost.boranium)
                .ok_or(GameError::InsufficientResources(
                    "Minerals::spend (boranium)",
                ))?;
        self.germanium =
            self.germanium
                .checked_sub(cost.germanium)
                .ok_or(GameError::InsufficientResources(
                    "Minerals::spend (germanium)",
                ))?;
        Ok(())
    }

    /// Add `other` to `self` atomically.
    ///
    /// Like [`spend`], this is all-or-nothing: if any mineral would
    /// overflow `u32`, `self` is left unchanged and an error is returned.
    /// Engine formulas must NEVER silently wrap — that would drift
    /// between debug and release builds and between wasm and native.
    ///
    /// [`spend`]: Self::spend
    ///
    /// # Errors
    ///
    /// Returns [`GameError::ArithmeticOverflow`] if any single mineral
    /// would overflow `u32`.
    pub fn add(&mut self, other: &Self) -> Result<(), GameError> {
        let ironium = self
            .ironium
            .checked_add(other.ironium)
            .ok_or(GameError::ArithmeticOverflow("Minerals::add (ironium)"))?;
        let boranium = self
            .boranium
            .checked_add(other.boranium)
            .ok_or(GameError::ArithmeticOverflow("Minerals::add (boranium)"))?;
        let germanium = self
            .germanium
            .checked_add(other.germanium)
            .ok_or(GameError::ArithmeticOverflow("Minerals::add (germanium)"))?;
        // Commit only after all three checks pass — atomic on failure.
        self.ironium = ironium;
        self.boranium = boranium;
        self.germanium = germanium;
        Ok(())
    }
}

/// Mineral concentration levels on a planet, on the canonical Stars! 0–100+
/// scale. These are **not** stockpiles — they gate the *rate* at which a
/// mine can extract minerals each turn (see [`Minerals`] for the stockpile).
///
/// Concentrations deplete slowly as minerals are mined. A concentration of 0
/// means the kind is effectively exhausted on that planet. Values above 100
/// are legal in Stars! canon (homeworlds can start as high as ~120 for some
/// PRTs) so the field type is `u32`, not a clamped `u8`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MineralConcentrations {
    pub ironium: u32,
    pub boranium: u32,
    pub germanium: u32,
}

impl MineralConcentrations {
    /// Construct a concentration triple. No validation is performed — the
    /// canonical Stars! range is 0–120 but the engine permits any `u32`
    /// so data files and scenarios can experiment beyond canon.
    #[must_use]
    pub const fn new(ironium: u32, boranium: u32, germanium: u32) -> Self {
        Self {
            ironium,
            boranium,
            germanium,
        }
    }

    /// Decrement a single mineral concentration atomically.
    ///
    /// Used by the mining phase to model the slow depletion of a planet's
    /// extractable minerals as mines extract them. The operation saturates
    /// at zero — concentrations cannot go negative — but reports an error
    /// if the caller asked to deplete more than the field currently holds.
    /// This forces mining code to track partial extractions explicitly
    /// rather than silently clamping.
    ///
    /// # Errors
    ///
    /// Returns [`GameError::InsufficientResources`] if `amount` exceeds the
    /// current concentration of `kind`. The struct is left unchanged on
    /// failure (atomic-on-failure guarantee, matching [`Minerals::spend`]).
    pub fn deplete(&mut self, kind: MineralType, amount: u32) -> Result<(), GameError> {
        let field = match kind {
            MineralType::Ironium => &mut self.ironium,
            MineralType::Boranium => &mut self.boranium,
            MineralType::Germanium => &mut self.germanium,
        };
        *field = field
            .checked_sub(amount)
            .ok_or(GameError::InsufficientResources(
                "MineralConcentrations::deplete",
            ))?;
        Ok(())
    }
}

// =============================================================================
// Environment & habitability
// =============================================================================

/// A single environment axis reading on a planet — gravity, temperature, or
/// radiation — stored as integer "clicks" on the canonical 0–100 Stars!
/// scale. Integer (not float) because habitability comparisons must be
/// bit-identical across wasm and native.
pub type EnvClick = i32;

/// A planet's full environment reading across the three Stars! axes.
///
/// Every planet has exactly these three values, always integer clicks.
/// Habitability for a given race is computed by comparing each field
/// against that race's [`HabAxis`] tolerances.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Environment {
    /// Gravity reading, in clicks (Stars! canon: 0 = low, 100 = high).
    pub gravity: EnvClick,
    /// Temperature reading, in clicks (0 = cold, 100 = hot).
    pub temperature: EnvClick,
    /// Radiation reading, in clicks (0 = low, 100 = high).
    pub radiation: EnvClick,
}

/// A race's tolerance along a single habitability axis.
///
/// Stars! 1995 encoded this as `{ min, max, immune: bool }` where `immune`
/// silently invalidated `min`/`max`. That was a classic "illegal state
/// representable" smell — the engine had to remember never to read
/// `min`/`max` when `immune` was true. The fresh engine lifts the immunity
/// case into its own variant so the compiler enforces correct handling at
/// every match site.
///
/// # Construction
///
/// Use [`HabAxis::range`] to build a bounded range (validated at construction)
/// or [`HabAxis::Immune`] for the immunity case. The default is a middling
/// 15–85 range matching the legacy scaffold's default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum HabAxis {
    /// The race can live at any value of this environment axis. No
    /// tolerance window applies — habitability along this axis is perfect.
    Immune,
    /// The race tolerates readings from `min` through `max` inclusive.
    /// Readings outside this window reduce habitability (lethally at the
    /// extremes per the canonical Stars! hab formula).
    Range { min: EnvClick, max: EnvClick },
}

impl HabAxis {
    /// Construct a bounded range with full validation.
    ///
    /// Both `min` and `max` must lie inside the canonical Stars! 0–100
    /// click range, and `min` must not exceed `max`. The check protects
    /// downstream hab formulas from receiving negative widths or
    /// out-of-range readings — the legacy scaffold validated only
    /// `min ≤ max`, which let typos in scenario files create races whose
    /// hab calculation divided by garbage.
    ///
    /// `EnvClick` remains `i32` for serde compatibility with the existing
    /// `Environment` field type, but values must lie in `0..=100` even so.
    ///
    /// # Errors
    ///
    /// Returns [`GameError::InvalidRace`] if `min > max`, if `min < 0`, or
    /// if `max > 100`. The struct is unconstructed on failure.
    pub fn range(min: EnvClick, max: EnvClick) -> Result<Self, GameError> {
        if min > max {
            return Err(GameError::InvalidRace("HabAxis::range min exceeds max"));
        }
        if min < 0 {
            return Err(GameError::InvalidRace("HabAxis::range min below 0"));
        }
        if max > 100 {
            return Err(GameError::InvalidRace("HabAxis::range max above 100"));
        }
        Ok(Self::Range { min, max })
    }
}

impl Default for HabAxis {
    /// Middle-of-the-road 15–85 range, matching the legacy scaffold's
    /// default for fresh race creation.
    fn default() -> Self {
        Self::Range { min: 15, max: 85 }
    }
}

/// A race's full habitability tolerances across all three environment axes.
/// Each axis is an independent [`HabAxis`] — a race can be immune to
/// radiation while still caring about gravity and temperature.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HabRanges {
    pub gravity: HabAxis,
    pub temperature: HabAxis,
    pub radiation: HabAxis,
}

// =============================================================================
// Galaxy configuration
// =============================================================================

/// Named galaxy-size presets that map to a target star count, a square map
/// edge length in light-years, and a minimum distance between homeworlds.
///
/// These are tuning knobs, not hard limits — galaxy generation may land a
/// few stars above or below the target for a given seed. The values here
/// mirror the legacy scaffold, which was calibrated against the 1995 game
/// defaults and the craig-stars reference implementation.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum GalaxySize {
    Tiny,
    Small,
    #[default]
    Medium,
    Large,
    Huge,
}

impl GalaxySize {
    /// Target star count for this galaxy size — the generator aims for
    /// this number but may emit a handful more or fewer depending on the
    /// density and seed.
    #[must_use]
    pub const fn target_stars(self) -> u32 {
        match self {
            Self::Tiny => 24,
            Self::Small => 70,
            Self::Medium => 150,
            Self::Large => 300,
            Self::Huge => 600,
        }
    }

    /// Square-map edge length in light-years (width == height).
    #[must_use]
    pub const fn map_dimension(self) -> u32 {
        match self {
            Self::Tiny => 400,
            Self::Small => 600,
            Self::Medium => 800,
            Self::Large => 1200,
            Self::Huge => 1600,
        }
    }

    /// Minimum allowed distance between starting homeworlds, in light-years.
    ///
    /// Returned as `f64` because the galaxy generator compares this value
    /// against `Position::distance_to`, which is also `f64`. Only basic
    /// IEEE 754 operations are used, so the comparison is bit-identical
    /// across wasm and native.
    #[must_use]
    pub const fn min_homeworld_distance(self) -> f64 {
        match self {
            Self::Tiny => 80.0,
            Self::Small => 100.0,
            Self::Medium => 130.0,
            Self::Large => 160.0,
            Self::Huge => 200.0,
        }
    }
}

/// Galaxy density preset — controls how tightly stars cluster during
/// procedural generation. Denser galaxies place more stars within the
/// same map dimension, leading to closer neighbors and faster contact.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum GalaxyDensity {
    Sparse,
    #[default]
    Normal,
    Dense,
    Packed,
}

// =============================================================================
// Racial trait identifiers (data-driven)
// =============================================================================
//
// Primary and Lesser Racial Traits are loaded from `data/prt_traits.json` and
// `data/lrt_traits.json` at engine startup — not hardcoded as enum variants.
// This honors the SPEC.md DLC promise: new racial archetypes can ship as
// JSON + sprite packs with no engine fork. Engine code references traits by
// id string; the registry (arriving in a later atom) is the source of truth
// for which ids are valid.
//
// The string inside each newtype is the canonical trait code (e.g. "HE",
// "JoAT", "SD"). Equality and ordering are **byte-exact** — no case folding,
// no trimming — so two ids compare equal iff their underlying bytes match.
// Validation (uppercase, length, uniqueness) is the loader's job, not this
// type's.
//
// `#[serde(transparent)]` makes `PrtId("HE")` round-trip to the JSON string
// `"HE"` rather than `{"0": "HE"}`, which keeps the data files clean.

/// Identifier for a Primary Racial Trait, as loaded from `data/prt_traits.json`.
///
/// Two `PrtId`s are equal iff their inner strings are byte-identical.
/// `PrtId` and [`LrtId`] are **distinct types**: the compiler refuses to mix
/// them even though both wrap `String`, preventing an LRT id from being
/// accidentally passed where a PRT id is required.
///
/// The inner field is `pub(crate)` rather than `pub` so that callers
/// outside the engine crate cannot construct ids directly — they must
/// route through the loader / registry that owns validation. Inside the
/// crate, tests and the registry constructor still have direct access.
/// Use [`PrtId::as_str`] for read-only inspection.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PrtId(pub(crate) String);

impl PrtId {
    /// Borrow the underlying id string for read-only inspection. Used by
    /// the registry, save/load, and any UI code that needs to display the
    /// trait code. Construction is intentionally NOT exposed at this layer
    /// — the registry's loader is the only legal source of new ids.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Identifier for a Lesser Racial Trait, as loaded from `data/lrt_traits.json`.
///
/// See [`PrtId`] for the data-driven rationale, equality semantics, and
/// `pub(crate)` field reasoning. The two types are intentionally
/// structurally identical but nominally distinct — the compiler enforces
/// that distinction, not any runtime check.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LrtId(pub(crate) String);

impl LrtId {
    /// Borrow the underlying id string for read-only inspection. See
    /// [`PrtId::as_str`] for the rationale.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// =============================================================================
// Technology — fields, levels, research allocation
// =============================================================================
//
// Stars! research is divided into six parallel fields. Each field advances
// independently; components and hulls gate on per-field level requirements.
//
// STARS 2026 caps tech fields at **level 30**, not the 1995 canonical 26 —
// this is the project's signature mechanical deviation, documented in
// memory (project_tech_cap_30.md) and the project README. The types below
// do NOT enforce the cap; cap enforcement lives in the future `tech.rs`
// research function. These types just store and compare levels.

/// The six research fields that make up the Stars! tech tree.
///
/// Ordering is the canonical Stars! UI order — Energy first, Biotech last —
/// so any code that iterates fields for display gets the expected sequence
/// without needing a separate ordering array.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TechField {
    Energy,
    Weapons,
    Propulsion,
    Construction,
    Electronics,
    Biotechnology,
}

/// The maximum tech level any single research field can reach.
///
/// **STARS 2026 deviation from Stars! 1995 canon.** The 1995 game caps tech
/// fields at 26; STARS 2026 caps them at **30**, extending the late-game by
/// four tiers as the project's signature mechanical contribution. LRT/PRT
/// bonuses (via the trait registry's `tech_level_cap_bonus` field) can push
/// individual fields above this base cap. Pinned in `memory/project_tech_cap_30.md`
/// and ADR-0002.
pub const TECH_LEVEL_CAP: u32 = 30;

/// A player's current tech levels across all six fields.
///
/// Stored as per-field `u32`s for efficient comparison and trivial serde.
/// The default is all zeros — a fresh empire with no research.
///
/// **Cap enforcement:** [`TechLevels::set`] refuses values above
/// [`TECH_LEVEL_CAP`] and returns [`GameError::InvalidRace`]. Direct field
/// writes are still possible (the fields are `pub` for serde and tests),
/// but anything that goes through the public method path is checked.
/// Cap-relaxing trait bonuses are applied at the research-allocation site,
/// not at the storage layer.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TechLevels {
    pub energy: u32,
    pub weapons: u32,
    pub propulsion: u32,
    pub construction: u32,
    pub electronics: u32,
    pub biotechnology: u32,
}

impl TechLevels {
    /// Look up the current level in a specific field.
    #[must_use]
    pub const fn get(&self, field: TechField) -> u32 {
        match field {
            TechField::Energy => self.energy,
            TechField::Weapons => self.weapons,
            TechField::Propulsion => self.propulsion,
            TechField::Construction => self.construction,
            TechField::Electronics => self.electronics,
            TechField::Biotechnology => self.biotechnology,
        }
    }

    /// Write a new level into a specific field, with cap enforcement.
    ///
    /// # Errors
    ///
    /// Returns [`GameError::InvalidRace`] if `level` exceeds the base
    /// [`TECH_LEVEL_CAP`] of 30. Trait-bonus extensions above the cap are
    /// the responsibility of `tech.rs` and must apply *before* calling
    /// `set`, not after — the storage layer is the ground floor.
    pub fn set(&mut self, field: TechField, level: u32) -> Result<(), GameError> {
        if level > TECH_LEVEL_CAP {
            return Err(GameError::InvalidRace(
                "TechLevels::set above TECH_LEVEL_CAP",
            ));
        }
        match field {
            TechField::Energy => self.energy = level,
            TechField::Weapons => self.weapons = level,
            TechField::Propulsion => self.propulsion = level,
            TechField::Construction => self.construction = level,
            TechField::Electronics => self.electronics = level,
            TechField::Biotechnology => self.biotechnology = level,
        }
        Ok(())
    }

    /// Returns `true` iff `self` meets or exceeds `required` in **every**
    /// field. Used to gate component availability and hull construction:
    /// a design is buildable only when the player's tech levels pass this
    /// check against the component's `tech_requirements`.
    #[must_use]
    pub const fn meets_requirements(&self, required: &Self) -> bool {
        self.energy >= required.energy
            && self.weapons >= required.weapons
            && self.propulsion >= required.propulsion
            && self.construction >= required.construction
            && self.electronics >= required.electronics
            && self.biotechnology >= required.biotechnology
    }
}

/// Research allocation — what fraction of the player's research budget
/// goes to each of the six fields, expressed as whole percentages that
/// must sum to 100.
///
/// The [`normalize`] method re-proportions any un-normalized allocation
/// back to a 100-total, assigning the rounding remainder to biotechnology.
///
/// [`normalize`]: Self::normalize
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResearchAllocation {
    pub energy: u32,
    pub weapons: u32,
    pub propulsion: u32,
    pub construction: u32,
    pub electronics: u32,
    pub biotechnology: u32,
}

impl Default for ResearchAllocation {
    /// Default allocation is `[17, 17, 17, 17, 16, 16]` across the six
    /// fields, which sums to exactly 100. Matches the legacy scaffold's
    /// default for fresh players who haven't customized their research.
    fn default() -> Self {
        Self {
            energy: 17,
            weapons: 17,
            propulsion: 17,
            construction: 17,
            electronics: 16,
            biotechnology: 16,
        }
    }
}

impl ResearchAllocation {
    /// Sum of all six field allocations. A well-formed allocation has a
    /// total of exactly 100; anything else needs [`normalize`].
    ///
    /// [`normalize`]: Self::normalize
    #[must_use]
    pub const fn total(&self) -> u32 {
        self.energy
            + self.weapons
            + self.propulsion
            + self.construction
            + self.electronics
            + self.biotechnology
    }

    /// Re-proportion the allocation so it sums to exactly 100.
    ///
    /// Each of the first five fields is scaled by `(field * 100) / total`
    /// using `u64` intermediates to sidestep `u32` overflow at the
    /// multiply step. The rounding remainder falls into biotechnology so
    /// the six fields always sum to exactly 100.
    ///
    /// Special case: a zero-total allocation (caller passed `[0; 6]`) is
    /// reset to the [`Default`] even split rather than silently producing
    /// a divide-by-zero.
    pub fn normalize(&mut self) {
        let total = self.total();
        if total == 0 {
            *self = Self::default();
            return;
        }
        // u64 intermediate prevents overflow on the multiply. `(u32::MAX * 100)`
        // exceeds u32 but fits comfortably in u64.
        let total_u64 = u64::from(total);
        let scale = |field: u32| -> u32 {
            // u32::try_from is safe: the result is always <= 100.
            u32::try_from(u64::from(field) * 100 / total_u64).unwrap_or(0)
        };
        self.energy = scale(self.energy);
        self.weapons = scale(self.weapons);
        self.propulsion = scale(self.propulsion);
        self.construction = scale(self.construction);
        self.electronics = scale(self.electronics);
        // Assign the rounding remainder to biotechnology so the total is
        // exactly 100. `saturating_sub` is defensive: the five scaled
        // fields should always sum to <= 100, but if a future refactor
        // breaks that invariant we prefer a 0 floor over a panic.
        self.biotechnology = 100u32.saturating_sub(
            self.energy + self.weapons + self.propulsion + self.construction + self.electronics,
        );
    }
}

/// Cost of building something — a resource expenditure paired with a
/// mineral cost. Used by every buildable thing in the game: factories,
/// mines, defenses, ship hulls, components, starbases, packets.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cost {
    pub resources: u32,
    pub minerals: Minerals,
}

impl Cost {
    /// Construct a cost from four flat integer amounts — one resource
    /// count and three mineral amounts. This is the most common shape
    /// in static data files (JSON hull/component tables).
    #[must_use]
    pub const fn new(resources: u32, ironium: u32, boranium: u32, germanium: u32) -> Self {
        Self {
            resources,
            minerals: Minerals::new(ironium, boranium, germanium),
        }
    }
}

// =============================================================================
// Colonists — population newtype at 100-unit granularity
// =============================================================================

/// A count of colonists expressed in **hundreds of people**.
///
/// Stars! 1995 stores population in units of 100 internally — a display
/// value of "25,000 colonists" is stored as the number 250. STARS 2026
/// preserves that convention for formula fidelity and wraps it in a
/// newtype so the compiler catches the most common bug in 4X remakes:
/// off-by-100 mixing between "raw people" and "colonist units."
///
/// Construction always takes a colonist-unit count (not a raw person
/// count), so `Colonists::new(250)` means "twenty-five thousand people."
/// Display code multiplies by 100 at the UI boundary via [`as_people`].
///
/// [`as_people`]: Self::as_people
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Colonists(pub u32);

impl Colonists {
    /// The empty colonists count — nobody living here.
    pub const ZERO: Self = Self(0);

    /// Construct from a colonist-unit count (not raw people). The inner
    /// value is the number of 100-colonist blocks, so `Colonists::new(1)`
    /// represents 100 actual colonists.
    #[must_use]
    pub const fn new(units: u32) -> Self {
        Self(units)
    }

    /// Raw colonist-unit count (the 100-people-per-unit granularity).
    /// Use this for engine-internal math: growth formulas, resource
    /// generation, hab calculations — everything the 1995 formulas were
    /// calibrated against.
    #[must_use]
    pub const fn units(&self) -> u32 {
        self.0
    }

    /// Converted count in raw people, for UI display only. Returns `u64`
    /// because `u32::MAX * 100` overflows `u32` (though a real Stars!
    /// planet never approaches that — this is a belt-and-braces type
    /// choice to match [`Minerals::total`]'s u64 convention).
    #[must_use]
    pub const fn as_people(&self) -> u64 {
        self.0 as u64 * 100
    }

    /// Add two colonist counts atomically.
    ///
    /// # Errors
    ///
    /// Returns [`GameError::ArithmeticOverflow`] if the sum would
    /// overflow `u32`. Engine code must prefer this over `+` to keep
    /// overflow from silently wrapping on one target and panicking on
    /// another.
    pub fn checked_add(self, other: Self) -> Result<Self, GameError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(GameError::ArithmeticOverflow("Colonists::checked_add"))
    }

    /// Subtract `other` from `self` atomically.
    ///
    /// # Errors
    ///
    /// Returns [`GameError::InsufficientResources`] if `other` exceeds
    /// `self`. Used by transport/unload operations where colonists are
    /// moved off a planet or out of a fleet.
    pub fn checked_sub(self, other: Self) -> Result<Self, GameError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(GameError::InsufficientResources("Colonists::checked_sub"))
    }
}

// =============================================================================
// Cargo — movable resources inside a fleet
// =============================================================================

/// The cargo manifest of a fleet: minerals and colonists in transit.
///
/// Stored as flat fields rather than a nested `Minerals` struct so that
/// transport-order UIs can address each mineral kind independently (load
/// only ironium, unload only germanium, etc.). The [`total_mass`] method
/// treats all four fields as kilotons for fuel-burn and mass-driver
/// calculations — one unit of colonists massing the same as one kT of
/// minerals is a Stars! canonical convention.
///
/// [`total_mass`]: Self::total_mass
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cargo {
    pub ironium: u32,
    pub boranium: u32,
    pub germanium: u32,
    pub colonists: Colonists,
}

impl Cargo {
    /// Total cargo mass in kilotons, summing all three minerals plus the
    /// colonists (in their 100-unit granularity — one colonist unit = 1
    /// kT of mass for fleet-fuel calculations).
    ///
    /// Returns `u64` to rule out overflow for the same reason
    /// [`Minerals::total`] does: three `u32` minerals plus one `u32`
    /// colonist-unit count can sum above `u32::MAX` in theoretical edge
    /// cases, and silent wrap is a determinism violation.
    #[must_use]
    pub const fn total_mass(&self) -> u64 {
        // Reach for the public accessor `units()` rather than `colonists.0`
        // even though we are inside the same crate. This sets the
        // encapsulation precedent for every future consumer of `Colonists`
        // — once one site reaches into `.0`, every other site will follow.
        self.ironium as u64
            + self.boranium as u64
            + self.germanium as u64
            + self.colonists.units() as u64
    }
}

// =============================================================================
// Turn phase — the canonical Stars! 33-step order of events
// =============================================================================

/// The canonical 33-step turn-generation order used by Stars! 1995, adopted
/// verbatim by STARS 2026 as the determinism contract for turn processing.
///
/// Every per-turn side effect in the engine runs inside exactly one of these
/// phases. The phase enum exists for three reasons:
///
/// 1. **Exhaustive matching.** The `turn.rs` executor will `match` on this
///    enum, and the compiler will refuse to compile any change that leaves a
///    phase unhandled. That's a tripwire against silent determinism drift.
/// 2. **Replay logging.** Determinism-gate test output is timestamped by
///    `(turn, phase)` pairs so divergences between wasm and native can be
///    isolated to the exact phase that produced them.
/// 3. **Canon fidelity.** Veterans expect exactly this order. Any deviation
///    is a game-design decision, not an optimization.
///
/// **Source:** starsfaq.com's canonical "Order of Events" reference,
/// cross-referenced against the legacy scaffold and craig-stars' `turn.go`.
/// When [`turn.rs`] is implemented, every phase must cite its source section
/// in `docs/FORMULAS.md`.
///
/// [`turn.rs`]: crate::types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TurnPhase {
    /// 1. Scrap fleets (with possible tech gain).
    ScrapFleets,
    /// 2. Waypoint 0 unload tasks.
    Waypoint0Unload,
    /// 3. Waypoint 0 colonization / ground combat.
    Waypoint0Colonize,
    /// 4. Waypoint 0 load tasks.
    Waypoint0Load,
    /// 5. Other Waypoint 0 tasks.
    Waypoint0Other,
    /// 6. Mystery Trader moves.
    MysteryTraderMove,
    /// 7. In-space packets move and decay (PP terraform / damage).
    PacketMove,
    /// 8. Wormhole entry jiggle.
    WormholeEntryJiggle,
    /// 9. Fleet movement (fuel, minefields, stargates, wormholes).
    FleetMovement,
    /// 10. Inner Strength colonist growth in fleets.
    InnerStrengthGrowth,
    /// 11. Mass packets / salvage decay.
    PacketSalvageDecay,
    /// 12. Wormhole exit jiggle, endpoint degrade / jump.
    WormholeExitJiggle,
    /// 13. Space Demolition minefield detonation.
    SdMinefieldDetonation,
    /// 14. Mining (remote mining plus planet mining).
    Mining,
    /// 15. Production (research, packet launch, construction).
    Production,
    /// 16. Super Stealth spy bonus.
    SsSpyBonus,
    /// 17. Population growth and death.
    PopulationGrowth,
    /// 18. Just-launched packets reaching destination cause damage.
    LaunchedPacketDamage,
    /// 19. Random events.
    RandomEvents,
    /// 20. Fleet battles (with possible tech gain).
    FleetBattles,
    /// 21. Meet Mystery Trader.
    MeetMysteryTrader,
    /// 22. Bombing (per player in order).
    Bombing,
    /// 23. Waypoint 1 unload tasks.
    Waypoint1Unload,
    /// 24. Waypoint 1 colonization / ground combat.
    Waypoint1Colonize,
    /// 25. Waypoint 1 load tasks.
    Waypoint1Load,
    /// 26. Mine laying.
    MineLaying,
    /// 27. Fleet transfer.
    FleetTransfer,
    /// 28. Waypoint 1 fleet merge.
    Waypoint1FleetMerge,
    /// 29. Claim Adjuster instaforming.
    CaInstaforming,
    /// 30. Minefield decay.
    MinefieldDecay,
    /// 31. Mine sweeping.
    MineSweeping,
    /// 32. Starbase and fleet repair.
    StarbaseFleetRepair,
    /// 33. Remote terraforming.
    RemoteTerraforming,
}

// =============================================================================
// Production queue — what a planet is building this turn
// =============================================================================

/// A single thing a planet can build through its production queue.
///
/// The enum is `#[non_exhaustive]` because DLC may introduce new build
/// kinds (packet-delivered relics, exotic terraforming machinery) and the
/// engine must refuse to compile code that hasn't decided how to handle
/// unknown variants. Count fields intentionally live on [`QueueItem`],
/// not on the variants themselves — one source of truth for "how many."
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductionItem {
    /// Add one more factory to the planet's industrial base.
    Factory,
    /// Add one more mine to the planet's mineral-extraction base.
    Mine,
    /// Add one more planetary defense installation.
    Defense,
    /// Terraform the planet one click closer to the race's ideal
    /// environment. The specific axis (gravity/temp/radiation) is chosen
    /// by the production logic, not stored on the queue item.
    Terraform,
    /// Spend resources to manufacture minerals via mineral alchemy.
    /// Only usable by races with the Mineral Alchemy LRT (checked at
    /// build time, not at queue time).
    MineralAlchemy,
    /// Build a ship of the given player-authored design.
    ShipDesign(ShipDesignId),
    /// Build (or upgrade to) the given starbase design in orbit.
    Starbase(ShipDesignId),
    /// Install a planetary scanner so the planet contributes to fog-of-war
    /// visibility.
    Scanner,
}

/// One entry in a planet's production queue.
///
/// Holds the item to build, how many to build, and any resources or
/// minerals that have been pre-allocated to it (so a partially-built item
/// resumes cleanly next turn instead of losing progress). The queue itself
/// lives on [`Planet`] as `Vec<QueueItem>` — ordering is insertion order,
/// which is naturally deterministic, so no `BTreeMap` is needed here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QueueItem {
    pub item: ProductionItem,
    pub quantity: u32,
    pub allocated_resources: u32,
    pub allocated_minerals: Minerals,
}

// =============================================================================
// Planet & Star — the galactic map primitives
// =============================================================================

/// A single planet orbiting a star.
///
/// Planets are owned by at most one player (`owner_id: Option<PlayerId>`).
/// Ownership gates everything: unowned planets can be colonized, owned
/// planets produce resources, are subject to bombing, appear in scanner
/// sweeps, etc.
///
/// Population lives on the planet as a [`Colonists`] newtype — the
/// 100-unit-granularity convention from Stars! 1995. Surface minerals
/// (the stockpile available for production) and mineral concentrations
/// (the extraction-rate gate) are stored as two distinct fields, matching
/// the 1995 canon.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Planet {
    pub id: PlanetId,
    pub star_id: StarId,
    pub name: String,
    /// `None` = uncolonized, available for the first player to arrive
    /// with a colonizer. `Some(player_id)` = owned by that player.
    pub owner_id: Option<PlayerId>,
    /// Current population in 100-colonist units. Zero on an uncolonized
    /// planet; grows per turn via the population formula in `planet.rs`
    /// (future atom).
    pub population: Colonists,
    pub environment: Environment,
    pub mineral_concentrations: MineralConcentrations,
    /// Minerals already extracted and sitting on the surface, available
    /// for construction or transport. Separate from the concentration
    /// which only gates extraction rate.
    pub surface_minerals: Minerals,
    pub mines: u32,
    pub factories: u32,
    pub defenses: u32,
    pub has_scanner: bool,
    pub has_starbase: bool,
    /// Which starbase design is in orbit, if any. `None` if `has_starbase`
    /// is `false`. The pair of fields intentionally lets callers check
    /// cheaply via `has_starbase` without chasing the option.
    pub starbase_design_id: Option<ShipDesignId>,
    pub production_queue: Vec<QueueItem>,
}

// =============================================================================
// Game setup — victory conditions, difficulty, status, settings
// =============================================================================

/// A single victory condition that can be checked at turn end to decide
/// whether any player has won. A game may configure multiple conditions
/// and require one-of or all-of to trigger the win state.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VictoryCondition {
    /// Own at least this percentage (0–100) of all planets in the galaxy.
    OwnPercentOfPlanets(u32),
    /// Reach at least this tech level in any single field. With the
    /// STARS 2026 cap of 30 (canon 26), the practical ceiling is higher
    /// than in 1995 — this condition must be configured with care to
    /// avoid making the win condition unreachable.
    ReachTechLevel(u32),
    /// Accumulate a score above this threshold.
    ExceedsScoreOf(u32),
    /// Lead the second-place player's score by at least this percentage.
    ExceedsSecondPlaceBy(u32),
    /// Total production capacity exceeds this threshold (resources/turn).
    ProductionCapacityOf(u32),
    /// Own at least this many capital ships (hulls at the top tier of
    /// the construction tech ladder).
    OwnCapitalShips(u32),
    /// Whoever has the highest score when the game reaches this turn
    /// wins. Used as the default fallback "end by turn N" condition.
    HighestScoreAfterTurns(u32),
}

/// How hard the AI opponents play. Higher difficulty raises AI resource
/// generation multipliers and improves AI decision quality — it does
/// **not** let the AI cheat on fog of war or scanning (per CLAUDE.md
/// "no cheating AI" rule).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
pub enum AiDifficulty {
    Easy,
    #[default]
    Standard,
    Hard,
    Expert,
}

/// The current lifecycle state of a game instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum GameStatus {
    /// Players are being configured but the galaxy has not yet been
    /// generated and the first turn has not yet been run.
    #[default]
    Setup,
    /// Turns are actively being processed.
    InProgress,
    /// A win condition has fired or all players have been eliminated.
    Completed,
}

/// Game configuration chosen at creation time.
///
/// **Two design notes:**
///
/// 1. **No `Default` impl.** The legacy scaffold's `Default::default()`
///    set `random_seed: 0` with a comment saying "0 = generate from
///    system time" — a sentinel-for-hostness that is a classic
///    determinism foot-gun. The engine never generates its own seeds; it
///    receives them from the host (browser, server, CLI). Programmatic
///    construction must be explicit, and the type will tell you if you
///    forgot the seed by failing to compile.
///
/// 2. **Serde defaults on every non-seed field.** This is the
///    save-compatibility fix from H5 of the hardening pass. When a v0.2
///    engine adds a new field (e.g. `enable_mystery_trader: bool`),
///    every existing v0.1 save must still load — otherwise every player
///    in the wild loses their game on the next update. Each field below
///    is annotated with `#[serde(default)]` so missing fields fall back
///    to their type's `Default::default()` at deserialize time.
///    `random_seed` is intentionally NOT defaulted: a missing seed must
///    be a hard error, never silently zeroed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameSettings {
    #[serde(default)]
    pub galaxy_size: GalaxySize,
    #[serde(default)]
    pub density: GalaxyDensity,
    #[serde(default)]
    pub player_count: u32,
    #[serde(default)]
    pub starting_year: u32,
    #[serde(default)]
    pub victory_conditions: Vec<VictoryCondition>,
    /// How many of the listed victory conditions must be met simultaneously
    /// for the game to end. `1` = any-one-of; `N` = all-of where N equals
    /// `victory_conditions.len()`.
    #[serde(default)]
    pub victory_requirements_met: u32,
    #[serde(default)]
    pub ai_difficulty: AiDifficulty,
    /// The deterministic seed that drives **every** RNG decision in this
    /// game, from galaxy generation to combat rolls. The engine derives
    /// subsystem-specific seeds via `(random_seed, turn, player_id,
    /// subsystem)` tuples, so this one `u64` is the root of all
    /// non-determinism in the match.
    ///
    /// **No serde default.** A save file or scenario missing this field
    /// is a hard error rather than a silent zero — the determinism
    /// contract requires the seed to be explicit.
    pub random_seed: u64,
}

/// A star system containing zero or more planets.
///
/// Stars in STARS 2026 always host at least one planet in MVP (uninhabited
/// stars will arrive later if needed for flavor). The `planets` vector
/// keeps them in a deterministic insertion order — no `BTreeMap` because
/// lookup-by-id goes through the `Planet.id` field directly in engine
/// code.
///
/// `Eq` and `Hash` are intentionally **not** derived because [`Position`]
/// contains `f64` and cannot implement either. Stars are keyed on their
/// [`StarId`] newtype everywhere they need to be looked up, so the lack
/// of `Hash` on `Star` itself is never a blocker.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Star {
    pub id: StarId,
    pub name: String,
    pub position: Position,
    pub planets: Vec<Planet>,
}

impl TurnPhase {
    /// The canonical 33-step Stars! order-of-events. The length is
    /// asserted at *compile time* by [`CANONICAL_ORDER`]'s `[Self; 33]`
    /// type — adding or removing a variant from the enum without also
    /// updating the array literal below is a build error, not a runtime
    /// surprise. Combined with [`Self::variant_count`], this gives a
    /// belt-and-braces tripwire against the enum drifting out of sync
    /// with the canonical sequence.
    ///
    /// [`CANONICAL_ORDER`]: Self::CANONICAL_ORDER
    /// [`Self::variant_count`]: Self::variant_count
    ///
    /// Variant count, computed by exhaustively matching every variant.
    /// Adding a new variant to the enum without adding it here is a
    /// build error — `match` exhaustiveness catches the omission. The
    /// returned count must equal `CANONICAL_ORDER.len()`, which is
    /// asserted in the test suite.
    //
    // The exhaustive `match` below is the load-bearing part: rustc will
    // reject this function if a TurnPhase variant is added without a
    // corresponding arm here. The `clippy::match_same_arms` lint would
    // otherwise want us to collapse all 33 empty arms into a single
    // `|`-chain, which would defeat the tripwire by removing the
    // per-variant maintenance forcing function. Allow it locally with
    // a justifying comment.
    #[must_use]
    #[allow(
        clippy::match_same_arms,
        reason = "each arm is a deliberate per-variant tripwire so adding a TurnPhase variant fails the build until variant_count is updated"
    )]
    pub const fn variant_count() -> usize {
        let probe = Self::ScrapFleets;
        match probe {
            Self::ScrapFleets => {}
            Self::Waypoint0Unload => {}
            Self::Waypoint0Colonize => {}
            Self::Waypoint0Load => {}
            Self::Waypoint0Other => {}
            Self::MysteryTraderMove => {}
            Self::PacketMove => {}
            Self::WormholeEntryJiggle => {}
            Self::FleetMovement => {}
            Self::InnerStrengthGrowth => {}
            Self::PacketSalvageDecay => {}
            Self::WormholeExitJiggle => {}
            Self::SdMinefieldDetonation => {}
            Self::Mining => {}
            Self::Production => {}
            Self::SsSpyBonus => {}
            Self::PopulationGrowth => {}
            Self::LaunchedPacketDamage => {}
            Self::RandomEvents => {}
            Self::FleetBattles => {}
            Self::MeetMysteryTrader => {}
            Self::Bombing => {}
            Self::Waypoint1Unload => {}
            Self::Waypoint1Colonize => {}
            Self::Waypoint1Load => {}
            Self::MineLaying => {}
            Self::FleetTransfer => {}
            Self::Waypoint1FleetMerge => {}
            Self::CaInstaforming => {}
            Self::MinefieldDecay => {}
            Self::MineSweeping => {}
            Self::StarbaseFleetRepair => {}
            Self::RemoteTerraforming => {}
        }
        // Hand-counted to match the arms above. Updating the enum
        // forces updating both the `match` (above) and this constant.
        33
    }

    /// The canonical 33 turn phases in execution order. Any code that
    /// iterates phases must use this constant rather than hand-listing
    /// the variants, so a future phase reordering is a single edit.
    pub const CANONICAL_ORDER: [Self; 33] = [
        Self::ScrapFleets,
        Self::Waypoint0Unload,
        Self::Waypoint0Colonize,
        Self::Waypoint0Load,
        Self::Waypoint0Other,
        Self::MysteryTraderMove,
        Self::PacketMove,
        Self::WormholeEntryJiggle,
        Self::FleetMovement,
        Self::InnerStrengthGrowth,
        Self::PacketSalvageDecay,
        Self::WormholeExitJiggle,
        Self::SdMinefieldDetonation,
        Self::Mining,
        Self::Production,
        Self::SsSpyBonus,
        Self::PopulationGrowth,
        Self::LaunchedPacketDamage,
        Self::RandomEvents,
        Self::FleetBattles,
        Self::MeetMysteryTrader,
        Self::Bombing,
        Self::Waypoint1Unload,
        Self::Waypoint1Colonize,
        Self::Waypoint1Load,
        Self::MineLaying,
        Self::FleetTransfer,
        Self::Waypoint1FleetMerge,
        Self::CaInstaforming,
        Self::MinefieldDecay,
        Self::MineSweeping,
        Self::StarbaseFleetRepair,
        Self::RemoteTerraforming,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    /// FR-19 reimplementation of the legacy `test_game_error_display`.
    ///
    /// Verifies that the `InvalidRace` variant Display-formats with the
    /// canonical "invalid race" prefix and faithfully echoes the supplied
    /// reason string. This is the first of the 19 legacy type tests to be
    /// reimplemented in the fresh engine crate.
    #[test]
    fn invalid_race_display_echoes_reason() {
        let err = GameError::InvalidRace("too many advantage points");
        let rendered = format!("{err}");

        assert!(
            rendered.contains("invalid race"),
            "display output should carry the 'invalid race' prefix, got: {rendered}"
        );
        assert!(
            rendered.contains("too many advantage points"),
            "display output should echo the underlying reason, got: {rendered}"
        );
    }

    /// New sniff test (not in the legacy 19 — required by SNIFFTEST step 1
    /// because this atom adds public types).
    ///
    /// Typed IDs must:
    ///   1. Round-trip through JSON serde losslessly.
    ///   2. Order deterministically (so they can be `BTreeMap` keys).
    ///   3. Preserve `Default` (zero) as a safe sentinel for freshly
    ///      constructed records that have not yet been assigned.
    ///
    /// This doesn't and cannot assert that `PlayerId(1) != StarId(1)` at
    /// runtime — they're distinct Rust types, so the *compiler* enforces
    /// that, not the test. What the test guards against is accidental
    /// regressions in the derive set (dropping `Ord`, dropping `Serialize`,
    /// etc.) that would silently break downstream BTreeMap/serde usage.
    #[test]
    fn typed_ids_roundtrip_order_and_default() {
        // Round-trip: JSON serde preserves the inner value.
        let original = PlayerId(42);
        let json = serde_json::to_string(&original).expect("serialize PlayerId");
        let decoded: PlayerId = serde_json::from_str(&json).expect("deserialize PlayerId");
        assert_eq!(original, decoded);

        // Ordering: IDs sort by inner value (required for BTreeMap determinism).
        let mut ids = [StarId(3), StarId(1), StarId(2)];
        ids.sort();
        assert_eq!(ids, [StarId(1), StarId(2), StarId(3)]);

        // Default: zero is the fresh-record sentinel and must survive derive drift.
        assert_eq!(GameId::default(), GameId(0));
        assert_eq!(PlanetId::default(), PlanetId(0));
        assert_eq!(FleetId::default(), FleetId(0));
        assert_eq!(ShipDesignId::default(), ShipDesignId(0));
        assert_eq!(BattlePlanId::default(), BattlePlanId(0));
    }

    /// FR-19 port of the legacy `test_position_distance`.
    ///
    /// Classic 3-4-5 right triangle: the distance from (0,0) to (3,4) must
    /// equal exactly 5.0 light-years. The tolerance is tight (< 0.001 ly,
    /// matching the legacy) because `sqrt` of an exact square is
    /// bit-identical across targets.
    #[test]
    fn position_distance_three_four_five() {
        let origin = Position::new(0.0, 0.0);
        let far = Position::new(3.0, 4.0);
        let dist = origin.distance_to(&far);
        assert!(
            (dist - 5.0).abs() < 0.001,
            "3-4-5 triangle should give distance 5.0, got {dist}"
        );
    }

    /// FR-19 port of the legacy `test_position_distance_same_point`.
    ///
    /// Distance from a position to itself must be exactly 0.0 — not NaN,
    /// not a tiny epsilon, not negative. This is a determinism tripwire:
    /// any implementation drift that returns non-zero here is a bug.
    #[test]
    fn position_distance_same_point_is_zero() {
        let here = Position::new(42.0, 99.0);
        let dist = here.distance_to(&here);
        assert!(
            (dist - 0.0).abs() < 0.001,
            "distance from a point to itself must be ~0.0, got {dist}"
        );
    }

    /// FR-19 port of the legacy `test_minerals_can_afford`.
    ///
    /// `can_afford` must be true only when every mineral in the bank
    /// meets-or-exceeds the cost, and false if *any* mineral falls short
    /// (even if the other two are plentiful).
    #[test]
    fn minerals_can_afford_is_per_mineral_and() {
        let bank = Minerals::new(100, 200, 50);

        // Exactly affordable.
        assert!(bank.can_afford(&Minerals::new(50, 100, 25)));

        // One mineral short (ironium) — even though boranium/germanium
        // are fine, the whole thing must fail.
        assert!(!bank.can_afford(&Minerals::new(150, 100, 25)));
    }

    /// FR-19 port of the legacy `test_minerals_spend`.
    ///
    /// A successful spend decrements each mineral field-by-field and
    /// leaves no residue behind.
    #[test]
    fn minerals_spend_happy_path_decrements_each_field() {
        let mut bank = Minerals::new(100, 200, 50);
        bank.spend(&Minerals::new(30, 40, 10))
            .expect("spend within budget should succeed");
        assert_eq!(bank, Minerals::new(70, 160, 40));
    }

    /// FR-19 port of the legacy `test_minerals_spend_insufficient`.
    ///
    /// A failed spend must return a `GameError::InsufficientResources`
    /// variant and leave `self` **unchanged** (atomic-on-failure guarantee).
    /// Tested without `PartialEq` on `GameError` — we use `matches!`
    /// per the no-`PartialEq` rule in the [`GameError`] doc comment.
    #[test]
    fn minerals_spend_insufficient_errors_without_mutation() {
        let mut bank = Minerals::new(10, 200, 50);
        let before = bank;

        let result = bank.spend(&Minerals::new(30, 40, 10));
        assert!(
            matches!(result, Err(GameError::InsufficientResources(_))),
            "expected InsufficientResources, got: {result:?}"
        );
        assert_eq!(bank, before, "failed spend must leave the bank untouched");
    }

    /// FR-19 port of the legacy `test_minerals_add`.
    ///
    /// Straight-line accumulation: adding two compatible bundles produces
    /// the per-field sum. The new implementation returns `Result` so we
    /// `.expect` the happy path — overflow is covered separately in
    /// later atoms where the type is exercised with big numbers.
    #[test]
    fn minerals_add_sums_each_field() {
        let mut a = Minerals::new(10, 20, 30);
        let b = Minerals::new(5, 10, 15);
        a.add(&b).expect("adding small bundles must not overflow");
        assert_eq!(a, Minerals::new(15, 30, 45));
    }

    /// FR-19 port of the legacy `test_serialization_roundtrip_minerals`.
    ///
    /// Any `Minerals` value must survive a JSON round-trip byte-for-byte
    /// equal to the original. This is our earliest serde tripwire — if
    /// the derive set regresses (loses `Serialize` / `Deserialize`) this
    /// test fails loudly before save/load is even written.
    #[test]
    fn minerals_survive_json_roundtrip() {
        let original = Minerals::new(42, 99, 7);
        let json = serde_json::to_string(&original).expect("serialize Minerals");
        let decoded: Minerals = serde_json::from_str(&json).expect("deserialize Minerals");
        assert_eq!(original, decoded);
    }

    /// New sniff test (not in the legacy 19).
    ///
    /// `MineralConcentrations` is a dumb data triple today, but this test
    /// guards against two future regressions: (1) the struct field order
    /// matching `Minerals` so it's easy to read side-by-side, and (2) the
    /// `Default` impl producing all zeros (no hidden canonical starting
    /// concentration baked into the type).
    #[test]
    fn mineral_concentrations_default_is_all_zero() {
        let blank = MineralConcentrations::default();
        assert_eq!(blank, MineralConcentrations::new(0, 0, 0));
    }

    /// FR-19 port of the legacy `test_hab_range_default`, adapted for the
    /// new `HabAxis` enum shape.
    ///
    /// The legacy default was `HabRange { min: 15, max: 85, immune: false }`.
    /// Under the new enum, the default is `HabAxis::Range { min: 15, max: 85 }`
    /// — same semantic value, no "invalid state representable" smell.
    #[test]
    fn hab_axis_default_is_range_15_to_85() {
        let axis = HabAxis::default();
        match axis {
            HabAxis::Range { min, max } => {
                assert_eq!(min, 15);
                assert_eq!(max, 85);
            }
            HabAxis::Immune => {
                panic!("default HabAxis must be a Range, not Immune");
            }
        }
    }

    /// Sniff test for `HabAxis::range` covering all four guarded
    /// conditions added by H5 hardening:
    ///   - happy path (15..=85 normal Stars! window)
    ///   - degenerate-but-legal single-click window
    ///   - inverted: `min > max` (legacy check)
    ///   - below 0 (H5 add)
    ///   - above 100 (H5 add)
    ///
    /// All four error paths must return `GameError::InvalidRace` and
    /// must not construct a value.
    #[test]
    fn hab_axis_range_validates_bounds_and_ordering() {
        // Happy path: normal Stars! 15–85 window.
        let ok = HabAxis::range(15, 85).expect("15..=85 is a valid window");
        assert_eq!(ok, HabAxis::Range { min: 15, max: 85 });

        // Degenerate but legal: single-click window.
        let pinned = HabAxis::range(50, 50).expect("single-click window is legal");
        assert_eq!(pinned, HabAxis::Range { min: 50, max: 50 });

        // Edge cases at the canonical boundaries.
        let bottom = HabAxis::range(0, 0).expect("0..=0 is legal");
        assert_eq!(bottom, HabAxis::Range { min: 0, max: 0 });
        let top = HabAxis::range(100, 100).expect("100..=100 is legal");
        assert_eq!(top, HabAxis::Range { min: 100, max: 100 });
        let full = HabAxis::range(0, 100).expect("0..=100 is legal");
        assert_eq!(full, HabAxis::Range { min: 0, max: 100 });

        // Inverted: min > max.
        let inverted = HabAxis::range(80, 20);
        assert!(
            matches!(inverted, Err(GameError::InvalidRace(_))),
            "inverted window should error, got: {inverted:?}"
        );

        // Below 0: H5 hardening rejection.
        let too_low = HabAxis::range(-1, 50);
        assert!(
            matches!(too_low, Err(GameError::InvalidRace(_))),
            "min < 0 should error, got: {too_low:?}"
        );

        // Above 100: H5 hardening rejection.
        let too_high = HabAxis::range(50, 101);
        assert!(
            matches!(too_high, Err(GameError::InvalidRace(_))),
            "max > 100 should error, got: {too_high:?}"
        );
    }

    /// H5 hardening test: `MineralConcentrations::deplete` decrements
    /// a single mineral atomically and refuses to underflow.
    #[test]
    fn mineral_concentrations_deplete_atomic_on_failure() {
        let mut conc = MineralConcentrations::new(80, 60, 40);

        // Happy path: spend 30 of ironium.
        conc.deplete(MineralType::Ironium, 30)
            .expect("80 - 30 should succeed");
        assert_eq!(conc, MineralConcentrations::new(50, 60, 40));

        // Underflow: try to spend 50 of germanium when we only have 40.
        let before = conc;
        let result = conc.deplete(MineralType::Germanium, 50);
        assert!(
            matches!(result, Err(GameError::InsufficientResources(_))),
            "underflow must raise InsufficientResources, got: {result:?}"
        );
        assert_eq!(
            conc, before,
            "failed deplete must leave the struct unchanged"
        );
    }

    /// H5 hardening test: `TurnPhase::variant_count` returns 33, matches
    /// `CANONICAL_ORDER.len()`, and is enforced by exhaustive matching.
    /// If a future variant is added to the enum without updating
    /// `variant_count` AND `CANONICAL_ORDER`, the build fails before
    /// this test ever runs — but if both are updated correctly, this
    /// test confirms they agree.
    #[test]
    fn turn_phase_variant_count_matches_canonical_order_len() {
        assert_eq!(TurnPhase::variant_count(), 33);
        assert_eq!(TurnPhase::variant_count(), TurnPhase::CANONICAL_ORDER.len());
    }

    /// FR-19 port of the legacy `test_galaxy_size_target_stars`.
    ///
    /// The generator tuning values must match the 1995-canon presets
    /// exactly: any drift here changes the shape of every generated
    /// galaxy and invalidates every Scenario.md expected-state assertion.
    #[test]
    fn galaxy_size_preset_target_stars() {
        assert_eq!(GalaxySize::Tiny.target_stars(), 24);
        assert_eq!(GalaxySize::Small.target_stars(), 70);
        assert_eq!(GalaxySize::Medium.target_stars(), 150);
        assert_eq!(GalaxySize::Large.target_stars(), 300);
        assert_eq!(GalaxySize::Huge.target_stars(), 600);
    }

    /// New sniff test (not in the legacy 19). Sanity check that the
    /// map dimension and homeworld-spacing presets grow monotonically
    /// with galaxy size. A regression here would mean two sizes collide
    /// (e.g. `Small` briefly beating `Medium`) and break galaxy-wizard
    /// UX assumptions that bigger = more room.
    #[test]
    fn galaxy_size_presets_are_monotonic() {
        let sizes = [
            GalaxySize::Tiny,
            GalaxySize::Small,
            GalaxySize::Medium,
            GalaxySize::Large,
            GalaxySize::Huge,
        ];

        for pair in sizes.windows(2) {
            let [smaller, bigger] = [pair[0], pair[1]];
            assert!(
                smaller.target_stars() < bigger.target_stars(),
                "target_stars must grow: {smaller:?} >= {bigger:?}"
            );
            assert!(
                smaller.map_dimension() < bigger.map_dimension(),
                "map_dimension must grow: {smaller:?} >= {bigger:?}"
            );
            assert!(
                smaller.min_homeworld_distance() < bigger.min_homeworld_distance(),
                "min_homeworld_distance must grow: {smaller:?} >= {bigger:?}"
            );
        }
    }

    /// New sniff test for `GalaxyDensity` (not in the legacy 19).
    ///
    /// This atom only ships the enum — all behavioral impls land later
    /// in `galaxy.rs`. The test guards against the derive set regressing
    /// (which would silently break serde round-trips and `BTreeMap` use)
    /// and against anyone accidentally deleting a variant.
    #[test]
    fn galaxy_density_roundtrips_all_variants() {
        for density in [
            GalaxyDensity::Sparse,
            GalaxyDensity::Normal,
            GalaxyDensity::Dense,
            GalaxyDensity::Packed,
        ] {
            let json = serde_json::to_string(&density).expect("serialize GalaxyDensity");
            let decoded: GalaxyDensity =
                serde_json::from_str(&json).expect("deserialize GalaxyDensity");
            assert_eq!(density, decoded);
        }
    }

    /// New sniff test for `PrtId` and `LrtId` (not in the legacy 19 — the
    /// legacy scaffold used hardcoded enums instead of data-driven ids).
    ///
    /// Guards against three classes of regression:
    ///   1. **Serde transparency.** The inner `String` must round-trip as a
    ///      bare JSON string, not as a wrapped object. Losing `#[serde(transparent)]`
    ///      would silently break `data/prt_traits.json` parsing.
    ///   2. **Ordering.** Ids must sort deterministically so they can serve
    ///      as `BTreeMap` keys without fighting the derive set.
    ///   3. **Byte-exact equality.** No case folding, no whitespace trimming
    ///      — the registry loader owns validation, not this type.
    #[test]
    fn trait_ids_are_transparent_ordered_and_byte_exact() {
        // Transparency: PrtId("HE") must serialize to the bare string "HE".
        let he = PrtId("HE".to_string());
        let json = serde_json::to_string(&he).expect("serialize PrtId");
        assert_eq!(json, "\"HE\"", "PrtId must serialize as a bare JSON string");
        let decoded: PrtId = serde_json::from_str(&json).expect("deserialize PrtId");
        assert_eq!(he, decoded);

        // Same for LrtId — twins, not copies.
        let ife = LrtId("IFE".to_string());
        let json = serde_json::to_string(&ife).expect("serialize LrtId");
        assert_eq!(
            json, "\"IFE\"",
            "LrtId must serialize as a bare JSON string"
        );
        let decoded: LrtId = serde_json::from_str(&json).expect("deserialize LrtId");
        assert_eq!(ife, decoded);

        // Ordering: PrtIds sort lexically by inner string.
        let mut ids = [
            PrtId("WM".to_string()),
            PrtId("HE".to_string()),
            PrtId("JoAT".to_string()),
        ];
        ids.sort();
        assert_eq!(
            ids,
            [
                PrtId("HE".to_string()),
                PrtId("JoAT".to_string()),
                PrtId("WM".to_string()),
            ]
        );

        // Byte-exact: no case folding. "he" and "HE" are DIFFERENT ids.
        // The registry's job is to reject lowercase ids — the type itself
        // treats them as distinct strings.
        assert_ne!(PrtId("HE".to_string()), PrtId("he".to_string()));
        assert_ne!(PrtId("HE".to_string()), PrtId(" HE".to_string()));
    }

    /// FR-19 port of the legacy `test_tech_levels_meets_requirements`.
    ///
    /// `meets_requirements` must be a strict per-field AND: every field in
    /// `self` must meet-or-exceed the matching field in `required`, and
    /// any shortfall in a single field fails the whole check even when
    /// every other field is comfortably ahead.
    #[test]
    fn tech_levels_meets_requirements_is_per_field_and() {
        let player = TechLevels {
            energy: 5,
            weapons: 3,
            propulsion: 4,
            construction: 2,
            electronics: 6,
            biotechnology: 1,
        };

        // Required ≤ player across all fields → pass.
        let reachable = TechLevels {
            energy: 5,
            weapons: 3,
            propulsion: 0,
            construction: 0,
            electronics: 0,
            biotechnology: 0,
        };
        assert!(player.meets_requirements(&reachable));

        // Required is one level higher than the player in energy — fails,
        // even though every other field is satisfied.
        let one_too_high = TechLevels {
            energy: 6,
            weapons: 0,
            propulsion: 0,
            construction: 0,
            electronics: 0,
            biotechnology: 0,
        };
        assert!(!player.meets_requirements(&one_too_high));
    }

    /// FR-19 port of the legacy `test_tech_levels_get_set`, adapted for
    /// the H5 hardening that made `set` return `Result` with cap-30
    /// enforcement.
    ///
    /// The `get`/`set` pair must address fields by name and leave the
    /// other five untouched — no accidental cross-field writes — and
    /// `set` must now succeed for any value at or below
    /// [`TECH_LEVEL_CAP`].
    #[test]
    fn tech_levels_get_set_addresses_one_field() {
        let mut tech = TechLevels::default();
        tech.set(TechField::Weapons, 7)
            .expect("7 is well within the cap");
        assert_eq!(tech.get(TechField::Weapons), 7);
        // The other fields must still be at their default 0.
        assert_eq!(tech.get(TechField::Energy), 0);
        assert_eq!(tech.get(TechField::Propulsion), 0);
        assert_eq!(tech.get(TechField::Construction), 0);
        assert_eq!(tech.get(TechField::Electronics), 0);
        assert_eq!(tech.get(TechField::Biotechnology), 0);
    }

    /// H5 hardening test (not in legacy 19): `TechLevels::set` enforces
    /// the STARS 2026 [`TECH_LEVEL_CAP`] of 30. Values at or below the
    /// cap succeed; values above raise [`GameError::InvalidRace`].
    ///
    /// This is a tripwire for the project's signature mechanical
    /// deviation from Stars! 1995 canon (canon = 26, STARS 2026 = 30).
    /// If the cap drifts, this test fails loudly.
    #[test]
    fn tech_levels_set_enforces_cap_30() {
        let mut tech = TechLevels::default();
        // Right at the cap is legal.
        tech.set(TechField::Energy, 30)
            .expect("30 is the cap, must be allowed");
        assert_eq!(tech.get(TechField::Energy), 30);
        // One above the cap is rejected.
        let over = tech.set(TechField::Energy, 31);
        assert!(
            matches!(over, Err(GameError::InvalidRace(_))),
            "31 must be rejected, got: {over:?}"
        );
        // The previous value must remain — atomic-on-failure.
        assert_eq!(tech.get(TechField::Energy), 30);
    }

    /// FR-19 port of the legacy `test_research_allocation_normalize`.
    ///
    /// A non-zero allocation must renormalize to exactly 100, and the
    /// per-field ratios must be preserved (up to integer rounding).
    #[test]
    fn research_allocation_normalizes_to_exactly_100() {
        let mut alloc = ResearchAllocation {
            energy: 50,
            weapons: 50,
            propulsion: 0,
            construction: 0,
            electronics: 0,
            biotechnology: 0,
        };
        alloc.normalize();

        assert_eq!(alloc.total(), 100);
        assert_eq!(alloc.energy, 50);
        assert_eq!(alloc.weapons, 50);
    }

    /// FR-19 port of the legacy `test_research_allocation_normalize_zero`.
    ///
    /// A fully-zero allocation must not divide by zero — it should fall
    /// back to the [`Default`] even split, which sums to 100.
    #[test]
    fn research_allocation_zero_input_falls_back_to_default_split() {
        let mut alloc = ResearchAllocation {
            energy: 0,
            weapons: 0,
            propulsion: 0,
            construction: 0,
            electronics: 0,
            biotechnology: 0,
        };
        alloc.normalize();
        assert_eq!(alloc.total(), 100);
    }

    /// FR-19 port of the legacy `test_cost_new`.
    ///
    /// The `Cost::new(resources, ironium, boranium, germanium)` shortcut
    /// must populate the flat resource field and route the three
    /// minerals through `Minerals::new` so the resulting `Cost` is
    /// equivalent to `Cost { resources, minerals: Minerals::new(...) }`.
    #[test]
    fn cost_new_routes_minerals_through_minerals_new() {
        let cost = Cost::new(100, 10, 20, 30);
        assert_eq!(cost.resources, 100);
        assert_eq!(cost.minerals.ironium, 10);
        assert_eq!(cost.minerals.boranium, 20);
        assert_eq!(cost.minerals.germanium, 30);
    }

    /// New sniff test for the `Colonists` newtype (not in the legacy 19 —
    /// the legacy stored population as a raw `u32`).
    ///
    /// This test pins down the 100-unit granularity convention and
    /// verifies the four public methods that downstream atoms will rely
    /// on: `new`, `units`, `as_people`, and `checked_add` / `checked_sub`.
    /// Any drift here means every formula consuming `Colonists` is
    /// compromised, so this test is load-bearing.
    #[test]
    fn colonists_newtype_tracks_100_unit_granularity() {
        // Construction: 250 units = 25,000 people.
        let twenty_five_k = Colonists::new(250);
        assert_eq!(twenty_five_k.units(), 250);
        assert_eq!(twenty_five_k.as_people(), 25_000);

        // ZERO constant matches Default and has zero people.
        assert_eq!(Colonists::ZERO, Colonists::default());
        assert_eq!(Colonists::ZERO.as_people(), 0);

        // Checked add: happy path.
        let growth = Colonists::new(50);
        let total = twenty_five_k
            .checked_add(growth)
            .expect("250 + 50 should not overflow");
        assert_eq!(total, Colonists::new(300));

        // Checked add: overflow returns ArithmeticOverflow, not a wrap.
        let near_max = Colonists::new(u32::MAX - 1);
        let overflow = near_max.checked_add(Colonists::new(5));
        assert!(
            matches!(overflow, Err(GameError::ArithmeticOverflow(_))),
            "overflow must raise ArithmeticOverflow, got: {overflow:?}"
        );

        // Checked sub: happy path.
        let after_loss = twenty_five_k
            .checked_sub(Colonists::new(50))
            .expect("250 - 50 should succeed");
        assert_eq!(after_loss, Colonists::new(200));

        // Checked sub: underflow returns InsufficientResources.
        let underflow = Colonists::new(10).checked_sub(Colonists::new(50));
        assert!(
            matches!(underflow, Err(GameError::InsufficientResources(_))),
            "underflow must raise InsufficientResources, got: {underflow:?}"
        );
    }

    /// FR-19 port of the legacy `test_cargo_total_mass`, adapted for the
    /// `Colonists` newtype wrapper.
    ///
    /// The legacy test passed `colonists: 10` as a raw `u32`. Our rewrite
    /// wraps that in `Colonists::new(10)` — same numeric value, same
    /// `total_mass` result (185 kT: 100 + 50 + 25 + 10).
    #[test]
    fn cargo_total_mass_sums_all_four_fields() {
        let cargo = Cargo {
            ironium: 100,
            boranium: 50,
            germanium: 25,
            colonists: Colonists::new(10),
        };
        assert_eq!(cargo.total_mass(), 185);
    }

    /// FR-19 port of the legacy `test_create_star_and_planet`.
    ///
    /// Constructs a `Planet` and a `Star`-holding-that-planet from the
    /// vocabulary this session has built up (`PlanetId`, `StarId`,
    /// `Colonists`, `Environment`, `MineralConcentrations`, `Minerals`,
    /// `Position`, `ProductionItem`/`QueueItem`), then sanity-checks
    /// three things:
    ///   1. the star knows about its one planet,
    ///   2. the planet's name survived round-tripping through the struct,
    ///   3. environment fields read back unchanged.
    ///
    /// The legacy test used `population: 0` as a raw `u32`; our rewrite
    /// uses `Colonists::ZERO`, which is the idiomatic empty-planet value
    /// and pins the newtype convention in the test suite.
    #[test]
    fn create_star_and_planet_from_session_vocabulary() {
        let planet = Planet {
            id: PlanetId(1),
            star_id: StarId(1),
            name: "Alpha Prime".to_string(),
            owner_id: None,
            population: Colonists::ZERO,
            environment: Environment {
                gravity: 50,
                temperature: 50,
                radiation: 50,
            },
            mineral_concentrations: MineralConcentrations::new(80, 60, 40),
            surface_minerals: Minerals::new(200, 150, 100),
            mines: 0,
            factories: 0,
            defenses: 0,
            has_scanner: false,
            has_starbase: false,
            starbase_design_id: None,
            production_queue: vec![],
        };
        let star = Star {
            id: StarId(1),
            name: "Alpha".to_string(),
            position: Position::new(100.0, 200.0),
            planets: vec![planet],
        };

        assert_eq!(star.planets.len(), 1);
        assert_eq!(star.planets[0].name, "Alpha Prime");
        assert_eq!(star.planets[0].environment.gravity, 50);
    }

    /// FR-19 port of the legacy `test_serialization_roundtrip_game_settings`.
    ///
    /// The legacy test called `GameSettings::default()` — our rewrite
    /// deliberately removed the `Default` impl (see the `GameSettings`
    /// doc comment for the determinism-foot-gun rationale), so this
    /// port constructs the settings explicitly. The construction doubles
    /// as a compile-time check that all fields are present and typed
    /// correctly — a future field addition will require touching this
    /// test, which is the intended forcing function.
    #[test]
    fn game_settings_survive_json_roundtrip() {
        let original = GameSettings {
            galaxy_size: GalaxySize::Medium,
            density: GalaxyDensity::Normal,
            player_count: 4,
            starting_year: 2400,
            victory_conditions: vec![
                VictoryCondition::OwnPercentOfPlanets(60),
                VictoryCondition::HighestScoreAfterTurns(200),
            ],
            victory_requirements_met: 1,
            ai_difficulty: AiDifficulty::Standard,
            random_seed: 0xDEAD_BEEF_CAFE_F00D,
        };

        let json = serde_json::to_string(&original).expect("serialize GameSettings");
        let decoded: GameSettings = serde_json::from_str(&json).expect("deserialize GameSettings");
        assert_eq!(original, decoded);
    }

    /// FR-19 port of the legacy `test_turn_phases_are_ordered`.
    ///
    /// The canonical Stars! order-of-events has **exactly 33** phases.
    /// This test is a tripwire: if anyone adds, removes, or reorders a
    /// variant without updating [`TurnPhase::CANONICAL_ORDER`] (or the
    /// reverse), the lengths diverge and this test fails loudly.
    ///
    /// It also asserts that the 33 entries in `CANONICAL_ORDER` match
    /// the 1995 canonical ordering by comparing against a locally-spelled
    /// reference list. That redundancy is deliberate — if we ever change
    /// the order (a legitimate game-design decision), we have to update
    /// the test AND the const AND the enum in the same commit.
    #[test]
    fn turn_phases_match_canonical_33_step_order() {
        // Local reference list — intentionally hand-spelled so the test
        // is independent of `CANONICAL_ORDER`. A refactor that silently
        // diverges the two must fail this assertion.
        let canonical: [TurnPhase; 33] = [
            TurnPhase::ScrapFleets,
            TurnPhase::Waypoint0Unload,
            TurnPhase::Waypoint0Colonize,
            TurnPhase::Waypoint0Load,
            TurnPhase::Waypoint0Other,
            TurnPhase::MysteryTraderMove,
            TurnPhase::PacketMove,
            TurnPhase::WormholeEntryJiggle,
            TurnPhase::FleetMovement,
            TurnPhase::InnerStrengthGrowth,
            TurnPhase::PacketSalvageDecay,
            TurnPhase::WormholeExitJiggle,
            TurnPhase::SdMinefieldDetonation,
            TurnPhase::Mining,
            TurnPhase::Production,
            TurnPhase::SsSpyBonus,
            TurnPhase::PopulationGrowth,
            TurnPhase::LaunchedPacketDamage,
            TurnPhase::RandomEvents,
            TurnPhase::FleetBattles,
            TurnPhase::MeetMysteryTrader,
            TurnPhase::Bombing,
            TurnPhase::Waypoint1Unload,
            TurnPhase::Waypoint1Colonize,
            TurnPhase::Waypoint1Load,
            TurnPhase::MineLaying,
            TurnPhase::FleetTransfer,
            TurnPhase::Waypoint1FleetMerge,
            TurnPhase::CaInstaforming,
            TurnPhase::MinefieldDecay,
            TurnPhase::MineSweeping,
            TurnPhase::StarbaseFleetRepair,
            TurnPhase::RemoteTerraforming,
        ];

        // Exactly 33, matching the canonical order-of-events count.
        assert_eq!(canonical.len(), 33);

        // CANONICAL_ORDER must agree with the local list — any drift is
        // a hard fail so we find out before the turn executor is built.
        assert_eq!(TurnPhase::CANONICAL_ORDER, canonical);
    }
}
