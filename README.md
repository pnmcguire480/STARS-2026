<div align="center">

# 🌌 STARS 2026

### The legendary 1995 4X strategy game *Stars!* — reborn for the modern web.

**Browser-native · Deterministic · Multiplayer-ready · Mobile-bound**

[![Status](https://img.shields.io/badge/status-Phase%200%20%7C%20Skeleton-blue)](https://github.com/pnmcguire480/STARS-2026)
[![Stack](https://img.shields.io/badge/engine-Rust%20%2B%20WASM-orange)](https://www.rust-lang.org)
[![Frontend](https://img.shields.io/badge/frontend-SvelteKit-ff3e00)](https://kit.svelte.dev)
[![License](https://img.shields.io/badge/license-TBD-lightgrey)](#license)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](#contributing)

*A clean-room remake of the classic space 4X. No emulators. No Windows 3.1. No DOSBox tricks. Just open a tab and conquer the galaxy.*

</div>

---

## 🚀 What is STARS 2026?

**STARS 2026** is an open-source, IP-clean, modern reimagining of ***Stars!*** — the cult-classic 4X turn-based strategy game by Jeff Johnson and Jeff McBride (Empire Interactive, 1995). Considered by many veterans to be **the deepest 4X ever made**, the original has been trapped on 16-bit Windows for three decades. We're freeing it.

> 💡 *Same legendary gameplay. All-original code. Every PRT, every LRT, every formula faithfully recreated from public documentation. Designed to run anywhere a browser lives.*

### Why this exists

- 🚫 The original *Stars!* is **abandonware** that won't run on modern 64-bit Windows without arcane hacks
- 📱 **No mobile version exists** — anywhere
- 💀 Every existing clone (craig-stars, Stars Nova, FreeStars, Thousand Parsec) is incomplete or stalled
- ⏳ The classic PBEM async multiplayer model **takes weeks per game** in 2026
- 🎯 The *Stars!* community has been waiting **30 years** for a proper remake

**STARS 2026 fixes all of it.**

---

## ✨ Features

### v0.1 — Core (in progress)
- 🌠 **Procedural galaxy generation** — Tiny to Huge, deterministic seeds
- 🧬 **Race designer** — Primary Racial Traits + Lesser Racial Traits, point-balanced
- 🪐 **Planet colonization** — habitability, terraforming, mining, production queues
- 🔬 **6-field tech tree** — Energy, Weapons, Propulsion, Construction, Electronics, Biotech
- 🚢 **Ship designer** — hull + components → derived stats
- ⚔️ **Tactical combat** — beams, torpedoes, shields, armor, the canonical *Stars!* battle resolution
- 🎯 **AI opponents** — rule-based, faithful, **never cheats**
- 💾 **Browser saves** — IndexedDB, instant load
- 👥 **Hotseat multiplayer** — local, no server needed
- 🎮 **Faithful 1995 pixel art** at modern resolution

### v0.2 — Networked Multiplayer
- 🌐 **1–16 players per game** (any mix human + AI)
- ⚡ **Speed modes:** Blitz (5 min) · Daily (24 hr) · Marathon (72 hr)
- 🤖 **AI takeover on missed deadlines** — you never get booted from a game
- 🏖️ **Vacation mode** — pause your empire safely, immune to attack
- 🔄 **Substitute players** — drop-in replacement for abandoned empires
- 📲 **Push notifications** — Web Push, FCM, APNs
- 🔐 **Determinism gate** — server and client produce byte-identical state, every turn

### v0.3 — Everywhere
- 📱 **iOS & Android** via Capacitor
- 🖥️ **Native desktop** via Tauri (Windows · macOS · Linux)
- ☁️ **Offline-first** single-player

### v1.0 — Full Game
- 🎭 **All 10 Primary Racial Traits** + complete LRT roster
- 💣 **Minefields** & 🚀 **Mass Driver Packets**
- 🏆 **Balance pass** verified across 1000 AI matches
- 🎨 **Final art** by hand, in Aseprite

### Forever
- 📦 **DLC framework** — new races, hulls, scenarios ship as `data/*.json` + sprite packs. Engine never forks.

---

## 🏗️ Tech Stack

> One Rust engine. Two compile targets. Identical bits.

| Layer | Choice | Why |
|---|---|---|
| **Game engine** | 🦀 **Rust** | Determinism, performance, single source of truth |
| **Browser delivery** | 🕸️ **WebAssembly** (wasm-bindgen + wasm-pack) | Native browser speed, no plugin |
| **Multiplayer server** (v0.2) | 🦀 Same Rust crate, native target + **Axum** | Byte-identical determinism client ↔ server |
| **Frontend** | ⚡ **SvelteKit** + TypeScript | Fast, modern, small bundles |
| **Styling** | 🎨 **Tailwind CSS** | Rapid iteration |
| **Galaxy renderer** | 🖼️ **HTML5 Canvas** | Pixel-perfect 2D |
| **Local saves** | 💾 **IndexedDB** | Browser-native, offline |
| **Multiplayer DB** | 🐘 **PostgreSQL** + **Redis** | State + realtime fanout |
| **Mobile** (v0.3) | 📱 **Capacitor** | iOS + Android from one codebase |
| **Desktop** (v0.3) | 🖥️ **Tauri** | Rust-native shell |
| **Build** | 📦 Cargo + ⚡ Vite | Standard |
| **Test** | 🧪 cargo test + Vitest + Playwright | Unit · Component · E2E |
| **Art** | 🎨 **Aseprite** | Pixel art standard |

📖 **Full technical details:** [SPEC.md](SPEC.md) · [ARCHITECTURE.md](ARCHITECTURE.md)

---

## 🎯 Project Status

🌿 **Phase 0: Skeleton (complete)**
- ✅ 10 governance documents
- ✅ Cargo workspace + engine crate (sentinel test green)
- ✅ SvelteKit frontend scaffold (svelte-check clean)
- ✅ Git history initialized

🌱 **Phase 1: Engine Foundation (next)**
- ⏳ Type system, RNG, galaxy generator, race system

🛣️ **Roadmap**

| Version | Scope | ETA |
|---|---|---|
| **v0.1** | Single-player + hotseat, core loop, deterministic engine | TBD |
| **v0.2** | Networked multiplayer 1–16 players, speed modes, AI takeover | After v0.1 |
| **v0.3** | Capacitor mobile + Tauri native desktop | After v0.2 |
| **v1.0** | Full 10 PRTs + LRTs + minefields + packets + final art | After v0.3 |
| **DLC** | New races, hulls, scenarios via data packs | Ongoing |

---

## 🧭 Design Principles

1. **Faithful, not slavish** — same gameplay, modernized UX, *zero* changes to combat math
2. **Determinism is law** — same seed + same orders = byte-identical outcome
3. **Sniff test after every function** — no shortcuts, no batching, no "I'll test it later"
4. **Clean room** — formulas sourced from public FAQ/wiki only, never decompiled binaries
5. **No microtransactions, no AI art, no Web3, no real-time mode** — ever

📖 [Read the full design doctrine →](SPEC.md#workflow-rules-non-negotiable)

---

## 🤝 Contributing

We're in Phase 0. Issues, ideas, and *Stars!* veterans welcome.

- 📋 [SPEC.md](SPEC.md) — full functional spec
- 🏛️ [ARCHITECTURE.md](ARCHITECTURE.md) — system design
- 🧪 [SCENARIOS.md](SCENARIOS.md) — holdout test scenarios
- 📐 [CODEGUIDE.md](CODEGUIDE.md) — Rust + TypeScript conventions
- 🔍 [SNIFFTEST.md](SNIFFTEST.md) — the mandatory test protocol
- 🎨 [ART.md](ART.md) — pixel art bible
- 👥 [AGENTS.md](AGENTS.md) — the specialist council

**Stars! veterans:** if you remember the canonical formulas, [open an issue](https://github.com/pnmcguire480/STARS-2026/issues) — we'd love your knowledge.

---

## 🔗 Reference Material

External references — studied, never copied:

- 🌐 **[craig-stars](https://github.com/sirgwain/craig-stars)** — MIT-licensed Go+SvelteKit clone, module-boundary reference
- 📚 **[starsfaq.com](http://www.starsfaq.com)** — canonical formula source
- 📖 **[Stars! Wiki at wiki.starsautohost.org](https://wiki.starsautohost.org)** — tech tables, hull specs, historical knowledge
- 💬 **[Stars! AutoHost](http://starsautohost.org)** — the community that kept the flame alive

---

## ❓ FAQ

<details>
<summary><b>Is this legal?</b></summary>

Yes. STARS 2026 is a clean-room implementation. Zero original code is used. All formulas come from publicly documented community sources (FAQ, wiki, manuals, decades of forum discussion). No binaries are decompiled. No assets are extracted.
</details>

<details>
<summary><b>Will it run on my phone?</b></summary>

That's the goal. v0.1 targets desktop browsers. v0.3 ships iOS + Android via Capacitor.
</details>

<details>
<summary><b>Why Rust + WASM instead of Go like craig-stars?</b></summary>

Two reasons: (1) the same Rust crate compiles to both browser WASM *and* native multiplayer server, guaranteeing byte-identical determinism. (2) Rust's type system makes it nearly impossible to accidentally introduce non-determinism into game logic.
</details>

<details>
<summary><b>How is this different from craig-stars?</b></summary>

craig-stars is excellent and we respect it deeply. Differences: (1) different language stack (Rust vs Go), (2) browser-first architecture, (3) mobile-bound from day one, (4) hard determinism contract from line one of the engine, (5) multiplayer designed around modern async/notifications instead of PBEM.
</details>

<details>
<summary><b>Will my single-player saves work in multiplayer?</b></summary>

No — they're different game types. But the engine is the same code, so a single-player game has the *exact same physics* as a multiplayer one.
</details>

<details>
<summary><b>How can I help right now?</b></summary>

Star the repo. Watch for v0.1. If you played the original and remember a formula, file an issue. If you draw pixel art, watch [ART.md](ART.md).
</details>

---

## 📜 License

TBD — will be a permissive open-source license (MIT or Apache 2.0) confirmed before v0.1 release.

## ⚠️ Not Affiliated

STARS 2026 is **not affiliated with, endorsed by, or derived from** the source code of the original *Stars!* by Jeff Johnson, Jeff McBride, or Empire Interactive / indiePub Entertainment. It is a clean-room implementation based on publicly documented gameplay mechanics. *Stars!* is a trademark of its respective owners.

---

<div align="center">

**🌟 If you played the original, you know why this matters. 🌟**

⭐ **Star the repo** · 👁️ **Watch for v0.1** · 🐦 **Share with your old PBEM crew**

*Built with love by [@pnmcguire480](https://github.com/pnmcguire480) · Powered by Rust 🦀 + SvelteKit ⚡*

`stars-game` `stars-1995` `4x-strategy` `space-strategy` `turn-based-strategy` `webassembly` `rust` `sveltekit` `space-4x` `pbem` `remake` `clean-room` `open-source-game` `browser-game` `pixel-art`

</div>
