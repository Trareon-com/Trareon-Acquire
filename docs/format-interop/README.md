# Format interoperability operator pack

`trareon.e01-lite/1` is a documented subset, not a claim of full EWF/libewf compatibility. The
smoke script creates synthetic bytes, writes an E01-lite file through the core example, performs a
local round trip, and records the reported SHA-256 in a sidecar evidence folder.

Run:

```sh
scripts/format-interop-smoke.sh
```

The script output is software evidence only. Complete these human checks before any
interoperability claim:

- [ ] Open the generated E01 in the exact Autopsy version, record version and result in `EVIDENCE.md`.
- [ ] Open it in the exact FTK/FTK Imager version, record version and result in `EVIDENCE.md`.
- [ ] Compare the external-tool extracted hash with `SHA256.txt`.
- [ ] Record any rejection as a compatibility limitation; do not alter the generated artifact.
- [ ] Independent reviewer validates the evidence record.

The output contains synthetic data and is not forensic evidence.
