# Content Angles & Hooks — STARS 2026

Five angles the project tells the world. Each is a story we can lead with.

## 1. "Stars! finally runs on your phone."
**The hook:** A 1995 16-bit Windows game that has resisted modernization for 30 years now plays in any browser tab and installs on iOS/Android. No emulators. No `winevdm`. No DOSBox tricks.
**Who cares:** The veteran community that has been begging for this since the 2000s.
**Proof point:** Open the URL. Play. That's it.

## 2. "Determinism as a feature, not a footnote."
**The hook:** Same seed, same orders, byte-identical outcome — every time. The same Rust engine code runs in your browser AND on the multiplayer server, so the server can never tell you something the client can't verify. No more "the host's PC generated the turn weirdly."
**Who cares:** Competitive players, the autohost community, anyone who has ever lost a game to a desync or a corrupted save.
**Proof point:** A CI gate runs 100 random seeds through 200-turn games and hashes the final state. Any drift = build fails.

## 3. "Multiplayer that doesn't stall."
**The hook:** Most async 4X games die when one player goes silent. STARS 2026 fixes this with: deadlines that auto-resolve, AI takeover from your last orders + standing doctrine, vacation tokens that freeze your empire safely, and drop-in substitute players.
**Who cares:** Anyone who has ever been in a 16-player PBEM game that died on turn 47 because Steve stopped checking his email.
**Proof point:** The longest-running modern *Stars!* PBEM Discord can finish a 200-turn game in 6 months instead of 2 years.

## 4. "Faithful pixel art, modern resolution."
**The hook:** The 1995 aesthetic was perfect — and it scales. Crisp 2x/4x sprites, animated, modern color depth, but the *vibe* is intact. Aseprite-sourced. Hand-drawn. No AI art.
**Who cares:** Veterans who hate when remakes "modernize" the art into bland 3D mush.
**Proof point:** Side-by-side screenshots: 1995 original vs STARS 2026 at 4K. Same soul, sharper teeth.

## 5. "DLC without forks."
**The hook:** New PRTs, hulls, components, scenarios, and campaigns ship as `data/*.json` files plus sprite packs. Zero engine changes. The community can fork content without forking code, and we can keep the engine clean forever.
**Who cares:** Modders, content creators, anyone who remembers when *Stars!* mods were impossible because the binary was sealed.
**Proof point:** A first-party DLC pack ships with v1.0 to prove the data-only model works end to end.

---

## Channels

- **Primary:** GitHub repo + Discord (the *Stars!* community already lives there).
- **Secondary:** r/4Xgaming, r/strategygames, BoardGameGeek-adjacent video game communities.
- **Tertiary:** YouTube longform devlog — "Rebuilding Stars! from scratch in Rust."
- **Not the channels:** TikTok, Instagram, paid ads. Wrong audience.

## What we're NOT going to say

- "Revolutionary."
- "Reimagined for the modern era." (We *are* doing this, but saying it is hollow.)
- "AI-powered." (Anywhere.)
- "Web3."
- "Like *Stars!* but..." (No "but." Like *Stars!*. Period.)
