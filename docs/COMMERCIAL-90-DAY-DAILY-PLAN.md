# Commercial launch — 90-day daily plan

Start: **2026-07-18** (Hari 1) · End: **2026-10-15** (Hari 90)
Scope: [`COMMERCIAL-V1-SCOPE.md`](COMMERCIAL-V1-SCOPE.md) · Cash: [`ZERO-CASH-LAUNCH-PLAN.md`](ZERO-CASH-LAUNCH-PLAN.md)
Progress tracker: [`COMMERCIAL-LAUNCH-STATUS.md`](COMMERCIAL-LAUNCH-STATUS.md)

Legend: `[x]` done · `[~]` in progress · `[ ]` pending · `[W]` deferred to Windows pack · `[H]` needs human/business gate

Windows hardware work is collected in [`platform/WINDOWS-LAB-OPERATOR-PACK.md`](platform/WINDOWS-LAB-OPERATOR-PACK.md).

---

## Hari 1–30 — Core lintas platform

| Hari | Tanggal | Tugas | Done |
|------|---------|-------|------|
| 1 | 18 Jul | Freeze `COMMERCIAL-V1-SCOPE.md`; buat gap audit | [x] |
| 2 | 19 Jul | Gap audit Windows/Linux/macOS blockers | [x] |
| 3 | 20 Jul | Operator: unmount tiny11 → smoke `disk10s1` | [x] |
| 4 | 21 Jul | Broker CI tests `PhysicalDrive0` + shell deny | [x] |
| 5 | 22 Jul | `macos-elevation-operator-guide.md` | [x] |
| 6 | 23 Jul | Windows bounded smoke example + allowlist template | [x] |
| 7 | 24 Jul | Review minggu 1; merge; update matrix | [x] |
| 8 | 25 Jul | Windows lab inventory + allowlist decision | [W] |
| 9 | 26 Jul | Allowlist JSON Windows USB | [W] |
| 10 | 27 Jul | Windows elevated open probe non-system drive | [W] |
| 11 | 28 Jul | Windows bounded 1 MiB + verifier | [W] |
| 12 | 29 Jul | Windows full-disk USB → staging disk | [W] |
| 13 | 30 Jul | Windows split-RAW + resume | [W] |
| 14 | 31 Jul | `windows-lab-acquire.md` report | [W] |
| 15 | 1 Agu | Linux lab inventory | [x] |
| 16 | 2 Agu | Linux loop RO attach + allowlist | [~] software prep; attach [H] |
| 17 | 3 Agu | Linux bounded acquire + verify | [~] example ready; lab [H] |
| 18 | 4 Agu | Linux fault dest-full | [x] portable suite |
| 19 | 5 Agu | Linux fault detach mid-read | [H] physical lab |
| 20 | 6 Agu | `linux-loop-lab.md` | [x] scaffold |
| 21 | 7 Agu | macOS cancel mid-acquire raw | [H] elevated lab |
| 22 | 8 Agu | Windows cancel mid-acquire | [W] |
| 23 | 9 Agu | Linux cancel mid-acquire | [H] |
| 24 | 10 Agu | UI cancel → `cancel_flag` E2E | [x] |
| 25 | 11 Agu | UI verifier result display | [x] |
| 26 | 12 Agu | UI allowlist + elevation preflight | [x] |
| 27 | 13 Agu | CI regression 3 OS | [x] |
| 28 | 14 Agu | Capability matrix 3 OS evidence | [x] updated; Windows pending pack |
| 29 | 15 Agu | Unsigned install limitation docs | [x] |
| 30 | 16 Agu | Gate: 3 OS demo video | [H] |

## Hari 31–60 — Waitlist & preorder

| Hari | Tanggal | Tugas | Done |
|------|---------|-------|------|
| 31 | 17 Agu | Feature freeze v1 | [x] software freeze candidate doc |
| 32 | 18 Agu | Landing/README commercial copy | [x] |
| 33 | 19 Agu | User guide Windows unsigned | [x] |
| 34 | 20 Agu | User guide macOS unsigned | [x] |
| 35 | 21 Agu | User guide Linux tarball | [x] |
| 36 | 22 Agu | Test `Build It For Me` 3 platforms | [x] how-to; hosted run [H] |
| 37 | 23 Agu | Artifact checksum + COMMUNITY notice | [x] founder-build script |
| 38 | 24 Agu | One-command self-test script | [x] |
| 39 | 25 Agu | **Gate 1:** open waitlist | [H] draft ready |
| 40 | 26 Agu | Technical article #1 tamper demo | [x] draft |
| 41 | 27 Agu | Article #2 failure matrix | [x] draft |
| 42 | 28 Agu | Outreach 10 prospects | [H] |
| 43 | 29 Agu | Collect 3 pain points | [H] |
| 44 | 30 Agu | Prioritize bugs from feedback | [H] |
| 45 | 31 Agu | Fix P0 feedback | [H] |
| 46 | 1 Sep | Guided mode polish | [x] |
| 47 | 2 Sep | Standard mode USB path | [x] |
| 48 | 3 Sep | Expert mode raw warnings | [x] |
| 49 | 4 Sep | About limitations screen | [x] |
| 50 | 5 Sep | Founder legal/refund draft | [x] |
| 51 | 6 Sep | Payment channel (zero setup fee) | [H] |
| 52 | 7 Sep | Founder page Rp199k + unsigned disclosure | [x] draft (open [H]) |
| 53 | 8 Sep | **Gate 2:** Founder preorder open | [H] |
| 54 | 9 Sep | Fulfillment ledger | [x] template |
| 55 | 10 Sep | Unsigned build script per OS | [x] |
| 56 | 11 Sep | RC `v0.9.0-founder` internal | [H] |
| 57 | 12 Sep | Fix preorder blockers | [H] |
| 58 | 13 Sep | CoC/report export minimal | [x] UI JSON export |
| 59 | 14 Sep | Analysis golden smoke | [x] covered by self-test/goldens |
| 60 | 15 Sep | 10+ preorder or pivot decision | [H] |

## Hari 61–90 — Deliver & harden

| Hari | Tanggal | Tugas | Done |
|------|---------|-------|------|
| 61 | 16 Sep | Ship Founder batch 1 | [H] |
| 62 | 17 Sep | Assisted install docs/calls | [x] install guides |
| 63 | 18 Sep | Support triage P0 | [H] |
| 64 | 19 Sep | Windows USB media #2 | [W] |
| 65 | 20 Sep | macOS media #2 | [H] |
| 66 | 21 Sep | Linux Ubuntu HW if available | [H] |
| 67 | 22 Sep | Resume after reboot (1 OS) | [H] |
| 68 | 23 Sep | Destination-full HW case | [H] |
| 69 | 24 Sep | Re-plug source identity | [H] |
| 70 | 25 Sep | Performance sample doc | [x] Day 26 baseline exists |
| 71 | 26 Sep | Property/fuzz full pass | [~] bounded suite; full fuzz later |
| 72 | 27 Sep | Security audit disposition | [x] summary doc |
| 73 | 28 Sep | SBOM + checksum manifest | [x] founder-build checksums; full SBOM later |
| 74 | 29 Sep | `v1.0.0-founder.2` bugfix | [H] |
| 75 | 30 Sep | Matrix update | [x] this pass |
| 76 | 1 Okt | Independent review #1 | [H] |
| 77 | 2 Okt | Triage review | [H] |
| 78 | 3 Okt | Failure-state screenshots | [H] |
| 79 | 4 Okt | Buyer training 1-pager | [x] |
| 80 | 5 Okt | Gate 3 cash vs signing decision | [H] |
| 81 | 6 Okt | Public release notes v1.0 | [x] draft |
| 82 | 7 Okt | Offline Founder binary → Lynk.id/Gumroad (not GH Release) | [H] |
| 83 | 8 Okt | Individual Rp299k tier | [H] |
| 84 | 9 Okt | Launch article | [H] |
| 85 | 10 Okt | 48h support monitor | [H] |
| 86 | 11 Okt | Hotfix v1.0.1 if needed | [H] |
| 87 | 12 Okt | Intel Mac spike or NotValidated | [H] |
| 88 | 13 Okt | Win11 24H2 smoke if different | [W] |
| 89 | 14 Okt | Conversion retrospective | [H] |
| 90 | 15 Okt | Commercial v1.0 sign-off | [H] |

## Daily habit (every day)

- [x] Prefer `./scripts/self-test.sh` (or CI equivalent)
- [x] One line in session log or `COMMERCIAL-LAUNCH-STATUS.md`
- [x] No evidence bytes committed to git
