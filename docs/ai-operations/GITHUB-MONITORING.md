# GitHub Monitoring

Repository: `Trareon-com/Trareon-Acquire`. GitHub adalah single monitoring pane; physical validation tetap berlangsung pada lab device dan hanya mengirim laporan teredaksi.

## Project configuration

Status: `Backlog`, `Ready`, `Claude Implementing`, `Codex Reviewing`, `Antigravity Validating`, `CI Running`, `Hardware Validation`, `Human Approval`, `Done`.

Fields: `Day`, `Task ID`, `Milestone`, `Risk`, `Autonomy`, `Author`, `Reviewer`, `Frozen SHA`, `CI`, `Hardware`, `Incident`, `Human Gate`, `Evidence URL`.

Satu item Project terhubung ke satu Day Issue. PR terhubung ke Issue. `Done` hanya ketika local SHA, remote SHA, PR SHA, CI SHA, dan evidence SHA identik serta review lulus.

## Claude Code GitHub gateway prompt

```text
Anda adalah satu-satunya GitHub gateway untuk task aktif Trareon Acquire. Baca Day runbook dan TaskResult.v1 lokal. Pastikan branch dan local SHA sama dengan hasil author sebelum remote operation.

Di GitHub:
1. Push hanya task branch yang disebut runbook.
2. Buat/perbarui satu Issue dan satu PR; jangan merge.
3. Isi exact local/remote/PR/CI SHA, tests, platforms tested, NotValidated capabilities, limitations, dan evidence links.
4. Relay hasil Codex secara utuh dan tandai “Relayed by Claude Code; reviewed locally by Codex”.
5. Pindahkan Project status hanya sesuai evidence.
6. Unggah hanya synthetic artifact atau laporan teredaksi.

Stop BLOCKED + REMOTE-STATE-DIVERGENCE bila SHA berbeda. Jangan force push, rebase shared history, mengubah protection/secrets, merilis, menandatangani, atau mengunggah barang bukti/data sensitif.

Akhiri dengan TaskResult.v1.
```

## Hardware evidence

Gunakan `TEMPLATES/PLATFORM-TEST-REPORT.md`. Redaksi serial sensitif dan data pribadi. Jangan unggah raw disk image, real evidence, credential, signing material, atau log tidak ditinjau. Persistent self-hosted runner tidak menjalankan kode pull request publik; hardware test dimulai manual dari trusted frozen SHA.

## Monitoring minimum

- Issue menunjukkan owner, dependency, checklist, dan human decision.
- PR menunjukkan diff, review, checks, exact SHA, limitation, dan artifact.
- Actions menunjukkan hosted build/test/security; hosted success bukan hardware proof.
- Security findings ditautkan ke task tanpa memasukkan secret ke komentar.
- Evidence URL mengarah ke artifact/report untuk SHA yang sama.
