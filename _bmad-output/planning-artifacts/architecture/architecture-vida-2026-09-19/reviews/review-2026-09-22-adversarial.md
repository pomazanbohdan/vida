---
review: release-1-two-compliant-units-adversarial
artifact: ../ARCHITECTURE-SPINE.md
date: 2026-09-22
verdict: conditional-architecture-pass; call-control-contract-blocked
---

# Release 1: two compliant units

## Verdict

AD-20 fixes installed Flutter Android/iOS/Windows clients, and AD-35 correctly keeps call authority in Core rather than in a media SDK. The CRDT and media choices are honestly prototype-gated. Nevertheless, two independently built Release-1 units can satisfy every current AD and still disagree on call authorization/context and simultaneous answers. Those seams need a normative Core contract and cross-platform fixtures before a call profile or Release-1 interoperation claim. This review identifies specification gaps, not observed implementation defects.

## Construction used for the test

- **Unit A:** Android Flutter Messenger and Notes shell through `vida-sdk`, shared Rust Core/runtime, and a conforming `MediaSessionAdapter`; it starts a call from a conversation opened through a Project relation in a dedicated Space. Its Notes editor sends versioned intents to the Rust collaborative-document port.
- **Unit B:** Windows Flutter Project/Messenger and Notes shell through the same required facade and Rust semantics, with an independently written platform/media adapter; it opens the same conversation from a cross-Space search or resource relation. A second authorized Device of one Persona may answer independently from A while B is partitioned.
- Both units respect the eight-person call cap, E2EE, signed Core call intent, current local grant, durable operation evidence axes, and the provisional CRDT/media gates. No browser client, WinUI Release-1 shell, device priority or SFU domain authority is assumed.

## High findings

### H1 — A call has no normative owning Space/conversation binding

**Evidence:** AD-35 ([spine:307–311](../ARCHITECTURE-SPINE.md)) requires Core participant/Device authorization and key epochs but does not bind call intent to the owning `SpaceId`, `AppInstanceId`, conversation/resource ID, and effective control frontier. `REQ-CALL-006` in `e2ee-calls-conformance.md` binds a call transcript to call/room ID, participant set, adapter profile, epoch and expiry, but omits the owning context. The UX allows relation navigation while preserving Space context (`EXPERIENCE.md:118`), and AD-32 lets Messenger be a dependent AppInstance in a dedicated Space.

**Divergence:** A authorizes the call under the Project's active Space; B resolves the relation's target conversation under a different Space or a resource-scoped Guest grant. Both can form a valid-looking signed call transcript with different membership/key sources. Cross-Space replay or accidental disclosure becomes possible if media tokens or key packages are accepted under the wrong context.

**Closure:** Define one Core-owned call-context identity and authorization source: immutable call → owning Space/AppInstance/conversation binding, grant scope, control frontier, and key-epoch lineage. Join, reconnect, transfer and invite must revalidate that binding; a relation or UI-selected Space alone confers no call access. Add wrong-Space, shared-resource Guest, revoked-member and dedicated-Space dependency fixtures to `OQ-0072`/the call gate. Keep opaque per-call identifiers toward the SFU.

### H2 — Concurrent answers lack a Core serialization/failure rule

**Evidence:** `REQ-CALL-008` requires exactly one accepted Device per Persona, and fixture F07 exercises simultaneous answer, but neither specifies the logical authority, competing-answer order/proof, or partition behavior. AD-21 rejects device rank and arbitrary peer finality; AD-35 rejects SFU/media authority. `SPEC-OPERATION-FINALITY-001` requires a serialized logical authority for exclusive outcomes but does not explicitly classify call answer as such.

**Divergence:** A and B answer the same ringing call while partitioned. Each locally declares itself accepted and receives a media key; on reconnection they each revoke the other. Both were following signed Core transitions and F07's desired outcome in their local view, but for an interval two devices can publish, the participant count differs, and the loser may retain an active media token.

**Closure:** Classify answer/transfer as a named exclusive Core transition with one authority/acceptance proof and an explicit `pending`/unavailable behavior. State when a losing token/key is invalidated and how accepted state is replayed after crash. Test concurrent answer, partition, late acceptance, reconnect and one-Device transfer against the same deterministic oracle; SFU admission must consume Core proof, not generate it.

## Medium findings

### M1 — Call signaling finality is not mapped to the five evidence axes

AD-35 says signed call intent and Core state transitions; AD-7 separates transient signals from durable operations. Neither says which call transitions are durable domain operations, which are ephemeral notifications, and what a UI may call `ringing`, `accepted`, `connected`, `ended` or `missed` after crash/replay. A could persist invite/cancel and show a missed call; B could treat both as TTL-only signaling. Both obey AD-35 yet produce incompatible histories and stale ring behavior. Bind call control to `SPEC-OPERATION-FINALITY-001` where durability/authority is required; define transient-only events, expiry and observable states. Add direct/mailbox duplicate, cancel-after-answer, restart and old-invite replay fixtures. This can be part of OQ-0072; the media SDK selection alone cannot settle it.

### M2 — Collaborative update context can differ across Space boundaries

`REQ-COLLAB-001/005` require a Note authorization boundary, and F07 rejects cross-document bytes. The current `crdt-editor-v1.yaml:33` update envelope lists `document_id`, actor/device, epoch, schema, frontier, hash and signature but does not state a signed Space/Resource binding or globally unique document-ID registry. A may treat `document_id` as Space-local; B may treat it as global. A relation to a shared Note may then select the wrong authorization context even though each editor passes its local F07. Specify an immutable canonical `(SpaceId, ResourceId, DocumentId)` binding (or an equally unambiguous signed derivation) and cross-Space collision/replay fixtures before approving the CRDT/editor pair. This is a contract/fixture refinement, not a reason to choose a CRDT prematurely.

## Gates that already contain divergence

- AD-20 excludes a first-party Release-1 browser/PWA or separate WinUI client; Android/iOS/Windows platform proof remains mandatory.
- AD-4/5/13/31 and `SPEC-OPERATION-FINALITY-001` separate origin durability, replication, recipient delivery, domain authority and external effect. The 25-vector implementation gate at spine:333 blocks a false `Synchronized` or authority claim from transport ACK alone.
- AD-35 plus the E2EE call gate block a media SDK/SFU from becoming Core authority and block a non-E2EE profile. OQ-3/OQ-0072 still require physical device/security evidence and a selected media profile; no candidate is approved yet.
- The CRDT/editor gate leaves Loro/Automerge/Yrs and the Flutter editor adapter open until paired Android/iOS/Windows evidence passes. Its YAML explicitly remains a scenario contract pending executable canonical serialization/assertions. The gate must not be reported as a completed conformance run.

## Gate disposition

Architecture direction: **conditional pass**. Independent Release-1 call control and media-profile selection: **blocked by H1/H2** until the Core context and exclusive-answer rules plus fixtures close. CRDT/editor selection: **prototype-gated**, with M2 to incorporate into its authorization fixtures. No change to AD-20's platform decision or to the selected media/CRDT candidate is proposed.
