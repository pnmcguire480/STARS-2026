# CONTEXT.md — Rolling State Tracker

> Updated every session. The fastest way to know "where are we right now."

## Now
- **Date:** 2026-04-07
- **Phase:** 1 — core engine vocabulary
- **Active task:** **Phase 1 Task 1 COMPLETE.** `engine/src/types.rs` is the full type vocabulary required by SPEC FR-19 — 16 atoms shipped, 27 tests passing, FR-19 at **19/19**, clippy::pedantic clean. Awaiting Tier 5 review before Atom 2 (`galaxy.rs`).
- **Blocker:** None. Tier 5 review of `types.rs` is the next gate per AGENTS.md — foundational engine modules cannot begin their downstream dependents until Tier 5 signs off.

## Just Finished
- Phase 0 closed: governance kernel committed, Cargo workspace green, `v0.0.1-skeleton` released.
- Phase 1 Council summoned (Rust + Game Design + Plan + Refactoring) and synthesized a 29-atom sequence for `types.rs`.
- **Atoms 1.1–1.14 shipped** — see `engine/src/types.rs`. Each atom: one cohesive type cluster + its tests, then a six-step sniff test (unit → module → AC → regression → clippy → stop).
  - 1.1 `GameError` (grown on demand, 3 variants — `InvalidRace`, `ArithmeticOverflow`, `InsufficientResources`).
  - 1.2 Seven typed ID newtypes (`GameId`, `PlayerId`, `StarId`, `PlanetId`, `FleetId`, `ShipDesignId`, `BattlePlanId`) — `Ord`-derived for BTreeMap use.
  - 1.3 `Position` + `distance_to`.
  - 1.4 `MineralType` + `Minerals` with **checked arithmetic** (atomic-on-failure `spend`/`add`).
  - 1.5 + 1.6 `MineralConcentrations`, `Environment`, **`HabAxis` enum** (killed the legacy `immune: bool` invalid-state smell), `HabRanges`.
  - 1.7 + 1.8 `GalaxySize` (+3 preset impls), `GalaxyDensity`.
  - 1.9 `PrtId` / `LrtId` newtypes with `#[serde(transparent)]` — **data-driven**, replacing the legacy hardcoded enum.
  - 1.10 + 1.11 `TechField`, `TechLevels` (get/set/meets_requirements), `ResearchAllocation` (+ `normalize` with u64-intermediate safety).
  - 1.12 `Cost` + `new`.
  - 1.13 **`Colonists` newtype** at 100-unit granularity with checked arithmetic; `Cargo` + `total_mass` using it.
  - 1.14 `TurnPhase` (33 canonical variants, `CANONICAL_ORDER` const, tripwire test).
  - 1.15 `ProductionItem` (8 canonical variants) + `QueueItem` (resource allocation tracking) + `Planet` (using `Colonists` newtype, not raw `u32`) + `Star` (derives drop `Eq`/`Hash` because of embedded `Position` with `f64`). FR-19 port: `create_star_and_planet_from_session_vocabulary`.
  - 1.16 `VictoryCondition` (7 variants, `#[non_exhaustive]`) + `AiDifficulty` (default: Standard) + `GameStatus` (default: Setup) + `GameSettings` (**no `Default` impl** — removed the legacy `random_seed: 0` sentinel per Refactoring council; engine receives seeds from the host, it never generates them). FR-19 port: `game_settings_survive_json_roundtrip`.
- **FR-19 scorecard: 19/19 COMPLETE.** SPEC Functional Requirement 19 formally satisfied. All legacy type tests reimplemented and passing against fresh, IP-clean, governance-aligned types.
- Clippy `-D warnings` under `clippy::pedantic` remains green across the entire engine crate after every atom.
- 4 governance decisions locked and saved to persistent memory (see Decisions Log below).

## Next (in order)
1. **Tier 5 review** of the full `engine/src/types.rs` file per AGENTS.md (Claude Opus chat). Foundational engine modules require Tier 5 sign-off before their downstream dependents begin. Bring the file, the FR-19 checklist, and the four governance memory files.
2. One-atom update to `SPEC.md` documenting the tech cap 30 deviation with a callout under FR-9 (per `project_tech_cap_30.md` memory).
3. Begin Atom 2 — `engine/src/galaxy.rs`: procedural star placement with seeded `ChaCha20Rng`, density curves, homeworld distance validation against `GalaxySize::min_homeworld_distance`. Council = Rust + Plan + Performance Engineer. First function = the RNG constructor that seeds from `(game_seed, turn, player_id, subsystem_tag)` — the determinism primitive everything else builds on.
4. Subsequent atoms per the Plan council's sequence: race (data-driven loader), planet mechanics, tech research, ship designer, fleet movement, combat, turn engine. Each with its own council summons.

## Open Questions (deferred to the atom that needs them)
- **Tech cost curve for levels 27–30** — deferred until `tech.rs` lands. Options: extrapolate the canonical Stars! curve mathematically, hand-tune four levels, or lift from a community mod with attribution.
- **What unlocks at tech levels 27–30** — deferred to `ship.rs` / `components.json`. Options: pure score/bragging levels with no new components, new DLC-style content designed from scratch, or redistribute existing Stars! content across a wider ladder.
- **Starting ship design ids referenced from `data/prt_traits.json`** — will point at `data/hulls.json` entries that do not yet exist. The loader must tolerate unknown ids during v0.1 bootstrap and validate strictly once `hulls.json` lands.
- **AC-2 pacing impact** (200-turn game in 60s) from the 30-level tech ladder — measure when `tech.rs` and `turn.rs` are runnable end-to-end.

## Decisions Log
- 2026-04-07: Stack locked — Rust+WASM engine, SvelteKit/TS frontend, Axum server (v0.2). See SPEC.md tech table.
- 2026-04-07: Player cap set at 16 (canonical). v0.1 single-player only.
- 2026-04-07: Multiplayer = v0.2, mobile/desktop wrap = v0.3, full PRT roster = v1.0.
- 2026-04-07: DLC strategy = `data/*.json` + sprite packs only, never engine forks.
- 2026-04-07: Reference material in `reference/legacy-desktop-scaffold/` is study-only, never inherited.
- **2026-04-07:** PRTs and LRTs are **data-driven** (`PrtId(String)` + `LrtId(String)` + JSON registries), not Rust enums. Honors the DLC promise in SPEC. Phase 1 Council recommendation; Patrick confirmed. See `memory/project_prt_data_driven.md`.
- **2026-04-07:** **Never `HashMap`, always `BTreeMap`** in any type that is serialized, persisted, iterated during turn generation, or hashed for the cross-target determinism gate. `HashMap`'s randomized iteration order would break the wasm/native byte-identical contract. Unanimous council. See `memory/feedback_determinism_btreemap.md`.
- **2026-04-07:** Population and colonist cargo use a **`Colonists(u32)` newtype** where the inner value counts 100-colonist blocks, matching 1995 Stars! canon exactly. Prevents off-by-100 bugs at compile time. See `memory/project_colonists_newtype.md`.
- **2026-04-07:** **Tech field cap is 30, not canon 26** — STARS 2026's signature mechanical deviation and its positioning differentiator vs every other Stars! clone. Extends late-game depth by four tiers; LRT/PRT bonuses can push further. Requires a SPEC.md callout atom, a FORMULAS.md cost-curve extension when `tech.rs` lands, and a README marketing bullet. See `memory/project_tech_cap_30.md`.
- **2026-04-07:** PRT/LRT JSON schema **Option B** (flat effects table) approved — see the schema design captured in session notes. Single `data/prt_traits.json` file with `schema_version: 1`, 9 sections (identity, starting state, economy, research, terraforming, combat, stealth, infrastructure, exclusive content). LRTs use a parallel schema in `data/lrt_traits.json`.

## Files Created This Phase
- `SPEC.md`, `SCENARIOS.md`, `ARCHITECTURE.md`, `AGENTS.md`, `CODEGUIDE.md`, `ART.md`, `CONTEXT.md`, `SNIFFTEST.md`, `README.md`
- `brainstormer/angles.md`, `brainstormer/hooks.md`, `brainstormer/ruleset.md`
- `Cargo.toml`, `engine/Cargo.toml`, `engine/src/lib.rs`
- **`engine/src/types.rs`** (new this session, ~1300 lines, 24 tests + 1 sentinel, clippy pedantic clean)
- `frontend/` (via SvelteKit installer)
- `.gitignore`
