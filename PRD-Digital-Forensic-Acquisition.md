# PRD - Desktop

## 1. Ringkasan

- **Nama produk:** Trareon Acquire *(working title)*
- **Jenis produk:** Aplikasi desktop portable untuk pengumpulan dan akuisisi barang bukti digital
- **Platform target:** Windows, macOS, Linux
- **Arsitektur produk:** Shared core dengan adapter native untuk akses perangkat dan privilege pada setiap OS
- **Mode operasi:** Live acquisition, dead-box acquisition melalui media boot forensik, dan targeted collection
- **Konektivitas:** Full offline; tidak membutuhkan internet atau layanan cloud
- **Versi dokumen:** 0.3
- **Tanggal:** 16 Juli 2026
- **Status:** Draft untuk persetujuan produk
- **Standar utama:** ISO/IEC 27037:2012, ISO/IEC 17025:2017, ISO/IEC 17043:2023
- **Standar pendamping yang direkomendasikan:** ISO/IEC 27041:2015, ISO/IEC 27042:2015, ISO/IEC 27043:2015

### 1.1 Visi produk

Trareon Acquire membantu ahli digital forensik mengidentifikasi, mengumpulkan, mengakuisisi, memverifikasi, dan mempreservasi barang bukti digital dari komputer melalui satu workflow terpandu yang konsisten, dapat diaudit, dan dapat dipertanggungjawabkan.

Produk menyatukan akuisisi storage, RAM, keadaan jaringan, dan artefak terpilih tanpa melakukan pemeriksaan atau analisis forensik. Hasil diserahkan sebagai evidence package terbuka dan terverifikasi kepada aplikasi analisis yang terpisah.

### 1.2 Keputusan produk yang telah disetujui

- Produk tersedia sebagai aplikasi native portable untuk Windows, macOS, dan Linux.
- Ketiga build menggunakan shared core untuk workflow, model kasus, hashing, manifest, audit log, chain of custody, verifikasi, dan reporting.
- Akses raw disk, RAM, network capture, privilege elevation, serta device discovery diimplementasikan melalui adapter native masing-masing OS.
- Produk hanya mencakup pengumpulan, akuisisi, dan preservasi. Pemeriksaan serta analisis forensik berada pada produk lain.
- Produk beroperasi sepenuhnya offline dan tidak memiliki telemetry, cloud sync, atau online licensing.
- Evidence package menggunakan nama kerja `.fsnap`, dengan schema terbuka dan payload dalam format forensik standar.
- System Snapshot merupakan pengecualian analisis yang disetujui: aplikasi boleh membandingkan dua snapshot, menghasilkan diff, dan memberi risk indicator deterministic yang dapat dijelaskan.

### 1.3 Definisi “portable dan tanpa dependency eksternal”

Produk harus dapat dijalankan dari removable media atau direktori biasa tanpa installer dan tanpa membutuhkan runtime, package manager, driver, atau aplikasi pihak ketiga yang telah terpasang sebelumnya.

Komponen privileged atau driver yang diperlukan boleh dimuat secara sementara dari paket aplikasi apabila:

- ditandatangani sesuai persyaratan OS;
- hanya aktif selama operasi yang membutuhkannya;
- tidak membuat service, startup item, kernel extension, atau konfigurasi persisten setelah proses selesai;
- setiap load, unload, kegagalan, dan perubahan yang ditimbulkan dicatat dalam audit log;
- aplikasi memberi peringatan sebelum tindakan yang dapat mengubah source state atau membutuhkan reboot.

Source dependency yang digunakan harus dibundel atau static-linked, dipin versinya, dibangun bersama produk, memiliki lisensi yang kompatibel, tercantum dalam SBOM, dan divalidasi sebagai bagian dari build. Implementasi ulang dilakukan bila dependency tidak dapat dipelihara, tidak dapat divalidasi, tidak aman, atau lisensinya tidak kompatibel; bukan dengan menulis ulang primitive kriptografi secara sembarangan.

## 2. Masalah yang Ingin Diselesaikan

### 2.1 Masalah utama

Ahli digital forensik saat ini sering menggunakan tool berbeda untuk imaging HDD/SSD, capture RAM, network capture, targeted artifacts, hashing, chain of custody, dan reporting. Fragmentasi ini menyebabkan:

- metadata kasus harus dimasukkan berulang kali;
- identifier, timestamp, dan format log tidak konsisten;
- keputusan live-versus-dead dan urutan volatilitas tidak terdokumentasi seragam;
- dampak tool terhadap live source sulit dijelaskan;
- bukti parsial, bad sector, retry, dan area yang tidak terbaca mudah terlewat dalam laporan;
- validasi metode dan versi tool terpisah dari aktivitas akuisisi;
- chain of custody sering berada pada formulir di luar tool;
- operator harus memahami batas teknis yang berbeda pada Windows, macOS, dan Linux;
- ketergantungan eksternal dapat gagal pada lokasi yang offline atau komputer yang tidak memiliki tool pendukung.

### 2.2 Siapa yang terdampak

- Digital Evidence First Responder dan petugas olah TKP digital.
- Ahli digital forensik pada laboratorium pemerintah atau swasta.
- Penyidik yang melakukan penyitaan dan pengumpulan perangkat komputer.
- Tim DFIR enterprise yang melakukan koleksi lokal untuk proses hukum atau insiden internal.
- Evidence custodian yang menjaga penyimpanan dan perpindahan barang bukti.
- Quality manager laboratorium yang mengelola validasi metode dan bukti kompetensi.
- Reviewer, auditor, jaksa, pengacara, atau pihak lain yang perlu memahami bagaimana bukti diperoleh.

### 2.3 Dampak bisnis dan pengguna

- Waktu persiapan dan pengumpulan lebih panjang dari yang diperlukan.
- Risiko barang bukti volatil hilang akibat keputusan yang terlambat.
- Risiko source berubah tanpa dokumentasi memadai.
- Risiko akuisisi tidak lengkap tetapi dilaporkan sebagai berhasil.
- Risiko bukti ditolak atau dipertanyakan karena proses, tool, dan chain of custody tidak dapat direkonstruksi.
- Biaya lisensi, pelatihan, validasi, dan pemeliharaan meningkat karena terlalu banyak tool.
- Laboratorium kesulitan menunjukkan metode yang konsisten, repeatable, dan tervalidasi.

## 3. Tujuan

### 3.1 Tujuan utama

1. Menyediakan workflow terpandu untuk seluruh aktivitas ISO/IEC 27037 yang berada dalam lingkup aplikasi: identification, collection, acquisition, dan preservation.
2. Menggabungkan akuisisi storage, RAM, keadaan jaringan, dan artefak terpilih ke satu model kasus dan evidence package.
3. Meminimalkan perubahan pada source serta mencatat perubahan yang tidak dapat dihindari pada live acquisition.
4. Menghasilkan bukti yang dapat diverifikasi secara independen tanpa membutuhkan Trareon Acquire.
5. Menjamin aplikasi dapat digunakan di lapangan secara portable dan offline pada Windows, macOS, dan Linux.
6. Mendukung laboratorium dalam menghasilkan rekaman metode, validasi, kompetensi, kondisi, deviasi, dan quality control yang relevan untuk ISO/IEC 17025.
7. Mendukung partisipasi proficiency testing dan pertukaran paket hasil yang dapat diaudit untuk proses ISO/IEC 17043, tanpa mengklaim aplikasi sebagai penyelenggara PT atau pemberi akreditasi.

### 3.2 Tujuan sekunder

- Mengurangi tool switching dan pengisian metadata berulang.
- Mengurangi kesalahan operator melalui preflight, capability checks, dan mandatory documentation gates.
- Membuat keputusan dan deviasi dapat ditinjau oleh pemeriksa lain.
- Menyediakan format output terbuka, terdokumentasi, dan stabil untuk aplikasi analisis terpisah.
- Memudahkan regression testing ketika OS, driver, filesystem, atau komponen produk berubah.

## 4. Non-Tujuan

Hal berikut tidak dibangun dalam produk ini:

- pemeriksaan dan analisis forensik terhadap isi evidence;
- indexing, keyword search, timeline, correlation, link analysis, atau interpretation;
- file carving, recovery file terhapus, malware analysis, reverse engineering, atau sandboxing;
- password cracking, credential recovery, atau bypass autentikasi;
- AI classification, content categorization, face recognition, atau penentuan relevansi otomatis;
- magic-byte interpretation, extension-mismatch classification, entropy scoring, atau AI-generated collection summary di luar deterministic System Snapshot;
- local-LLM recommendation, evidence prioritization, atau narasi otomatis di dalam acquisition core;
- penentuan bersalah/tidak bersalah atau rekomendasi hukum;
- mobile-device acquisition pada MVP;
- cloud acquisition pada MVP;
- remote fleet-scale collection pada MVP;
- recurring scheduling, directory watchdog, event-triggered auto-collection, atau endpoint monitoring;
- chip-off, JTAG, media repair, atau hardware recovery pada media rusak;
- fungsi evidence management system jangka panjang di luar pembuatan, verifikasi, copy, export, dan transfer package;
- klaim bahwa penggunaan aplikasi otomatis membuat organisasi compliant atau terakreditasi ISO.

Preview pada produk dibatasi pada informasi yang diperlukan untuk mengidentifikasi source dan menentukan ruang lingkup koleksi. Preview tidak boleh berkembang menjadi fitur analisis bukti.

## 5. Ruang Lingkup

### In scope

#### Platform dan distribusi

- Build native portable untuk Windows, macOS, dan Linux.
- Shared core untuk workflow dan evidence integrity.
- Native adapter per OS untuk device enumeration, raw I/O, RAM, network capture, filesystem metadata, privilege, dan removable media.
- Media boot forensik x86_64 untuk dead-box storage acquisition.
- Capability matrix yang ditandatangani dan dibundel per release.

#### Identification

- Case creation dan identitas pemeriksa.
- Rekaman otorisasi, tujuan, lokasi, waktu, serta batas pencarian/koleksi.
- Rekaman keadaan perangkat, layar, power state, encryption state, network state, dan peripheral.
- Inventarisasi storage, volume, interface jaringan, removable media, serta identifier perangkat.
- Dokumentasi foto, screenshot, catatan, zona waktu, waktu perangkat, dan offset waktu.
- Penentuan order of volatility dan acquisition plan terpandu.
- Deteksi status BitLocker, FileVault, LUKS, VeraCrypt, device encryption, TPM, dan kebutuhan mempertahankan live state sejauh dapat dilakukan tanpa bypass atau analisis isi.

#### Case lifecycle

- Membuat, mencari, membuka, mengedit metadata, menutup, dan mengarsipkan kasus.
- Filter berdasarkan nama, nomor, tanggal, status, dan tipe kasus.
- Seluruh acquisition, evidence item, attachment, dan custody event wajib terikat ke case; orphan evidence dilarang.

#### Collection

- Evidence item ID dan label offline.
- Checklist penanganan, keselamatan, isolasi jaringan, power decision, serta potensi anti-forensics.
- Chain of custody sejak item pertama kali dicatat.
- Rekaman segel, kondisi, pemegang, transfer, dan tujuan.
- Emergency override dengan alasan wajib.

#### Acquisition

- Physical storage imaging.
- Logical volume acquisition.
- Targeted file dan artefact collection.
- RAM acquisition ketika didukung oleh platform dan security state.
- Live system-state snapshot.
- Network interface snapshot dan packet capture.
- Hashing, verification, coverage map, error map, split image, pause/resume terkontrol, dan recovery setelah interruption.
- Deteksi dan pencatatan hardware/software write protection.
- Capture provenance untuk setiap output.
- Preset targeted collection Quick, Standard, Deep, dan Custom dengan scope preview serta exclusion record.
- Controlled multi-source acquisition sebagai kemampuan setelah MVP core stabil.

#### Preservation

- Evidence package dengan manifest terbuka.
- Append-only audit trail dan chain-of-custody record.
- Cryptographic hashing dan digital signature offline.
- Evidence master, verified working copy, dan archive copy relationship.
- Independent verifier portable.
- Human-readable acquisition report dan machine-readable export.
- Fixity verification tanpa membuka atau mengubah isi evidence.
- Paket `.fsnap` dalam bentuk direktori atau single-file archive yang dapat diverifikasi dan dibuka tanpa aplikasi analisis.

#### Quality dan validation support

- Build identity dan component inventory.
- Offline self-test dan known-dataset validation suite.
- Rekaman validasi metode, performance check, limitation, anomaly, dan deviation.
- Export validation pack untuk quality management system.
- Import/export paket proficiency-testing yang ditandatangani pada fase setelah acquisition core stabil.

### Out of scope

- Seluruh fitur pada bagian Non-Tujuan.
- Apple Silicon boot environment pada MVP jika platform security tidak memungkinkan metode yang telah divalidasi.
- Full physical RAM capture ketika OS/hardware/security policy tidak menyediakan metode yang tervalidasi.
- Full-disk acquisition pada source yang tidak dapat diakses tanpa exploit, bypass, atau perubahan security policy berisiko tinggi.
- RAID reconstruction dan acquisition media rusak tingkat lanjut pada MVP.
- Long-term evidence repository dan multi-site case collaboration.
- Mobile triage/acquisition, cloud snapshot, dan remote collection; masing-masing memerlukan PRD serta threat model terpisah.
- Auto-collect saat boot tanpa konfirmasi operator dan acquisition plan yang tercatat.

System Snapshot diff dan deterministic risk indicator secara eksplisit dikecualikan dari batas analisis di atas. Fitur tersebut tidak boleh berkembang menjadi interpretasi isi file, attribution, malware verdict, atau kesimpulan investigatif.

## 6. User Story

### 6.1 First responder

- Sebagai first responder, saya ingin aplikasi menanyakan keadaan perangkat dan memberi urutan tindakan yang sesuai supaya bukti volatil tidak hilang karena langkah yang salah.
- Sebagai first responder, saya ingin setiap keputusan power, network isolation, dan acquisition method dicatat supaya tindakan saya dapat dijelaskan kemudian.
- Sebagai first responder, saya ingin menjalankan aplikasi dari USB tanpa instalasi atau internet supaya dapat bekerja di lokasi terisolasi.

### 6.2 Ahli digital forensik

- Sebagai ahli, saya ingin mengakuisisi storage, RAM, network state, dan artefak terpilih dari satu case supaya metadata dan provenance konsisten.
- Sebagai ahli, saya ingin mengetahui capability dan limitation sebelum acquisition dimulai supaya tidak mengandalkan metode yang tidak didukung.
- Sebagai ahli, saya ingin hasil parsial, bad sector, retry, dan skipped range terlihat jelas supaya saya tidak salah menyatakan completeness.
- Sebagai ahli, saya ingin hasil dapat diverifikasi oleh executable terpisah supaya pihak lain tidak perlu mempercayai aplikasi utama.
- Sebagai ahli, saya ingin membandingkan dua system snapshot dan melihat perubahan file, proses, serta koneksi dengan alasan risk indicator yang transparan supaya saya dapat memprioritaskan collection lanjutan tanpa menganggapnya kesimpulan forensik.

### 6.3 Evidence custodian

- Sebagai custodian, saya ingin setiap package memiliki identifier, hash, signature, condition, dan transfer history supaya chain of custody dapat direkonstruksi.
- Sebagai custodian, saya ingin membuat working copy yang terhubung ke evidence master supaya analisis tidak dilakukan pada master.

### 6.4 Quality manager

- Sebagai quality manager, saya ingin mengetahui build, adapter, driver, metode, konfigurasi, dan validation status yang dipakai supaya hasil dapat ditelusuri ke metode tervalidasi.
- Sebagai quality manager, saya ingin mengekspor validation evidence dan deviation record supaya dapat dimasukkan ke sistem mutu laboratorium.

### 6.5 Pengguna aplikasi analisis terpisah

- Sebagai analis, saya ingin menerima format image, capture, artefak, manifest, dan provenance yang terdokumentasi supaya dapat mengimpor bukti tanpa menggunakan acquisition tool untuk analisis.

### 6.6 Case supervisor dan operator multi-source

- Sebagai supervisor, saya ingin mencari, menutup, dan mengarsipkan case tanpa memutus lineage evidence supaya tidak ada bukti yang kehilangan konteks.
- Sebagai operator, saya ingin menjalankan beberapa acquisition independen dengan resource guard supaya waktu lapangan berkurang tanpa merusak throughput, source, atau auditability.

## 7. Kebutuhan Fungsional

Prioritas menggunakan **P0** untuk MVP wajib, **P1** untuk rilis berikutnya, dan **P2** untuk pengembangan lanjutan.

### 7.1 Shared core dan model kasus

| ID | Prioritas | Kebutuhan |
|---|---:|---|
| FR-001 | P0 | Sistem harus membuat Case ID unik atau menerima Case ID organisasi tanpa menimpa kasus yang sudah ada; case memuat name, number, type, description, status, dan lifecycle timestamps. |
| FR-002 | P0 | Sistem harus merekam identitas pemeriksa, organisasi, lokasi, tujuan, otorisasi, yurisdiksi, dan scope collection. |
| FR-003 | P0 | Semua record harus memiliki timestamp UTC, waktu lokal, zona waktu, sumber waktu, serta confidence/offset bila diketahui. |
| FR-004 | P0 | Sistem harus menggunakan state machine yang mencegah tahapan wajib dilewati tanpa override yang beralasan. |
| FR-005 | P0 | Semua tindakan, keputusan, perubahan konfigurasi, error, retry, cancel, dan override harus masuk append-only audit log. |
| FR-006 | P0 | UI harus menjelaskan tindakan yang akan dilakukan, privilege yang diperlukan, expected source changes, dan batas metode sebelum eksekusi. |
| FR-007 | P0 | Shared core harus menghasilkan perilaku manifest, hashing, reporting, dan audit schema yang sama pada seluruh OS. |
| FR-008 | P0 | Sistem harus menyimpan draft kasus secara aman dan dapat melanjutkan workflow setelah aplikasi ditutup atau komputer examiner restart. |
| FR-009 | P0 | Sistem harus menyediakan list, search, filter, open, edit metadata, close, dan archive case; setiap evidence object wajib memiliki Case ID dan sistem harus menolak orphan evidence. |

### 7.2 Identification

| ID | Prioritas | Kebutuhan |
|---|---:|---|
| FR-010 | P0 | Wizard harus merekam apakah perangkat hidup, mati, sleep, hibernating, locked, atau tidak dapat ditentukan. |
| FR-011 | P0 | Sistem harus merekam keadaan layar, encryption indication, logged-in users, network connectivity, peripheral, dan destructive activity yang terlihat. |
| FR-012 | P0 | Sistem harus menginventarisasi storage device, volume, removable media, network interface, OS/build, arsitektur CPU, dan identifier yang tersedia. |
| FR-013 | P0 | Setiap source harus memperoleh Evidence Source ID yang unik dan stabil di dalam kasus. |
| FR-014 | P0 | Pengguna harus dapat melampirkan foto, screenshot, diagram koneksi, dan catatan tanpa mengubah file asli yang diimpor. |
| FR-015 | P0 | Sistem harus merekam system clock, reference clock, timezone, dan perbedaan waktu tanpa otomatis mengubah clock target. |
| FR-016 | P0 | Sistem harus menghasilkan urutan volatilitas yang direkomendasikan berdasarkan source yang ditemukan dan membiarkan ahli mengubahnya dengan alasan. |
| FR-017 | P0 | Sistem harus menampilkan capability matrix spesifik OS/build/hardware sebelum menawarkan metode acquisition. |
| FR-018 | P0 | Aplikasi tidak boleh melakukan acquisition otomatis hanya karena perangkat terdeteksi. |
| FR-019 | P0 | Identification harus mendeteksi dan mencatat status BitLocker, FileVault, LUKS, VeraCrypt/device encryption, protector/key requirement, TPM presence/version, serta memberi rekomendasi mempertahankan live state atau RAM tanpa mengambil atau membypass credential. |

### 7.3 Collection dan chain of custody

| ID | Prioritas | Kebutuhan |
|---|---:|---|
| FR-020 | P0 | Sistem harus membuat Evidence Item ID, label teks, dan QR code yang dapat dicetak atau disimpan offline; pola ID dapat dikonfigurasi dengan default seperti `CL-YYYYMMDD-XXXX`. |
| FR-021 | P0 | Sistem harus merekam make, model, serial, asset tag, interface, kapasitas, kondisi, lokasi ditemukan, collector, serta waktu collection. |
| FR-022 | P0 | Sistem harus menyediakan checklist penanganan hidup/mati, network isolation, power, encryption, anti-forensics, dan bukti tradisional. |
| FR-023 | P0 | Setiap transfer custody harus merekam pemberi, penerima, waktu, tujuan, kondisi, segel, dan acknowledgement/signature. |
| FR-024 | P0 | Chain of custody harus dapat diekspor secara machine-readable dan human-readable. |
| FR-025 | P0 | Emergency mode boleh melewati field tertentu hanya setelah mencatat alasan, operator, dan field yang belum lengkap. |
| FR-026 | P1 | Sistem harus mendukung template field dan approval organisasi tanpa mengubah audit schema inti. |

### 7.4 Acquisition planning dan preflight

| ID | Prioritas | Kebutuhan |
|---|---:|---|
| FR-030 | P0 | Sistem harus menawarkan physical, logical, targeted, live, atau dead-box method hanya bila capability matrix menyatakan metode didukung. |
| FR-031 | P0 | Setiap acquisition plan harus mencatat source, destination, method, scope, expected output, hash, verification, compression, segmentation, dan error policy. |
| FR-032 | P0 | Preflight harus memeriksa privilege, source accessibility, destination capacity, filesystem limit, file-size limit, write permission, dan estimated duration. |
| FR-033 | P0 | Sistem harus membedakan source dan destination secara visual dan meminta konfirmasi ulang sebelum write dimulai. |
| FR-034 | P0 | Sistem harus menolak destination yang sama dengan source atau berada pada volume source kecuali skenario tervalidasi dan override beralasan. |
| FR-035 | P0 | Sistem harus memeriksa status write blocker/read-only dan mencatat cara verifikasinya. |
| FR-036 | P0 | Jika write protection tidak dapat diverifikasi, aplikasi harus memberi warning dan tidak boleh mengklaim source terlindungi. |
| FR-037 | P0 | Sistem harus memeriksa build dan validation status adapter/driver sebelum acquisition dimulai. |
| FR-038 | P1 | Pada platform yang tervalidasi, sistem boleh mengaktifkan OS-level software read-only control, memverifikasi statusnya, dan mencatat perubahan; kontrol ini tidak boleh dilabeli setara hardware write blocker. Menonaktifkannya pada source memerlukan dangerous-action confirmation dan alasan. |

### 7.5 Storage acquisition

| ID | Prioritas | Kebutuhan |
|---|---:|---|
| FR-040 | P0 | Sistem harus mendukung physical bitstream acquisition ke RAW/dd dengan segmenting opsional. |
| FR-041 | P0 | Sistem harus mendukung logical volume acquisition dan targeted file collection dengan preservasi metadata yang dapat diperoleh. |
| FR-042 | P0 | Sistem harus merekam media type SSD/HDD/removable/virtual, device geometry, logical/physical sector size, capacity, partition layout, dan identifier storage. |
| FR-043 | P0 | Sistem harus mendeteksi atau mencatat keterbatasan HPA, DCO, AMA, hidden area, 4Kn, sparse source, dan unsupported geometry bila platform memungkinkan. |
| FR-044 | P0 | Imaging engine harus menghitung SHA-256 selama penulisan dan melakukan post-acquisition verification. |
| FR-045 | P0 | Sistem harus membuat coverage map dan error map yang menunjukkan range acquired, unreadable, skipped, retried, padded, atau not-addressable. |
| FR-046 | P0 | Bad-sector policy, retry count, timeout, read block size, padding behavior, dan hasil setiap retry harus tercatat. |
| FR-047 | P0 | Sistem harus membedakan status Verified Complete, Verified Partial, Completed Unverified, Failed, Aborted, dan Resumable Interrupted. |
| FR-048 | P0 | Pause/resume hanya diperbolehkan melalui checkpoint dan segment hash yang dapat diverifikasi; resume tidak boleh menimpa data yang sudah terverifikasi. |
| FR-049 | P0 | Kehilangan daya atau destination disconnect harus meninggalkan package parsial yang dapat diidentifikasi dan diaudit. |
| FR-050 | P1 | Sistem harus mendukung EWF/E01 setelah format implementation lulus validation suite dan interoperability test. |
| FR-051 | P1 | Sistem harus mendukung lossless compression dengan algoritma, level, dan library build yang tercatat. |
| FR-052 | P2 | Sistem dapat mendukung RAID/JBOD acquisition dan reconstruction metadata tanpa melakukan analisis isi. |
| FR-053 | P1 | Setelah lulus conformance dan interoperability test, sistem harus mendukung E01/EWF, Ex01/EWF2, dan AFF4 selain RAW/dd dan split-dd. |
| FR-054 | P1 | Sistem dapat memakai SHA-512 atau BLAKE3 sebagai hash tambahan; MD5/SHA-1 hanya untuk kompatibilitas legacy dan tidak boleh menjadi satu-satunya integrity hash. |
| FR-055 | P1 | Source acquisition dapat berupa physical device, partition/volume, atau existing image untuk verified copy/repack; operasi terhadap existing image tidak boleh diposisikan sebagai analisis. |

### 7.6 RAM dan volatile-state acquisition

| ID | Prioritas | Kebutuhan |
|---|---:|---|
| FR-060 | P0 | Sistem harus dapat mengakuisisi physical RAM pada kombinasi OS/hardware yang tercantum sebagai tervalidasi. |
| FR-061 | P0 | Windows adapter harus memakai signed transient driver/helper, mencatat load/unload, dan menghapus komponen sementara setelah selesai. |
| FR-062 | P0 | Linux adapter harus memilih metode userland atau module yang tervalidasi dan mendeteksi Secure Boot/kernel lockdown sebelum capture. |
| FR-063 | P0 | macOS adapter harus menolak full RAM capture bila platform security tidak menyediakan metode tervalidasi tanpa reboot atau perubahan berisiko. |
| FR-064 | P0 | Sistem harus menjelaskan bahwa live capture mengubah memory dan system state, serta merekam tool footprint yang diketahui. |
| FR-065 | P0 | Bila full RAM tidak tersedia, aplikasi boleh menawarkan volatile-state snapshot tetapi harus memberi label berbeda dan tidak menyebutnya memory image. |
| FR-066 | P0 | Volatile-state snapshot harus dapat mencakup process list, logged-on users, open connections, interface, routing, ARP/neighbor, mounted volume, system time, serta optional filesystem inventory berupa path/size/timestamp melalui native source yang tercatat; fitur ini tidak melakukan diff atau risk scoring. |
| FR-067 | P0 | Output RAM harus memiliki format, size, acquisition method, source ranges, error, hash, dan verification status yang tercatat. |
| FR-068 | P1 | RAM output dapat memakai RAW/MEM, platform-native dump, AFF4, atau VMEM bila tervalidasi, dengan compression gzip, LZ4, atau Zstandard yang dibundel dan dicatat. |
| FR-069 | P0 | Aplikasi harus mendeteksi kemampuan komponen RAM yang dibundel, bukan mencari dan menjalankan AVML, WinPmem, LiME, Belkasoft, atau tool pihak ketiga yang kebetulan terpasang pada target. |

### 7.7 Network acquisition

| ID | Prioritas | Kebutuhan |
|---|---:|---|
| FR-070 | P0 | Sistem harus menginventarisasi interface, addresses, routes, DNS configuration, neighbor/ARP state, dan active connection snapshot. |
| FR-071 | P0 | Sistem harus mendukung packet capture ke PCAPNG dengan pilihan interface, duration, size limit, BPF-compatible capture filter, snap length, dan ring buffer. |
| FR-072 | P0 | Packet capture harus memakai transient privileged helper/driver yang dibundel dan tidak memerlukan packet-capture software terpasang. |
| FR-073 | P0 | Sistem harus merekam timestamp source, dropped packet count, interface state changes, filter, privilege mode, dan capture limitation. |
| FR-074 | P0 | Aplikasi harus memperingatkan bahwa capture hanya mencakup traffic yang terlihat pada interface dan periode tersebut. |
| FR-075 | P1 | Sistem dapat mendukung capture dari network TAP atau remote sensor yang dihubungkan secara lokal dan telah tervalidasi. |

### 7.8 System Snapshot dan deterministic comparison

| ID | Prioritas | Kebutuhan |
|---|---:|---|
| FR-076 | P0 | Sistem harus membuat point-in-time System Snapshot yang memuat host/source identity, OS/build/kernel, clock/timezone, filesystem inventory berupa path/size/timestamp/type dan optional hash, running processes, services, logged-on users, open connections, interface, route, ARP/neighbor, mounted volumes, serta collection limitation. |
| FR-077 | P0 | Sistem harus membandingkan dua snapshot dan mengidentifikasi file, process/service, connection, interface, route, user-session, dan mount yang baru, hilang, atau berubah; host identity mismatch harus menghasilkan warning dan memerlukan override untuk dilanjutkan. |
| FR-078 | P0 | Diff harus menerima deterministic risk indicator dari signed dan versioned rules. Setiap indicator harus menjelaskan rule, input fact, score/severity, confidence/limitation, serta referensi langsung ke perubahan yang memicunya; AI/LLM dan opaque scoring dilarang. |
| FR-079 | P0 | Snapshot, diff, rule-pack identity, dan risk indicator harus dapat disimpan sebagai JSON serta MessagePack yang terdokumentasi, masuk ke `.fsnap`, dan diekspor ke CSV/PDF/A/HTML tanpa mengubah source snapshot. |
| FR-118 | P1 | Organisasi dapat membuat custom signed rule pack setelah validation dan approval; rule bawaan tidak boleh dimodifikasi in-place dan setiap re-score menghasilkan derived result baru dengan lineage. |

### 7.9 Targeted artefact collection

| ID | Prioritas | Kebutuhan |
|---|---:|---|
| FR-080 | P0 | Sistem harus menyediakan signed, versioned collection profiles untuk Windows, macOS, dan Linux. |
| FR-081 | P0 | Profile harus mendefinisikan path/source, inclusion rule, metadata, locked-file strategy, expected source changes, dan known limitation. |
| FR-082 | P0 | Pengguna harus dapat memilih profile, user, volume, timeframe, dan file category dalam batas otorisasi kasus. |
| FR-083 | P0 | Sistem harus menyimpan alasan targeted collection dan daftar eksplisit item/source yang tidak dikumpulkan. |
| FR-084 | P0 | File content, alternate stream, extended attribute, ACL, ownership, timestamp, hard-link/symlink relationship, dan sparse metadata harus dipreservasi sejauh OS mengizinkan. |
| FR-085 | P0 | Locked-file atau snapshot mechanism harus dicatat bersama perubahan yang ditimbulkan pada target. |
| FR-086 | P0 | Tool tidak boleh menginterpretasikan arti artefak atau memberi kesimpulan investigatif. |
| FR-087 | P1 | Organisasi dapat membuat custom profile yang ditandatangani dan divalidasi tanpa memodifikasi profile bawaan. |
| FR-088 | P0 | Built-in profiles harus mencakup sesuai scope: Windows browser/Event Logs/Registry/Prefetch/NTFS metadata/LNK/Jump Lists/SRUM/AmCache/ShimCache/BAM-DAM; Linux system/auth/kernel logs, shell history, cron, SSH metadata, package-manager history dan systemd journal; macOS Unified Logs, FSEvents, Spotlight metadata, Keychain metadata, serta application firewall logs. |
| FR-089 | P0 | Preset Quick, Standard, Deep, dan Custom harus menampilkan estimasi waktu/ukuran, daftar source, expected source changes, serta exclusion list sebelum collection; label preset tidak boleh menyiratkan completeness di luar source yang dipilih. |

### 7.10 Evidence integrity dan preservation

| ID | Prioritas | Kebutuhan |
|---|---:|---|
| FR-090 | P0 | Setiap output harus terdaftar dalam manifest `.fsnap` dengan Case ID, Evidence ID, source, acquisition ID, path, size, format, hash, status, dan provenance. |
| FR-091 | P0 | Manifest, audit log, chain of custody, coverage map, dan laporan harus diikat dengan cryptographic hash dan digital signature offline. |
| FR-092 | P0 | Evidence package harus memakai schema dan format terbuka yang terdokumentasi serta versioned. |
| FR-093 | P0 | Evidence master harus read-only dari UI setelah finalization; perubahan menghasilkan package/turunan baru dengan lineage, bukan mutasi diam-diam. |
| FR-094 | P0 | Sistem harus membuat verified working copy dan archive copy tanpa mengubah master serta merekam hubungan hash dan transfer. |
| FR-095 | P0 | Independent verifier harus dapat memverifikasi package, signature, hash, schema, missing file, extra file, coverage map, dan log continuity secara offline. |
| FR-096 | P0 | Verifier harus tersedia sebagai binary portable terpisah untuk ketiga OS dan tidak memerlukan privilege raw-device. |
| FR-097 | P0 | Acquisition report harus menjelaskan source, method, tool/build, settings, timeline, hash, coverage, error, limitation, deviation, dan final status. |
| FR-098 | P0 | Machine-readable export harus dapat diimpor aplikasi analisis terpisah tanpa memodifikasi evidence master. |
| FR-099 | P1 | Sistem harus mendukung scheduled fixity checks dan menghasilkan laporan perubahan/ketidakcocokan. |
| FR-109 | P0 | `.fsnap` harus memuat case metadata, evidence payload, target collections, network captures, coverage/error maps, append-only hash-chained audit log, chain of custody, manifest, dan detached signature; tersedia sebagai direktori kerja serta single-file archive untuk transfer dan dapat dibuka kembali hanya dalam mode verify/custody/export tanpa analisis isi. |

### 7.11 Validation dan quality support

| ID | Prioritas | Kebutuhan |
|---|---:|---|
| FR-100 | P0 | Setiap release harus memiliki immutable build ID, source revision, toolchain version, SBOM, dependency hash, driver ID, dan signing identity. |
| FR-101 | P0 | Aplikasi harus menjalankan self-test terhadap hashing, manifest, signature, destination I/O, dan critical adapter sebelum digunakan. |
| FR-102 | P0 | Offline validation suite harus menyediakan known datasets untuk disk imaging, logical collection, hashing, write blocking, interruption, dan error behavior. |
| FR-103 | P0 | Validation result harus mencatat environment, hardware, OS, build, settings, expected result, actual result, anomaly, reviewer, dan approval. |
| FR-104 | P0 | Aplikasi harus memberi status Validated, Conditionally Validated, Not Validated, atau Validation Expired untuk setiap method/platform combination. |
| FR-105 | P0 | Major update pada acquisition engine, adapter, driver, hashing, compression, atau format harus membuat validation status terkait menjadi expired sampai diuji ulang. |
| FR-106 | P0 | Operator deviation dari validated method harus meminta alasan, risk note, approval field, dan follow-up/corrective-action reference. |
| FR-107 | P1 | Sistem harus mengimpor offline proficiency-test package dan mengekspor signed result package tanpa membuka expected answer sebelum submission final. |
| FR-108 | P1 | Fitur proficiency testing harus mendukung separation of roles dan tidak mengklaim nilai kompetensi tanpa proses provider yang sah. |

### 7.12 Offline operation dan update

| ID | Prioritas | Kebutuhan |
|---|---:|---|
| FR-110 | P0 | Semua fungsi P0 harus bekerja tanpa koneksi internet, DNS, cloud API, online activation, atau time server. |
| FR-111 | P0 | Produk tidak boleh mengirim telemetry, crash report, usage data, hash, metadata kasus, atau evidence ke jaringan. |
| FR-112 | P0 | Network access oleh aplikasi harus default-deny; packet capture tidak boleh memberi aplikasi izin melakukan outbound connection. |
| FR-113 | P0 | Update hanya melalui signed offline update bundle yang diverifikasi sebelum diterapkan. |
| FR-114 | P0 | Update tidak boleh dilakukan di tengah acquisition dan harus mempertahankan build lama yang diperlukan untuk memverifikasi kasus sebelumnya. |
| FR-115 | P0 | Aplikasi harus dapat mengekspor capability/validation report tanpa mengekspos data kasus. |
| FR-116 | P1 | Extension hanya boleh berupa signed, versioned, capability-declared acquisition module yang dibundel melalui offline package dan memiliki validation status; arbitrary executable plugin dilarang. |
| FR-117 | P1 | Sistem harus dapat menghitung dan memverifikasi hash file atau direktori secara recursive/multi-threaded serta mengekspor manifest JSON/CSV/text tanpa melakukan magic-byte classification atau entropy analysis. |

### 7.13 Controlled multi-source acquisition

| ID | Prioritas | Kebutuhan |
|---|---:|---|
| FR-120 | P1 | Pengguna dapat memilih beberapa source independen dalam satu case dan menjalankan acquisition secara paralel setelah preflight. |
| FR-121 | P1 | Resource governor harus mendeteksi contention pada source bus, destination, CPU, RAM, disk I/O, dan network serta merekomendasikan sequential execution bila parallelism menurunkan keselamatan atau reliability. |
| FR-122 | P1 | UI harus menampilkan progress, sector/item count, bytes, throughput, ETA, current operation, warning, dan final status per source serta ringkasan keseluruhan. |
| FR-123 | P1 | Pengguna dapat menghentikan satu acquisition atau semua acquisition melalui safe checkpoint; cancel tidak boleh menghapus output parsial atau audit record. |
| FR-124 | P1 | Parallel acquisition tidak boleh menjadi default sebelum capability/resource test menyatakan konfigurasi aman dan tervalidasi. |

### 7.14 Forensic boot-media creator

| ID | Prioritas | Kebutuhan |
|---|---:|---|
| FR-130 | P1 | Aplikasi harus dapat membuat forensic boot USB dari signed offline image yang dibundel atau dipilih pengguna. |
| FR-131 | P1 | Creator harus memverifikasi signature/hash source image, memperingatkan bahwa destination akan ditimpa, lalu memverifikasi seluruh boot media setelah penulisan. |
| FR-132 | P1 | Boot environment harus default read-only terhadap source, meminta pemilihan case/source/destination, dan tidak boleh auto-image atau auto-collect hanya karena sistem selesai boot. |
| FR-133 | P1 | Hasil boot acquisition harus menggunakan `.fsnap` schema yang sama dan disimpan ke destination lokal; remote upload bukan bagian offline acquisition core. |

## 8. Kebutuhan Desktop Khusus

- **Perilaku jendela:** Satu jendela utama dengan workflow kasus; dialog privilege dan destructive-risk harus modal serta tidak dapat tersembunyi di belakang jendela.
- **Multi-window / single-window:** Single-window untuk case workflow. Window terpisah hanya untuk progress monitor, verifier, atau attachment preview non-analitis.
- **Navigasi:** Sidebar case-centric dengan urutan Case → Identify → Collect → Configure → Acquire → Verify → Package serta indikator status untuk case, write protection, privilege, source, dan destination.
- **Tema:** Dark theme sebagai default lapangan dengan light/dark toggle; tema tidak boleh mengurangi kontras warning dan final status.
- **Bahasa visual:** Mengutamakan hierarki tipografi, whitespace, alignment, subtle border, restrained color, dan surface yang tenang seperti aplikasi native macOS; tidak memakai dashboard penuh kartu, gradient dekoratif, atau chrome visual berlebihan.
- **Progressive disclosure:** Default view hanya menampilkan pilihan aman dan informasi inti. Advanced drawer/panel membuka raw parameters, error policy, segmenting, compression, filters, rule details, dan diagnostics tanpa memindahkan operator dari workflow.
- **Power-user behavior:** Tabel mendukung sort, filter, column chooser, resize, pin, multi-select, bulk action terkontrol, context menu, saved view, dan copy/export seperti aplikasi profesional Windows.
- **Command access:** Command palette dan shortcut menyediakan akses cepat ke case, acquisition job, snapshot compare, verify, notes, dan export; semua dangerous action tetap memerlukan confirmation.
- **Offline support:** Wajib penuh; tidak ada fitur yang berubah atau hilang ketika tidak ada jaringan.
- **Penyimpanan lokal:** Pengguna memilih workspace dan evidence destination. Data kasus tidak boleh diam-diam disimpan di profile OS atau cloud-synced directory.
- **Update aplikasi:** Signed offline bundle; tidak ada background updater.
- **Akses file sistem:** Least privilege. UI berjalan sebagai user normal; raw access dilakukan transient helper dengan scope minimal.
- **Notifikasi desktop:** Hanya lokal untuk completion, warning, destination-space, disconnect, thermal/power, atau failure. Isi notifikasi tidak boleh memuat data sensitif kasus.
- **Shortcut keyboard:** Shortcut untuk save note, add attachment, pause terkontrol, verify, dan emergency log marker. Tidak ada single-key destructive action.
- **Pencarian:** Daftar case, evidence item, acquisition job, custody event, dan validation record harus dapat dicari serta difilter; pencarian isi evidence tidak disediakan.
- **Ekspor:** Data non-payload harus dapat diekspor ke JSON dan CSV; report ke PDF/A dan HTML mandiri; Markdown dapat tersedia untuk dokumentasi internal.
- **Progress:** Setiap job menampilkan bytes/sector/item, persentase, throughput, ETA, current operation, warning, dan verification phase secara real time.
- **Responsivitas lintas resolusi:** Mendukung minimum 1366×768, scaling 100–200%, high-DPI, dan keyboard-only navigation.
- **Media portable:** Aplikasi harus mendeteksi bila dijalankan dari removable/read-only media dan menempatkan temporary state pada lokasi yang dipilih pengguna.
- **Privilege UX:** Windows memakai UAC/signed driver; macOS memakai OS-native authorization; Linux memakai OS-native privilege elevation. Semua prompt harus menjelaskan alasan dan operasi.
- **Cleanup:** Temporary helper, driver, mount point, snapshot, dan staging data harus dibersihkan setelah selesai; kegagalan cleanup harus dilaporkan.

## 9. Kebutuhan Non-Fungsional

### 9.1 Performa

- Disk imaging pada media sehat harus mencapai sekurang-kurangnya 80% throughput sequential-read baseline pada hardware uji yang sama ketika SHA-256 aktif.
- Hashing dan penulisan harus menggunakan streaming pipeline dengan bounded memory.
- UI harus tetap responsif selama acquisition dan memperbarui progress sekurang-kurangnya setiap dua detik tanpa mengganggu throughput secara material.
- Startup dari SSD USB harus kurang dari lima detik pada reference hardware, tidak termasuk privilege prompt.
- Memory footprint shared core dan UI ditargetkan di bawah 512 MiB di luar acquisition buffer yang dikonfigurasi.
- Estimasi waktu harus diperbarui berdasarkan throughput aktual dan menampilkan confidence rendah ketika source tidak stabil atau rusak.

### 9.2 Keamanan

- Release binary, driver/helper, profile, capability matrix, validation pack, dan update bundle harus ditandatangani.
- Build harus memiliki SBOM, license inventory, dependency provenance, dan reproducibility evidence.
- Shared core harus menggunakan memory-safe implementation sejauh praktis; unsafe/native code dibatasi pada boundary yang diaudit.
- Secret signing key tidak boleh disimpan di source tree atau binary.
- Case workspace dapat dienkripsi menggunakan mekanisme yang terdokumentasi; evidence encryption tidak boleh mengubah kemampuan verifikasi hash plaintext/ciphertext secara ambigu.
- Aplikasi tidak boleh menjalankan binary dari komputer target atau mempercayai system utility target untuk fungsi kritis bila versi bundled tersedia.
- Semua input path, filename, metadata, image, dan package dianggap tidak tepercaya.
- Privileged helper harus memiliki command surface minimal, authenticated IPC, dan lifetime terbatas.
- Security review, SAST, dependency audit, fuzzing parser/schema, dan signed-release verification wajib sebelum rilis.

### 9.3 Reliabilitas dan integritas

- Crash atau power loss tidak boleh membuat output parsial terlihat sebagai verified complete.
- Setiap write harus dapat ditelusuri ke acquisition ID dan checkpoint.
- Verifikasi harus dapat diulang oleh build verifier lain yang kompatibel.
- Tidak boleh ada silent fallback pada method, hash, compression, source, destination, atau privilege.
- Error harus actionable dan menyebut operasi, source range/item, consequence, serta next safe action.
- Aplikasi harus mampu menangani evidence package berukuran multi-terabyte tanpa memuat manifest atau image seluruhnya ke RAM.

### 9.4 Aksesibilitas

- Memenuhi WCAG 2.2 AA untuk komponen UI yang relevan pada desktop.
- Seluruh workflow utama dapat dijalankan dengan keyboard.
- Status tidak boleh dibedakan hanya dengan warna.
- Screen reader harus dapat membaca source/destination, warning, progress, error, dan final status.
- Bahasa teknis harus memiliki mode ringkas dan detail tanpa menyembunyikan fakta forensik.

### 9.5 Kompatibilitas OS

- Release harus menyertakan support matrix eksplisit, bukan klaim umum “semua versi”.
- Windows: x64 dan ARM64 pada versi yang tercantum dalam support matrix; Windows 10/11 diprioritaskan untuk perangkat kasus yang masih banyak ditemui.
- macOS: Intel dan Apple Silicon pada versi current dan versi sebelumnya yang tervalidasi, dengan limitation per security state.
- Linux: x86_64 sebagai P0; ARM64 sebagai P1. Distro/kernel/filesystem combination harus dicatat dalam validation matrix.
- OS unsupported tetap boleh diidentifikasi, tetapi acquisition method tidak boleh ditawarkan sebagai tervalidasi.

### 9.6 Maintainability dan supply chain

- Shared core, adapter, driver/helper, format schema, verifier, dan UI harus memiliki boundary serta versioning yang jelas.
- Dependency harus vendored/pinned atau diambil dari reproducible internal mirror saat build; runtime tidak boleh mengunduh dependency.
- Perubahan format harus backward-compatible atau disertai migrator/verifier lama yang tetap tersedia.
- Release lama yang dipakai pada kasus harus dapat direproduksi atau setidaknya diverifikasi dari artifact dan provenance yang diarsipkan.

## 10. Edge Case

### Source dan perangkat

- Perangkat tampak mati tetapi sebenarnya sleep/hibernating.
- Device tidak memiliki serial atau melaporkan serial duplikat/palsu.
- Storage tersolder, NVMe behind proprietary controller, USB bridge, 4Kn, atau geometry tidak lazim.
- Source berisi HPA/DCO/AMA atau kapasitas yang berubah selama sesi.
- SSD melakukan TRIM atau background garbage collection.
- Source rusak, lambat, overheating, intermittently disconnecting, atau menghasilkan inconsistent reads.
- RAID, dynamic disk, LVM, APFS Fusion, encrypted container, VM host, atau network-mounted volume.
- Source dan destination memiliki ukuran/nama/model yang mirip.
- Hardware write blocker tidak terdeteksi atau mengklaim read-only tetapi performance check gagal.

### Live system

- Malware atau anti-forensics memalsukan system utility/output.
- Endpoint security mengarantina helper/driver.
- Secure Boot, kernel lockdown, SIP, Gatekeeper, Full Disk Access, atau policy organisasi menolak akses.
- Privilege prompt dibatalkan atau akun tidak berhak.
- Akuisisi RAM membutuhkan reboot; aplikasi harus menolak default karena reboot menghancurkan RAM yang hendak diambil.
- Full RAM tidak mungkin tetapi volatile-state snapshot masih tersedia.
- Sistem clock salah, berubah, tidak sinkron, atau melewati DST selama acquisition.
- User logout, shutdown, atau network state berubah selama capture.

### Destination dan output

- Destination penuh, read-only, dilepas, rusak, atau filesystem membatasi file lebih kecil dari output.
- Destination berada pada source yang sama.
- Power loss terjadi di tengah segment write atau post-verification.
- Hash source berubah karena live system aktif.
- Dua operator mencoba memakai workspace/case yang sama.
- Nama file mengandung karakter non-UTF-8, reserved name, path terlalu panjang, atau collision.
- Evidence package memiliki missing file, extra file, signature invalid, truncated log, atau unsupported schema.

### Scope dan human factor

- Otorisasi hanya mencakup user, waktu, folder, atau kategori tertentu.
- Targeted profile berpotensi mengambil data di luar scope.
- Operator mengubah order of volatility atau melewati rekomendasi.
- Emergency acquisition dimulai sebelum seluruh metadata tersedia.
- Operator memilih cancel ketika device berada pada kondisi tidak aman untuk dihentikan langsung.
- Acquisition complete tetapi verification belum selesai.

## 11. Acceptance Criteria

### Workflow ISO/IEC 27037

- [ ] Case dapat dicari, dibuka, ditutup, dan diarsipkan tanpa memutus hubungan ke evidence; aplikasi menolak orphan evidence.
- [ ] Kasus tidak dapat difinalisasi sebelum identification, collection, acquisition, dan preservation record P0 lengkap atau memiliki override beralasan.
- [ ] Setiap evidence source memiliki identifier unik, kondisi, lokasi, collector, waktu, serta hubungan ke acquisition output.
- [ ] Setiap keputusan live/dead, power, network isolation, method, scope, dan deviation dapat ditemukan di audit report.
- [ ] Chain of custody dapat direkonstruksi dari collection pertama sampai export terakhir.

### Portabilitas dan offline

- [ ] Build Windows, macOS, dan Linux dapat dijalankan dari removable media tanpa installer.
- [ ] Seluruh skenario P0 yang didukung dapat selesai pada jaringan yang diblokir total.
- [ ] Tidak ada DNS query, telemetry, activation, update check, atau outbound connection selama pengujian offline.
- [ ] Helper/driver sementara dihapus setelah sesi dan cleanup diverifikasi; sisa yang tidak dapat dihapus dilaporkan.

### Storage acquisition

- [ ] RAW image media uji identik secara bit-for-bit dengan expected dataset untuk seluruh range yang dinyatakan acquired.
- [ ] SHA-256 hasil dapat direproduksi oleh verifier independen.
- [ ] Bad sector, skipped range, retry, padding, hidden-area limitation, dan output parsial muncul pada coverage/error map serta laporan.
- [ ] Interruption dan resume tidak menimpa segment terverifikasi atau menghasilkan status complete yang salah.
- [ ] Source/destination reversal dicegah pada test scenario.
- [ ] RAW/dd dan split-dd lulus bit-for-bit test; format P1 E01/Ex01/AFF4 tidak dirilis sebelum conformance serta interoperability test lulus.
- [ ] MD5 atau SHA-1 tidak pernah menjadi satu-satunya integrity hash.

### RAM, network, dan artefak

- [ ] RAM acquisition hanya tersedia pada kombinasi OS/hardware yang berstatus tervalidasi.
- [ ] Platform yang tidak mendukung full RAM menolak dengan aman dan menawarkan fallback yang diberi label benar jika tersedia.
- [ ] PCAPNG dapat dibuka oleh tool standar dan memuat metadata capture, timestamp, filter, serta dropped-packet count yang tersedia.
- [ ] Targeted collection mempertahankan metadata yang didukung dan mencatat setiap metadata yang tidak dapat dipreservasi.
- [ ] Tidak ada fitur yang menginterpretasikan isi bukti atau menghasilkan kesimpulan investigatif.
- [ ] Status enkripsi, protector/key requirement, dan TPM dicatat tanpa mengambil credential atau melakukan bypass.
- [ ] Preset Quick/Standard/Deep/Custom menampilkan source, estimasi, expected changes, dan exclusion list sebelum dijalankan.

### System Snapshot

- [ ] Dua snapshot dengan known changes menghasilkan daftar file, process/service, connection, route, session, dan mount baru/hilang/berubah yang sesuai expected dataset.
- [ ] Host identity mismatch tidak dapat dibandingkan diam-diam dan memerlukan override beralasan.
- [ ] Risk indicator yang sama menghasilkan output identik untuk snapshot pair dan rule-pack version yang sama pada seluruh OS yang didukung.
- [ ] Setiap risk indicator dapat ditelusuri ke rule dan exact diff fact; aplikasi tidak menghasilkan malware verdict, attribution, atau kesimpulan investigatif.
- [ ] Re-score dengan rule pack baru menghasilkan derived result baru dan mempertahankan snapshot/diff asli.

### UI/UX

- [ ] Standard workflow dapat diselesaikan tanpa membuka advanced panel.
- [ ] Expert dapat membuka parameter, diagnostics, filter, dan rule detail tanpa meninggalkan case context.
- [ ] Case/evidence/snapshot tables mendukung sort, filter, configurable columns, multi-select, context menu, dan keyboard operation.
- [ ] Dark dan light theme memenuhi contrast requirement serta mempertahankan makna warning/status yang sama.

### Preservation dan interoperability

- [ ] Verifier terpisah mendeteksi file hilang, file tambahan, byte berubah, signature invalid, hash mismatch, dan audit-log discontinuity.
- [ ] Evidence master tidak dapat dimodifikasi melalui UI setelah finalization.
- [ ] Working copy memiliki lineage dan hash yang dapat ditelusuri ke master.
- [ ] Aplikasi analisis terpisah dapat mengimpor output melalui schema/format terdokumentasi tanpa mengubah master.
- [ ] Human-readable report dan machine-readable manifest berisi nilai yang konsisten.
- [ ] `.fsnap` directory dan archive variant menghasilkan manifest/signature yang ekuivalen serta dapat diverifikasi lintas OS.

### Controlled parallel dan boot media

- [ ] Parallel acquisition hanya aktif setelah resource preflight dan setiap job memiliki audit/progress/final status sendiri.
- [ ] Cancel satu job tidak menghentikan atau merusak job lain dan selalu meninggalkan safe checkpoint.
- [ ] Boot-media creator menolak penulisan sebelum destination teridentifikasi dan destructive confirmation disetujui.
- [ ] Boot environment tidak melakukan acquisition otomatis setelah boot dan menghasilkan `.fsnap` yang sama dengan aplikasi native.

### Quality dan validation

- [ ] Setiap acquisition output dapat ditelusuri ke build ID, adapter/driver, configuration, capability status, dan validation record.
- [ ] Perubahan major pada engine/adapter membuat validation status terkait expired secara otomatis.
- [ ] Known-dataset suite mencakup normal, bad-sector, interrupted, destination-full, write-blocking, hashing, dan manifest-tampering scenarios.
- [ ] Release tidak dapat ditandai production-ready apabila P0 validation suite gagal.

## 12. Metrik Keberhasilan

### Correctness dan integrity

- 100% P0 known-dataset acquisition tests menghasilkan byte/metadata yang diharapkan atau documented known limitation.
- 100% perubahan pada evidence package terdeteksi oleh independent verifier.
- 0 silent fallback pada method, hash, source, destination, atau privilege dalam test suite.
- 0 kasus output parsial dilabeli Verified Complete.
- 100% acquisition memiliki build, method, configuration, hash, coverage, dan final status.
- 100% System Snapshot risk indicator dapat direproduksi dari snapshot pair dan rule-pack version yang tercatat.
- 0 opaque atau AI-generated risk indicator di System Snapshot.

### Efisiensi

- Median waktu dari launch sampai acquisition plan siap kurang dari 10 menit untuk examiner terlatih pada skenario standar.
- Pengisian ulang metadata yang sama berkurang minimal 80% dibanding workflow multi-tool baseline.
- Disk imaging mencapai minimal 80% sequential-read baseline pada reference hardware dengan SHA-256 aktif.
- Minimal 95% sesi P0 selesai tanpa restart aplikasi pada compatibility matrix yang didukung.

### Dokumentasi dan usability

- 100% keputusan kritis memiliki actor, timestamp, value, dan rationale/automatic basis.
- Minimal 90% examiner pada usability test dapat menyelesaikan skenario standar tanpa bantuan di luar panduan aplikasi.
- Minimal 95% mandatory field lengkap sebelum finalization; sisanya harus memiliki emergency/deviation record.

### Quality management

- 100% release memiliki SBOM, validation report, capability matrix, limitation list, signing evidence, dan reproducible-build record atau documented exception.
- 100% method/platform combination memiliki status validasi yang terlihat sebelum digunakan.

## 13. Risiko

| Risiko | Dampak | Mitigasi |
|---|---|---|
| Pembatasan Windows/macOS/Linux berubah | Adapter/driver berhenti bekerja atau mengubah source lebih besar | Capability matrix per release, signed adapter, regression lab, safe refusal, rapid offline update pack |
| Klaim “portable tanpa instalasi” disalahartikan | Ekspektasi bahwa kernel access tidak memerlukan komponen privileged | Definisi transient helper yang eksplisit, cleanup verification, source-change log |
| Menulis ulang semua dependency | Bug kriptografi/filesystem, waktu pengembangan ekstrem, validasi sulit | Vendored/static-linked audited components, SBOM, reproducible builds, rewrite hanya bila beralasan |
| Live acquisition mengubah source | Integritas dipertanyakan | Least-invasive method, expected/observed source-change accounting, order of volatility, comprehensive log |
| Hash dianggap bukti completeness | Area tidak terbaca/tersembunyi terlewat | Coverage map, error map, geometry/HPA checks, functional validation, explicit final status |
| Targeted collection melewatkan bukti relevan/ekskulpatif | Bias dan sengketa scope | Rationale, signed profile, exclusion list, scope preview, full acquisition recommendation bila relevan |
| Custom evidence format tidak interoperabel | Vendor lock-in dan aplikasi analisis gagal | Open documented schema, standard payload formats, independent verifier, conformance tests |
| Destination/power failure | Bukti parsial atau korup | Checkpoint, segment hash, transactional manifest, resumable status, never-complete-on-failure |
| Driver/helper diblokir security control | RAM/raw/network capture gagal | Preflight, signed components, documented compatibility, safe fallback, no bypass exploit |
| Full offline menghilangkan trusted timestamp | Waktu absolut sulit dibuktikan | Record clock source/offset/confidence, external reference field, signed local event sequence |
| ISO direvisi | PRD/control mapping menjadi usang | Versioned compliance profile dan traceability matrix yang dapat diperbarui terpisah dari engine |
| Pengguna menganggap aplikasi menjamin kepatuhan | Risiko hukum dan audit | Disclaimer, scope statement, organization policy fields, licensed-standard clause review sebelum klaim |
| Bukti sensitif bocor | Dampak hukum dan privasi | No telemetry, destination warning, optional encryption, least privilege, signed access log |
| Scope MVP terlalu luas | Kualitas acquisition core menurun | Tahapan rilis, P0/P1/P2, analysis/mobile/cloud/remote dikeluarkan |
| Arbitrary plugin mengubah metode tanpa validasi | Hasil tidak repeatable dan supply-chain risk | Hanya signed acquisition module, capability declaration, SBOM, validation gate, dan offline package |
| Parallel-by-default menyebabkan contention | Drop packet, throughput turun, thermal/power failure, atau output korup | Parallel opt-in, resource governor, independent checkpoint, rekomendasi sequential |
| Auto-collection mengambil data di luar otorisasi | Pelanggaran scope dan chain of custody | Tidak ada auto-collect-on-detection/boot; recurring watchdog dipisahkan dari acquisition core |
| Deterministic risk rule memberi false positive/negative | Operator salah memprioritaskan langkah berikutnya | Explainable fact linkage, severity bukan verdict, rule validation, versioning, override, dan review berkala |
| UI terlalu bersih menyembunyikan detail kritis | Ahli tidak melihat parameter/limitation penting | Progressive disclosure, persistent status summary, advanced drawer, command palette, dan pre-finalization review |
| UI terlalu padat menambah human error | First responder salah memilih source/destination | Safe default, visual source/destination separation, hierarchy konsisten, confirmation, dan usability testing |

## 14. Open Questions

Pertanyaan berikut tidak menghambat persetujuan PRD, tetapi harus diselesaikan pada RFC atau release planning:

1. Nama produk final dan identitas penerbit/signing organization.
2. Pengguna komersial pertama: penegak hukum, laboratorium pemerintah, konsultan, atau DFIR enterprise.
3. Bahasa UI/laporan awal selain Bahasa Indonesia dan Inggris.
4. Yurisdiksi serta format chain-of-custody/report yang harus menjadi preset bawaan.
5. Daftar hardware write blocker yang harus dideteksi dan diverifikasi pada MVP.
6. Urutan implementasi format P1: E01/EWF, Ex01/EWF2, AFF4, serta format RAM terkompresi.
7. Minimum OS/kernel compatibility matrix berdasarkan perangkat nyata pengguna awal.
8. Apakah Linux ARM64 dan Apple Silicon dead-box workflow menjadi P1 atau target MVP tambahan.
9. Kebijakan penggunaan dependency: vendored open source sebagai default atau clean-room implementation untuk komponen tertentu.
10. Apakah proficiency-testing participant mode ISO/IEC 17043 masuk rilis 1.1 atau produk quality companion terpisah.
11. Apakah evidence-package encryption wajib pada MVP dan bagaimana key custody diintegrasikan dengan SOP organisasi.
12. Apakah dedicated hardware appliance akan dikembangkan atau hanya didukung melalui integrasi.

## 15. Traceability terhadap Standar

Matriks ini menunjukkan area dukungan produk, bukan pernyataan sertifikasi. Clause-level mapping harus ditinjau menggunakan salinan standar berlisensi.

| Area | Kontrol produk utama | Bukti yang dihasilkan |
|---|---|---|
| ISO/IEC 27037 — Identification | Case scope, source inventory, device state, time record, attachments, volatility plan | Case record, source inventory, foto/screenshot hash, decision ledger |
| ISO/IEC 27037 — Collection | Evidence ID, handling checklist, condition, seal, custody, power/network decision | Collection record, label, chain of custody, override/deviation log |
| ISO/IEC 27037 — Acquisition | Method selection, least-invasive warning, write protection, disk/RAM/network/artifact acquisition, coverage/error map | Image/capture, manifest, hashes, source-change record, acquisition log, final status |
| ISO/IEC 27037 — Preservation | Evidence master, working/archive copy, fixity, signing, verifier, transfer record | Signed package, verification report, lineage, custody export |
| ISO/IEC 17025 — Method control | Build/method identity, validation state, known dataset, limitation, deviation, corrective-action reference | Validation pack, capability matrix, anomaly/deviation record |
| ISO/IEC 17025 — Technical records | Environment, equipment/source identifiers, operator, settings, result, review | Audit log, acquisition report, validation result |
| ISO/IEC 17043 — PT support | Signed offline challenge/result package, role separation, locked expected result | PT participation package dan submission record |
| ISO/IEC 27041 | Fit-for-purpose requirement, method validation, capability and limitation | Method requirement, validation evidence, approval status |
| ISO/IEC 27042 | Continuity, validity, reproducibility, repeatability support | Open schema, exact settings, hashes, verifier, machine-readable provenance |
| ISO/IEC 27043 | Preparation-to-closure process interoperability | Case state, decision history, handoff package untuk aplikasi/proses berikutnya |
| System Snapshot analysis exception | Deterministic snapshot diff dan explainable risk indicator untuk prioritas collection | Original snapshots, derived diff, signed rule-pack ID, indicator rationale, lineage |

## 16. Tahapan Rilis

### MVP / Release 1.0

- Shared core, case workflow, audit, chain of custody, manifest, hashing, signing, dan verifier.
- Native portable build Windows, macOS, Linux.
- Identification dan collection workflow.
- RAW physical imaging, logical acquisition, dan targeted profiles dasar.
- Case list/search/close/archive dengan larangan orphan evidence.
- Deteksi BitLocker, FileVault, LUKS/VeraCrypt/device encryption, serta TPM metadata.
- Preset Quick, Standard, Deep, dan Custom dengan signed target definitions.
- RAM acquisition hanya pada platform combination yang tervalidasi.
- Volatile-state snapshot dan PCAPNG capture.
- Full System Snapshot comparison, deterministic diff, dan explainable risk indicator.
- Evidence package, working copy, report, dan analysis-app handoff.
- Bootable x86_64 forensic environment.
- Offline validation suite dan release qualification pack.

### Release 1.1

- E01/EWF, Ex01/EWF2, dan AFF4 setelah interoperability serta validation lulus.
- Custom signed profiles.
- Custom signed System Snapshot rule packs.
- Proficiency-testing participant mode.
- Expanded write-blocker detection.
- Linux ARM64 dan compatibility expansion.
- Fixity scheduling serta organization templates.
- Controlled multi-source acquisition dan resource governor.
- Forensic boot-media creator dari signed offline image.

### Release 2.x

- Advanced RAID/JBOD acquisition metadata.
- Apple Silicon dead-box workflow bila dapat divalidasi.
- Remote single-endpoint collection sebagai produk/module terpisah bila diperlukan.
- Integrasi dedicated hardware appliance.
- Mobile/cloud acquisition tetap memerlukan PRD terpisah.

## 17. Referensi

- [ISO/IEC 27037:2012 — Guidelines for identification, collection, acquisition and preservation of digital evidence](https://www.iso.org/standard/44381.html)
- [ISO/IEC 17025:2017 — General requirements for the competence of testing and calibration laboratories](https://www.iso.org/standard/66912.html)
- [ISO/IEC 17043:2023 — General requirements for the competence of proficiency testing providers](https://www.iso.org/standard/80864.html)
- [ISO/IEC 27041:2015 — Guidance on assuring suitability and adequacy of incident investigative method](https://www.iso.org/standard/44405.html)
- [ISO/IEC 27042:2015 — Guidelines for the analysis and interpretation of digital evidence](https://www.iso.org/standard/44406.html)
- [ISO/IEC 27043:2015 — Incident investigation principles and processes](https://www.iso.org/standard/44407.html)
- [SWGDE Best Practices for Computer Forensic Acquisitions](https://www.swgde.org/documents/published-complete-listing/17-f-002-best-practices-for-computer-forensic-acquisitions/)
- [SWGDE Best Practices for Digital Evidence Collection](https://www.swgde.org/documents/published-complete-listing/18-f-002-best-practices-for-digital-evidence-collection/)
- [SWGDE Focused Collection and Examination of Digital Evidence](https://www.swgde.org/documents/published-complete-listing/14-f-003-focused-collection-and-examination-of-digital-evidence/)
- [NIST Computer Forensics Tool Testing Program](https://www.nist.gov/itl/csd/secure-systems-and-applications/computer-forensics-tool-testing-program-cftt)
- [NIST Federated Testing Project](https://www.nist.gov/itl/csd/secure-systems-and-applications/computer-forensics-tool-testing-program-cftt/federated)
- [NIST SP 800-86 — Guide to Integrating Forensic Techniques into Incident Response](https://csrc.nist.gov/pubs/sp/800/86/final)
- [Riset kompetitor dan pola arsitektur](./Research/Digital-Forensic-Acquisition-Landscape.md)

## 18. Integrasi Riset Internal Collection Mode

Bagian ini merekam bagaimana feature list riset internal diintegrasikan agar keputusan scope dapat ditelusuri.

| Area riset | Keputusan PRD | Penempatan |
|---|---|---|
| Case management | Diterima P0; ditambah search/filter/close/archive dan no orphan evidence | FR-001–FR-009 |
| Disk formats dd/split-dd/E01/Ex01/AFF4 | RAW/dd dan split-dd P0; format container lain P1 setelah conformance test | FR-040–FR-055 |
| Multi-hash | SHA-256 wajib; SHA-512/BLAKE3 tambahan; MD5/SHA-1 compatibility only | FR-044, FR-054 |
| Magic-byte dan entropy | Tidak masuk acquisition core karena merupakan pemeriksaan/analisis | Non-Tujuan |
| RAM tool detection | Diadaptasi menjadi bundled native capability, bukan menjalankan tool eksternal yang terpasang | FR-060–FR-069 |
| Software write blocker | Diterima terbatas P1 dengan validation dan tidak diklaim setara hardware | FR-035–FR-038 |
| Mobile triage | Dipisahkan; memerlukan PRD produk/module mobile | Out of scope |
| Network capture | Diterima P0: interface, BPF/capture filter, PCAPNG, ring buffer, progress, drop count | FR-070–FR-075 |
| Cloud snapshot | Dipisahkan; bertentangan dengan full-offline core dan memerlukan credential/threat model lain | Out of scope |
| System snapshot | Diterima penuh sebagai pengecualian analisis: point-in-time state, compare, diff, dan deterministic explainable risk indicator | FR-065–FR-066, FR-076–FR-079, FR-118 |
| Target collection ala KAPE | Diterima sebagai signed versioned profiles serta Quick/Standard/Deep/Custom presets | FR-080–FR-089 |
| Remote collection | Roadmap/module terpisah; tidak masuk MVP lokal/offline | Release 2.x / PRD terpisah |
| Encryption detection | Diterima P0 tanpa credential extraction atau bypass | FR-019 |
| Hash/verify bulk | Manifest, recursive bulk hash, dan verifier diterima; content classification tidak diterima | FR-090–FR-099, FR-117 |
| Chain of custody dan QR | Diterima P0 dengan configurable ID, signature, serta custody export | FR-020–FR-026, FR-091 |
| Acquire All parallel | Diterima P1 sebagai controlled parallel, bukan parallel-by-default | FR-120–FR-124 |
| `.fsnap` evidence package | Diterima sebagai nama kerja open package; directory dan archive variant | FR-090–FR-099, FR-109 |
| Scheduling/watchdog | Tidak masuk acquisition core karena monitoring dan risiko scope | Non-Tujuan |
| Boot USB creator | Diterima P1; signed image, destructive confirmation, verify-after-write, tanpa auto-collect | FR-130–FR-133 |
| AI-assisted features | Tidak masuk produk acquisition; System Snapshot memakai deterministic signed rules, bukan AI/LLM | Non-Tujuan dan FR-078 |
| UI/UX principles | Diterima dan diperluas: clean macOS-like visual hierarchy dengan Windows-like tables, filters, context menus, bulk actions, shortcuts, advanced panels, dan command palette | Bagian 8 |
| Plugin-ready | Diadaptasi menjadi signed validated acquisition module; arbitrary plugin dilarang | FR-116 |
| ISO/NIST compliant | Diubah menjadi traceable/aligned support; tidak ada klaim sertifikasi otomatis | Bagian 15 |
