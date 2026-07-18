# EWF adoption spike (Wave 3a)

Date: 2026-07-18  
Workspace MSRV: Rust 1.95  
Decision owner: software (oracle + Autopsy/FTK still human)

## Candidates

| Crate | License | Write | Notes |
|---|---|---|---|
| [`ewf-image` 0.2.0](https://crates.io/crates/ewf-image) | Apache-2.0 | Yes (`EwfWriter`) | Pure Rust EWF1/EWF2; **requires rustc 1.96** |
| `ewf-rs` (acquiredsec) | MIT | — | Not published on crates.io (2026-07-18) |
| libewf | LGPL-3.0 | via FFI | Oracle verify only (`ewfverify`) |

## Spike result

1. `ewf-image` compiles on this machine with `cargo check --ignore-rust-version`.
2. Default `cargo check` **fails** under workspace MSRV 1.95 (`ewf-image` declares 1.96).
3. `scripts/ewf-spike.sh` produced a valid `spike.Ex01` segment (4 MiB raw → ~4 MiB Ex01) on 2026-07-18.
4. Therefore **Path B is not merged into `trareon-core` yet** — UI remains `E01-lite`.
5. **Winner (pending MSRV bump):** `ewf-image` (Apache-2.0, no FFI).

## Reproduce locally

```sh
scripts/ewf-spike.sh
```

Optional when `ewfinfo` / `ewfverify` are installed:

```sh
ewfinfo "$OUT/*.Ex01"
ewfverify "$OUT/*.Ex01"
```

## Gate before UI label `E01` (no `-lite`)

- [ ] Workspace `rust-version` ≥ 1.96 (or vendored compatible fork)
- [ ] `ewf-image` writer wired behind feature `ewf-image`
- [ ] libewf `ewfverify` green on CI or lab host
- [ ] Autopsy/FTK rows filled in `EVIDENCE.md`
- [ ] Capability matrix updated

Until then: market **Path A** (RAW / `.fsnap`) as the court-usable track.
