# CONTEXT.md — Rolling State Tracker

> Updated every session. The fastest way to know "where are we right now."

## Now
- **Date:** 2026-04-07
- **Phase:** 0b — Project skeleton
- **Active task:** Authoring kernel governance docs + scaffolding Cargo workspace + SvelteKit frontend.
- **Blocker:** None.

## Just Finished
- Phase 0a: archived legacy `files/` to `reference/legacy-desktop-scaffold/`.
- Ran `brainstormer init`.
- Authored `SPEC.md`, `brainstormer/angles.md`, `brainstormer/hooks.md`.
- Updated `CLAUDE.md` with locked stack and sniff-test rules.

## Next (in order)
1. ✅ Write SCENARIOS.md
2. ✅ Write ARCHITECTURE.md
3. ✅ Write AGENTS.md, CODEGUIDE.md, ART.md
4. (now) Write CONTEXT.md, SNIFFTEST.md, README.md, brainstormer/ruleset.md
5. Create root Cargo.toml workspace + empty `engine/` crate
6. Scaffold `frontend/` via SvelteKit installer
7. `git init` + first commit
8. **STOP** for Patrick sign-off
9. Phase 1 Task 1: implement fresh `engine/src/types.rs` (informed by legacy study material) — sniff test, stop.

## Open Questions
- Cargo workspace will use Rust edition 2024 — confirm toolchain available.
- SvelteKit installer is interactive — may need manual flag passing for non-interactive run.
- Defer wasm-pack setup until first WASM build is needed (Phase 1 Task ~16).

## Decisions Log
- 2026-04-07: Stack locked — Rust+WASM engine, SvelteKit/TS frontend, Axum server (v0.2). See SPEC.md tech table.
- 2026-04-07: Player cap set at 16 (canonical). v0.1 single-player only.
- 2026-04-07: Multiplayer = v0.2, mobile/desktop wrap = v0.3, full PRT roster = v1.0.
- 2026-04-07: DLC strategy = data/*.json + sprite packs only, never engine forks.
- 2026-04-07: Reference material in `reference/legacy-desktop-scaffold/` is study-only, never inherited.

## Files Created This Phase
- `SPEC.md`, `SCENARIOS.md`, `ARCHITECTURE.md`, `AGENTS.md`, `CODEGUIDE.md`, `ART.md`, `CONTEXT.md`, `SNIFFTEST.md`, `README.md`
- `brainstormer/angles.md`, `brainstormer/hooks.md`, `brainstormer/ruleset.md`
- `Cargo.toml`, `engine/Cargo.toml`, `engine/src/lib.rs`
- `frontend/` (via SvelteKit installer)
- `.gitignore`
