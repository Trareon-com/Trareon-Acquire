# Trareon Acquire Zero-Cash Launch Plan

## Tujuan

Membangun, memvalidasi secara bertahap, mendistribusikan, dan menghasilkan pendapatan dari Trareon Acquire tanpa menggunakan uang pribadi Yusuf Shalahuddin Al Ayyubi As Sobari.

`Zero-cash` berarti Rp0 uang pribadi keluar sebelum ada kas proyek. Ini tidak berarti tidak ada biaya ekonomi, transaction fee, pajak, refund, signing, hardware, atau tenaga. Biaya hanya boleh dibayar dari kas sponsor, preorder, atau penjualan yang telah diterima.

## Aturan kas yang tidak boleh dilanggar

1. Tidak memakai utang, paylater, kartu kredit bergulir, atau dana kebutuhan pribadi.
2. Tidak membeli certificate, Apple membership, hardware, iklan, SaaS, atau jasa sebelum saldo proyek tersedia.
3. Pendapatan proyek dicatat terpisah dari uang pribadi meskipun rekening hukum masih atas nama individu.
4. Refund, tax, payment fee, dan delivery reserve dihitung sebelum dana dianggap dapat dibelanjakan.
5. Preorder tidak disebut penjualan produk selesai; milestone, limitation, estimasi, dan refund trigger ditampilkan sebelum pembayaran.
6. Tidak menjual klaim court admissibility, compliance, accreditation, atau validation yang belum dimiliki.
7. Source dan community build tidak sengaja dirusak untuk memaksa pembelian official build.

## Produk gratis dan berbayar

| Penawaran | Gratis | Berbayar |
|---|---:|---:|
| Source code, schemas, tests, build recipe | Ya | — |
| `Build It For Me` pada fork pengguna | Ya | — |
| Community/self build | Ya | — |
| Independent verifier | Ya | — |
| Official ready-to-run build | — | Ya |
| Native signing/notarization identity | — | Ya, setelah dibiayai kas proyek |
| Published official validation statement | — | Ya |
| Priority support dan assisted setup | — | Ya |
| Organization SOP/profile/report template | — | Ya |
| Training dan certificate of completion | — | Ya |
| Custom validation matrix | — | Ya |

Yang dibayar adalah convenience, provenance, validation evidence, support, dan penghematan waktu; bukan kemampuan memverifikasi evidence lama atau artificial feature lock.

## Penawaran awal

### Community

- Harga: Rp0.
- Source/self-build atau `Build It For Me`.
- Tanpa SLA.
- Tidak mendapat status Official Production.
- Dukungan melalui public documentation dan issue tracker.

### Founder Individual

- Harga target: Rp199.000 sekali bayar.
- Kuota awal: 100 pembeli.
- Hak: official Version 1 build untuk OS yang lulus release gate, update keamanan selama support window Version 1, dan founder badge opsional.
- Tidak menjanjikan semua capability pada semua OS.
- Refund trigger: proyek membatalkan Version 1, target pendanaan minimum tidak tercapai dalam periode yang dipublikasikan, atau milestone delivery melewati batas refund yang disepakati.

### Individual Ready Build

- Harga target setelah Founder: Rp299.000 sekali bayar.
- Termasuk binary siap pakai, provenance, documentation, self-test, dan published capability/validation matrix.
- Major version berikutnya dapat berbayar; verifikasi evidence Version 1 tetap tersedia.

### Organization

- Harga awal: Rp1.500.000–Rp5.000.000 berdasarkan jumlah workstation, support, profile, dan validation scope.
- Tidak mengubah evidence semantics dibanding individual build.
- Nilai tambahan: deployment pack, organization profile, SOP mapping, onboarding, priority support, dan invoice/documentation administratif.

### Services

- Training penggunaan aman dan certificate of completion.
- Assisted build/install.
- Custom report/profile.
- Validation pada hardware/OS milik organisasi.
- Priority incident support.
- Trareon Acquire + Trareon Analysis bundle setelah Analysis tersedia.

## Funding target

Target Founder minimum:

```text
100 pembeli × Rp199.000 = Rp19.900.000 gross project cash
```

Gross bukan dana bebas. Sebelum pembelian eksternal, kas dibagi menjadi:

- refund reserve;
- tax/administration reserve sesuai kewajiban aktual;
- payment/payout fee;
- signing/notarization;
- validation hardware dan media;
- contingency/security response;
- operating reserve.

Persentase final ditentukan setelah kanal pembayaran dan kewajiban pajak dikonfirmasi. Tidak ada pembelanjaan berdasarkan angka gross.

## Jalur Rp0 sebelum Founder target

### Infrastruktur

- Public GitHub repository.
- Standard GitHub-hosted Actions runners.
- GitHub artifact/SBOM attestations.
- GitHub Pages atau halaman yang sudah tersedia di `trareon.com`.
- GitHub Issues, Discussions, Security Advisories, dan Sponsors.
- Open-source security/testing tools yang ditetapkan RFC.
- Public documentation dalam repository.

### Distribusi

1. Pengguna memilih signed source tag.
2. Pengguna menjalankan `Build It For Me` pada fork atau documented workflow.
3. Workflow menghasilkan artifact, checksum, SBOM, attestation, dan capability matrix.
4. Artifact berstatus Community/Self Build dan tidak memakai official Trareon validation identity.
5. Official ready build tidak diterbitkan sampai kas proyek dapat menutup signing, notarization, dan release validation.

### Sponsorship

Tier awal:

- Supporter: Rp25.000 satu kali/bulan.
- Builder: Rp100.000.
- Founder Sponsor: Rp199.000.
- Organization Sponsor: mulai Rp1.000.000.

Sponsor tidak membeli pengaruh terhadap validation result, severity, disclosure, atau evidence semantics.

## Launch gates

### Gate 0 — Credible public foundation

Wajib sebelum membuka waitlist:

- RFC accepted baseline.
- Roadmap publik.
- License file yang benar.
- SECURITY, CONTRIBUTING, CODE_OF_CONDUCT, trademark notice, dan support boundary.
- Synthetic acquisition demo.
- Independent verifier demo.
- CI/security badges berasal dari gate nyata, bukan dekorasi.

### Gate 1 — Waitlist

Target:

- 200 alamat peminat yang melakukan explicit opt-in;
- minimal 20 calon pengguna yang bersedia diwawancarai;
- minimal 10 pengguna menjalankan community build;
- daftar tiga pain point paling sering beserta bukti.

Tidak membeli iklan. Akuisisi pengguna berasal dari dokumentasi, demo, komunitas DFIR, artikel teknis, video demo, GitHub, dan outreach langsung yang sopan.

### Gate 2 — Founder preorder

Preorder dibuka hanya setelah:

- vertical slice dapat didemonstrasikan;
- limitation ditampilkan;
- milestone dan refund trigger dipublikasikan;
- identitas Yusuf serta Trareon jelas;
- delivery target menggunakan rentang, bukan janji satu minggu;
- payment record dan fulfillment ledger tersedia.

### Gate 3 — External spending

Pengeluaran pertama hanya dilakukan setelah cleared project cash melebihi seluruh reserve dan biaya target. Urutan:

1. Wajib hukum/administratif untuk penjualan yang dipilih.
2. Signing/notarization yang membuka official distribution.
3. Dedicated validation media.
4. Independent security/forensic review.
5. Hardware matrix expansion.
6. Marketing berbayar hanya bila conversion organik sudah terbukti.

### Gate 4 — Paid release

- Release gate RFC lulus.
- Binary, SBOM, provenance, documentation, support window, refund terms, dan capability matrix tersedia.
- Purchase tidak memerlukan online activation / license key di aplikasi.
- Binary berbayar dikirim privat (Lynk.id / Gumroad); bukan artifact GitHub Release.
- Critical security fix tidak ditahan untuk memaksa major upgrade.

## Conversion funnel tanpa iklan

```text
Technical article/demo
  -> GitHub/source/verifier
  -> Build It For Me
  -> successful synthetic acquisition
  -> waitlist
  -> Founder/official convenience purchase
  -> training/support/organization expansion
```

Setiap tahap mempunyai metric:

- landing-page visit ke source view;
- source view ke successful community build;
- build ke synthetic verification;
- verification ke waitlist;
- waitlist ke Founder;
- Founder ke successful onboarding;
- support volume dan refund rate.

Tidak ada telemetry dalam aplikasi. Metric berasal dari opt-in website, repository data publik, sales record, dan feedback sukarela.

## Konten pemasaran yang tetap defensible

- Demo failure handling, bukan hanya happy path.
- Tampilkan tampered package yang ditolak verifier.
- Bandingkan workflow dan transparency, bukan membuat klaim lebih lengkap dari tool lain tanpa test.
- Terbitkan capability dan limitation matrix bersama release.
- Jelaskan mengapa macOS RAM, Windows driver, HPA/DCO, SSD/TRIM, dan live state memiliki batas.
- Gunakan frasa `aligned with`, `tested on`, dan `validated for the listed combination`; hindari `court-approved`, `100% complete`, dan `works everywhere`.

## Risiko bisnis dan respons

| Risiko | Respons |
|---|---|
| Tidak ada yang mau preorder | Jangan membeli layanan; wawancara ulang dan kecilkan scope |
| Source build dianggap terlalu sulit | Perbaiki `Build It For Me`, documentation, dan self-test |
| Community binary mengurangi penjualan | Jual official trust, support, validation, dan convenience; jangan merusak community build |
| Apple/Windows cost melebihi target | Tunda official platform tersebut dan nyatakan statusnya; cari sponsor khusus platform |
| Founder meminta semua fitur | Kontrak Founder hanya Version 1 scope dan capability matrix |
| Refund tinggi | Hentikan spending non-wajib dan perbaiki messaging/delivery |
| Security incident | Ikuti coordinated disclosure, revoke affected artifact, dan gunakan contingency reserve |
| Satu maintainer menjadi bottleneck | Dokumentasikan release process dan cari reviewer independen sebelum production |

## Definition of business readiness

Trareon siap mengambil pembayaran ketika:

- prototype nyata dapat dicoba;
- pembeli memahami bahwa produk masih Founder/preview bila belum production;
- price, scope, delivery range, support, refund, privacy, dan limitation tertulis;
- fulfillment dan refund dapat dilakukan secara konsisten;
- tidak ada klaim validation palsu;
- uang proyek dapat dipisahkan dan dicatat;
- pembuat siap memenuhi kewajiban hukum dan pajak yang berlaku.

## Keputusan akhir

Trareon tidak mengejar `zero total cost`; Trareon menjalankan `zero personal cash before revenue`. Model ini memungkinkan proyek dimulai dengan Rp0, tetapi tidak menjamin pendapatan. Satu-satunya alasan melanjutkan pengeluaran setelah prototype adalah bukti pengguna: successful community builds, waitlist berkualitas, Founder conversion, dan retention/support demand.
