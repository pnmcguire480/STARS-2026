---
id: SESSION-BRIEF-atom-2
title: "Session brief — Atom 2 (engine/src/galaxy.rs) autonomous run"
status: active
date: 2026-04-08
author: patrick (via prior-session agent)
mode: autonomous
tags: [session-brief, atom-2, galaxy, autonomous-mode, codeglass]
---

# Session brief — Atom 2 (`engine/src/galaxy.rs`) autonomous run

> **Read this file FIRST. Before anything else.** This is the runway for an
> authorized autonomous session. Patrick is asleep. He has explicitly
> delegated decision authority for this session to the agent within the
> boundaries below. Do not require approval at every atom. Run the work.

---

## Mission

**Complete `engine/src/galaxy.rs` end-to-end** through every atom in the
sequence below. Stop only when:

1. The full atom sequence is shipped, sniff-tested, and committed.
2. The closing Paladin + Crucible verdict has been written to a final
   ADR.
3. The brainstormer sync is up to date.
4. CI is green on the final commit.

**Then write a wake-up report at the top of CONTEXT.md** describing what
happened while Patrick slept.

---

## Authorization (read carefully)

### What you MAY do without approval

- Write new Rust code in `engine/src/galaxy.rs` and any new module files
  that the atom sequence requires (`engine/src/rng.rs`, etc.).
- Add new tests to `engine/src/galaxy.rs::tests`, new integration tests
  in `engine/tests/`, and extend the determinism fingerprint in
  `engine/tests/determinism.rs` as new public arithmetic paths land.
- Update `engine/src/lib.rs` to wire new modules with `pub mod ...`.
- Update `engine/src/types.rs` if (and only if) Atom 2 reveals a missing
  type or method that should have been in the type vocabulary. Justify
  each such change in the commit message.
- Update `CLAUDE.md`, `CONTEXT.md`, and `.brainstormer/session.json` at
  every brainstormer-sync milestone (cadence below).
- Commit and push to `main` after every atom whose sniff test is green.
  Do **not** batch atoms into a single commit — one atom per commit so
  the history is granular.
- Spawn subagents for the council, syncs, and the closing Paladin +
  Crucible.
- Create new ADRs in `docs/codeglass/` when a non-obvious decision is
  made. The ADR-0001/ADR-0002 template is in those files.
- Run `cargo fmt -p stars2026-engine` to fix format drift before
  re-running the sniff test.
- Update memory files in `c:\Users\pnmcg\.claude\projects\c--Dev-STARS-2026\memory\`
  if a new feedback or project memory becomes load-bearing.

### What you MAY NOT do without explicit approval

- **Add new dependencies** to any `Cargo.toml`. The current dep list
  (`serde`, `serde_json`, `bincode`, `rand`, `rand_chacha`, `thiserror`)
  is sufficient for galaxy generation. If you find yourself wanting to
  add a crate, **stop and document the reason in a new file**
  `docs/codeglass/Atom-2-deferred-dependency-<name>.md` and route around
  it for now.
- **Modify SPEC.md** or any of the canonical governance docs (AGENTS.md,
  CODEGUIDE.md, ART.md, ARCHITECTURE.md, README.md). If a SPEC.md update
  is needed, document it as a deferred atom in CONTEXT.md "Open
  Questions" and let Patrick handle it on his return.
- **Change the four governance decisions** (PRT data-driven, BTreeMap
  not HashMap, Colonists newtype, Tech cap 30). These are load-bearing
  and were made with Patrick's explicit consent. If a governance
  decision feels wrong during Atom 2 implementation, **stop, document
  the conflict in a new ADR, and route around it for now** — do not
  silently override.
- **Skip the sniff test** between atoms. The sniff test is the contract.
  If an atom fails sniff, fix it and re-sniff before moving to the next
  atom. No exceptions.
- **Skip the council summon** at the start. The round table is the
  first thing you do. Even if it slows you down by 5 minutes.
- **Push if CI fails.** If a push triggers a red CI run, stop, fix
  locally, re-push. Don't let red builds accumulate.
- **Use `--no-verify` on git commits** or any flag that bypasses hooks.
- **Touch the `reference/legacy-desktop-scaffold/`** directory for
  anything other than read-only study. The IP-clean rule applies.

### When to STOP and wait for Patrick

- A council member raises a load-bearing objection that contradicts a
  prior governance decision.
- The determinism fingerprint diverges between two same-target runs
  (this means ambient state is leaking — a real bug).
- The wasm32 build breaks and the fix isn't obvious from the error
  message.
- Test mortality: more than two consecutive sniff-test failures on
  the same atom suggest you're confused about the design, not just
  the implementation.
- You discover that a P0 hardening assumption was wrong.

In any STOP case: write a brief note at the top of CONTEXT.md "Now"
section ("PAUSED — needs Patrick: <reason>"), commit it, and wait.
Patrick will resume when he's awake.

---

## Pre-flight checklist

**Read these files in this order, before doing anything else:**

1. `CLAUDE.md` — project intelligence + governance rules
2. `c:\Users\pnmcg\.claude\projects\c--Dev-STARS-2026\memory\MEMORY.md`
   and the 5 memory files it points at — the four governance decisions
   plus the sniff-test feedback memory
3. `SPEC.md` — at minimum, FR-1 (galaxy generation), FR-19/20 (status),
   AC-1/AC-2 (determinism + perf budget)
4. `SNIFFTEST.md` — the protocol (now references `scripts/sniff.sh`)
5. `AGENTS.md` — the Galaxy generation council assignment
6. `engine/src/types.rs` — the type vocabulary you'll be using
7. `engine/tests/determinism.rs` — the gate you'll be extending
8. `docs/codeglass/ADR-0001-sniff-test-includes-cargo-fmt.md`
9. `docs/codeglass/ADR-0002-hardening-pass-after-crucible.md` — the
   pattern this session inherits ("local-first verification,
   mechanically enforced")
10. `docs/codeglass/H7-mutation-testing-deferred.md` — open work
11. `scripts/sniff.sh` — the source of truth for the sniff test
12. `clippy.toml` — the compile-enforced governance rules
13. **This file** (you're reading it).

Confirm each file's existence by reading or listing it. If any of these
files is missing, **stop immediately** — the project state has drifted
and the brief is no longer accurate.

---

## Step 1 — The agentic round table

Before any code is written, summon the council in **parallel** (one
message, multiple Agent tool calls) and wait for all responses. The
prompts are pre-written below. Copy them verbatim — they're calibrated
to give independent, high-signal output.

**The five-agent council for galaxy.rs:**

| Agent | Role | Why |
|---|---|---|
| `Rust` | Idiom + RNG patterns | Seeded ChaCha20 setup, Result handling, no unwrap in lib |
| `Plan` | Atom sequencing | Confirm or revise the proposed sequence below |
| `Performance Engineer` | AC-2 budget | 200 turns in 60s — galaxy gen is one slice |
| `Game Design` | Stars! canon fidelity | Density curves, name lists, FR-1 details from craig-stars and starsfaq |
| `First Principles` | Scope discipline | Crucible's Q1 found the prior session shipped 4× more types than Phase 1 needed; do not repeat |

**Why these five and not the maximalist set:** the Bias Auditor's #1
finding from the Phase 1 Crucible was that unanimous council agreement
from a single-model family is **correlation, not validation**. Five
focused voices with distinct mandates produce more signal than ten
overlapping ones. Refactoring is folded into Rust's mandate; the
oracle perspective is folded into First Principles.

### Council prompts (copy-paste, parallel)

**Prompt 1 — Rust:**
```
You are being summoned as the Rust specialist on the Atom 2 council
for STARS 2026 — a Rust+WASM+SvelteKit remake of the 1995 4X game
Stars!. We are about to write fresh `engine/src/galaxy.rs` for
procedural galaxy generation.

Context:
- Phase 1 Task 1 (`engine/src/types.rs`) is complete and hardened
  by ADR-0002. 32 tests passing, clippy pedantic clean, dual-target
  build verified, BTreeMap-not-HashMap mechanically enforced via
  clippy.toml, determinism fingerprint pinned at 406 bytes.
- Workspace deps (locked, no new deps without Patrick approval):
  serde, serde_json, bincode, rand, rand_chacha, thiserror.
- All RNG must use rand::SeedableRng (ChaCha20Rng from rand_chacha).
  No `thread_rng`. Ever.
- All fallible code returns `Result<T, GameError>`. No `unwrap` in
  lib code. No `todo!()`, no `unimplemented!()`.
- Engine is `#![forbid(unsafe_code)]` and `#![warn(clippy::pedantic)]`.

Task: write the foundational galaxy generator. FR-1 calls for 32–100
stars on a Tiny galaxy from a seed. SPEC AC-1 requires same-seed →
same-galaxy (deterministic). AC-2 requires the full game to fit in
60s of compute, so galaxy generation must be cheap.

Your advice (terse — under 400 words total):

1. The seeded RNG primitive — should it be a free function
   `seeded_rng(game_seed: u64, turn: u32, player_id: PlayerId,
   subsystem: &'static str) -> ChaCha20Rng`, a struct with methods,
   or something else? What's the idiomatic Rust pattern that's also
   deterministic across wasm/native?

2. How should galaxy.rs derive sub-seeds from the master `(game_seed,
   turn, player_id, subsystem)` tuple? Hash-based? Concatenation?
   ChaCha20Rng's `seed_from_u64`?

3. Star placement uses rejection sampling (place a random point,
   reject if too close to existing stars, retry up to N times).
   What's the right Rust idiom for this? Iterator chain? Manual loop?
   Where does the retry budget live?

4. What's the right error path when rejection sampling fails (e.g.,
   density too high for the requested star count)? Return
   `GameError::GalaxyGenerationFailed("...")`?

5. Performance: the AC-2 budget is 60s for 200 turns of a full game.
   Galaxy generation is called ONCE at game start, but it will be
   called many times in CI determinism tests. What's the upper bound
   on per-galaxy-generation time we should target?

Return a numbered list. Be opinionated. Do not write code.
```

**Prompt 2 — Plan:**
```
You are being summoned as the Plan architect for Atom 2 of STARS 2026
— writing fresh `engine/src/galaxy.rs`. The prior agent has proposed
the following atom sequence; your job is to confirm it, revise it, or
replace it.

Hard constraints:
- One atom = one cohesive function or function-cluster + its tests.
- After every atom: sniff test (test/clippy/fmt/wasm via
  scripts/sniff.sh), then commit, then next atom.
- The session is authorized to run autonomously through the full
  sequence — there is no per-atom Patrick approval gate, but every
  atom MUST sniff-green before the next one starts.
- Each new atom that introduces public arithmetic MUST extend
  `compute_determinism_fingerprint` in engine/tests/determinism.rs
  and re-pin EXPECTED_FINGERPRINT.
- Total target: 8–10 atoms for the full galaxy.rs MVP.

Proposed atom sequence (your job: confirm or revise):

  Atom 2.1 — `engine/src/rng.rs`: seeded_rng helper +
              determinism test.
  Atom 2.2 — Star name list (compile-time const array of canonical
              Stars!-flavored names) + deterministic name picker
              from RNG.
  Atom 2.3 — `random_position(rng, dimension) -> Position`: pick a
              random point inside the square map.
  Atom 2.4 — `actual_star_count(size, density, rng) -> u32`: jitter
              the GalaxySize::target_stars by density.
  Atom 2.5 — `place_one_star_with_retry(...)` rejection sampling
              against an existing star list.
  Atom 2.6 — `place_all_stars(...)` loop that calls 2.5 N times,
              respecting GalaxySize::min_homeworld_distance.
  Atom 2.7 — `Galaxy { stars: Vec<Star>, size: GalaxySize, density:
              GalaxyDensity, seed: u64 }` struct + constructor.
  Atom 2.8 — `generate_galaxy(settings: &GameSettings) -> Result<
              Galaxy, GameError>`: top-level entry point.
  Atom 2.9 — Determinism fingerprint extension: add galaxy gen calls
              to compute_determinism_fingerprint, re-pin bytes.
  Atom 2.10 — (optional) FR-1 verification test: assert a fixed seed
               produces a galaxy in [32, 100] stars for Tiny size.

Your deliverable (under 400 words):

1. Is this sequence sound? Is the order right? Are atoms missing or
   redundant? Specifically: does the RNG primitive (2.1) need to live
   in its own module, or should it be a top-of-galaxy.rs helper?
2. Where should `Galaxy` struct live — types.rs (with Star/Planet) or
   galaxy.rs? The other big structs (Planet, Star) live in types.rs;
   does Galaxy belong there too, or is it a galaxy.rs concern?
3. Should planet generation be IN galaxy.rs (one module, full map
   generation) or DEFERRED to engine/src/planet.rs (Atom 3)? FR-1 only
   requires stars; SPEC FR-2 needs starting worlds (planets).
4. Are there atoms that could be parallelized into one bigger atom
   without hurting reviewability?
5. What's the natural sync milestone — every 3 atoms? After 2.6?

Return a numbered list. Do not write code.
```

**Prompt 3 — Performance Engineer:**
```
You are being summoned as the Performance Engineer for Atom 2 of
STARS 2026 — fresh `engine/src/galaxy.rs`. AC-2 requires a full
200-turn game to finish in under 60 seconds of compute. Galaxy
generation is called once per game, BUT a determinism CI gate runs
it many times per push.

Your task (terse — under 400 words):

1. What's the realistic per-galaxy-generation budget? 10ms? 50ms?
   100ms? Galaxy generation includes: ChaCha20 seeding, 32–100 random
   positions, 32–100 random names from a const list, ~100 rejection
   samples (rejected stars don't count toward the final list, so
   loops may exceed 100 iterations), an N×N pairwise distance check
   per placement (~5000 distance calls in the worst case for 100
   stars).

2. Allocation patterns: should the star list pre-allocate to capacity
   (Vec::with_capacity) or grow naturally? Rejection sampling will
   discard candidates — does that imply a Vec<Position> scratch pad
   that gets reused, or per-star throwaway?

3. ChaCha20Rng is the deterministic choice. How fast is it relative
   to thread_rng (which we banned)? Is the seeding cost
   (`SeedableRng::seed_from_u64`) a one-time per-galaxy or per-call?

4. The clippy.toml ban on HashMap means we use BTreeMap. For a
   Vec<Position> scratch pad of 100 elements, BTreeMap vs Vec is
   irrelevant — but if any future galaxy code needs lookup, BTreeMap
   it is. Are there places in the proposed atom sequence where this
   choice has measurable impact?

5. The wasm32 target may have different performance characteristics
   than native. Anything in the proposed galaxy code that would be
   especially fragile under wasm? (Float math is the obvious one,
   but galaxy gen uses integer star count and i32-tagged positions.)

Return a numbered list with concrete numbers where possible. Do not
write code.
```

**Prompt 4 — Game Design:**
```
You are being summoned as the Game Design specialist for Atom 2 of
STARS 2026 — fresh `engine/src/galaxy.rs`. Stars! 1995 had a very
specific feel for its galaxy generation that the remake must honor.

Reference sources (Patrick has approved web search if needed):
- starsfaq.com (galaxy generation section)
- wiki.starsautohost.org (galaxy/star tables)
- craig-stars on GitHub (Go reference, MIT — module boundaries
  only, never copy code)

Your advice (under 400 words):

1. Stars! star NAMES — what's the canonical naming convention? Real
   star names (Vega, Antares)? Procedural greek-letter style (Alpha
   Centauri)? Numbered (Star 247)? Mix? What's the size of the
   canonical list and how does it handle running out?

2. GalaxyDensity — STARS 2026's enum is Sparse/Normal/Dense/Packed.
   What's the canonical Stars! mapping from density to actual
   star-spacing behavior? Is density a clustering effect (some
   areas dense, some sparse), a global multiplier on
   min_homeworld_distance, or something else?

3. Star placement — Stars! veterans expect homeworlds to be
   *roughly* equidistant from each other for fairness. Is this
   handled at galaxy generation (place homeworlds, fill in around
   them) or as a separate "balanced placement" pass? Should the
   first 1–4 stars (the player count from GameSettings) be the
   homeworlds, or are homeworlds chosen later from the placed stars?

4. The 32-star floor for Tiny — is that canonical or arbitrary?
   What's the canonical Stars! Tiny range? STARS 2026's
   GalaxySize::Tiny.target_stars() returns 32 from
   `engine/src/types.rs`; is that the right number?

5. Anything else from the canon that an engineer would miss but a
   designer would catch? (E.g., star color/type isn't in the
   current Star struct — should it be?)

Return a numbered list with sources. Do not write code. Cite
starsfaq.com or wiki URLs where you can.
```

**Prompt 5 — First Principles:**
```
You are being summoned as the First Principles auditor for Atom 2 of
STARS 2026 — fresh `engine/src/galaxy.rs`. Your job: prevent the
scope creep that the Phase 1 Crucible found in Phase 1 Task 1 (the
prior session shipped 4× more types than Phase 1 strictly needed,
because "what the game will eventually need" got conflated with
"what Phase 1 needs").

Strip away inheritance, analogy, and habit. Answer from fundamentals.

The project's first-principle goals (relevant subset):
- A deterministic engine compiles to both wasm32 and native.
- v0.1 plays a 200-turn AI game in 60 seconds (AC-2).
- FR-1 says "Generate a procedural galaxy from a seed (32–100 stars
  for v0.1, Tiny size)."
- The galaxy will be consumed by planet placement (FR-2), then
  population (FR-5), then everything else.

Questions:

1. **MINIMUM SCOPE for galaxy.rs.** What is the smallest galaxy.rs
   that satisfies FR-1 AND unblocks FR-2 (placing players on
   starting worlds)? Not "what we'll need eventually" — what's the
   ground floor? A Vec<Star> placed by seed. That's it. Anything
   else is scope creep.

2. **What MUST be in galaxy.rs vs deferred to planet.rs?** Stars
   without planets satisfy FR-1. Planets are FR-2's problem.
   Defending the line is the whole point of this audit.

3. **The seeded RNG primitive.** Does it belong in galaxy.rs (single
   file), engine/src/rng.rs (separate module), or types.rs (the
   vocabulary file)? First principles: which placement minimizes
   future churn? The RNG primitive will be used by galaxy.rs,
   planet.rs, race.rs, combat.rs, AI — it has the broadest reach.
   Where does the broadest-reach utility live?

4. **The Galaxy struct itself.** Does it need to exist as its own
   type, or is `Vec<Star>` sufficient for v0.1? Stars! 1995 didn't
   have a `struct Galaxy` — it had a flat list of stars. Are we
   over-engineering by introducing the wrapper now? Or is the
   wrapper load-bearing because it carries the seed and size for
   determinism replay?

5. **What atoms in the proposed sequence are SCOPE CREEP?** Be
   specific. Cite atom numbers from the Plan agent's prompt above.

Return a numbered list. Be opinionated. Reject anything that doesn't
serve goals 1–4. The Crucible found Phase 1 went 4× over scope; do
not repeat the mistake here.
```

**Synthesize the council output** (after all 5 return) into a single
ranked action list. Where they agree: act on it. Where they disagree:
prefer First Principles + Game Design (canon + scope discipline) over
Rust + Plan + Performance Engineer (idiom + sequencing + budget) when
the disagreement is about *what to build*. Prefer the latter group when
the disagreement is about *how to build it*.

If the council reveals a fundamental flaw in the proposed atom sequence
(e.g., "the RNG primitive belongs in `rng.rs`, not at the top of
galaxy.rs"), revise the sequence in CONTEXT.md before starting Atom 2.1.

---

## Step 2 — The atom sequence (default plan, council may revise)

Each atom is a sniff-tested cycle. Run them in order. After every atom:

1. Run `bash scripts/sniff.sh`.
2. If green: commit and push (one atom = one commit).
3. If red: fix and re-sniff. Do not move to the next atom.
4. Update the determinism fingerprint if the atom introduces public
   arithmetic (see "Determinism fingerprint protocol" below).

### Atom 2.1 — Seeded RNG primitive (`engine/src/rng.rs`)

A new module file `engine/src/rng.rs` with a `seeded_rng` constructor:

```rust
pub fn seeded_rng(
    game_seed: u64,
    turn: u32,
    player_id: PlayerId,
    subsystem: &'static str,
) -> ChaCha20Rng
```

The function derives a sub-seed from the inputs and returns a fresh
ChaCha20Rng. Test: same inputs → same first 8 random `u64`s.

This atom unblocks every other atom in galaxy.rs and every future
RNG-using module (planet.rs, race.rs, combat.rs, AI).

Wire `pub mod rng;` into `engine/src/lib.rs`.

### Atom 2.2 — Star name registry

A small const array of canonical-feeling star names in `galaxy.rs`,
plus a function `pick_star_name(rng) -> &'static str` that picks one
without state (each call is independent — duplicate names are OK at
this stage; uniqueness is a polish atom).

Council (Game Design) will rule on whether the names should be real
stars, Greek letter conventions, or procedural. Until ruled, use a
small list of ~50 Stars!-flavored names. Test: same RNG → same name.

### Atom 2.3 — Random position generator

`fn random_position(rng: &mut ChaCha20Rng, dimension: u32) -> Position`
that picks an integer-aligned `(x, y)` inside the square map. Use
integer math via `rng.gen_range(0..dimension)` and cast to f64 for the
Position fields. Determinism wins from integer placement; the f64 is
just for type compatibility with `Position`.

Test: 1000 positions, all within `0.0..dimension as f64`.

### Atom 2.4 — Actual star count

`fn actual_star_count(size: GalaxySize, density: GalaxyDensity,
rng: &mut ChaCha20Rng) -> u32` that returns a star count near
`size.target_stars()`, jittered by density. For Tiny + Normal, return
something in [32, 100] per FR-1.

Test: 100 random seeds against (Tiny, Normal) all return values in
[32, 100].

### Atom 2.5 — Place one star with retry budget

`fn place_one_star(rng, existing_stars, dimension, min_distance,
retry_budget) -> Result<Position, GameError>` that rejection-samples
until it finds a position at least `min_distance` from every existing
star, or returns `GalaxyGenerationFailed` after `retry_budget` attempts.

Test: place 10 stars in a small map, all pairs satisfy the distance
constraint.

### Atom 2.6 — Place all stars

`fn place_all_stars(rng, count, dimension, min_distance) ->
Result<Vec<Star>, GameError>` that calls `place_one_star` `count` times,
generating Star structs (id, name, position, empty planets vec).

Test: place 50 stars on a Tiny map, all positions within bounds, all
pairwise distances ≥ min_distance, all StarIds unique.

### Atom 2.7 — Galaxy struct

`pub struct Galaxy { pub stars: Vec<Star>, pub size: GalaxySize,
pub density: GalaxyDensity, pub seed: u64 }`. Decide with the council
whether this lives in `types.rs` or `galaxy.rs`. Default: galaxy.rs
(it's the entry point's return type, not a vocabulary type).

Test: construct a Galaxy with 32 stars, verify field round-trips.

### Atom 2.8 — Top-level `generate_galaxy`

`pub fn generate_galaxy(settings: &GameSettings) -> Result<Galaxy,
GameError>` that ties it all together:

```rust
let mut rng = seeded_rng(settings.random_seed, 0, PlayerId(0), "galaxy_generation");
let count = actual_star_count(settings.galaxy_size, settings.density, &mut rng);
let stars = place_all_stars(&mut rng, count, settings.galaxy_size.map_dimension(),
                             settings.galaxy_size.min_homeworld_distance())?;
Ok(Galaxy { stars, size: settings.galaxy_size, density: settings.density,
            seed: settings.random_seed })
```

Test: same `GameSettings.random_seed` produces an IDENTICAL Galaxy
across two calls. This is the determinism contract for galaxy
generation.

### Atom 2.9 — Extend determinism fingerprint

Add `generate_galaxy(&settings)` to `compute_determinism_fingerprint`
in `engine/tests/determinism.rs`. Re-run the test, capture the new
bytes, pin them in `EXPECTED_FINGERPRINT`. **This is the cross-target
gate for everything you just built.**

### Atom 2.10 — FR-1 acceptance test

Integration test asserting that a fixed seed against `GalaxySize::Tiny`
+ `GalaxyDensity::Normal` produces a galaxy with star count in
[32, 100], and that two consecutive calls produce identical galaxies.
Cite FR-1 in the test doc comment.

---

## Step 3 — Sync cadence

**Brainstormer + Obsidian sync after every 3 atoms** AND at any
council-flagged decision. The minimum cadence is:

- After Atom 2.3 (post-RNG, post-name, post-position — the primitives
  are in)
- After Atom 2.6 (post-placement — the hard-math atoms are in)
- After Atom 2.10 (final — the full module is shipped)

**What each sync does:**

1. Update CLAUDE.md "Last Session" block with current atom count,
   test count, and next-atom pointer.
2. Update CONTEXT.md "Now" + "Just Finished" + "Next" sections.
3. Update `.brainstormer/session.json` with progress counters.
4. Update the Obsidian vault at `c:\Dev\Anthropicer\projects\STARS-2026\`:
   - `dashboard.md` — bump the Phase 1 snapshot
   - `ledger-session.md` — add a row for the autonomous run
5. Commit the sync (separate from atom commits) with message
   `docs(brainstormer): sync after Atom 2.<N>`.

---

## Step 4 — Determinism fingerprint protocol

Every atom that adds a public arithmetic path or a new public type to
`types.rs` MUST extend `compute_determinism_fingerprint` in
`engine/tests/determinism.rs` and re-pin `EXPECTED_FINGERPRINT`.

**Procedure:**

1. Add the new path's call to `compute_determinism_fingerprint`.
2. Comment out the `assert_eq!` in `determinism_fingerprint_is_pinned`
   (or set `EXPECTED_FINGERPRINT` to an empty array).
3. Run `cargo test -p stars2026-engine --test determinism -- --nocapture`.
4. Copy the printed bytes from the failure message into
   `EXPECTED_FINGERPRINT`.
5. Restore the assertion.
6. Re-run the test — it should now pass.
7. Run the full sniff test.

**Critical:** the byte sequence MUST be identical across `cargo test`
runs on the same target. If it isn't, ambient state is leaking and
that's a bug, not a constant to update.

**Cross-target verification** (wasm vs native byte equality) is still
deferred to the wasm-bindgen-test atom (post-Atom-2). For now,
extending the fingerprint locks in same-target stability and provides
the byte sequence that will be diffed when the cross-target runner
finally lands.

---

## Step 5 — Closing Paladin + Crucible

**After Atom 2.10 ships with green sniff and CI:**

### Run Paladin (six-tier testing wall) again

Use the same approach as the prior session — load
`C:\Users\pnmcg\.claude\skills\brainstormer-quality\quality\references\rust.md`
for the exact commands. Run all applicable tiers (1, 2, 3, 5; tier 4
is N/A for engine; tier 6 is human-only).

Particular attention to:
- **Tier 1:** the wasm32 build (now part of the sniff script).
- **Tier 2:** total test count, pub-fn-vs-test-fn ratio.
- **Tier 3:** integration tests in `engine/tests/` — there should now
  be at least the determinism gate AND any galaxy-specific
  integration tests from Atom 2.10.
- **Tier 5:** release build size for both targets, no unsafe blocks
  (still forbidden), no new dependencies (still pre-authorized list).

Write the verdict to a new file
`docs/codeglass/PALADIN-VERDICT-atom-2.md` using the same format the
prior session used (copy the structure from
`docs/codeglass/ADR-0002-hardening-pass-after-crucible.md` Paladin
section).

### Run the Crucible (six adversarial agents)

Spawn the same six agents the prior session did, in parallel:
- Devil's Advocate
- Red Teamer
- Inversion Agent
- Assumption Auditor
- Bias Auditor
- First Principles

Each prompt should reference Atom 2's specific outputs:
`engine/src/rng.rs` (if created), `engine/src/galaxy.rs`, the new
determinism fingerprint, the council decisions made at Step 1.

The Crucible's job is to find the Atom 2 equivalent of the wasm32-not-
installed gap that bit the prior session. Look hard. The pattern from
ADR-0002 is "any rule that lives only in markdown is a rule that will
be violated" — apply the same lens to Atom 2's outputs.

Synthesize the Crucible findings into a **ranked action list** in a
new file `docs/codeglass/CRUCIBLE-VERDICT-atom-2.md`. Distinguish:
- **P0 — must fix before Atom 3 starts** (apply during this session)
- **P1 — fix early in Atom 3** (queue for next session)
- **P2 — worth addressing eventually** (queue for backlog)

Apply the P0 fixes as additional atoms (Atom 2.11+) BEFORE writing
the wake-up report. Each P0 fix is a sniff-tested cycle of its own.

---

## Step 6 — Wake-up report

**Last thing before Patrick's morning:**

1. Run the full sniff test one final time.
2. Confirm CI is green on the latest commit.
3. Update CLAUDE.md "Last Session" block with the final state.
4. Write a clear, friendly wake-up paragraph at the **top** of
   CONTEXT.md "Now" section. Format:

   ```
   ## Wake-up report — <date> ~07:00

   Patrick: while you slept, the agent completed Atom 2 (galaxy.rs)
   with N atoms shipped, M tests passing, sniff green, CI green on
   commit <hash>. <One-sentence summary of any council-flagged
   decisions>. <One-sentence summary of any P0 items the closing
   Crucible surfaced and how they were handled>. <Pointer to the
   Paladin and Crucible verdict files for the morning review>.
   The next session should start with: <next-step>.
   ```

5. Final commit + push of the wake-up report.

---

## Stop conditions (recap)

You stop the autonomous run, write a PAUSED note to CONTEXT.md, and
wait for Patrick if:

- Council disagrees on a load-bearing decision that contradicts a
  governance memory.
- Determinism fingerprint diverges between two same-target runs.
- The wasm32 build breaks and the cause isn't in the diff.
- More than two consecutive sniff failures on the same atom.
- A P0 hardening assumption turns out to be wrong (e.g., a clippy
  rule that should fire doesn't, or the determinism gate doesn't
  catch a divergence it should).
- You need a new dependency.
- The closing Crucible finds a P0 that requires a SPEC.md change or
  contradicts a governance decision.
- You complete the full atom sequence and the wake-up report is
  written.

**You DO NOT stop for:**

- A clippy nit you can fix in 30 seconds.
- A test that needs an extra assertion.
- A council member suggesting an ADR.
- An atom taking longer than expected.
- The determinism fingerprint needing an update (that's expected,
  follow the protocol).

---

## Final notes from the prior-session agent

The Phase 1 Crucible's most uncomfortable finding (Bias Auditor #12)
was that decisions were presented to Patrick A/B/C with the agent's
recommendation pre-baked, which collapsed the choice into a rubber
stamp. **In autonomous mode, this dynamic doesn't apply** — the agent
is decision-maker AND implementer. But the lesson still matters: when
the closing Crucible finds something that *might* warrant a
governance change, **document it as a question for Patrick on his
return, do NOT silently implement the change**. The agent's authority
during sleep is implementation, not policy.

The other thing the Crucible flagged was that "16 atoms in one
session exceeded human review bandwidth." Atom 2 is 8–10 atoms, which
is within the bandwidth limit for an autonomous session if the work
is granular and well-tested. Don't try to bundle atoms to "save time"
— the granularity is the audit trail Patrick will read in the
morning.

The single most important property of this session is that **every
commit on `main` is a green CI run**. If you have to choose between
finishing the atom sequence and leaving CI red, leave the unfinished
atoms and ensure CI is green. A red main is a worse outcome than an
incomplete Atom 2.

Build clean. Build careful. Build proud. Patrick is trusting you with
his nights — earn the second one.

---

**End of brief.** Now read CLAUDE.md, then SPEC.md FR-1 + AC-1/AC-2,
then run Step 1 — the council prompts in parallel.
