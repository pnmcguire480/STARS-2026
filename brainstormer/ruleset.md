# Brainstormer Ruleset — STARS 2026

> Project-specific rules surfaced for ideation passes.

## Hard Rules
1. **No code from `reference/legacy-desktop-scaffold/` is inherited.** Study only.
2. **Determinism is non-negotiable.** Any proposal that breaks determinism is rejected.
3. **No real-time gameplay.** Turn-based only.
4. **No 3D, no AI art, no microtransactions, no Web3.**
5. **Sniff test after every function.** No batching, ever.
6. **Formulas cited or rejected.** Every game formula must trace to starsfaq.com, wiki.starsautohost.org, or `docs/FORMULAS.md`.
7. **One Rust engine, two compile targets.** Browser (wasm32) and server (x86_64). Identical bits.
8. **DLC = data + sprites only.** Engine never forks for content.

## Soft Rules
- Prefer `BTreeMap` over `HashMap` in any code path that touches turn generation.
- Prefer integer math over floats in any code path that affects game state.
- Prefer fewer dependencies. Every new crate / npm package needs justification.
- Prefer faithful 1995 mechanics over "modernized" ones unless the modernization fixes a documented original bug.

## Open To
- Modern UX affordances that don't change game math (search, sort, filter, undo within a turn).
- New tooltips, tutorials, accessibility features.
- Replay viewer.
- Spectator mode for multiplayer.
- New galaxy generation parameters.

## Closed To
- Changing combat math from canonical *Stars!*.
- Changing the order of events.
- Changing PRT/LRT point values.
- Adding loot, gacha, energy systems, daily login bonuses.
- AI that cheats by seeing through fog.

## Ideation Sessions Logged
- 2026-04-07: Initial scope locked. Stack, MVP, multiplayer model, mobile/desktop strategy, DLC strategy. See SPEC.md.
