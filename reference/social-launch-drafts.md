# Social Launch Drafts — STARS 2026

> Copy/paste these when you're ready. Local-only file (gitignored via reference/ being intentionally committed but you can move this anywhere).

---

## 🟧 Reddit — r/4Xgaming

**Title:**
> I'm rebuilding *Stars!* (1995) from scratch in Rust + WebAssembly. Browser-native, mobile-bound, 1–16 player async multiplayer.

**Body:**

Long-time *Stars!* veterans — this one's for you.

The original *Stars!* (Jeff Johnson + Jeff McBride, 1995) is a 16-bit Windows binary that won't run on modern hardware without `winevdm` hacks or Win98 VMs. There's no mobile version. Every existing clone (craig-stars, Stars Nova, FreeStars) is incomplete or stalled. The community has been waiting 30 years for a proper remake.

I'm building one. Open source. Clean room. From scratch.

**Stack:**
- 🦀 Rust game engine, deterministic, compiles to **both** browser WASM and native multiplayer server (byte-identical bits — same seed + same orders = same outcome, every time, on every platform)
- ⚡ SvelteKit + TypeScript frontend, HTML5 Canvas galaxy map
- 🐘 Postgres + Redis + Axum for v0.2 multiplayer
- 📱 Capacitor (mobile) + Tauri (native desktop) in v0.3
- 🎨 Faithful 1995 pixel art, modern resolution, hand-drawn in Aseprite

**Roadmap:**
- **v0.1** — single-player + hotseat, core loop, deterministic engine
- **v0.2** — networked 1–16 players, Blitz/Daily/Marathon speed modes, **AI takeover on missed deadlines** (you never get booted), vacation mode, push notifications
- **v0.3** — iOS/Android/desktop wraps
- **v1.0** — full 10 PRTs, LRTs, minefields, packets, balance pass

**Design rules I'm holding myself to:**
- Faithful, not slavish — same combat math, modernized UX
- Sniff test after every function (no shortcuts, no batching, ever)
- Every formula sourced to starsfaq.com / wiki.starsautohost.org — never guessed
- No AI art, no microtransactions, no Web3, no real-time mode

**What I need from you:**
If you played the original and remember a formula, an edge case, or "the way it really worked" — there's a [Veteran Knowledge issue template](https://github.com/pnmcguire480/STARS-2026/issues/new?template=veteran_knowledge.yml) ready for you. You don't need to write code. Your memory is the highest-leverage contribution there is.

**Repo:** https://github.com/pnmcguire480/STARS-2026

Currently in Phase 0 (skeleton committed, CI green). Phase 1 — engine foundation — starts this week. Watch for v0.1.

⭐ if you've been waiting for this. 💬 in Discussions if you have lore to share.

---

## 🟪 Stars! Discord (find via starsautohost.org)

Hey everyone — long-time *Stars!* fan here. I'm finally building the remake we've been talking about for 20 years.

🌌 **STARS 2026** — clean-room reimagining of *Stars!* in Rust + WebAssembly. Browser-native. 1–16 player async multiplayer with AI takeover on missed deadlines (no more dead PBEMs). Mobile + desktop wraps in v0.3. Faithful pixel art. Open source.

📦 Repo: https://github.com/pnmcguire480/STARS-2026
🛣️ Roadmap: v0.1 single-player → v0.2 multiplayer → v0.3 mobile → v1.0 full game

Just shipped Phase 0 — governance docs, Cargo workspace, SvelteKit scaffold, CI green. Phase 1 starts this week.

I need veterans. If you remember a formula, an edge case, the *real* way HE-vs-IS used to play out — there's a [Veteran Knowledge issue template](https://github.com/pnmcguire480/STARS-2026/issues/new?template=veteran_knowledge.yml) ready for you. You don't have to write code. Just tell me what you remember and link a source.

Watch the repo for v0.1 release pings. ⭐ if you've been waiting.

---

## 🐦 Twitter / X / Bluesky thread

**1/**
🌌 I'm rebuilding *Stars!* — the legendary 1995 4X strategy game — from scratch in Rust + WebAssembly.

Browser-native. Deterministic. 1–16 player async multiplayer that doesn't stall.

Open source. Clean room. Mobile-bound.

🧵👇

**2/**
Why?

The original *Stars!* is 16-bit Windows abandonware. It won't run on modern hardware. Every existing clone is stalled. There's no mobile version. The community has been waiting 30 years.

I'm done waiting. 🚀

**3/**
The architecture is the part I'm proudest of:

One Rust crate. Two compile targets. Browser WASM + native multiplayer server. **Byte-identical bits.**

Same seed + same orders = same outcome. Every time. On every platform. The server can never tell you something the client can't verify.

**4/**
Multiplayer that doesn't stall:

⚡ Speed modes: Blitz (5 min) · Daily (24 hr) · Marathon (72 hr)
🤖 AI takeover when you miss a deadline (you never get booted)
🏖️ Vacation mode with empire freeze + immunity
🔄 Substitute players for abandoned empires
📲 Push notifications

**5/**
Stack:

🦀 Rust engine
🕸️ WebAssembly
⚡ SvelteKit + TypeScript
🎨 Tailwind + HTML5 Canvas
🐘 PostgreSQL + Redis + Axum (v0.2)
📱 Capacitor + Tauri (v0.3)
🎨 Faithful 1995 pixel art in Aseprite

**6/**
Phase 0 just shipped:
✅ 10 governance docs
✅ Cargo workspace
✅ SvelteKit scaffold
✅ CI green

Phase 1 (engine foundation) starts this week. Watching the build live? Star + watch the repo:

https://github.com/pnmcguire480/STARS-2026

**7/**
If you played the original — file a Veteran Knowledge issue. Your memory is gold. Every formula has to be sourced to public docs and your recollection counts.

Let's bring this game back. ⭐

#4X #IndieGameDev #Rust #WebAssembly #SvelteKit #StrategyGames #OpenSource #StarsGame

---

## 📌 Reminders

- **Pin the repo** manually: github.com/pnmcguire480 → "Customize your pins" → tick STARS-2026
- **Set a social preview image** manually: Repo → Settings → Social preview → upload PNG (1280×640)
- **Watch your own repo** as Releases-only so you get notified of CI on tags
- **First post timing:** Tuesday or Wednesday morning EST tends to peak r/4Xgaming
