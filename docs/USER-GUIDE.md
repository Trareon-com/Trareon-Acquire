# Trareon Acquire — User guide

Result class: **Engineering Alpha — Lab Use Only**. Builds are **UNSIGNED**.

For the full step-by-step (every nav surface + screenshots), use:

**→ [Operator tutorial](tutorials/OPERATOR-TUTORIAL.md)**

## Launch

```bash
export PATH="$HOME/.cargo/bin:$PATH"   # macOS: prefer rustup over Homebrew
cargo run -p acquire-slint --features gui
```

## Surfaces (summary)

| Nav | Job |
|---|---|
| Cases | Case + examiner (custody start) |
| Identify | ISO checklist + encryption probe |
| Acquire | Prepare → run (MiB/s, ETA, coverage) → seal |
| Tools | Verify / hash / compare / export |
| Triage | Read-only triage + Analysis lite |
| QMS | Self-test + known dataset |
| Boot | Portable pack dry-run plan |
| Help | SOP + deep-links; EN/ID; light/dark |

## Verify independently

```bash
cargo run -p trareon-verifier --locked -- verify /path/to/package.fsnap
```

## Limits

See [CAPABILITY-MATRIX-M0.md](CAPABILITY-MATRIX-M0.md) and [live-gate-checklist.md](live-gate-checklist.md). Do not claim production evidence or full E01 without evidence rows in [format-interop/EVIDENCE.md](format-interop/EVIDENCE.md).
