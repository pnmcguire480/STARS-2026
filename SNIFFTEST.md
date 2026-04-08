# SNIFFTEST.md — The Sniff Test Protocol

> Inherited from BabyTierOS.md. Adapted for STARS 2026. Non-negotiable.

## The Rule

**After every function, run the sniff test. Then STOP.**

No batching. No "I'll test it after the next three functions." No "the unit test passes so we're good." STOP at every function.

## How to run it

**Single command, single source of truth:**

```bash
bash scripts/sniff.sh
```

The script runs all four mandatory gates in order and exits non-zero on
any failure. CI runs the **same script verbatim**, so "I ran the sniff
test" and "CI will pass" are the same statement. There is exactly one
definition of "did the sniff test pass," and it lives in `scripts/sniff.sh`.

If you find yourself running individual `cargo` commands instead of the
script, stop — you are recreating the failure mode that ADR-0001 caught
on commit `2180c78`. The script exists because the human cannot be
trusted to remember every gate.

To extend the sniff test (add a new gate), edit BOTH `scripts/sniff.sh`
AND `.github/workflows/ci.yml`. The two are kept identical on purpose.
ADR-0002 covers the rationale.

## The Four Mandatory Gates

`scripts/sniff.sh` runs these in order. ALL FOUR must exit 0.

### Gate 1: Unit + module + workspace test (`cargo test`)
- Every public fn touched in this atom has at least one `#[test]` exercising the happy path AND at least one edge case.
- The test was written **with** the function, not after.
- ALL tests in the workspace still pass — no previously-green test went red.

### Gate 2: Lint (`cargo clippy --all-targets -- -D warnings`)
- Pedantic clippy with **deny warnings**. Any new warning fails the gate.
- The workspace's `clippy.toml` encodes governance rules as compile errors (HashMap → BTreeMap, etc.). New atoms must respect them.

### Gate 3: Format (`cargo fmt --check`)
- **The gate added by ADR-0001.** Author-time format drift is caught locally before it reaches CI.
- If this gate fails, run `cargo fmt -p stars2026-engine` to apply, then **re-run the FULL sniff test from gate 1** (a reformat can theoretically affect test output).

### Gate 4: WASM compile (`cargo check --target wasm32-unknown-unknown`)
- **The gate added by ADR-0002.** STARS 2026's architectural premise is dual-target compilation from one source. If this gate fails, the project has lost its reason to exist regardless of how green native is.
- This gate is the cheap proxy for the cross-target determinism contract. The full determinism gate is the integration test in `engine/tests/determinism.rs`, which must also pass on the wasm target once `wasm-bindgen-test` is wired (deferred — see ADR-0002).

## The Six Steps (for the human author, around the four gates)

For every function written or meaningfully changed:

### 1. Run gates 1–4 via `scripts/sniff.sh`
The script is the source of truth. Do not run individual cargo commands and call it a sniff test.

### 2. Acceptance Criteria Verification (human review)
- Identify which AC(s) from `SPEC.md` this function contributes to.
- Confirm the function moves us closer to that AC, not away from it.
- If no AC applies, ask: "should this function exist?"

### 3. Manual Spot-Check (human review)
- Eyeball the diff. Is anything obviously off?
- If a UI change: open the page, click the thing, confirm it works visually.
- If an engine change with no UI: confirm test output matches expectation by reading the assertion, not just the green check.

### 4. STOP
- Mark the task complete in TodoWrite.
- Report what was done — paste the `scripts/sniff.sh` output, not a summary.
- Wait for "go" before starting the next function.

## What Counts as "a function"

- A new public function in any module → sniff test.
- A meaningful change to an existing function → sniff test.
- Adding a new struct field that affects behavior → sniff test.
- A bug fix → sniff test (with a regression test added).
- Pure refactoring (rename, reorganize) with zero behavior change → no sniff test, but full `cargo test` still required.

## Anti-Shortcuts

Claude is notorious for:
- ❌ "I'll test these three functions together" — **No.** One at a time.
- ❌ "The compiler accepts it so it works" — **No.** Run the test.
- ❌ "It's a trivial getter" — **No.** Trivial getters get a trivial test.
- ❌ "I already have a test for the parent function" — **No.** Each leaf gets its own.
- ❌ "The user is in a hurry" — **No.** The user is paying for quality, not speed.
- ❌ Marking sniff test "passed" without showing test output — **No.** Show the output.

## How To Report a Sniff Test Pass

When you finish a function, paste the actual `scripts/sniff.sh` output —
do NOT summarize or paraphrase. The literal output is the proof.

Example (post-ADR-0002 format):

```
Function: engine::planet::habitability_for
Atom:     2.7 of N (Phase 2 — planet.rs)

▶ scripts/sniff.sh
  ✓ test       (native)         — 47 passed
  ✓ clippy     (pedantic, deny warnings)
  ✓ fmt        (rustfmt --check)
  ✓ wasm32     (dual-target gate)

  AC contribution:  AC-1 (determinism), S-04 (hab formula)
  Spot-check:       ran sample race vs sample planet, output matches
                    starsfaq.com hab formula example for race=Humanoid,
                    planet=(grav=50, temp=50, rad=50) → hab=100%

STOP — awaiting approval.
```

## Escalation

If a sniff test fails:
1. **Do not "fix it real quick" and re-run.** Report the failure first.
2. Identify the root cause.
3. Propose the fix.
4. Wait for approval before applying.
5. After fix, re-run **all six steps** from the top.

## Why This Matters

Stars! has hundreds of interlocking formulas. A subtle bug in habitability silently breaks population growth, which breaks resource generation, which breaks the production queue, which breaks combat readiness. Without sniff tests, you find the bug 40 functions later and have to bisect through everything.

The sniff test is the price of determinism. Pay it every time.
