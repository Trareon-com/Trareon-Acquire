# Foundation Demo

**Result class: Engineering Alpha.** This demo exercises the M0 foundation
slice (core acquisition, hashing, audit chain, `.fsnap` packaging, and
independent verification) end to end on a synthetic fixture. It is not a
production forensic acquisition tool and makes no claim about any real
device, OS, or hardware.

## Run the demo

```bash
dd if=/dev/zero of=/tmp/trareon-source.img bs=1M count=8
cargo test --workspace --locked --exclude acquire-slint
cargo test -p acquire-slint --features gui --locked
cargo run -p acquire-slint --features gui
cargo run -p trareon-verifier --locked -- verify /tmp/trareon-output/foundation.fsnap
```

In the running app, set the source path to `/tmp/trareon-source.img` and the
output directory to `/tmp/trareon-output`, confirm the synthetic-source
checkbox, and press Run. The UI reports `Verified Complete` only after the
Rust core has acquired the file, built the `.fsnap` package, and the
independent verifier has accepted it — the UI never decides success on its
own.

## Tamper demonstration

To see the independent verifier reject tampered evidence:

```bash
cp -r /tmp/trareon-output/foundation.fsnap /tmp/trareon-output/foundation-tampered.fsnap
echo "tampered" >> /tmp/trareon-output/foundation-tampered.fsnap/acquisitions/0001/evidence.raw
cargo run -p trareon-verifier --locked -- verify /tmp/trareon-output/foundation-tampered.fsnap
echo "exit code: $?"
```

Expected: the verifier prints `INVALID ...` and exits with code `2`.

## Scope

This demo covers RFC Track A/B foundation contracts on a file-backed source.
Raw-device adapters, privileged brokers, live/targeted/RAM collection, and
production release engineering are separate tracks with their own plans; see
`docs/IMPLEMENTATION-ROADMAP.md`.
