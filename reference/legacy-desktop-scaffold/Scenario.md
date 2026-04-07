# Scenario Suite: Stars 2026

**Purpose:** External behavioral scenarios that validate outcomes. These live OUTSIDE the codebase the generator reads. Treat as holdout evaluation. The builder model must never see these during implementation.

---

## Rules

1. Write scenarios as black-box behavior: inputs, user actions, expected outputs.
2. No internal implementation hints.
3. Include happy path, edge cases, abuse cases, and failure injection.
4. Each scenario has an ID and is runnable in CI against a built artifact.
5. **Sniff Test Integration:** Every new function triggers the relevant scenario subset. Not just the function's unit test — the scenario validates end-to-end behavior.

---

## Naming convention

```
SC-<area>-<number>-<short_name>
```

Areas: `galaxy`, `race`, `planet`, `tech`, `ship`, `fleet`, `combat`, `turn`, `ui`, `perf`, `save`, `multi`, `sec`, `fail`

---

## Environment assumptions

- **Runtime:** Local dev build (Vite dev server + WASM module)
- **Test doubles:** Seeded random number generator (seed = test name hash) for deterministic galaxy generation
- **Seed data:** Each scenario specifies its own initial game state as a JSON fixture

---

## Core Scenarios

### SC-galaxy-001-generation_sizes: Galaxy generates correct star counts per size setting

**Intent:** Verify procedural galaxy generation produces the right number of stars for each size option.

**Given:**
- New game creation screen
- No existing game state

**When:**
- Player creates a new game with size "Tiny"
- Repeat for "Small", "Medium", "Large", "Huge"

**Then:**

*Expected outputs:*
- Tiny: 24–40 stars
- Small: 55–85 stars
- Medium: 120–180 stars
- Large: 240–360 stars
- Huge: 480–720 stars
- Every star has at least 1 planet
- No two stars occupy the same coordinates
- All stars are within the defined galaxy bounds

*Invariants:*
- Galaxy generation is deterministic for a given seed
- Same seed + same settings = identical galaxy every time

---

### SC-galaxy-002-homeworld_assignment: Each player gets a valid homeworld

**Intent:** Verify homeworld assignment gives every player a habitable starting planet.

**Given:**
- 4-player game, Medium galaxy
- Players have different race hab ranges

**When:**
- Game generates and assigns homeworlds

**Then:**

*Expected outputs:*
- Each player has exactly 1 homeworld
- Homeworld hab value is 100% for that player's race
- Homeworld has starting population per PRT rules
- Homeworlds are minimum distance apart (no two homeworlds adjacent)
- Starting mineral concentrations on homeworld are within documented ranges

*Invariants:*
- No player starts without a homeworld
- No two players share a homeworld

---

### SC-race-001-point_balance_legal: Legal race validates with ≤ 0 advantage points

**Intent:** Verify the race point calculator correctly balances a complex custom race.

**Given:**
- Race designer open
- PRT: Hyper-Expansion
- LRT: Improved Fuel Efficiency, Only Basic Remote Mining
- Hab ranges: Gravity 15–85, Temp 15–85, Rad 15–85
- Growth rate: 15%
- Factory efficiency: 10/10/15, Mine efficiency: 10/5/10

**When:**
- Player finishes configuring and clicks "Validate"

**Then:**

*Expected outputs:*
- Advantage points displayed
- Points ≤ 0 (legal race)
- "Start Game" button is enabled

*Invariants:*
- Point calculation is deterministic
- Same inputs always produce same point value

---

### SC-race-002-illegal_race_rejected: Overpowered race is blocked

**Intent:** Verify that a race with too many advantages and too few penalties is rejected.

**Given:**
- Race designer open
- PRT: Jack of All Trades
- All LRTs that give advantages selected, none that give penalties
- Widest possible hab ranges, max growth, max factory/mine efficiency

**When:**
- Player clicks "Validate"

**Then:**

*Expected outputs:*
- Advantage points displayed as positive (illegal)
- Warning message: "Race has too many advantage points. Add penalties or narrow your hab range."
- "Start Game" button is disabled

---

### SC-planet-001-production_queue: Planet builds items in queue order

**Intent:** Verify production queue processes correctly across turns.

**Given:**
- Player owns a planet with 100k population, 50 factories, 200 resources/turn
- Production queue: [10 Factories, 5 Mines, 1 Scout ship]
- Sufficient minerals on planet surface

**When:**
- 3 turns generate

**Then:**

*Expected outputs:*
- Factories build first (cheapest items), factory count increases
- Mines build next
- Scout ship builds last (if resources remain)
- Queue items are removed as completed
- Minerals are deducted from planet surface per item cost
- Resources consumed match the sum of completed items

*Invariants:*
- Production never exceeds available resources + minerals
- Partially built items carry over between turns (not lost)

---

### SC-planet-002-population_growth: Population grows per hab value formula

**Intent:** Verify population growth follows the documented formula.

**Given:**
- Planet with 50k population, 100% hab value, growth rate 15%
- No crowding (population well below planet capacity)

**When:**
- 1 turn generates

**Then:**

*Expected outputs:*
- New population = old population + (old population × growth_rate × hab_factor)
- Growth matches formula to within 1 colonist (rounding)

*Expected side effects:*
- Planet population field updated
- No population appears/disappears from nowhere

---

### SC-planet-003-zero_population: Planet with 0 population reverts to unowned

**Intent:** Verify that a planet losing all population correctly becomes uncolonized.

**Given:**
- Player owns a planet with 100 population
- Planet has 0% hab value (killing environment)

**When:**
- Turn generates (population dies from inhospitable environment)

**Then:**

*Expected outputs:*
- Population reaches 0
- Planet owner set to null/unowned
- Existing structures (factories, mines) remain on planet
- Planet is visible as unowned on scanner

*Invariants:*
- Structures are not destroyed when ownership changes
- No ghost population remains

---

### SC-tech-001-research_advances_level: Research spending advances tech level

**Intent:** Verify research allocation produces tech level increases at the correct thresholds.

**Given:**
- Player at Tech Level 3 in Weapons
- Research allocation: 100% to Weapons
- Total research resources: 500/turn
- Cost to reach Level 4: known threshold from tech cost table

**When:**
- Enough turns generate to accumulate required research points

**Then:**

*Expected outputs:*
- Weapons tech level advances from 3 to 4
- Research points reset for next level (overflow carries over)
- Message generated: "You have reached Weapons Tech Level 4"
- New components unlocked at Weapons 4 are now available in ship designer

---

### SC-ship-001-design_valid: Ship designer creates valid design with correct stats

**Intent:** Verify ship designer calculates derived stats correctly.

**Given:**
- Player has Tech: Energy 5, Weapons 3, Propulsion 4, Construction 3, Electronics 3, Biotech 1
- Ship designer open

**When:**
- Player selects Scout hull
- Adds Fuel Mizer engine to engine slot
- Adds Bat Scanner to scanner slot (if tech allows)
- Adds Laser to weapon slot
- Saves design as "Alpha Scout"

**Then:**

*Expected outputs:*
- Design saved successfully
- Calculated mass = hull mass + sum of component masses
- Fuel capacity = hull fuel + engine fuel bonus (if any)
- Scan range = scanner range value
- Cost = sum of all component costs (resources, ironium, boranium, germanium)
- Design appears in production queue component list

*Invariants:*
- Components not meeting tech requirements are not selectable
- Wrong-type components cannot be placed in incompatible slots

---

### SC-ship-002-no_engine_design: Ship with no engine has 0 speed

**Intent:** Verify edge case where a ship design has no engine.

**Given:**
- Ship designer open, Starbase hull selected (no engine slot required)

**When:**
- Player creates design with weapons and shields but no engine
- Design is assigned to a fleet

**Then:**

*Expected outputs:*
- Fleet speed = 0
- Fleet cannot be given move waypoint orders
- Fleet can still fire weapons if enemies arrive at its location

---

### SC-fleet-001-movement: Fleet moves correct distance per warp speed

**Intent:** Verify fleet movement calculation.

**Given:**
- Fleet at coordinates (100, 100)
- Waypoint set to star at (149, 100) — distance = 49 ly
- Fleet speed: Warp 7 (moves 49 ly/turn)

**When:**
- 1 turn generates

**Then:**

*Expected outputs:*
- Fleet arrives at (149, 100)
- Fleet is at the destination star
- Fuel consumed = calculated fuel for 49ly at Warp 7 for fleet mass

---

### SC-fleet-002-fuel_exhaustion: Fleet runs out of fuel mid-journey

**Intent:** Verify fleet behavior when fuel runs out.

**Given:**
- Fleet with 100mg fuel, needs 200mg to reach destination at current warp
- Waypoint set to distant star

**When:**
- Turn generates

**Then:**

*Expected outputs:*
- Fleet moves as far as fuel allows
- Fleet stops at intermediate position (deep space)
- Fleet fuel = 0
- Fleet is NOT destroyed or removed
- Message generated: "Fleet [name] has run out of fuel"

*Invariants:*
- Fleet with 0 fuel still exists and is scannable
- Fleet can be rescued by another fleet with fuel transfer orders

---

### SC-combat-001-basic_engagement: Two fleets fight, one wins

**Intent:** Verify basic combat resolution between two opposing fleets.

**Given:**
- Player A fleet: 3 Destroyers with lasers and shields
- Player B fleet: 1 Scout with no weapons
- Both fleets at same star, players are enemies

**When:**
- Turn generates, combat phase triggers

**Then:**

*Expected outputs:*
- Battle occurs
- Player B's scout is destroyed (no weapons to fight back)
- Player A's destroyers survive (overwhelming force)
- Battle report generated for both players
- Destroyed ship is removed from fleet
- If fleet has 0 ships, fleet is removed

*Invariants:*
- Damage calculations follow beam attenuation formula
- Shields absorb damage before armor
- Initiative determines attack order correctly

---

### SC-combat-002-retreat: Fleet retreats when conditions met

**Intent:** Verify retreat mechanic works per battle plan settings.

**Given:**
- Player A fleet: 2 Cruisers, battle plan retreat condition: "if shields < 33%"
- Player B fleet: 5 Battleships (overwhelming force)
- Same location

**When:**
- Turn generates, combat triggers, damage accumulates

**Then:**

*Expected outputs:*
- After shields drop below 33%, Player A's surviving ships attempt retreat
- Retreating ships move away from battle
- Battle report shows retreat event
- Retreated fleet appears at a location 1 turn's travel away from battle

---

### SC-turn-001-order_of_events: Turn phases execute in canonical order

**Intent:** Verify the complete turn generation sequence matches the Stars! order of events.

**Given:**
- Active game at turn 50 with multiple players, fleets, planets, research, combat situations

**When:**
- All players submit orders, turn generates

**Then:**

*Expected outputs:*
- Phases execute in this exact order:
  1. Waypoint 0 unload tasks
  2. Waypoint 0 load tasks
  3. Other waypoint 0 tasks
  4. MT genesis (if applicable)
  5. Fleet movement (+ mine hits)
  6. Inner planet production
  7. Player research
  8. Mining (remote + planet)
  9. Production (planets build queue items)
  10. Population growth
  11. Packet movement
  12. Fleet battles
  13. Bombing
  14. Waypoint 1 unload/load/colonize
  15. Starbase refuel
  16. Minefield decay
  17. Mine sweep
  18. Scanner/cloak recalculation

*Invariants:*
- Each phase's outputs are inputs to subsequent phases
- Changing the order would produce different results (validates ordering matters)

---

### SC-save-001-save_load_integrity: Game saves and loads without data loss

**Intent:** Verify save/load round-trips preserve all game state.

**Given:**
- Active game at turn 25, multiple planets colonized, fleets in motion, tech researched, production queues set

**When:**
- Player saves game
- Player loads the saved game

**Then:**

*Expected outputs:*
- All planet data matches pre-save state
- All fleet positions, cargo, waypoints match
- All tech levels match
- All production queues match
- Turn number matches
- Galaxy map renders identically

*Invariants:*
- Save file size is reasonable (< 5MB for Huge galaxy at turn 100)
- Save file is valid JSON/schema
- No floating point drift between save and load

---

### SC-perf-001-turn_gen_huge: Turn generation meets performance target

**Intent:** Verify turn generation performance at scale.

**Given:**
- 16-player Huge galaxy, turn 100
- Each player has 30+ planets, 50+ fleets, multiple combat encounters queued

**When:**
- Turn generates

**Then:**

*Expected outputs:*
- Turn completes in < 2 seconds on reference hardware (i7-10700F, 48GB RAM)
- No memory leaks (heap stays bounded)
- UI remains responsive during generation (generation runs in web worker / separate thread)

---

### SC-ui-001-mobile_responsive: UI works on 375px viewport

**Intent:** Verify mobile usability.

**Given:**
- Game loaded on a 375x812px viewport (iPhone SE size)

**When:**
- Player navigates: main menu → galaxy map → planet detail → ship designer → fleet orders → generate turn

**Then:**

*Expected outputs:*
- All screens are usable without horizontal scrolling
- Touch targets are ≥ 44px
- Galaxy map supports pinch-to-zoom and touch-drag pan
- Text is readable without zooming
- No UI elements are clipped or overlapping

---

### SC-fail-001-crash_recovery: Crash during turn gen doesn't corrupt state

**Intent:** Verify game state survives a crash.

**Given:**
- Game at turn 50, turn generation initiated

**When:**
- Browser tab is force-closed mid-turn-generation

**Then:**

*Expected outputs:*
- On reopen, game loads at turn 50 (pre-generation state)
- No partial turn state is persisted
- Player can re-submit orders and regenerate

*Invariants:*
- Game state writes are atomic (all or nothing)
- Pre-turn state is preserved until new state is fully committed

---

### SC-fail-002-invalid_orders_rejected: Malformed orders don't crash turn gen

**Intent:** Verify robustness against garbage input.

**Given:**
- Player submits orders including: fleet waypoint to coordinates outside galaxy bounds, production queue with item ID that doesn't exist, research allocation totaling 150%

**When:**
- Turn generates

**Then:**

*Expected outputs:*
- Invalid waypoint is ignored, fleet stays put, warning message generated
- Invalid production item is skipped, rest of queue processes
- Research allocation is normalized to 100%
- Turn generates successfully despite bad input
- No crash, no panic, no data corruption

---

## Minimum Required Suite Summary

| ID | Area | Purpose | Priority |
|----|------|---------|----------|
| SC-galaxy-001 | Galaxy | Star counts per size | must_pass |
| SC-galaxy-002 | Galaxy | Homeworld assignment | must_pass |
| SC-race-001 | Race | Legal race validates | must_pass |
| SC-race-002 | Race | Illegal race rejected | must_pass |
| SC-planet-001 | Planet | Production queue | must_pass |
| SC-planet-002 | Planet | Population growth | must_pass |
| SC-planet-003 | Planet | Zero pop ownership | must_pass |
| SC-tech-001 | Tech | Research advancement | must_pass |
| SC-ship-001 | Ship | Design stats correct | must_pass |
| SC-ship-002 | Ship | No-engine edge case | should_pass |
| SC-fleet-001 | Fleet | Movement distance | must_pass |
| SC-fleet-002 | Fleet | Fuel exhaustion | must_pass |
| SC-combat-001 | Combat | Basic engagement | must_pass |
| SC-combat-002 | Combat | Retreat mechanic | should_pass |
| SC-turn-001 | Turn | Order of events | must_pass |
| SC-save-001 | Save | Save/load integrity | must_pass |
| SC-perf-001 | Perf | Turn gen < 2s at scale | must_pass |
| SC-ui-001 | UI | Mobile responsive | must_pass |
| SC-fail-001 | Fail | Crash recovery | must_pass |
| SC-fail-002 | Fail | Invalid orders handled | must_pass |

---

## CI Gate Checklist

### Build
- [ ] Build is reproducible from clean checkout (`git clone && npm install && npm run build`)
- [ ] Rust WASM module compiles without warnings
- [ ] TypeScript compiles with strict mode, no errors
- [ ] Lint passes (Clippy for Rust, ESLint for TS)

### Tests
- [ ] Rust unit tests pass (`cargo test`)
- [ ] TypeScript unit tests pass (`npm run test:unit`)
- [ ] Integration tests pass (turn generation end-to-end)
- [ ] Scenario suite passes (all `must_pass` scenarios green)
- [ ] Sniff test log shows every new function was verified

### Security
- [ ] `cargo audit` — no critical vulnerabilities
- [ ] `npm audit` — no critical vulnerabilities
- [ ] No hardcoded secrets in codebase
- [ ] Multiplayer API endpoints require authentication (Chunk 2)

### Data
- [ ] Save file schema is versioned
- [ ] Save files from previous versions still load (backward compat)
- [ ] No data loss on save/load round-trip

### Observability
- [ ] Turn generation logs phase timings
- [ ] Errors are logged with game context (game_id, turn, phase)
- [ ] Performance metrics captured for turn generation

### Performance
- [ ] Turn gen < 2s for 16-player Huge at turn 100
- [ ] UI renders at 60fps on reference hardware
- [ ] Memory usage stays bounded over 200 turns

### Release
- [ ] Version tagged (semver)
- [ ] Rollback plan documented
- [ ] CHANGELOG updated
- [ ] README updated with current build instructions
