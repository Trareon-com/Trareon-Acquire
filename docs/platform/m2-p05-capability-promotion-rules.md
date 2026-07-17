# M2-P05 — Capability matrix promotion rules

1. **Probe ≠ acquire.** Days 23–25 privilege probes cannot promote raw-device acquire to Implemented.
2. **Bounded ≠ full-disk.** A `max_bytes` lab sample (e.g. 1 MiB) may claim only “bounded raw sample”; never promote to full-media acquire or Lab Beta exit.
3. **Exact evidence only.** Updates must cite OS, arch, media, privilege, SHA of report, and test command.
4. **Allowlist required.** Block-device rows need `human_approved` allowlist reference.
5. **No umbrella language.** Never write “all Windows” / “all OS”.
6. **Fail closed.** Missing evidence → `NotValidated`, not Implied Pass.
7. **Analysis freeze.** Do not silently widen `.fsnap` v0.1 Analysis goldens when adding split packaging claims.

See `docs/CAPABILITY-MATRIX-M0.md`.
