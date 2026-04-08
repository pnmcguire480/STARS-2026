---
id: H7-deferred
title: "Hardening Atom H7 — mutation testing deferred"
status: deferred
date: 2026-04-08
author: patrick (via agent)
chunk: "Phase 1 hardening pass (Crucible + Paladin response)"
tags: [hardening, mutation-testing, deferred, windows-toolchain, codeglass]
---

# H7 — mutation testing deferred to next session

## What this is

Hardening Atom H7 was supposed to run [`cargo-mutants`](https://mutants.rs)
across `engine/src/types.rs` to measure whether the existing test suite
actually catches semantic-preserving mutations, or whether mutants survive
(meaning the tests are theatrical — they pass without exercising the
behavior they claim to cover). This is the audit recommended by the Red
Teamer and Bias Auditor in the Phase 1 Crucible: 27 passing tests is a
count, not a measure.

## Why it's deferred

`cargo install cargo-mutants` failed on this machine with:

```
error: error calling dlltool 'dlltool.exe': program not found
error: could not compile `windows-sys` (lib) due to 1 previous error
error: failed to compile `cargo-mutants v27.0.0`
```

Root cause: the active Rust toolchain is `stable-x86_64-pc-windows-gnu`,
which builds dependencies that link against the MinGW toolchain. Some
windows-related crates in `cargo-mutants`'s dependency tree need
`dlltool.exe` (part of `binutils`) on `PATH`, and it's not installed in
the user's MinGW environment. Installing it is a non-trivial system
toolchain change that goes beyond the scope of the hardening pass.

Three options for closing this gap in a future session:

1. **Switch to the MSVC toolchain.** `rustup default stable-x86_64-pc-windows-msvc`
   (after installing the Visual Studio Build Tools' C++ workload). MSVC's
   linker doesn't need `dlltool`, and most Rust dev tooling targets MSVC
   as the Windows default. This is the cleanest fix but is a meaningful
   environment change for the user.

2. **Install `binutils` for MinGW.** `pacman -S mingw-w64-x86_64-binutils`
   from an MSYS2 shell, then add the MinGW bin directory to `PATH`.
   Smaller change than #1 but requires MSYS2 to be installed.

3. **Run mutation testing on CI only.** Configure a separate
   `mutation-test.yml` GitHub Actions workflow that runs `cargo-mutants`
   on Linux (where it builds without friction). The local sniff test
   stays as-is; the mutation gate runs nightly or on PR. This is the
   pattern several Rust projects use to keep local dev fast while
   still measuring test quality.

## Current state of test quality

Before deferring, here's the honest readout of where the test suite stands
without mutation testing:

| Signal | Value | Caveat |
|---|---|---|
| Unit tests in `types::tests` | **30** | Author-chosen paths only — survivorship bias applies |
| Integration tests in `tests/determinism.rs` | **2** | Same-target stability + pinned-byte fingerprint |
| Pub fn count | **24** | Ratio of 30:24 looks healthy but doesn't measure path coverage |
| Branch coverage | **unknown** | `cargo-llvm-cov` also not installed; same toolchain blocker |
| Mutation score | **unknown** | This atom |
| Determinism fingerprint coverage | **14 paths** in `compute_determinism_fingerprint` | Exercises arithmetic, serde, comparison, error paths |
| Sniff-test gates | test + clippy + fmt + wasm-check | Locally green, CI green |

The Crucible's specific concerns about test theater that were NOT closed by
the determinism gate:

- **`Minerals::add` overflow path:** explicitly named as untested in the
  doc comment (test 1335: "overflow is covered separately in later atoms")
  and still untested. A mutant that removes the `checked_add` overflow
  branch in `Minerals::add` and replaces it with `wrapping_add` would
  pass all 30 unit tests AND the determinism fingerprint, because no
  test drives the type to overflow.
- **`ResearchAllocation::normalize` integer rounding bias:** the
  remainder always lands in biotech (line 696). A mutant that distributes
  the remainder to electronics instead would pass every existing test
  because no test checks the remainder distribution shape.
- **`HabAxis::range` boundary behavior at the legal extremes:** the H5
  test covers `0`, `100`, `-1`, `101` — but not, e.g., `i32::MIN` or
  `i32::MAX`. A mutant that drops the `min < 0` check and only checks
  `min < -100_000` would still pass.

These are the kinds of holes mutation testing is designed to find. They
are real holes. They are deferred until the toolchain blocker is
resolved.

## Next-session action

Pick one of options 1, 2, or 3 above. If option 3 (CI-only), the
`.github/workflows/mutation.yml` file can be authored without any local
toolchain change — it just needs the `cargo-mutants` step to run on
`ubuntu-latest`, which already has the right toolchain. The cost is one
extra CI job per PR.

The Tier 5 review of `types.rs` should treat this as the highest-priority
followup item from the hardening pass.

## Related

- ADR-0002 (the umbrella ADR for the hardening pass)
- Crucible Red Teamer audit (in-session 2026-04-08)
- Crucible Bias Auditor finding #11 (survivorship bias on test coverage)
- `c:\Dev\STARS-2026\engine\src\types.rs` (the file under audit)
- `c:\Dev\STARS-2026\engine\tests\determinism.rs` (the partial mitigation)
