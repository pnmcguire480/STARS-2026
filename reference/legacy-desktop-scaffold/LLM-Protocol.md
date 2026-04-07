# LLM-PROTOCOL.md — Stars 2026 AI Coding Operating Manual

> **Who this is for:** Patrick McGuire — solo developer building a full 4X strategy game using AI-assisted workflows. Primary setup: Windows PC (i7-10700F, 48GB RAM, GTX 1660 Super) running VS Code with Claude Code. Mobile fallback: phone + Bluetooth keyboard via GitHub Codespaces. Every instruction is explicit. Nothing is assumed.

---

## The Loop: Plan → Spec → Generate → Sniff Test → Fix → Re-verify → Ship

```
┌─────────┐
│  PLAN   │ ← Which game system am I building? (1 sentence)
└────┬────┘
     ▼
┌─────────┐
│  SPEC   │ ← What does Spec.md say about this system?
└────┬────┘
     ▼
┌─────────┐
│ GENERATE│ ← Prompt Claude Code to write the function + its test
└────┬────┘
     ▼
┌───────────┐
│ SNIFF TEST│ ← Run the test. Check against AC. Check regression.
└────┬──────┘
     ▼
  Pass? ──Yes──► LOG in CONTEXT.md ──► Next function
     │
     No
     ▼
┌──────────┐
│ DIAGNOSE │ ← Read the error. Paste ONLY the error + function.
└────┬─────┘
     ▼
┌─────────┐
│  PATCH  │ ← Ask Claude to fix the SPECIFIC error (diff only)
└────┬────┘
     ▼
┌───────────┐
│ RE-VERIFY │ ← Sniff test again
└────┬──────┘
     ▼
  Pass? ──Yes──► LOG ──► Next function
     │
     No (3rd attempt)
     ▼
┌──────────────┐
│ ESCALATE     │ ← Switch model (DeepSeek or Tier 5 Claude Chat)
└──────┬───────┘   OR rewrite the spec section. The problem is
       │           the spec or the approach, not more prompting.
       ▼
  Still stuck after model switch?
       ▼
┌──────────────┐
│ REWRITE SPEC │ ← Simplify the feature. Break it smaller.
└──────────────┘
```

**CRITICAL RULES:**
1. If Claude Code fails 3 times on the same function, STOP PROMPTING. Switch models or rewrite the spec.
2. Never skip the sniff test. Not once. Not even for "simple" functions.
3. Never let Claude Code chain multiple functions without a checkpoint between each one.

---

## Stars 2026 Prompt Patterns

### Pattern 1: Planning a Game System

```
I'm implementing the [SYSTEM NAME] system for Stars 2026.

Here is the relevant section from Spec.md:
[paste FR items and data model for this system]

Here are the existing types it needs to work with:
[paste types.rs relevant sections]

What functions need to be written and in what order?
For each function, state:
- Function signature
- What it does (1 sentence)
- What it depends on
- How to test it

Do NOT write any code yet. Just the plan.
```

### Pattern 2: Implementing a Single Function

```
Implement this function for the Stars 2026 game engine:

Function: <name>(<params>) -> <return type>
Purpose: <1 sentence from plan>
Spec requirement: <paste the specific FR>

Here are the types it uses:
[paste relevant type definitions from types.rs]

Here are the game data constants it needs:
[paste from hulls.json / components.json / tech_costs.json if relevant]

Rules:
- Write ONLY this function and its unit test
- Use Result<T, GameError> for error handling (no unwrap/panic)
- Use seeded RNG for any randomness (accept seed as parameter)
- Follow the formula from the Stars! wiki: [paste formula if applicable]
- Do NOT write any other functions
- Do NOT modify any existing functions
- Show me the function, the test, and how to run the test

After writing, STOP. I will run the sniff test.
```

### Pattern 3: Implementing a UI Component

```
Build a SvelteKit component for Stars 2026:

Component: <ComponentName>.svelte
Purpose: <what it displays/does>
Spec requirement: <paste FR>

Here are the WASM bindings it will call:
[paste the JS-side WASM interface for this data]

Here is the game state type it receives:
[paste the TypeScript type/interface]

Rules:
- Use Tailwind CSS for styling (utility classes only)
- Component must work at 375px viewport width (mobile)
- Touch targets minimum 44px
- Pixel art assets referenced as sprites from the asset sheet
- No external dependencies beyond what's in package.json
- Write the component AND a basic Vitest/component test
- Do NOT modify other components
```

### Pattern 4: Sniff Test Verification Prompt

```
I just ran the sniff test for [function name].

Here are the results:
- Unit test: [PASS/FAIL + output]
- Module tests: [PASS/FAIL + output]
- Regression (full suite): [PASS/FAIL + output]
- Manual check: [what I observed]

The relevant acceptance criteria is:
AC-<N>: [paste AC]

Does this implementation satisfy the AC?
If not, what specifically needs to change?
Show ONLY the lines that need to change. Do not rewrite the function.
```

### Pattern 5: Formula Verification

```
Verify this implementation matches the Stars! game formula.

The documented formula (from Stars! wiki) is:
[paste the formula]

My implementation:
[paste the function]

Test case:
- Input: [specific values]
- Expected output (calculated by hand): [value]
- Actual output: [value]

Does the implementation match the formula?
If not, where does it diverge?
```

### Pattern 6: Debugging a Game Logic Bug

```
I have a bug in the Stars 2026 [SYSTEM] system.

Expected behavior (from Spec.md):
[paste the relevant FR or AC]

What actually happens:
[describe the wrong behavior]

Test output:
[paste failing test output]

Relevant function:
[paste the function — ONLY this function]

What is causing the wrong behavior and what is the minimal fix?
Do NOT rewrite the function. Show only what changes.
```

### Pattern 7: Connecting Rust Engine to SvelteKit UI

```
I need to bridge the [SYSTEM] from the Rust WASM engine to the SvelteKit frontend.

Rust function signature (in lib.rs):
[paste the #[wasm_bindgen] function]

TypeScript type the frontend expects:
[paste the TS interface]

Write:
1. The wasm-bindgen export in lib.rs (if not already done)
2. The TypeScript wrapper in frontend/src/lib/wasm/[system].ts
3. The Svelte store that holds this data in frontend/src/lib/stores/[system].ts

Keep it minimal. No extra features.
```

---

## Anti-Hallucination Rules (Game-Specific)

| Rule | What It Means | How to Enforce |
|------|---------------|----------------|
| **Do Not Invent Game Mechanics** | Claude must not add mechanics not in the Spec. Stars! has documented rules — use them. | Always paste the Spec FR and/or Stars! wiki formula into the prompt |
| **Do Not Guess Formulas** | Combat damage, population growth, mineral extraction — all have specific formulas | Paste the formula. Make Claude implement it exactly. Verify with hand-calculated examples. |
| **Do Not Invent Components** | Hull types, weapon stats, engine fuel tables are defined in data JSON files | Always paste the relevant data table into the prompt |
| **Ask-for-Plan-First** | Before writing any system, ask Claude for a function-by-function plan | Use Pattern 1 before Pattern 2. Always. |
| **No Implied Context** | Claude Code does not remember previous sessions | Paste CONTEXT.md at the start of every session |
| **Verify Every Import** | After code generation, verify all imports exist | `grep -rn "use crate::" engine/src/` or `grep -rn "import" frontend/src/` |
| **Chunk Long Files** | If a Rust module exceeds 300 lines, only paste the relevant function + struct definitions | Prevents context overflow and formula confusion |

### When to Restart with a Fresh Approach

Stop retrying and escalate when:
- [ ] 3 failed attempts on the same function
- [ ] Claude keeps adding game mechanics not in the Spec
- [ ] Claude invents formulas instead of using the ones you provided
- [ ] Generated code is 3x longer than expected
- [ ] Each response contradicts the previous one
- [ ] You've been on the same function for over 45 minutes

**Restart process:** Close the session. Open a new one. Paste CONTEXT.md. Paste the specific function plan. Start clean.

---

## Development Environment Commands

### Engine (Rust/WASM)

```bash
# Navigate to engine
cd engine

# Build for native testing
cargo build

# Run all tests
cargo test

# Run a specific test
cargo test test_medium_galaxy_star_count

# Run tests for one module
cargo test galaxy::

# Build for WASM
wasm-pack build --target web --out-dir ../frontend/src/lib/wasm/pkg

# Check for issues without building
cargo check

# Lint
cargo clippy -- -D warnings
```

### Frontend (SvelteKit)

```bash
# Navigate to frontend
cd frontend

# Install dependencies
npm install

# Dev server (hot reload)
npm run dev

# Run unit tests
npm run test:unit

# Run E2E tests (Playwright)
npm run test:e2e

# Build for production
npm run build

# Lint
npm run lint
```

### Full Project

```bash
# Build everything (from project root)
npm run build:all    # builds engine WASM + frontend

# Test everything
npm run test:all     # cargo test + npm test

# Quick smoke test
npm run smoke        # generates a game, runs 5 turns, verifies no crash
```

---

## Session Discipline

| Habit | Why | Stars 2026 Specific |
|-------|-----|---------------------|
| **One game system per session** | Don't mix combat code with UI work | Systems are isolated in separate .rs files for a reason |
| **Commit after every passing sniff test** | Instant rollback point | `git commit -m "✅ galaxy_generate passes AC-1"` |
| **Sessions < 60 minutes** | Context degrades, especially with game math | Fresh session = paste CONTEXT.md + pick up next task |
| **Start each session with CONTEXT.md** | Claude Code has no memory | First message: "Here's the project state and what we're building" |
| **End each session with CONTEXT.md update** | Tomorrow-you needs to know where today-you stopped | Update: current goal, last decisions, sniff test log |
| **Checkpoint before risky changes** | Combat engine changes can break everything | `git commit -m "checkpoint: before battle engine refactor"` |
| **Test game math by hand** | Formulas are the soul of this game | Keep a scratchpad with hand-calculated test cases |

---

## Quality Bar

### When a Function Is Done (Stop Conditions)

- [ ] Unit test passes
- [ ] Module test suite passes
- [ ] Full test suite passes (no regression)
- [ ] Behavior matches the relevant AC from Spec.md
- [ ] If it's a formula: hand-calculated verification matches
- [ ] Sniff test logged in CONTEXT.md
- [ ] Code has no `todo!()`, `unimplemented!()`, or placeholder logic

### When to Escalate

| Situation | Action |
|-----------|--------|
| Formula doesn't match Stars! wiki after 3 attempts | Paste the wiki formula into Tier 5 Claude Chat for analysis |
| Turn generation order is wrong | Compare against starsfaq.com/order_events.htm step by step |
| Combat results don't match expected | Use craig-stars (MIT, Go) as a reference — read their combat code, don't copy it |
| UI component doesn't work on mobile | Test in Chrome DevTools mobile emulator before debugging |
| Build fails after WASM update | Check wasm-pack version, verify types match between Rust and TS |
| Performance is too slow | Profile with `cargo bench` (engine) or Chrome DevTools Performance tab (UI) |

### The Cardinal Rule

> **Never ship a function you haven't sniff-tested.**
>
> You don't need to write the code. You DO need to verify it works, understand what it does, and confirm it matches the Spec. If Claude generated a formula you can't verify, hand-calculate a test case. If the result doesn't match, the code is wrong. Your job is judgment, verification, and quality. Claude's job is implementation.

---

## Quick Reference Card

```
THE LOOP:
Plan → Spec → Generate → Sniff Test → Fix → Re-verify → Ship

RULES:
- 3 fails = switch models or rewrite the spec
- Sniff test EVERY function (no exceptions)
- Never let Claude chain tasks without a checkpoint
- Paste CONTEXT.md at the start of every session
- Hand-verify all game formulas
- Commit after every passing sniff test

PROMPTS:
- "What functions need to be written?" (Plan first)
- "Write ONLY this function and its test" (One at a time)
- "Show ONLY the lines that change" (Diffs, not rewrites)
- "STOP. I will run the sniff test." (Enforce checkpoints)
- "Here is the formula from the wiki" (Anti-hallucination)

COMMANDS:
cargo test                    # Test engine
cargo test <test_name>        # Test one function
npm run test:unit             # Test frontend
npm run build:all             # Build everything
git add -A && git commit -m "msg"  # Save
git revert HEAD               # Undo last commit
```
