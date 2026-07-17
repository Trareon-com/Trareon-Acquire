# Decision request — Windows lab removable media allowlist

- **Day / Task:** Commercial Hari 8–9 / M2-P01
- **Gate status:** `HUMAN_APPROVAL` **REQUIRED**
- **Incident category:** none — first Windows narrow-storage lab media
- **Request:** Approve one named removable USB for Windows lab acquires only
- **Required inventory:** `docs/platform/windows-lab-inventory.md` filled on the Windows host
- **Software ready:** `lab_windows_bounded_smoke`, broker `PhysicalDrive0` hard-deny,
  `fixtures/lab-allowlists/windows-usb-template.json`
- **Forbidden until approval:** any `human_approved: true` Windows allowlist;
  any claim of Windows raw content PASS; imaging `PhysicalDrive0`
- **After approval:** copy the template to a media-specific allowlist, set
  `human_approved: true`, cite this decision, then run elevated open-only and
  bounded 1 MiB smoke only

## Acceptance options

| Option | Meaning |
|--------|---------|
| A | Approve the inventoried removable media for bounded Windows lab smoke |
| B | Defer — inventory incomplete or media is system/shared |
| C | Reject — do not allowlist |

## Evidence to attach when accepting

- Filled inventory table from `windows-lab-inventory.md`
- Exact `\\.\PhysicalDriveN` path and confirmation it is not system/boot
- Operator name / date of approval
