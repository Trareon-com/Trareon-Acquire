# Design plan — Trareon Acquire shell (frontend-design skill)

## Visual source of truth

**Layout SoT:** ASCII remount (banner → header → rail → safety → Source/Prepare/Run → status)  
**Taste reference (Lab family):** [`docs/media/screenshots/plan-ecr/taste-reference-lab-ecr.png`](../../../docs/media/screenshots/plan-ecr/taste-reference-lab-ecr.png)  
**Legacy ECR screenshot** (`03-acquire-from-lab-ecr.png`): copper / density reference only — not layout lock.

Structured Slint shell with progressive disclosure (Guided density, Advanced drawer). Theme/locale live in the header (Help keeps secondary settings).

## Chrome (fixed heights)

| Region | Height / width | Content |
|--------|----------------|---------|
| Disclosure banner | 26px | Lab use only · NOT court-ready · NOT ISO-certified |
| Header | 48px | TRAREON \| ACQUIRE \| Case \| Search \| EN\|ID \| theme |
| Rail | 188px wide (56px compact) | WORKFLOW / LAB / SUPPORT (version lives in About) |
| Safety strip | 40px | CASE · WB · INT \| icon stage tabs PREPARE / ACQUIRE / SEAL |
| Status bar | 30px | Hardware status icons (disks / blocker / integrity) · Coverage |

## Perfect Product IA (M14)

Rail groups:

1. **WORKFLOW:** Cases → Identify → Acquire → Triage  
2. **LAB:** Tools → QMS → Boot  
3. **SUPPORT:** Help · About  

**Safety strip** (always): Case · Write-blocker · Integrity + 3-stage dots  

Footer ambient: Evidence count · Coverage · acquire-mode (not blockers).

## Acquire body (best practice)

LTR zones (wide):

1. **SOURCE ~42%** — host disks (+ detail); Guided → demo card  
2. **PREPARE ~33%** — mode, format, paths, confirm, Advanced ▸  
3. **RUN/SEAL ~25%** — single START ACQUIRE CTA, Cancel/Demo, progress, Seal post-run  

## Responsive breakpoints

| Mode | Window width | Behavior |
|------|--------------|----------|
| Wide | ≥ 1280px | Rail labeled; 3 columns; full search |
| Medium | 1100–1279px | Rail labeled; 3 columns; search shortened |
| Compact | 920–1099px | Icon rail 56px; body stacks Source → Prepare → Run in ScrollView |
| Minimum | 920 × 680 | `Theme.window-min-w` / `window-min-h` |

Search jumps nav by keyword (cases, identify, acquire, triage, tools, qms, boot, help, about).

## Aesthetic

Light lab instrument: white / soft gray, burnt-orange copper (`#AF622E`), thin hairlines,
small radii (~4–6px), Avenir Next + Menlo. Copper only for active nav, active stage,
selected format/radio, and **START ACQUIRE**.

## Token system (light SoT)

| Name | Hex | Role |
|------|-----|------|
| void | `#F8F9FA` | Window / bench wash |
| ink / panel | `#FFFFFF` | Rail / cards |
| raised | `#F3F5F7` | Inputs / footer |
| hairline | `#E0E3E7` | 1px structure |
| copper | `#AF622E` | Primary CTA + active |
| copper-dim | `#8F4A24` | Dim accent |
| readout / mute / faint | `#2C3E50` / `#718096` / `#8A94A0` | Text |
| ok-glow | `#2F855A` | Connected / OK |
| unsigned-well | `#FFF2E8` | Disclosure banner |

Dark theme available via header toggle or Help → Settings (prefs).

## UX rules (summary)

- Stage dots from state (Prepare / Acquire / Seal) — not a separate Rust machine
- Format hierarchy: fsnap = Court path A; lite/Advanced = subset labels
- START disabled + reason line; busy-lock; Seal fills after run
- Deep links: Case chip → Cases; Identify gate banner → Identify; Search → nav

## Critique vs AI defaults

- Rejected cream + serif brochure and neon cyber HUD
- Copper replaces neon; light Lab is SoT (not dark-first SaaS)
- ASCII chrome + LTR acquire zones replace the old ECR 78px instrument / 5-step wizard lock
