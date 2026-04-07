# Spec: Stars 2026

**Owner:** Patrick McGuire
**Date:** 2026-03-12
**Status:** Draft
**Repo/Path:** `github.com/patrickmcguire/stars2026` (TBD)

---

## 0) One-sentence outcome

Build **Stars 2026**, a modern ground-up reimplementation of the classic Stars! 4X space strategy game, so that **strategy gamers on any device** can **explore, expand, exploit, and exterminate across procedurally generated galaxies** without **emulation headaches, 16-bit compatibility nightmares, or 1995-era UI limitations**, measured by **feature parity with core Stars! mechanics, sub-second turn generation, and native mobile playability**.

---

## 1) Problem and user

- **User(s):** Strategy gamers (desktop + mobile), Stars! nostalgists, 4X newcomers who never got to experience the original
- **Job-to-be-done:** Play a deep, turn-based space 4X game with custom race design, ship design, fleet management, and multiplayer — on modern hardware including phones and tablets
- **Current workaround:** Running the original Stars! 2.6j via winevdm, DOSBox with Windows 3.1, or VirtualBox Win98 images. Craig-stars (web clone, 51% complete, Go/SvelteKit). None run natively on mobile.
- **Why now:** The original is functionally abandonware (rights held by indiePub after Empire Interactive folded, developers gave tacit blessing to community distribution). No native mobile version exists. Modern tech (Rust/WASM, reactive UIs, cross-platform tooling) can deliver the full experience without the 16-bit baggage. The community is still active (Discord, AutoHost, wiki maintained through 2024).

---

## 2) Scope

**In scope (MVP — "Chunk 1"):**
- Single-player game against AI opponents (1–15 AI players)
- Procedural galaxy generation (Tiny through Huge)
- Race designer with all 10 Primary Racial Traits (PRT) and Lesser Racial Traits (LRT)
- Planet colonization, population management, mineral extraction
- Technology research across 6 fields (Energy, Weapons, Propulsion, Construction, Electronics, Biotechnology)
- Ship designer with hull selection and component slots
- Fleet movement, waypoint orders, fuel management
- Basic combat engine (grid-based tactical battle resolution)
- Turn generation engine with correct order of events
- Pixel art UI with modern responsive layout
- Desktop build (web-based, runs in browser)

**In scope (Chunk 2 — post-MVP):**
- Mobile builds via Capacitor (Android + iOS)
- Multiplayer via PBEM-style async turns (hosted server)
- Full battle viewer/replay
- Minefield mechanics
- Packet physics (mass drivers)
- Tech trading between players
- Advanced AI behaviors
- Save/load game state

**Out of scope (explicit non-goals):**
- Real-time gameplay (this is strictly turn-based)
- 3D graphics or cinematic visuals (pixel art aesthetic, intentionally)
- Exact binary compatibility with original Stars! save files
- Replicating original bugs/exploits (we fix them)
- MMO-scale multiplayer (cap at 16 players per game)
- Blockchain, NFTs, or monetization schemes
- Reproducing any copyrighted assets, code, or IP from the original

**Success criteria (measurable):**
- Turn generation for a 16-player Huge galaxy completes in < 2 seconds
- All 10 PRTs are playable with mechanically distinct gameplay
- Ship designer supports all standard hull types with drag-and-drop components
- Game runs at 60fps on a mid-range 2024 Android phone (browser-based)
- Single-player game is completable (victory conditions trigger correctly)
- Zero IP entanglement — all code, art, and text is original

---

## 3) Functional requirements (FR)

### Galaxy & Planets
- **FR-1:** Procedural galaxy generation with configurable size (Tiny: ~30 stars, Small: ~70, Medium: ~150, Large: ~300, Huge: ~600), density, and player count
- **FR-2:** Each star system has 0–N planets with randomized attributes: gravity, temperature, radiation, mineral concentrations (Ironium, Boranium, Germanium), mineral surface deposits
- **FR-3:** Planet habitability calculated per-race based on race hab ranges vs planet environment values
- **FR-4:** Terraforming modifies planet environment toward race ideal over time

### Race Design
- **FR-5:** Race designer with 10 Primary Racial Traits: Hyper-Expansion (HE), Super Stealth (SS), War Monger (WM), Claim Adjuster (CA), Inner Strength (IS), Space Demolition (SD), Packet Physics (PP), Interstellar Traveler (IT), Alternate Reality (AR), Jack of All Trades (JoAT)
- **FR-6:** Lesser Racial Traits (LRT) system with point-buy balance: Improved Fuel Efficiency, Total Terraforming, Advanced Remote Mining, Improved Starbases, Generalized Research, Ultimate Recycling, Mineral Alchemy, No Ram Scoop Engines, Cheap Engines, Only Basic Remote Mining, No Advanced Scanners, Low Starting Population, Bleeding Edge Technology, Regenerating Shields
- **FR-7:** Hab range configuration (gravity, temperature, radiation) with immunity option
- **FR-8:** Growth rate, economy, factory, mine efficiency configuration
- **FR-9:** Race point balance validation (advantage points ≤ 0 for a legal race)

### Production & Economy
- **FR-10:** Planet production queue supporting factories, mines, defenses, terraforming, ships, starbases, mineral alchemy, scanner, mineral packets
- **FR-11:** Population growth model based on hab value and crowding
- **FR-12:** Mineral extraction rates based on mine count and concentration
- **FR-13:** Resource generation from population and factories
- **FR-14:** Auto-build and production templates

### Technology
- **FR-15:** 6-field tech tree (Energy, Weapons, Propulsion, Construction, Electronics, Biotech) with levels 0–26+
- **FR-16:** Research allocation with percentage-based distribution across fields
- **FR-17:** Tech level requirements gate component availability
- **FR-18:** Racial tech advantages/disadvantages (some PRTs get cheaper research in certain fields)

### Ship Design & Fleets
- **FR-19:** Ship designer with hull browser, component slot system (weapons, shields, engines, scanners, armor, special), mass/fuel calculations
- **FR-20:** Standard hulls: Scout, Frigate, Destroyer, Cruiser, Battleship, Dreadnought, Privateer, etc. plus Starbase hulls
- **FR-21:** Fleet composition, merging, splitting
- **FR-22:** Waypoint system with orders: move, colonize, remote mine, patrol, transport (load/unload minerals and population), lay mines, sweep mines, etc.
- **FR-23:** Fuel consumption based on engine type, fleet mass, warp speed, and distance
- **FR-24:** Cargo capacity and transport orders

### Combat
- **FR-25:** Tactical grid-based battle engine with movement phases and weapon phases
- **FR-26:** Battle plans (targeting priorities, retreat conditions) assignable per ship design
- **FR-27:** Weapon types: beams (range-attenuated), torpedoes (accuracy-based), bombs (planetary)
- **FR-28:** Shields, armor, jamming, capacitors, deflectors modifying combat outcomes
- **FR-29:** Battle reports viewable after turn generation

### Scanning & Intelligence
- **FR-30:** Scanner ranges (normal and penetrating) per ship/starbase
- **FR-31:** Cloaking and cloaking detection
- **FR-32:** Fog of war — players only see what their scanners reach

### Turn Engine
- **FR-33:** Turn generation follows the canonical Stars! order of events (see starsfaq.com/order_events.htm)
- **FR-34:** All player orders processed simultaneously per phase
- **FR-35:** Deterministic — same inputs always produce same outputs (critical for replays and debugging)

### UI/UX
- **FR-36:** Galaxy map (pan, zoom) with star/fleet/waypoint rendering
- **FR-37:** Planet detail panel with production queue, population, minerals, environment
- **FR-38:** Ship designer with visual hull layout and component drag-and-drop
- **FR-39:** Fleet management panel with waypoint orders
- **FR-40:** Turn summary / message log
- **FR-41:** Research allocation panel
- **FR-42:** Race designer (pre-game)
- **FR-43:** Pixel art asset rendering at multiple zoom levels
- **FR-44:** Responsive layout that works on 1920x1080 desktop down to 375px mobile

---

## 4) Non-functional requirements (NFR)

- **Performance:** Turn generation < 2s for 16-player Huge galaxy. UI renders at 60fps. Galaxy map pan/zoom is smooth at all zoom levels. Ship designer component placement has zero perceptible lag.
- **Reliability:** Game state is never corrupted by a crash or disconnect. Autosave every turn. Save files are JSON-based and human-inspectable.
- **Security:** Multiplayer: server-authoritative turn generation (no client-side game logic). API endpoints require authentication. Player can only see data their scanners reveal.
- **Privacy:** No telemetry without consent. No account required for single-player. Multiplayer accounts use email + password or OAuth.
- **Cost:** Development: $20/month LLM budget + free-tier hosting during development. Production: target < $50/month for a hosted multiplayer server supporting 20 concurrent games.
- **Accessibility/UX:** Keyboard navigable. Color-blind friendly palette option. Scalable UI text. Touch-friendly hit targets on mobile (minimum 44px).

---

## 5) User flows

**Flow A: New Single-Player Game**
1. Player opens app → main menu
2. Selects "New Game"
3. Configures galaxy (size, density, player count, AI difficulty)
4. Designs custom race OR selects a preset race
5. Race validator confirms point balance is legal
6. Game generates galaxy, assigns homeworlds
7. Player sees galaxy map with their homeworld, initial fleet
- **Expected result:** Playable galaxy with correct star count for selected size, homeworld is habitable for player's race, starting fleet matches PRT starting conditions

**Flow B: Playing a Turn**
1. Player reviews messages from previous turn
2. Sets production queues on planets
3. Designs/modifies ships if new tech available
4. Issues fleet waypoint orders (move, colonize, attack, etc.)
5. Adjusts research allocation if desired
6. Clicks "Generate Turn"
7. Turn engine processes all orders, resolves combat, grows populations, advances research
8. Player receives new turn with updated galaxy state, battle reports, messages
- **Expected result:** All orders executed in correct sequence per order of events. No data loss. Battle reports accurate. Galaxy state consistent.

**Flow C: Ship Design**
1. Player opens ship designer
2. Selects hull type from available hulls (gated by tech level)
3. Views hull slot layout (weapon slots, shield slots, general slots, etc.)
4. Drags components into slots from component palette (gated by tech level)
5. Designer shows real-time stats: mass, fuel capacity, armor, shields, weapon power, cost (resources + minerals), initiative, movement
6. Player names the design and saves
- **Expected result:** Design correctly calculates all derived stats. Design is available in production queues. Invalid slot assignments are prevented (e.g., weapon component in engine-only slot).

**Flow D: Combat Resolution**
1. Fleets from different players arrive at same location
2. Turn engine detects combat situation
3. Battle engine creates tactical grid, places tokens based on ship mass/initiative
4. Battle runs phase-by-phase: movement → weapons fire → damage resolution
5. Ships destroyed, retreated, or victorious
6. Battle report generated for all involved players (limited by scanner info)
- **Expected result:** Combat follows deterministic rules. Damage calculations match documented formulas. Retreat conditions honored. Salvage generated correctly.

---

## 6) Data model

**Core Entities:**
- **Game:** `{ id, name, galaxy_size, density, turn_number, status, settings{} }`
- **Player:** `{ id, game_id, race_id, name, relations[], research_allocation{} }`
- **Race:** `{ id, name, prt, lrt[], hab_ranges{grav, temp, rad}, growth_rate, factory_efficiency, mine_efficiency, ... }`
- **Star:** `{ id, game_id, x, y, name }`
- **Planet:** `{ id, star_id, owner_id?, population, grav, temp, rad, ironium_conc, boranium_conc, germanium_conc, ironium_surface, boranium_surface, germanium_surface, mines, factories, defenses, scanner, production_queue[] }`
- **ShipDesign:** `{ id, player_id, name, hull_type, slots[{slot_type, component_id, count}], calculated_stats{} }`
- **Fleet:** `{ id, player_id, x, y, ships[{design_id, count, damage}], fuel, cargo{}, waypoints[], battle_plan_id }`
- **BattlePlan:** `{ id, player_id, name, primary_target, secondary_target, tactic, retreat_condition }`
- **TechLevel:** `{ player_id, energy, weapons, propulsion, construction, electronics, biotech }`
- **Message:** `{ id, player_id, turn, type, content }`
- **BattleReport:** `{ id, game_id, turn, location, participants[], rounds[], outcome }`

**Relationships:**
- `Game` 1..N `Player`
- `Player` 1..1 `Race`
- `Game` 1..N `Star`
- `Star` 1..N `Planet`
- `Player` 1..N `ShipDesign`
- `Player` 1..N `Fleet`
- `Player` 1..N `BattlePlan`
- `Player` 1..1 `TechLevel`

**Storage:**
- DB: SQLite for single-player (embedded, zero-config). PostgreSQL for multiplayer server.
- Migrations: Versioned SQL migrations via built-in migration runner
- Save format: Full game state exportable as compressed JSON for portability
- Backfill: Not applicable for v1 (no legacy data)

---

## 7) API / Interface surface

### Single-Player (local)
All game logic runs client-side via WASM module. No network API needed for single-player.

### Multiplayer Server API (Chunk 2)
- `POST /api/games` — Create game — input: `GameSettings` — output: `GameId` — errors: `400 invalid settings`
- `POST /api/games/:id/join` — Join game — input: `RaceConfig` — output: `PlayerId` — errors: `404, 409 game full`
- `POST /api/games/:id/turns` — Submit turn orders — input: `TurnOrders` — output: `202 accepted` — errors: `400, 403, 409 already submitted`
- `GET /api/games/:id/state` — Get current game state (player-scoped, fog of war applied) — output: `PlayerGameState`
- `GET /api/games/:id/battles/:turn` — Get battle reports — output: `BattleReport[]`

**Events / Webhooks:**
- `turn_generated` — payload: `{ game_id, turn_number, timestamp }` — notifies all players new turn is ready

---

## 8) Edge cases and failure modes

**Edge cases:**
- Race with 100% hab range (lives everywhere) vs race with 1-tick narrow hab (lives almost nowhere) — both must be balanced via point system
- Fleet with 0 fuel at waypoint — must handle gracefully (fleet stops, does not vanish)
- Planet population reaches 0 — planet reverts to unowned, structures remain
- Two fleets meet at deep space waypoint (not at a star) — combat still triggers
- Player designs a ship with no engines — valid design but fleet speed is 0
- Mineral packet hits a planet with no defenses — damage calculation must handle
- Tech level exceeds 26 — some components have no upper bound

**Failure modes:**
- Turn generation crashes mid-process → detection: try/catch at turn-level → mitigation: roll back to pre-turn state, report error, never save partial state
- Save file corrupted → detection: JSON parse fails or schema validation fails → mitigation: maintain backup of previous turn's save
- Browser tab crashes during long turn gen → detection: N/A (client-side) → mitigation: WASM turn gen is atomic, state only written on success
- Multiplayer: player submits orders twice → detection: turn submission tracking → mitigation: last submission wins (overwrite), timestamp logged

---

## 9) Observability

- **Logs (structured):** `{ timestamp, level, module, event, game_id?, player_id?, turn?, details }` — logged for: turn generation steps, combat resolution, production completion, errors
- **Metrics:** Turn generation duration (histogram), game count (gauge), active players (gauge), combat events per turn (counter)
- **Tracing:** Turn generation phases traced: fleet movement → production → research → combat → population growth → cleanup
- **Dashboards:** Turn generation performance over time, error rate per game
- **Alerts:** Turn generation > 5s (warning), > 10s (critical). Save file write failure (critical).

---

## 10) Security review (minimum)

**Threat model summary (top 3):**
- Threat: Multiplayer client sends forged game state → mitigation: server-authoritative turn generation, client only submits orders (never state)
- Threat: Player reads other player's fleet positions from API → mitigation: fog-of-war filter applied server-side before response serialization, scanner-range validated
- Threat: Save file tampering in single-player → mitigation: single-player is trust-the-client (acceptable — it's their game). Multiplayer saves are server-only.

- **Secrets:** API keys and DB credentials in environment variables, never in code. JWT signing key rotated monthly.
- **Permissions:** Players can only access their own game state. Game host can configure settings but not view other players' hidden data.
- **Audit logs:** Multiplayer: all turn submissions logged with timestamp and player ID. Admin actions logged.

---

## 11) Rollout / rollback

**Rollout plan (MVP):**
1. Single-player web build deployed to Netlify (static site + WASM)
2. Beta testers from Stars! Discord community invited
3. Feedback collected via GitHub Issues
4. Mobile builds via Capacitor after web is stable

**Rollback plan:**
- What to revert: Netlify instant rollback to previous deploy
- Data rollback: Single-player saves are client-local (no server data to roll back)
- Multiplayer (Chunk 2): Database snapshots before each deploy, migration rollback scripts tested

---

## 12) Acceptance criteria (must be testable)

- **AC-1:** Given a new game with "Medium" galaxy size, when the game generates, then there are 120–180 stars with planets, and each planet has valid environmental values within documented ranges.
- **AC-2:** Given a custom race with HE PRT and narrow hab range, when the race point calculator runs, then the advantage points balance is within ±5 of the expected value per the Stars! race wizard formula.
- **AC-3:** Given a planet with 100k population, 50 factories, and 75% hab value, when a turn generates, then resource production matches the formula: `resources = population * efficiency * hab_factor + factory_output`.
- **AC-4:** Given a fleet ordered to move from Star A to Star B at Warp 7, when the turn generates, then the fleet moves exactly `7² = 49` light-years toward the destination, consuming fuel per the engine fuel table.
- **AC-5:** Given two opposing fleets at the same location, when combat resolves, then damage is calculated per beam attenuation / torpedo accuracy formulas, shields absorb before armor, and destroyed ships are removed from the fleet.
- **AC-6:** Given a ship design with a Scout hull, long-range scanner, and fuel mizer engine, when the design is saved, then calculated mass, fuel capacity, scan range, and cost are correct per component stats.
- **AC-7:** Given a player with Tech Level 5 in Weapons, when they open the ship designer, then only components requiring Weapons ≤ 5 are available.
- **AC-8:** Given a 16-player Huge galaxy at turn 100, when a turn generates, then it completes in under 2 seconds on the reference hardware.
- **AC-9:** Given the game running on a 375px-wide mobile viewport, when the player navigates the galaxy map, then pan and zoom work via touch gestures and all controls are reachable without horizontal scrolling.
- **AC-10:** Given the complete turn generation sequence, when every phase executes, then the order of events matches the canonical Stars! order of events document exactly.

---

## 13) Open questions / unknowns (blockers)

- **Q1:** Exact tech stack decision — Rust/WASM for engine + SvelteKit for UI vs. pure TypeScript (simpler but slower turn gen)? — decision needed by: 2026-03-20
- **Q2:** Pixel art style direction — retro CRT aesthetic vs. modern clean pixel art vs. hand-drawn? Need concept art before UI build begins. — decision needed by: 2026-03-25
- **Q3:** AI difficulty — implement Stars! original AI behavior (well-documented) or build a new AI from scratch? — decision needed by: Chunk 2
- **Q4:** Multiplayer architecture — WebSocket real-time updates vs. polling vs. email-based PBEM? — decision needed by: Chunk 2
- **Q5:** Name/branding — "Stars 2026" is a working title. Final name must avoid trademark conflicts. — decision needed by: before public release

---

## 14) Implementation notes (for AI builder)

**Tech stack (proposed — see Q1):**
- **Game engine (core logic):** Rust compiled to WebAssembly — handles turn generation, combat resolution, all game math. Deterministic. Testable in isolation.
- **UI layer:** SvelteKit + TypeScript — reactive UI, component-based, handles all rendering and user interaction
- **Rendering:** HTML5 Canvas (galaxy map, battle viewer) + DOM (panels, menus, designer) with pixel art sprite sheets
- **Data layer:** IndexedDB (client-side persistence for single-player saves) + SQLite via WASM (game state during play)
- **Mobile:** Capacitor wrapping the web app for Android/iOS
- **Build tooling:** Vite, wasm-pack, GitHub Actions CI
- **Multiplayer server (Chunk 2):** Rust (Axum framework), PostgreSQL, hosted on Fly.io or Railway

**Libraries allowed:**
- Rust: `serde`, `rand` (seeded), `wasm-bindgen`, `js-sys`
- TypeScript: SvelteKit, Tailwind CSS, Pixi.js (Canvas rendering), Capacitor
- Testing: `cargo test` (Rust), Vitest (TS), Playwright (E2E)

**No changes to:**
- Original Stars! source code (we don't have it and don't need it)
- Craig-stars codebase (we are not forking — clean-room implementation)

**Sniff Test Protocol (mandatory after every function/feature):**
After every function is created or modified:
1. Run the function's unit test
2. Run the module's integration test
3. Verify the behavior against the relevant acceptance criteria
4. Check that no previously passing tests now fail (regression check)
5. If the function touches turn generation, run a full turn-gen smoke test
6. Log pass/fail in Context.md

**Claude Code guardrails:**
- Claude must not skip steps in the sniff test protocol
- Claude must not generate code for multiple features in a single pass
- Claude must show test results before moving to the next task
- If Claude generates a function, it must also generate the corresponding unit test in the same pass
- Patrick reviews behavior between every task — Claude does not chain tasks without human checkpoint

**Definition of done:**
- [ ] All AC pass
- [ ] Scenario suite pass
- [ ] CI gates pass (lint, test, build)
- [ ] Sniff tests logged for every new function
- [ ] No regressions in existing tests
- [ ] Docs updated (README, CHANGELOG)
