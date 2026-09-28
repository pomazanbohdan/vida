---
id: SPEC-vida-mobile-availability
status: draft
companions:
  - availability-conformance-cases.md
  - ../../../docs/04-specifications/sync-presence-status-model.md
  - ../../planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md
  - ../../../docs/02-requirements/platform-nfr.md
  - ../../../docs/02-requirements/identity-requirements.md
  - ../../../docs/02-requirements/transport-sync-requirements.md
sources: []
---

> **Decision-gated OQ-0064/0067 kernel.** Approved requirements and adopted architecture control over this draft. The cases are plans, not passed installed-device evidence or selected timing constants.

# VIDA mobile availability and truthful presence

## Why

Messenger, Notes and Projects need prompt synchronization while devices can run, but Android and iOS can suspend or stop a client. A transport socket, push hint or second copy must not be sold as current reachability, completed synchronization or business acceptance. The product needs rapid recovery **and** truthful state for each Persona.

## Capabilities

- **CAP-1**
  - **intent:** Keep or restore authenticated sync whenever the OS permits execution.
  - **success:** Network change, wake and foreground paths re-evaluate Iroh direct/relay connectivity, then bounded reconnect and durable reconciliation; interruption loses no pending operation and creates no duplicate business operation.
- **CAP-2**
  - **intent:** Show a truthful online count for one visible Persona.
  - **success:** Only a current user-controlled device with fresh authenticated VIDA sync-capability proof counts; iOS suspension, stale/replayed proof, transport bootstrap, bots and devices of other Personas cannot inflate it.
- **CAP-3**
  - **intent:** Let a user choose additional Android availability and permitted Public-Persona wake without making either mandatory.
  - **success:** Android per-device mode can be declined, enabled or disabled in onboarding/settings; allowed background handling meets current OS/store constraints; Autonomous anonymous Persona never registers an external push/wake binding, and Public push requires metadata preview and per-device consent.
- **CAP-4**
  - **intent:** Explain temporary loss and recovery without merging unrelated status dimensions.
  - **success:** Owner-only reconnecting, public count, local durability, application sync, delivery and domain acceptance remain separately testable; a device with one local copy never claims synchronized solely because its profile is online.

## Constraints

- `VidaNodeHost` owns endpoint lifecycle; shells signal OS lifecycle/network changes, while Rust runtime retains durable pending work and performs reconciliation. Iroh transport state does not authorize a Persona or certify an application receipt.
- Android high-availability is voluntary and needs a supported foreground-service use case/type, user-visible behavior where required and store-policy review. Baseline correctness cannot depend on an overlay, VPN privilege or permanent service.
- iOS public-online means active client plus controlled VIDA connection; APNs/background scheduling may wake bounded work but never establishes online status by itself. No platform has a literal 100% mobile background guarantee.
- Current Google Play and Apple App Review background/resource policies are rechecked before every store submission; an earlier approval or OS API capability is not a permanent policy waiver.
- Public presence counts only fresh proofs within one visible Persona; reconnecting details remain owner-only, raw device/endpoint identifiers and cross-Persona correlation are not disclosed by count.
- Reconnect must attempt Iroh path migration before application retry; retry uses bounded exponential backoff with jitter/cap and resets on new network/proven connection. Numeric values await measured platform profiles.

## Non-goals

- Selecting a fixed lease TTL, retry interval, battery/data budget, Android foreground-service type or universal wake guarantee without installed-build and store-policy evidence.
- Treating presence, push delivery or transport ACK as `delivery.applied`, `Synchronized`, authority acceptance or external-effect completion.
- Granting ordinary VIDA the lifecycle privileges of a VPN, OS daemon or floating-window feature.

## Success signal

Installed Android and iOS clients, with Windows as the persistent-connection control, pass the same semantic availability/privacy cases under foreground, network handover, suspension, force-stop, relay outage and restart. Reports distinguish observed path/proof/retry/outbox state from unverified promises; OQ-0064/0067 stay open until measured thresholds and UX decisions are approved.

## Open Questions

- Which measured freshness/expiry, retry/backoff and owner-visible unknown-state thresholds should each platform profile use (`OQ-0064/0067`)?
- Which count update cadence/coalescing and optional privacy controls limit timing correlation without misrepresenting the approved exact count (`OQ-0064`)?
- What battery/mobile-data budgets and force-stop recovery UX pass the representative vertical-slice gate (`OQ-0067/0076`)?
- What proves a single-device Persona currently reachable without inventing a peer-specific receipt from Iroh bootstrap (`OQ-0064`)?
