# Decision — M2 lab allowlist (tiny11 2311 / disk10)

- **Day / Task:** M2 lab media allowlist
- **Frozen SHA:** `0fe8a33` (pre-trial `main` tip)
- **Gate status:** `HUMAN_APPROVAL` **PROVIDED** for this named removable media only
- **Incident category:** none — operator designated path in chat: `/Volumes/tiny11 2311`
- **Decision:** Option A for **this media** — allowlist entries for `/dev/disk10`, `/dev/rdisk10`, `/dev/disk10s1`, and the mount point
- **Evidence:** `docs/platform/m2-lab-tiny11-2311-disk10.md`, `fixtures/lab-allowlists/tiny11-2311-disk10.json`
- **Still forbidden without further approval:** any other disk; system `disk0`; Official Production claims
- **Superseded for full-disk of this media only:** imaging the whole USB is now gated by `2026-07-17-m2-fulldisk-rdisk10-to-untitled.md` (destination `/Volumes/Untitled`)

- **Actions still not performed:** privileged helper install; FDA claims; Official Production classification

Supersedes the open request in `2026-07-17-m2-lab-allowlist-required.md` **only** for this device identity.
