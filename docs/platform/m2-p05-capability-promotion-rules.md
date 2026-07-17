# M2-P05 — Capability matrix promotion rules

1. **Probe ≠ acquire.** Days 23–25 privilege probes cannot promote raw-device acquire to Implemented.
2. **Exact evidence only.** Updates must cite OS, arch, media, privilege, SHA of report, and test command.
3. **Allowlist required.** Block-device rows need `human_approved` allowlist reference.
4. **No umbrella language.** Never write “all Windows” / “all OS”.
5. **Fail closed.** Missing evidence → `NotValidated`, not Implied Pass.
6. **Analysis freeze.** Do not silently widen `.fsnap` v0.1 Analysis goldens when adding split packaging claims.

See `docs/CAPABILITY-MATRIX-M0.md`.
