---
id: SPEC-vida-signed-log-engine
status: draft
companions:
  - candidate-evaluation-and-conformance.md
  - ../../planning-artifacts/research/technical-iroh-project-library-inventory-for-vida-2026-09-23/research.md
  - ../spec-vida-storage-provider/SPEC.md
  - ../../../docs/04-specifications/sync-log-contract.md
  - ../../../docs/04-specifications/operation-finality-contract.md
  - ../../../docs/04-specifications/operation-semantics-catalog.md
  - ../../../docs/04-specifications/device-sync-session.md
  - ../../../docs/02-requirements/transport-sync-requirements.md
sources: []
---

> **Decision-gated OQ-0033/OQ-0034 kernel.** This spec defines how to compare a signed operation-log and repair engine. It does not select Irokle, iroh-db, Knot or a custom implementation; it does not close serverless authority ordering or safe pruning.

# VIDA signed-log and repair engine

## Why

Messenger, Notes, Projects and Files need one verifiable operation history across equal, sometimes disconnected devices. Iroh carries bytes; a physical store persists bytes; neither alone validates a signed operation, repairs missing causal history or decides whether an operation is only replicated or has domain authority. VIDA needs a provider-neutral engine contract before choosing libraries.

## Capabilities

- **CAP-1**
  - **intent:** Validate and append one signed, versioned operation under a stable identity.
  - **success:** Invalid signature, membership epoch, schema/capability or precondition cannot advance the accepted frontier; duplicate delivery produces one logical operation and one set of derived effects.
- **CAP-2**
  - **intent:** Reconcile missing history between any authorized peers, including after interruption.
  - **success:** Out-of-order and repeated transfers converge to the same validated causal DAG; missing dependencies stay pending and are fetched without silently discarding an offline branch.
- **CAP-3**
  - **intent:** Keep durability, replication, delivery, authority and external-effect evidence distinct.
  - **success:** A transport ACK or ciphertext mailbox write never becomes “synchronized”, “delivered” or domain-accepted by inference; a signed application-level receipt proves the corresponding frontier.
- **CAP-4**
  - **intent:** Rebuild materialized state and resume synchronization from persisted history.
  - **success:** Crash/restart and projection rebuild preserve operation IDs, outbox, unresolved conflicts and receipts without rerunning originating business commands or external effects.
- **CAP-5**
  - **intent:** Support each Core semantic class without adopting a library's accidental conflict winner.
  - **success:** Mergeable independent edits combine; incomparable incompatible status/file branches remain explicit conflicts; exclusive claims remain pending until a named authority decides under the operation contract.
- **CAP-6**
  - **intent:** Select a maintainable engine from the Iroh ecosystem using current evidence and identical fixtures.
  - **success:** Each candidate has a dated, reproducible evidence card covering development pace, substantive commit/PR frequency, implemented required functionality, issue volume and handling, plus compatibility, security and license gates; a `<1.0` version is not an automatic rejection.

## Constraints

- Devices of one Persona are equal replicas. No device becomes master merely by receiving or storing an operation first.
- The signed log and validation rules belong to VIDA Core. Iroh transport, physical store and CRDT document provider stay replaceable behind separate contracts.
- `Accept` into local validated history is not a domain-authority decision. Exclusive-resource finality requires the named authority and verified current frontier; partitions cannot manufacture a global winner from timestamps or arrival order.
- Pending operations, unresolved variants and required history are not removed solely by age. Snapshot/GC requires a separately proven safe frontier.
- The candidate decision must pass the same conformance suite on a resolved dependency graph and targeted Android, iOS and Windows builds; repository popularity, stars, raw commit count, issue count or semver alone cannot establish production fitness.

## Non-goals

- Choosing operation wire serialization/signature bytes (OQ-0028), serialized serverless authority topology (OQ-0033), equivalence predicate and safe pruning (OQ-0034), or physical storage provider (OQ-0036).
- Importing an upstream library's roles, membership, conflict winner, application schema or receipt semantics as VIDA policy.
- Claiming exactly-once external effects when the target service cannot verify/reconcile a repeated request.

## Success signal

The same recorded fixtures pass for a candidate implementation and a minimal reference: valid/invalid signed operations, 2–3 partitioned peers, duplicate/reordered transfer, missing-history repair, crash/restart, membership revocation, mixed client versions, conflict retention and independently verifiable replication/authority receipts. A dated evidence card explains remaining failures and trade-offs before an ADR may lock a provider.

## Open Questions

- **Product behavior approved 2026-09-29:** [D10–D12](../../planning-artifacts/implementation-readiness-epics-1-2-2026-09-29.md): stable Note blocks plus range-aware operations, explicit conflict only for truly overlapping intent; equivalent resolutions require equal canonical content and domain effects, unknown remains conflict; no age-only history deletion, with verified snapshot+tail repair. Exact predicates, frontiers and fixtures remain open.

- Which candidate parts are reusable as libraries rather than architectural patterns after version-pinned builds and fixtures?
- Which authority topology gives a comparable current frontier for serverless exclusive operations?
- What equivalence predicate, snapshot proof and retention frontier permit safe compaction across offline peers?
