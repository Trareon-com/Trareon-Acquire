# Acquire completion gate log (M0–M15)

Use one dated copy per release candidate. A checked item needs a link to reproducible evidence,
exact revision, and operator/reviewer identity.

| Gate | Status | Evidence / owner |
|---|---|---|
| M0–M3 foundation acquisition, package, audit, verification | [ ] | |
| M4–M6 source policy, write-blocker handling, format boundaries | [ ] | |
| M7–M9 preservation, custody, reporting, signing boundary | [ ] | |
| M10–M12 platform/live gate and capability matrix evidence | [ ] | |
| M13 regression, adverse cases, documentation review | [ ] | |
| M14 accessibility, SOP copy, and product IA review | [ ] | |
| M15 complete gate log and claim review | [ ] | |

## Allowed claims after software evidence

- “Implemented for synthetic or file-backed workflows” when the linked automated check passes.
- “Live gate pending” or “human lab required” when the procedure and missing evidence are explicit.
- “E01-lite” and other subset labels only with their format limitation.
- “Read-only Analysis import” only when the package remains unchanged and verification succeeds.

## Forbidden claims without human evidence

- Court-ready, production-ready, validated, certified, or for-real-evidence acquisition.
- Hardware write-blocker detection, ATA/HPA/DCO live support, or platform support without the
  matching live-gate record.
- Interoperability with Autopsy, FTK, libewf, or another external tool without its recorded result.
- Signed, notarized, officially released, supported, or approved for production.

## Release decision

- [ ] Software evidence reviewed
- [ ] Human lab evidence reviewed
- [ ] Claim wording reviewed
- [ ] Human release owner decision recorded

AI may prepare this log but cannot check human approval or release-decision items.
