# CONTEXT.md — Stars 2026

## Current goal
Complete Phase 1 (Foundation): types, galaxy generation, race design

## Current phase
Phase 1: Foundation — Task 1 of 5 (Types & Constants)

## Last 3 decisions
1. Tech stack: Rust/WASM engine + SvelteKit frontend — Rust gives us deterministic turn gen, fast combat resolution, and compiles to WASM for browser + future mobile via Capacitor
2. Clean-room implementation — no forking craig-stars, no original Stars! code. All original code, referencing documented formulas only
3. Sniff test protocol mandatory after every function — logged in this file

## Current failure / blocker
None — project just initialized

## Sniff test log
| Function | Test | AC | Pass/Fail | Date |
|----------|------|----|-----------|------|
| types.rs (all types) | 19 unit tests | Foundation | ✅ 19/19 pass | 2026-03-12 |

## Key file paths
- engine/src/types.rs: All shared types, enums, constants, game error
- engine/src/lib.rs: Crate root, module declarations
- engine/Cargo.toml: Rust dependencies
- CLAUDE.md: Claude Code session instructions
- Spec.md: Master specification
- Scenario.md: Holdout test scenarios (do NOT show to builder)
- BabyTierOS.md: Workflow system
- LLM-Protocol.md: AI coding operating manual

## Commands to reproduce
- Build engine: `cd engine && cargo build`
- Test engine: `cd engine && cargo test`
- Lint engine: `cd engine && cargo clippy -- -D warnings`
- Build WASM: `cd engine && wasm-pack build --target web`

## Next task
Task 2: Galaxy generation (galaxy.rs) — procedural star placement with seeded RNG
