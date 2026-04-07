# Stars 2026

A modern ground-up reimplementation of **Stars!** (1995), the legendary 4X turn-based space strategy game.

**Status:** Phase 1 — Foundation (types & constants complete, galaxy generation next)

## What Is This?

Stars 2026 is a clean-room rebuild of the classic Stars! game using modern technology:
- **Game engine:** Rust compiled to WebAssembly — fast, deterministic, runs in any browser
- **Frontend:** SvelteKit + TypeScript + pixel art aesthetic
- **Mobile:** Capacitor wrapper for Android/iOS (planned)

The original Stars! (1995) is functionally abandonware. This project reimplements its mechanics from publicly documented formulas and community knowledge — no original code or assets.

## Building

### Engine (Rust)
```bash
cd engine
cargo build
cargo test
```

### Frontend (SvelteKit) — coming in Phase 6
```bash
cd frontend
npm install
npm run dev
```

## Project Structure

```
engine/src/types.rs   — Core types, enums, constants
engine/src/lib.rs     — Crate root
data/                 — Game data tables (JSON)
Spec.md               — Master specification
CLAUDE.md             — Claude Code session instructions
CONTEXT.md            — Rolling development state
BabyTierOS.md         — AI-assisted development workflow
LLM-Protocol.md       — Prompt patterns and operating manual
```

## License

MIT

## Credits

Original Stars! by Jeff Johnson and Jeff McBride (Star Crossed Software, 1995).
Stars 2026 is an independent reimplementation with all-original code and assets.
