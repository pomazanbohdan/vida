---
id: SPEC-E2EE-CALLS-CONFORMANCE-DRAFT
status: draft
implementation_status: prototype-required
last_updated: 2026-09-25
requirement_refs:
  - ../02-requirements/native-client-requirements.md
  - ../02-requirements/browser-client-requirements.md
  - ../02-requirements/platform-nfr.md
research_refs:
  - ../../_bmad-output/planning-artifacts/research/technical-vida-e2ee-media-stack-2026-09-22/research.md
contract_refs:
  - operation-finality-contract.md
---

# E2EE calls conformance gate

## Purpose and status

This draft defines the same evidence for every Release-1 media candidate. It fixes the product capacity target but does not select LiveKit, `flutter_webrtc`, `iroh-live` or numeric quality budgets.

## Normative boundaries

| ID | Requirement |
|---|---|
| REQ-CALL-001 | VIDA Core `MUST` own signed call intent, participant/device authorization, state transitions and E2EE key epochs; media SDK types `MUST NOT` become Core domain types. |
| REQ-CALL-002 | Iroh `MAY` carry VIDA call control and key packages but `MUST NOT` be treated as proof that RTP/WebRTC media requirements are satisfied. |
| REQ-CALL-003 | `MediaSessionAdapter` `MUST` be replaceable and expose versioned Core-owned commands/events for create, join, accept, decline, leave, reconnect, participant and track state. |
| REQ-CALL-004 | 1:1 and group audio/video media `MUST` be E2EE. SFU, TURN, signaling and logs `MUST NOT` recover media plaintext or media keys. Hop DTLS-SRTP alone is insufficient when an SFU can access decoded frames. |
| REQ-CALL-005 | Media keys `MUST` originate from VIDA-authorized key state, use platform-protected storage and rotate on join, leave, kick/device revoke and suspected compromise. Joiners `MUST NOT` decrypt pre-join media; leavers `MUST NOT` decrypt post-leave media. |
| REQ-CALL-006 | A signed VIDA call transcript `MUST` bind call ID, opaque media room ID, immutable owning Space/AppInstance/conversation-or-Resource ID, effective grant scope/control frontier, authorized Persona/device set, adapter profile, key-epoch lineage and expiry. Join, reconnect, invite and transfer `MUST` recheck this Core-owned context; a UI relation never grants call access. Replay, wrong-Space/resource, mismatched identity/room, expired token and removed device `MUST` fail closed. |
| REQ-CALL-007 | SFU/session tokens `MUST` be short-lived and least-privilege. SFU-facing identifiers `MUST` be per-call opaque; Space title, contact name, chat content and stable Persona ID `MUST NOT` be disclosed. |
| REQ-CALL-008 | Multi-device ringing `MUST` converge on exactly one authority-accepted Device per Persona unless an explicit transfer occurs. Answer/transfer are exclusive Core transitions with a verifiable acceptance proof; a partitioned attempt without proof remains pending and cannot obtain a publishing media token. Losing devices stop ringing, invalidate tokens/keys and do not join as ghost participants; an SFU cannot choose the winner. |
| REQ-CALL-009 | Android, iOS and Windows implementations `MUST` use supported OS incoming-call, audio-session, permission and lifecycle APIs. Microphone/camera capture before user consent is forbidden. |
| REQ-CALL-010 | Recording, screen sharing, webinar and broadcast capabilities `MUST` be absent from Release 1 UI, permissions, service deployment and key principals. |
| REQ-CALL-011 | Diagnostics `MUST` redact media, keys, stable identity, Space/contact labels and SDP/IP data unless the user explicitly previews and sends a local diagnostic bundle under diagnostics policy. |
| REQ-CALL-012 | Numeric quality/resource thresholds belong to OQ-4. This gate `MUST` record raw measurements without inventing final budgets; correctness, E2EE, reachability and lifecycle hard gates remain mandatory. |
| REQ-CALL-013 | `Autonomous anonymous` Persona `MUST NOT` register external push/wake bindings. Incoming calls are best-effort only while a device is directly reachable. Push-assisted background/terminated ringing `MAY` exist only for an explicitly consented `Public` profile, with Persona-scoped non-reused provider registration. |
| REQ-CALL-014 | Release 1 `MUST` support group audio/video calls with up to 8 total participants, while interaction/layout `SHOULD` be optimized for 4–6. All 8 authorized participants `MUST` be able to publish audio/video subject to explicit device controls. Claims above 8 are post-Release-1 stretch scope and require a new measured decision. |
| REQ-CALL-015 | Durable invite, accept, decline, cancel, transfer and terminal outcome `MUST` use `SPEC-OPERATION-FINALITY-001` evidence axes and survive crash/replay; transient ring/track/presence signals `MUST NOT` become durable acceptance. The UI `MUST` distinguish pending, ringing, accepted, connected, ended and missed from actual Core/application evidence. Exact expiry and state wire encoding remain OQ-0072. |
| REQ-CALL-016 | OQ-0072 `MUST` define the logical authority and comparable order/proof for competing answers, partition behavior, token/key invalidation on loss, and replay after crash before a Release-1 call profile can be approved. No Device priority or SFU arrival order is an authority rule. |
| REQ-CALL-017 | Static Flutter Web is a full Release-1 calling client. Its media profile `MUST` pass browser permission-denial, microphone/camera, E2EE/key custody, inactive-tab/suspend/reload/reconnect, mixed Web↔installed participants and browser accessibility fixtures. Browser transport relaying or a working native media SDK does not prove Web media conformance; failure blocks Release-1 Web acceptance. |

## Candidate profiles

- `A-livekit`: self-hosted LiveKit for 1:1 and groups; first measured baseline.
- `B-direct-plus-livekit`: direct `flutter_webrtc` for 1:1 plus LiveKit groups; mandatory architectural control after A.
- `C-iroh-live`: R&D control; it cannot be selected while upstream/platform/security hard gates fail.

Versions and artifacts `MUST` be pinned. A skipped candidate/fixture requires an explicit incompatibility result, never an assumed pass.

## Hard gates

`G1` E2EE/key lifecycle; `G2` physical Android/iPhone/Windows and supported-browser Web behavior; `G3` ICE/TURN/SFU reachability; `G4` mobile/desktop/browser lifecycle and consent; `G5` identity/replay authorization; `G6` multi-device convergence; `G7` network/quality recovery; `G8` privacy/operations; `G9` forbidden-scope absence; `G10` license/SBOM/reproducibility. Any failure rejects the profile.

## Weighted rubric

`E2EE/key/identity 20 + platforms 16 + reachability/quality 15 + group scaling 12 + OS lifecycle 10 + Rust/Iroh boundary 9 + decentralization/privacy 8 + operations 5 + resources 3 + supply chain 2 = 100`.

Passing score is `>=80` with no hard-gate failure. Lead `<8` remains inconclusive. Repeat on representative low/mid/high mobile device classes and Windows x64.

## Required fixtures

Machine-readable source: [e2ee-calls-v1.yaml](fixtures/e2ee-calls-v1.yaml).

F01–F18 cover cross-platform pair/group calls, including the approved 8-participant group target, E2EE observation, protected key epochs and compromise rotation, transcript/token attacks, hostile networks with qualitative continuity cases, multi-device ringing and exclusive-answer proof, hosted-push versus anonymous/no-server lifecycle, accessibility, permissions, audio routes, crash recovery, metadata, raw resource baselines, forbidden-scope absence, mixed versions, supply-chain evidence, wrong-Space/resource-context rejection, durable-versus-transient signaling replay and the supported-browser Web media boundary.

## Approval evidence

Approval requires reproducible source/lockfiles, exact devices/OS/toolchains, candidate configuration, packet/log captures with secrets removed, fixture results, raw quality/resource measurements, accessibility/OS integration evidence, SBOM/license record, threat-model review and an ADR selecting the media profile and Core↔adapter boundary.
