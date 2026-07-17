# Design plan — Trareon Acquire shell (frontend-design skill)

## Subject
Desktop forensic **acquire** tool for lab operators / Founder buyers.
Audience: DFIR operators comparing to CollectionLoom-class tools.
Single job: mode → confirm → acquire → show verified hash.

## Brief
Clean, simple, luxurious — not loud SaaS, not hacker dark, not cream/terracotta.

## Token system

### Color
| Name | Hex | Role |
|------|-----|------|
| graphite | `#161B22` | Primary ink / active mode |
| slate-paper | `#E9EEF2` | Lab bench surface |
| porcelain | `#FCFCFD` | Raised work surface |
| case-indigo | `#2A3F5F` | Accent (case-file blue, not purple SaaS) |
| mute | `#66707C` | Secondary text |
| faint | `#94A0AB` | Labels / captions |
| line | `#D2D9E0` | Hairlines |
| deny | `#8E2F2F` | Preflight deny |
| seal | `#F7F1E6` | Hash seal panel (warm paper *inside* seal only) |

### Type
- Display (brand only): Optima / Avenir Next — tracked TRAREON, large Acquire
- Body: native UI face (quiet)
- Utility (hash): Menlo / Consolas — oversized SHA as artifact

### Layout
Quiet instrument bench: brand thesis at top, controls mid, **hash seal** as the only loud region at bottom.

### Signature
**Evidence seal** — oversized monospace SHA-256 block as the product’s jewel; everything else restrained.

## Critique vs AI defaults
- Rejected cream+#serif+terracotta page
- Rejected near-black + acid green
- Rejected broadsheet dense columns
- Rejected purple gradient SaaS teal clone as the *only* idea — indigo case-file + seal is forensic-specific
