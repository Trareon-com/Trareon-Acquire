# Wave 3c — OSS extras — DONE (adapters)

Evaluated and wired 2026-07-18.

| Project | License | Integration |
|---|---|---|
| `ewf-image` | Apache-2.0 | Path B writer (Wave 3a) — feature `ewf` |
| `zff` / `zffacquire` | MIT OR Apache-2.0 | Optional output via external `zffacquire` — feature `zff` (`write_zff_physical`) |
| [microsoft/avml](https://github.com/microsoft/avml) | MIT | `sources_ext::probe_avml` / `run_avml_capture` — spawn when present |
| [Lazza/Fuji](https://github.com/Lazza/Fuji) | GPL-3.0 | `sources_ext::probe_fuji` / `run_fuji_plan` — probe + help dry-run; audit before deeper reuse |

## Adapter contract

- Probe `PATH` or `TRAREON_AVML` / `TRAREON_FUJI` / `TRAREON_ZFFACQUIRE`.
- On miss: honest `Unavailable` / `Verification` error — never fake RAM or ZFF bytes.
- UI Source kind chips surface probe text (RAM → avml, Snapshot → fuji).
- ZFF format chip (index 8) calls `zffacquire physical` when the feature + binary exist.

## Follow-ups (not blockers)

- Embed `zff` crate writer directly once a minimal physical API is reviewed.
- Fuji full logical workflow after GPL reuse audit.
- libewf `ewfverify` CI job when runners have libewf packages.
