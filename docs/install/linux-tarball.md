# Linux tarball / binary install (Hari 35)

Result class: **Engineering Alpha / Founder unsigned**.

## Build

```bash
# Linux GUI deps: see .github/workflows/ci.yml (fontconfig, wayland, …)
cargo build -p acquire-slint --features gui --release --locked
./target/release/trareon-acquire
```

## Self-test

```bash
./scripts/self-test.sh
```

## Limitations

- No vendor package signature; verify checksum + commit.
- Loop/raw lab needs root/`disk` + allowlist — see
  [`../platform/linux-lab-inventory.md`](../platform/linux-lab-inventory.md).
- Never image the system disk (`nvme0n1` / `sda`).
