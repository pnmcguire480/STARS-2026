# CODEGUIDE.md — STARS 2026

## Rust (engine, server)

### Style
- `rustfmt` on save. Default config + `max_width = 100`.
- `clippy::pedantic` enabled. Allow per-call only with justification comment.
- Module per file. No inline `mod foo { ... }` blocks except for tests.

### Naming
- Types: `PascalCase`
- Functions, variables: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- IDs: typed wrappers (`PlayerId(u32)`, `PlanetId(u32)`, etc.) — never raw integers in domain code.

### Errors
- All fallible functions return `Result<T, GameError>`.
- `GameError` is a single enum in `types.rs`. New variants added as needed.
- **No `unwrap()`, no `expect()`, no `panic!()` in library code.** Test code may.
- **No `todo!()`, no `unimplemented!()`** in any committed code.

### Determinism
- No `std::time` in `engine/`.
- No `HashMap` iteration in turn-critical paths. Use `BTreeMap` or sorted `Vec`.
- No `thread_rng()`. Ever. RNG comes from `engine::rng::seeded_for(...)`.
- No floating point in turn math. Fixed-point or integer.

### Tests
- Unit tests in the same file under `#[cfg(test)] mod tests`.
- Integration tests in `engine/tests/` directory.
- Every public function has at least one test.
- Every formula function has a test that cites the source URL or `docs/FORMULAS.md` line.

### Comments
- Doc comments (`///`) on every public item.
- Inline comments only when the *why* isn't obvious. The *what* is in the code.
- Cite formula sources: `// Formula: starsfaq.com/economy.htm § Production`

## TypeScript (frontend)

### Style
- `prettier` + `eslint` with `@typescript-eslint/strict`.
- 2-space indent, single quotes, semicolons.
- `noUncheckedIndexedAccess: true` in tsconfig.

### Naming
- Components: `PascalCase.svelte`
- Stores: `camelCase.ts`
- Types: `PascalCase`

### State
- Game state lives in the WASM engine. **Never** duplicate it in Svelte stores.
- Svelte stores hold UI-only state (selected planet, modal open, hover target, etc.).
- All game mutations go through the engine. Frontend never mutates game state directly.

### Async
- `async/await` only. No `.then()` chains.
- Errors: `try/catch` with explicit user-facing error messages.

### Tests
- `Vitest` for unit + component tests.
- `Playwright` for E2E.
- Every store has a test. Every component with logic has a test.

## Git

### Commits
- Conventional commits: `feat:`, `fix:`, `refactor:`, `test:`, `docs:`, `chore:`.
- One logical change per commit. The sniff-test workflow naturally produces small commits.
- Body explains *why*, not *what*.

### Branches
- `main` is always green.
- Feature work on `feat/<short-name>` branches.
- PRs require at least one Tier 5 review for critical modules.

### CI Gates (must pass before merge)
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test --workspace`
- Cross-target determinism test (S-21)
- `npm run lint`
- `npm run check` (svelte-check)
- `npm run test` (Vitest)
- `npm run test:e2e` (Playwright, on PRs touching frontend)

## File Layout Conventions

- One responsibility per file.
- File name matches the primary type it exports.
- Tests live next to code, not in a parallel tree.
- Generated files (wasm-bindgen output) live in `pkg/` and are gitignored except for `.gitkeep`.
