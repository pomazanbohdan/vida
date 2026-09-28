---
id: SPEC-VIDA-APPROVAL-PROCESS
status: draft
companions:
  - approval-cases.md
  - ../../../docs/02-requirements/effect-execution-constraints.md
  - ../../../docs/04-specifications/operation-finality-contract.md
  - ../../../docs/04-specifications/operation-semantics-catalog.md
  - ../../../docs/02-requirements/access-control-requirements.md
  - ../spec-vida-exclusive-authority/SPEC.md
sources: []
---

> **Draft BMad contract.** Approved approval modes and policy ownership are distilled here into planned checks. Authority topology, in-flight policy changes and several vote transitions remain open; no fixture has been executed.

# VIDA Core approval processes

## Why

Some resource changes need an explicit decision after a user has saved a request. VIDA must show that difference honestly across offline Devices and offer reusable approval behavior to Apps without letting a package invent a resolver or weaken Space ownership rules. A process may need one person, ordered steps or a committee, but every result must remain tied to the exact request and protected revision.

## Capabilities

- **CAP-1**
  - **intent:** An App supplies a recommended confirmation policy for each named process, and the Space Admin chooses an effective Core-supported mode for that process.
  - **success:** Two processes in one AppInstance can retain different policies; changing one does not alter the other or install resolver code, and the package default is not treated as an immutable minimum.
- **CAP-2**
  - **intent:** A requester can submit a durable candidate and later receive a distinct authoritative result.
  - **success:** Local save, network delivery or replication alone leaves the request pending; a signed `accepted|rejected|expired` outcome is matched to its `RequestId`, revision and frontier, reaches the requester as a separate sync fact and is idempotent on replay.
- **CAP-3**
  - **intent:** A named process can require one approver, ordered approval stages or a threshold of distinct approvers.
  - **success:** Core evaluates `single-approver`, `sequential-stages`, `M-of-N` and `all-of` as `M=N`; no stage is skipped and repeat votes by one Persona or its Devices do not increase the count.
- **CAP-4**
  - **intent:** An authorized approver decides on the exact protected revision that was presented for review.
  - **success:** An unauthorized, duplicate or stale-revision vote cannot finalize the request; changing protected data creates a new revision whose outcome needs fresh applicable approval.
- **CAP-5**
  - **intent:** A process may have automatic confirmation when no human approval is required, without bypassing Core safeguards.
  - **success:** `auto-approve` still checks current rights, preconditions, semantic class and named authority; it cannot alter Owner/Admin governance or turn two incompatible exclusive claims into two confirmed results.

## Constraints

- `ConfirmationPolicy` is configured per named process/operation family inside an AppInstance. The AppPackage recommends a default; Space Admin selects only a Core-supported profile, not arbitrary resolver code.
- The three manual approval profiles are `single-approver`, `sequential-stages` and `threshold M-of-N`; `all-of` is `M=N`. `Auto-approve` is a no-manual-vote mode, not a fourth manual profile.
- Each vote is append-only, attributable to a current authorized approver and bound to the precise request, protected revision and frontier. Several Devices of one Persona are not independent votes.
- Transport ACK, local save and replication do not constitute approval. Exclusive-claim serialization and Owner/Admin/key-revocation governance stay Core invariants regardless of configured profile.
- [OWASP Authorization](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) is a verification reference for deny-by-default and permission checks on each action; it does not define VIDA peer authority topology.

## Non-goals

- This Core spec does not add a leave, purchase or City-booking App to Release 1; examples in [approval-cases.md](approval-cases.md) exercise generic process behavior.
- It does not select a serverless serialized authority/current-frontier mechanism (`OQ-0033`), arbitrary semantic resolvers, external API retry policy or agent delegation model.
- It does not redefine ordinary editing, Core Conflict resolution or Space governance as an approval process.

## Success signal

A test AppInstance can run three named processes with different Admin-selected policies: a single approver, ordered stages and `2-of-3`; the same requester sees pending before an authenticated outcome, duplicate Devices cannot inflate votes, and an edit to protected content requires a new revision decision. [APR-F01–F13](approval-cases.md) are planned checks, not implementation evidence.

## Open Questions

- When Admin changes a process policy or its approver set, which version governs already pending requests, and how is that version carried in the signed request/outcome?
- What logical authority/current-frontier proof serializes approval or exclusive outcome during a serverless partition (`OQ-0033`)?
- What exact effect do approver delegation, revocation after a vote, explicit rejection/veto and expiry have in each profile? Candidate age alone must not silently produce an expired outcome.
