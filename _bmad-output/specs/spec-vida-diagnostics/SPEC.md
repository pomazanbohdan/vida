---
id: SPEC-vida-diagnostics
status: draft
companions:
  - diagnostic-conformance-cases.md
  - ../../planning-artifacts/prds/prd-vida-2026-09-22/prd.md
  - ../../../docs/02-requirements/diagnostics-feedback-requirements.md
  - ../spec-vida-release-security-evidence/SPEC.md
sources: []
---

> **Decision-gated draft.** PRD FR-35 and `REQ-DIAG-001–009` are not an approved/implemented support-service contract. This kernel isolates the local-first, preview, explicit-disclosure and minimization baseline for review; `OQ-0079` remains open.

# VIDA privacy-preserving diagnostics

## Why

Release-1 defects need useful reports without turning VIDA's private, multi-Persona clients into a tracking channel. A crash report can itself contain messages, secrets or linkable identities. The planning baseline therefore needs testable user control and safe defaults before any support infrastructure is chosen.

## Capabilities

- **CAP-1**
  - **intent:** A user can create and inspect local diagnostic evidence for the selected Persona after a crash or failure.
  - **success:** The user can preview, edit or cancel an offline bundle; evidence from another Persona does not enter it by default, and cancellation causes no network send.
- **CAP-2**
  - **intent:** The user explicitly decides whether to disclose the prepared report.
  - **success:** Creation, crash, restart and a failed send never trigger automatic transfer by default; the UI distinguishes a local report from a report actually sent through a later-approved channel.
- **CAP-3**
  - **intent:** A default report carries technical evidence without private content or cross-Persona tracking identifiers.
  - **success:** Canary tests find no Message/Note/File content, contacts, keys, tokens, raw Persona IDs or stable cross-Persona correlators in the default bundle or its diagnostic/error surfaces.

## Constraints

- The detailed diagnostics requirements remain `status: review`; candidate fixtures do not approve a recipient, channel, schema, retention period, background opt-in or operational support system.
- No diagnostic transfer occurs by default. In the proposed manual flow, a person sees the material and explicitly initiates send/export; whether a separately consented background mode exists remains `OQ-0079`.
- Local sensitive evidence must be protected and Persona-isolated. For mobile profiles, [OWASP MASVS-PRIVACY-1](https://mas.owasp.org/MASVS/controls/MASVS-PRIVACY-1/) and [MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) inform privacy/storage checks; the [OWASP Logging Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html) informs secret/PII exclusion. None is a Windows certification or proof that VIDA passes.
- A diagnostic security `pass` cannot be inferred from this plan. The adopted security-evidence draft expects a tested build/profile, fixture, result and artifact; its exact manifest fields remain proposed.

## Non-goals

- Selecting Support Contact ownership, E2EE routing, encrypted-export format, ticket semantics, retention or deletion durations here.
- Permitting silent background upload, cross-Persona analytics or bundling user content by default.
- Claiming independent security review or a deployed support service.

## Success signal

On installed Android, iOS and Windows release candidates, `DIAG-F01–F06` demonstrate local preview, explicit disclosure and exclusion of sensitive canaries. No fixture has run; the service/channel decisions under `OQ-0079` remain prerequisites to a full release claim.

## Open Questions

- Who receives support reports, and how are recipient identity, access, routing and abuse controls governed?
- Is Release 1 manual-only, or may a separately opted-in background mode exist?
- What bundle schema, exact redaction rules, retention/deletion periods and export/send mechanisms are approved?
- Should Release 1 allow a user to add a screenshot or selected content after a separate warning, as proposed by review-stage `REQ-DIAG-004`?
