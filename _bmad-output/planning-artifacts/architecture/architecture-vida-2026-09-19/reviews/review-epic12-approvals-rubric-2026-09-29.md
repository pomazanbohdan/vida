# Rubric review — Epic 1/2 approval batch (2026-09-29)

Verdict: pass for the ten approved product behaviors; production readiness is not claimed. The architecture, recovery spec/cases, story acceptance criteria and operation-finality contract agree at this altitude. Exact cryptographic, controller, independent-host and recovery proof remains explicitly gated.

Scope: `ARCHITECTURE-SPINE.md` AD-20/26/37/40 and implementation gates; recovery `SPEC.md`, `recovery-bundle-contract.md`, `recovery-cases.md`; `epics.md` Stories 1.2/1.4/2.1/2.11/2.12/2.14; `operation-finality-contract.md` and its semantic vectors; `open-questions.md` OQ-0022/0024.

## Resolved during review

1. Story 1.4 approval addendum had been placed after the Epic 2 introduction. It is now under Story 1.4, immediately before the Epic 2 heading (`epics.md:361-366`).
2. The operation-finality fixture grew from 25 to 26 vectors while the architecture named 25. The spine now requires **all approved vectors**, avoiding a stale count (`ARCHITECTURE-SPINE.md:381`).
3. The same-host browser-profile vector used one-off receipt fields. It now uses the existing `receipts` structure with `independent: false` and `same_physical_host_as_origin: true` (`fixtures/operation-finality-v1.yaml:265-279`).

## Remaining gates, not defects in this update

- OQ-0022/0024: scoped-grant wire/merge proof, frontier/epoch rotation guard, crash-safe repair, bundle crypto/encoding and cross-platform fresh-profile restore evidence.
- Independent-host proof must not turn into cross-Persona tracking; the operation-finality contract names this a conformance gate (`REQ-FINALITY-003`).
- This is a document-consistency review, not evidence that implementation or conformance tests passed.
