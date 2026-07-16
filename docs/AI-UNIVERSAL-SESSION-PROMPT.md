# Trareon Acquire Universal AI Session Prompt

Use this prompt unchanged to start a fresh session or to hand off between AIs in the same repository. It is designed to require no manual fill-in fields. The agent must discover the current state from the folder itself and record the session output back into the repository.

```text
You are working inside the Trareon Acquire repository.

Repository root:
- Discover automatically from the current working directory.

Primary rule:
- Read the repository state first, then continue from the latest recorded session note in the repo.
- Do not ask me to fill in any placeholders.
- Do not repeat work that is already finished.
- Treat the repository itself as the source of truth.

Mandatory discovery steps:
1. Read the current working directory.
2. Inspect `git status`.
3. Inspect the active branch and current HEAD.
4. Read the latest session note from the repository log folder if one exists.
5. Read the most relevant project docs before making changes.
6. Identify the smallest safe next action.

Required context sources, in this order:
- `docs/AI-DEVELOPMENT-WORKFLOW.md`
- `docs/IMPLEMENTATION-ROADMAP.md`
- `docs/ZERO-CASH-LAUNCH-PLAN.md`
- The latest session note under `docs/ai-session-log/`
- Any task- or feature-specific document that matches the current work

Working rules:
- Keep the scope bounded to one clear deliverable.
- Prefer the smallest change that moves the repo forward.
- Preserve valid work already present in the repository.
- If a choice has trade-offs, explain the recommendation before acting.
- If implementation is risky, stop and describe the risk before changing anything.
- If the current work is already complete, do not invent new work.
- If a command fails, fix the root cause instead of retrying blindly.
- Use the repository's existing patterns, filenames, and wording.

Documentation and logging rules:
- Every session must end with a written session note saved in `docs/ai-session-log/`.
- The session note must include:
  - timestamp
  - agent name or tool name if known
  - what was discovered automatically
  - files changed
  - commands run
  - verification results
  - open risks or next steps
- If no code or docs changed, still write a session note explaining what was checked.
- Never leave a session without recording its outcome in the repository.

Output rules:
- Start with the final state, not a plan.
- Mention the files that changed.
- Mention the verification you actually ran.
- Mention any unresolved risk clearly.
- If work remains, end with a short handoff note.

Finish condition:
- The session is only complete after the repo state has been checked and the session note has been written.
```

## Session log folder

Write session notes to `docs/ai-session-log/` using timestamped markdown files such as:

- `docs/ai-session-log/2026-07-16-claude-code.md`
- `docs/ai-session-log/2026-07-16-codex-handoff.md`

If the folder does not exist yet, create it before writing the first note.

