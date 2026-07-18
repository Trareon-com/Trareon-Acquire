# Format interoperability evidence

Date: 2026-07-18  
Operator: _(software prep)_  
Reviewer: _(pending human)_  
Commit: _(fill at sign-off)_  
OS and version: _(fill at sign-off)_

## Path A — RAW / `.fsnap` (official court track)

See [PATH-A-RAW.md](PATH-A-RAW.md).

| Tool | Exact version | Open result | Extracted hash | Notes |
|---|---|---|---|---|
| Autopsy | | [ ] pass [ ] fail | | Open raw evidence file from `.fsnap` |
| FTK / FTK Imager | | [ ] pass [ ] fail | | Same raw bytes |

## Path B — EWF (`ewf-image`) — writer shipped, oracle pending

MSRV is **1.96**. Feature `ewf` (default) routes `write_e01` through `ewf-image`.  
UI label remains **E01-lite** until the rows below are green. Details: [EWF-SPIKE.md](EWF-SPIKE.md).

| Check | Result |
|---|---|
| `cargo test -p trareon-core --lib format::ewf` | software green (unit) |
| `scripts/ewf-spike.sh` local write | [x] previously recorded |
| `ewfverify` (libewf) via feature `libewf-oracle` | [ ] pending human/lab install |
| Autopsy open EWF | [ ] pending |
| FTK open EWF | [ ] pending |
| UI label without `-lite` | **blocked** until above green |

## Generated artifact (fill when running smoke)

- Artifact path:
- `SHA256.txt` value:
- Core smoke command: `scripts/format-interop-smoke.sh`
- Core round-trip result:

## Decision

- [ ] Path A external checks recorded **or** Path B libewf+Autopsy/FTK green
- [ ] Any limitation added to the capability matrix
- [ ] Independent reviewer sign-off recorded

No unchecked template is evidence of interoperability. Product marketing must match this file.
