# Contributing to STARS 2026

First — thank you. If you're here, you probably loved *Stars!* too.

## Ground rules

1. **Clean room.** No code, assets, or formulas from decompiled binaries. Public FAQ / wiki / forum knowledge only.
2. **Determinism is law.** Anything that breaks "same seed + same orders → same outcome" gets reverted.
3. **Sniff test after every function.** See [SNIFFTEST.md](SNIFFTEST.md). No batching, ever.
4. **Faithful before fancy.** We change combat math zero times. We change UX a thousand times.
5. **Be kind.** This is a love letter to a 30-year-old game. Act like it.

## What we need most

### 🧠 Veteran knowledge (no code required)
If you played the original and remember a formula, a quirk, or an edge case — **open an issue with the "Veteran Knowledge" template**. You don't need to write code. Just tell us what you remember and link the source. This is the highest-leverage contribution there is.

### 🔬 Formula sourcing
Every game formula in `engine/src/` must cite a source (starsfaq.com, wiki.starsautohost.org, or a forum thread). If you find one missing or wrong, file an issue.

### 🦀 Rust engine
- Read [SPEC.md](SPEC.md), [ARCHITECTURE.md](ARCHITECTURE.md), [CODEGUIDE.md](CODEGUIDE.md), [SNIFFTEST.md](SNIFFTEST.md) **before writing code**.
- Module ownership: one responsibility per file. No god-modules.
- All RNG is seeded via `engine::rng`. No `thread_rng()`. Ever.
- All errors return `Result<T, GameError>`. No `unwrap()`/`panic!()` in lib code.
- No floating point in turn-critical math.

### ⚡ SvelteKit frontend
- Game state lives in the WASM engine, **never** duplicated in Svelte stores.
- Stores hold UI-only state (selected planet, modal open, etc.).
- Components have tests if they have logic.

### 🎨 Pixel art
- Read [ART.md](ART.md) first.
- Aseprite source files only, palette-locked.
- No AI-generated art. Anywhere. Ever.

## Workflow

1. **Open an issue first** for anything bigger than a typo. Discuss before you build.
2. Fork & branch: `feat/<short-name>` or `fix/<short-name>`.
3. **Write the test with the function.** Not after.
4. **Run the sniff test** before you push (`cargo test --workspace && cd frontend && npm run check && npm run test`).
5. Open a PR with:
   - What changed and why
   - Which AC(s) from SPEC.md it advances
   - Test output pasted in the PR description
6. Critical-module PRs (`engine/src/turn.rs`, `combat.rs`, `rng.rs`, `race.rs`, `tech.rs`) require Tier 5 review before merge.

## What we **won't** accept

- ❌ AI-generated art or code without disclosure and a damn good reason
- ❌ Changes to combat math that aren't sourced to FAQ/wiki
- ❌ "Modernization" of game mechanics (we changed UX, not gameplay)
- ❌ Real-time mode bolt-ons
- ❌ Microtransactions, gacha, energy systems, daily login bonuses
- ❌ Reverse-engineered or decompiled material from the original binary
- ❌ PRs without tests
- ❌ PRs that batch multiple unrelated changes

## Code style

- Rust: `rustfmt` + `clippy::pedantic`
- TypeScript: `prettier` + `eslint --strict`
- See [CODEGUIDE.md](CODEGUIDE.md) for details

## Commits

Conventional commits: `feat:`, `fix:`, `refactor:`, `test:`, `docs:`, `chore:`. One logical change per commit.

## Code of Conduct

Be kind. Be patient. Assume good faith. If you wouldn't say it in a *Stars!* PBEM email circa 2002, don't say it here.

## Questions?

Open a [Discussion](https://github.com/pnmcguire480/STARS-2026/discussions). For sensitive issues, see SECURITY.md (coming soon).

---

*The masses are summoned. The oracles stand ready. Let's build a flawless game.*
