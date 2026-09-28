---
id: SPEC-DURABLE-DELIVERY-001
status: approved
implementation_status: unplanned
last_updated: 2026-09-22
requirement_refs:
  - ../02-requirements/transport-sync-requirements.md
  - ../02-requirements/platform-nfr.md
  - ../02-requirements/effect-execution-constraints.md
decision_refs:
  - ../03-architecture/decisions/ADR-0006-durable-delivery-and-operation-envelope.md
---

# SPEC-DURABLE-DELIVERY-001: DurableDelivery contract

## Purpose

Доставляти signed encrypted operations адресатам, які можуть бути offline, без створення другого source of truth.

## State model

Delivery is not one linear chain. Direct delivery may skip mailbox storage, and none of these axes implies domain authority acceptance:

| Evidence | Meaning |
|---|---|
| `origin.committed` | Atomic local log + outbox commit. |
| `stored_for_delivery` | Configured mailbox/topology durably stored ciphertext; not Synchronized or Delivered. |
| `replica.applied` | Independent authorized application replica durably validated/applied the Operation; may satisfy Replication Policy. |
| `recipient.delivered` | At least one recipient-controlled Device decrypted, validated and durably stored the Operation. |
| `recipient.read` | Persona-level read operation after actual presentation; sender visibility follows privacy policy. |

1:1 `Delivered` is satisfied by the first qualifying Device receipt. Group delivery is aggregated per logical recipient as `N/M` against the message's immutable audience revision; exact Device topology is not exposed to the sender. Canonical receipt and bounded-finality semantics are defined by [SPEC-OPERATION-FINALITY-001](operation-finality-contract.md).

## Interfaces

```text
Enqueue(envelope, recipients, retentionClass)
Fetch(cursor, limits) -> EncryptedEnvelope[]
Ack(operationId, deviceId, state)
Retry(operationId)
Expire(policyCutoff)
```

All calls `MUST` be idempotent by operation/recipient/device key. Sender-visible aggregation additionally binds the logical recipient and immutable audience revision; Device binding is verified internally but is not disclosed to the sender. Service implementations `MUST NOT` decrypt payload or issue Space authorization.

## Failure behavior

- crash before `delivery.accepted` returns failure and creates no committed operation;
- crash after `delivery.accepted` is recoverable from outbox;
- direct and mailbox duplicates converge to one apply;
- out-of-order envelopes remain pending until causal dependencies arrive or reconciliation diagnoses a gap;
- quota/TTL rejection is explicit and observable.

## Security and privacy

Routing metadata `MUST` be minimized. Payload and domain-sensitive manifest fields `MUST` remain E2E encrypted. Node storage deletion semantics and unavoidable backups/caches `MUST` be documented.

## Acceptance criteria

- deterministic crash matrix for every transition;
- offline reconnect and multi-device fan-out tests;
- duplicate/out-of-order property tests;
- two-node failover prototype;
- node-side inspection finds no plaintext payload.

## Deferred

Topology, retention classes, quantitative SLO and canonical binary envelope encoding. Product-level replication/delivery aggregation is approved in [SPEC-OPERATION-FINALITY-001](operation-finality-contract.md).

Production or independently implemented adapters are blocked until `OQ-0027` and `OQ-0028` are closed and shared golden/conformance fixtures pass. `OQ-0032` product aggregation is resolved by `SPEC-OPERATION-FINALITY-001`.
