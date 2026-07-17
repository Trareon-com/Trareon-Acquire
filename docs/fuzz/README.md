# Fuzz / property corpus (Day 22)

Result class: **Engineering Alpha** foundation slice.

This directory documents the bounded property/fuzz baseline for
`trareon-core` and `trareon-verifier`. It does **not** claim production
libFuzzer coverage.

## What runs in CI today

Hosted CI runs the bounded property suite:

```bash
cargo test -p trareon-core --test properties --locked
```

That suite is also included in `cargo test --workspace --locked`.

## Resource bounds (recorded)

| Surface | Bound used by Day 22 smoke |
|---|---|
| Acquisition state matrix | 8×8 = 64 transitions |
| Audit chain length | 1..=64 events |
| Evidence sizes | {1, 16, 256, 1024, 8192} bytes (empty source rejected by core) |
| Path corpus | ≤ 32 seeds in `fixtures/fuzz-corpus/v0/` |
| Hostile verifier inputs | fixed seed set (missing/empty/garbage) |

Foundation audit journal hard limit remains `MAX_FOUNDATION_EVENTS`
(100_000) in `crates/trareon-core/src/audit.rs` — property smoke stays
far below that.

## Corpus layout

```
fixtures/fuzz-corpus/v0/
  unsafe-relative-paths.txt   # one seed per line; `#` comments allowed
```

Seeds are synthetic only. Do not add real evidence, case data, secrets,
or raw-device captures.

## Reproducer rule

If a property/fuzz case fails:

1. Keep the exact seed input that failed (path string, bytes, or package tree).
2. Add a regression assertion under `crates/trareon-core/tests/` (or extend
   `properties.rs`) before changing production code.
3. Do not "fix" by skipping, weakening, or deleting the failing case.

## NotValidated / deferred

Full `cargo-fuzz` / libFuzzer targets and a long-running CI fuzz job are
**NotValidated** in this slice. Adding those tools is a dependency/toolchain
change that needs explicit review (Day 22 author prompt: no new dependency
without review). When enabled later, place targets under a dedicated
`fuzz/` workspace member and keep corpus seeds synthetic.
