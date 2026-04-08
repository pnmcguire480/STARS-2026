# STARS 2026 — Backlog

> Durable, in-repo record of deferred work. Exists because cross-chat memory
> between sessions is unreliable; if it isn't written down here, it's lost.
> **Every session MUST read this file after CLAUDE.md and before starting work.**

Last updated: 2026-04-08 (Atom 3 planning, pre-council)

---

## Blocking future phases

### Blocks Phase 2 start
- **FORMULAS.md full derivation sweep** — `docs/FORMULAS.md` stub will be created in Atom 3.0 with a "pending derivation" section. Every game formula used in Phases 2–3 must be cited or rejected per CLAUDE.md rule 8. Sweep before Phase 2 ships.
  - Known pending entries as of Atom 3 planning: `min_star_distance` constants (Sparse=30, Normal=25, Dense=20, Packed=15) — council-authorized, not canon-cited.

---

## Deferred atoms

### PRT/LRT JSON registry — Phase 1 wrap-up
Standalone atom. Data-driven primary/lesser racial traits loaded from JSON at engine init. Pattern was already established on the easier case (`data/star_names.json`) in Atom 3.0 P1-1, so this atom inherits the JSON loader scaffolding. Council needed: Rust + Game Design + Plan + First Principles.

### Tier 5 review of `engine/src/types.rs` — deferred from pre-sleep (2026-04-07)
The Phase 1 Task 1 close landed 16 atoms of type vocabulary. Patrick authorized deferring the Tier 5 (adversarial / long-horizon) review to manual return. Do before any type in `types.rs` gets a new consumer in Phase 2.

### Mutation testing (H7) — still deferred
`cargo-mutants` install failed on Windows-GNU during the hardening pass. Three paths forward are documented in `docs/codeglass/H7-mutation-testing-deferred.md`. Priority targets when it lands:
- `min_star_distance` constants
- FNV-1a magic numbers
- Asymmetric jitter bounds in `actual_star_count`

---

## Notes-for-next-session

### The "types.rs tech cap 30 SPEC callout" is CLOSED
Originally flagged as deferred pre-sleep. Atom 3.0 P1-9 adds a "Deviations from 1995 canon" section to SPEC.md that covers both Tiny=32 AND tech-cap=30, so this item is absorbed and closed — no separate atom needed.

### `GameError::GalaxyGenerationFailed(&'static str)` — v0.2 refactor
Tagged with `// i18n:v0.2` at the call site in Atom 3.0 P1-5. When v0.2 (multiplayer server) work begins, grep `i18n:v0.2` and convert the variant to a struct payload for proper localization.

---

## How to use this file

1. **At session start:** After CLAUDE.md, read this file.
2. **When something is deferred:** Add it here BEFORE the chat ends, not after. The chat may end abruptly or compact before you get to it.
3. **When something is closed:** Either delete the entry, or move it under a dated "Closed" section if the history matters.
4. **When in doubt, over-record.** A stale backlog entry costs 10 seconds to skim; a forgotten one costs a session.
