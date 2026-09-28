# Architecture currentness review — 2026-09-22

- Artifact: `../ARCHITECTURE-SPINE.md`
- Lens: current technology facts, official primary sources, and consistency with current repo decisions
- Verdict: **CONDITIONAL PASS** — 0 P0, 1 P1, 1 P2. The Iroh 1.2, Flutter platform, and media separation claims are current; one mobile wake rule needs a profile qualification before implementation.
- Scope: read-only review of the spine and cited/adjacent project artifacts; no source document was changed.

## P1 — AD-28 mobile wake baseline conflicts with anonymous Persona policy

**Location:** spine line 268 (AD-28); compare line 311 (AD-35), `docs/02-requirements/identity-requirements.md` line 87 (REQ-ID-018), and `docs/04-specifications/e2ee-calls-conformance.md` line 35 (REQ-CALL-013).

**Evidence:** AD-28 says Android baseline uses FCM wake and iOS uses APNs/background tasks without qualifying which Persona/profile may register a provider token. REQ-ID-018 forbids an `Autonomous anonymous` Persona from registering any Apple, Google, or other external push token/stable wake handle, for messages as well as calls; a `Public` profile requires explicit per-device consent. AD-35 applies the restriction to call ringing only. Official [Firebase priority documentation](https://firebase.google.com/docs/cloud-messaging/android-message-priority) describes FCM as an external push channel with limited wake time; [Apple background strategy documentation](https://developer.apple.com/documentation/backgroundtasks/choosing-background-strategies-for-your-app) does not make background execution a general always-on substitute.

**Impact:** A platform implementer following AD-28 could register push for an anonymous Persona, contradicting the accepted privacy boundary, or treat push wake as a prerequisite for baseline sync despite a mode that intentionally lacks it.

**Recommended fix:** Qualify AD-28: FCM/APNs wake is available only for a profile with an explicit consented provider binding; autonomous anonymous sync/calls are reachable only while the app/device can maintain the permitted direct/relay connection and reconcile on foreground/network wake. Include a negative fixture proving that anonymous contexts create no provider registration. Keep the existing no-100%-availability caveat.

## P2 — AD-35's new media evidence is absent from `sources`

**Location:** spine frontmatter lines 22–46 and AD-35 lines 307–311.

**Evidence:** The frontmatter lists the older transport, identity, and UI research and includes `e2ee-calls-conformance.md` as a companion, but omits the 2026-09-22 `technical-vida-e2ee-media-stack-2026-09-22/research.md` that directly supports AD-35's Iroh/control versus media split and the eight-person target. That research itself says the target is not a verified eight-client guarantee; the spine correctly phrases it as a target and gates selection on physical devices.

**Impact:** A later reviewer cannot follow the spine's explicit source list to the primary media comparison and may mistake the target for library-proven capacity.

**Recommended fix:** Add the media research to `sources`; retain the prototype gate and `OQ-3/OQ-0072` as written. The companion conformance spec already links the research.

## Verified current claims and limits

| Spine claim | Result | Primary evidence / limit |
|---|---|---|
| Iroh core `1.2.0` baseline; Endpoint, Router, ALPN and Address Lookup | **Confirmed current on 2026-09-22.** GitHub marks v1.2.0 latest; docs.rs exposes Endpoint, protocol/Router and Address Lookup. | [Iroh v1.2.0 release](https://github.com/n0-computer/iroh/releases/tag/v1.2.0), [versioned API](https://docs.rs/iroh/1.2.0/iroh/), [protocol model](https://docs.iroh.computer/concepts/protocols), [Address Lookup](https://docs.iroh.computer/concepts/address-lookup). Release page's heading dates the release notes 2026-09-09 while GitHub's publication badge says 2026-09-11; this is a metadata discrepancy, not a version conflict. |
| AD-20 Android/iOS/Windows installed Flutter clients | **Technically supported; product delivery remains unproven.** Flutter officially supports all three; ADR-0020 fixes their inclusion, with Windows-specific conformance still required. | [Flutter supported platforms](https://docs.flutter.dev/reference/supported-platforms); `docs/03-architecture/decisions/ADR-0020-flutter-windows-in-release-1.md`. Toolkit support does not prove VIDA's Rust FFI, desktop accessibility, packaging, or feature parity. |
| AD-35 Iroh control is not a media stack; SFU transport encryption is not automatically E2EE | **Confirmed as a sound separation.** Iroh documents encrypted QUIC application protocols; LiveKit separately documents media E2EE, application-owned key distribution, and server-readable signaling. | [Iroh protocol building blocks](https://docs.iroh.computer/concepts/protocols), [LiveKit encryption model](https://docs.livekit.io/transport/encryption/). The Flutter SDK advertises Android/iOS/Windows and native E2EE, but VIDA-specific key epochs and eight physical E2EE clients still require the named fixtures: [LiveKit Flutter SDK](https://github.com/livekit/client-sdk-flutter). |
| Deferring `iroh-live` as a release media choice | **Confirmed.** Upstream calls it an early tech preview; Windows has not been run and its relay lacks authentication. | [iroh-live README](https://github.com/n0-computer/iroh-live). |

## Source-path check

All relative paths under the spine's current `sources` and `companions` frontmatter resolve to existing local files. This check proves path presence, not that every cited document remains semantically current.
