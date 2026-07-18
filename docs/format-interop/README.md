# Format interoperability operator pack

**Path A (candidate RAW interoperability; external verification pending):** RAW / `.fsnap` — see [PATH-A-RAW.md](PATH-A-RAW.md). Authority for green/red rows: [EVIDENCE.md](EVIDENCE.md).

**Path B (competitive E01):** `ewf-image` writer is wired (MSRV 1.96, feature `ewf`).
UI still says **E01-lite** until libewf/Autopsy evidence is green —
see [EWF-SPIKE.md](EWF-SPIKE.md).

`trareon.e01-lite/1` is a documented subset, not a claim of full EWF/libewf compatibility. The
smoke script creates synthetic bytes, writes an E01-lite file through the core example, performs a
local round trip, and records the reported SHA-256 in a sidecar evidence folder.

Run:

```sh
scripts/format-interop-smoke.sh
# Optional Path B spike (ignore-rust-version sandbox):
scripts/ewf-spike.sh
```

The script output is software evidence only. Complete human checks in [EVIDENCE.md](EVIDENCE.md)
before any interoperability claim. Optional extras: [OSS-EXTRAS.md](OSS-EXTRAS.md).

The output contains synthetic data and is not forensic evidence.
