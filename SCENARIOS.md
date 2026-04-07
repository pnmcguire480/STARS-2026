# SCENARIOS.md — Holdout Test Scenarios

> **WARNING:** This file is the holdout test set. It is **never** shown to the implementation model during a build session. It exists so a separate verification pass (Tier 5 architect, or a fresh Claude instance with no implementation context) can confirm the build actually works against unseen requirements.
>
> Treat this file like a final exam answer key. Read it before sign-off, hide it during the build.

---

## Tagging

Each scenario is tagged:
- **must_pass** — blocks the version release if it fails
- **should_pass** — fix before next milestone, but doesn't block

---

## v0.1 Scenarios (Single Player + Hotseat)

### S-01 [must_pass] Deterministic galaxy generation
**Given** seed `0xDEADBEEF` and galaxy size `Tiny`
**When** the engine generates a galaxy 1000 times
**Then** all 1000 galaxies are byte-identical (same star positions, names, mineral concentrations, hab values).

### S-02 [must_pass] Deterministic full-game replay
**Given** seed `0xCAFEBABE`, 1 human + 3 AI players, 200-turn game played to completion with a recorded order log
**When** the orders are replayed against a fresh engine instance
**Then** the final game state hashes byte-identical to the original.

### S-03 [must_pass] Race point balance
**Given** the canonical balanced race "Humanoid" from the legacy *Stars!* defaults
**When** the race point calculator runs
**Then** the total advantage points equals the documented value (sourced from starsfaq.com race wizard) within ±0 points.

### S-04 [must_pass] Habitability formula
**Given** a planet with grav=50, temp=50, rad=50 and a race with center hab=50/50/50, range ±20 on all
**When** habitability is calculated
**Then** the result is `100%` (perfect).

### S-05 [must_pass] Population growth — uncrowded
**Given** a planet at 10% capacity, 100% hab, race growth rate 15%
**When** one turn is processed
**Then** population grows by exactly 15.0% (rounded per *Stars!* convention).

### S-06 [must_pass] Population growth — crowded
**Given** a planet at 50% capacity, 100% hab, race growth rate 15%
**When** one turn is processed
**Then** growth is reduced by the canonical crowding penalty (see docs/FORMULAS.md when filled).

### S-07 [must_pass] Resource generation
**Given** a planet with 100,000 colonists, 100% hab, race economy 1000 colonists/resource
**When** resources are generated
**Then** the planet produces exactly 100 resources from population alone.

### S-08 [must_pass] Mineral extraction with depletion
**Given** a planet with concentration 100/100/100 ironium/boranium/germanium, 100 mines, mining rate 10%
**When** 10 turns of extraction are processed
**Then** concentrations decrease per the canonical depletion curve.

### S-09 [must_pass] Production queue order
**Given** a planet with a queue of `[Factory×10, Mine×5, ScoutShip×1]` and 1000 resources
**When** production runs
**Then** items are built in order, partial completions carry to next turn, no item is built out of order.

### S-10 [must_pass] Tech research progression
**Given** a player with energy=0, 100 resources/turn allocated 50% to energy
**When** N turns pass
**Then** energy level advances exactly when accumulated research equals the canonical level cost (sourced from starsfaq.com).

### S-11 [must_pass] Ship designer derived stats
**Given** a hull "Scout" + Engine "Quick Jump 5" + Scanner "Bat Scanner"
**When** the design is finalized
**Then** mass, fuel capacity, scanner range, and warp speed match the canonical *Stars!* values (sourced from wiki.starsautohost.org components table).

### S-12 [must_pass] Fleet movement and fuel burn
**Given** a fleet with Engine "Quick Jump 5" set to warp 6, traveling 100 LY
**When** movement resolves
**Then** the fleet arrives in the correct number of turns and consumes exactly the canonical fuel amount.

### S-13 [must_pass] Fuel exhaustion mid-route
**Given** a fleet with insufficient fuel for the full waypoint
**When** movement resolves
**Then** the fleet exhausts fuel and continues at the documented "free speed" (warp 1 / engine idle).

### S-14 [must_pass] Combat — one beam frigate vs one unarmed scout
**Given** Frigate (1× Laser, 0 shields, 0 armor) vs Scout (0 weapons, 0 shields, 10 armor)
**When** combat resolves
**Then** the Scout dies, the Frigate survives, and the canonical battle log matches the *Stars!* combat resolution order.

### S-15 [must_pass] Combat — initiative tiebreak
**Given** two identical fleets with identical ships
**When** combat resolves
**Then** initiative is broken by the canonical *Stars!* tiebreak rule and combat is deterministic.

### S-16 [must_pass] Turn order of events — full canonical 33 steps
**Given** a turn with a player who has fleets moving, planets producing, research advancing, and a battle pending
**When** the turn engine runs
**Then** all 33 steps execute in the canonical *Stars!* order (sourced from starsfaq.com order_events.htm) and side effects are visible only at the documented step.

### S-17 [must_pass] Save/load roundtrip
**Given** a game state at turn 47 with N players, M planets, K fleets, J pending battles
**When** state is serialized to IndexedDB and deserialized
**Then** the loaded state hashes byte-identical to the original and a subsequent turn produces the same result as if no save/load had happened.

### S-18 [must_pass] Hotseat privacy
**Given** a 2-player hotseat game where player A has just submitted orders
**When** player B takes the seat
**Then** player B's UI shows only player B's fog of war — no information from player A's view leaks through DOM, localStorage, or in-memory state.

### S-19 [should_pass] Performance — 200-turn AI game
**Given** a 1+3 AI Tiny galaxy
**When** the game runs to turn 200 with no UI, pure compute
**Then** total wall clock time is under 60 seconds on a mid-range 2024 laptop.

### S-20 [should_pass] WASM bundle size
**Given** a release build of the engine compiled to wasm32
**When** measured after wasm-opt and gzip
**Then** the resulting `.wasm.gz` is under 2 MB.

### S-21 [must_pass] Cross-target determinism (engine)
**Given** the same seed, same orders, same starting state
**When** turn N is generated **once on wasm32** and **once on native x86_64**
**Then** the resulting state hashes byte-identical.

---

## v0.2 Scenarios (Networked Multiplayer)

### S-22 [must_pass] Server-client determinism gate
**Given** a multiplayer game where the server generates turn N
**When** any client replays turn N from the same starting state and order log
**Then** the client's resulting state hashes byte-identical to the server's.

### S-23 [must_pass] AI takeover on missed deadline
**Given** a 16-player game where player 7's deadline expires with no orders submitted
**When** the deadline timer fires
**Then** within 1 second the server generates orders for player 7 from their last orders + standing doctrine and processes the turn.

### S-24 [must_pass] Vacation mode immunity
**Given** a player with vacation mode active
**When** another player attempts to attack one of their planets
**Then** the attack is blocked with the canonical "vacation immunity" result and no damage is dealt to either side.

### S-25 [must_pass] Substitute player handoff
**Given** a player who has been AI-controlled for 3+ turns and a new human requesting to substitute
**When** the substitute joins
**Then** they receive full visibility into their inherited empire, the AI relinquishes control, and no game state is lost.

### S-26 [must_pass] WebSocket presence latency
**Given** 16 connected clients
**When** one client submits orders
**Then** all other clients see the presence update within 200 ms p95.

### S-27 [should_pass] 16-player AI-only performance
**Given** a 16-AI Huge galaxy
**When** the server runs to turn 200
**Then** total server compute time is under 5 minutes.

---

## v1.0 Scenarios

### S-28 [must_pass] PRT balance — 1000 AI matches
**Given** all 10 PRTs in 1000 AI-vs-AI matches with random LRTs and balanced galaxies
**When** results are tallied
**Then** no PRT has a win rate above 65% or below 35%.

### S-29 [must_pass] Minefield decay and detonation
**Given** a heavy minefield from a Space Demolition race
**When** an enemy fleet enters at warp 8
**Then** detonation odds and damage match canonical formulas (sourced from starsfaq.com).

### S-30 [must_pass] Mass driver packet
**Given** a Packet Physics race firing a 100kT packet at warp 10 toward an enemy planet 5 LY away
**When** the packet impacts an undefended target
**Then** damage equals the canonical mass-driver impact formula.

---

## Notes

- All formulas referenced as "canonical" must be cited in `docs/FORMULAS.md` (created in Phase 0b) before the corresponding scenario is implemented.
- Scenarios are added/refined as systems are built. New scenarios go at the bottom with the next available S-NN.
- A scenario is **not** considered "passing" until it runs in CI.
