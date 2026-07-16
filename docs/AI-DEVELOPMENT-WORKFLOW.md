# Trareon Acquire AI-Assisted Development Workflow

## Status dan tujuan

- **Status:** Prosedur implementasi turunan RFC v1.0.
- **Berlaku untuk:** Claude Code, Codex, Antigravity, contributor manusia, dan automation yang mengubah repository.
- **Tujuan:** Mempercepat implementasi tanpa membuat tiga AI mengubah area yang sama, tanpa menurunkan validitas forensik, dan tanpa mengklaim dukungan OS sebelum diuji.

Dokumen ini tidak menggantikan RFC. Urutan kebenaran adalah RFC, acceptance criteria task, test oracle, kemudian implementasi. Saran AI yang bertentangan dengan urutan tersebut harus ditolak.

## Keputusan platform

Trareon Acquire memakai strategi **cross-platform by architecture, sequential by hardware implementation**:

1. Shared Rust core, schemas, verifier, dan TypeScript contracts harus portable sejak commit pertama.
2. Compile, unit test, lint, dan package smoke dijalankan pada Windows, macOS, dan Linux sejak M0 sejauh runner memungkinkan.
3. Raw-device adapter, privileged helper, dan destructive hardware validation dikerjakan satu platform pada satu waktu.
4. Capability hanya diumumkan untuk kombinasi OS, architecture, privilege state, dan hardware yang sudah melewati validation gate.

Urutan engineering yang dipilih:

1. **M0 — portable foundation:** file-backed source, synthetic fixtures, shared core, verifier, UI shell, dan CI tiga OS.
2. **Feasibility spikes — tiga OS:** enumerasi disk read-only, stable identity candidate, privilege boundary, dan capability report. Spike tidak menjadi production adapter.
3. **Linux reference adapter:** kontrak raw-device dan failure injection dibentuk pada environment yang paling transparan dan mudah diotomasi.
4. **Windows production adapter:** menjadi prioritas produk komersial pertama setelah kontrak reference stabil; feasibility-nya dimulai sejak awal agar risiko privilege/API tidak terlambat ditemukan.
5. **macOS production adapter:** core dan UI tetap diuji setiap saat, tetapi helper/raw-device capability diselesaikan setelah Linux dan Windows.

Urutan engineering bukan urutan pemasaran. Windows dapat menjadi artifact siap-pakai pertama yang dijual meskipun Linux dipakai sebagai reference implementation.

## Fungsi perangkat yang tersedia

| Perangkat | Peran utama | Cocok untuk | Tidak cukup untuk |
|---|---|---|---|
| MacBook M4 Pro | Development control plane | Rust core, Tauri/UI, docs, tests, macOS arm64, orchestration | macOS Intel, Windows x64 raw-device validation |
| ThinkPad X270 | Windows compatibility and negative-test lab | USB/raw-media lab, privilege behavior, lower-spec performance, Windows x64 smoke | Menjadi satu-satunya mesin sertifikasi Windows modern sebelum CPU dan dukungan OS-nya diverifikasi |
| Mesin Kali Linux | Linux engineering and adversarial lab | `/dev` behavior, permissions, loop devices, disconnect/fault tests, security testing | Menjadi baseline distribusi stabil sendirian karena Kali adalah rolling distribution |

Untuk baseline Linux, gunakan Ubuntu/Debian LTS pada VM, external boot, atau partisi lab tanpa biaya lisensi. Kali tetap dipakai sebagai compatibility dan adversarial environment. VM tidak menggantikan pengujian perangkat fisik.

Semua destructive test hanya memakai media lab yang sengaja dikorbankan dan sudah di-allowlist. System disk, disk kerja, dan barang bukti nyata tidak pernah menjadi fixture pengembangan.

## Pembagian peran AI

Pembagian awal berikut menjaga independensi. Peran boleh ditukar per task, tetapi author dan reviewer utama tidak boleh menjadi agen yang sama.

| Peran | Default tool | Tanggung jawab |
|---|---|---|
| Spec guardian dan orchestrator | Codex | Memecah plan menjadi task, menjaga RFC mapping, review arsitektur/security/forensic semantics, memverifikasi hasil |
| Primary implementer | Claude Code atau Codex yang bukan reviewer | Mengerjakan satu bounded task dengan TDD dalam worktree terisolasi |
| UI and exploratory validator | Antigravity | Visual/UI flow, browser interaction, accessibility exploration, screenshot/artifact, dan exploratory test; bukan penentu validitas evidence |
| Final authority | Test gates dan manusia | CI, fixture oracle, hardware protocol, serta approval manusia untuk perubahan berisiko tinggi |

Antigravity dapat menjadi author UI pada worktree sendiri. Codex dapat menjadi author Rust core bila Claude menjadi reviewer. Nama alat bukan bukti kualitas; independensi review dan hasil test yang menentukan.

## Aturan satu task

Setiap task wajib memiliki satu task packet:

- ID dan judul, misalnya `M0-T01`;
- RFC requirement atau section yang dipenuhi;
- scope dan explicit non-goals;
- file/API yang boleh berubah;
- acceptance criteria yang dapat diuji;
- perintah verification;
- risk class;
- owner/author dan reviewer yang berbeda.

Gunakan `Template/Desktop/Task-Desktop.md` sebagai bentuk canonical task packet.

Satu task hanya memiliki satu author aktif. AI lain boleh melakukan review read-only terhadap commit atau diff yang stabil, bukan ikut mengedit worktree author.

## Branch, worktree, dan commit discipline

- `main` tidak diedit langsung.
- Satu task memakai satu branch: `feat/m0-t01-short-name`, `fix/...`, atau `docs/...`.
- Satu author memakai satu worktree terisolasi.
- Jangan menjalankan Claude Code, Codex, dan Antigravity dengan akses tulis ke worktree yang sama.
- Reviewer membaca commit hash yang pasti. Bila reviewer harus memperbaiki kode, buat task/branch fix baru atau kembalikan temuan ke author.
- Commit harus kecil, buildable bila praktis, dan mengacu pada task ID.
- Generated files, dependency changes, dan source changes dipisahkan bila itu membuat review lebih jelas.
- PR mencatat alat AI yang membantu, test yang benar-benar dijalankan, limitation, dan bagian yang belum diverifikasi.

Tool-specific instruction files, bila diperlukan, hanya menjadi pointer tipis ke aturan canonical repository. Jangan memelihara tiga versi arsitektur yang bisa berbeda.

## Siklus Code dan Review

1. **Prepare:** orchestrator membuat task packet dan menentukan risk class.
2. **Plan check:** author menjelaskan perubahan minimum dan test oracle sebelum menulis implementation.
3. **Code:** author bekerja dengan test-first pada worktree sendiri.
4. **Self-check:** author menjalankan formatting, lint, targeted tests, dan secret/dependency checks yang relevan.
5. **Handoff:** author memberikan commit hash, changed files, commands/results, limitation, dan open questions.
6. **Independent review:** reviewer lain memeriksa RFC compliance, correctness, error paths, security, dan test adequacy dengan `Template/Desktop/Review-Desktop.md`.
7. **Fix loop:** temuan kembali ke author; reviewer memeriksa commit baru, bukan review yang sudah basi.
8. **CI:** matrix test harus hijau. Hosted CI tidak dianggap sebagai bukti raw-device behavior.
9. **Platform smoke:** jalankan test pada perangkat/OS yang disentuh task.
10. **Human gate:** manusia menyetujui perubahan high-risk sebelum merge.
11. **Merge:** hanya setelah acceptance criteria memiliki evidence dan limitation dicatat.

Review AI adalah lapisan tambahan, bukan pengganti human sign-off atau protocol validation.

## Tingkat review berdasarkan risiko

| Risk | Contoh | Gate minimum |
|---|---|---|
| Low | Dokumentasi, copy UI, styling tanpa state semantics | Satu author, targeted check, satu review |
| Medium | Typed command, manifest field non-critical, state presentation | Satu author, reviewer AI berbeda, CI tiga OS yang relevan |
| High | Raw device, privilege broker, hashing, canonicalization, audit chain, completion state, recovery | Satu author, dua sudut review independen, hardware/failure tests, approval manusia |
| Release-critical | Signing, updater, packaging, provenance, published binary | High-risk gates plus reproducibility/provenance review dan release checklist terpisah |

Perubahan high-risk tidak boleh auto-merge dan tidak boleh hanya dinilai dari screenshot atau happy path.

## Batas akses AI

- Gunakan synthetic evidence dan media lab; jangan unggah barang bukti atau data pribadi ke layanan AI.
- Jangan memberikan signing key, production secret, credential, atau akses akun penjualan kepada agent.
- Agent tidak boleh mengubah security exception, baseline hash, atau release claim tanpa task dan approval eksplisit.
- Agent tidak boleh menjalankan destructive raw-device command tanpa allowlist perangkat dan konfirmasi manusia.
- Public pull request tidak boleh menjalankan job pada self-hosted lab runner yang memiliki akses ke disk, secret, atau jaringan internal.
- Dependency baru membutuhkan alasan, license check, lockfile, vulnerability review, dan evaluasi apakah fungsi kecil lebih aman ditulis di shared core.
- Output AI diperlakukan sebagai untrusted proposal sampai diff dan test-nya diperiksa.

## CI versus hardware lab

### Hosted CI sejak M0

- compile/check Rust pada Windows, macOS, dan Linux;
- TypeScript typecheck, lint, unit test, dan frontend build;
- fixture-based acquisition/verifier tests;
- dependency, license, secret, SAST, dan artifact checks sesuai pipeline;
- package smoke jika runner mendukungnya.

### Manual/self-hosted hardware gate

- raw-device enumeration dan read;
- privilege elevation/helper behavior;
- source/destination reversal protection;
- disconnect, bad-sector simulation, destination-full, suspend, reboot, dan resume;
- throughput, memory ceiling, thermal behavior, dan long-run stability;
- exact OS build, architecture, filesystem, controller, enclosure, dan media identity dicatat.

Self-hosted hardware gate dijalankan manual terhadap commit tepercaya. Jangan menghubungkannya langsung ke workflow pull request publik.

## Operating cadence

Cadence default untuk satu pembuat:

- Pilih paling banyak satu high-risk dan satu low/medium-risk task aktif.
- Selesaikan task sampai review sebelum membuka task core berikutnya.
- Jalankan CI pada setiap PR; jalankan hardware smoke hanya ketika platform boundary berubah.
- Setiap akhir milestone, lakukan adversarial review terhadap completion semantics, tamper behavior, dan failure paths.
- Catat keputusan arsitektur baru sebagai ADR/RFC amendment, bukan hanya di percakapan AI.

Memakai tiga AI pada setiap baris kode akan menambah konflik dan biaya review. Gunakan ketiganya ketika risiko membenarkan; untuk task kecil, satu author dan satu reviewer independen cukup.

## Universal session prompt

Gunakan `docs/AI-UNIVERSAL-SESSION-PROMPT.md` untuk sesi panjang yang butuh discovery otomatis, handoff lintas AI, dan logging ke repo.

Gunakan `docs/AI-UNIVERSAL-SESSION-PROMPT-SIMPLE.md` untuk sesi cepat yang tetap harus membaca konteks repo, melanjutkan kerja terakhir, dan menulis catatan akhir ke `docs/ai-session-log/`.

## Definition of ready untuk mulai Code

Code M0 boleh dimulai setelah:

- repository Git diinisialisasi;
- baseline RFC dan checksum masuk commit awal;
- task packet pertama disetujui;
- branch/worktree policy diterapkan;
- CI skeleton dan protected-branch intent dicatat;
- fixture sepenuhnya synthetic;
- tidak ada agent yang memegang secret atau akses raw device secara default.

Rencana eksekusi M0 berada di `docs/superpowers/plans/2026-07-17-trareon-acquire-foundation.md`. Kerjakan satu task dan satu review checkpoint pada satu waktu; jangan memberikan seluruh 52 langkah kepada tiga agent sekaligus.
