---
id: SPEC-vida-call-control
status: draft
companions:
  - call-state-machine.md
  - ../../planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md
  - ../../planning-artifacts/research/technical-vida-e2ee-media-stack-2026-09-22/research.md
  - ../../../docs/04-specifications/e2ee-calls-conformance.md
  - ../../../docs/04-specifications/operation-finality-contract.md
  - ../../../docs/02-requirements/native-client-requirements.md
sources: []
---

> **Decision-gated OQ-0072 kernel.** This draft defines observable call-control behavior and proof obligations; it does not select a media SDK, a serverless serialization mechanism, token TTL or numeric quality budget. The adopted companions remain normative where already approved.

# VIDA call control and exclusive answer

## Why

One Persona can receive a call on several equal Devices, including during a network partition. A device ring, transport packet or media-room token cannot decide which Device answered or whether the actor still has access to the owning Space. Release-1 E2EE calls need Core-owned, recoverable call state before a media provider may admit participants.

## Capabilities

- **CAP-1**
  - **intent:** Create and receive calls in the exact authorized conversation or resource context.
  - **success:** A signed call intent binds the owning Space, AppInstance, conversation/resource, grant scope, control frontier and key lineage; a wrong-context or revoked actor cannot ring, join or reconnect.
- **CAP-2**
  - **intent:** Let one of a Persona's equal Devices answer without duplicate participation.
  - **success:** Simultaneous and partitioned answers leave at most one Device with a verifiable accepted answer and publish-capable admission; absent proof, attempts remain pending.
- **CAP-3**
  - **intent:** Preserve call decisions across crash, retry and delayed delivery.
  - **success:** Replay of invite, answer, decline, cancel, transfer or end yields one coherent outcome per accepted revision without resurrecting a terminated ring or admitting a stale Device.
- **CAP-4**
  - **intent:** Restrict media to currently authorized Devices and key epochs.
  - **success:** A removed or transferred Device cannot obtain a new publishing admission or decrypt post-removal media; reconnect and provider-token reuse are negative fixtures.
- **CAP-5**
  - **intent:** Show the actual call state and reachability to the user.
  - **success:** UI distinguishes pending, ringing, accepted, connected, ended and missed from their respective Core/media evidence; anonymous-no-server does not claim background ringing or external push.

## Constraints

- `vida-core` owns call intent, authorization, exclusive answer/transfer and E2EE key epochs. `MediaSessionAdapter`, Iroh, OS calling UI and SFU implement mechanism, not domain authority.
- Devices of one Persona are equal peers; Device ID, client clock, delivery order and SFU arrival cannot choose an answer winner.
- Exclusive answer/transfer without the required `AuthorityOutcome` remains pending, including in a serverless partition; a pending answer cannot receive publish-capable admission.
- Durable call facts survive restart. Ringing, track state and network presence are transient observations and cannot establish finality; sync replay never launches a fresh call command.
- Release 1 requires E2EE 1:1 and up-to-eight-person group audio/video on Android, iOS and Windows; recording/screen sharing are absent. Autonomous anonymous mode has no external push registration.
- Media admission and local key storage must pass the adopted E2EE fixtures and OWASP mobile authorization, cryptography, storage and network checks; transport encryption alone is insufficient.

## Non-goals

- Choosing LiveKit, direct WebRTC or Iroh-native media by inference from this control contract.
- Inventing a serverless logical authority, timeout or wall-clock finality rule before OQ-0033/OQ-0072 proof.
- Claiming self-hosted SFU token expiration or participant removal immediately revokes every cached token or current media capability without tested enforcement.

## Success signal

One test harness and installed Android/iOS/Windows clients execute the same F07/F16/F17 schedules plus self-hosted token-reuse attacks: at most one accepted Device per Persona publishes, wrong-context or stale access fails closed, restart converges, and neither UI nor SFU invents an acceptance proof. Until the authority topology and media-admission gate pass, OQ-0072 remains open.

## Open Questions

- What logical authority and comparable proof serializes two answers from disconnected Devices of one autonomous Persona without permanently privileging a Device?
- Which authority-backed expiry and replay window makes an unanswered invite `missed` or `expired` when no common wall clock or always-online node can be trusted?
- Which tested admission/revocation mechanism prevents self-hosted provider-token reuse after Core grant loss, including an already connected participant?
