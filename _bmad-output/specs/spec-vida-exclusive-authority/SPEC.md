---
id: SPEC-vida-exclusive-authority
status: draft
companions:
  - authority-options-and-cases.md
  - ../../../docs/03-architecture/decisions/ADR-0016-equal-device-peers.md
  - ../../../docs/03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md
  - ../../../docs/04-specifications/operation-finality-contract.md
  - ../../../docs/04-specifications/device-sync-session.md
  - ../../planning-artifacts/research/technical-vida-multi-device-federation-ownership-c-2026-09-19/research.md
sources: []
---

> **Decision-gated OQ-0033 kernel.** This defines the already accepted boundary for exclusive outcomes and a testable comparison. It does not choose a serverless quorum, a permanent service or a wire certificate.

# VIDA exclusive-operation authority

## Why

Offline Devices must preserve user intent, yet two disconnected peers cannot each promise the same scarce slot or exclusive transition. VIDA needs one business-level confirmation rule that remains valid for a Personal Space, a shared serverless Space and an externally owned resource without making a physical Device the master.

## Capabilities

- **CAP-1**
  - **intent:** An authorized Device can durably create and exchange an exclusive-operation request while offline.
  - **success:** The request survives restart and duplicate delivery under one stable identity, but is shown as pending until its named authority issues an outcome.
- **CAP-2**
  - **intent:** The named Resource Authority can issue a verifiable outcome for one request revision.
  - **success:** Peers with the same accepted control and outcome proof agree on `accepted|rejected|expired` for the exact `RequestId`, revision and frontier regardless of transport arrival order; `expired` requires an explicit domain deadline or precondition policy, not candidate age alone.
- **CAP-3**
  - **intent:** Users can distinguish a locally saved or synchronized request from a confirmed exclusive result.
  - **success:** Neither isolated peer group labels a booking/claim confirmed merely because it was saved, replicated or acknowledged by Iroh; reconnect exposes the authority outcome or continues to show pending.
- **CAP-4**
  - **intent:** Current membership and rights govern acceptance even when a request was authored earlier.
  - **success:** Stale grant/epoch, revoked actor or invalid authority proof cannot accept a pending request; preserving a valid pre-revocation outcome requires the still-open verified control order that defines the effective cut.

## Constraints

- Owner is a Space role of a Persona, not a favored phone, laptop, endpoint or replica. Equal Devices have no built-in master or merge priority under [ADR-0016](../../../docs/03-architecture/decisions/ADR-0016-equal-device-peers.md).
- An exclusive claim/approval stays `domain.pending` until the named logical Resource Authority signs the exact outcome. Without a reachable serialized logical authority in a serverless Space it stays pending; no clock, arrival order, local durability or replica ID invents a winner.
- Offline candidates have no age-only expiry (OQ-0054). `expired` is a signed authority outcome only under a defined domain deadline or stale-precondition policy; otherwise the request remains pending until an authorized decision.
- The per-process `ConfirmationPolicy` defines approver conditions, not an alternative Resource Authority. The named authority remains accountable for issuing/serializing the exact terminal outcome; Admin configuration cannot replace it or bypass Core validation.
- Local durability, replication, delivery, domain authority and external effect are separate evidence axes. Iroh transport/relay and a mailbox cannot grant Resource authority.
- At acceptance, Core rechecks signature, current grant/control state, schema/policy and causal preconditions. A stale offline candidate is recoverable but not automatically valid.
- The effective ordering of concurrent grants/revocations between partitioned groups remains open. A historical acceptance is preserved only when a later verified control order proves it preceded the revocation cut.
- Mergeable content and incomparable status/file branches follow their own OQ-0034/ADR-0017 rules; a CRDT internal winner is not business approval. Replica quorum, if chosen, is not M-of-N approval votes by Owners.

## Non-goals

- Choosing a serverless authority topology, quorum size, member rotation, signed certificate format or first-release operation-family catalog here.
- Replacing accepted offline merge behavior with global locking for every Note, chat message or Task field.
- Treating an Iroh ACK, cloud relay, mailbox storage or two Devices of one Persona as sufficient booking approval.

## Success signal

In a test with two disconnected peer groups requesting the same exclusive resource, both retain recoverable requests but neither side claims unsupported confirmation. After the selected authority mechanism can act, all authorized peers verify the same bounded outcome and a stale/revoked participant cannot counterfeit it. The comparison fixtures are not yet executed, so OQ-0033 stays open.

## Open Questions

- For each exclusive operation family, who owns the logical right to confirm: Personal Persona, shared Space process or external resource owner?
- In a shared serverless Space, is waiting for reconnection sufficient, or must an intersecting replica quorum confirm while a majority is reachable?
- If quorum is required, who counts as a member, how do removal/recovery and epoch changes work, and what certificate proves a single current frontier?
- How is the effective grant/revocation cut ordered and proved across partitioned peers so that a historical acceptance can be distinguished from a stale one?
- Which user-facing pending/confirmed/correction states apply to each exclusive family without confusing synchronization with approval?
