# Path A — RAW / `.fsnap` candidate interoperability track

> **Not court-certified.** Record Autopsy/FTK results in [EVIDENCE.md](EVIDENCE.md) before any court or production claim.

Trareon Acquire’s **candidate** RAW acquisition path today is:

1. Acquire to RAW bytes inside a `.fsnap` package (or plain RAW / split-RAW).
2. Seal with SHA-256 (optional SHA-512 sidecar).
3. Export CoC JSON + QR + custody JSONL.
4. Open the raw evidence stream in Autopsy / FTK Imager as a **raw/dd image**.

This path does **not** depend on EWF interoperability.

## Operator steps

```sh
# Software smoke (synthetic):
cargo test -p acquire-slint --lib --features gui foundation_demo

# Open the evidence file from the package (evidence/ relative path in manifest)
# in Autopsy → Add Data Source → Disk Image or VM File → select the raw file.
```

Record Autopsy/FTK versions and hash match in `EVIDENCE.md` under **Path A**.

## What is not claimed

- Full EnCase E01 / libewf compatibility (that is Path B — see `EWF-SPIKE.md`).
- AFF4 / VMDK / VHD / QCOW2 / DMG wrappers (removed from product surface).
