---
id: ADR-0006
status: accepted
last_updated: 2026-09-19
source_refs:
  - ../../../_bmad-output/planning-artifacts/research/technical-iroh-ecosystem-and-project-patterns-for-2026-09-18/research.md
decision_refs:
  - ADR-0003-offline-revocation.md
  - ADR-0005-iroh-transport-foundation.md
supersedes: []
superseded_by: []
---

# ADR-0006: Durable delivery and unified operation envelope

## Context

Iroh relay stateless і не зберігає payload для offline recipient. Direct P2P, relay forwarding і reconnect не утворюють durable messaging semantics самі по собі.

## Decision drivers

- offline-first availability;
- відсутність split authority між direct та mailbox paths;
- crash safety, retry та deduplication;
- E2E encryption на federated delivery nodes;
- однакова модель для messages, notes, tasks і membership operations.

## Considered options

1. Лише direct Iroh delivery.
2. Використовувати Iroh relay як mailbox.
3. Окремий encrypted durable-delivery service над Iroh transport.

## Decision

Direct Iroh `MUST` бути fast path. Offline delivery `MUST` виконувати окремий `DurableDelivery` service, який зберігає ciphertext і `MUST NOT` бути domain authority.

Direct і durable paths `MUST` переносити той самий signed operation envelope та stable operation ID. Local accepted log і outbox `MUST` commit-итися атомарно. Receiver `MUST` застосовувати operations idempotently.

Delivery ACK states `MUST` розрізняти `delivery.accepted`, `delivery.stored`, `delivery.applied`, `delivery.read`. `delivery.accepted` означає atomic origin log+outbox commit і `MUST NOT` трактуватися як `authority.accepted` з ADR-0003. Multi-device fan-out `MUST NOT` створювати нову domain operation. Pull/reconcile `MUST` відновлювати durable state після пропущених transient events.

## Rationale

Один operation contract усуває дублікати, reorder divergence та втрату між local commit, direct send і mailbox enqueue.

## Consequences

- потрібна atomic log/outbox transaction;
- mailbox topology є окремим deployment decision;
- TTL, quota, retry, deletion та abuse policy входять до service contract;
- read receipt не визначає application truth;
- node compromise не повинен відкривати plaintext.

## Risks і mitigations

- direct/mailbox race → stable operation ID, idempotent apply, causal metadata;
- node loss → replicated topology prototype and failover test;
- metadata leakage → minimized envelope routing metadata and threat model;
- deletion ambiguity → explicit ciphertext retention/tombstone contract.

## Acceptance evidence

- crash test на кожній межі `log → outbox → stored → applied` не втрачає operation після `delivery.accepted`;
- одночасна direct/mailbox delivery створює один apply;
- offline recipient отримує operation після reconnect;
- втрата одного з двох prototype delivery nodes не порушує configured durability;
- delivery node не може прочитати E2E payload.

## Open questions

- single, replicated, keeper-peer або authoritative mailbox topology;
- replication factor та availability SLO;
- routing metadata, jurisdiction і retention limits;
- canonical envelope/schema.
