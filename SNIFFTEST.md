# SNIFFTEST.md — The Sniff Test Protocol

> Inherited from BabyTierOS.md. Adapted for STARS 2026. Non-negotiable.

## The Rule

**After every function, run the sniff test. Then STOP.**

No batching. No "I'll test it after the next three functions." No "the unit test passes so we're good." STOP at every function.

## The Six Steps

For every function written or meaningfully changed:

### 1. Unit Test
- The function has at least one `#[test]` (Rust) or `it()` block (Vitest) in the same file or sibling test file.
- The test exercises the happy path AND at least one edge case.
- The test was written **with** the function, not after.
- `cargo test <module>::tests::<function>` passes.

### 2. Module Test
- All other tests in the same module still pass.
- `cargo test <module>` is green.

### 3. Acceptance Criteria Verification
- Identify which AC(s) from `SPEC.md` this function contributes to.
- Confirm the function moves us closer to that AC, not away from it.
- If no AC applies, ask: "should this function exist?"

### 4. Regression Check
- Full crate test suite passes: `cargo test --workspace`.
- No previously-green test went red.
- Frontend (if touched): `npm run check && npm run test`.

### 5. Manual Spot-Check
- Eyeball the diff. Is anything obviously off?
- If a UI change: open the page, click the thing, confirm it works visually.
- If an engine change with no UI: run the engine in a small REPL or test harness and confirm output matches expectation.

### 6. STOP
- Mark the task complete in TodoWrite.
- Report what was done and which tests passed.
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

When you finish a function, report exactly:

```
Function: engine::planet::habitability_for
Sniff test:
  ✅ Unit test (3 cases): habitability_for::tests passed
  ✅ Module test: cargo test planet — 12 passed
  ✅ AC: contributes to AC-1 (determinism), S-04 (hab formula)
  ✅ Regression: cargo test --workspace — 47 passed
  ✅ Spot-check: ran sample race vs sample planet, output matches starsfaq.com example
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
