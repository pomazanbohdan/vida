---
id: SPEC-vida-operation-envelope
status: draft
companions:
  - wire-profile-decisions.md
  - semantic-field-dictionary.md
  - ../../../docs/02-requirements/transport-sync-requirements.md
  - ../../../docs/02-requirements/open-interoperability-requirements.md
  - ../../../docs/04-specifications/sync-log-contract.md
  - ../../../docs/04-specifications/durable-delivery-contract.md
  - ../../../docs/04-specifications/operation-finality-contract.md
  - ../../../docs/04-specifications/space-membership-contract.md
  - ../../../docs/04-specifications/device-sync-session.md
sources: []
---

> **Draft decision kernel for OQ-0028.** Read `companions:` for the accepted semantic contracts. This draft does not select a codec, signature suite or production wire profile; it cannot yet support an interoperability claim.

# VIDA signed operation envelope

## Why

Independent VIDA clients, nodes and Apps need one verifiable representation of a durable change across direct peer sync, offline delivery and recovery. Without byte-exact signing and compatibility rules, two implementations can agree on the apparent resource value while disagreeing on identity, authorization or receipt evidence. Closing this gap is a prerequisite for safe protocol fan-out and public conformance.

## Capabilities

- **CAP-1**
  - **intent:** A producer can create one stable, signed durable operation for direct and mailbox delivery.
  - **success:** Duplicate, reordered and post-crash deliveries of that operation retain its identity and bound content and produce one validated apply.
- **CAP-2**
  - **intent:** An authorized peer can verify an operation's author, scope, control state and causal prerequisites before making it visible.
  - **success:** Tampered, wrong-Space or dependency-incomplete input and an unaccepted candidate with revoked/stale grants never become accepted visible state; an operation accepted before the effective revocation cut remains valid history.
- **CAP-3**
  - **intent:** Peers with different supported versions can determine whether they can exchange and preserve an operation safely.
  - **success:** Supported optional information round-trips without changing proof semantics; an unknown mandatory feature or incompatible major version fails explicitly rather than being silently applied.
- **CAP-4**
  - **intent:** Independent implementations can reproduce the operation's signed representation and validation result.
  - **success:** Rust Core and an independently built reader produce identical byte-exact golden vectors and negative-security outcomes for the declared protocol profile without a private VIDA service.
- **CAP-5**
  - **intent:** Offline delivery can forward protected operations without reading their content or becoming their authority.
  - **success:** Direct and mailbox paths preserve the same signed inner operation and content binding; a route-specific wrapper cannot change its identity or turn ciphertext storage into domain acceptance.

## Constraints

- Iroh core 1.2 is the transport foundation; VIDA owns versioned ALPN and operation semantics. An Iroh `EndpointId`, connection or packet ACK is not a Persona, Space grant or authority proof.
- Origin operation, validated local log and outbox commit atomically. Repeated delivery is idempotent by stable `OperationId`; missing causal dependencies remain pending, not visibly applied.
- The operation must bind author/device and Space context, causal metadata, protocol/schema features, protected content and authentication evidence. The exact wire fields and signature bytes remain open in OQ-0028.
- Local durability, replication, recipient delivery, domain authority and external effect are independent evidence axes. The operation envelope alone cannot promote one axis from a transport ACK.
- Current grant/control epoch, signature, schema/policy and causal preconditions are checked before visible apply. Client timestamps and an old offline envelope cannot restore revoked rights.
- The mailbox sees only necessary routing metadata and E2E ciphertext; it gains no Space or resource authority from storage. Sender-facing receipts must not expose raw recipient device topology.
- Unknown mandatory semantics fail closed. An optional field may be skipped only when that omission does not alter proof semantics; a breaking semantic change needs a new protocol major or explicit migration.
- Equal devices of one Persona gain no permanent priority. Device type, endpoint, local clock, arrival order and transport ACK do not establish authority order.
- A claimed interoperable profile requires public versioned specifications, byte-exact and negative-security fixtures, and independent conformance proof without a paid VIDA dependency.

## Non-goals

- Selecting deterministic CBOR/PDM field IDs, COSE or another signature construction, hash/AEAD algorithms, key-management protocol or final byte framing in this draft.
- Defining a logical authority topology or winner for serverless exclusive operations (OQ-0033), operation-family merge rules (OQ-0034), or mailbox deployment topology (OQ-0027).
- Treating transport connection, CRDT materialization, storage receipt or a client timestamp as a business approval.

## Success signal

An independent implementation reads a public fixture bundle and returns the same operation IDs, signed-byte digests, validation failures and application-receipt bindings as VIDA Core across direct, mailbox, duplicate and hostile-input cases. Until OQ-0028 selects an exact profile and the vectors execute, this remains a testable target rather than a passed gate.

## Open Questions

- Which typed PDM subset and exact canonical serialization profile represent the signed operation, including field IDs, map order, numeric/tag/Unicode rules, duplicate keys and size limits?
- Which operation-ID, digest, signature, domain-separation and encryption-layer rules bind clear routing fields to protected payload and immutable Space/AppInstance/Resource/control context?
- How do ALPN major, mandatory/optional features, unknown-field preservation and the OQ-0037 compatibility manifest negotiate a mixed-version session?
- Which published operation/receipt vectors and independent implementation evidence are sufficient to move this draft to an approved wire profile?
