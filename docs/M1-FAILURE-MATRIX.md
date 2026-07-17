# M1 — Injected-failure / false-complete matrix

Result class: **Engineering Alpha — Lab Use Only** (file-backed).

This matrix records synthetic and injected-failure cases that must never
produce a false `VerifiedComplete` / invented success. Hosted CI tip after M1
exit: see `docs/M1-MILESTONE-REVIEW.md`.

| Case | Injection | Expected outcome | Test evidence |
|---|---|---|---|
| Pre-armed cancel | `cancel_flag=true` before first read | `CoreError::Cancelled`; audit may contain `cancelled`; never `acquired_unverified` / VerifiedComplete | `cancellation_stops_acquisition_without_false_complete`, `cancel_writes_incomplete_checkpoint_never_verified_complete`, property cancel tests, Tauri pre-armed cancel |
| Mid-run cancel (UI) | `cancel_foundation_demo` while demo runs | Demo returns cancel error; UI shows Cancelled | `apps/trareon-acquire` Cancel control + shared `AtomicBool` |
| Destination unwritable | Output parent is a file | Error; no evidence file created | `destination_write_failure_produces_no_false_complete` |
| Checkpoint claims complete | `incomplete: false` on disk | Load rejected | `checkpoint_claiming_complete_is_rejected` |
| Resume non-split | Partial RAW + checkpoint | Hash matches uninterrupted run; checkpoint cleared | `resume_after_cancel_matches_full_hash_and_clears_checkpoint` |
| Resume split-RAW | Partial segments + checkpoint | Hash matches full split run; checkpoint cleared | `resume_split_after_partial_segments_matches_full_hash` |
| Split settings mismatch | Checkpoint split size ≠ request | Verification error | `resume_split_mismatch_settings_is_rejected` |
| Mutated / truncated / removed / discontinuous / unsupported `.fsnap` | Golden invalid packages | Verifier + Analysis reject; no index written; package unmodified | `trareon-verifier` CLI goldens + `rejects_all_invalid_goldens_without_writing_index` |
| Index inside package | Analysis `--index-dir` under package | Rejected | `rejects_index_dir_inside_package` |
| State skip to VerifiedComplete | Domain transition | Rejected | `domain.rs` / property state matrix |

## Explicitly out of this matrix (M2+)

- Physical raw-device disconnect / bad-sector / destination-full on real media
- OS elevation / privileged broker execute
- Peak RSS / full `cargo-fuzz`
