# Windows unsigned install (Hari 33)

Result class: **Engineering Alpha / Founder unsigned**.

## Install from Community build

```powershell
cargo build -p acquire-slint --features gui --release --locked
# Binary: target\release\trareon-acquire.exe
.\target\release\trareon-acquire.exe
```

SmartScreen may warn “Unknown publisher”. Prefer checksum + source tag over
clicking through blindly.

## Self-test

```powershell
bash scripts/self-test.sh
# or: cargo test --workspace --locked --exclude acquire-slint
#     cargo test -p acquire-slint --features gui --locked
```

## Limitations

- Not Authenticode-signed; see [`UNSIGNED-LIMITATIONS.md`](UNSIGNED-LIMITATIONS.md).
- Raw `\\.\PhysicalDriveN` needs elevated admin and an approved allowlist —
  collected in [`../platform/WINDOWS-LAB-OPERATOR-PACK.md`](../platform/WINDOWS-LAB-OPERATOR-PACK.md).
- Never use `PhysicalDrive0`.
