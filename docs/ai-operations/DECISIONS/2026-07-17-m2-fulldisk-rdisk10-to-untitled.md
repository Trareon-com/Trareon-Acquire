# Decision — M2 full-disk acquire tiny11 `rdisk10` → `/Volumes/Untitled`

- **Day / Task:** M2 lab full-disk (operator-gated)
- **Gate status:** `HUMAN_APPROVAL` **PROVIDED** (chat 2026-07-17: “fulldisk ke untitled aja”)
- **Source:** `/dev/rdisk10` (allowlisted tiny11 2311 / Sandisk USB, **not** system `disk0`)
- **Declared size:** `61524148224` bytes (~57.3 GiB) from `diskutil info disk10`
- **Destination:** `/Volumes/Untitled/trareon-lab/` on `disk11` (ExFAT USB, separate media, ~221+ GiB free)
- **Forbidden still:** system disk; any other unapproved media; Official Production / Lab Beta exit claims from this run alone
- **Operator preference:** unmount `/Volumes/tiny11 2311` before acquire when possible
- **Evidence (after run):** update `docs/platform/m2-lab-tiny11-2311-disk10.md` with SHA + verifier

Related: `2026-07-17-m2-lab-tiny11-disk10-approved.md` (media allowlist); this decision **extends** that gate to full-disk **content** imaging of the same device only.
