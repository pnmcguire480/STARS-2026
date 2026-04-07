# Baby-Tier AI Coding Operating System — Stars 2026 Edition

**Goal:** Build a full 4X space strategy game using AI-assisted development. You (Patrick) are the orchestrator, quality gatekeeper, and domain expert. LLMs are implementers and checkers. Every function gets a sniff test. No shortcuts. No premature completion.

---

## Core Principles

1. **Spec is the source of truth.** If it isn't in Spec.md, it isn't real. Stars! has 30 years of community documentation — use it to validate, but the Spec decides what we build.
2. **Scenarios are external holdouts.** The builder model must not see Scenario.md. Ever.
3. **Small diffs. Short cycles. Fast failure.** One game system per task. One function at a time. Test it. Move on.
4. **You do not review code. You review behavior.** Run the game. Click the buttons. Watch the turn generate. Does it do what the Spec says? That's the review.
5. **Sniff test every function.** No function ships without a verification pass against the Spec. This is non-negotiable. Claude does not get to chain tasks without Patrick confirming the sniff test passed.

---

## Model Roles (Stars 2026 Specific)

| Role | Model | Use For |
|------|-------|---------|
| **Tier 5 — Architect** | Claude Chat (Opus) | Architecture decisions, spec refinement, scenario generation, debugging complex game logic (combat engine, turn order), cross-system integration planning |
| **Tier 4 — Builder** | Claude Code (Sonnet) | Multi-file implementation, Rust game engine code, SvelteKit UI components, test writing, refactoring. The workhorse. |
| **Tier 3 — Specialist** | DeepSeek | Alternative implementation when Claude gets stuck, math-heavy formula verification, second opinions on combat/economy formulas |
| **Tier 1–2 — Local** | Ollama (Mistral/Qwen) | Linting, doc edits, boilerplate generation, commit message writing, quick lookup of Stars! formulas, summarizing diffs |

**Rules:**
- Never waste Tier 5 on boilerplate. Never waste tokens pasting whole files.
- When Claude Code stops short or takes shortcuts: STOP. Restate the task with explicit constraints. Reference the Sniff Test Protocol.
- Rotate models when one is stuck after 3 attempts.

---

## Repo Layout

```
stars2026/
├── CLAUDE.md                    # Claude Code instructions (project-level)
├── Spec.md                      # Master specification
├── Scenario.md                  # Holdout scenarios (⚠️ NEVER show to builder)
├── LLM-Protocol.md              # Operating manual for AI-assisted dev
├── BabyTierOS.md                # This file — workflow system
├── CONTEXT.md                   # Rolling state doc (updated every session)
├── ARCHITECTURE.md              # System architecture + dependency map
├── CHANGELOG.md                 # What shipped, when
├── README.md                    # Project overview + build instructions
│
├── engine/                      # Rust — core game logic (compiles to WASM)
│   ├── src/
│   │   ├── lib.rs               # WASM entry points
│   │   ├── galaxy.rs            # Galaxy generation
│   │   ├── race.rs              # Race design + point calculator
│   │   ├── planet.rs            # Planet mechanics, production, growth
│   │   ├── tech.rs              # Research + tech tree
│   │   ├── ship.rs              # Ship design, hull/component system
│   │   ├── fleet.rs             # Fleet movement, waypoints, fuel
│   │   ├── combat.rs            # Battle engine
│   │   ├── turn.rs              # Turn generation (order of events)
│   │   ├── scanner.rs           # Scanning, cloaking, fog of war
│   │   └── types.rs             # Shared types, constants, game data tables
│   ├── tests/                   # Rust unit + integration tests
│   ├── Cargo.toml
│   └── README.md
│
├── frontend/                    # SvelteKit — UI layer
│   ├── src/
│   │   ├── routes/              # SvelteKit pages
│   │   ├── lib/
│   │   │   ├── components/      # UI components (galaxy map, panels, designer)
│   │   │   ├── stores/          # Svelte stores (game state, UI state)
│   │   │   ├── wasm/            # WASM bindings + bridge layer
│   │   │   └── assets/          # Pixel art sprites, fonts, audio
│   │   └── app.html
│   ├── static/
│   ├── tests/                   # Vitest + Playwright
│   ├── svelte.config.js
│   ├── vite.config.ts
│   └── package.json
│
├── data/                        # Game data tables (JSON)
│   ├── hulls.json               # Hull definitions
│   ├── components.json          # Component stats (weapons, engines, etc.)
│   ├── tech_costs.json          # Research cost per level
│   └── presets/                 # Preset race definitions
│
├── scenarios/                   # ⚠️ Add to .gitignore for builder context
│   └── (scenario test runners)
│
└── .github/
    └── workflows/
        └── ci.yml               # GitHub Actions: build, test, lint
```

---

## The Build Loop (Repeat Until Shipped)

### Step 1: Spec a Game System — Patrick + Tier 5

Pick the next game system to implement. Use the Spec.md section as the source of truth.

**Build order (Chunk 1 — dependency-sorted):**

```
Phase 1: Foundation
  1. Types & constants (types.rs, game data tables)
  2. Galaxy generation (galaxy.rs)
  3. Race design & point calculator (race.rs)

Phase 2: Economy
  4. Planet mechanics — hab calculation, population growth (planet.rs)
  5. Mineral extraction + resource generation (planet.rs)
  6. Production queue (planet.rs)
  7. Tech research system (tech.rs)

Phase 3: Ships & Movement
  8. Ship designer — hulls, components, stat calculation (ship.rs)
  9. Fleet management — creation, merging, splitting (fleet.rs)
  10. Fleet movement — waypoints, fuel, distance (fleet.rs)

Phase 4: Combat
  11. Battle engine — grid, phases, damage, retreat (combat.rs)
  12. Battle plans (combat.rs)
  13. Battle reports (combat.rs)

Phase 5: Turn Engine
  14. Turn generation — order of events orchestrator (turn.rs)
  15. Scanner/fog of war (scanner.rs)
  16. Message system

Phase 6: UI
  17. Galaxy map (canvas rendering, pan/zoom)
  18. Planet detail panel
  19. Ship designer UI
  20. Fleet orders panel
  21. Research panel
  22. Race designer UI
  23. Turn summary / messages
  24. Main menu + new game flow

Phase 7: Polish & Ship
  25. Save/load system
  26. AI opponents (basic)
  27. Mobile responsive pass
  28. Performance optimization
  29. Pixel art asset integration
  30. Beta release
```

---

### Step 2: Create Task Batch — Patrick + Tier 5

For each game system, break it into 30–90 minute tasks.

**Example for "Galaxy Generation":**

```
Task 1: Define Star and Planet types in types.rs (30 min)
  Done-check: Types compile, unit test creates a Star and Planet instance

Task 2: Implement galaxy_generate() — creates N stars with random positions (60 min)
  Done-check: galaxy_generate(seed, size=Medium) returns 120–180 stars, no duplicates

Task 3: Implement planet_generate() — each star gets 1–5 planets (60 min)
  Done-check: Every star has at least 1 planet, planet env values in valid ranges

Task 4: Implement homeworld_assign() — place homeworlds for N players (60 min)
  Done-check: Each player gets 1 homeworld, 100% hab, minimum distance apart

Task 5: WASM bridge — expose galaxy_generate to JS (30 min)
  Done-check: frontend can call generate and receive galaxy JSON
```

---

### Step 3: Implement One Task — Tier 4 (Claude Code)

**Prompt template for each task:**

```
You are implementing Task <N> for the Stars 2026 game engine.

Here is the relevant section of the spec:
<paste ONLY the relevant FR items and data model section>

Here are the existing types/interfaces this must work with:
<paste types.rs or relevant type definitions>

Constraints:
- Implement ONLY this task. Do not touch unrelated files.
- Write the function AND its unit test in the same pass.
- Keep the diff as small as possible.
- Use the seeded RNG (rand::SeedableRng) for any randomness.
- Follow Rust conventions: Result types for errors, no unwrap() in library code.
- Provide: (a) files changed, (b) the code, (c) how to run the test.

After writing the code, STOP. Do not proceed to the next task.
I will run the sniff test before you continue.
```

---

### Step 4: Sniff Test Protocol — Patrick (MANDATORY)

**After EVERY function or feature implementation:**

```
SNIFF TEST CHECKLIST:
□ 1. Does the function's unit test pass? (cargo test <test_name>)
□ 2. Does the module's full test suite pass? (cargo test <module>)
□ 3. Does the behavior match the relevant Spec.md acceptance criteria?
     → Which AC does this touch? Write the AC number.
□ 4. Regression check: Do ALL existing tests still pass? (cargo test)
□ 5. If this touches turn generation: run the turn-gen smoke test.
□ 6. Manual spot-check: Does the output look right? (inspect logs/output)
□ 7. Log result in CONTEXT.md under "Sniff Test Log"
```

**If any check fails:** STOP. Do not proceed. Fix the failure before moving on. Use the Debug Protocol.

**If Claude tries to skip the sniff test or move to the next task:** Remind it: "Stop. Sniff test first. Show me the test results."

---

### Step 5: Outcome Verification — Patrick + Tier 5

When a sniff test fails, do NOT paste the whole repo. Paste only:
- The failing test output (exact error message)
- The relevant function code (smallest excerpt)
- The Spec section it should match

**Prompt:**
```
This function was supposed to: <paste AC or FR>
Here's the test failure: <paste error>
Here's the function: <paste code>

What's most likely wrong? Give me the smallest fix.
Do NOT rewrite the whole function.
```

---

### Step 6: Scenario Evaluation — Patrick (Periodic)

After completing each Phase (not each task), run the relevant scenario subset.

**Phase 1 complete?** Run: SC-galaxy-001, SC-galaxy-002, SC-race-001, SC-race-002
**Phase 2 complete?** Run: SC-planet-001 through SC-planet-003, SC-tech-001
**Phase 3 complete?** Run: SC-ship-001, SC-ship-002, SC-fleet-001, SC-fleet-002
**Phase 4 complete?** Run: SC-combat-001, SC-combat-002
**Phase 5 complete?** Run: SC-turn-001 (the big one — full order of events)

Record pass/fail in CONTEXT.md. If scenarios fail, update the task batch to fix failures before adding new features.

---

### Step 7: Pre-Ship Gate

- [ ] All `must_pass` scenarios pass
- [ ] All unit and integration tests green
- [ ] Sniff test log shows every function was verified
- [ ] Turn generation < 2s on reference hardware
- [ ] Game is playable end-to-end (new game → play turns → reach victory)
- [ ] Mobile viewport is usable
- [ ] Save/load works without data loss
- [ ] README has current build instructions
- [ ] CHANGELOG is updated
- [ ] Version tagged

---

## Anti-Shortcut Tactics (Claude-Specific)

1. **Claude loves to chain tasks.** After every code generation, say: "Stop. Show me the test results before proceeding."
2. **Claude loves to generate placeholder implementations.** If you see `todo!()`, `unimplemented!()`, or `// TODO` in shipped code, reject it immediately.
3. **Claude loves to skip edge cases.** After every function, ask: "What inputs would break this function?" Then write those tests.
4. **Claude loves to rewrite when asked to fix.** Always say: "Show me ONLY the lines that change. Do NOT rewrite the function."
5. **Keep Scenario.md out of builder context.** Never paste scenarios into Claude Code sessions.
6. **Spot-check math.** For combat, economy, and research formulas: manually calculate 2–3 examples and verify Claude's implementation matches. Use the Stars! wiki formula documentation.
7. **Rotate models when stuck.** If Claude Code fails 3 times on the same task, switch to DeepSeek or take the problem to Claude Chat for architectural diagnosis.

---

## Debug Protocol (When Stuck)

1. **Reproduce** with one command: `cargo test <failing_test>`
2. **Minimize:** What's the smallest input that triggers the bug?
3. **Instrument:** Add `println!` or `dbg!()` in the failing path
4. **Hypothesize:** List 3 causes, test in order of likelihood
5. **Fix** the smallest thing that makes the failing case pass
6. **Add a test** so this failure never returns silently
7. **Re-run full suite** to verify no regression

---

## CONTEXT.md Template

```markdown
# CONTEXT.md — Stars 2026

## Current goal
<one sentence: what game system we're building right now>

## Current phase
Phase <N>: <name> — Task <N> of <M>

## Last 3 decisions
1. <decision + rationale>
2. <decision + rationale>
3. <decision + rationale>

## Current failure / blocker
<description + reproduction command>

## Sniff test log (last 5)
| Function | Test | AC | Pass/Fail | Date |
|----------|------|----|-----------|------|
| galaxy_generate() | test_medium_galaxy_star_count | AC-1 | ✅ | 2026-03-15 |

## Key file paths
- engine/src/galaxy.rs: Galaxy generation
- engine/src/types.rs: Shared types and game constants
- data/hulls.json: Hull definitions

## Commands to reproduce
- Build engine: `cd engine && cargo build --target wasm32-unknown-unknown`
- Test engine: `cd engine && cargo test`
- Build frontend: `cd frontend && npm run build`
- Dev server: `cd frontend && npm run dev`
- Full build: `npm run build:all`
```

---

## Token-Minimizing Habits

- Never paste whole files. Paste the function + its test + the relevant Spec section.
- Always ask for a plan before asking for code.
- Prefer "diff-only" responses from the builder model.
- Use Ollama (Tier 1–2) for doc edits, commit messages, and formula lookups.
- Update CONTEXT.md after every session so you don't re-explain state.
- The Stars! wiki has formulas for everything. Paste the formula into the prompt instead of asking the model to guess.

---

## Stars! Reference Sources (Bookmark These)

- **Stars! FAQ:** starsfaq.com — game mechanics documentation
- **Stars! Wiki:** wiki.starsautohost.org — comprehensive reference
- **Order of Events:** starsfaq.com/order_events.htm — canonical turn sequence
- **Battle Engine:** starsfaq.com/battleengine.htm — combat formulas
- **Tech Trading:** starsfaq.com/tech_trade.htm — research mechanics
- **Minefields:** starsfaq.com/minefield.htm — minefield formulas
- **Bugs & Features:** starsfaq.com/bugs.htm — known issues (we fix these)
- **Craig-Stars (reference only):** github.com/sirgwain/craig-stars — MIT-licensed Go/Svelte clone (read for formula reference, do NOT copy code)
- **Stars! Discord:** discord.gg/6eEDvgVEWK — active community for playtesting
