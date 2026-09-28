---
id: RESEARCH-VIDA-E2EE-MEDIA-2026-09-22
status: completed-research-prototype-required
date: 2026-09-22
updated: 2026-09-22
verified_claims: 3
unverified_claims: 0
scope: Release-1 1:1 and group audio/video on Flutter Android/iOS/Windows
---

# VIDA E2EE calls: media, signaling and keying research

## Decision summary

No production stack is selected without cross-platform prototype evidence.

1. **First measured baseline:** self-hosted LiveKit SFU + official Flutter SDK; VIDA Core/Iroh owns signed call intent, membership, ringing state and E2EE key packages.
2. **Mandatory architectural control:** direct 1:1 with `flutter_webrtc` + VIDA/Iroh signaling/keying, while group calls use LiveKit. It is evaluated only to determine whether serverless 1:1 justifies two media implementations.
3. **R&D control:** `iroh-live`. Upstream describes it as an early tech preview with material Windows/iOS, A/V sync, room and relay gaps; it does not currently satisfy Release-1 gates.

## Architecture boundary

- WebRTC media uses ICE/STUN/TURN, DTLS-SRTP and optionally an SFU. Iroh is authenticated QUIC/NAT traversal infrastructure, not a production WebRTC media stack.
- `CallIntent` and `CallSession` live in VIDA Core. Iroh transports invite/accept/decline/cancel, participant-set facts, key packages and application receipts.
- `MediaSessionAdapter` is replaceable. LiveKit room, participant or track types do not leak into Core contracts.
- Direct 1:1 may use endpoint DTLS-SRTP. An SFU terminates hop encryption, so group E2EE requires client-side frame encryption; SFrame standardizes the frame layer but leaves key management to the application.
- Media keys originate from VIDA Core, are protected by platform keystores and rotate on join, leave, device revocation and suspected compromise. A joiner cannot decrypt pre-join media; a leaver cannot decrypt post-leave media.
- SFU tokens use opaque per-call room and participant IDs, short expiry and least privilege. Persona IDs, contact names, Space titles and chat content never enter SFU metadata.
- Screen sharing and recording are absent in v1: no UI/action, recorder principal/key, server egress deployment, Android MediaProjection permission or iOS broadcast extension.

## Candidate analysis

### A. Self-hosted LiveKit baseline

Strengths: Apache-2.0 server and Flutter SDK; Android/iOS/Windows support; SFU, TURN and native E2EE capability; one media implementation for 1:1 and groups.

Risks: always-online SFU/session service; signaling metadata is server-readable; VIDA must provide key generation, distribution, rotation and identity binding; operational TLS/UDP/TCP/TURN capacity is required.

### B. Direct 1:1 plus LiveKit groups

Strengths: serverless direct 1:1 where ICE succeeds; MIT `flutter_webrtc`; retains SFU for group scalability.

Risks: two media session implementations; duplicated reconnect, routing and E2EE behavior; TURN is still required for hard NAT/corporate networks; Windows/mobile behavior requires measurement.

### C. Iroh-native media

Strengths: Rust/Iroh alignment and permissive licensing.

Current blockers: upstream early-tech-preview status, Windows mostly missing, iOS untested, basic A/V sync, lightly tested rooms, unauthenticated relay and unreleased dependencies.

## Platform constraints

- iOS terminated/background incoming calls require APNs PushKit and CallKit. Approved VIDA boundary: `Autonomous anonymous` never registers external push/wake bindings and therefore receives calls only while directly reachable. Push-assisted ringing is available only to an explicitly consented `Public` profile with isolated Persona-scoped registration.
- Android incoming calls use supported Telecom/Core-Telecom, CallStyle and foreground-service paths; Doze/force-stop behavior must be explicit.
- Windows Release 1 proof targets supported x64. ARM64 is not silently promised without evidence.

## Security and privacy gates

The conformance gate covers: ciphertext-only SFU/TURN/signaling observation; wrong/stale key failure; join/leave/revoke rotation; signed VIDA transcript binding; replay/expired token denial; no media capture before consent; platform keystore; minimized metadata/logs; no SDP/IP/key/media diagnostics without explicit local preview; absence of recording/screen-capture capability.

OWASP MASVS Network, Crypto and Storage controls are verification baselines, not stack-selection substitutes.

## Prototype order

1. LiveKit thin slice: Android↔iOS↔Windows 1:1 and proposed eight-party small-team group with externally supplied E2EE keys.
2. Add VIDA/Iroh control, identity transcript, key epochs and multi-device ringing.
3. Add iOS/Android/Windows lifecycle, hosted-push versus anonymous/no-server profiles, accessibility and OS calling integration.
4. Run adverse network, crash, metadata, resource and mixed-version fixtures.
5. Run hybrid direct-1:1 control against identical fixtures; quantify whether its decentralization benefit justifies the dual stack.

## Group capacity extension — 2026-09-22

LiveKit's official 150×150 synthetic server benchmark shows that an eight-person room is not SFU-bound, but it does not test Flutter decoding, rendering, thermals, TURN, E2EE or long calls.[1][2] Signal permits 75 participants but rings only small groups up to 16; Slack permits 50 participants but only 25 cameras; Teams and Meet decouple attendance from visible tiles and reduce galleries under device/network pressure.[3][4][5][6] The shared pattern is that vendor attendance ceilings are not end-client usability guarantees.

Two independent research passes recommend a **Release-1 target of 8 total participants**, with all eight allowed to publish audio/video, UX optimized for 4–6 active discussants, one-page desktop grid and active-speaker/filmstrip/swipe on mobile. Twelve/sixteen become stretch targets only after the same physical-device/E2EE/network fixtures pass. Confidence: medium-high for the recommendation; the product pattern is verified, but no primary source publishes a real Flutter E2EE eight-client guarantee.

Required proof: eight physical Flutter clients for 60 minutes; low/mid/high mobile classes plus Windows; E2EE; 720p simulcast; adaptive stream/dynacast; Wi-Fi, cellular and forced TURN; reconnect; raw CPU/thermal/battery/memory/freeze/audio/RTC stats. Until proof, wording is “designed for 8,” not “supports 8.”

## Open product decisions

- The Release-1 product target is approved at 8 total participants with UX optimized for 4–6; 12/16 remain post-Release-1, post-proof stretch targets.
- Numeric call-quality/resource thresholds belong to OQ-4 and must come from representative builds.
- LiveKit frame E2EE must not be claimed RFC 9605/SFrame interoperable without protocol verification and crypto review.

## Primary sources

- [RFC 8835: Transports for WebRTC](https://www.rfc-editor.org/rfc/rfc8835.html)
- [RFC 9605: SFrame](https://www.rfc-editor.org/rfc/rfc9605.html)
- [Iroh Endpoint](https://docs.rs/iroh/latest/iroh/endpoint/index.html)
- [LiveKit server](https://github.com/livekit/livekit)
- [LiveKit Flutter SDK](https://github.com/livekit/client-sdk-flutter)
- [LiveKit encryption](https://docs.livekit.io/transport/encryption/)
- [LiveKit E2EE start](https://docs.livekit.io/transport/encryption/start/)
- [LiveKit self-hosting](https://docs.livekit.io/transport/self-hosting/deployment/)
- [flutter_webrtc](https://pub.dev/packages/flutter_webrtc)
- [iroh-live](https://github.com/n0-computer/iroh-live)
- [Signal encrypted group calls](https://signal.org/blog/how-to-build-encrypted-group-calls/)
- [Signal RingRTC](https://github.com/signalapp/ringrtc)
- [Element Call LiveKit branch](https://github.com/element-hq/element-call/tree/livekit)
- [mediasoup](https://github.com/versatica/mediasoup)
- [Apple PushKit VoIP notifications](https://developer.apple.com/documentation/pushkit/responding-to-voip-notifications-from-pushkit)
- [Android Telecom for VoIP](https://developer.android.com/develop/connectivity/telecom/voip-app/telecom)
- [Android CallStyle](https://developer.android.com/develop/ui/compose/notifications/call-style)
- [OWASP MASVS Network](https://mas.owasp.org/MASVS/08-MASVS-NETWORK/)
- [OWASP MASVS Crypto-2](https://mas.owasp.org/MASVS/controls/MASVS-CRYPTO-2)
- [OWASP MASWE platform keystore](https://mas.owasp.org/MASWE/MASVS-STORAGE/MASWE-0003/)

## Capacity source appendix

| Ref | Finding | Publisher | Date | Accessed | Confidence |
|---|---|---|---|---|---|
| [1] | 150 publishers + 150 subscribers at 720p on stated 16-core node; synthetic server benchmark | [LiveKit](https://docs.livekit.io/transport/self-hosting/benchmark/) | not stated | 2026-09-22 | high for lab result; low for client UX |
| [2] | Load tool supports publishers/subscribers, simulcast and 3×3/4×4/5×5 layouts | [LiveKit](https://github.com/livekit/livekit-cli#load-testing) | continuously updated | 2026-09-22 | high |
| [3] | Group cap 75; ringing notification only for groups up to 16 | [Signal](https://support.signal.org/hc/en-us/articles/360052977792-Group-Video-Calling) | not stated | 2026-09-22 | high |
| [4] | Paid huddles 50; at most 25 video; 5+ video bandwidth guidance | [Slack](https://slack.com/help/articles/4402059015315-Use-huddles-in-Slack) | not stated | 2026-09-22 | high |
| [5] | Desktop gallery tiers 4/9/16/49; mobile featured-video limits and resource adaptation | [Microsoft](https://support.microsoft.com/en-us/teams/meetings/use-video-in-microsoft-teams) | not stated | 2026-09-22 | high |
| [6] | Dynamic/tiled layouts decouple visible video tiles from total attendance and advise reducing tiles under client pressure | [Google](https://support.google.com/meet/answer/9292748?co=GENIE.Platform%3DDesktop&hl=en) | not stated | 2026-09-22 | high |

## Capacity staleness map

- Performance and compatibility evidence: re-check by **2026-10-22**.
- Eight-participant selection recommendation: re-check by **2027-03-22**, and earlier if the LiveKit/Flutter/E2EE profile changes.
- Computed source: [claims-capacity.json](claims-capacity.json); current stale claims: `0`.
