# Trareon Acquire Implementation Roadmap

> **HISTORICAL / pre-Slint in places.** Product UI is `apps/acquire-slint`. Current capability: [CAPABILITY-MATRIX-M0.md](CAPABILITY-MATRIX-M0.md). Nav: [INDEX.md](INDEX.md).

## Status dan tujuan

- **Baseline:** RFC Trareon Acquire v1.0, 17 Juli 2026
- **Tujuan:** Mengubah arsitektur menjadi produk forensik yang dapat diuji tanpa mengklaim capability sebelum validation gate lulus.
- **Model kerja:** TDD, vertical slices, fail-closed DevSecOps, synthetic fixtures lebih dahulu, kemudian perangkat nyata.
- **Batas satu minggu:** Demonstrable foundation slice, bukan aplikasi production forensic.

RFC memuat beberapa subsystem independen. Karena itu implementasi tidak dijalankan sebagai satu pekerjaan besar. Setiap track menghasilkan software yang dapat diuji dan ditolak atau diterima secara mandiri.

## Definisi hasil

| Kelas hasil | Makna | Boleh digunakan pada barang bukti nyata? |
|---|---|---:|
| Prototype | UI dan workflow dapat didemonstrasikan dengan fixture | Tidak |
| Engineering Alpha | Core bekerja pada synthetic/file-backed source dan failure injection | Tidak |
| Lab Beta | Bekerja pada allowlisted lab devices dengan limitation matrix | Hanya pengujian lab |
| Release Candidate | Cross-platform gate, documentation, provenance, dan validation candidate selesai | Hanya validation protocol |
| Official Production | Release gate RFC lulus dan validation statement diterbitkan | Sesuai capability matrix |

Status tidak ditentukan oleh tanggal atau banyaknya fitur. Status ditentukan oleh evidence hasil test.

## Scope release pertama

Release pertama hanya memuat:

- case identity dan operator authorization record;
- preflight serta source/destination confirmation;
- RAW dan split-RAW imaging;
- streaming SHA-256 dan independent post-write verification;
- bad-sector, retry, padding, coverage, dan gap records;
- append-only hash-chained audit journal;
- `.fsnap` package baseline;
- independent CLI verifier;
- Chain of Custody dan report;
- Guided, Standard, Expert, dan Training modes;
- Windows, macOS, dan Linux capability reporting.

Release pertama tidak memuat custom driver, production E01/AFF4 writer, full RAM parity, mobile, cloud, boot environment, carving, filesystem interpretation, atau analysis. Batas ini tidak menghapus fitur dari roadmap; batas ini menjaga validitas dan waktu menuju pengguna pertama.

## Workstream dan urutan

Strategi platform adalah **portable core sejak hari pertama, platform implementation secara berurutan**. CI melakukan compile dan fixture tests pada tiga OS sejak M0, tetapi raw-device adapter dan hardware validation tidak dipaksakan selesai bersamaan.

Urutan engineering: foundation portable, feasibility spike read-only pada tiga OS, Linux reference adapter, Windows production adapter, lalu macOS production adapter. Windows tetap menjadi kandidat artifact komersial pertama; urutan engineering tidak menentukan urutan pemasaran. Detail pembagian perangkat, agent, branch, review, dan security gate berada di `docs/AI-DEVELOPMENT-WORKFLOW.md`.

### Track A — Foundation and evidence semantics

**Deliverables:** repository, DevSecOps, domain types, state machine, audit journal, `.fsnap` schema, independent verifier, fixtures.

**Entry:** RFC v1.0 accepted.

**Exit:** Fixture acquisition menghasilkan package yang dapat diverifikasi oleh proses terpisah; perubahan, file hilang, audit discontinuity, dan path traversal terdeteksi.

### Track B — Storage acquisition engine

**Deliverables:** streaming pipeline, RAW/split output, hashing, verification, coverage map, bad-sector policy, checkpoint/resume, cancellation, performance limits.

**Entry:** Track A contracts stabil.

**Exit:** Golden images, disconnect, destination-full, power-loss simulation, retry, padding, dan resume tests lulus.

### Track C — Platform adapters and privilege

**Deliverables:** source enumeration, stable device identity, destination qualification, privileged broker, Windows/macOS/Linux adapters.

**Entry:** File-backed engine dan negative tests lulus.

**Exit:** Main UI tidak elevated; broker command surface diautentikasi; allowlisted lab device tests lulus per OS.

### Track D — Desktop workflow and documentation

**Deliverables:** Tauri 2/Svelte UI, progressive disclosure, in-app guidance, documentation bundle, accessibility, localization, report preview.

**Entry:** Domain/API contracts stabil.

**Exit:** End-to-end guided acquisition pada synthetic source lulus tanpa UI menentukan final status.

### Track E — Validation and production release

**Deliverables:** validation protocol, hardware/OS matrix, reproducible builds, SBOM, provenance, signed/notarized artifacts, limitation statement.

**Entry:** Alpha feature freeze.

**Exit:** Official Production gate RFC lulus untuk setiap capability yang diumumkan.

### Track F — Live, volatile, and extended acquisition

**Deliverables:** targeted profiles, system snapshot, PCAPNG, RAM methods per platform, optional mobile/cloud adapters.

**Entry:** Storage MVP production boundary terbukti.

**Exit:** Setiap metode memiliki tool-footprint record, consistency class, capability matrix, dan independent validation evidence.

### Track G — Formats, boot, and ecosystem

**Deliverables:** EWF/E01, Ex01/EWF2, AFF4 interoperability, boot environment, proficiency-test support, Trareon Analysis import contract.

**Entry:** Release format library dan validation resources tersedia.

**Exit:** Conformance/interoperability tests lulus dan tidak ada custom subset yang diklaim sebagai standard format.

## Estimasi realistis

Asumsi: satu pembuat bekerja penuh waktu dengan bantuan AI, review disiplin, akses ke minimal satu perangkat setiap OS, dan tidak ada penundaan eksternal signing.

| Waktu | Hasil realistis |
|---|---|
| 5–7 hari | Foundation vertical slice: synthetic acquisition, hash, manifest, verifier, minimal UI, CI |
| 2–4 minggu | Engineering Alpha yang stabil pada file-backed/virtual sources |
| 8–16 minggu | Storage-acquisition Lab Beta lintas OS dengan dokumentasi dan failure tests |
| 4–9 bulan | Credible signed storage MVP, bergantung signing, hardware, dan validation |
| 9–18 bulan | Live/targeted/network/RAM capability yang bertahap dan tervalidasi |
| 18–36 bulan | Maturity, interoperability luas, boot media, external review, dan ecosystem |

AI dapat mempercepat scaffolding, code generation, test cases, documentation, dan mechanical review. AI tidak dapat menggantikan driver signing, OS behavior, destructive hardware tests, independent validation, legal review, atau pengalaman lapangan.

## Milestone gates

### M0 — Week-one foundation

- Workspace dan CI lintas OS terbentuk.
- Core tidak bergantung pada Tauri.
- Synthetic fixture berhasil diakuisisi ke RAW dengan SHA-256.
- `.fsnap` minimal dapat diverifikasi CLI terpisah.
- Tamper test gagal dengan reason yang benar.
- UI hanya memanggil typed command dan menampilkan core state.

### M1 — Engineering Alpha

- State machine, audit chain, manifest canonicalization, package safety, dan error contract stabil.
- Property/fuzz tests berjalan.
- Pause/cancel/failure injection tidak pernah menghasilkan complete palsu.
- Documentation mapping mulai diberlakukan.

### M2 — Storage Lab Beta

- Raw devices hanya pada allowlisted lab media.
- Stable source identity dan source/destination reversal protection bekerja.
- Bad-sector, disconnect, destination-full, split, resume, dan post-verification lulus.
- Capability dan limitation matrix diterbitkan.

### M3 — Release Candidate

- Reproducible build comparison, SBOM, attestation, security review, accessibility, dan documentation gate lulus.
- Privileged broker direview terpisah.
- Validation report tidak membuat klaim di luar evidence.

### M4 — Official Production

- Signing/notarization tersedia untuk artifact terkait.
- Two-person sign-off dilakukan; bila hanya ada satu maintainer, reviewer eksternal/sponsor teknis mengisi reviewer kedua.
- Official build, source revision, provenance, validation status, dan support period dapat diperiksa dari aplikasi.

## Rencana turunan wajib

Rencana detail dibuat tepat sebelum track dikerjakan agar tidak membekukan asumsi platform yang belum diuji:

1. Foundation vertical slice — tersedia di `docs/superpowers/plans/2026-07-17-trareon-acquire-foundation.md`.
2. `.fsnap` schema and cryptographic profile.
3. Storage streaming and interruption recovery.
4. Windows adapter and broker feasibility.
5. macOS adapter and helper feasibility.
6. Linux adapter and portable packaging.
7. Desktop UX and documentation system.
8. Validation protocol and release engineering.
9. Live/targeted/network collection.
10. RAM per-platform capability plans.

Prosedur Code dan Review lintas tool tersedia di `docs/AI-DEVELOPMENT-WORKFLOW.md`. Satu task hanya memiliki satu author aktif pada satu worktree; reviewer utama harus independen dari author.

## Scope-control rules

- Satu milestone tidak menerima fitur dari milestone setelahnya kecuali diperlukan untuk membuktikan interface.
- Fitur tanpa test oracle tidak masuk acquisition core.
- Parser kompleks tidak ditulis sendiri bila mature compatible library tersedia.
- UI tidak pernah menjadi sumber kebenaran final status.
- Tidak ada `VerifiedComplete` sebelum independent verification dan coverage evaluation selesai.
- Tidak ada klaim “semua OS” tanpa menyebut exact OS, architecture, security state, dan capability.
- Tidak ada deadline yang mengubah failed test menjadi accepted risk secara diam-diam.

## Keputusan waktu

Target satu minggu diterima hanya untuk M0. Target satu minggu ditolak untuk MVP forensik production. Memaksakan seluruh RFC selesai dalam satu minggu akan menghasilkan demo besar tetapi tidak defensible, dan bertentangan langsung dengan prioritas RFC: validitas lebih tinggi daripada luas fitur dan kecepatan rilis.
