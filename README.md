# STARS 2026

> A modern reimagining of *Stars!* (1995). Faithful 1995 pixel-art soul. 2026 tech bones.

## What

STARS 2026 is a from-scratch, IP-clean remake of the legendary 1995 4X turn-based strategy game *Stars!* by Jeff Johnson and Jeff McBride. The original is 16-bit Windows abandonware that won't run on modern hardware. Existing clones are stalled or incomplete. STARS 2026 fixes that.

- **Browser-native.** Open a tab, play. No install.
- **Deterministic.** Same seed, same orders, same outcome — every time.
- **Multiplayer-ready.** 1–16 players, async with deadlines, AI takeover on no-show, vacation mode.
- **Mobile + native desktop** (v0.3 via Capacitor + Tauri).
- **Pixel art.** Faithful to 1995. Modern resolution. Aseprite-sourced.
- **DLC-friendly.** New content ships as `data/*.json` + sprite packs. Engine never forks.

## Why

The *Stars!* community has been waiting 30 years for this. We have the modern stack to do it right.

## Stack

| Layer | Choice |
|---|---|
| Engine | Rust → WebAssembly + native (one source, two targets) |
| Frontend | SvelteKit + TypeScript + Tailwind + Canvas |
| Multiplayer (v0.2) | Axum + PostgreSQL + Redis + WebSocket |
| Mobile (v0.3) | Capacitor |
| Native desktop (v0.3) | Tauri |
| Build | Cargo + Vite |
| Test | cargo test + Vitest + Playwright |

See [SPEC.md](SPEC.md) for the full spec, [ARCHITECTURE.md](ARCHITECTURE.md) for the design.

## Status

🌿 **Phase 0b — Project Skeleton**

- ✅ Governance docs (SPEC, SCENARIOS, ARCHITECTURE, AGENTS, CODEGUIDE, ART, SNIFFTEST, etc.)
- ⏳ Cargo workspace + SvelteKit scaffold
- ⏳ First gameplay code (Phase 1)

## Roadmap

| Version | Scope |
|---|---|
| **v0.1** | Single-player + hotseat, core loop, deterministic engine proven |
| **v0.2** | Networked multiplayer 1–16 players, speed modes, AI takeover, push |
| **v0.3** | Capacitor mobile + Tauri native desktop |
| **v1.0** | Full 10 PRTs + LRTs + minefields + packets + final art |
| **DLC** | New PRTs, hulls, scenarios via data/*.json + sprite packs |

## Workflow

This project follows the **sniff-test protocol** (see [SNIFFTEST.md](SNIFFTEST.md)). Every function gets a unit test, module test, AC verification, regression check, and manual spot-check before the next function begins. No shortcuts.

## Reference Material

`reference/legacy-desktop-scaffold/` contains earlier brainstorm notes. **Study only.** No code in this repo is inherited from there.

External references (study, never copy):
- [craig-stars](https://github.com/sirgwain/craig-stars) — MIT-licensed Go+SvelteKit clone, module-boundary reference
- [starsfaq.com](http://www.starsfaq.com) — canonical formula source
- [wiki.starsautohost.org](https://wiki.starsautohost.org) — tech tables, hull/component specs

## License

TBD. The engine and frontend will be open source. License chosen before v0.1 release.

## Not Affiliated

STARS 2026 is not affiliated with, endorsed by, or derived from the source code of the original *Stars!*. It is a clean-room implementation based on publicly documented gameplay mechanics.
