import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "..");
const output = resolve(root, "docs/ai-operations/MONTH-01");
mkdirSync(output, { recursive: true });

const days = [
  [1,"M0-D01","Baseline Git dan GitHub monitoring","Claude Code","Codex","Medium","AUTO_WITH_REVIEW","feat/m0-d01-baseline","Verifikasi baseline Git, origin, worktree policy, Project/Issue/PR monitoring, pinned toolchains, dan CI skeleton tanpa menginisialisasi ulang repository.",".gitignore; rust-toolchain.toml; Cargo.toml; package.json; .github/workflows/ci.yml; GitHub Project/Issue","git status --short --branch; git remote -v; shasum -a 256 -c docs/RFC-BASELINE.sha256","baseline/remote benar, toolchain pin eksplisit, workflow least privilege, monitoring exact-SHA","NO_RUN_NOT_VISUAL"],
  [2,"M0-D02","Workspace dan core boundary","Claude Code","Codex","Medium","AUTO_WITH_REVIEW","feat/m0-d02-workspace","Buat Rust workspace, trareon-core boundary, build identity, dan failing public-boundary test sesuai Foundation Plan Task 1.","Cargo.toml; crates/trareon-core/Cargo.toml; crates/trareon-core/src/lib.rs; crates/trareon-core/tests/foundation.rs","cargo test --workspace --all-targets; cargo fmt --all -- --check; cargo clippy --workspace --all-targets -- -D warnings","core bebas Tauri/frontend, dependency exact, failing test dibuktikan sebelum implementasi","NO_RUN_NOT_VISUAL"],
  [3,"M0-D03","Domain identifiers dan capability types","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d03-domain","Implementasikan ID, capability state, acquisition status, dan error/result types dari Foundation Plan Task 2 dengan serialization tests.","crates/trareon-core/src/domain.rs; crates/trareon-core/src/lib.rs; crates/trareon-core/tests/domain.rs","cargo test -p trareon-core --test domain; cargo test --workspace --all-targets","type tidak ambigu, unsupported capability eksplisit, tidak ada status complete palsu","NO_RUN_NOT_VISUAL"],
  [4,"M0-D04","State transition engine","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d04-state","Lengkapi allowed transition table dan negative tests untuk invalid, cancel, fail, incomplete, verification, serta terminal states.","crates/trareon-core/src/domain.rs; crates/trareon-core/tests/domain.rs","cargo test -p trareon-core --test domain; cargo test --workspace --all-targets","semua forbidden transition fail-closed dan terminal state tidak dapat dimutasi","NO_RUN_NOT_VISUAL"],
  [5,"M0-D05","Append-only audit hash chain","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d05-audit","Implementasikan audit events dan hash chain dari Foundation Plan Task 3 dengan mutation, removal, reorder, dan discontinuity tests.","crates/trareon-core/src/audit.rs; crates/trareon-core/src/lib.rs; crates/trareon-core/tests/audit.rs","cargo test -p trareon-core --test audit; cargo test --workspace --all-targets","canonical event bytes, previous-hash continuity, tamper selalu terdeteksi","NO_RUN_NOT_VISUAL"],
  [6,"M0-D06","Canonical serialization dan hashing","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d06-canonical","Bekukan canonical field ordering/version profile dan buktikan repeated-run equality, changed-field inequality, serta unsupported-version failure.","crates/trareon-core/src/audit.rs; crates/trareon-core/tests/audit.rs; docs cryptographic profile","cargo test -p trareon-core --test audit -- --nocapture; cargo fmt --all -- --check","deterministik lintas repeated run; tidak mengandalkan map iteration atau locale/timezone","NO_RUN_NOT_VISUAL"],
  [7,"M0-D07","Week 1 integration gate","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d07-integration","Integrasikan Days 01-06, jalankan clean checks dua kali, periksa dependency boundary, dan buat discrepancy register.","workspace; Cargo.lock; Week 1 evidence and discrepancy report","cargo test --workspace --all-targets; cargo test --workspace --all-targets; cargo fmt --all -- --check; cargo clippy --workspace --all-targets -- -D warnings","dua run identik, no P0/P1, diff tetap sesuai scope, baseline RFC tidak berubah","NO_RUN_NOT_VISUAL"],
  [8,"M0-D08","Synthetic streaming acquisition","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d08-streaming","Implementasikan file-backed streaming acquisition dari Foundation Plan Task 4 dengan byte-for-byte oracle dan streaming SHA-256.","crates/trareon-core/src/acquisition.rs; crates/trareon-core/tests/acquisition.rs; crates/trareon-core/src/lib.rs","cargo test -p trareon-core --test acquisition; cargo test --workspace --all-targets","output byte-identik, hash oracle independen, memory tidak membaca seluruh source sekaligus","NO_RUN_NOT_VISUAL"],
  [9,"M0-D09","Source destination identity guard","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d09-identity","Tambahkan preflight identity, same-source/destination rejection, alias/symlink containment, dan no-output-on-failed-preflight.","crates/trareon-core/src/acquisition.rs; crates/trareon-core/tests/acquisition.rs","cargo test -p trareon-core --test acquisition -- --nocapture","reversal/alias ditolak sebelum write dan rejection diaudit","NO_RUN_NOT_VISUAL"],
  [10,"M0-D10","Cancel interruption dan failure semantics","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d10-failure","Uji cancel, read failure, destination-full simulation, interrupted artifact, retry boundary, dan larangan false complete.","crates/trareon-core/src/acquisition.rs; crates/trareon-core/tests/acquisition.rs","cargo test -p trareon-core --test acquisition; cargo test --workspace --all-targets","setiap failure menghasilkan explicit non-complete state, coverage/gap dan audit record","NO_RUN_NOT_VISUAL"],
  [11,"M0-D11","RAW dan split-RAW contract","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d11-raw","Implementasikan RAW dan split-RAW file-backed output dengan boundary-size, segment-order, final-short-segment, zero-length, dan reassembly tests.","crates/trareon-core/src/acquisition.rs; crates/trareon-core/tests/acquisition.rs","cargo test -p trareon-core --test acquisition -- --nocapture","reassembly byte-identik dan segment metadata deterministik","NO_RUN_NOT_VISUAL"],
  [12,"M0-D12","fsnap manifest schema","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d12-manifest","Definisikan schema/version, source identity, segments, hashes, coverage, gaps, audit reference, tool identity, dan validation errors.","schemas/fsnap-manifest-v1.schema.json; crates/trareon-core/src/package.rs; crates/trareon-core/tests/package.rs","cargo test -p trareon-core --test package; cargo test --workspace --all-targets","schema menolak required-field loss, invalid path/hash/version dan tidak mengklaim complete tanpa coverage","NO_RUN_NOT_VISUAL"],
  [13,"M0-D13","Safe fsnap package writer","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d13-package","Implementasikan safe package writer dari Foundation Plan Task 5 dengan traversal, duplicate, missing, mutation, dan audit-discontinuity fixtures.","crates/trareon-core/src/package.rs; crates/trareon-core/tests/package.rs; schemas/fsnap-manifest-v1.schema.json","cargo test -p trareon-core --test package -- --nocapture","path containment fail-closed, package partial tidak dianggap verified","NO_RUN_NOT_VISUAL"],
  [14,"M0-D14","Independent verifier","Codex","Claude Code","High","AUTO_WITH_REVIEW","feat/m0-d14-verifier","Buat trareon-verifier tanpa dependency pada writer crate/Tauri; verifikasi schema, files, sizes, hashes, audit continuity, coverage, dan terminal result.","crates/trareon-verifier/Cargo.toml; crates/trareon-verifier/src/main.rs; crates/trareon-verifier/tests/cli.rs; Cargo.toml","cargo tree -p trareon-verifier; cargo test -p trareon-verifier; cargo test --workspace --all-targets","independensi nyata, exit code berbeda untuk valid/invalid/unsupported, tidak memperbaiki package","NO_RUN_NOT_VISUAL"],
  [15,"M0-D15","Verifier CLI dan fsnap v0.1 draft","Codex","Claude Code","High","AUTO_WITH_REVIEW","feat/m0-d15-contract","Tambahkan CLI round-trip dan golden packages valid, mutated, truncated, removed-file, audit-discontinuous, unsupported-version; tulis read contract v0.1 draft.","crates/trareon-verifier/tests/cli.rs; fixtures/fsnap-v0.1; schemas; docs/fsnap-v0.1-read-contract.md","cargo test -p trareon-verifier -- --nocapture; cargo test --workspace --all-targets","golden fixture deterministik dan Analysis reader contract tidak mengizinkan silent repair","NO_RUN_NOT_VISUAL"],
  [16,"M0-D16","Typed Tauri command boundary","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d16-tauri","Implementasikan Tauri/Svelte shell dari Foundation Plan Task 7; UI hanya memanggil typed commands dan core menentukan final state.","apps/trareon-acquire/src-tauri; apps/trareon-acquire/src/lib/api.ts; apps/trareon-acquire/src/App.svelte; Cargo.toml","cargo test --workspace --all-targets; npm ci; npm run check; npm run build","main UI tidak elevated, no shell/arbitrary filesystem, error/state mapping lengkap","NO_RUN_NOT_VISUAL"],
  [17,"M0-D17","Guided synthetic acquisition UI","Claude Code","Codex","Medium","AUTO_WITH_REVIEW","feat/m0-d17-ui","Buat guided workflow synthetic source: case identity, preflight, confirmation, progress, result, verifier summary, dan limitation display.","apps/trareon-acquire/src/App.svelte; apps/trareon-acquire/src/lib/api.ts; UI tests","npm run check; npm run build; cargo test --workspace --all-targets","UI tidak mengarang success dan semua state berasal dari typed core response","VALIDATE_UI_FLOW"],
  [18,"M0-D18","In-app guidance dan accessibility","Claude Code","Codex","Medium","AUTO_WITH_REVIEW","feat/m0-d18-guidance","Tambahkan progressive guidance, keyboard/focus semantics, accessible labels, confirmation copy, Lab Use Only banner, dan NotValidated explanations.","apps/trareon-acquire/src/App.svelte; guidance content; accessibility tests","npm run check; npm run build","guided/standard/expert copy tidak mengubah semantics; destructive wording jelas","VALIDATE_ACCESSIBILITY"],
  [19,"M0-D19","Chain of Custody dan report preview","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d19-report","Render CoC/report preview hanya dari core data dan bedakan verified, incomplete, failed, cancelled, serta NotValidated.","core report types; typed command; report UI and tests","cargo test --workspace --all-targets; npm run check; npm run build","report provenance exact, no UI-only fields, limitation terlihat","VALIDATE_REPORT"],
  [20,"M0-D20","Cross-platform CI matrix","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d20-ci","Buat pinned hosted CI untuk Rust, TypeScript, fixtures, verifier, Windows/macOS/Linux compile, dan prompt-pack checks. Hosted CI bukan hardware proof.",".github/workflows/ci.yml; .github/workflows/build-it-for-me.yml; CI documentation","sh scripts/validate-ai-operations.sh; cargo test --workspace --all-targets; npm ci; npm run check; npm run build","least-privilege permissions, action pins direview, exact SHA checks, no secrets on untrusted PR","NO_RUN_NOT_VISUAL"],
  [21,"M0-D21","DevSecOps gates","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d21-devsecops","Tambahkan secret, dependency, license, SAST, SBOM, artifact, provenance-intent, dan policy checks tanpa auto-release.","CI security jobs; deny/license policy; SBOM config; SECURITY.md","cargo test --workspace --all-targets; npm audit --omit=dev; sh scripts/validate-ai-operations.sh","finding tidak disembunyikan, dependency baru high-risk meminta human approval","NO_RUN_NOT_VISUAL"],
  [22,"M0-D22","Property dan fuzz baseline","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d22-fuzz","Tambahkan bounded property/fuzz targets untuk state, canonical path, manifest, package containment, audit continuity, dan verifier input.","core property tests; fuzz targets/corpus documentation; CI bounded fuzz smoke","cargo test --workspace --all-targets","corpus synthetic, reproducer disimpan, parser tidak panic, resource bounds dicatat","NO_RUN_NOT_VISUAL"],
  [23,"M0-D23","Linux read-only feasibility","Claude Code","Codex","High","MANUAL_START","feat/m0-d23-linux","Pada Kali dan Ubuntu/Debian LTS, lakukan read-only enumeration/identity/permission feasibility terhadap virtual atau allowlisted lab media; jangan implementasikan production adapter.","Linux feasibility code behind capability boundary; platform report; limitation matrix","cargo test --workspace --all-targets; execute only approved read-only lab command","exact OS/device/security state tercatat; divergence menjadi NotValidated","NO_RUN_PLATFORM_REPORT"],
  [24,"M0-D24","Windows read-only feasibility","Claude Code","Codex","High","MANUAL_START","feat/m0-d24-windows","Pada trusted Windows commit, lakukan read-only enumeration/identity/privilege feasibility dan catat status dukungan exact OS/hardware; jangan menulis system disk.","Windows feasibility boundary; platform report; limitation matrix","cargo test --workspace --all-targets; execute only approved read-only lab command","X270 tidak menjadi satu-satunya production validation; unsupported combo NotValidated","NO_RUN_PLATFORM_REPORT"],
  [25,"M0-D25","macOS read-only feasibility","Claude Code","Codex","High","MANUAL_START","feat/m0-d25-macos","Pada Mac M4, lakukan read-only enumeration serta TCC/SIP/helper feasibility; tidak meminta privilege atau raw access tanpa approval.","macOS feasibility boundary; platform report; limitation matrix","cargo test --workspace --all-targets; execute only approved read-only lab command","arm64/security state tercatat; Intel dan untested helper path NotValidated","NO_RUN_PLATFORM_REPORT"],
  [26,"M0-D26","Performance baseline","Claude Code","Codex","High","AUTO_WITH_REVIEW","feat/m0-d26-performance","Ukur throughput, peak memory, cancellation latency, hashing/package overhead, dan repeated-run variance pada synthetic sizes; jangan optimasi sebelum baseline diterima.","benchmark harness; synthetic datasets; performance report","cargo test --workspace --all-targets; run documented benchmark twice","memory bounded, output/hash/audit equivalence, variance dan environment dicatat","NO_RUN_NOT_VISUAL"],
  [27,"M0-D27","Capability matrix dan fsnap freeze candidate","Codex","Claude Code","High","AUTO_WITH_REVIEW","feat/m0-d27-matrix","Konsolidasikan platform evidence, discrepancy register, capability/limitation matrix, dan `.fsnap` v0.1 compatibility freeze candidate.","docs capability matrix; discrepancy report; fsnap v0.1 contract/golden fixtures","cargo test --workspace --all-targets; sh scripts/validate-ai-operations.sh","hanya evidence-backed capability; breaking ambiguity memblokir freeze","NO_RUN_NOT_VISUAL"],
  [28,"M0-D28","User guide About dan limitations","Claude Code","Codex","Medium","AUTO_WITH_REVIEW","feat/m0-d28-docs","Buat user-guide skeleton, in-app help mapping, About dengan Trareon/Yusuf attribution, legal limitation draft, operator warnings, dan support matrix.","docs user guide; in-app help; About UI; limitation matrix links","npm run check; npm run build; sh scripts/validate-ai-operations.sh","legal text jelas sebagai draft, no certification claim, guidance sesuai workflow","VALIDATE_DOCUMENTATION"],
  [29,"M0-D29","Frozen-commit adversarial review","Codex","Claude Code","High","AUTO_WITH_REVIEW","feat/m0-d29-adversarial","Pada frozen commit, serang tamper, false completion, paths, privilege boundary, dependency, nondeterminism, cancellation, dan performance equivalence secara synthetic/local.","adversarial findings report; deterministic reproducers; no production data","cargo test --workspace --all-targets; npm run check; npm run build; sh scripts/validate-ai-operations.sh","semua finding direproduksi; P0/P1 memblokir Day 30","NO_RUN_NOT_VISUAL"],
  [30,"M0-D30","Production-Directed EAC dan fsnap v0.1 freeze","Codex","Human","Release-critical","HUMAN_APPROVAL","feat/m0-d30-gate","Audit seluruh evidence dan klasifikasikan hanya Production-Directed Engineering Alpha Candidate — Lab Use Only; bekukan `.fsnap` v0.1 untuk Analysis bila semua compatibility gate lulus.","evidence index; milestone review; risk register; capability matrix; fsnap contract/golden fixtures","cargo test --workspace --all-targets; npm run check; npm run build; sh scripts/validate-ai-operations.sh; shasum -a 256 -c docs/RFC-BASELINE.sha256","zero open P0/P1, exact-SHA evidence lengkap, human approval; tidak publish/release/sign","VALIDATE_MILESTONE_DISPLAY"]
];

function blockResult() {
  return `schema: TaskResult.v1
day_id:
task_id:
tool:
role:
gate_status:
incident_category:
branch:
worktree:
commit_before:
commit_after:
local_sha:
remote_sha:
pull_request_sha:
ci_sha:
files_changed:
files_inspected:
commands:
tests_passed:
tests_failed:
tests_skipped:
platforms_tested:
capabilities_not_validated:
unexpected_observations:
security_validity_impact:
remaining_risks:
next_action:
human_approval_required:`;
}

for (const [day, taskId, title, author, reviewer, risk, autonomy, branch, objective, files, verify, focus, visual] of days) {
  const id = String(day).padStart(2, "0");
  const previous = day === 1 ? "baseline commit 5bfabef0c3a9aa8194130a44072dc0dfdaab7f0e" : `Day ${String(day - 1).padStart(2, "0")} dengan independent review EXPECTED_PASS`;
  const next = day === 30 ? "M1 rolling-wave decision" : `DAY-${String(day + 1).padStart(2, "0")}.md`;
  const authorGithub = author === "Codex" ? "Codex bekerja local-only; Claude Code menjadi relay/GitHub gateway setelah commit dibekukan." : "Claude Code boleh menjadi GitHub gateway hanya setelah local commit dibekukan dan diperiksa.";
  const visualPrompt = visual === "NO_RUN_NOT_VISUAL" || visual === "NO_RUN_PLATFORM_REPORT"
    ? `Task ini tidak memerlukan visual validation. Jangan mengedit kode atau GitHub. Periksa bahwa runbook memang tidak memiliki UI artifact baru, lalu keluarkan TaskResult.v1 dengan role Antigravity dan catatan ${visual}. Bila menemukan perubahan UI di luar scope, gunakan gate_status FIX dan incident_category SCOPE-DRIFT.`
    : `Validasi frozen SHA untuk mode ${visual}. Jangan mengubah core semantics. Uji alur, keyboard/focus, error/limitation visibility, viewport, dan copy yang relevan. Simpan screenshot teredaksi tanpa data pribadi. Laporkan mismatch sebagai FIX; jangan memperbaiki branch author. Akhiri dengan TaskResult.v1.`;

  const text = `# Day ${id} — ${title}

## Outcome

${objective}

## Entry Gate

- Prasyarat: ${previous}.
- Project root: \`/Users/user/Projects/Trareon/Trareon Acquire\`.
- Branch: \`${branch}\`; satu author dan satu worktree aktif.
- Baca RFC, roadmap, AI workflow, hasil Day sebelumnya, serta bagian relevan \`docs/superpowers/plans/2026-07-17-trareon-acquire-foundation.md\`.
- Working tree harus bersih atau perubahan pengguna dicatat dan dipreservasi.

## Risk and Autonomy

- Risk: \`${risk}\`.
- Autonomy: \`${autonomy}\`.
- Author: ${author}; reviewer: ${reviewer}.
- ${authorGithub}
- Raw device, privilege, real evidence, secret, signing, release, dependency architecture, destructive operation, atau perubahan RFC berhenti dengan \`HUMAN_APPROVAL_REQUIRED\`.

## Operator Checklist

- [ ] Pastikan Issue Day ${id} berada di status yang benar dan tidak ada Day lain menulis file yang sama.
- [ ] Catat commit_before, branch, worktree, dan recovery count.
- [ ] Salin Author Prompt hanya ke ${author}.
- [ ] Setelah author membekukan commit, salin Reviewer Prompt hanya ke ${reviewer}.
- [ ] Jalankan Antigravity Prompt setelah review kode bila ditugaskan.
- [ ] Jangan maju bila ada \`UNKNOWN\`, SHA mismatch, failing gate, atau approval belum diberikan.

## Author Prompt

\`\`\`text
Anda adalah author untuk ${taskId} Trareon Acquire.

Outcome: ${objective}
Scope file/interface: ${files}.
Branch wajib: ${branch}. Baca authority files dan existing code sebelum perubahan. Gunakan TDD: tulis test yang gagal dengan alasan yang diharapkan, jalankan dan catat exit code, implementasikan perubahan minimum, lalu jalankan verification lengkap.

Verification wajib: ${verify}.
Acceptance focus: ${focus}.

Jangan memperluas scope, melemahkan assertion, skip test, menghapus perubahan pengguna, memakai real evidence, mengakses raw disk/elevated privilege, menambah dependency tanpa review, merge, release, atau signing. Simpan semua status sebagai core-owned typed result. Capability tanpa evidence adalah NotValidated.

Buat commit kecil dengan task ID setelah test lulus. Jangan menyatakan berhasil tanpa command, exit code, diff, dan exact SHA. Bila blocked atau protected boundary muncul, berhenti dan klasifikasikan; jangan mengarang workaround.

Akhiri dengan schema berikut lengkap:
${blockResult()}
\`\`\`

## Expected Change Map

- Expected files/interfaces: ${files}.
- Expected behavior: ${objective}
- Explicit non-goals: fitur setelah Day ${id}, production claim, raw/elevated operation tanpa gate, refactor di luar scope, dan perubahan RFC.
- Diff di luar map menjadi \`FIX + SCOPE-DRIFT\` sampai dijelaskan dan disetujui.

## Expected Result

- Verification: \`${verify}\`.
- Expected: command relevan exit 0 setelah TDD cycle; targeted dan regression tests lulus.
- Required evidence: failing-test proof bila berlaku, commands/exit codes, files changed, exact commit, platforms tested, NotValidated list, dan remaining risks.
- Performance-sensitive change wajib menjaga byte/hash/coverage/audit/cancel/tamper equivalence.

## Reviewer Prompt

\`\`\`text
Anda adalah independent reviewer ${reviewer} untuk ${taskId}. Jangan menulis pada worktree author. Baca RFC, active runbook, commit_before, frozen commit_after, diff, test logs, dan local evidence.

Verifikasi sendiri scope, correctness, negative paths, false-complete resistance, forensic validity, security, dependency boundary, deterministic behavior, performance equivalence, documentation, dan test oracle. Fokus khusus: ${focus}.

Jalankan command aman yang relevan: ${verify}. Bandingkan exact local/remote/PR/CI SHA bila remote evidence tersedia. SHA mismatch adalah BLOCKED + REMOTE-STATE-DIVERGENCE. Hasil tanpa reproducible evidence adalah UNVERIFIED.

Jangan memperbaiki branch author, merge, release, sign, mengakses real evidence/raw disk, atau menyetujui scope expansion. Kembalikan finding ke author dan akhiri dengan TaskResult.v1 lengkap.
${blockResult()}
\`\`\`

## Antigravity Prompt

\`\`\`text
${visualPrompt}

${blockResult()}
\`\`\`

## Unexpected-Output Routing

- Compile/behavior salah → \`FIX + IMPLEMENTATION-FAILURE\` → \`RECOVERY-PROMPTS/IMPLEMENTATION-FAILURE.md\`.
- Test runner/toolchain tidak tepercaya → \`BLOCKED + TEST-INFRA-FAILURE\`.
- Hasil antar-OS/hardware berbeda → \`BLOCKED + PLATFORM-DIVERGENCE\`.
- Repeated run berbeda/flaky → \`BLOCKED + FLAKY-OR-NONDETERMINISTIC\`.
- Performance turun tanpa validity failure → \`FIX + PERFORMANCE-REGRESSION\`.
- Secret/trust/privilege/dependency issue → \`HUMAN_APPROVAL_REQUIRED + SECURITY-FINDING\`.
- Requirement ambigu atau agent berbeda kesimpulan → \`BLOCKED + SPEC-AMBIGUITY/AGENT-DISAGREEMENT\`.
- Diff melebar → \`FIX + SCOPE-DRIFT\`.
- Local/remote/PR/CI SHA berbeda → \`BLOCKED + REMOTE-STATE-DIVERGENCE\`.
- Device/service/network tidak tersedia → \`BLOCKED + ENVIRONMENT-BLOCKED\`.
- Maksimal dua recovery cycle; setelah itu tetap \`BLOCKED\` dan minta keputusan manusia.

## Exit Checklist

- [ ] Author result lengkap dan tidak mengandung \`UNKNOWN\`.
- [ ] Independent reviewer memberi \`EXPECTED_PASS\`.
- [ ] Targeted/regression verification lulus dengan exit code tercatat.
- [ ] Exact SHA konsisten pada semua evidence yang berlaku.
- [ ] P0/P1 ditutup; limitation dan NotValidated dicatat.
- [ ] GitHub Issue/PR/Project diperbarui oleh gateway tanpa merge.
- [ ] Tidak ada protected action yang dilakukan tanpa approval.

## Handoff

Jika semua exit checklist lulus, handoff ke \`${next}\` dengan frozen SHA, TaskResult.v1 author/reviewer/Antigravity, CI/platform evidence, incident/recovery count, dan remaining risks. Selain itu tetap pada Day ${id}.
`;

  writeFileSync(resolve(output, `DAY-${id}.md`), text);
}

console.log(`generated_runbooks=${days.length}`);
