# Code - Desktop

## Tujuan
- Implementasi slice terkecil yang bernilai dan sesuai RFC Desktop.

## Input Wajib
- PRD Desktop yang disetujui
- RFC Desktop yang disetujui
- Task ID, risk class, dan acceptance criteria
- Target platform serta capability boundary
- Kode existing yang relevan

## Scope Implementasi
- Fitur utama:
- File yang boleh disentuh:
- File yang tidak boleh disentuh:

## Aturan
- Ikuti arsitektur RFC.
- Jangan tambah fitur di luar MVP.
- Prioritaskan perubahan kecil dan mudah di-review.
- Gunakan pattern native desktop yang sudah ada di repo.

## Rencana Kerja
1. Identifikasi titik masuk kode.
2. Implementasi model data / boundary.
3. Implementasi UI atau shell desktop.
4. Tambah validasi dan error handling.
5. Tambah test.
6. Jalankan verifikasi.

## Definisi Selesai
- [ ] Portable core/contract checks lulus pada macOS, Windows, dan Linux sejauh runner mendukung
- [ ] Fitur platform-specific berjalan pada target OS yang tercantum dalam task
- [ ] Platform yang belum diuji dilaporkan sebagai `NotValidated`, bukan diasumsikan didukung
- [ ] Test relevan lulus
- [ ] Type-check lulus
- [ ] Tidak ada capability claim di luar evidence
- [ ] Commit hash, perintah verifikasi, hasil, dan limitation dicatat untuk handoff

## Verifikasi
- Test fokus:
- Type-check:
- Smoke test:
- Packaging check:
- Target OS/build/architecture:
- Hardware/fixture:
- Commit hash yang diuji:

## Catatan Implementasi
- Keputusan penting:
- Tradeoff:
- Risiko tersisa:
