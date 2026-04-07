# ARCHITECTURE.md — STARS 2026

## North Star
One Rust engine. Two compile targets. Identical bits.

## High-Level Diagram

```
                        ┌──────────────────────────┐
                        │  engine/  (Rust crate)   │
                        │  Pure logic, no I/O      │
                        │  Deterministic           │
                        └────────────┬─────────────┘
                                     │
                ┌────────────────────┴────────────────────┐
                │                                         │
        cargo build --target          cargo build --target
        wasm32-unknown-unknown        x86_64-pc-windows-msvc
                │                                         │
                ▼                                         ▼
   ┌────────────────────┐                    ┌──────────────────────┐
   │  pkg/  (wasm-pack) │                    │  server/  (Axum)     │
   │  TS bindings       │                    │  HTTP + WebSocket    │
   └─────────┬──────────┘                    │  PostgreSQL persist  │
             │                               │  Redis pub/sub       │
             ▼                               └──────────┬───────────┘
   ┌────────────────────┐                               │
   │  frontend/         │                               │
   │  SvelteKit + TS    │◄──────── WebSocket ───────────┘
   │  Tailwind          │
   │  Canvas map        │
   │  IndexedDB saves   │
   └────────────────────┘
             │
             ├──── v0.3 Capacitor wrap ────► iOS / Android
             └──── v0.3 Tauri wrap     ────► Native desktop
```

## Crates / Packages

| Path | Kind | Target(s) | Purpose |
|---|---|---|---|
| `engine/` | Rust library | wasm32 + x86_64 | All game logic. Pure functions. No `std::fs`, no time, no global state. |
| `engine-wasm/` | Rust crate | wasm32 only | wasm-bindgen wrapper exposing engine to JS. Built with wasm-pack into `frontend/src/lib/engine/pkg/`. |
| `server/` | Rust binary | x86_64 only (v0.2+) | Axum HTTP + WebSocket. Imports `engine` crate. Persists to PostgreSQL. Fanout via Redis. |
| `frontend/` | SvelteKit app | browser | UI, canvas map, IndexedDB local saves, WebSocket client. |
| `data/` | JSON | n/a | Hulls, components, tech costs, PRT traits. The DLC surface. |
| `art/` | Aseprite + PNG | n/a | Source sprites + exported atlases. |

## Determinism Contract

1. **No floating point in turn-critical math.** Use fixed-point or integer arithmetic. Floats are allowed only in render code.
2. **Single RNG.** One `rand::SeedableRng` (ChaCha20Rng) seeded from `(game_seed, turn_number, player_id, subsystem)`. No `thread_rng()`. Ever.
3. **No `HashMap` iteration in turn logic.** Use `BTreeMap` or sorted `Vec` so iteration order is deterministic across platforms.
4. **No `time::SystemTime` in engine.** Time is passed in as a parameter from the host (browser or server).
5. **CI gate:** every PR runs S-21 (cross-target determinism) before merge.

## Data Flow — Single Player Turn

```
1. UI collects player orders → OrderLog
2. JS calls engine.process_turn(state, order_log) via wasm-bindgen
3. Engine runs canonical 33-step order of events
4. Engine returns new GameState + per-player ScanReport
5. JS persists state to IndexedDB
6. UI re-renders from new state
```

## Data Flow — Multiplayer Turn (v0.2)

```
1. Each client submits OrderLog over HTTPS POST
2. Server stores orders in PostgreSQL
3. When all submitted OR deadline expires:
   3a. Server invokes engine.process_turn() (native build, same code as wasm)
   3b. Server hashes resulting state for the determinism gate
   3c. Server persists new state + state hash
   3d. Server pushes new state + hash to all clients via WebSocket
4. Each client locally replays the turn against its own engine
5. Client compares its hash to the server's hash → mismatch = desync alert
```

## Save Format

- **Local (single player):** IndexedDB, one record per game, value is `bincode`-serialized `GameState`.
- **Server (multiplayer):** PostgreSQL JSONB column, value is `serde_json`-serialized `GameState`. JSON for inspectability and migration ease; performance is fine at our scale.
- **Versioned:** every save carries `engine_version`. Loading a save from an older engine triggers a migration function chain.

## Module Boundaries (engine crate)

```
engine/src/
├── lib.rs              ← public API: process_turn, generate_galaxy, validate_race
├── types.rs            ← all domain types, IDs, errors
├── rng.rs              ← seeded RNG factory
├── galaxy.rs           ← procedural star generation
├── race.rs             ← PRT/LRT system, race point calculator
├── planet.rs           ← hab, population, production, mining
├── tech.rs             ← research tree, costs, racial modifiers
├── ship.rs             ← hull + component → derived stats
├── fleet.rs            ← waypoints, movement, fuel
├── combat.rs           ← tactical resolution
├── scanner.rs          ← fog of war, scan ranges
├── turn.rs             ← canonical 33-step orchestrator
└── ai/                 ← AI player implementation (one module per subsystem)
```

Each module is **a single responsibility**, **pure**, and **independently testable**. No cross-module mutable state. State flows in via parameters, out via return values.

## Frontend Module Boundaries

```
frontend/src/
├── routes/             ← SvelteKit pages
│   ├── +page.svelte    ← landing
│   ├── play/+page.svelte
│   └── lobby/+page.svelte (v0.2)
├── lib/
│   ├── engine/         ← wasm-bindgen glue + pkg/ output
│   ├── stores/         ← Svelte stores for UI state (NOT game state)
│   ├── components/     ← reusable UI
│   ├── canvas/         ← galaxy map renderer (Canvas2D)
│   └── persistence/    ← IndexedDB save/load
└── app.html
```

## Out of Scope for the Architecture

- No GraphQL.
- No tRPC.
- No microservices.
- No Kubernetes for v0.2 (one Axum binary on one box is fine until we have >1000 concurrent games).
- No blockchain. Ever.
- No client-side game logic divergence from server. The whole point is one engine.

## Open Questions (resolve in Phase 1+)

- Exact fixed-point representation: `i32` Q16.16 vs `i64` Q32.32 vs canonical *Stars!* integer-only? → revisit when implementing combat math.
- Procedural star naming: ship a name table or generate? → table for v0.1, generator considered for v1.0.
- AI architecture: rule-based (canonical *Stars!* AI) or learned? → rule-based, faithful. No ML.
