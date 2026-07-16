import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "..");
const output = resolve(root, "docs/ai-operations/RECOVERY-PROMPTS");
mkdirSync(output, { recursive: true });

const recoveries = [
  ["IMPLEMENTATION-FAILURE","Kode atau output tidak memenuhi acceptance criteria.","Reproduksi failure dengan command terkecil; pastikan test gagal karena behavior yang salah; perbaiki hanya file dalam scope; jalankan targeted dan regression tests.","Jangan melemahkan assertion, skip test, mengganti expected result, atau refactor di luar scope.","FIX bila satu bounded repair masih tersedia; BLOCKED setelah recovery cycle kedua."],
  ["TEST-INFRA-FAILURE","Runner, fixture, harness, clock, filesystem, atau tool test tidak memberi hasil tepercaya.","Pisahkan product failure dari infrastructure failure; catat tool/version/environment; reproduksi dengan minimal fixture; pulihkan harness tanpa mengubah product acceptance.","Jangan menyatakan product pass, mengganti test menjadi warning, atau memakai hasil run yang stale.","BLOCKED sampai infrastructure menghasilkan evidence tepercaya."],
  ["PLATFORM-DIVERGENCE","Exact OS, architecture, policy, filesystem, atau hardware memberi behavior berbeda.","Catat exact environments dan frozen SHA; buat reproducer identik; bandingkan API/error/bytes; pisahkan capability matrix per kombinasi.","Jangan membuat fallback diam-diam atau menyatakan semua OS didukung.","BLOCKED; affected capability menjadi NotValidated sampai evidence dan decision tersedia."],
  ["FLAKY-OR-NONDETERMINISTIC","Repeated equivalent runs menghasilkan output, ordering, timing, hash, atau status berbeda.","Jalankan minimal tiga reproduksi; bekukan seed/timezone/locale/concurrency; bandingkan byte, manifest, audit, dan logs; isolasi race atau unstable input.","Jangan rerun sampai hijau, menaikkan timeout tanpa bukti, atau menerima non-determinism pada evidence semantics.","BLOCKED sampai deterministik atau limitation/decision manusia disetujui."],
  ["PERFORMANCE-REGRESSION","Throughput, memory, latency, atau overhead melewati baseline tanpa validity failure.","Ulangi benchmark pada environment sama; catat variance; profile bounded scope; buktikan byte/hash/coverage/audit/cancel/tamper equivalence setelah perubahan.","Jangan menukar validitas dengan kecepatan, menghapus verification, atau mengubah benchmark dataset.","FIX bila optimasi equivalen dan teruji; BLOCKED bila target memerlukan perubahan semantics."],
  ["SECURITY-FINDING","Muncul secret, unsafe privilege, injection, dependency, path, telemetry, data exposure, atau trust-boundary issue.","Preservasi evidence teredaksi; hentikan affected path; klasifikasikan severity/reachability; buat reproducer synthetic/local; rekomendasikan minimal containment.","Jangan mengeksploitasi sistem nyata, mengunggah secret/data, menonaktifkan security gate, atau memperluas akses.","HUMAN_APPROVAL_REQUIRED untuk security exception/dependency/privilege; selain itu FIX dan independent re-review."],
  ["SPEC-AMBIGUITY","Dua interpretasi requirement yang masuk akal menghasilkan behavior berbeda.","Kutip exact RFC/spec text; tunjukkan kedua interpretasi dan test outcome; isi TEMPLATES/DECISION-REQUEST.md dengan safe default dan dampak.","Jangan memilih interpretasi yang paling mudah atau mengubah code sebelum keputusan.","BLOCKED sampai human decision/ADR atau RFC amendment disetujui."],
  ["SCOPE-DRIFT","Diff menyentuh file, interface, dependency, atau behavior di luar Day runbook.","Daftar perubahan out-of-scope; pisahkan required coupling dari opportunistic work; kembalikan perubahan tidak perlu tanpa merusak user work; buat task terpisah bila diperlukan.","Jangan menyelundupkan fitur/refactor, menghapus user changes, atau memperluas acceptance criteria.","FIX setelah diff kembali bounded; HUMAN_APPROVAL_REQUIRED bila coupling mengubah arsitektur."],
  ["AGENT-DISAGREEMENT","Author dan reviewer memiliki kesimpulan berbeda yang sama-sama mengutip evidence.","Bekukan commit; tulis claim A/B, command, output, dan spec basis; jalankan neutral reproducer; isi DECISION-REQUEST bila bukti tetap konflik.","Jangan voting model, mengambil jawaban mayoritas, atau membiarkan author menutup review sendiri.","BLOCKED sampai evidence menyelesaikan konflik atau manusia memutuskan."],
  ["REMOTE-STATE-DIVERGENCE","Local, remote, PR, CI, atau evidence tidak menunjuk frozen SHA yang sama.","Catat semua SHA; hentikan state transition; fetch metadata melalui GitHub gateway; tentukan commit authoritative; buat branch/PR baru bila diperlukan tanpa rewrite history.","Jangan force push, rebase shared history, merge, atau memilih passing run dari SHA lain.","BLOCKED sampai semua evidence terikat pada satu frozen SHA."],
  ["ENVIRONMENT-BLOCKED","Network, service, toolchain, device, permission, quota, atau dependency availability mencegah progress.","Catat exact error/version/state; cek safe local alternative yang tidak mengubah contract; preservasi partial work; tentukan apakah retry atau environment lain dapat memberi evidence setara.","Jangan unpin dependency, mengunduh tool tidak terverifikasi, meminta privilege, atau mengklaim platform pass.","BLOCKED; gunakan retry terjadwal atau human decision untuk perubahan environment."],
  ["HUMAN-APPROVAL-REQUIRED","Langkah berikutnya menyentuh raw device, privilege, destructive operation, real evidence, secret, signing, release, legal claim, dependency architecture, atau RFC change.","Isi DECISION-REQUEST dengan exact action/command/target, alasan, rollback, data risk, validity impact, dan safe default; lakukan tidak lebih dari read-only diagnosis.","Jangan menjalankan action, meminta credential dalam chat, mengurangi scope guard, atau menganggap diam sebagai persetujuan.","Tetap HUMAN_APPROVAL_REQUIRED sampai keputusan eksplisit dicatat untuk tindakan yang sama."],
];

const result = `schema: TaskResult.v1
day_id:
task_id:
tool:
role: recovery
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

for (const [name, trigger, allowed, prohibited, exit] of recoveries) {
  const text = `# Recovery — ${name}

Gunakan prompt ini hanya ketika incident category sama dengan nama file.

## Required input

- Day dan Task ID aktif.
- Original author/reviewer serta frozen SHA.
- Exact error, command, exit code, log teredaksi, dan minimal reproducer.
- Diff serta daftar perubahan pengguna yang harus dipreservasi.
- Recovery count 0 atau 1. Count 2 langsung menjadi BLOCKED.

## Trigger

${trigger}

## Copy-paste prompt

\`\`\`text
Anda adalah recovery agent untuk incident ${name} pada Trareon Acquire. Kerjakan hanya Day/task/commit yang diberikan. Validasi bahwa incident memang ${name}; bila salah, berhenti UNVERIFIED dan sebutkan kategori yang didukung evidence.

Tindakan yang diizinkan: ${allowed}

Larangan: ${prohibited}

Preservasi user changes. Maksimal dua bounded recovery cycle. Original author melakukan code repair; reviewer memeriksa frozen commit baru. Jangan merge, release, sign, memakai real evidence/raw disk/secret, atau mengubah RFC/acceptance tanpa human decision.

Exit rule: ${exit}

Akhiri dengan schema lengkap:
${result}
\`\`\`

## Expected evidence

- Reproducer dan exact command/exit code.
- Before/after SHA bila ada repair.
- Targeted dan regression result.
- Perubahan capability/limitation.
- Recovery count dan exact next action.
`;
  writeFileSync(resolve(output, `${name}.md`), text);
}

console.log(`generated_recovery_prompts=${recoveries.length}`);
