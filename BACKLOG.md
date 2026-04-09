# STARS 2026 — Backlog

> Durable, in-repo record of deferred work. Exists because cross-chat memory
> between sessions is unreliable; if it isn't written down here, it's lost.
> **Every session MUST read this file after CLAUDE.md and before starting work.**

Last updated: 2026-04-09 (Atom A close-out, A.12)

---

## Blocking future phases

### Blocks Phase 2 start
- **FORMULAS.md full derivation sweep** — `docs/FORMULAS.md` was created in Atom A.8 with three seeded entries (F-1, F-2, F-3) and five pending stubs (F-4..F-8). Every game formula used in Phases 2–3 must be cited or rejected per CLAUDE.md rule 8. Sweep before Phase 2 ships.

---

## Immediate next — Atom B (wasm-bindgen-test cross-target fingerprint)

**The single highest-risk deferred item in the codebase.** The 1204-byte determinism fingerprint in `engine/tests/determinism.rs` is same-target stable only — wasm32 and native have never been proven to produce the same bytes. The entire determinism contract (FR-16 save/load, FR-31 multiplayer replay) is theater until Atom B closes. Flagged P0-level by Red Teamer, Inversion, and Assumption Auditor in the Atom 2 closing Crucible. Mini-council needed: Rust + Performance Engineer + Plan.

## Deferred atoms

### PRT/LRT JSON registry — Phase 1 wrap-up
Standalone atom. Data-driven primary/lesser racial traits loaded from JSON at engine init. Pattern established in Atom A.6 (`data/star_names.json` + `engine/src/data.rs` loader). Council needed: Rust + Game Design + Plan + First Principles.

### Tier 5 review of `engine/src/types.rs` — deferred from pre-sleep (2026-04-07)
The Phase 1 Task 1 close landed 16 atoms of type vocabulary. Patrick authorized deferring the Tier 5 (adversarial / long-horizon) review to manual return. Do before any type in `types.rs` gets a new consumer in Phase 2.

### Mutation testing (H7) — still deferred
`cargo-mutants` install failed on Windows-GNU during the hardening pass. Three paths forward are documented in `docs/codeglass/H7-mutation-testing-deferred.md`. Priority targets when it lands:
- `min_star_distance` constants (now documented in FORMULAS.md F-1)
- FNV-1a magic numbers (now pinned by 11 per-subsystem vectors, A.10)
- Asymmetric jitter bounds in `actual_star_count` (now documented in FORMULAS.md F-3)

### P1-6 — pre-commit hook for `git add -A` guard (deferred from Atom A)
The Devil's Advocate recommended a pre-commit hook rejecting `git add -A` in autonomous mode after the Atom 2.3 stray-file sweep. Not urgent (targeted `git add` is now the convention), but worth implementing before the next autonomous-mode session.

---

## Closed in Atom A (2026-04-08 — 2026-04-09)

| P1 # | Finding | Closed by | Commit |
|---|---|---|---|
| P1-1 | STAR_NAMES const → `data/star_names.json` | A.6 | `1a3535c` |
| P1-3 | `rand`/`rand_chacha` exact version pins | A.7 | `ea75513` |
| P1-4 | `min_star_distance` FORMULAS.md citation | A.8 | `ea75513` |
| P1-5 | `GalaxyGenerationFailed` struct variant | A.9 | `ea75513` |
| P1-7 | FNV-1a per-subsystem test vectors | A.10 | `ea75513` |
| P1-8 | Huge+Packed saturation stress test | A.11 | `ea75513` |
| P1-9 | Tiny=24 canon (SPEC FR-1 amended) | A.1+A.4 | `860ead4`, `5c6b4c6` |

Also closed:
- **"types.rs tech cap 30 SPEC callout"** — absorbed into SPEC.md "Deviations from 1995 canon" section (D-1) in A.2, commit `860ead4`.
- **BACKLOG.md "Atom 3.0 P1-1/P1-5/P1-9" aspirational language** — those references were future-tense planning, not real atoms. The work landed in Atom A instead.

---

## Notes-for-next-session

### `GameError::GalaxyGenerationFailed { reason }` — v0.2 i18n
The struct variant (A.9) keeps `reason: &'static str`. When v0.2 (multiplayer server) work begins, grep `i18n:v0.2` and convert the field to a `String` or add an `error_code` field for proper localization.

---

## How to use this file

1. **At session start:** After CLAUDE.md, read this file.
2. **When something is deferred:** Add it here BEFORE the chat ends, not after. The chat may end abruptly or compact before you get to it.
3. **When something is closed:** Either delete the entry, or move it under a dated "Closed" section if the history matters.
4. **When in doubt, over-record.** A stale backlog entry costs 10 seconds to skim; a forgotten one costs a session.
