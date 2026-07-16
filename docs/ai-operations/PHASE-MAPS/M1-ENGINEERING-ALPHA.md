# M1 — Engineering Alpha

## Entry evidence

- Day 30 classified `Production-Directed Engineering Alpha Candidate — Lab Use Only` by human approval.
- `.fsnap` v0.1 read contract, golden packages, and independent verifier pass exact-SHA compatibility tests.
- No open P0/P1; limitation, performance baseline, and discrepancy registers complete.

## Scope

- Stabilize file-backed acquisition, checkpoint/recovery, cancellation, deterministic package/report output, fuzz/property coverage, documentation mapping, and repeatable builds.
- Start Trareon Analysis read-only importer against `.fsnap` v0.1 golden valid/invalid fixtures.
- Analysis verifies before import, writes indexes outside evidence package, and never repairs/upgrades package silently.

## Exit evidence

- Synthetic and injected-failure matrix has no false-complete path.
- Repeated runs are deterministic and performance remains within documented ceilings.
- Acquire/Analysis compatibility suite covers valid, mutated, truncated, removed-file, audit-discontinuous, and unsupported-version packages.
- Next rolling-wave prompts are generated from observed Day 30/M1 evidence.

## Anticipated incidents

Schema ambiguity routes to `SPEC-AMBIGUITY`; reader/writer disagreement to `AGENT-DISAGREEMENT`; unstable recovery to `FLAKY-OR-NONDETERMINISTIC`; Analysis mutation to `SECURITY-FINDING`.

## Trigger

Generate detailed M1 prompts only after the Day 30 Evidence Index and human classification are committed and linked from GitHub.
