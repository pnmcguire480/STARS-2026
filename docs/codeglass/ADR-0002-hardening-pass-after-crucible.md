---
id: ADR-STARS-2026-0002
title: "Hardening pass after the Crucible + Paladin verdict"
status: active
date: 2026-04-08
author: patrick (via agent)
chunk: "Phase 1 hardening pass — between Phase 1 Task 1 close and Atom 2 (galaxy.rs) start"
tags:
  - hardening
  - sniff-test
  - ci
  - wasm32
  - determinism
  - codeglass
  - decision-record
  - crucible
  - paladin
  - lessons-learned
---

# ADR-STARS-2026-0002 — Hardening pass after the Crucible + Paladin verdict

> **Sister ADR.** [ADR-0001](ADR-0001-sniff-test-includes-cargo-fmt.md)
> patched the symptom (missing `cargo fmt --check`) of a deeper structural
> gap in the sniff-test discipline. ADR-0002 closes the gap structurally
> after a multi-agent decision audit (the Crucible) and an automated
> testing-wall pass (Paladin) surfaced 21 findings, the scariest being
> that the **wasm32-unknown-unknown target had never been compiled** —
> meaning the project's defining "Rust + WASM" architectural claim was
> unverified across 16 atoms of foundational engine code.

## Context

### What triggered this ADR

After Phase 1 Task 1 closed (commit `ff864e3`, `engine/src/types.rs` at
27 tests / 19/19 FR-19 / clippy pedantic clean), the user asked for the
**Crucible** and **Paladin** to be run. Both produced significant
findings:

- **The Crucible** (six adversarial agents — Devil's Advocate, Red
  Teamer, Inversion Agent, Assumption Auditor, Bias Auditor, First
  Principles) identified 21 findings ranked by likelihood × blast radius.
  The headline: 14 of those were below MEDIUM evidence strength, 8
  were load-bearing, and 7 were both. Several agents independently
  identified the same gaps, which is strong-signal consensus.
- **Paladin** (the six-tier automated testing wall, run via the Rust
  reference) returned a verdict of **WARN** with one critical Tier-1
  failure: `cargo check --target wasm32-unknown-unknown` could not run
  because the target was not installed. The dual-target architectural
  claim from SPEC.md had never been empirically verified.

The user chose **Option 1** (full hardening pass before Atom 2), with
the explicit instruction to fix the P0 items, add the determinism gate,
and harden the protocol so the next gap can't happen.

### Why this matters

The Phase 1 Task 1 close commit `2180c78` failed CI on a format check
(see ADR-0001). That was a *symptom* — the underlying disease was that
the sniff-test protocol was an unenforced checklist in the human's
head. ADR-0001 patched the symptom by adding a fourth command. The
Crucible and Paladin found that the disease itself had several other
symptoms waiting to fire:

- The dual-target (wasm + native) claim was untested for 16 atoms.
- The "BTreeMap not HashMap" governance rule lived in markdown only;
  any future session that didn't read the memory file could violate it.
- `MineralConcentrations` had no checked arithmetic, so the next atom
  to need depletion would either reach into `pub` fields (silent
  wrap → determinism violation) or reinvent it inconsistently.
- `GameSettings` had no `#[serde(default)]` on any field, so the first
  field addition in v0.2 would brick every v0.1 save.
- `TechLevels::set` had no cap enforcement, so any caller could write
  garbage levels.
- `HabAxis::range` only validated `min ≤ max`, not `0 ≤ min ≤ max ≤ 100`.
- `PrtId(pub String)` and `LrtId(pub String)` let anyone construct
  arbitrary trait ids that the registry would never validate.
- The sniff-test protocol was still a checklist (ADR-0001 added a
  fourth command, but the script didn't exist yet — the human still
  had to remember to run all four).

The hardening pass closes all of these structurally — by code, by
script, by clippy.toml, by integration test — rather than by hope or
discipline.

## Decision

A nine-atom hardening pass (`H1` through `H9`), each its own sniff-test
cycle, lands before Atom 2 (`engine/src/galaxy.rs`) starts.

### H1 — Install `wasm32-unknown-unknown` and verify dual-target compile

`rustup target add wasm32-unknown-unknown`. Then `cargo check -p
stars2026-engine --target wasm32-unknown-unknown` clean. The dual-target
claim is now empirically true, not aspirational.

**Result:** Compiles cleanly on first try. No code changes required.
The architectural premise is verified.

### H2 — CI matrix wires the wasm32 build step

`.github/workflows/ci.yml` adds `targets: wasm32-unknown-unknown` to the
toolchain install and a "Build (wasm32)" step. CI now continuously
verifies dual-target compilation on every push and PR.

### H3 — `scripts/sniff.sh` becomes the single source of truth

Rather than maintain two parallel definitions of "sniff test" (one in
SNIFFTEST.md prose, one in CI YAML inline steps), both human and CI
now run `bash scripts/sniff.sh` verbatim. The script runs the four
gates in order:

1. `cargo test -p stars2026-engine`
2. `cargo clippy -p stars2026-engine --all-targets -- -D warnings`
3. `cargo fmt --check -p stars2026-engine` (the ADR-0001 gate)
4. `cargo check -p stars2026-engine --target wasm32-unknown-unknown` (the H1 gate)

Adding a check requires editing BOTH the script AND the CI YAML — the
two are kept identical on purpose. ADR-0001's "alternative #4" rejection
("wrap the three commands in a script — deferred for visibility") is
explicitly overturned: the visibility argument failed empirically on
commit `2180c78`, so the script wins.

### H4 — `clippy.toml` encodes the BTreeMap governance rule

`clippy.toml` at the workspace root uses `disallowed-types` to convert
the soft "memory file" governance rule into a compile error:

```toml
disallowed-types = [
    { path = "std::collections::HashMap", reason = "..." },
    { path = "std::collections::HashSet", reason = "..." },
]
```

The reason string cites the determinism rule and points back to memory
and ADR-0002. The rule was **smoke-tested live** during the hardening
pass: a temporary file with `use std::collections::HashMap; fn smoke() ->
HashMap<u32, u32> { HashMap::new() }` was added, clippy rejected it with
the custom reason, and the file was deleted. Verified firing.

### H5 — `types.rs` P0/P1 fixups

Seven targeted fixes to `engine/src/types.rs`:

| # | Fix | Source |
|---|---|---|
| 5.1 | `MineralConcentrations::deplete(kind, amount) -> Result` (atomic-on-failure decrement) | Red Teamer P0 |
| 5.2 | `PrtId.0` and `LrtId.0` → `pub(crate)`, with `as_str()` accessors | Red Teamer P1 |
| 5.3 | `TechLevels::set` returns `Result`; enforces `TECH_LEVEL_CAP = 30` | Red Teamer P1 |
| 5.4 | `pub const TECH_LEVEL_CAP: u32 = 30;` (codifies the signature deviation in code, not just memory) | Crucible First Principles |
| 5.5 | `HabAxis::range` validates `0 ≤ min ≤ max ≤ 100` (not just `min ≤ max`) | Red Teamer P1 |
| 5.6 | `Cargo::total_mass` uses `.units()` instead of `.0` (encapsulation precedent) | Red Teamer P1 |
| 5.7 | `TurnPhase::variant_count()` exhaustive-match tripwire + test asserting it equals `CANONICAL_ORDER.len()` | Red Teamer P1 |

Plus: `GameSettings` gets `#[serde(default)]` on every field except
`random_seed` (which must be a hard error if missing — the determinism
contract requires explicit seeds). `GalaxySize::Medium` and
`GalaxyDensity::Normal` get `#[default]` so the serde defaults work.

Three new tests added to `types::tests`: `tech_levels_set_enforces_cap_30`,
`mineral_concentrations_deplete_atomic_on_failure`,
`turn_phase_variant_count_matches_canonical_order_len`. Two existing tests
extended (`hab_axis_range_validates_bounds_and_ordering`,
`tech_levels_get_set_addresses_one_field`).

**Result:** 30 unit tests pass. Clippy + fmt + wasm gates green.

### H6 — Determinism gate at `engine/tests/determinism.rs`

Two integration tests:

1. `determinism_fingerprint_is_pinned` — calls
   `compute_determinism_fingerprint()`, which exercises 14 distinct
   paths through `types.rs` (Minerals arithmetic, Position::distance_to,
   Colonists newtype math, MineralConcentrations::deplete, TechLevels
   meets_requirements, HabAxis::range happy/error, Cargo::total_mass,
   GameSettings JSON round-trip, TurnPhase counts, Cost, Environment,
   ID newtypes, GameStatus default), collects bytes, and asserts
   equality with a 406-byte pinned constant `EXPECTED_FINGERPRINT`.

2. `determinism_fingerprint_stable_across_runs` — calls
   `compute_determinism_fingerprint` twice in the same test and asserts
   the bytes are identical. Catches the trivial case where ambient
   state (RNG, time, env) accidentally leaks into the computation.

The fingerprint was captured on the first run (the test prints the
bytes if `EXPECTED_FINGERPRINT` is empty and panics with instructions),
then pinned. The integration test compiles cleanly for
`wasm32-unknown-unknown`, confirming no wasm-incompatible features are
used.

**Cross-target byte equality is NOT yet verified.** The test runs on
whichever target `cargo test` is targeting (currently native on local,
native on CI). Wasm execution requires `wasm-bindgen-test` and either
`wasm-pack test --node` or a browser-based runner — both significant
scope additions deferred until the WASM bridge atom (likely Atom 5–8).

The Red Teamer's "kill shot" scenario (FMA fusion drift between targets
producing divergent `Position::distance_to` results, leading to
multiplayer desync in v0.2) is the threat this gate addresses. If the
gate fails after the bridge atom wires `wasm-bindgen-test`, the
fingerprint divergence pinpoints the exact path that drifted. Today
the gate locks in same-target stability and provides the byte sequence
that future cross-target runs will be compared against.

### H7 — Mutation testing deferred

`cargo install cargo-mutants` failed on Windows-GNU toolchain due to
missing `dlltool.exe`. Three paths forward documented in
[`H7-mutation-testing-deferred.md`](H7-mutation-testing-deferred.md):
switch to MSVC toolchain, install MinGW binutils, or run
`cargo-mutants` on Linux CI only. The honest gap is named: three
specific holes in the test suite are listed (Minerals overflow path,
ResearchAllocation remainder distribution bias, HabAxis extreme i32
values) that mutation testing would have caught and the current
suite doesn't.

### H8 — `SNIFFTEST.md` references the script

The protocol spec now points at `scripts/sniff.sh` as the single source
of truth, lists all four mandatory gates with their ADR provenance
(ADR-0001 for fmt, ADR-0002 for wasm), and updates the "How To Report"
example to show the new format with all four gates pasted from the
script's literal output.

### H9 — This ADR

Captures the full hardening pass and the Crucible/Paladin findings that
drove it, so the next session and any future contributor can read the
context without re-running the audit.

## Alternatives Considered

### A1. Just fix the P0 items, skip the protocol changes

Rejected. The P0 fixes alone would close today's gaps but not prevent
tomorrow's. Hardening the protocol (`scripts/sniff.sh`, `clippy.toml`)
is what makes the *next* gap fail loudly instead of silently.

### A2. Defer the wasm32 install to the WASM bridge atom

Rejected. Running 16 foundational atoms without ever compiling for
the target the project exists to serve is the kind of compounding
unverified-assumption debt the Crucible was specifically designed to
flush out. Installing `wasm32-unknown-unknown` is one rustup command
and ~30 seconds; doing it now means every subsequent atom inherits
empirical dual-target verification.

### A3. Use `IndexMap` instead of banning `HashMap`

Considered. `IndexMap` provides insertion-order iteration with O(1)
lookup, which is faster than BTreeMap's O(log n). Rejected on
simplicity grounds: BTreeMap is in std (no new dep), is the cheapest
mental model, and the perf difference at the project's scale (16
players × 150 stars × 33 turn phases) is negligible. If profiling
later shows BTreeMap as a hot-path bottleneck, IndexMap can be added
as a second-tier exception via per-callsite `#[allow(...)]` and a
documented justification.

### A4. Hash-based determinism gate (SHA256) instead of byte fingerprint

Considered. SHA256 of the byte sequence would be 32 bytes instead of
406, much more compact in the source. Rejected because: (a) `sha2`
adds a dependency, (b) the 406-byte raw sequence is human-inspectable
and a divergence is easier to localize ("byte 145 changed" vs "the
hash changed"), (c) the byte sequence is cheap to grow when new types
land. May reconsider when the fingerprint exceeds ~5 KB.

### A5. Skip mutation testing entirely

Rejected. Mutation testing is the only way to distinguish "tests that
exercise the path" from "tests that pass without exercising the path."
Deferring is acceptable; skipping is not. H7 documents the
toolchain-blocker reason for the deferral and three concrete paths to
unblock it.

## Consequences

- **+** The dual-target architectural claim is now empirically verified
  on every push, not aspirational.
- **+** The "BTreeMap not HashMap" rule is mechanically enforced — a
  future session that doesn't read the memory file still cannot violate
  it without a compile error.
- **+** The sniff-test protocol is structurally robust (script-based,
  not memory-based) and CI runs the same script verbatim.
- **+** Seven concrete code-level landmines in `types.rs` are defused
  before any consumer touches them.
- **+** The Red Teamer's "kill shot" scenario (FMA fusion drift across
  targets) has a specific failure-detection mechanism (the determinism
  fingerprint) waiting for the cross-target test runner to wire up.
- **+** ADR-0002 itself becomes the template for future hardening
  passes — when the next gap surfaces, the response is "another atom
  in this style."
- **−** ~9 atoms of pure infrastructure work added between Phase 1
  Task 1 close and Atom 2 (galaxy.rs) start. Phase 1 timeline shifts
  later by that much.
- **−** The mutation-testing gap is documented but not closed. Three
  named holes in the test suite remain known and unmitigated until
  the next session resolves the toolchain blocker.
- **−** Cross-target determinism (the actual byte equality between
  wasm and native) is verified only at the compile level, not yet
  at the runtime level. Same-target stability is locked in; the
  cross-target lock-in waits for `wasm-bindgen-test`.

## Expiration Conditions

Revisit this ADR if:

- The Rust toolchain or `rustup` introduces a different way to verify
  cross-target compilation that obsoletes the current `--target`
  pattern.
- A future atom adds a fourth target (e.g. `aarch64-apple-darwin`
  for native macOS builds) and the sniff-test script needs a matrix.
- `cargo-mutants` becomes installable on the active toolchain and
  H7 can be retroactively closed.
- The 406-byte determinism fingerprint exceeds ~5 KB and a hash-based
  representation becomes worthwhile.
- BTreeMap shows up as a measured perf bottleneck and IndexMap
  becomes a justified exception.

**Review no later than:** 2026-10-08 (6 months).

## Related

### ADRs
- [ADR-0001 — Sniff test includes cargo fmt --check](ADR-0001-sniff-test-includes-cargo-fmt.md) — the symptom-level patch that this ADR's structural fix supersedes
- [H7 — Mutation testing deferred](H7-mutation-testing-deferred.md) — the documented gap

### Files changed in the hardening pass
- `engine/src/types.rs` (H5 fixes — 7 changes)
- `engine/tests/determinism.rs` (H6 — new file)
- `scripts/sniff.sh` (H3 — new file)
- `clippy.toml` (H4 — new file)
- `.github/workflows/ci.yml` (H2 — wasm matrix step + sniff.sh wire)
- `SNIFFTEST.md` (H8 — references script)
- `docs/codeglass/ADR-0002-hardening-pass-after-crucible.md` (H9 — this file)
- `docs/codeglass/H7-mutation-testing-deferred.md` (H7 — documented gap)

### Persistent memory
- `c:\Users\pnmcg\.claude\projects\c--Dev-STARS-2026\memory\feedback_determinism_btreemap.md`
- `c:\Users\pnmcg\.claude\projects\c--Dev-STARS-2026\memory\feedback_sniff_test_includes_fmt.md`
- `c:\Users\pnmcg\.claude\projects\c--Dev-STARS-2026\memory\project_tech_cap_30.md`
- `c:\Users\pnmcg\.claude\projects\c--Dev-STARS-2026\memory\project_colonists_newtype.md`
- `c:\Users\pnmcg\.claude\projects\c--Dev-STARS-2026\memory\project_prt_data_driven.md`

### Crucible session output (in-conversation, not saved to file)
- Devil's Advocate audit
- Red Teamer audit (kill-shot scenario, 12 specific code findings, 6 sniff-test gaps)
- Inversion Agent audit (4 load-bearing guardrails)
- Assumption Auditor audit (14 assumptions, 7 load-bearing + weak)
- Bias Auditor audit (12 biases, consensus-illusion identified as #1)
- First Principles audit (8 questions, 4 mismatches with what we did)

### Paladin verdict (in-conversation)
- Tier 1 PARTIAL PASS (wasm32 not installed → fixed in H1)
- Tier 2 PASS (27 tests, healthy ratio)
- Tier 3 N/A (no integration tests pre-H6 → added in H6)
- Tier 5 PARTIAL PASS (no coverage tool, no determinism gate → H7 documents, H6 adds)

## Pattern name

**"Local-first verification, mechanically enforced."** The structural
upgrade over ADR-0001's "local-first verification" pattern is the word
"mechanically." A pattern that depends on the human running the right
checks every time is not a pattern — it's a wish. ADR-0002 closes the
loop by:

1. Encoding the rules in tools (`clippy.toml`, `cargo fmt --check`)
2. Wrapping the tools in a script (`scripts/sniff.sh`)
3. Making CI run the script verbatim (no parallel definition possible)
4. Adding integration tests for properties tools cannot check (the
   determinism fingerprint)

Any future "this should have been caught locally" failure points at a
gap in one of those four layers, not at human discipline. Discipline is
finite; mechanical enforcement is not.
