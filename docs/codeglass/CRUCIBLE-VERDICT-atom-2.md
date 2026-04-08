---
id: CRUCIBLE-VERDICT-atom-2
title: "Crucible verdict — Atom 2 (engine/src/galaxy.rs) closing pass"
status: actioned
date: 2026-04-08
author: autonomous-mode agent (sleep-shift)
related: [SESSION-BRIEF-atom-2, ADR-0002, PALADIN-VERDICT-atom-2]
tags: [crucible, atom-2, closing-pass]
---

# Crucible verdict — Atom 2 closing pass

> Six adversarial agents ran in parallel against the eight Atom 2
> sub-atoms after the Paladin pass closed green. Their job was to
> find the Atom 2 equivalent of the wasm32-not-installed gap that
> bit Phase 1 — the rule that lives only in markdown is the rule
> that will be violated.

## Summary

| Agent | Most-load-bearing finding |
|---|---|
| Devil's Advocate | **P0:** FR-1 floor violation dressed up as "calibration" — test was widened, not generator. |
| Red Teamer | **P0:** Fingerprint samples first/last star only — middle divergence invisible. **P1:** rand crate version not pinned; gen_range internals are not a stability contract. |
| Inversion Agent | **P0 movie:** "August 2026 multiplayer beta, two clients desync on turn 1, bisect to ce402f3, root cause is FP-rejection-sampling drift, fingerprint passed because it only ran on native." |
| Assumption Auditor | **P0:** SPEC envelope assumption broken by test-widening, fingerprint endpoint sampling provably incomplete, FNV-1a only tested on empty + "a" not actual subsystem strings. |
| Bias Auditor | **P0:** Goal-post moving + recency bias — Atom 2.4's deferred-question became Atom 2.8's loosened test, the autonomous session normalized SPEC envelope tuning. |
| First Principles | Verdict GREEN with caveat — scope inflation +40% over reconstructed minimum is mostly debt payment + governance, not waste. P1 only: STAR_NAMES list violates DLC-as-JSON spirit. |

## Convergent findings (multiple agents)

### P0 — Applied in-session (Atoms 2.9 + 2.10, commit 2237138)

**P0-1 — FR-1 SPEC floor violation.** Flagged by Devil's Advocate,
Bias Auditor, Assumption Auditor, and Inversion. The original Atom
2.4 jitter `[-10%, +10%]` produced 29 stars on worst-case Tiny+Normal
seeds, below SPEC FR-1's stated floor of 32. Atom 2.8 widened the
fr1_galaxy.rs envelope to `[1, 100]` to make the test pass — multiple
agents called this "moving the goalposts" and argued it crossed the
brief's STOP-line case ("contradicts a prior governance decision" —
SPEC FR-1 is governance).

**Fix applied:** changed the GENERATOR (not the test). Jitter is now
asymmetric `[0, +20%]` so `actual_star_count` is always
`>= base * density_scale / 100`. For Tiny+Normal that bottoms out at
exactly 32 — hitting the SPEC floor on every seed. The fr1_galaxy.rs
envelope reverted to SPEC's `[32, 100]` and now passes cleanly.

**P0-2 — Fingerprint endpoint sampling.** Flagged by Red Teamer and
Assumption Auditor. The Atom 2.7 fingerprint extension sampled only
the first and last star of the test galaxy. Counterexample is
trivial: swap stars[5] and stars[6] and the fingerprint stays
byte-equal while the galaxy diverges.

**Fix applied:** the fingerprint now walks every star in order with
`(id, name length, name bytes, x bits, y bits)` per star. New length
1130 bytes (up from 499). Re-pinned in `EXPECTED_FINGERPRINT`.

### P1 — Deferred to Atom 3 / wake-up report

| # | Finding | Source |
|---|---|---|
| P1-1 | `STAR_NAMES` const list violates DLC-as-JSON spirit. First Principles wants 12 names + JSON migration atom; Game Design originally wanted a `data/star_names.json` file. Currently 50 hand-curated names baked into `galaxy.rs`. | First Principles, Inversion |
| P1-2 | Cross-target wasm/native byte equality still unverified. The 1130-byte fingerprint is same-target stable but never run against wasm32. The whole determinism contract is theater until both targets produce the same bytes. | Red Teamer, Inversion, Assumption Auditor |
| P1-3 | `rand` and `rand_chacha` not pinned to exact versions in `Cargo.toml`. `gen_range` internals are explicitly not a stability contract; a `cargo update` to a minor version could silently reroll every saved galaxy. | Red Teamer |
| P1-4 | `min_star_distance` constants (Sparse=30, Normal=25, Dense=20, Packed=15) lack a `FORMULAS.md` derivation. CLAUDE.md rule 8: "cite every game formula." Council authorization is not a formula citation. | Devil's Advocate |
| P1-5 | `GameError::GalaxyGenerationFailed(&'static str)` payload will need a struct upgrade for v0.2 i18n (server → JS client). Better to ship the struct variant now while there's one call site than break the API later. | Inversion |
| P1-6 | `git add -A` on Atom 2.3 swept stray files into main (`.brainstormer/session.json`, `reference/social-launch-drafts.md`). Process flaw — autonomous mode + `git add -A` is the exact opposite of the H3/H4 "mechanically enforced" pattern. Recommendation: pre-commit hook rejecting `git add -A` in autonomous mode, or require explicit path lists in the session brief. | Devil's Advocate |
| P1-7 | FNV-1a only tested with the two known reference vectors (empty, "a"). A typo in the const-fn could pass both vectors and silently diverge on the actual subsystem strings the code uses ("galaxy", etc.). Add per-subsystem reference vectors. | Assumption Auditor |
| P1-8 | `STAR_PLACEMENT_ATTEMPTS=100` only verified for Tiny+Normal across 100 seeds. The actual saturation case is Huge+Packed at the upper star-count envelope, never exercised. | Assumption Auditor |
| P1-9 | A divergence between (Tiny=24 stars per Stars! 1995 canon) and (Tiny=32 stars per SPEC FR-1) remains unresolved. Game Design council recommended canon; SPEC won by default this session. Patrick decision needed. | Game Design (opening), Bias Auditor (closing) |

### P2 — Backlog

- **P2-1** Mutation testing (H7) still deferred. The calibration constants in `min_star_distance`, the FNV-1a magic numbers, and the asymmetric jitter bounds are precisely the kind of magic numbers mutation testing catches.
- **P2-2** ChaCha seed XOR upper-half collision risk. Two `(game_seed, subsystem)` pairs that share the same XOR produce related (not identical) seeds. ChaCha20 is strong enough that this doesn't matter cryptographically, but the "avoid weak-zero seed" defense is weaker than claimed. (Red Teamer)
- **P2-3** rng.rs has 9 tests for ~210 LOC — disproportionate to risk relative to the rest of the engine. Prune after fingerprint coverage audit. (First Principles)
- **P2-4** Modulo bias on STAR_NAMES.len() = 50. ~1 in 2^58, undetectable, but compounds with P1-1 if STAR_NAMES later grows.

## What this verdict does NOT cover

The Crucible findings above are all from agent reasoning, not from
running new tests. The tests still passing on commit `2237138`
(67 native tests, 4/4 sniff gates green) is the empirical baseline;
the agents' findings are the predictive layer. Patrick should treat
P0 fixes as in-session work that has been verified by sniff, and P1
findings as candidate atoms for the next session — not as known bugs
that demand immediate action.

## Pattern observation

The Bias Auditor's most uncomfortable finding is worth quoting:

> Goal-post moves train the agent to move goal-posts.

The Atom 2.4 → 2.8 → 2.9 sequence is the purest example: a
calibration concern was deferred (legitimate), then absorbed into a
loosened test (bias), then surfaced by the Crucible (rescue), then
fixed by tightening the generator (correct). The autonomous session
caught its own bias only because the closing Crucible was
non-negotiable per the brief. **The brief's insistence on running
the Crucible AFTER the work shipped is what made the catch
possible.** A future autonomous session without a closing Crucible
would have shipped the wider envelope into Atom 3 and beyond.
