# PRD - Website

## 1. Ringkasan
- **Nama produk:** Trareon Acquire
- **Platform target:** web desktop, tablet, mobile
- **Versi dokumen:** 0.1.0
- **Status:** Authored for marketing site (not a web shell clone)

## 2. Masalah yang Ingin Diselesaikan
- Lab operators and evaluators need a public landing page that states what Acquire is, its lab/unsigned limits, and how to get docs or builds.
- Siapa yang terdampak: DFIR lab leads, evaluators, open-source contributors
- Dampak: clearer expectations before download; less “court-ready” misunderstanding

## 3. Tujuan
- Brand-first hero for Trareon Acquire
- One CTA to docs / GitHub
- Explicit lab-use / UNSIGNED disclosure

## 4. Non-Tujuan
- Web clone of the Slint desktop shell
- Auth, billing, or SaaS console
- Production evidence claims

## 5. Ruang Lingkup
### In scope
- Static marketing site (Netlify)
- EN primary copy; optional ID note
- Disclosure banner matching product honesty UX

### Out of scope
- Interactive acquire workflow in browser
- Account systems

## 6. User Story
- Sebagai evaluator, saya ingin memahami batas lab sebelum mengunduh, supaya tidak salah klaim.
- Sebagai operator, saya ingin satu tautan ke panduan dan repositori.

## 7. Kebutuhan Fungsional
- Landing: brand, one headline, one supporting sentence, one CTA group
- Disclosure: Lab use only · NOT court-ready · NOT ISO-certified
- Footer: GPLv3, GitHub link

## 8. Kebutuhan Website Khusus
- Responsif: yes
- SEO / metadata: title + description
- Routing: single page (+ optional /docs redirect)
- Auth / session: none
- Browser support: modern evergreen
- Accessibility: keyboard focus, contrast on light SoT

## 9. Kebutuhan Non-Fungsional
- Performa: static assets only
- Keamanan: no forms collecting PII in v0.1
- Aksesibilitas: WCAG-oriented software floor; human review separate

## 10. Edge Case
- Deep link to old ECR screenshots must not imply current chrome

## 11. Acceptance Criteria
- [x] PRD filled from product README + DESIGN
- [x] Static site builds for Netlify
- [x] Disclosure visible without scroll on desktop

## 12. Metrik Keberhasilan
- Visitors reach docs/GitHub in one click from hero CTA

## 13. Risiko
- Marketing screenshots stale vs live Slint — refresh after GUI captures

## 14. Open Questions
- Hosted download binaries vs GitHub Releases only (default: GitHub)
