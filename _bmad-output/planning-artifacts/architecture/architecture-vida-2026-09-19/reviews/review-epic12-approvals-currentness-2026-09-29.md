# Currentness review — Epic 1/2 approval batch (2026-09-29)

## Verdict

Pass for document currentness. The ten approved product behaviors are reflected across the recovery spec, architecture spine, Epic 1/2 story acceptance, operation-finality contract and fixture, and open-question register. This is a consistency verdict, not a production-readiness or executed-conformance claim.

## Resolved during review

- `ARCHITECTURE-SPINE.md:381` no longer hard-codes 25 operation-finality vectors; the fixture currently contains 26 and the spine requires all approved vectors.
- `ARCHITECTURE-SPINE.md:337` now distinguishes a newer backup's current status from physical deletion; the prior verified resource snapshot remains until explicit cleanup, matching `SPEC.md:64`.
- `SPEC.md:52` and `epics.md:346-349` now state that automated fresh-profile restoration is a release gate, not mandatory per-user restoration; guided user verification remains optional.
- `operation-finality-contract.md:223` now repeats the separate-failure-domain requirement and excludes two browser profiles on one physical host, matching `REQ-FINALITY-003` and the same-host fixture.

## Remaining gates, not currentness defects

- `OQ-0022/0024` still require the canonical scoped-grant representation, atomic rotation guard, repair protocol and executable recovery proof. The specifications explicitly keep these open.
- Independent-host evidence for `Synchronized` remains a conformance gate; an authorized receipt from a second browser profile on the same host does not meet it.
- `Story 2.2` concerns direct Note synchronization; the voluntary Web unlink/loss-warning acceptance belongs to `Story 2.12`, avoiding duplicate criteria.

No further document-currentness correction is required for this batch on the reviewed paths.
