# ART.md — STARS 2026 Pixel Art Bible

## North Star
Faithful 1995 *Stars!* aesthetic, modern resolution. The vibe is intact, the pixels are sharper.

## Resolution Strategy
- **Source resolution:** native pixel art at the size it was designed.
- **Display resolution:** integer scaling (2×, 3×, 4×) to fill modern displays without blur.
- **No bilinear filtering anywhere.** Nearest-neighbor only.
- **Canvas rendering:** `imageSmoothingEnabled = false` always.

## Palette
- Start with a constrained palette (~32 colors) reminiscent of the 1995 VGA era.
- Add a few accent colors for modern HUD readability.
- All sprites snap to the canonical palette. No off-palette colors.
- **Palette file:** `art/palette.gpl` (GIMP/Aseprite format).

## Sprite Sizes (canonical)
| Asset | Source size | Notes |
|---|---|---|
| Stars (galaxy map) | 8×8 | Class O–M color variation via palette swap |
| Planets (galaxy map) | 16×16 | Owner color overlay |
| Planets (planet view) | 64×64 | Hi-detail with terraform indicators |
| Ship hulls (designer view) | 64×64 | One per hull type |
| Ship hulls (combat view) | 32×32 | Animated 4-frame loop |
| UI icons | 16×16 and 24×24 | Resource, mineral, tech, action icons |
| Combat backgrounds | 320×240 | Tiled or scrolling |

## Animation
- 4–8 frame loops at 8–12 FPS for ships in combat.
- Subtle 2-frame idle for planets.
- No tweening. Frame-by-frame only.

## Tooling
- **Aseprite** is the source of truth. `.aseprite` files live in `art/source/`.
- Export to PNG atlases via Aseprite CLI in a build script.
- Atlases live in `frontend/static/sprites/`.

## Naming
- `art/source/ships/scout.aseprite` → `frontend/static/sprites/ships/scout.png`
- Atlas metadata in JSON next to PNG.

## What We Will NOT Do
- ❌ AI-generated art. Anywhere. Ever.
- ❌ 3D models pretending to be pixel art.
- ❌ Bilinear/anisotropic filtering.
- ❌ Modern hi-res "HD remaster" sprites that lose the 1995 soul.
- ❌ Stock asset packs.

## Phase Plan
- **v0.1:** Placeholder programmer art. Solid colors, simple shapes. Mechanics first.
- **v0.2:** Coordinated palette + first ship pass.
- **v0.3:** Full UI iconography pass for mobile.
- **v1.0:** Final art bible enforced, all placeholders replaced, animation pass.

## References
- Original *Stars!* screenshots (study only, do not copy or trace).
- Master of Orion 1 & 2 sprite work (era reference).
- Aseprite documentation for animation timing.
