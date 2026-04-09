---
id: ADR-0003
title: "Atom A — hardening interlude between Atom 2 and Atom 3"
status: accepted
date: 2026-04-09
author: Patrick McGuire + Claude Opus 4.6 (1M context)
related: [ADR-0002, CRUCIBLE-VERDICT-atom-2, PALADIN-VERDICT-atom-2]
tags: [hardening, atom-a, crucible-p1, governance]
---

# ADR-0003 — Atom A hardening interlude

## Context

After the autonomous Atom 2 sleep-shift completed `engine/src/galaxy.rs`
(10 sub-atoms, 67 tests, CI green), a deep audit on 2026-04-08 revealed:

- **9 Crucible P1 findings** still open in code (from `docs/codeglass/CRUCIBLE-VERDICT-atom-2.md`)
- **Stale governance docs** — CLAUDE.md and CONTEXT.md both reported Atom 2 as "4 of 8 shipped" while all 10 sub-atoms had landed
- **BACKLOG.md spoke in future tense** about "Atom 3.0" sub-atoms that did not exist — aspirational, not plan
- **Patrick decisions needed**: Tiny=24 (canon vs SPEC), STAR_NAMES migration path, atom ordering

## Decision

Patrick authorized a **13-sub-atom mechanical hardening interlude** (Atom A) to close cheap debt before starting Atom 3 (planet.rs). No 5-agent council was needed — all work was governance sync + mechanical P1 closures.

Three Patrick decisions locked on 2026-04-08:
1. **P1-9:** Tiny galaxy star count = **24** (canon), not 32 (SPEC draft). SPEC FR-1 amended.
2. **P1-1:** STAR_NAMES migrates **straight to `data/star_names.json`** — no interim "shrink to 12 const" step. Loader receives the list as an argument to `generate_galaxy` (keeps seed→galaxy mapping pure).
3. **Atom order:** A (hardening) → B (wasm-bindgen-test cross-target fingerprint) → C (Atom 3 = planet.rs).

## Sub-atoms

| # | What | Commit |
|---|---|---|
| A.1+A.2 | SPEC FR-1 → 24 stars; new "Deviations from 1995 canon" section (D-1, D-2) | `860ead4` |
| A.3 | CLAUDE.md + CONTEXT.md governance sync to post-Atom-2 reality | `9ab931f` |
| A.4+A.5 | `GalaxySize::Tiny::target_stars()` 32→24; generator re-derivation; `fr1_galaxy.rs` envelope 32→24; `determinism.rs` fingerprint re-pin (1485→1204 bytes, fewer stars) | `5c6b4c6` |
| A.6 | `data/star_names.json` + `engine/src/data.rs` loader; `generate_galaxy` takes `star_names: &[String]`; STAR_NAMES const deleted | `1a3535c` |
| A.7 | Exact-pin `rand = "=0.8.5"`, `rand_chacha = "=0.3.1"` | `ea75513` |
| A.8 | `docs/FORMULAS.md` stub with F-1/F-2/F-3 + 5 pending stubs; `min_star_distance` doc-comment cites F-1 | `ea75513` |
| A.9 | `GameError::GalaxyGenerationFailed(&'static str)` → struct variant `{ reason: &'static str }` | `ea75513` |
| A.10 | 11 per-subsystem FNV-1a test vectors + pairwise-distinct tripwire | `ea75513` |
| A.11 | Huge+Packed saturation stress test (20 seeds × GalaxySize::Huge + GalaxyDensity::Packed) | `ea75513` |
| A.12 | BACKLOG.md rewrite: close 7 P1s, note P1-6 deferred, promote Atom B | (this commit) |
| A.13 | This ADR + final CLAUDE.md/CONTEXT.md sync | (this commit) |

## Outcome

- **72 tests** (up from 67 at Atom A start)
- **7 of 9 Crucible P1s closed** (P1-2 → Atom B, P1-6 → deferred TODO)
- Determinism fingerprint stable through A.6–A.11 — the JSON loader migration, version pins, and struct variant rewrite were all provably behavior-preserving at the byte level
- Every commit on main is a green CI run (no red main, no force-push, no `--no-verify`)
- The `data/*.json` loader pattern is now real code and the PRT/LRT registry has its template

## What this ADR does NOT cover

- **Atom B** (wasm-bindgen-test cross-target fingerprint) — the highest-risk deferred item, scoped out of Atom A because it requires a mini-council and non-trivial setup
- **Atom C** (planet.rs) — requires a full 5-agent council
- **P2 items** from the Crucible verdict — mutation testing, ChaCha XOR collision, rng.rs test count pruning
