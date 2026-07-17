# Decision: Desktop shell = Slint; project license = GPLv3

Date: 2026-07-17
Status: **ACCEPTED** (operator decision)
Applies to: Trareon Acquire

## Decision

1. **Desktop UI** for Trareon Acquire is **Slint + Rust**, aligned with Trareon Lab (`C-SLINT`).
2. **Project license** is **GNU General Public License v3** (`GPL-3.0-only` in Cargo/SPDX).
3. The former **Tauri + Svelte** app (`apps/trareon-acquire`) is **archived**
   (not a workspace member; see `DEPRECATED.md`).

## Why

- One desktop stack across Trareon Acquire + Trareon Lab (packaging, CI deps, operator docs).
- No OS webview; forensic bytes stay in-process Rust (same security rationale as Lab Gate A).
- Removes long-term dependence on Tauri’s Linux GTK3/`glib` chain for the product UI.
- GPLv3 matches Slint’s GPL licensing path and keeps “sell binary + ship source” monetization honest.

## Monetization note (unchanged business model)

GPLv3 **allows selling** binaries. Recipients of binaries must be able to get corresponding source under GPLv3. Unsigned Founder builds remain the zero-cash launch path; signing stays post-revenue.

## Non-goals this decision does not claim

- Immediate deletion of every Tauri file in the same commit as the first Slint skeleton.
- Court / Official Production readiness.
- Changing Trareon Lab’s license (separate repo).

## Follow-ups

- [x] Replace `LICENSE` + workspace SPDX with GPLv3
- [x] Add `apps/acquire-slint` foundation shell
- [x] Wire Slint demo: path pickers, synthetic confirm, cancel_flag, verify display
- [x] Remove Tauri/Svelte from workspace + CI (archived under `apps/trareon-acquire/`)
- [x] Progressive Guided/Standard/Expert modes on Slint
- [x] Update commercial / user docs for “source with binary” obligation
