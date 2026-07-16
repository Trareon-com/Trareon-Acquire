# AI Session Log

This folder stores append-only session notes produced by whichever AI is currently working in the repository.

Use one markdown file per session. Keep the note short, factual, and tied to what was actually checked.

Minimum contents for each note:

- timestamp
- agent name or tool name
- repository state discovered automatically
- files changed, if any
- commands run
- verification results
- next step or unresolved risk

Suggested filename pattern:

- `YYYY-MM-DD-agent-or-task.md`

The universal session prompt in `docs/AI-UNIVERSAL-SESSION-PROMPT.md` requires the active AI to write one of these notes before ending the session.
