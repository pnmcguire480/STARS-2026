# SPEC.md — STARS 2026

> The 1995 4X classic *Stars!* reimagined with 2026 tech. Faithful soul, modern bones.

---

## One-Sentence Pitch

**A browser-native, deterministic, multiplayer-ready remake of *Stars!* (1995) — same beloved 4X turn-based gameplay, all-original code, faithful pixel art, runs anywhere from desktop to mobile.**

---

## The Problem

The original *Stars!* (Jeff Johnson + Jeff McBride, 1995) is a 16-bit Windows 3.1 binary. It is abandonware, IP held by the dissolved indiePub Entertainment. It does not run on modern 64-bit Windows without `winevdm` hacks or Windows 98 VMs. There is no mobile version. Every existing clone is incomplete or stalled:

- **craig-stars** (Go+SvelteKit, MIT) — ~51% complete, actively maintained, but partial
- **Stars Nova** (C#, GPL) — last meaningful activity 2023
- **FreeStars** — stalled
- **Thousand Parsec** — dead

The 4X community of *Stars!* veterans has nowhere to play the game they love on the hardware they own.

## The Solution

STARS 2026 is an all-original implementation written from scratch in modern languages, IP-clean, deterministic, and designed for both single-player and async multiplayer from day one. Faithful to the *feel* of the original — tech tree, racial trait system (PRTs/LRTs), turn-based fleet warfare, the legendary order-of-events — but every implementation detail is 2026.

---

## Audience

**Primary:** Patrick (and any *Stars!* veteran like him). Long-time fans of the original who want to play it on a modern machine, share games with friends asynchronously, and eventually on mobile during downtime.

**Secondary:** New 4X players discovering the depth of the *Stars!* design for the first time via a clean modern UI.

**Not the audience:** Casual mobile gamers looking for a 5-minute experience. STARS 2026 is deep, slow-burn strategy by design.

---

## User Stories

### Single Player (v0.1)
- As a *Stars!* fan, I can open a browser tab, generate a small galaxy, pick a race, and play a complete 200-turn game against AI opponents without ever installing anything.
- As a player, every action I take in a turn produces the *exact same outcome* if I replay with the same seed (deterministic engine).
- As a player, I can save/load my game in IndexedDB so I can close the tab and resume tomorrow.
- As a player, I can play hotseat locally with a friend on the same device, no network needed.

### Multiplayer (v0.2)
- As a player, I can create a game lobby for 1–16 players (any mix human/AI) and invite friends by link.
- As a player, I can pick a turn cadence: **Blitz** (5 min), **Daily** (24 hr), or **Marathon** (72 hr).
- As a player, I get a push notification when it's my turn or when the deadline is approaching.
- As a player, if I miss a deadline, the game auto-plays my turn from my last orders + standing doctrine — I never get booted.
- As a player on vacation, I can use a free pause token (1 per N turns) to freeze my empire (no production, immune to attack) so the game waits for me.
- As a player, if someone abandons their empire mid-game, a substitute can drop in and take over.

### Mobile / Native Desktop (v0.3)
- As a mobile player, I can install STARS 2026 from the App Store or Play Store (Capacitor wrap) and play with the same account as my desktop game.
- As an offline player, I can install the native desktop build (Tauri) and play single-player without ever touching the network.

### Full Game (v1.0)
- As a *Stars!* veteran, I can design races using all 10 Primary Racial Traits and the full Lesser Racial Trait set, with the canonical advantage point system.
- As a player, I can deploy minefields and mass-driver mineral packets.
- As a player, I see a battle replay after every combat with the canonical *Stars!* ordering.

### DLC
- As a player, I can install DLC packs that add new PRTs, hulls, components, scenarios, and campaigns — delivered as `data/*.json` + sprite packs only, no engine fork.

---

## Tech Stack (locked)

| Layer | Choice | Why |
|---|---|---|
| Game engine | **Rust** | Determinism, performance, single source of truth |
| Browser delivery | **WebAssembly** (`wasm32-unknown-unknown`, wasm-bindgen, wasm-pack) | Browser-native, no plugin |
| Multiplayer server | **Same Rust crate compiled to native `x86_64`** + **Axum** | Byte-identical determinism across client and server |
| Persistence (server) | **PostgreSQL** | Game state, accounts, lobbies |
| Realtime fanout | **Redis pub/sub** + **WebSocket** | Live presence, push triggers |
| Frontend | **SvelteKit + TypeScript** | Fast, modern, small bundles |
| Styling | **Tailwind CSS** | Rapid iteration |
| Galaxy map renderer | **HTML5 Canvas** | Performant 2D |
| Local saves | **IndexedDB** | Browser-native, offline |
| Mobile wrap (v0.3) | **Capacitor** | iOS + Android from same codebase |
| Native desktop wrap (v0.3) | **Tauri** | Rust-native shell, offline-first |
| Build | **Vite** (frontend), **Cargo** (Rust) | Standard |
| Test | `cargo test`, **Vitest**, **Playwright** | Unit, component, E2E |
| CI | **GitHub Actions** | Free for public, standard |
| Push notifications | **Web Push** + **FCM** + **APNs** | Cross-platform |
| Art tooling | **Aseprite** | Pixel art standard |

---

## Functional Requirements (v0.1 MVP)

| # | Requirement |
|---|---|
| FR-1 | Generate a procedural galaxy from a seed (24–100 stars for v0.1, "Tiny" size). See "Deviations from 1995 canon" below — Tiny floor is 24 per canon, not 32. |
| FR-2 | Place 1 human player + 1–3 AI opponents on starting worlds with balanced hab values. |
| FR-3 | Allow race creation with 1 PRT (the rest deferred to v1.0) and a basic LRT picker. |
| FR-4 | Calculate planet habitability per race using the canonical *Stars!* hab formula (sourced from starsfaq.com). |
| FR-5 | Population growth per turn with crowding penalty above 25% capacity. |
| FR-6 | Resource generation from population + factory output (hab affects resources *indirectly* via growth/capacity, not as a direct multiplier — per canon). |
| FR-7 | Mineral extraction with mine count, mining rate, and concentration depletion. |
| FR-8 | Production queue (auto-build + manual orders) processed each turn. |
| FR-9 | Six-field tech tree with research allocation slider; tech levels gate component availability. |
| FR-10 | Ship designer: pick hull, fill component slots, calculate derived stats (mass, armor, shields, weapons, fuel, scanners). |
| FR-11 | Fleet assembly from ship designs at a starbase; fleets are first-class movable objects. |
| FR-12 | Fleet movement via waypoint queue; engine speed determines warp factor; fuel burn calculated per leg. |
| FR-13 | Scanner-based fog of war: a player only sees what their ships, planets, or starbases can scan. |
| FR-14 | Tactical combat on a 10×10 grid with movement, beams, torpedoes, shields, armor, initiative — using the canonical *Stars!* combat resolution order. |
| FR-15 | Turn engine that runs the canonical 33-step *Stars!* order of events (sourced from starsfaq.com). |
| FR-16 | Save/load to IndexedDB; saves are deterministic (same seed + same orders = same state). |
| FR-17 | Hotseat mode: two human players take turns on the same device with privacy screen between turns. |
| FR-18 | Responsive UI from 1920px desktop down to 1024px tablet (375px mobile is v0.3). |
| FR-19 | All 19 type tests from the legacy scaffold's `types.rs` study material are reimplemented and passing in the new `engine` crate. |
| FR-20 | Engine compiles to **both** `wasm32-unknown-unknown` and native `x86_64` from a single source tree. |

## Functional Requirements (v0.2 Multiplayer)

| # | Requirement |
|---|---|
| FR-21 | Authoritative server (Axum, native Rust) holds game state in PostgreSQL. |
| FR-22 | Lobby creation: 1–16 player slots, any mix human/AI, host picks galaxy size and speed mode. |
| FR-23 | Async order submission: each player submits orders any time before deadline. |
| FR-24 | Speed modes: Blitz (5 min), Daily (24 hr), Marathon (72 hr), Hotseat (no server). |
| FR-25 | Auto-advance: as soon as all players submit, server runs the turn immediately. |
| FR-26 | Live presence over WebSocket: every player sees who has submitted in real time. |
| FR-27 | Web Push / FCM / APNs notification on turn start, deadline approach, turn ready. |
| FR-28 | AI takeover on missed deadline: server runs the player's turn from their last orders + saved doctrine. Player stays in the game. |
| FR-29 | Vacation mode: 1 free pause token per N turns; empire frozen, immune to attack, no production. |
| FR-30 | Substitute system: dropped players can be replaced mid-game by an incoming player. |
| FR-31 | Server-side turn generation produces byte-identical state to client-side replay (determinism contract). |

---

## Acceptance Criteria

### v0.1
- AC-1: Same seed + same orders → same final state on every run (engine determinism, 100 random seeds tested in CI).
- AC-2: A complete 200-turn AI game finishes in under 60 seconds of pure compute time on a mid-range laptop.
- AC-3: All 20 holdout scenarios from `Scenario.md` (rewritten by brainstormer in Phase 1) pass.
- AC-4: WASM bundle is under 2 MB gzipped.
- AC-5: First playable turn renders within 3 seconds of opening the page on a cold cache.
- AC-6: Save → close tab → reopen → load → game state is identical, no drift.
- AC-7: Hotseat mode privacy screen prevents player B from seeing player A's fog state.

### v0.2
- AC-8: 16-player game with all-AI runs to turn 200 in under 5 minutes server-side.
- AC-9: Server-generated turn state hashes byte-identical to client-replayed turn state (determinism gate, every turn, every game).
- AC-10: AI takeover triggers within 1 second of deadline expiry.
- AC-11: WebSocket presence updates land in under 200 ms p95.

### v1.0
- AC-12: All 10 PRTs balance-tested via 1000 AI-vs-AI matches; no PRT has >65% or <35% win rate.

---

## Deviations from 1995 canon

STARS 2026 is a faithful remake, not a clone. A small number of mechanical
deviations from the original *Stars!* (1995) have been authorized. Every
deviation on this list has a documented reason and a reviewable source.
**Any new deviation must be added here before shipping.**

| # | Deviation | Canon | STARS 2026 | Reason |
|---|---|---|---|---|
| D-1 | Tech field cap | 26 | **30** | Signature STARS 2026 extension — adds four late-game tiers. LRT/PRT bonuses can push higher. Content + cost curve for levels 27–30 to be decided by the `tech.rs` council. |
| D-2 | "Tiny" galaxy star count (FR-1) | **24**–100 | 24–100 | Actually *restored* to canon after an earlier draft of FR-1 wrote "32–100". Canon wins; the earlier 32 floor was a non-canonical default that survived by inertia. |

---

## Out of Scope (forever, not just for now)

- Real-time strategy mode (this is turn-based, full stop).
- 3D galaxy view (2D pixel art is the soul).
- Microtransactions / loot boxes.
- AI-generated art (faithful pixel art only).
- Reverse-engineering or decompiling original *Stars!* binaries (clean room, formulas from public FAQ/wiki only).

---

## Workflow Rules (non-negotiable)

These come from the legacy `BabyTierOS.md` and `LLM-Protocol.md` study material and are adopted verbatim:

1. **Sniff test after every function.** Unit test → module test → AC verification → regression check → manual spot-check → STOP for approval. No batching.
2. **One function per task.** Write the function and its test together, then stop.
3. **No placeholders.** Zero `todo!()`, `unimplemented!()`, `unwrap()` in shipped library code.
4. **Deterministic randomness.** All RNG uses seeded `rand::SeedableRng`.
5. **Result, not panic.** All fallible code returns `Result<T, GameError>`.
6. **Formula sourcing.** Every game formula cited to starsfaq.com, wiki.starsautohost.org, or `docs/FORMULAS.md` — never guessed.
7. **Scenario isolation.** Holdout scenarios are never shown to the implementation model.

---

## Reference Material (study only, never copy)

- `reference/legacy-desktop-scaffold/` — prior Claude Desktop session output (Spec, Scenario, BabyTierOS, LLM-Protocol, Cargo.toml, types.rs). Study for ideas, do **not** inherit.
- **craig-stars** on GitHub (sirgwain, MIT, Go+SvelteKit) — module boundary reference only. Different language, no copy risk.
- **starsfaq.com** — canonical formula source.
- **wiki.starsautohost.org** — tech/component/hull tables.

---

## Roadmap

| Version | Scope |
|---|---|
| v0.1 | Single-player + hotseat. Core loop. Deterministic engine proven. 1 PRT. |
| v0.2 | Networked multiplayer 1–16 players. Speed modes. AI takeover. Vacation. Push. |
| v0.3 | Capacitor mobile (iOS + Android) + Tauri native desktop. |
| v1.0 | Full 10 PRTs + LRTs. Minefields, packets. Advanced AI. Final art. Balance pass. |
| DLC | New PRTs, hulls, components, scenarios, campaigns via `data/*.json` + sprite packs. |
