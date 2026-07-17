# Decision Request — M2 lab media allowlist

- **Day / Task:** M2 Storage Lab Beta entry gate
- **Frozen SHA:** `a037e01` (M1 exit tip on `main` at request time)
- **Gate status:** `HUMAN_APPROVAL_REQUIRED`
- **Incident category:** `HUMAN-APPROVAL-REQUIRED` (control-plane hard stop: raw device / privilege / destructive lab)
- **RFC/spec section affected:** Storage Lab Beta / privileged broker / platform adapters
- **Observed evidence:**
  - M1 file-backed Engineering Alpha exit complete (`docs/M1-MILESTONE-REVIEW.md`)
  - Days 23–25 privilege probes only — raw *acquisition* remains `NotValidated`
  - Software prep: lab allowlist schema `trareon.lab-allowlist/1`, system-disk hard-deny, synthetic fault suite
- **Conflicting evidence or ambiguity:** Operator asked to continue “until finished”; full M2 exit requires physical allowlisted media and privilege that this agent must not invent
- **Safe default if no decision:** Remain on file-backed Engineering Alpha; do not open raw devices; do not elevate; do not claim Lab Beta
- **Option A — validity/performance/schedule impact:** Approve named removable lab media + write-block process + inventory → unlock M2-P01/P02 hardware execution (validity ↑, schedule advances)
- **Option B — validity/performance/schedule impact:** Defer hardware; keep software-only prep (validity unchanged, Lab Beta exit blocked)
- **Recommended option and reason:** Option A when operator has spare removable media; otherwise Option B
- **Exact authority or command requested:** Human must publish an allowlist JSON with `human_approved: true`, `approved_by: <name>`, and exact lab media path/identity digests — **never** system disks (`PhysicalDrive0`, `/dev/rdisk0`, `/dev/nvme0n1`, `/dev/sda`)
- **Actions explicitly not performed:** Raw-device open/read/write; UAC/sudo elevation; privileged helper install; FDA grant claims; Official Production classification

AI tidak boleh menjalankan akuisisi raw-device sampai keputusan manusia dicatat di sini atau di `docs/ai-operations/DECISIONS/`.
