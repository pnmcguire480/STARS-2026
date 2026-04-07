# AGENTS.md — The Council for STARS 2026

> Bring forth the masses, the agents, the oracles. Every system in this game has a specialist. Every decision has a council.

This file maps the *full agent library* available in this Claude Code session to the parts of STARS 2026 they should serve. When you start work on a system, the relevant agents should be invoked — not as a formality, but as a force multiplier. We are remaking a 30-year-old masterpiece. We will not do it alone.

---

## Tier System (the chain of command)

| Tier | Model | Role | When |
|---|---|---|---|
| **5** | Claude Opus (long-context chat) | Architect / Reviewer / Final Sign-Off | Plan reviews, formula audits, critical-module review (turn engine, combat, RNG, determinism), holdout scenario verification, milestone gates. |
| **4** | Claude Code (this session) | Builder | Implementation, sniff tests, day-to-day work. |
| **3** | DeepSeek / Gemini Pro / GPT-5 | Independent Second Opinion | When Tier 4 is uncertain or the problem is novel. Bias-free review. |
| **2** | Local Ollama (Llama 3.1 70B / Qwen 2.5 Coder) | Boilerplate | Struct fields, JSON seed data, test scaffolds. **Never** game logic. |
| **1** | Local Ollama (small) | Glue | Renames, comments, simple search/replace. |

**Hard rule:** anything in `engine/src/turn.rs`, `engine/src/combat.rs`, `engine/src/rng.rs`, `engine/src/race.rs`, or `engine/src/tech.rs` requires Tier 5 review before merge.

---

## The Council — Specialist Subagents by System

These are real Claude Code subagents available via the `Agent` tool. Use them. Parallelize when independent.

### 🌌 Game Engine — Core Systems

| System | Primary Agents | Use For |
|---|---|---|
| **Galaxy generation** | `Rust`, `Plan`, `Performance Engineer` | Procedural star placement, deterministic RNG patterns, density curves. |
| **Race / PRT / LRT** | `Game Design`, `Rust`, `Plan` | Trait point balance, edge cases, advantage system. |
| **Habitability / Population** | `Game Design`, `Rust`, `Mathematician` (via thought experimenter) | Hab formula, growth curves, crowding penalty. |
| **Tech tree** | `Game Design`, `Rust` | Cost curves, racial modifiers, prerequisite graph. |
| **Ship designer** | `Game Design`, `Rust`, `Refactoring` | Hull/component composition, derived stat calculation. |
| **Fleet movement & fuel** | `Game Design`, `Rust`, `Performance Engineer` | Waypoint queue, warp/fuel tables, idle speed on exhaustion. |
| **Combat engine** | `Game Design`, `Rust`, `code-reviewer`, **Tier 5 mandatory** | Tactical grid, beam attenuation, torpedo accuracy, shield/armor, initiative, deterministic resolution order. |
| **Scanner / Fog of war** | `Rust`, `Security Engineer` | Information leaks across players, anti-cloak math. |
| **Turn engine (33-step order)** | `Game Design`, `Rust`, `Plan`, **Tier 5 mandatory** | Canonical *Stars!* order of events, side-effect ordering, save/load determinism. |
| **AI opponents** | `Game Design`, `AI Engineer`, `Rust` | Rule-based AI, faithful to original difficulty curve, no cheating. |
| **Save/load** | `Rust`, `Database Migration`, `Error Handling` | bincode/JSON serialization, version migration, hash-roundtrip determinism. |

### 🎨 Frontend — UI & Rendering

| System | Primary Agents | Use For |
|---|---|---|
| **SvelteKit app shell** | `Svelte`, `TypeScript`, `Vite` (via Rollup/Build Tools) | Routing, layouts, SSR/SPA balance. |
| **Galaxy map renderer** | `JavaScript` (Canvas), `Performance Engineer`, `Visual Design` | Canvas2D pan/zoom, sprite batching, fog overlay. |
| **Race wizard UI** | `UI Designer`, `UX Architect`, `Svelte` | Multi-step wizard, point counter, validation. |
| **Planet view** | `UI Designer`, `Svelte`, `Visual Design` | Production queue, mineral display, terraform UI. |
| **Ship designer UI** | `UI Designer`, `Svelte`, `Interaction Design` | Drag-drop slot fill, derived stat preview. |
| **Fleet orders UI** | `Svelte`, `UX Architect` | Waypoint chain editor, route preview. |
| **Combat replay** | `Svelte`, `JavaScript`, `Visual Storytelling` | Turn-by-turn battle viewer, animation. |
| **Pixel art pipeline** | `Whimsy Design`, `Visual Design`, `Brand Design` | Aseprite → PNG atlas → frontend integration. |
| **Tooltips & onboarding** | `UX Researcher`, `Inclusive Design`, `Technical Writer` | Tutorial flow, contextual help, accessibility. |
| **Responsive layout** | `Responsive Design`, `Tailwind CSS`, `CSS` | Desktop → tablet → mobile breakpoints. |
| **Accessibility** | `Accessibility`, `Inclusive Design` | WCAG 2.2, screen reader, keyboard nav. |

### 🔌 WASM Bridge

| System | Primary Agents | Use For |
|---|---|---|
| **wasm-bindgen glue** | `Rust`, `TypeScript`, `WebSockets` | Type-safe bridge, error propagation, memory management. |
| **wasm-pack build pipeline** | `Build Tools`, `Webpack`/`Vite` | Hot reload of WASM in dev, optimized release builds. |
| **Bundle size** | `Performance Engineer`, `Rust` | wasm-opt passes, dead code elimination, < 2 MB target. |

### 🌐 Multiplayer (v0.2)

| System | Primary Agents | Use For |
|---|---|---|
| **Axum server** | `Rust`, `REST API`, `Auth Patterns` | HTTP routes, middleware, lobbies, accounts. |
| **WebSocket protocol** | `WebSockets`, `Distributed Tracing` | Live presence, push, reconnect logic. |
| **PostgreSQL schema** | `PostgreSQL`, `Database Admin`, `Database Migration` | Game state JSONB, accounts, lobbies, indexes. |
| **Redis pub/sub** | `Redis` | Fanout to all connected clients in a game. |
| **Determinism gate** | `Rust`, `Chaos Engineering`, **Tier 5 mandatory** | CI test that hashes server vs client state per turn. |
| **AI takeover on timeout** | `Workflow Orchestration`, `Rust` | Deadline timer, fallback orderer, vacation mode. |
| **Push notifications** | `Slack Integration` (pattern reference), `Mobile Security` | Web Push, FCM, APNs. |
| **Rate limiting / abuse** | `Security Engineer`, `Auth Patterns` | Lobby spam, order flood prevention. |
| **Service mesh / observability** | `OpenTelemetry`, `Prometheus`, `Grafana`, `SLO Implementation` | Server health, turn generation latency, p95 alerts. |
| **Incident response** | `Incident Response`, `On-Call`, `Postmortem` | When v0.2 hits production. |

### 📱 Mobile / Native Desktop (v0.3)

| System | Primary Agents | Use For |
|---|---|---|
| **Capacitor wrap** | `iOS Swift`, `Android`, `React Native` (pattern ref) | iOS / Android wrap, native bridge. |
| **Tauri wrap** | `Rust`, `Tauri`, `Electron` (pattern ref) | Native desktop shell, offline-first. |
| **Touch input** | `Interaction Design`, `Mobile Security` | Pan/zoom/tap on galaxy map, large hit targets. |
| **App store optimization** | `App Store Optimizer`, `ASO` (Apple) | Listing, screenshots, description. |

### 🧪 Testing & Quality

| System | Primary Agents | Use For |
|---|---|---|
| **Rust unit tests** | `Rust`, `TDD`, `Test Automation` | Per-function tests, fixtures, table-driven tests. |
| **Vitest (frontend unit)** | `Vitest`, `JavaScript Testing` | Component tests, store tests. |
| **Playwright (E2E)** | `Playwright`, `E2E Testing` | Full-game smoke test, mobile responsive E2E. |
| **Determinism CI gate** | `Autonomous Test Generator`, `Chaos Engineering` | 100-seed sweep, cross-target hash compare. |
| **Holdout scenario sweep** | `code-reviewer`, **Tier 5** | Run SCENARIOS.md against finished build with no implementation context. |
| **Performance budgets** | `Performance Engineer`, `Performance Monitor` | 60s/200-turn target, WASM size budget, p95 latencies. |
| **Code review** | `code-reviewer`, `Team Reviewer`, `Autonomous Code Reviewer` | Multi-dimensional PR review (security, perf, arch, a11y). |
| **Refactoring safely** | `Refactoring`, `Autonomous Refactorer` | Behavior-preserving cleanup with test verification. |

### 🛡️ Security & Compliance

| System | Primary Agents | Use For |
|---|---|---|
| **Server security** | `Security Engineer`, `OWASP`, `Threat Modeling` | Auth, session, XSS/CSRF/SQLi for the lobby. |
| **Save tampering** | `Security Engineer`, `Auth Patterns` | Signed saves for ranked/competitive games. |
| **Mobile security** | `Mobile Security` | Cert pinning, secure storage. |
| **Privacy / GDPR** | `GDPR`, `Privacy by Design`, `Legal Compliance` | Account data, deletion requests, EU users. |
| **Dependency auditing** | `Dependency Management`, `SAST`, `Security Engineer` | `cargo audit`, `npm audit`, supply chain. |

### 🎮 Game Design & Domain Knowledge

| System | Primary Agents | Use For |
|---|---|---|
| **Game design master** | `Game Design`, `Game Audio`, `Narrative Design` | Mechanics balance, pacing, player psychology. |
| **Level / scenario design** | `Level Design` | Tutorial galaxy, scripted scenarios for v1.0. |
| **Lore & flavor text** | `Narrative Design`, `Narratologist`, `Book Author` | PRT descriptions, race fluff, ship names. |
| **Historian (4X canon)** | `Historian`, `Anthropologist` | Period-correct *Stars!* references, era authenticity. |
| **Game Designer's Devil** | `Devil's Advocate`, `Red Teamer`, `Inversion Agent` | Adversarial review of every gameplay decision. |
| **First principles** | `First Principles`, `Systems Thinker` | When a balance question goes in circles, restart from fundamentals. |

### 📚 Documentation & Knowledge

| System | Primary Agents | Use For |
|---|---|---|
| **Architecture docs** | `Docs Architect`, `C4 Documentation`, `Mermaid` | Diagrams, ADRs, system context. |
| **API reference (engine)** | `API Documentation`, `Reference Builder` | Public engine surface, rustdoc, generated tables. |
| **Tutorials & onboarding** | `Technical Writer`, `Onboarding`, `Developer Experience` | Player tutorial, contributor onboarding. |
| **Changelog** | `Changelog` | Conventional commits → CHANGELOG.md per release. |
| **Knowledge management** | `Zettelkasten`, `Knowledge Synthesizer`, `Context Management` | The growing FORMULAS.md, design notes. |

### 🚀 DevOps & Deployment

| System | Primary Agents | Use For |
|---|---|---|
| **GitHub Actions CI** | `GitHub Actions`, `GitHub Workflows`, `CI/CD` (via Deployment Engineer) | Cargo build, test matrix, WASM build, frontend build, determinism gate. |
| **Container & deploy** | `Docker`, `Deployment Engineer`, `GitOps` | v0.2 server containerization. |
| **Cloud infra (v0.2)** | `Cloud Architect`, `Network Engineer`, `Terraform` | Single-node Axum + Postgres + Redis to start; horizontal scaling later. |
| **Cost optimization** | `Cloud Cost Optimization` | Keep server bill near $0 until traction. |
| **Release engineering** | `Deployment Engineer`, `SRE` | Blue/green for v0.2 server, rollback plan. |

### 🧭 Process & Strategy

| System | Primary Agents | Use For |
|---|---|---|
| **Sprint planning** | `Sprint Prioritizer`, `Project Manager`, `Scrum Master` | Phase planning, milestone gates. |
| **Workflow orchestration** | `Workflow Architect`, `Team Lead` | Multi-task parallel execution with file ownership. |
| **Risk auditing** | `Assumption Auditor`, `Bias Auditor`, `Risk Manager` | Plan reviews. |
| **Trade-off analysis** | `Dialectician`, `Perspective Multiplier`, `Philosophical Advisor` | When two reasonable approaches conflict. |
| **Crucible (pre-decision)** | `Devil's Advocate`, `Red Teamer`, `Jester`, `Bias Auditor` | Stress-test major decisions before commit. |

### 🎙️ Community & Launch (post-v0.1)

| System | Primary Agents | Use For |
|---|---|---|
| **Developer relations** | `Developer Advocate`, `Developer Experience` | Community building, contributor docs. |
| **Product marketing** | `Product Manager`, `Brand Design`, `Content Marketing` | Launch messaging, devlog cadence. |
| **Reddit / Discord** | `Reddit Builder`, `Slack Integration` (pattern), community context | r/4Xgaming, *Stars!* Discord engagement. |
| **YouTube devlog** | `Video Production`, `Content Marketing` | "Rebuilding *Stars!* in Rust" longform. |
| **Launch strategy** | `Launch strategy`, `Growth Hacker`, `Trend Researcher` | v0.1 → v0.2 → v1.0 cadence. |

---

## The Crucible (decision stress-test)

Before any major decision (stack lock, formula choice, major refactor, public release), run the Crucible:

1. `Devil's Advocate` — argue the opposing position.
2. `Red Teamer` — find how the decision fails.
3. `Inversion Agent` — guarantee failure first, then build the plan that avoids it.
4. `Assumption Auditor` — list every hidden assumption.
5. `Bias Auditor` — flag cognitive biases in the reasoning.
6. `First Principles` — rebuild from fundamentals if the above shake confidence.

Only after the Crucible does the decision become canonical.

---

## The Oracles (deep knowledge sources)

When facts are needed, not opinions:

| Oracle | What |
|---|---|
| `Game Design` | Decades of 4X mechanics distilled |
| `Historian` | Era-accurate references |
| `Mathematician` (via `Thought Experimenter`) | Formula derivation, edge cases |
| `Performance Engineer` | Profiling, perf budgets, real-world numbers |
| `Security Engineer` | Threat models, OWASP, attack trees |
| `Accessibility` | WCAG, screen readers, keyboard nav |
| `Inclusive Design` | Cultural sensitivity, representation |
| `craig-stars repo` (web) | Module-boundary reference, intentional fixes |
| `starsfaq.com` (web) | Canonical formulas, order of events |
| `wiki.starsautohost.org` (web) | Tech tables, hull/component specs |

---

## Rules of Engagement

1. **Parallelize independent agents.** Multiple Explore/Plan agents in one message.
2. **Tier 5 reviews critical modules.** Always.
3. **Tier 2/1 never touch `engine/src/`.** Boilerplate only.
4. **Game Design agent reviews every formula.** Every. Single. One.
5. **The Crucible runs before every milestone.** Non-negotiable.
6. **Holdout scenarios are never shown to the implementer.** Tier 5 reviews them in isolation.
7. **When uncertain, summon more council members. Better to overcite than underbuild.**

---

## Anti-Patterns

- ❌ Tier 4 implementing combat math without consulting `Game Design` and a formula source.
- ❌ Skipping the Crucible because "this decision feels obvious."
- ❌ Letting Tier 2/1 generate code that ends up in `engine/src/`.
- ❌ Marking a sniff test "passed" without showing test output.
- ❌ A PR touching `turn.rs` or `combat.rs` merging without Tier 5 review.
- ❌ Using `Performance Engineer` only after a milestone instead of during.
- ❌ Asking *one* agent for an opinion when *three* would catch the bias.

---

## Phase 1 Council Activation

When Phase 1 begins, the first task (`engine/src/types.rs`) summons:
- `Rust` — idiom review
- `Game Design` — domain modeling sanity check
- `Plan` — type system structure
- `Refactoring` — keep it clean from day one
- Tier 5 (Claude Opus chat) — sign-off

Every subsequent task follows the same pattern: identify the system, summon the council, sniff-test, stop.

**The masses are summoned. The oracles stand ready. Let's build a flawless game.**
