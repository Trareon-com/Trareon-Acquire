# macOS unsigned install (Hari 34)

Result class: **Engineering Alpha / Founder unsigned**.

## Install from Community build

```bash
# From a workflow artifact or local release build:
cargo build -p acquire-slint --features gui --release --locked
# Binary: target/release/trareon-acquire
xattr -d com.apple.quarantine ./trareon-acquire 2>/dev/null || true
./trareon-acquire
```

If Gatekeeper blocks: Finder → right-click → Open → Open.

## Self-test

```bash
./scripts/self-test.sh
```

## Limitations

- Not notarized; see [`UNSIGNED-LIMITATIONS.md`](UNSIGNED-LIMITATIONS.md).
- Raw-device paths need elevation (`sudo` / `operator` group) — see
  [`../platform/macos-elevation-operator-guide.md`](../platform/macos-elevation-operator-guide.md).
- Synthetic / allowlisted lab media only until matrix says otherwise.
