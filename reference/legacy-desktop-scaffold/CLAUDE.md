# CLAUDE.md — Stars 2026 Project Instructions

> **Read this file at the start of every Claude Code session.** This is your operating manual for this project.

## Project Overview

Stars 2026 is a ground-up reimplementation of the classic Stars! (1995) 4X space strategy game. Rust game engine compiled to WebAssembly, SvelteKit/TypeScript frontend, pixel art aesthetic.

## Critical Rules

1. **ONE function per task.** Write the function AND its unit test. Then STOP.
2. **Never skip the sniff test.** After writing code, show the test command and wait for Patrick to confirm results.
3. **Never chain tasks.** Do not proceed to the next function until Patrick says "next" or "continue."
4. **No placeholders.** Never ship `todo!()`, `unimplemented!()`, or `// TODO` in completed code. If a function isn't ready, say so.
5. **No invented game mechanics.** Every formula must come from Spec.md or the Stars! wiki. If you don't have the formula, ASK.
6. **Diffs only when fixing.** When fixing a bug, show ONLY the lines that change. Do not rewrite the function.
7. **Error handling in library code.** Use `Result<T, GameError>` — no `unwrap()` or `panic!()` in engine code.
8. **Deterministic randomness.** All RNG must use `rand::SeedableRng` with an explicit seed parameter. Same seed = same output. Always.

## Architecture

```
engine/          → Rust library, compiles to WASM
  src/types.rs   → All shared types, enums, constants
  src/galaxy.rs  → Galaxy generation
  src/race.rs    → Race design + point calculator
  src/planet.rs  → Planet mechanics, production, population
  src/tech.rs    → Research + tech tree
  src/ship.rs    → Ship design, hulls, components
  src/fleet.rs   → Fleet movement, waypoints, fuel
  src/combat.rs  → Battle engine
  src/turn.rs    → Turn generation orchestrator
  src/scanner.rs → Scanning, cloaking, fog of war
  src/lib.rs     → WASM entry points

frontend/        → SvelteKit + TypeScript
data/            → JSON game data (hulls, components, tech costs)
```

## Tech Stack

- **Engine:** Rust → WebAssembly (wasm-pack, wasm-bindgen)
- **Frontend:** SvelteKit, TypeScript, Tailwind CSS, HTML5 Canvas
- **Data:** JSON files for game tables, IndexedDB for saves
- **Testing:** cargo test (Rust), Vitest (TS), Playwright (E2E)

## Conventions

- Rust: snake_case functions, PascalCase types, SCREAMING_SNAKE constants
- TypeScript: camelCase functions, PascalCase components/types
- Files: lowercase-kebab for frontend, snake_case for Rust
- Commits: `✅ function_name passes AC-N` or `🔧 fix: description`

## Commands

```bash
cd engine && cargo test                    # Test engine
cd engine && cargo test galaxy::           # Test one module
cd engine && cargo clippy -- -D warnings   # Lint engine
cd frontend && npm run dev                 # Dev server
cd frontend && npm run test:unit           # Test frontend
```

## Order of Events (Stars! Canonical — REFERENCE)

Turn generation must follow this exact sequence:
1. Scrapping fleets (with possible tech gain)
2. Waypoint 0 unload tasks
3. Waypoint 0 colonization / ground combat
4. Waypoint 0 load tasks
5. Other Waypoint 0 tasks
6. Mystery Trader moves
7. In-space packets move and decay (PP terraform, damage)
8. Wormhole entry jiggle
9. Fleet movement (fuel, minefields, stargates, wormholes)
10. Inner Strength colonist growth in fleets
11. Mass packets/salvage decay
12. Wormhole exit jiggle, endpoint degrade/jump
13. SD minefield detonation
14. Mining (remote + planet)
15. Production (research, packet launch, construction)
16. SS spy bonus
17. Population growth/death
18. Just-launched packets reaching destination cause damage
19. Random events
20. Fleet battles (with possible tech gain)
21. Meet Mystery Trader
22. Bombing (per player in order)
23. Waypoint 1 unload tasks
24. Waypoint 1 colonization / ground combat
25. Waypoint 1 load tasks
26. Mine laying
27. Fleet transfer
28. Waypoint 1 fleet merge
29. CA instaforming
30. Minefield decay
31. Mine sweeping
32. Starbase and fleet repair
33. Remote terraforming
