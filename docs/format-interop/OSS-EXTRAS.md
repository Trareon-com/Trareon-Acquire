# Wave 3c — optional OSS extras (post-E01)

Evaluated 2026-07-18. None of these replace `.fsnap` / Path A.

| Project | License | Decision |
|---|---|---|
| [zff-rs](https://github.com/zff-team/zff-rs) (`zff` crate) | Apache-2.0 | **Optional output** after Path B EWF is green; not a court default |
| [microsoft/avml](https://github.com/microsoft/avml) | MIT | Linux RAM: spawn external binary when present; else UI `Unavailable` |
| [Lazza/Fuji](https://github.com/Lazza/Fuji) | GPL-3.0 | macOS logical/triage patterns; audit before any code reuse |

## Adapter contract (sources_ext)

- Probe `PATH` for `avml` / documented Fuji workflow.
- On miss: return honest `Unavailable` with install note — never fake a RAM image.
- Multisource governor already limits parallel disk+RAM once adapters exist.
