# Riset Lanskap Akuisisi Barang Bukti Digital

**Tanggal riset:** 16 Juli 2026  
**Tujuan:** Menjadi dasar PRD aplikasi desktop portable, offline, lintas Windows/macOS/Linux untuk pengumpulan dan akuisisi barang bukti digital yang selaras dengan ISO/IEC 27037.

## 1. Kesimpulan Eksekutif

Tidak ada satu executable universal yang dapat mengakuisisi semua HDD/SSD, RAM, jaringan, dan artefak pada seluruh versi Windows, macOS, dan Linux secara forensically sound. Produk industri menyelesaikannya dengan beberapa pola yang digabungkan:

1. Aplikasi portable untuk live acquisition dan targeted collection.
2. Media boot forensik untuk dead-box acquisition dan menghindari penulisan ke media sumber.
3. Helper atau driver privileged yang ditandatangani untuk akses RAM, raw disk, dan packet capture.
4. Hardware write blocker atau forensic imager untuk kontrol sumber yang paling kuat.
5. Hash, verifikasi, log, metadata kasus, dan laporan untuk membuktikan proses.
6. Tool khusus yang berbeda untuk disk, RAM, jaringan, remote endpoint, dan media rusak.

Peluang produk terbesar bukan sekadar menaruh semua utilitas dalam satu USB. Pain-point yang belum terselesaikan secara konsisten adalah **pengambilan keputusan dan dokumentasi end-to-end**: menentukan keadaan perangkat, urutan volatilitas, metode paling sedikit mengubah sumber, kewenangan dan batas koleksi, dampak tindakan, verifikasi, chain of custody, serta paket preservasi yang dapat diaudit.

Rekomendasi awal adalah produk **Forensic Acquisition Orchestrator** dengan satu UX dan model kasus, tetapi memiliki aplikasi native portable untuk setiap OS, mode live, mode boot, dan integrasi hardware write blocker. Produk harus mengotomatisasi kontrol proses tanpa menyembunyikan tindakan teknis dari pemeriksa. Pemeriksaan dan analisis forensik berada pada aplikasi terpisah dan tidak menjadi tanggung jawab produk ini.

## 2. Batas Riset

“Semua aplikasi” secara literal tidak dapat dicakup karena jumlah produk, edisi, dan tool internal sangat besar. Riset ini mencakup produk representatif dari setiap pola akuisisi utama:

- imaging disk komersial dan gratis;
- live/dead-box serta targeted collection;
- media boot forensik;
- remote dan offline endpoint collector;
- RAM acquisition lintas platform;
- network capture;
- hardware imager dan write blocker;
- validasi tool dan preservasi bukti.

Klaim kemampuan produk diambil dari dokumentasi vendor/proyek. Klaim kepatuhan ISO tidak diterima hanya berdasarkan pemasaran vendor.

## 3. Baseline Standar

### 3.1 Standar yang diminta

- [ISO/IEC 27037:2012](https://www.iso.org/standard/44381.html) adalah edisi terbit terbaru untuk identifikasi, koleksi, akuisisi, dan preservasi bukti digital. Pada saat riset, ISO menandainya masih terbit tetapi akan direvisi.
- [ISO/IEC 17025:2017](https://www.iso.org/standard/66912.html) adalah edisi ketiga dan masih current setelah dikonfirmasi pada 2023. Standar ini mengatur kompetensi, ketidakberpihakan, dan operasi konsisten laboratorium pengujian/kalibrasi.
- [ISO/IEC 17043:2023](https://www.iso.org/standard/80864.html) adalah edisi kedua untuk kompetensi dan ketidakberpihakan penyelenggara proficiency testing.

Implikasi produk: aplikasi dapat menyediakan kontrol, rekaman, validasi, dan ekspor yang **mendukung** sistem mutu ISO/IEC 17025 dan program proficiency testing ISO/IEC 17043. Aplikasi tidak dapat membuat organisasi otomatis terakreditasi atau compliant.

### 3.2 Standar pendamping yang direkomendasikan

- [ISO/IEC 27041:2015](https://www.iso.org/standard/44405.html): assurance bahwa metode investigasi fit for purpose.
- [ISO/IEC 27042:2015](https://www.iso.org/standard/44406.html): analisis dan interpretasi yang memperhatikan continuity, validity, reproducibility, dan repeatability.
- [ISO/IEC 27043:2015](https://www.iso.org/standard/44407.html): prinsip dan proses investigasi insiden dari persiapan sampai penutupan.

Untuk matriks klausul normatif yang lengkap, tim produk harus memiliki salinan ISO berlisensi. PRD tidak boleh mengklaim clause-by-clause compliance hanya dari abstrak publik.

## 4. Bagaimana Produk Forensik Melakukan Akuisisi

| Produk/pola | Cara kerja utama | Kontrol integritas dan workflow | Batas yang relevan |
|---|---|---|---|
| [Exterro FTK Imager](https://www.exterro.com/digital-forensics-software/ftk-imager) | Imaging disk/volume, preview, custom content image, ekspor, dan capture RAM pada Windows | Hash verification dan preservasi image | Fokus utama Windows; keputusan kasus dan chain of custody tetap banyak bergantung pada operator/SOP |
| [Magnet Acquire](https://www.magnetforensics.com/resources/magnet-acquire/) | Full atau targeted acquisition untuk komputer/removable media dan perangkat mobile | Activity logging, dokumentasi metode, dan targeted collection | Akuisisi RAM Windows ditangani produk lain seperti DumpIt; cakupan metode berbeda menurut platform/perangkat |
| [Cellebrite Digital Collector](https://cellebrite.com/en/digital-collector/) | Live/dead-box imaging, triage, targeted collection Windows/macOS; tersedia sebagai USB self-contained | Hash, write protection, tujuan bukti khusus, dukungan encrypted/decrypted collection pada skenario tertentu | Dukungan bergantung model/versi OS; Linux bukan fokus utama; solusi berbasis perangkat dan lisensi khusus |
| [X-Ways Forensics](https://www.x-ways.net/forensics/) | Portable Windows dari USB; disk cloning/imaging dan logical acquisition ke evidence container | Case management dan preservasi metadata pada logical collection | Sangat Windows-centric; full physical RAM bukan kapabilitas utama yang dipasarkan |
| [Belkasoft X/T/R](https://belkasoft.com/x) | Imaging DD/E01, targeted triage, RAM Windows, dan remote acquisition melalui produk berbeda | Hash opsional/verifikasi dan evidence container | Kapabilitas tersebar pada beberapa produk; RAM remote dibatasi Windows pada dokumentasi produk |
| [Guymager](https://guymager.sourceforge.io/) + [CAINE](https://www.caine-live.net/) | Imaging dd/EWF/AFF dari live Linux; CAINE bootable dan memblokir disk read-only secara default | Hash selama imaging, info akuisisi, serta software write-blocking | Membutuhkan root; Guymager memiliki banyak library sistem; software write block tidak menggantikan hardware write blocker untuk semua kebijakan lab |
| [SUMURI PALADIN](https://sumuri.com/product/paladin-lts/) | Live/bootable Linux untuk imaging, triage, dan reporting | Workflow dalam boot environment | Kompatibilitas boot dan hardware modern tetap menjadi variabel; bukan live-RAM collector universal |
| [Atola TaskForce](https://atola.com/products/tf1/) | Appliance untuk imaging paralel, media sehat/rusak, RAID, dan remote iSCSI | Setiap tindakan masuk case log; linear/segmented hashing; source dan image dapat diverifikasi | Hardware khusus dan mahal; bukan pengganti live volatile acquisition pada host aktif |
| [OpenText Tableau TX1](https://security.opentext.com/docs/default-source/document-library/user-guide/opentext-tableau-tx1-user-guide-21-3.pdf) | Hardware forensic imager dengan write-blocking terintegrasi | Kontrol fisik source dan workflow imaging; tersedia hasil pengujian CFTT | Hardware khusus; fokus media storage, bukan RAM/jaringan |
| [Velociraptor Offline Collector](https://docs.velociraptor.app/docs/deployment/offline_collections/) | Targeted artifact collection lintas platform ke container ZIP lokal | Log lokal dan SHA-256 container; tool dapat dibundel | macOS code signing membatasi repack binary; offline collector tidak memberi telemetri/progress ke server; bukan full disk imager |
| [KAPE](https://www.kroll.com/en/publications/cyber/kroll-artifact-parser-extractor-kape) | Targeted file collection dan processing berbasis konfigurasi Targets/Modules | Konfigurasi dapat diulang dan koleksi cepat | Fokus Windows dan triage; module dapat memanggil tool tambahan sehingga dependency/licensing/validation terfragmentasi |
| [F-Response](https://www.f-response.com/) | Menyediakan read-only remote access ke disk, volume, RAM, cloud, dan shares agar tool forensik lain dapat membaca | Memisahkan akses remote dari tool imaging/analysis | Bergantung jaringan, endpoint component, dan tool imaging eksternal; tidak full offline |
| [WinPmem](https://github.com/Velocidex/WinPmem) | Executable Windows self-contained memuat driver RAM yang sesuai lalu menghasilkan RAW | Signed binary, driver di-unload setelah capture, userspace dapat melakukan hash/stream | Driver/kernel compatibility dan kebijakan security OS; proyek upstream memiliki siklus rilis sendiri |
| [AVML](https://github.com/microsoft/avml) | Static Linux userland binary membaca `/proc/kcore`, `/dev/crash`, atau `/dev/mem` | Dapat menyimpan/stream snapshot dan memakai format LiME | Gagal ketika kernel lockdown aktif; x86_64 dan sumber memori OS tertentu |
| [Linpmem](https://github.com/Velocidex/Linpmem) / [LiME](https://github.com/jtsylve/LiME) | Kernel module Linux untuk membaca memori fisik | Akses level kernel dan output yang cocok untuk memory forensics | Harus cocok dengan kernel/header/signature; Secure Boot dapat memblokir; Linpmem masih menyebut batas dan fitur yang belum selesai |
| [Volexity Surge](https://www.volexity.com/products-overview/surge/) | Akuisisi RAM Windows, Linux, dan macOS | UX terarah untuk mengurangi crash/corrupt capture | Komersial dan tetap tunduk pada pembatasan keamanan/versi platform, khususnya macOS |
| [Wireshark dumpcap](https://www.wireshark.org/docs/man-pages/dumpcap.html) | Capture packet ke pcapng/pcap; capture dipisah ke proses privileged | Privilege separation dan metadata timestamp/capture comment | Membutuhkan libpcap/Npcap serta hak raw network; capture hanya melihat trafik yang tersedia di interface/tap tersebut |

## 5. Pola Arsitektur Industri

### 5.1 Portable host application

Executable dijalankan dari USB pada OS aktif. Ini cocok untuk RAM, encryption keys, process/network state, targeted artifacts, dan disk logis yang hanya dapat dibaca ketika sistem hidup. Konsekuensinya, executable dan OS akan mengubah sebagian state sumber; setiap perubahan harus diminimalkan dan dicatat.

### 5.2 Bootable forensic environment

Komputer target di-boot dari media forensik. Disk internal diperlakukan read-only dan di-image ke media tujuan. Pola ini mengurangi penulisan dari OS target, tetapi tidak mempertahankan RAM yang sudah hilang karena reboot dan dapat terhalang Secure Boot, firmware password, Apple security policy, atau hardware baru.

### 5.3 Hardware write blocker / dedicated imager

Media dilepas atau dihubungkan melalui perangkat yang mengizinkan baca dan menolak tulis. Ini memberi kontrol paling kuat untuk dead-box storage, termasuk workflow media rusak, tetapi tidak menyelesaikan volatile evidence, encrypted live volume, atau perangkat dengan storage tersolder.

### 5.4 Targeted collection

Collector mengambil file dan artefak relevan berdasarkan profil, rentang waktu, pengguna, atau jenis data. Pola ini mengurangi waktu dan kapasitas, tetapi menambah risiko selection bias dan bukti ekskulpatif terlewat. [SWGDE focused collection](https://www.swgde.org/documents/published-complete-listing/14-f-003-focused-collection-and-examination-of-digital-evidence/) meminta alasan, batas, tool, metode, dan eksklusi didokumentasikan agar dapat dipertanggungjawabkan.

### 5.5 Remote collection

Agent/temporary collector membuka disk, volume, RAM, atau artefak melalui jaringan. Ini membantu sistem yang tidak boleh dimatikan dan lokasi jauh, tetapi menambah risiko jaringan putus, autentikasi, bandwidth, source changes, dan chain of custody saat transit.

## 6. Pemetaan ISO/IEC 27037 ke Kebutuhan Produk

### 6.1 Identification

Produk perlu:

- membuat identitas kasus, otorisasi, ruang lingkup, dan tujuan akuisisi sebelum membaca sumber;
- merekam keadaan perangkat: hidup, mati, sleep, locked, encrypted, terhubung jaringan, dan gejala destructive activity;
- menginventarisasi perangkat serta sumber data dengan make/model/serial/asset tag, kapasitas, interface, volume, partisi, dan identifier OS;
- memandu foto/screenshot dan pencatatan tampilan layar, kabel, port, waktu perangkat, zona waktu, serta offset dari waktu referensi;
- menyusun urutan volatilitas dan rekomendasi live/dead/targeted/physical tanpa mengambil keputusan hukum menggantikan pemeriksa;
- mencatat alasan memasukkan atau mengecualikan setiap sumber potensial.

### 6.2 Collection

Produk perlu:

- menyediakan checklist keselamatan, otorisasi, anti-forensics, enkripsi, dan bukti tradisional sebelum menyentuh perangkat;
- merekam siapa mengumpulkan apa, kapan, di mana, bagaimana, dan kondisi item;
- memberi identifier unik serta label/QR offline untuk device, media tujuan, image, dan container;
- mencatat keputusan isolasi jaringan, cabut daya, pertahankan daya, atau boot dari media forensik beserta rasionalnya;
- mendukung chain of custody kontemporer, termasuk transfer, segel, kondisi, penerima, dan tanda tangan;
- tidak mengizinkan langkah acquisition dimulai sebelum field minimum yang diwajibkan kebijakan kasus terpenuhi, kecuali emergency override yang tercatat.

### 6.3 Acquisition

Produk perlu:

- memilih physical, logical, targeted, live, dead-box, atau remote berdasarkan keadaan dan tujuan kasus;
- memakai metode paling sedikit invasif dan menampilkan expected source changes sebelum operator menyetujui;
- memverifikasi write blocker atau status read-only sebelum imaging jika skenario mendukungnya;
- melakukan preflight sumber/tujuan, ruang kosong, filesystem tujuan, batas ukuran file, listrik, suhu, dan estimasi waktu;
- memperoleh disk/volume/file/RAM/network state dalam urutan volatilitas yang dapat dikonfigurasi;
- menghitung SHA-256 minimal saat akuisisi serta verifikasi independen setelah penulisan; algoritma tambahan hanya untuk interoperabilitas;
- menangani bad sector, retry policy, timeout, HPA/DCO/AMA, disk 4Kn, sparse areas, RAID, dan media rusak secara eksplisit;
- mencatat tool build, komponen, driver, konfigurasi, command/effective parameters, start/end time, error, retry, skip, cancellation, dan operator override;
- menghasilkan hasil parsial yang dapat diidentifikasi dan tidak pernah menyebutnya “berhasil penuh” bila coverage tidak lengkap;
- memisahkan acquisition evidence dari preview/triage result agar scope dan provenance tetap jelas.

### 6.4 Preservation

Produk perlu:

- membuat evidence package berisi image/container, manifest, hash, log append-only, metadata kasus, chain of custody, foto/screenshot, error map, dan laporan metode;
- menandatangani manifest secara digital dengan identitas operator/perangkat yang dapat diverifikasi offline;
- mendukung evidence master, verified working copy, dan archive copy dengan hubungan provenance yang eksplisit;
- melakukan scheduled fixity check tanpa mengubah evidence master;
- merekam setiap akses, copy, export, verify, dan transfer;
- menggunakan format interoperabel dan mendokumentasikan spesifikasinya;
- menyediakan export PDF/A dan JSON/CSV yang dapat dibaca tanpa aplikasi, tetapi menjaga manifest mesin sebagai sumber data terstruktur;
- menolak modifikasi evidence master melalui aplikasi dan memperingatkan jika filesystem tujuan tidak mendukung kebutuhan preservasi.

## 7. Pain-point Ahli Digital Forensik

### 7.1 Tool fragmentation

Disk, RAM, jaringan, targeted artifacts, remote endpoint, dan chain of custody sering berada pada tool yang berbeda. Operator harus menyelaraskan case ID, waktu, hash, dan log secara manual.

### 7.2 Keputusan volatile-vs-preservation di bawah tekanan

Mematikan perangkat menghilangkan RAM dan kemungkinan encryption key; mempertahankan sistem hidup mengubah state. Banyak tool melakukan capture, tetapi sedikit yang membantu mencatat alasan, alternatif, dan dampak metode secara konsisten.

### 7.3 Portabilitas berlawanan dengan security platform

Raw disk, RAM, dan packet capture membutuhkan privilege. Windows memerlukan driver yang dapat dimuat; macOS menuntut code signing dan kebijakan keamanan ketat; Linux dapat memblokir akses melalui Secure Boot/kernel lockdown. “Tanpa instalasi” tidak sama dengan “tanpa komponen privileged sementara”.

### 7.4 Enkripsi dan storage modern

FileVault, BitLocker, encrypted Linux volumes, storage tersolder, T2/Apple Silicon, RAID, NVMe, 4Kn, HPA/DCO, dan TRIM membuat dead-box bitstream tidak selalu menjadi metode terbaik atau bahkan mungkin.

### 7.5 Data terlalu besar

[NIST](https://www.nist.gov/digital-evidence) menyoroti kesulitan menemukan bukti di antara data dalam jumlah besar. SWGDE juga menyatakan kapasitas multi-terabyte dapat membuat koleksi/review menyeluruh tidak praktis. Targeted collection diperlukan, tetapi harus defensible dan tidak menyembunyikan risiko eksklusi.

### 7.6 Hash tidak membuktikan coverage

Hash membuktikan byte yang disimpan tidak berubah; hash tidak dengan sendirinya membuktikan seluruh sektor yang seharusnya dibaca telah diambil. Hasil [CFTT](https://www.nist.gov/itl/csd/secure-systems-and-applications/computer-forensics-tool-testing-program-cftt) menunjukkan pentingnya pengujian fungsi, hidden area, defective sector, format, dan konfigurasi.

### 7.7 Validasi cepat kedaluwarsa

OS, kernel, driver, filesystem, firmware, dan hardware berubah. Menurut [SWGDE minimum tool testing](https://www.nist.gov/document/swgde-18-q-001-10-minimum-requirements-testing-tools-used-digital-and-multimedia-forensics), imaging tool harus diuji sebelum dipakai, setelah repair, dan setelah update relevan; konfigurasi serta anomali harus dicatat.

### 7.8 Chain of custody berada di luar acquisition tool

Banyak imager menghasilkan log teknis, tetapi perpindahan fisik, segel, foto, otorisasi, penerima, dan working-copy lineage masih dikerjakan di formulir terpisah.

### 7.9 Error handling tidak seragam

Bad sector, power loss, target full, network interruption, driver rejection, atau capture parsial dapat menghasilkan bukti yang tetap berguna. Tool perlu membedakan success, verified partial, failed, aborted, dan resumed secara eksplisit.

### 7.10 Ketergantungan dan lisensi sulit diaudit

Toolkit yang memanggil banyak executable meningkatkan risiko versi yang tidak konsisten, dependency hilang, lisensi tidak kompatibel, dan bukti validasi yang terpecah.

## 8. Rekomendasi Produk

### 8.1 Pilihan arsitektur

**Pendekatan A — satu aplikasi host-only:** paling sederhana, tetapi tidak dapat menjamin full disk/RAM pada semua platform dan tidak memadai untuk perangkat mati.

**Pendekatan B — aplikasi native portable per OS + forensic boot media:** direkomendasikan dan telah dipilih. Build Windows, macOS, dan Linux menggunakan case model, audit schema, evidence package, serta verifier yang sama. Implementasi akses perangkat dan privilege tetap native untuk masing-masing OS.

**Pendekatan C — dedicated hardware appliance:** kontrol storage terbaik dan cocok untuk paralel/damaged media, tetapi biaya dan pengembangan hardware tinggi. Dapat menjadi fase lanjutan atau integrasi.

### 8.2 Arti “full offline tanpa dependency” yang aman

Requirement sebaiknya didefinisikan sebagai:

> Produk dapat dijalankan dan menyelesaikan fungsi yang didukung tanpa internet, package manager, runtime, driver, atau aplikasi lain yang sebelumnya harus terpasang pada komputer target.

Ini tidak sama dengan melarang seluruh third-party source code. Menulis ulang library kriptografi, filesystem, compression, EWF/AFF, packet capture, dan driver dari nol meningkatkan risiko cacat serta beban validasi. Rekomendasi:

- bundle atau static-link dependency yang lisensinya kompatibel;
- pin source dan toolchain, simpan SBOM, hash, license inventory, dan provenance;
- gunakan reproducible builds dan tanda tangan rilis offline;
- fork/maintain komponen kritis bila upstream tidak stabil;
- implementasikan ulang hanya ketika lisensi, keamanan, atau kebutuhan validasi memang menuntut;
- uji setiap modul sebagai bagian dari build produk, bukan mengandalkan reputasi dependency.

### 8.3 Diferensiasi utama

1. **ISO-guided acquisition wizard** yang memblokir kelalaian dokumentasi tanpa mengambil alih professional judgment.
2. **Decision ledger** untuk semua keputusan live/dead, scope, volatility, network isolation, write blocking, error, dan deviasi.
3. **Source-change accounting** yang menjelaskan perubahan yang diperkirakan dan yang teramati pada live acquisition.
4. **Unified evidence package** lintas disk, RAM, network, dan targeted artifacts.
5. **Offline validation center** dengan known datasets, regression tests, build qualification, dan evidence pack untuk ISO/IEC 17025.
6. **Proficiency-testing mode** untuk membuat, mendistribusikan, mengunci, menilai, dan melaporkan exercise secara offline guna mendukung ISO/IEC 17043.
7. **Capability honesty:** sebelum mulai, aplikasi menyatakan apa yang dapat/tidak dapat diperoleh pada OS/hardware tersebut dan tidak melakukan silent fallback.
8. **Independent verifier:** executable kecil terpisah untuk memverifikasi manifest, signature, hash, coverage map, dan package structure tanpa membuka evidence.

## 9. Scope MVP yang Direkomendasikan

MVP sebaiknya fokus pada akuisisi lokal satu komputer per sesi:

- Windows 10/11 x64 dan ARM64 yang masih didukung vendor OS;
- macOS pada Intel dan Apple Silicon dengan capability matrix eksplisit;
- Linux x86_64 dengan distro/kernel support matrix;
- physical/logical disk imaging untuk media yang dapat dibaca;
- RAM capture bila platform state mengizinkan;
- live system state dan network snapshot/capture;
- targeted artefact profiles untuk Windows/macOS/Linux;
- bootable x86_64 forensic environment;
- case metadata, chain of custody, evidence manifest, hashing, signing, verification, dan report;
- integrasi hardware write blocker melalui detection/status, bukan membuat hardware sendiri pada MVP.

Mobile, cloud acquisition, remote fleet-scale collection, pemeriksaan dan analisis forensik, password cracking, serta hardware recovery media rusak berada di luar MVP. Hasil produk harus dapat diserahkan melalui format evidence package terdokumentasi kepada aplikasi analisis terpisah tanpa mengubah evidence master.

## 10. Sumber Praktik dan Validasi

- [SWGDE Best Practices for Computer Forensic Acquisitions](https://www.swgde.org/documents/published-complete-listing/17-f-002-best-practices-for-computer-forensic-acquisitions/): minimalkan perubahan sumber, dokumentasikan dampak, gunakan write-blocking, dan validasi tool.
- [SWGDE Best Practices for Digital Evidence Collection](https://www.swgde.org/documents/published-complete-listing/18-f-002-best-practices-for-digital-evidence-collection/): keadaan perangkat, live collection, integrity, chain of custody, dan working/archive copy.
- [NIST SP 800-86](https://csrc.nist.gov/pubs/sp/800/86/final): urutan volatilitas, keputusan live system, dan tool mandiri yang telah dipersiapkan.
- [NIST CFTT](https://www.nist.gov/itl/csd/secure-systems-and-applications/computer-forensics-tool-testing-program-cftt): spesifikasi, metode uji, test set, dan laporan kemampuan/anomali tool.
- [NIST Federated Testing](https://www.nist.gov/itl/csd/secure-systems-and-applications/computer-forensics-tool-testing-program-cftt/federated): lingkungan uji disk imaging, write blocking, dan fungsi forensik lain yang dapat dijalankan laboratorium.

## 11. Keputusan yang Dibutuhkan Sebelum PRD Final

Keputusan yang telah dikonfirmasi:

- Produk memiliki aplikasi native portable untuk Windows, macOS, dan Linux.
- Ketiga aplikasi memakai shared core dengan adapter akses perangkat dan privilege yang native untuk masing-masing OS.
- Produk hanya mencakup tahap pengumpulan dan akuisisi; pemeriksaan dan analisis forensik dibuat sebagai aplikasi terpisah.

Pertanyaan lanjutan:

1. Apakah produk boleh memakai dependency open source yang dibundel/static-linked dan divalidasi, atau harus clean-room implementation untuk komponen tertentu?
2. Apakah pengguna utama penegak hukum, laboratorium pemerintah, konsultan forensik, tim DFIR enterprise, atau gabungan?
3. Yurisdiksi dan bahasa laporan apa yang harus didukung pertama?
4. Apakah mode proficiency testing ISO/IEC 17043 harus masuk MVP atau fase setelah acquisition core stabil?
