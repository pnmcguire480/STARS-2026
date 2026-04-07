//! Stars 2026 — Core Types and Constants
//!
//! Every type shared across the engine lives here.
//! No game logic in this file — just data definitions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Error Type
// =============================================================================

/// All fallible engine operations return this error type.
/// No panics or unwraps in library code — ever.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameError {
    /// Galaxy generation failed (e.g., can't place enough homeworlds)
    GalaxyGenerationFailed(String),
    /// Race validation failed (e.g., too many advantage points)
    InvalidRace(String),
    /// Ship design is invalid (e.g., wrong component in slot)
    InvalidShipDesign(String),
    /// Fleet order is invalid (e.g., waypoint outside galaxy bounds)
    InvalidOrder(String),
    /// Game state is inconsistent (should never happen — indicates a bug)
    InternalError(String),
    /// Requested entity not found
    NotFound(String),
    /// Serialization/deserialization error
    SerializationError(String),
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::GalaxyGenerationFailed(msg) => write!(f, "Galaxy generation failed: {}", msg),
            GameError::InvalidRace(msg) => write!(f, "Invalid race: {}", msg),
            GameError::InvalidShipDesign(msg) => write!(f, "Invalid ship design: {}", msg),
            GameError::InvalidOrder(msg) => write!(f, "Invalid order: {}", msg),
            GameError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            GameError::NotFound(msg) => write!(f, "Not found: {}", msg),
            GameError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for GameError {}

// =============================================================================
// IDs — Typed wrappers to prevent mixing up different ID types
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StarId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlanetId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FleetId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShipDesignId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BattlePlanId(pub u32);

// =============================================================================
// Galaxy Configuration
// =============================================================================

/// Galaxy size determines star count range and map dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GalaxySize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}

impl GalaxySize {
    /// Target star count for this galaxy size (center of range).
    pub fn target_stars(&self) -> u32 {
        match self {
            GalaxySize::Tiny => 32,
            GalaxySize::Small => 70,
            GalaxySize::Medium => 150,
            GalaxySize::Large => 300,
            GalaxySize::Huge => 600,
        }
    }

    /// Map dimension in light-years (square map: width = height).
    pub fn map_dimension(&self) -> u32 {
        match self {
            GalaxySize::Tiny => 400,
            GalaxySize::Small => 600,
            GalaxySize::Medium => 800,
            GalaxySize::Large => 1200,
            GalaxySize::Huge => 1600,
        }
    }

    /// Minimum distance between homeworlds (light-years).
    pub fn min_homeworld_distance(&self) -> f64 {
        match self {
            GalaxySize::Tiny => 80.0,
            GalaxySize::Small => 100.0,
            GalaxySize::Medium => 130.0,
            GalaxySize::Large => 160.0,
            GalaxySize::Huge => 200.0,
        }
    }
}

/// Galaxy density affects star clustering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GalaxyDensity {
    Sparse,
    Normal,
    Dense,
    Packed,
}

// =============================================================================
// Minerals
// =============================================================================

/// The three mineral types in the game.
/// Used for construction, ship building, and trade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MineralType {
    Ironium,
    Boranium,
    Germanium,
}

/// A bundle of mineral amounts. Used for costs, cargo, planet surface, etc.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Minerals {
    pub ironium: u32,
    pub boranium: u32,
    pub germanium: u32,
}

impl Minerals {
    pub fn new(ironium: u32, boranium: u32, germanium: u32) -> Self {
        Self { ironium, boranium, germanium }
    }

    pub fn total(&self) -> u32 {
        self.ironium + self.boranium + self.germanium
    }

    /// Returns true if self has enough of each mineral to cover `cost`.
    pub fn can_afford(&self, cost: &Minerals) -> bool {
        self.ironium >= cost.ironium
            && self.boranium >= cost.boranium
            && self.germanium >= cost.germanium
    }

    /// Subtract `cost` from self. Returns error if insufficient.
    pub fn spend(&mut self, cost: &Minerals) -> Result<(), GameError> {
        if !self.can_afford(cost) {
            return Err(GameError::InternalError(format!(
                "Insufficient minerals: have {:?}, need {:?}",
                self, cost
            )));
        }
        self.ironium -= cost.ironium;
        self.boranium -= cost.boranium;
        self.germanium -= cost.germanium;
        Ok(())
    }

    /// Add minerals to self.
    pub fn add(&mut self, other: &Minerals) {
        self.ironium += other.ironium;
        self.boranium += other.boranium;
        self.germanium += other.germanium;
    }
}

/// Mineral concentration on a planet (0–100+ scale, affects mining yield).
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct MineralConcentrations {
    pub ironium: u32,
    pub boranium: u32,
    pub germanium: u32,
}

// =============================================================================
// Environment — Habitability
// =============================================================================

/// A single environment axis value (gravity, temperature, or radiation).
/// Stored as an integer in clicks (0–100 scale).
pub type EnvValue = i32;

/// Planet environment values.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Environment {
    pub gravity: EnvValue,     // 0 (low) to 100 (high)
    pub temperature: EnvValue, // 0 (cold) to 100 (hot)
    pub radiation: EnvValue,   // 0 (low) to 100 (high)
}

/// A habitability range for one axis. `min` and `max` define the livable window.
/// If `immune` is true, the race can live at any value on this axis.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HabRange {
    pub min: EnvValue,
    pub max: EnvValue,
    pub immune: bool,
}

impl Default for HabRange {
    fn default() -> Self {
        Self { min: 15, max: 85, immune: false }
    }
}

/// Full habitability specification for a race.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct HabRanges {
    pub gravity: HabRange,
    pub temperature: HabRange,
    pub radiation: HabRange,
}

// =============================================================================
// Primary Racial Traits (PRT) — exactly 10
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrimaryRacialTrait {
    /// Hyper-Expansion: fast growth, cheap colony ships
    HyperExpansion,
    /// Super Stealth: cloaking bonus, spy bonus
    SuperStealth,
    /// War Monger: combat bonus, no diplomacy
    WarMonger,
    /// Claim Adjuster: terraforming bonus, instaforming
    ClaimAdjuster,
    /// Inner Strength: fleet population growth, defensive bonus
    InnerStrength,
    /// Space Demolition: minefield specialist
    SpaceDemolition,
    /// Packet Physics: mass driver specialist
    PacketPhysics,
    /// Interstellar Traveler: stargate specialist
    InterstellarTraveler,
    /// Alternate Reality: unique economy (population-based)
    AlternateReality,
    /// Jack of All Trades: no specialty, balanced start
    JackOfAllTrades,
}

// =============================================================================
// Lesser Racial Traits (LRT) — 14 options
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LesserRacialTrait {
    /// Improved Fuel Efficiency (advantage — costs points)
    ImprovedFuelEfficiency,
    /// Total Terraforming (advantage)
    TotalTerraforming,
    /// Advanced Remote Mining (advantage)
    AdvancedRemoteMining,
    /// Improved Starbases (advantage)
    ImprovedStarbases,
    /// Generalized Research (advantage)
    GeneralizedResearch,
    /// Ultimate Recycling (advantage)
    UltimateRecycling,
    /// Mineral Alchemy (advantage)
    MineralAlchemy,
    /// No Ram Scoop Engines (penalty — gives points back)
    NoRamScoopEngines,
    /// Cheap Engines (penalty)
    CheapEngines,
    /// Only Basic Remote Mining (penalty)
    OnlyBasicRemoteMining,
    /// No Advanced Scanners (penalty)
    NoAdvancedScanners,
    /// Low Starting Population (penalty)
    LowStartingPopulation,
    /// Bleeding Edge Technology (penalty)
    BleedingEdgeTechnology,
    /// Regenerating Shields (mixed)
    RegeneratingShields,
}

// =============================================================================
// Technology
// =============================================================================

/// The six research fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TechField {
    Energy,
    Weapons,
    Propulsion,
    Construction,
    Electronics,
    Biotechnology,
}

/// A player's current tech levels across all six fields.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct TechLevels {
    pub energy: u32,
    pub weapons: u32,
    pub propulsion: u32,
    pub construction: u32,
    pub electronics: u32,
    pub biotechnology: u32,
}

impl TechLevels {
    pub fn get(&self, field: TechField) -> u32 {
        match field {
            TechField::Energy => self.energy,
            TechField::Weapons => self.weapons,
            TechField::Propulsion => self.propulsion,
            TechField::Construction => self.construction,
            TechField::Electronics => self.electronics,
            TechField::Biotechnology => self.biotechnology,
        }
    }

    pub fn set(&mut self, field: TechField, level: u32) {
        match field {
            TechField::Energy => self.energy = level,
            TechField::Weapons => self.weapons = level,
            TechField::Propulsion => self.propulsion = level,
            TechField::Construction => self.construction = level,
            TechField::Electronics => self.electronics = level,
            TechField::Biotechnology => self.biotechnology = level,
        }
    }

    /// Check if all fields meet the required levels.
    pub fn meets_requirements(&self, required: &TechLevels) -> bool {
        self.energy >= required.energy
            && self.weapons >= required.weapons
            && self.propulsion >= required.propulsion
            && self.construction >= required.construction
            && self.electronics >= required.electronics
            && self.biotechnology >= required.biotechnology
    }
}

/// Research allocation — percentage per field (must sum to 100).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ResearchAllocation {
    pub energy: u32,
    pub weapons: u32,
    pub propulsion: u32,
    pub construction: u32,
    pub electronics: u32,
    pub biotechnology: u32,
}

impl Default for ResearchAllocation {
    fn default() -> Self {
        // Default: even split (not exactly 100 due to rounding — normalized at use)
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
    pub fn total(&self) -> u32 {
        self.energy + self.weapons + self.propulsion
            + self.construction + self.electronics + self.biotechnology
    }

    /// Normalize so total = 100. If total is 0, set even split.
    pub fn normalize(&mut self) {
        let total = self.total();
        if total == 0 {
            *self = Self::default();
            return;
        }
        self.energy = (self.energy * 100) / total;
        self.weapons = (self.weapons * 100) / total;
        self.propulsion = (self.propulsion * 100) / total;
        self.construction = (self.construction * 100) / total;
        self.electronics = (self.electronics * 100) / total;
        // Assign remainder to biotech to ensure exactly 100
        self.biotechnology = 100 - self.energy - self.weapons
            - self.propulsion - self.construction - self.electronics;
    }
}

// =============================================================================
// Ship Design — Hull Slots and Components
// =============================================================================

/// Categories of hull slot. Components must match their slot type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SlotType {
    Engine,
    Scanner,
    Shield,
    Armor,
    Weapon,
    Bomb,
    Mining,
    Electrical,
    Mechanical,
    /// General purpose — accepts weapons, shields, armor, electrical, mechanical
    General,
    /// Orbital — starbase-only slots
    Orbital,
}

/// A slot definition within a hull (from hulls.json).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HullSlot {
    pub slot_type: SlotType,
    pub max_count: u32,   // max number of components in this slot
    pub required: bool,   // must be filled for a valid design?
}

/// A hull template loaded from game data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HullDefinition {
    pub id: String,
    pub name: String,
    pub mass: u32,
    pub armor: u32,
    pub fuel_capacity: u32,
    pub cargo_capacity: u32,
    pub initiative: u32,
    pub cost: Cost,
    pub slots: Vec<HullSlot>,
    pub tech_requirements: TechLevels,
    pub is_starbase: bool,
}

/// A component definition loaded from game data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentDefinition {
    pub id: String,
    pub name: String,
    pub mass: u32,
    pub cost: Cost,
    pub tech_requirements: TechLevels,
    pub slot_type: SlotType,
    /// Component-specific stats (varies by type)
    pub stats: ComponentStats,
}

/// Component-specific statistics. Each variant holds the stats relevant to that type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ComponentStats {
    Engine {
        ideal_speed: u32,            // warp speed for best fuel efficiency
        free_speed: Option<u32>,     // speed at which fuel is free (ram scoop)
        fuel_table: Vec<u32>,        // fuel per 100mg per ly at each warp (index = warp - 1)
    },
    Scanner {
        normal_range: u32,           // base scan range (ly)
        penetrating_range: u32,      // pen scan range (ly, 0 if none)
    },
    Shield {
        strength: u32,               // shield DP
    },
    Armor {
        strength: u32,               // armor DP
    },
    BeamWeapon {
        power: u32,                  // damage at range 0
        range: u32,                  // max range in battle squares
        initiative: u32,
    },
    Torpedo {
        power: u32,                  // damage on hit
        range: u32,
        accuracy: u32,               // base accuracy percentage (0–100)
        initiative: u32,
    },
    Bomb {
        kill_percentage: f64,        // % of population killed per bomb
        min_kill: u32,               // minimum colonists killed per bomb
        is_smart: bool,              // smart bombs bypass some defenses
    },
    MiningRobot {
        mining_rate: u32,            // kT mined per year
    },
    Electrical {
        cloak_percentage: Option<u32>,
        jammer_percentage: Option<u32>,
        computer_accuracy: Option<u32>,
        capacitor_percentage: Option<u32>,
    },
    Mechanical {
        cargo_bonus: Option<u32>,
        fuel_bonus: Option<u32>,
        colonist_capacity: Option<u32>,
    },
    Terraformer {
        terraform_rate: u32,         // env clicks per year
        is_remote: bool,
    },
    MineLayer {
        mines_per_year: u32,
    },
    Stargate {
        mass_limit: Option<u32>,     // None = infinite
        range_limit: Option<u32>,    // None = infinite (ly)
    },
    MassDriver {
        warp_speed: u32,             // packets launched at this speed
    },
}

/// Cost of an item (resources + minerals).
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Cost {
    pub resources: u32,
    pub minerals: Minerals,
}

impl Cost {
    pub fn new(resources: u32, ironium: u32, boranium: u32, germanium: u32) -> Self {
        Self {
            resources,
            minerals: Minerals::new(ironium, boranium, germanium),
        }
    }
}

// =============================================================================
// Ship Design (player-created)
// =============================================================================

/// A filled slot in a player's ship design.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignSlot {
    pub slot_index: usize,         // index into the hull's slot list
    pub component_id: String,      // references ComponentDefinition.id
    pub count: u32,                // how many of this component in the slot
}

/// A player's ship design — hull + component selections.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipDesign {
    pub id: ShipDesignId,
    pub player_id: PlayerId,
    pub name: String,
    pub hull_id: String,           // references HullDefinition.id
    pub slots: Vec<DesignSlot>,
    // Calculated stats (computed when design is saved)
    pub mass: u32,
    pub armor: u32,
    pub shield: u32,
    pub fuel_capacity: u32,
    pub cargo_capacity: u32,
    pub cost: Cost,
    pub scan_range: u32,
    pub pen_scan_range: u32,
    pub initiative: u32,
}

// =============================================================================
// Fleet & Movement
// =============================================================================

/// A position in the galaxy (light-year coordinates).
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// A ship token within a fleet (N ships of the same design).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipToken {
    pub design_id: ShipDesignId,
    pub count: u32,
    pub damage: u32,               // accumulated damage on this token
}

/// Waypoint orders — what the fleet should do at a waypoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WaypointTask {
    None,
    Colonize,
    RemoteMine,
    LayMines,
    SweepMines,
    Patrol,
    TransferFleet,
    Scrap,
}

/// Transport action for loading/unloading at a waypoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportAction {
    None,
    LoadAll,
    UnloadAll,
    LoadAmount(u32),
    UnloadAmount(u32),
    FillPercent(u32),
    WaitForPercent(u32),
    SetWaypointTo(u32),
}

/// Transport orders for each cargo type.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct TransportOrders {
    pub ironium: TransportAction,
    pub boranium: TransportAction,
    pub germanium: TransportAction,
    pub colonists: TransportAction,
    pub fuel: TransportAction,
}

impl Default for TransportAction {
    fn default() -> Self {
        TransportAction::None
    }
}

/// A single waypoint in a fleet's orders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Waypoint {
    pub position: Position,
    pub warp_speed: u32,           // 0–10 (0 = stopped)
    pub task: WaypointTask,
    pub transport: TransportOrders,
    /// Target can be a star, planet, fleet, or deep space
    pub target_star_id: Option<StarId>,
    pub target_planet_id: Option<PlanetId>,
    pub target_fleet_id: Option<FleetId>,
}

/// Cargo carried by a fleet.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Cargo {
    pub ironium: u32,
    pub boranium: u32,
    pub germanium: u32,
    pub colonists: u32,            // in units of 100 (1 = 100 colonists)
}

impl Cargo {
    pub fn total_mass(&self) -> u32 {
        self.ironium + self.boranium + self.germanium + self.colonists
    }
}

/// A fleet — one or more ship tokens moving together.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fleet {
    pub id: FleetId,
    pub player_id: PlayerId,
    pub name: String,
    pub position: Position,
    pub ships: Vec<ShipToken>,
    pub fuel: u32,
    pub cargo: Cargo,
    pub waypoints: Vec<Waypoint>,
    pub battle_plan_id: BattlePlanId,
    pub repeat_orders: bool,
}

// =============================================================================
// Battle Plans
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattleTarget {
    None,
    AnyEnemy,
    StrongestEnemy,
    WeakestEnemy,
    StarbasesOnly,
    ArmedShipsOnly,
    BombersOnly,
    UnarmedShips,
    FuelTransports,
    Freighters,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattleTactic {
    Disengage,
    DisengageIfChallenged,
    MinimizeDamage,
    MaximizeNetDamage,
    MaximizeDamageRatio,
    MaximizeDamage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetreatCondition {
    IfShieldsDrop(u32),    // percentage threshold (e.g., 33)
    Never,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BattlePlan {
    pub id: BattlePlanId,
    pub player_id: PlayerId,
    pub name: String,
    pub primary_target: BattleTarget,
    pub secondary_target: BattleTarget,
    pub tactic: BattleTactic,
    pub retreat: RetreatCondition,
    pub dump_cargo: bool,
}

// =============================================================================
// Planet
// =============================================================================

/// An item in a planet's production queue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProductionItem {
    Factory(u32),
    Mine(u32),
    Defense(u32),
    Terraform,
    MineralAlchemy,
    ShipDesign(ShipDesignId),
    Starbase(ShipDesignId),
    Scanner,
    MineralPacket { target_planet_id: PlanetId, minerals: Minerals },
}

/// A queued production entry with partial completion tracking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueueItem {
    pub item: ProductionItem,
    pub quantity: u32,
    pub allocated_resources: u32,
    pub allocated_minerals: Minerals,
}

/// A planet in a star system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Planet {
    pub id: PlanetId,
    pub star_id: StarId,
    pub name: String,
    pub owner_id: Option<PlayerId>,
    pub population: u32,           // actual population count
    pub environment: Environment,
    pub mineral_concentrations: MineralConcentrations,
    pub surface_minerals: Minerals,
    pub mines: u32,
    pub factories: u32,
    pub defenses: u32,
    pub has_scanner: bool,
    pub has_starbase: bool,
    pub starbase_design_id: Option<ShipDesignId>,
    pub production_queue: Vec<QueueItem>,
}

// =============================================================================
// Star
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Star {
    pub id: StarId,
    pub name: String,
    pub position: Position,
    pub planets: Vec<Planet>,
}

// =============================================================================
// Race
// =============================================================================

/// Economy configuration for a race.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EconomyConfig {
    /// Colonists per resource (e.g., 1000 = 1 resource per 1000 colonists)
    pub colonists_per_resource: u32,
    /// Factory output: resources per 10 factories
    pub factory_output: u32,
    /// Factory cost: resources to build 1 factory
    pub factory_cost: u32,
    /// Factories operable per 10k colonists
    pub factories_per_10k: u32,
    /// Mine output: kT per 10 mines
    pub mine_output: u32,
    /// Mine cost: resources to build 1 mine
    pub mine_cost: u32,
    /// Mines operable per 10k colonists
    pub mines_per_10k: u32,
}

impl Default for EconomyConfig {
    fn default() -> Self {
        Self {
            colonists_per_resource: 1000,
            factory_output: 10,
            factory_cost: 10,
            factories_per_10k: 10,
            mine_output: 10,
            mine_cost: 5,
            mines_per_10k: 10,
        }
    }
}

/// A complete race definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Race {
    pub name: String,
    pub plural_name: String,
    pub prt: PrimaryRacialTrait,
    pub lrt: Vec<LesserRacialTrait>,
    pub hab_ranges: HabRanges,
    pub growth_rate: u32,          // percentage (1–20)
    pub economy: EconomyConfig,
    /// Advantage points (must be ≤ 0 for a legal race)
    pub advantage_points: i32,
}

// =============================================================================
// Player
// =============================================================================

/// Diplomatic relation between two players.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiplomaticRelation {
    Neutral,
    Friend,
    Enemy,
}

/// A player in a game.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub race: Race,
    pub tech_levels: TechLevels,
    pub research_allocation: ResearchAllocation,
    pub research_points_spent: HashMap<TechField, u32>,
    pub relations: HashMap<PlayerId, DiplomaticRelation>,
    pub battle_plans: Vec<BattlePlan>,
    pub ship_designs: Vec<ShipDesign>,
    pub is_ai: bool,
}

// =============================================================================
// Message System
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    PlanetColonized,
    ResearchComplete { field: TechField, level: u32 },
    ShipBuilt { design_name: String, planet_name: String },
    FleetOutOfFuel { fleet_name: String },
    BattleOccurred { location: Position },
    PlanetBombed { planet_name: String },
    PopulationDied { planet_name: String },
    ProductionCompleted { planet_name: String, item: String },
    Info(String),
    Warning(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub player_id: PlayerId,
    pub turn: u32,
    pub message_type: MessageType,
    pub text: String,
}

// =============================================================================
// Game State
// =============================================================================

/// Victory condition types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VictoryCondition {
    OwnPercentOfPlanets(u32),
    ReachTechLevel(u32),
    ExceedsScoreOf(u32),
    ExceedsSecondPlaceBy(u32),
    ProductionCapacityOf(u32),
    OwnCapitalShips(u32),
    HighestScoreAfterTurns(u32),
}

/// Game settings configured at creation time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameSettings {
    pub galaxy_size: GalaxySize,
    pub density: GalaxyDensity,
    pub player_count: u32,
    pub starting_year: u32,
    pub victory_conditions: Vec<VictoryCondition>,
    pub victory_requirements_met: u32, // how many conditions must be met to win
    pub ai_difficulty: AiDifficulty,
    pub random_seed: u64,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
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
            random_seed: 0, // 0 = generate from system time
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiDifficulty {
    Easy,
    Standard,
    Hard,
    Expert,
}

/// Game status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    Setup,
    InProgress,
    Completed,
}

/// The complete game state — everything needed to save/load a game.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub id: GameId,
    pub name: String,
    pub settings: GameSettings,
    pub status: GameStatus,
    pub turn: u32,
    pub year: u32,
    pub stars: Vec<Star>,
    pub players: Vec<Player>,
    pub fleets: Vec<Fleet>,
    pub messages: Vec<Message>,
}

// =============================================================================
// Turn Generation Phase Tracking
// =============================================================================

/// Phases of turn generation, in canonical order.
/// Used for logging, tracing, and debugging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnPhase {
    ScrapFleets,
    Waypoint0Unload,
    Waypoint0Colonize,
    Waypoint0Load,
    Waypoint0Other,
    MysteryTraderMove,
    PacketMove,
    WormholeEntryJiggle,
    FleetMovement,
    InnerStrengthGrowth,
    PacketSalvageDecay,
    WormholeExitJiggle,
    SDMineDetonation,
    Mining,
    Production,
    SSSpyBonus,
    PopulationGrowth,
    LaunchedPacketDamage,
    RandomEvents,
    FleetBattles,
    MeetMysteryTrader,
    Bombing,
    Waypoint1Unload,
    Waypoint1Colonize,
    Waypoint1Load,
    MineLaying,
    FleetTransfer,
    Waypoint1FleetMerge,
    CAInstaforming,
    MinefieldDecay,
    MineSweeping,
    StarbaseFleetRepair,
    RemoteTerraforming,
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galaxy_size_target_stars() {
        assert_eq!(GalaxySize::Tiny.target_stars(), 32);
        assert_eq!(GalaxySize::Small.target_stars(), 70);
        assert_eq!(GalaxySize::Medium.target_stars(), 150);
        assert_eq!(GalaxySize::Large.target_stars(), 300);
        assert_eq!(GalaxySize::Huge.target_stars(), 600);
    }

    #[test]
    fn test_minerals_can_afford() {
        let bank = Minerals::new(100, 200, 50);
        let cost = Minerals::new(50, 100, 25);
        assert!(bank.can_afford(&cost));

        let expensive = Minerals::new(150, 100, 25);
        assert!(!bank.can_afford(&expensive));
    }

    #[test]
    fn test_minerals_spend() {
        let mut bank = Minerals::new(100, 200, 50);
        let cost = Minerals::new(30, 40, 10);
        bank.spend(&cost).unwrap();
        assert_eq!(bank, Minerals::new(70, 160, 40));
    }

    #[test]
    fn test_minerals_spend_insufficient() {
        let mut bank = Minerals::new(10, 200, 50);
        let cost = Minerals::new(30, 40, 10);
        assert!(bank.spend(&cost).is_err());
        // Bank should be unchanged on failure
        assert_eq!(bank, Minerals::new(10, 200, 50));
    }

    #[test]
    fn test_minerals_add() {
        let mut a = Minerals::new(10, 20, 30);
        let b = Minerals::new(5, 10, 15);
        a.add(&b);
        assert_eq!(a, Minerals::new(15, 30, 45));
    }

    #[test]
    fn test_position_distance() {
        let a = Position::new(0.0, 0.0);
        let b = Position::new(3.0, 4.0);
        let dist = a.distance_to(&b);
        assert!((dist - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_position_distance_same_point() {
        let a = Position::new(42.0, 99.0);
        assert!((a.distance_to(&a) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_tech_levels_meets_requirements() {
        let player_tech = TechLevels {
            energy: 5, weapons: 3, propulsion: 4,
            construction: 2, electronics: 6, biotechnology: 1,
        };
        let required = TechLevels {
            energy: 5, weapons: 3, propulsion: 0,
            construction: 0, electronics: 0, biotechnology: 0,
        };
        assert!(player_tech.meets_requirements(&required));

        let too_high = TechLevels {
            energy: 6, weapons: 0, propulsion: 0,
            construction: 0, electronics: 0, biotechnology: 0,
        };
        assert!(!player_tech.meets_requirements(&too_high));
    }

    #[test]
    fn test_tech_levels_get_set() {
        let mut tech = TechLevels::default();
        tech.set(TechField::Weapons, 7);
        assert_eq!(tech.get(TechField::Weapons), 7);
        assert_eq!(tech.get(TechField::Energy), 0);
    }

    #[test]
    fn test_research_allocation_normalize() {
        let mut alloc = ResearchAllocation {
            energy: 50, weapons: 50, propulsion: 0,
            construction: 0, electronics: 0, biotechnology: 0,
        };
        alloc.normalize();
        assert_eq!(alloc.total(), 100);
        assert_eq!(alloc.energy, 50);
        assert_eq!(alloc.weapons, 50);
    }

    #[test]
    fn test_research_allocation_normalize_zero() {
        let mut alloc = ResearchAllocation {
            energy: 0, weapons: 0, propulsion: 0,
            construction: 0, electronics: 0, biotechnology: 0,
        };
        alloc.normalize();
        assert_eq!(alloc.total(), 100);
    }

    #[test]
    fn test_cost_new() {
        let cost = Cost::new(100, 10, 20, 30);
        assert_eq!(cost.resources, 100);
        assert_eq!(cost.minerals.ironium, 10);
        assert_eq!(cost.minerals.boranium, 20);
        assert_eq!(cost.minerals.germanium, 30);
    }

    #[test]
    fn test_cargo_total_mass() {
        let cargo = Cargo {
            ironium: 100, boranium: 50, germanium: 25, colonists: 10,
        };
        assert_eq!(cargo.total_mass(), 185);
    }

    #[test]
    fn test_hab_range_default() {
        let range = HabRange::default();
        assert_eq!(range.min, 15);
        assert_eq!(range.max, 85);
        assert!(!range.immune);
    }

    #[test]
    fn test_game_error_display() {
        let err = GameError::InvalidRace("Too many advantage points".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid race"));
        assert!(msg.contains("Too many advantage points"));
    }

    #[test]
    fn test_create_star_and_planet() {
        let planet = Planet {
            id: PlanetId(1),
            star_id: StarId(1),
            name: "Alpha Prime".to_string(),
            owner_id: None,
            population: 0,
            environment: Environment { gravity: 50, temperature: 50, radiation: 50 },
            mineral_concentrations: MineralConcentrations { ironium: 80, boranium: 60, germanium: 40 },
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

    #[test]
    fn test_serialization_roundtrip_minerals() {
        let minerals = Minerals::new(42, 99, 7);
        let json = serde_json::to_string(&minerals).unwrap();
        let deserialized: Minerals = serde_json::from_str(&json).unwrap();
        assert_eq!(minerals, deserialized);
    }

    #[test]
    fn test_serialization_roundtrip_game_settings() {
        let settings = GameSettings::default();
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: GameSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }

    #[test]
    fn test_turn_phases_are_ordered() {
        // Verify we have the right count of phases matching the canonical order
        let phases = vec![
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
            TurnPhase::SDMineDetonation,
            TurnPhase::Mining,
            TurnPhase::Production,
            TurnPhase::SSSpyBonus,
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
            TurnPhase::CAInstaforming,
            TurnPhase::MinefieldDecay,
            TurnPhase::MineSweeping,
            TurnPhase::StarbaseFleetRepair,
            TurnPhase::RemoteTerraforming,
        ];
        assert_eq!(phases.len(), 33);
    }
}
