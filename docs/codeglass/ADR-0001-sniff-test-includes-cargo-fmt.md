---
id: ADR-STARS-2026-0001
title: "Sniff-test protocol includes `cargo fmt --check`, not just test + clippy"
status: active
date: 2026-04-07
author: patrick
chunk: "Phase 1 Task 1 — engine/src/types.rs"
tags: [sniff-test, workflow, ci, rust, rustfmt]
---

# ADR-STARS-2026-0001 — Sniff test includes `cargo fmt --check`

## Context

**What situation prompted this decision?**

During Phase 1 Task 1 (the `engine/src/types.rs` build-out, atoms 1.1–1.16),
the per-atom sniff-test loop I was running looked like this:

```
cargo test -p stars2026-engine
cargo clippy -p stars2026-engine --all-targets -- -D warnings
```

Every atom passed both checks. All 27 tests green, clippy pedantic clean,
no unsafe code, no unwraps, no todo!(). I pushed two commits to main:

- `0d8be3d` — atoms 1.1–1.14 (17/19 FR-19)
- `2180c78` — atoms 1.15–1.16 (19/19 FR-19, Phase 1 Task 1 complete)

GitHub Actions CI **failed** on `2180c78` with a red X on the repo page.

The failure was in the **Format check** step — `cargo fmt --check` found
7 places where `rustfmt` wanted different line-wrapping than I had
written:

1. `Minerals::new` struct literal — one-liner wanted multi-line
2. `ResearchAllocation::normalize` `saturating_sub` — multi-line wanted
   one-liner
3. `Colonists` derive list — too long for one line, wanted wrapping
4. `Cargo::total_mass` body — multi-line wanted one-liner
5. `AiDifficulty` derive list — too long, wanted wrapping
6. `LrtId` serialize `assert_eq!` — one-liner wanted multi-line
7. `GameSettings` decode `let` — multi-line wanted one-liner

Zero logic errors. Zero test failures. Zero clippy warnings. Pure
whitespace preferences enforced by `rustfmt` and checked by CI but
not by my local sniff-test loop.

The fix was a single `cargo fmt` command followed by a new commit
(`c1d7919 style(engine): apply cargo fmt to types.rs`), which CI
accepted (all four steps green: Format / Clippy / Build / Test).

**Why this matters:** the sniff-test protocol (see [SNIFFTEST.md](../../SNIFFTEST.md))
exists specifically to catch failures locally before they reach CI.
A gap in the protocol means CI is doing work the sniff test was
supposed to do, which defeats the point of the protocol.

## Decision

**What did we decide and why?**

The sniff-test loop for any Rust-touching atom in this project must
include `cargo fmt --check` as a mandatory step, run **after** test
and clippy, **before** STOP-for-approval. The updated canonical
loop is:

```
cargo test -p stars2026-engine
cargo clippy -p stars2026-engine --all-targets -- -D warnings
cargo fmt --check -p stars2026-engine
```

All three steps must exit 0 before a sniff test is considered green.
The `--check` flag is load-bearing: it makes rustfmt return non-zero
on drift without actually modifying files, so the sniff test does
not silently fix drift that should have been caught at authoring
time.

If `cargo fmt --check` reports drift, the fix is always `cargo fmt`
(no `--check`) followed by re-running the full sniff test from step
one — not just re-running the format step.

## Alternatives Considered

1. **Run `cargo fmt` (not `--check`) in the sniff test.** Rejected
   because it would silently re-format drift and let sloppy authoring
   get committed without the author ever noticing. The `--check`
   variant forces the author to acknowledge the drift and decide.

2. **Add a pre-commit hook that runs `cargo fmt` automatically.**
   Rejected for now because the project explicitly forbids skipping
   hooks (`--no-verify`) and the hook-based approach adds a layer of
   magic that masks what's actually happening. The sniff-test protocol
   is supposed to be explicit and readable, not hidden in git plumbing.
   May reconsider once the atom sequence is more mature.

3. **Rely on CI to catch format drift.** This is what was happening
   implicitly, and it's exactly what went wrong — CI caught the drift
   after I had already pushed, which is too late for the sniff-test
   protocol's "stop at every function" promise. The whole point of
   the sniff test is local enforcement. CI is a backstop, not the
   primary check.

4. **Write a single script that wraps all three checks.** Good idea,
   deferred until the atom sequence reaches a point where the
   repetition becomes painful. For now, three plain commands in a
   sniff-test report are more visible than a script.

## Consequences

**What are the trade-offs?**

- **+** Format drift is caught locally on the atom that introduced it,
  not on the next CI push (which may be atoms later).
- **+** Every sniff-test report now contains an explicit "fmt check:
  clean" line, making the discipline visible and auditable.
- **+** When another model or contributor picks up the session, they
  inherit the protocol by reading `SNIFFTEST.md` — no tribal
  knowledge.
- **−** Three commands per atom instead of two. Negligible wall-clock
  cost (~1 second for `cargo fmt --check` on this crate).
- **−** More cognitive load per atom — the author must notice a third
  gate. Mitigated by the cost of a CI failure being much higher.

## Expiration Conditions

**When should this decision be revisited?**

- If the Rust toolchain introduces a new canonical format-checking tool
  that supersedes `cargo fmt --check`, switch to that.
- If the atom sequence grows past ~50 atoms and the three-command
  cadence becomes unwieldy, wrap the three checks in a script
  (`scripts/sniff-test.sh`) and update `SNIFFTEST.md` to reference it.
- If `rustfmt` preferences start producing results the team disagrees
  with, configure them in `rustfmt.toml` at the workspace root rather
  than removing the check from the sniff test.
- Revisit no later than: 2026-10-07 (6 months).

## Related

- [SNIFFTEST.md](../../SNIFFTEST.md) — the sniff-test protocol spec.
  Next-session Tier 5 review atom will add `cargo fmt --check` to the
  "How To Report a Sniff Test Pass" example output and to the six-step
  checklist.
- [CLAUDE.md](../../CLAUDE.md) — Phase 1 Task 1 session record. The
  lesson is mentioned in the "Last Session" block.
- Commits:
  - `2180c78` — the commit that tripped the CI format-check failure.
  - `c1d7919 style(engine): apply cargo fmt to types.rs` — the fix.
- Memory: `C:/Users/pnmcg/.claude/projects/c--Dev-STARS-2026/memory/feedback_sniff_test_includes_fmt.md`
  (sibling memory file saved alongside this ADR so future sessions
  inherit the discipline even before reading the project docs).

## Unknown Origin

N/A — this decision was made with full context on 2026-04-07 after a
live CI failure during the Phase 1 Task 1 push.
