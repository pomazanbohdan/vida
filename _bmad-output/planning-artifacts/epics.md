---
stepsCompleted:
  - step-01-validate-prerequisites
  - step-02-design-epics
inputDocuments:
  - prds/prd-vida-2026-09-22/prd.md
  - prds/prd-vida-2026-09-22/addendum.md
  - architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md
  - ux-designs/ux-vida-2026-09-22/DESIGN.md
  - ux-designs/ux-vida-2026-09-22/EXPERIENCE.md
  - ../../../docs/03-architecture/decisions/ADR-0021-static-web-client-in-release-1.md
  - ../specs/spec-vida-tor-network-mode/SPEC.md
  - ../../docs/02-requirements/tor-anonymous-mode-requirements.md
extractionStatus: confirmed-tor-delta-2026-09-28
epicDesignStatus: approved-2026-09-25
epicTorDeltaStatus: approved-2026-09-28
storyElaborationScope: rolling-wave-epics-1-2-first
epic2StoriesStatus: stories-2.1-to-2.12-approved-tor-2.13-to-2.15-drafted-platform-fanout-pending-2026-09-28
story1BootstrapRevisionStatus: approved-2026-09-26
epics12ImplementationReadiness: decision-gated-2026-09-26
---

# VIDA - Epic Breakdown

## Overview

This document decomposes Release-1 requirements into user-valued epics and, after epic approval, implementable stories. The 2026-09-25 decision adds a full static, peer-synchronizing Web client to Release 1. The product owner approved the ten-epic structure below on 2026-09-25; it covered the original 39 FRs, including FR-39: Epic 2 delivers the first useful Android↔Web Persona/Resource sync, while each later App epic must deliver the same user outcome on Web and installed clients. Full FR-39 closes only when all applicable Core-App, media, security and recovery suites pass; it is not a late browser companion. The 2026-09-28 Tor decision adds FR-40: any Persona may explicitly toggle Tor routing, while strict mutual-Tor communication is a separate option; Epic 2 owns the generic installed-Device path and later Apps own their leak/availability evidence. The product owner approved rolling-wave story elaboration: detail Epics 1–2 first, retain Epics 3–10 as the approved Release-1 scope map, and elaborate later stories after evidence from the first slice. Only contracts needed by active stories are pinned before implementation; published wire/data contracts evolve with explicit versions and compatibility evidence. Stories 1.1–2.12 are approved; revised Tor Stories 2.13–2.15 are drafted for story-level review, and stories for Epics 3–10 remain pending. The strict gate before protected Notes is VIDA product policy, not a Delta Chat or OWASP requirement. Epic order follows Core API dependencies without turning technical layers into standalone epics. UX documents remain a concept handoff; UJ-2W in EXPERIENCE.md covers the first Android↔Web behavior.

**Implementation readiness:** Approval of Stories 1.1–2.12 is approval of their user outcomes, not proof that the cross-cutting contracts have passed. The [scoped readiness assessment](implementation-readiness.md) records the decision gates and fixture evidence needed before production implementation; bounded prototypes may proceed without marking stories complete.

**Story readiness for rolling-wave elaboration:** Before detailing a story, identify its user-visible action and outcome, applicable FR/NFR and dependency contracts, platform-specific UI states, and observable positive plus relevant failure/denial acceptance cases. Each story names the fixture or test that can prove its claim; a planned fixture is not a passed test. Use the first end-to-end simple Note action as an exemplar before expanding the rest of Epics 1–2. Maintain a compact representative condition/outcome matrix across Device, network, restart/quota, key loss, duplicate delivery, rights and schema versions; this is not a claim to test every Cartesian combination.

## Requirements Inventory

### Functional Requirements

- **FR-1** Autonomous Persona, configurable Profile and Personal Space without external registration; isolation between Personas.
- **FR-2** Authorize an equal Device and synchronize permitted Spaces without device/platform priority.
- **FR-3** Generate, confirm storage of and use recovery material for an autonomous Persona; organizational sign-in cannot recover it.
- **FR-4** Space-scoped Contact Cards with stable ID, field provenance/visibility and identity bindings; optional previewed system-contact import to Personal Space only through conformant OS/provider connectors; Web supports manual cards and authorized card sync, not browser system-book or `.vcf` import; no automatic deduplication; explicit copy, snapshot/live sharing and safe connector unlink.
- **FR-5** Create Shared Space with separate membership, keys, Apps and rights; inviter becomes Owner.
- **FR-6** Apply Member/Guest/ServicePrincipal membership, fixed role presets and scoped resource rights; only Owner changes Owner status.
- **FR-7** Revoke access and stop managed reads after effective sync; shared cached reads require rights recheck within seven days.
- **FR-8** Share a Note/Section by resource-scoped Guest grant without exposing the rest of Personal Space, unshared attachments or private backlinks.
- **FR-9** Direct/group Chat: text, voice messages, files, replies, reactions, edits, deletion/tombstones and pins, including durable offline sends and publication time.
- **FR-10** Forum Topics/replies coexist with Chat and link to Project, Note, Task or File.
- **FR-11** ACL-safe local search for Messages/Topics in the selected Space, with explicit offline coverage.
- **FR-12** E2EE 1:1 and group audio/video calls up to eight participants, with incoming/outgoing flow and network recovery or clear failure; no screen sharing or recording.
- **FR-13** Rich-text/structured Personal and Shared Notes, Sections, attachments, Relations and backlinks.
- **FR-14** Collaborative Note editing with transparent merge, conflicts, transient presence and live cursors/selections on all Release-1 platforms.
- **FR-15** ACL-safe Note revision history; restoration creates a new operation and preserves audit.
- **FR-16** Personal/shared Projects, Tasks and Subtasks with status, assignee, dates and links to other Resources.
- **FR-17** List and Board as views of the same Task; denied move leaves state unchanged and explains why.
- **FR-18** Configurable Task workflow/status transitions under ordinary rights and global Conflict handling.
- **FR-19** File as reusable Space Resource with stable ID, versions, availability, Relations, Files view, download threshold and recoverable failure/retry.
- **FR-20** Resolve a File conflict by selecting main variant, retaining another as revision or creating a separate copy with its own ID.
- **FR-21** ACL-safe local search across accessible/synchronized Resources of the selected Space; filtered by AppInstance, author and type, without mixed-Space global search.
- **FR-22** Claim local save only after durable write of operation/bytes; Chat, Note, Task, Relation and File intent survive restart; disk failure is explicit.
- **FR-23** Idempotent peer synchronization of equal Devices, truthful local/sync/mailbox/delivery/approval/conflict receipts and Persona-scoped online Device count.
- **FR-24** Deterministic merge of compatible concurrent changes and stable published Message order; wall-clock time alone is not causal proof.
- **FR-25** Global explicit Conflict lifecycle preserves variants, can reopen on a late branch, retains equivalent operation history and checks read rights before resolution.
- **FR-26** Irreversible external effects wait for process authority confirmation; stable logical IDs/idempotency and unknown-outcome handling prevent blind retry.
- **FR-27** Per-named-process approval configured by Admin from one approver, sequential stages or M-of-N; Core deduplicates votes and records outcome.
- **FR-28** AppInstance/Space-scoped automation on lifecycle events creates one logical derived Resource; remote apply never reruns a business command.
- **FR-29** Authorized AppPackage activation with versioned dependencies, scoped authorization and atomic migration/conformance preflight.
- **FR-30** AppPackage update modes compatible-auto, security-auto, manual and pinned; discovery/download do not grant access or activate; failure affects only its AppInstance.
- **FR-31** Versioned schema evolution with lazy/default fields, migration on write, required-field validation, no silent data loss and incompatible old AppInstance read-only.
- **FR-32** Signed bundled/external AppPackage contract, application/extension distinction, managed Marketplace and external repository discovery; iOS Release-1 declarative packages only.
- **FR-33** Onboarding for Persona/Profile, chosen Core Apps, contacts, recovery and optional per-Device Android high availability without blocking local use.
- **FR-34** Eighteen languages and twenty-one locale profiles with correct formatting and direction; Ukrainian base, no Russian Release-1 locale.
- **FR-35** Locally assembled, previewable, redactable diagnostic bundle sent only by explicit user action; no content/keys/cross-Persona IDs.
- **FR-36** Encrypted portable export of schemas, Resources, Relations, Files and recovery metadata under an open versioned specification, readable by an authorized independent client.
- **FR-37** Personal/Shared Calendar Events with time zones, one-off and simple recurrence, reminders and creation from Contact Card in current Space.
- **FR-38** Invite existing VIDA Personas to a specific Event without Space membership; scoped preview, RSVP, time proposal and renewed consent after time change.
- **FR-39** Full static Flutter Web/Rust-Wasm client in public Release 1: local browser storage, authorized equal-Device direct-first peer sync with VIDA-operated encrypted Iroh relay fallback, all applicable Core-App/call flows and separate browser security/recovery conformance without paid Hosted Space. Stock Iroh/Wasm does not itself prove browser direct P2P; browser-compatible direct transport is a prototype gate. Static origin/code delivery is a trust boundary; offline cold reopen requires cached shell/JS/Wasm; a closed/suspended tab has no guaranteed call, reminder or sync.
- **FR-40** Any Persona may explicitly toggle Tor routing on installed Android, iOS and Flutter Windows; the preference synchronizes across its authorized Devices. When enabled, discovery, authorized E2EE communication, Resource sync and receipts use verified Tor-compatible paths or remain pending; ordinary direct/relay fallback is forbidden until the owner explicitly turns Tor off. Separately, an individual chat may require both peers to use Tor, and a Space may require Tor for all its network actions; neither policy silently downgrades. Static Chromium Web remains a full ordinary client but does not perform Tor-required network actions in Release 1.

### NonFunctional Requirements

- **NFR-1 Security:** threat/abuse model and OWASP MASVS-aligned mobile plus ASVS-aligned Web/exposed API verification.
- **NFR-2 Cryptography:** established primitives; OS-protected native key storage and separately proven browser key custody; distinct E2EE media review.
- **NFR-3 Local durability:** confirmed local writes survive normal restart/process kill and Web reopen while the browser profile and cached app shell remain; browser data eviction is an explicit risk, not remote backup.
- **NFR-4 Convergence:** identical operation sets yield identical user-visible state on conformant Devices.
- **NFR-5 Offline:** synchronized authorized Resources remain available offline; outbound operations expose pending state.
- **NFR-6 Accessibility:** screen reader, keyboard and visible focus on Android, iOS, Windows and Web; Windows system menus/tray where applicable.
- **NFR-7 Privacy:** no default content telemetry or diagnostic upload and no implicit cross-Persona correlation.
- **NFR-8 Openness:** MIT Core, public versioned specifications/fixtures/conformance, portable export, license manifest and independent reader.
- **NFR-9 Localization:** automated formatting for all 21 locale profiles and Arabic RTL layout verification.
- **NFR-10 Performance:** native G0 method approved; Web G0 supported-browser matrix/runner remains open before Web implementation fan-out; G1 representative physical-device baselines, numeric G2 budgets before feature completion and G3 regression/store checks before release.
- **NFR-11 Store compliance:** Android/iOS package, background, privacy and dynamic-content policy checks before submission.
- **NFR-12 No silent loss:** operation/migration/conflict cleanup/file eviction preserve the last recoverable copy unless an authorized lifecycle rule permits removal.
- **NFR-13 Inactive Apps:** no handlers, background work, optional permission prompts or material overhead for inactive AppInstances.
- **NFR-14 Platform conformance:** evidence for every mandatory platform-NFR row separately on Android, iOS, Windows and Web, with browser storage/direct-relay/security fixtures.
- **NFR-15 Interoperability:** distributable independent reader plus independent client/node conformance proof; publication of a specification alone is insufficient.
- **NFR-16 Privacy/compliance:** REQ-PRIV-001–009 evidence and responsible controller/operator/store-account roles before production or public-store activation.
- **NFR-17 Tor-enabled egress:** On each installed Release-1 platform, network-capture and OWASP-aligned privacy/network tests prove Persona isolation, explicit route switching and fail-closed handling while Tor or mutual-Tor policy is active across discovery, data, receipts and ancillary traffic; an Android proof cannot substitute for iOS or Windows. Tor does not replace E2EE or justify an absolute-anonymity claim.

### Additional Requirements

- **AR-1 Architecture boundary:** headless Rust VIDA Core owns identity, authorization, signed operations, merge, conflict, resource graph, schema/runtime and receipts; installed Flutter Android/iOS/Windows and static Flutter Web shells own presentation and platform integration through native FFI or browser Rust/Wasm bindings.
- **AR-2 Transport:** Iroh core 1.2 is the selected baseline; Iroh does not define VIDA business operation, approval, authorization or merge semantics. Pin exact compatible build after proof; optional `iroh-*` 0.x modules remain candidates, not rejected by version number alone.
- **AR-3 Protocol:** VIDA-owned versioned ALPN/operation envelope/serialization and public positive, negative and byte-exact fixtures; compatible clients must not infer acceptance from transport receipt.
- **AR-4 Storage:** replaceable local storage, delivery and discovery mechanisms behind versioned contracts; verify crash consistency, recovery, replay and GC before broad use.
- **AR-5 Compatibility:** versioned Core/API, FFI, ALPN, schema, adapter and platform tuple governs activation and interoperation.
- **AR-6 Security:** signed operations, encrypted Space data, scoped keys, revocation and Core governance are mandatory; AppPackage code cannot bypass these checks.
- **AR-7 Editor selection gate (OQ-2):** benchmark Loro first, Automerge as mandatory control and Yrs as ecosystem control against the collaborative-document conformance specification and fixtures F01–F14; no engine approval solely from reputation or version.
- **AR-8 Calls selection gate (OQ-3):** compare direct 1:1, LiveKit measured baseline and iroh-live R&D control against E2EE call fixtures F01–F18, including Web↔native/Web↔Web, browser E2EE, permission and lifecycle evidence before adapter approval.
- **AR-9 Open implementation contracts:** close Rust–Flutter binding OQ-0035, storage OQ-0036, operation envelope OQ-0028, authority/SyncLog OQ-0033/34 and AppPackage trust/format OQ-0043 with versioned profiles and conformance evidence before their implementation fan-out.
- **AR-10 Library evidence:** first require candidates to pass the applicable mandatory contract/conformance fixtures; only then compare functional breadth, interoperability, security/maintenance, commit and release cadence, issue health, supported platforms, license, resource budgets and integration cost. Record the evidence and decision at the relevant library/adapter gate; no candidate is approved by this planning document. A pre-1.0 version is not automatic disqualification.
- **AR-11 Staging:** bounded technical prototypes may precede full contract closure; broad implementation of dependent stories waits for compatible versioned contracts, fixtures and approved ADR/stack decisions.
- **AR-12 Release proof:** automated vertical slice covers two Personas, three Devices, Personal/Shared Space, Chat/Note/Task/File, offline restart/reconnect, merge/Conflict, repeat sync and 1:1 E2EE call; every FR still needs its own feature proof.
- **AR-13 Scope exclusion:** hosted paid Space, native WinUI3, CRM, external calendar/provider sync, VCF/ICS interchange, screen sharing, recording, import of competitor histories and arbitrary iOS downloadable executable code are not Release-1 work. Static synchronized Web is included; server-rendered/paid hosted Web is not.
- **AR-14 Source reconciliation:** ADR-0021 supersedes the old ADR-0018 free-local-only/post-v1 browser scope and ADR-0020 browser-post-v1 consequence; before stories, reconcile stale UX, fixture and draft epic claims with current Web scope, recording the normative source for each conflict.
- **AR-15 Web proof:** ADR-0021 supersedes the old free-local-only/post-v1 browser boundary. Web↔Android direct-first sync requires a version-pinned WebRTC custom transport within Iroh/VidaNodeHost, proven on Iroh 1.2 (not inferred from older experimental bridges), and explicit VIDA-operated encrypted Iroh relay fallback. Lookup/signaling may use relay; application forwarding waits for a bounded failed direct attempt. Direct proof excludes TURN/relay pairs and requires path trace, receipt and no duplicate apply. Browser Rust/Wasm parity and independent key/storage/media/browser-security gates remain; relay is not a mailbox or business authority. A failed direct proof blocks the Web Release-1 gate until a new explicit architecture decision.
- **AR-16 Tor adapter gate:** Select and exact-pin a Rust Tor runtime and VIDA/Iroh adapter only after build, lifecycle, routing, isolation and leak-proof fixtures. Keep enabled Tor transport separate from the ordinary direct-first plus VIDA-relay profile; an outage never silently changes route, while an explicit owner switch is allowed. This is a Release-1 requirement for installed Android, iOS and Flutter Windows, not proof that a library or implementation is already approved.

### UX Design Requirements

- **UX-DR-1 Concept:** graphite/terracotta candidate A is the preferred visual direction, applied coherently across Apps; exact tokens and individual App screens are deferred to prototype/implementation validation.
- **UX-DR-2 Context:** active Persona, Space and truthful synchronization state remain visible; navigation, creation, search and notification scope follow the selected Space. Last accessible Space/screen restores; a revoked Space falls back to Personal Space without leaking its name.
- **UX-DR-3 Navigation:** active Chats, Knowledge, Projects and More are compact mobile destinations; Calendar, Files and Contacts remain reachable auxiliary surfaces. App entries appear only in the Space where activated and may be hidden from the menu without deactivation.
- **UX-DR-4 Layout:** one-pane compact, list/detail medium and optional three-pane wide profiles preserve selected Space/resource, draft, filter, scroll position and return path across resize.
- **UX-DR-5 Accessibility:** every state and action has text/accessibility semantics, visible focus, predictable focus return, dynamic-text support and reduced-motion treatment; compactness cannot reduce native touch targets.
- **UX-DR-6 First run/recovery:** support local Persona setup, selective bundled App activation, user-confirmed separate storage of recovery secret and encrypted bundle, and optional contact/Android high-availability permissions. Refusal of optional permissions cannot block Personal Space; unconfirmed recovery storage keeps protected Space work pending under the approved VIDA setup policy. Confirmation is not proof that either external copy exists or restores current data.
- **UX-DR-7 State vocabulary:** distinguish durable local save, pending, transfer, independent replica sync, mailbox storage, recipient delivery/read, approval/rejection, unknown outcome, conflict, file not downloaded and rights expiry by actual evidence.
- **UX-DR-8 Collaboration:** show transparent compatible text merge and recent edit indication; preserve variants in a keyboard-accessible conflict comparison, with explicit authorized choice and separate file revision/copy actions.
- **UX-DR-9 Sharing:** preview precise Note/Section, attachment and recipient scope before grant; link/navigation does not confer access and inaccessible backlinks/titles remain hidden.
- **UX-DR-10 Per-surface failure:** save, sync, denied access, search, package activation, contact import, calendar invitation and call failure each show the affected item and a recoverable next action without misleading success.
- **UX-DR-11 Prototype gate:** validate concept on small/large phone, landscape/tablet, resizable Windows window and supported Web breakpoints for safe areas, active destination, focus/scroll/back behavior, enlarged text, reduced motion and screen-reader order. UJ-2W records first-slice Web behavior; exact layout and conformance proof remain pending.
- **UX-DR-12 Tor state:** Any Persona with Tor enabled distinctly shows local save, pending Tor, syncing, application receipt and route failure without implying absolute anonymity; the off switch warns about possible linkability. A Device never presents its synced Tor preference as active on other offline Devices without their receipts. Static Chromium Web explains that Tor-required network actions are unavailable there and never presents an ordinary-route fallback as Tor-protected.

### FR Coverage Map

| FR | Primary epic | Outcome covered |
|---|---|---|
| FR-1 | 1 | Autonomous Persona and Personal Space |
| FR-2 | 2 | Equal authorized Devices |
| FR-3 | 1 | User-controlled recovery |
| FR-4 | 7 | Space-scoped Contact Cards and import |
| FR-5 | 3 | Shared Space creation |
| FR-6 | 3 | Roles and scoped rights |
| FR-7 | 3 | Revocation and rights recheck |
| FR-8 | 5 | Resource-scoped Note/Section sharing |
| FR-9 | 6 | Direct and group Chats |
| FR-10 | 6 | Forums and linked Topics |
| FR-11 | 6 | Messenger/Forum search |
| FR-12 | 8 | E2EE audio/video calls |
| FR-13 | 5 | Notes and Knowledge |
| FR-14 | 5 | Collaborative editing and presence |
| FR-15 | 5 | Note revisions |
| FR-16 | 6 | Projects, Tasks and Subtasks |
| FR-17 | 6 | List and Board views |
| FR-18 | 6 | Task workflow |
| FR-19 | 5 | Reusable Files |
| FR-20 | 5 | File conflict choices |
| FR-21 | 5 | Space-local Resource search |
| FR-22 | 2 | Durable local save |
| FR-23 | 2 | Equal-peer synchronization and receipts |
| FR-24 | 2 | Deterministic merge/order foundation |
| FR-25 | 2 | Global Conflict lifecycle |
| FR-26 | 9 | Confirmed irreversible effects |
| FR-27 | 9 | Per-process approvals |
| FR-28 | 9 | AppInstance/Space automation |
| FR-29 | 4 | AppPackage activation/dependencies |
| FR-30 | 4 | Safe AppPackage updates |
| FR-31 | 4 | Schema evolution |
| FR-32 | 4 | Signed packages, Marketplace and repositories |
| FR-33 | 10 | Complete first-run onboarding |
| FR-34 | 10 | Language and locale experience |
| FR-35 | 10 | Privacy-preserving diagnostics |
| FR-36 | 10 | Portable encrypted export |
| FR-37 | 7 | Personal/Shared Calendar Events |
| FR-38 | 7 | Scoped Event invitations and RSVP |
| FR-39 | 2 primary, 1 and 3–10 cross-cutting | Android↔Web equal-Device sync foundation in Epic 2; full static-Web Persona/Space/App/call/security/recovery parity completed across all affected epics before Release 1 |
| FR-40 | 2 foundation, 3 Space governance, 6 chat, later Resource/App epics cross-cutting | Epic 2 proves synchronized Tor preference for any Persona and generic installed-Device sync; Epic 3 owns strict Space policy, Epic 6 strict chat policy; affected Apps prove no-leak actions on Android, iOS and Windows before Release 1 |

### NFR Coverage and Evidence Plan

NFRs are not a separate late technical epic: each epic must meet the applicable quality contract in its own user flow. This map assigns proof ownership without treating a prototype, draft contract or one platform's pass as Release-1 evidence.

| NFR | Epic application | Evidence before the affected epic/release is accepted |
|---|---|---|
| NFR-1 Security | 1–10 | Threat/abuse cases and applicable OWASP mobile/Web/API controls for each exposed flow. |
| NFR-2 Cryptography | 1–3, 5–9; 10 export | Key custody/rotation and encrypted data fixtures; Epic 8 additionally proves Web/native E2EE media. |
| NFR-3 Local durability | 1–2, 4–7, 9–10 | Crash/reopen, failed save and recovery evidence; Web profile loss/eviction is reported, not disguised as backup. |
| NFR-4 Convergence | 2–9 | Deterministic repeat-sync, merge and Conflict fixtures on every modified resource family. |
| NFR-5 Offline | 1–2, 4–7, 9–10 | Local read/write/pending outcomes per App; Web cold reopen only after proven cached shell/JS/Wasm. |
| NFR-6 Accessibility | 1–10 | Keyboard, focus, semantics, screen reader and relevant mobile/Windows/Web platform cases. |
| NFR-7 Privacy | 1–10 | Persona isolation, least disclosure, no default content telemetry and consented diagnostics. |
| NFR-8 Openness | 2, 4, 10; protocol work in 3–9 | Public versioned contracts, vectors, license evidence, portable export and independent reader. |
| NFR-9 Localization | 1–10 | Locale-aware strings/data formatting in each shipped flow; Epic 10 closes all 21 profiles and Arabic RTL. |
| NFR-10 Performance | 2 baseline; 4–10 expansion | Native G0 fixed; Web G0 pending; G1 raw baselines, G2 budgets and G3 regression before release. |
| NFR-11 Store compliance | Android/iOS portions of 1–10 | Relevant dynamic-code, background, media and privacy checks before store submission; Epic 4/8 have focused profiles. |
| NFR-12 No silent loss | 2, 4–7, 9–10 | Last-copy, outbox, migration, file, conflict and browser export/recovery failure fixtures. |
| NFR-13 Inactive Apps | 4–10 | Disabled AppInstance has no handler/background activity or optional permission prompt. |
| NFR-14 Platform conformance | 1–10 | Separate Android/iOS/Windows/Web evidence for each applicable flow; no inherited native-to-Web pass. |
| NFR-15 Interoperability | 2, 4, 10; wire changes in 3–9 | Independent reader/client-node fixtures, not only first-party↔first-party tests. |
| NFR-16 Privacy/compliance | 1–10; public-release closure in 10 | Applicable REQ-PRIV evidence; controller/operator/store-account roles resolved before public activation. |
| NFR-17 Tor-enabled egress | 2 foundation; affected App epics; public-release closure | Separate Android, iOS and Flutter Windows packet-capture/leak matrices, Persona-isolation, route-switch and fail-closed fixtures; generic Note proof does not certify Chat, Files, calls or diagnostics. |

## Epic List

### Epic 1: Create and recover an autonomous private identity

The user can create a Persona and Personal Space without a server account, write and reopen a first simple Note under owner authorization and later recover that Persona with self-held recovery material. The Note is backed by the generic Core Resource model; the full Notes App belongs to Epic 5. No future networking or App is required for this outcome.

**FRs covered:** FR-1, FR-3.

**Implementation considerations:** Build identity/key/recovery and minimal local owner-authorization APIs first. A thin Rust–Flutter binding and a persistent plain-text Note backed by a generic Resource demonstrate the complete local path; this does not imply rich text, collaboration or the full Notes App yet. Give that Note a stable Resource ID and durable content that Epic 5 extends in place, without a demo-only model or manual import. This is not registration on a mandatory central service; a future federated account remains a separate identity context.

**Recovery acceptance path:** After loss of every prior Device, an independently stored secret and versioned encrypted recovery bundle must support restoring Persona authority on a new Device without a mandatory account service. Resource bytes can be restored only from an actually available authorized replica, encrypted export or backup; the kit alone is not a data backup. Show the user clearly when no recoverable data copy exists. This is an explicit FR-3 story proof, not a claim that the proof has already passed; current-controller, crypto, rotation and atomic persistence remain `OQ-0024` gates.

### Story 1.1: Create an autonomous Persona and Personal Space

As a new user,
I want to create a private Persona without server registration,
So that I can prepare a Space under my own control and complete its recovery setup before working in it.

**Acceptance Criteria:**

**Given** a fresh installation with no network connection
**When** I create an autonomous Persona
**Then** VIDA durably stages one Persona, its keys, a random owner-held recovery secret, a versioned encrypted recovery bundle, Personal Space and Owner authority under stable IDs, and shows that setup is incomplete
**And** no external account or server registration is required; protected Space work is not enabled before the recovery confirmation in Story 1.2.

**Given** the pending Persona and Personal Space were durably staged but recovery storage was not confirmed
**When** I restart the app
**Then** setup resumes for the same Persona and Space IDs without generating replacement keys, secret or bundle
**And** the UI does not report `PersonaCreated`, active setup or a completed backup.

**Given** local persistence fails during creation
**When** VIDA reports the result
**Then** it does not claim the Persona or Space was successfully staged
**And** it explains the failed save without presenting a partial Persona, Owner grant or recovery material as durable.

**Given** two separately staged or activated Personas
**When** I switch between them
**Then** each shows only its own authorized Spaces and profile data
**And** neither Persona's private data appears in the other's context; a local-create/restart/isolation fixture verifies these FR-1 outcomes, including a crash before and after the complete staging commit.

### Story 1.2: Save autonomous Persona recovery material

As an autonomous Persona owner,
I want to receive and separately store recovery material,
So that I can later prove control of my Persona after losing a Device.

**Acceptance Criteria:**

**Given** a durably staged autonomous Persona whose setup is incomplete
**When** I open its recovery step
**Then** VIDA presents the already generated secret privately, makes the encrypted bundle exportable, and asks me to store both separately from this Device and from each other
**And** the secret is not automatically sent to a federated node, embedded in the bundle or included in diagnostics.

**Given** I have not confirmed separate storage
**When** I leave setup or restart the app
**Then** setup remains incomplete and I can resume the recovery step
**And** VIDA does not imply that a safe copy was made.

**Given** I explicitly confirm that I stored the secret and encrypted bundle
**When** I finish setup
**Then** VIDA durably records my confirmation, finalizes `PersonaCreated` and activates the existing Persona, Owner authority and Personal Space without changing their IDs
**And** it explains that this confirmation does not verify either stored copy, and Note bytes still require an available authorized encrypted data copy; a two-part-material/confirmation/no-escrow fixture verifies these FR-3 outcomes.

### Story 1.3: Create and reopen the first simple Note

As a Personal Space owner,
I want to create a simple Note and reopen it after restarting VIDA,
So that my private work remains available without a network connection.

**Acceptance Criteria:**

**Given** my autonomous Persona setup is complete and its Personal Space is selected on Android without a network connection
**When** I create a plain-text Note in that Space
**Then** VIDA stores it as an owner-authorized generic Core Resource with a stable ID and displays it as a Note
**And** “Збережено локально” appears only after the content is durably written, not merely held in memory.

**Given** that Note was durably saved
**When** I restart VIDA while still offline
**Then** the same Resource ID and text reopen in the same Personal Space
**And** VIDA does not create a duplicate Note or claim independent-device synchronization.

**Given** local storage cannot complete the write
**When** I try to save the Note
**Then** VIDA reports the failure and does not label it saved
**And** it retains the draft where possible and offers a retry without silently discarding my text.

**Given** another Persona is active on the same Device
**When** it lists or tries to open Notes
**Then** the first Persona's Note is neither visible nor readable
**And** a simple-Note/offline-restart/failed-save/Persona-isolation fixture verifies these outcomes before the full Notes App is added in Epic 5.

### Story 1.4: Recover an autonomous Persona on a replacement Device

As an autonomous Persona owner who lost my previous Device,
I want to use my recovery material on a replacement Device,
So that I can regain control of the same Persona without a server account.

**Acceptance Criteria:**

**Given** the valid owner-held secret, current encrypted recovery bundle and a replacement Device
**When** I complete the recovery flow
**Then** VIDA restores authority over the same Persona identity with distinct newly generated keys and a new DeviceGrant for the replacement Device
**And** no mandatory federated or organizational account is required.

**Given** the secret or encrypted bundle is invalid or missing
**When** I attempt recovery
**Then** VIDA does not open the Persona or grant access to its Personal Space
**And** it does not reveal private Space names, Note content or other protected metadata.

**Given** my Persona authority and required data keys have been restored and an authorized encrypted resource copy is available
**When** VIDA retrieves or imports that copy
**Then** the original Note ID and content become available under the restored Persona
**And** the app distinguishes restored data from identity recovery alone.

**Given** my Persona authority has been restored but no authorized data copy exists
**When** VIDA reports the recovery outcome
**Then** it states that my identity was restored but my earlier Note bytes were not
**And** it does not invent, duplicate or claim to have recovered the missing Note.

**Given** I can sign in to a corporate account but lack the autonomous Persona's recovery material
**When** I attempt to recover that Persona
**Then** the corporate sign-in does not grant its authority
**And** a valid/invalid-material, available/missing-data-copy and account-isolation fixture verifies these FR-3 outcomes.

### Epic 2: Continue private work across equal devices

The user can authorize a second equal Device, durably save operations, synchronize Personal Space and see deterministic convergence or an explicit preserved Conflict. This adds peer continuity to the independently useful Personal Space from Epic 1. The first visible proof is Android plus a full-authority static Web Device: create a Persona on Android, authorize Web, edit the simple Note on both and observe converged state without paid Hosted Space. An optional Tor-routed path for any Persona lets installed Devices synchronize without ordinary direct/relay egress while enabled; it does not redefine that first Android↔Web proof. Web is already a real locally usable Device for the ordinary profile here, although later Core Apps and calls arrive in their own epics.

**FRs covered:** FR-2, FR-22, FR-23, FR-24, FR-25; FR-39 browser identity/storage/Rust-Wasm/direct-first-plus-relay-fallback foundation (full FR-39 parity is a Release-1 exit criterion across Epics 4–10); FR-40 generic optional Tor-routed Persona path on installed Devices (mutual-Tor communication and App-specific leak/availability fixtures remain with later epics). Epic 2 proves the generic durable-operation and Conflict paths on a simple Note. Chat/Task/Relation/File durability and conflict fixtures are exercised in their App epics; published Chat Message order is exercised with the Message UI in Epic 6. This keeps FR-22/24/25 cross-App completion open rather than calling the Note slice complete parity.

**Implementation considerations:** Establish the minimum versioned signed-operation envelope, local log/outbox/recovery, generic Resource schema with stable IDs and a relation-compatible extension point, personal authority/frontier and truthful receipts needed by this slice before Iroh transport sync; do not prematurely select unrelated later-App libraries. Full Relation semantics are specified before linked-resource stories need them, not as a gate for the first simple Resource sync. Validate Iroh, browser direct transport, browser storage/Rust-Wasm, cached shell/offline reopen, origin trust and native FFI on Android-plus-Web. First prove authorized Web Device direct Note sync without relay data forwarding; then prove encrypted relay fallback, path migration, repeat delivery and truthful both-path outage. Later stories within this epic complete equal-peer Conflict behavior; later Apps reuse the same transaction/merge path. Media proof belongs to Epic 8. No Device gets priority by platform.

**Epic-1/2 user-journey acceptance anchor (to be decomposed into stories):** A person creates a Persona on Android, saves one visible plain-text Note, securely authorizes Web as an equal Device, edits that Note on Web while offline, reopens the cached Web client, reconnects and sees the same converged result on both Devices. The UI calls the record a Note, while Core stores it as a generic Resource. At each step the UI truthfully distinguishes local save, synchronization in progress, application-level synchronized receipt and a blocked or unknown outcome; connectivity alone is not proof of synchronization. Link each story to the relevant journey step, Android/Web UX state and contract/conformance test. This anchor does not claim a complete Notes App or parity for later Apps and platforms.

**Assumption-audit evidence gates (pending, not claims of feasibility):**
- Run the same signed Core operation through native Android FFI and Web Rust/Wasm, then compare canonical bytes, authorization decision and projected state.
- Save a user-visible simple Personal Note locally in both clients; close/reopen Web with network unavailable after the complete app shell/JS/Wasm was cached, and verify data plus pending-outbox state. Browser quota/eviction or profile loss must show truthful failure/recovery limits.
- Enroll Web as an equal Device, then test browser-key loss, fresh re-enrollment and revocation of the old grant without silently transferring authority.
- Exercise direct Android↔Web Note sync with relay forwarding unavailable, then direct-path failure with VIDA-operated encrypted relay fallback, path migration, lookup/relay outage, reconnect, duplicate delivery and crash/replay; require convergence and application-level sync receipts, never a transport ACK presented as business acceptance.
- Exercise two authorized Devices making incompatible offline edits to the same Note fragment: a verifiable order of authority acceptance keeps the first accepted edit current and the later intent recoverable; incomparable authority acceptances preserve both variants as an explicit Conflict. An authorized person resolves the latter with a new operation, and repeat sync converges without a device-based winner or lost history.
- Carry the FR-39 Web trust and transport boundary into story acceptance: Web sync tries a proven direct browser-compatible path first, then a reachable VIDA-operated encrypted Iroh relay fallback, without paid Hosted Space; only loss of both paths leaves remote work pending. Model compromised static origin as possible browser-key/content exposure, not as something the compromised page can reliably self-detect. Test the response from a trusted Device: revoke the exposed Web grant, re-enroll at a trusted origin and show honest recovery limits.
- Treat creation and editing of the first Note as a visible user action backed by the generic Resource contract rather than hidden test JSON. A failed gate changes the dependent bridge/storage/relay design and stories; it does not silently remove Web from Release 1.

**Rolling-wave checkpoint after Epic 2:** An Android↔Web demonstration proves only the exercised slice, not all Release-1 Apps or platforms. Before elaborating Epics 3–10, record passed fixtures, reusable contract versions, unresolved decisions and their owners. Confirm that the slice tested Space/action authorization, schema-version compatibility and idempotent replay, not transport alone. Classify every consequential Epic-1/2 choice as a proven shared contract, a provisional implementation/adapter or an open App-specific decision. Later stories may depend on proven contracts; provisional choices are re-evaluated against new evidence, while App-specific decisions are resolved before implementing that App. Keep unproven capabilities explicitly open rather than inheriting Android↔Web results as platform or App parity.

**Evidence levels:** The Epic-1/2 Android↔Web proof above is an early acceptance milestone. The separate AR-12 Release-1 integrated vertical slice still requires two Personas, three Devices, Personal and Shared Spaces, Chat, Note, Task, File and a 1:1 E2EE call, followed by each applicable FR/platform suite. Passing the first milestone never marks AR-12 or Release 1 complete.

**Change routing during implementation:** A story-local behavior change updates its story and tests. A shared Core, wire or data-contract change updates the versioned specification/ADR, compatibility fixtures and affected stories before adoption. A product-scope or UX change requires an impact decision across PRD, UX, architecture and epics, then resumes at the earliest affected artifact. Neither published contracts nor evidence gates may be silently reclassified as complete.

### Story 2.1: Authorize Web as an equal second Device

As an autonomous Persona owner,
I want to approve a static Web client from my trusted Android Device,
So that I can use the same Persona on a second equal Device without a hosted account.

**Acceptance Criteria:**

**Given** my Persona is active on a trusted Android Device and I open the static Web client
**When** Web requests enrollment
**Then** VIDA shows the Web origin, requested Persona/scope and browser-profile storage risk before approval
**And** the request does not itself expose private Personal Space data.

**Given** I review that request on Android
**When** I explicitly approve it
**Then** Core issues a signed DeviceGrant for Web's distinct Device ID and keys within the approved Persona/scope
**And** Android and Web show the new Device without assigning priority to either platform.

**Given** Web has been authorized but has not applied the Note from an independent replica
**When** it shows its connection and content state
**Then** it identifies the Persona and shows synchronization as pending where applicable
**And** it does not label the Note “Синхронізовано” merely because enrollment or transport succeeded.

**Given** I cancel the request or its authorization is invalid or replayed
**When** Web attempts to enter the Personal Space
**Then** no valid DeviceGrant or private data access is conferred
**And** hidden Space names and Note content are not disclosed.

**Given** the authorized browser profile loses its keys
**When** I open a fresh browser profile
**Then** it requires new enrollment instead of inheriting the old DeviceGrant
**And** the old grant remains identifiable for revocation from a trusted Device; an enrollment/denial/key-loss fixture verifies these FR-2 and FR-39 outcomes.

### Story 2.2: Synchronize the first Note directly between Android and Web

As an authorized Persona owner,
I want the same simple Note to synchronize directly between my Android and Web Devices,
So that I can continue private work on either Device without routing its content through a relay when a direct path is available.

**Acceptance Criteria:**

**Given** I have the durable plain-text Note from Story 1.3 and an authorized equal Web Device from Story 2.1
**When** both Devices establish an authenticated direct Iroh path and Android synchronizes the Note
**Then** Web applies the same signed operation to a Note with the same Resource ID and content
**And** an application-level receipt, not connection establishment or a transport ACK, is required before either UI says “Синхронізовано”.

**Given** the direct path is available and lookup or WebRTC signaling may use relay infrastructure
**When** the Note operation is transferred Android↔Web
**Then** the encrypted application payload traverses the direct path, with no relay application forwarding or TURN/relay candidate pair presented as direct
**And** a route/byte-class fixture proves the selected path without logging Note plaintext.

**Given** I edit that Note on Web while disconnected, with no competing concurrent edit
**When** the local browser write succeeds, I close and reopen the fully cached Web client offline, and later reconnect directly to Android
**Then** Web retains the same Note ID, durable edit and pending outbox across reopen, and both Devices apply the edit once
**And** Web shows “Збережено локально” until an independent application receipt supports “Синхронізовано”; a quota or failed-write error never claims the edit was saved.

**Given** an invalid, revoked, wrong-Persona or replayed Web grant or operation
**When** direct synchronization is attempted
**Then** Core refuses private Note disclosure or domain apply and does not issue a successful synchronization receipt
**And** a negative authorization/replay fixture verifies the result on Android and Web.

**Given** the direct path cannot be established in the direct-only fixture
**When** the bounded direct attempt ends
**Then** the locally durable operation remains pending, without a false synchronized state or silent relay payload forwarding
**And** the implementation gate records exact-pinned Iroh 1.2/custom-transport build, Android↔Web direct transfer, native FFI↔Web Rust/Wasm canonical-envelope parity and application receipt; it does not infer success from older experimental bridges. Relay fallback and route migration are separate Story 2.3 outcomes.

### Story 2.3: Continue Note synchronization through relay fallback

As an authorized Persona owner,
I want my Android and Web Devices to synchronize the same Note when a direct connection is unavailable,
So that a restrictive network does not silently strand my private work.

**Acceptance Criteria:**

**Given** the same authorized Android and Web Devices and a locally durable Note operation
**When** an authenticated direct attempt fails or exceeds its measured time limit and a configured VIDA relay is reachable
**Then** VIDA forwards the same signed E2EE operation envelope over the Iroh-compatible relay path and the peer applies it once to the same Resource ID
**And** lookup or SDP/ICE signaling may use relay earlier, but application payload is not forwarded through relay before the direct attempt ends.

**Given** relay fallback is carrying the Note operation
**When** the receiving Device applies it and returns an application-level receipt
**Then** both Devices may show “Синхронізовано” for the declared replication scope
**And** a relay connection, transport ACK or relay forwarding alone never counts as that receipt or as business authority acceptance.

**Given** a previously relayed operation is retried, arrives through both paths, or the direct path later recovers
**When** VIDA migrates or repeats delivery
**Then** the stable operation ID is deduplicated and the Note is applied once without changing its Resource ID or losing its local edit
**And** path and receipt fixtures show direct→relay and relay→direct migration without a duplicate domain effect or false success state.

**Given** neither a direct path nor the configured VIDA relay is reachable
**When** I save or reopen the Note on either Device
**Then** the locally durable operation remains visible and pending across a normal restart or cached Web reopen
**And** VIDA does not claim remote synchronization, delivery or backup until a valid peer application receipt arrives.

**Given** a relay path is reachable but the receiving DeviceGrant or operation is invalid, revoked, replayed or bound to another Persona
**When** the encrypted envelope reaches the peer
**Then** Core rejects unauthorized disclosure or domain apply without treating relay delivery as authorization
**And** a restricted-network/relay-egress test verifies that only explicitly configured VIDA relay endpoints carry ciphertext, with no unintended public/N0 default and no plaintext in relay diagnostics. These are FR-23 and FR-39 fallback proofs, not a durable-mailbox or paid-Hosted-Space implementation.

### Story 2.4: Preserve incompatible offline Note edits as an explicit Conflict

As an authorized Persona owner,
I want to see both versions when my Devices make incompatible offline edits to the same Note,
So that synchronization never silently discards my writing or invents a winner.

**Acceptance Criteria:**

**Given** Android and Web start from the same Note frontier and each durably replaces the same sentence with different text while offline
**When** their valid signed operations reconcile and there is neither a verifiable order between their authority acceptances nor a safe text merge
**Then** Core preserves both alternatives as one explicit unresolved Conflict for the same Resource ID
**And** neither client clock, Device ID, packet arrival nor library tie-break selects a canonical visible winner.

**Given** both Devices have received the same validated operations and acceptance evidence
**When** they project the Note after direct or relay synchronization in either delivery order
**Then** Android and Web show the same Conflict and both permitted text alternatives
**And** the ordinary Note view states that a choice is needed instead of claiming a final synchronized content value.

**Given** that Conflict has been recorded durably
**When** either Device restarts or the same operation is delivered again
**Then** both alternatives and their operation history remain recoverable without duplicate visible variants
**And** a crash/replay/reordered-delivery fixture verifies convergence for this simple Note case.

**Given** a candidate edit has an invalid or revoked grant or the active Persona lacks read access to an alternative
**When** reconciliation or conflict display runs
**Then** the invalid candidate does not become an accepted shared variant and restricted text is not disclosed
**And** an invalid local candidate remains recoverable only under its applicable rights policy. This story exposes and preserves Conflict; choosing a resolution is a separate next story, while exact general operation-family proof remains gated by OQ-0033/OQ-0034.

### Story 2.5: Merge independent offline edits to one Note

As a Persona owner editing the same Note on equal Devices,
I want independent text changes to combine after reconnection,
So that work in different parts of the Note is not mistaken for a conflict or lost.

**Acceptance Criteria:**

**Given** Android and Web start from the same saved Note frontier and edit different paragraphs offline
**When** both authorized operations reconcile through direct or fallback transport
**Then** both paragraphs contain both intended edits on both Devices without a manual choice
**And** the Note keeps its Resource ID and signed edit history; the order in which operations arrive does not change the result.

**Given** the editor adapter cannot prove that two edits affect independent logical fragments
**When** those operations reconcile
**Then** it preserves both intents for the Story 2.4 Conflict path rather than silently selecting a library-materialized winner
**And** the operation-family fragment/merge profile and fixture are versioned before implementation under OQ-0034; this story does not preselect Loro, Automerge or Yrs.

**Given** the merged Note has been durably applied on each Device
**When** either Device restarts, receives either operation again or rebuilds its projection
**Then** the same combined text and both operations remain visible without duplicate edits
**And** independent-paragraph, reordered-delivery and crash/rebuild fixtures prove FR-24 and REQ-SYNC-006 for the simple Note; rich editing and live cursors remain Epic 5 work.

### Story 2.6: Keep the first provably accepted overlapping Note edit current

As a Persona owner returning with an offline edit,
I want to understand when another edit to the same sentence was accepted first,
So that I can accept the current text or intentionally apply mine as a new change.

**Acceptance Criteria:**

**Given** two authorized Devices edit the same sentence from the same Note frontier and their authority acceptances have a shared verifiable order
**When** they reconcile
**Then** the first accepted edit remains current on both Devices and the later incompatible intent is retained for its author
**And** neither Device clock, first packet, transport ACK nor first replication receipt is used as acceptance-order proof.

**Given** my later intent differs from the current sentence
**When** I inspect the comparison
**Then** I can keep the accepted text or submit my intended text as a new operation based on the current frontier
**And** the new operation rechecks current rights and preconditions; the stale operation never silently overwrites the accepted text.

**Given** the implementation cannot verify a common authority-acceptance order
**When** the branches reconcile
**Then** it does not invent a first winner: two accepted incomparable branches use Story 2.4, while an unaccepted candidate remains pending/recoverable
**And** acceptance-proof and UI fixtures cover all three cases under REQ-SYNC-006; OQ-0033/0034 must supply the versioned proof profile before this story is implemented.

### Story 2.7: Resolve an explicit Note Conflict without erasing its history

As a Persona owner with a Note Conflict,
I want to compare its permitted alternatives and choose or compose a new text,
So that both Devices can converge on an intentional result.

**Acceptance Criteria:**

**Given** the unresolved Conflict from Story 2.4 and valid rights to read both alternatives and edit the Note
**When** I choose one alternative or compose text using both
**Then** VIDA commits a new signed resolution operation that causally references every current Conflict head
**And** both alternatives, authorship and prior operations remain in recoverable history rather than being rewritten.

**Given** the resolution operation is durably saved on one Device
**When** Android and Web synchronize it in either delivery order and rebuild after restart
**Then** both project the same resolved Note text and no duplicate resolution
**And** they show local save and independent-replica synchronization only when their separate evidence exists; they do not promise finality against an unknown offline branch.

**Given** the actor lacks current edit rights or cannot read every required variant
**When** the actor tries to compare or resolve the Conflict
**Then** VIDA withholds inaccessible text and refuses an accepted resolution
**And** the rights check repeats at authority acceptance, not only when the comparison screen opens; choice, composition, replay and denial fixtures prove FR-25 and REQ-SYNC-012.

### Story 2.8: Reopen a resolved Conflict for a previously unknown branch

As a Persona owner whose Note was resolved while another Device was offline,
I want a newly discovered incompatible edit to remain visible,
So that an earlier choice cannot erase work it never considered.

**Acceptance Criteria:**

**Given** a Note Conflict was resolved using all then-known heads and the other already authorized Device holds another duly accepted incompatible branch outside that causal base
**When** that Device reconnects and its authority acceptance is incomparable with the resolution
**Then** VIDA opens a new explicit Conflict containing the resolution result and the late branch
**And** both payloads and their audit history remain recoverable under current rights without an automatic winner.

**Given** the late branch has a verifiable acceptance order relative to the resolution or fails current authorization
**When** it arrives
**Then** VIDA applies the first-acceptance/stale-intent rule or rejects the invalid candidate rather than opening a false accepted Conflict
**And** an old client timestamp or late packet alone does not decide which case applies.

**Given** peers receive the late operation in different orders or restart during reconciliation
**When** they have the same verified operation/control evidence
**Then** they converge on the same conflict state and preserved variants
**And** late-branch, revoked-branch, arrival-permutation and crash fixtures prove REQ-SYNC-011 without assuming a master Device.

### Story 2.9: Reconcile simultaneous decisions about one Conflict

As a Persona owner who may resolve the same Conflict on two offline Devices,
I want compatible decisions to appear once and incompatible decisions to stay reviewable,
So that neither independent decision is silently discarded.

**Acceptance Criteria:**

**Given** two authorized resolution operations causally cover the same Note Conflict and have incomparable authority acceptances
**When** their proven outcomes are incompatible
**Then** VIDA opens a new explicit Conflict between the two resolution heads without an arbitrary winner
**And** a later decision must causally include both current heads and recheck rights.

**Given** the two resolution operations have a provably equivalent canonical Note outcome and no different business consequence
**When** they reconcile
**Then** the UI shows one result while retaining both signed operations, authors and causal history
**And** a duplicate receive does not create a second visible result or rerun an originating command.

**Given** equivalence cannot be proven under the versioned Note policy or one operation is invalid/revoked
**When** reconciliation runs
**Then** VIDA neither silently coalesces unknown outcomes nor promotes an invalid candidate to an accepted variant
**And** incompatible/equivalent/unknown/invalid permutation fixtures extend the global REQ-SYNC-009/010 lifecycle to Note; this extension and its exact equivalence predicate require an OQ-0034 decision before implementation.

### Story 2.10: Repair and converge the same Note across three equal Devices

As a Persona owner using two Android Devices and Web,
I want a Device returning after a long disconnection to obtain missing authorized history,
So that my Note and Conflict state agree everywhere without appointing a master Device.

**Acceptance Criteria:**

**Given** the Persona has authorized Android phone, Android emulator and static Web as distinct equal Devices
**When** one Device misses Note operations while offline and later reconnects
**Then** it requests missing causal dependencies, validates and durably applies them once, and reaches the same Note/Conflict projection as peers with the same accepted evidence
**And** no platform, replica ID or connection order gains priority.

**Given** direct and relay paths duplicate, reorder or interrupt the same operations and application receipts
**When** synchronization resumes after a crash
**Then** the signed log/outbox recovers, replay remains idempotent and the projection rebuild agrees with the verified operation/control frontier
**And** an Iroh ACK alone never advances the application or authority frontier.

**Given** a Device lacks a causal dependency or sees an unknown mandatory schema/protocol feature
**When** it receives the operation
**Then** it keeps the operation pending for repair or fails that incompatible apply closed with an actionable affected-Resource state
**And** it does not project partial accepted text, erase unknown fields or stop unrelated Personal Space use; missing-dependency, mixed-version, three-peer permutation and restart fixtures prove FR-2/23/24.

### Story 2.11: Show truthful synchronization and Persona Device presence

As a Persona owner,
I want to see what was only saved, what reached another Device and which of my Devices are currently reachable,
So that I do not mistake a connection for a completed synchronization.

**Acceptance Criteria:**

**Given** an operation is durable on only one Device
**When** I inspect its state before any independent authorized durable replica applies it
**Then** VIDA shows “Збережено локально” or “Синхронізація” according to actual progress, not “Синхронізовано”
**And** a single-Device Persona remains locally saved even if its endpoint appears reachable.

**Given** an independent authorized replica has validated, durably applied the operation and issued a signed receipt for its exact frontier
**When** the originating Device verifies that receipt
**Then** it may show “Синхронізовано” for that declared replication scope
**And** mailbox storage, relay forwarding, QUIC ACK, domain approval and recipient Read retain distinct meanings.

**Given** a visible Persona has several user-controlled Devices and one loses its fresh application-level sync-capability proof
**When** the Persona profile count updates
**Then** only currently proven reachable Devices count as online; the owner may see that Device reconnecting, while other viewers do not see its internal reconnect attempt
**And** the count never aggregates another Persona, bot or ServicePrincipal or exposes raw Device IDs. Expiry timings follow a measured platform profile, not an invented constant; receipt/presence/privacy fixtures prove FR-23 and UX-DR-7.

### Story 2.12: Recover trust after a Web Device or static origin is compromised

As a Persona owner using the static Web client,
I want to cut off a suspected exposed Web Device and safely enroll a replacement,
So that its old grant cannot authorize future synchronization.

**Acceptance Criteria:**

**Given** I suspect the Web origin, browser profile or Device keys were exposed
**When** I use a trusted Android Device to revoke that Web DeviceGrant
**Then** the affected future-access epoch advances and new operations from the revoked grant fail authorization after the effective control frontier
**And** the Android UI identifies the affected Device without making a compromised Web page the trusted revocation channel.

**Given** I later open a verified trusted static origin in a new browser profile
**When** I repeat the explicit Story 2.1 enrollment
**Then** it receives a distinct Device ID and keys and can synchronize only under its new grant
**And** lost or evicted browser storage never silently inherits the old grant or claims recoverable Note bytes without an authorized copy.

**Given** the previous Web code or profile may have exposed plaintext before revocation
**When** VIDA reports the result
**Then** it explains that future managed access is blocked after effective sync but past screenshots, exports or copied plaintext cannot be erased
**And** compromised-origin, revoked-envelope, new-enrollment and browser-profile-loss fixtures prove the recovery path without claiming the page can detect its own compromise.

### Story 2.13: Toggle Tor routing for an Android Persona

As a Persona owner on Android,
I want to turn Tor routing on or off explicitly,
So that I can choose the network route while understanding its privacy limits.

**Acceptance Criteria:**

**Given** any Persona uses ordinary direct-first networking
**When** I enable Tor for that Persona
**Then** its pending and new network actions use only a verified Tor route or wait locally
**And** a signed preference change is synchronized to the other authorized Devices of this Persona; no other Persona changes route, no ordinary retry escapes, and active sessions are re-evaluated before reuse.

**Given** another authorized Device is offline when I enable Tor
**When** I view the setting before that Device returns an application receipt
**Then** VIDA shows that propagation is pending or unknown rather than claiming the other Device already uses Tor
**And** when it reconnects, the Device applies the current signed preference before presenting its own Tor-protected state.

**Given** Tor is enabled
**When** I explicitly turn it off after a clear warning that earlier exposure cannot be undone and ordinary routing may link sessions
**Then** ordinary networking may resume for that Persona under its normal policy
**And** a separately configured strict chat or strict Space operation remains pending rather than silently downgrading.

**Given** Tor is unavailable or Android restarts during a route change
**When** I save an operation or reopen VIDA
**Then** the operation remains durably local with an honest pending state until its current route policy can be met
**And** Android packet-capture/restart tests show no bypassing egress or stale-session reuse. This story does not claim full App or platform conformance.

### Story 2.14: Authorize another Android Device through a typed invitation

As a Persona owner,
I want to enroll my second Android Device using a scanned QR, an imported QR image/file or equivalent pasted text,
So that it can receive a distinct authorized DeviceGrant without confusing a contact address with account access.

**Acceptance Criteria:**

**Given** I open an enrollment invitation on the new Android Device through any supported representation
**When** the trusted existing Device verifies the new Device key, Persona, intent, freshness and my explicit confirmation
**Then** it issues one signed DeviceGrant for that Device and the new Device may join the Persona under equal-device rules
**And** a copied, expired, mismatched or replayed invitation cannot silently enroll another Device.

**Given** I instead scan/import/paste a contact locator
**When** VIDA parses its typed payload
**Then** it may prepare a contact or conversation invitation but does not issue a DeviceGrant or reveal a recovery secret.

**Given** the second Device is offline
**When** I prepare enrollment or a Note change
**Then** the relevant action remains locally pending until both Devices can complete authorization/sync; Epic 2 does not require a Tor mailbox.

### Story 2.15: Synchronize a simple Note over Tor between Android Devices

As an owner of a Persona with Tor routing enabled,
I want its authorized Android Devices to synchronize a simple Note through a verified Tor network path,
So that I can continue private work without exposing an ordinary direct or VIDA-relay route for that Persona.

**Acceptance Criteria:**

**Given** two Android Devices are authorized for the same Persona and Tor routing is enabled on the participating network contexts
**When** I save a simple Note on one Device and both become reachable through a verified Tor-compatible path
**Then** the other Device validates and durably applies the same signed operation and returns an application-level receipt
**And** packet capture and operation-replay fixtures show no ordinary peer-IP path, ordinary VIDA-relay fallback, duplicate apply or false “Синхронізовано” state.

**Given** Tor becomes unavailable before or during synchronization
**When** I edit the Note and later reopen the app or regain Tor connectivity
**Then** the edit remains durably saved and pending until Tor resumes, after which it applies once on the peer
**And** no direct, ordinary relay, DNS, WebRTC/ICE, HTTP or push egress for this Persona bypasses its enabled Tor policy during outage, retry or route migration.

**Given** another Persona on the same Android Device uses the ordinary direct-first profile
**When** both Personas have pending or active operations
**Then** the Tor-routed Persona does not reuse the other's endpoint identifiers, Tor stream context, discovery requests or network route
**And** the ordinary Persona retains its own approved behavior without becoming a fallback for the Tor-routed Persona.

**Given** I inspect the Note and network-profile state
**When** its operation is local, waiting for Tor, transferring, receipt-confirmed or blocked
**Then** the Android UI names the evidenced state and explains that Tor routing does not guarantee absolute anonymity
**And** an exact-pinned Rust runtime/VIDA-Iroh adapter, Android release-build, packet-capture leak matrix and applicable OWASP MASVS-NETWORK/PRIVACY checks prove this Android slice against FR-40, NFR-17, UX-DR-12 and REQ-TOR-001–010. This does not certify iOS, Windows, Web, calls or App-specific egress.

### Epic 3: Work together in a Space without exposing private data

The user can create a Shared Space, invite people under explicit rights and revoke access without exposing unrelated Personal Space data. It extends the operation/sync kernel from Epic 2 without needing a later App.

**FRs covered:** FR-5, FR-6, FR-7.

**Implementation considerations:** Full layered ACL, role presets, control-before-data sync, key epochs and seven-day recheck are Core governance APIs. Minimal owner checks exist from Epic 1; Shared-Space grants/revocations require the signed durable operation path from Epic 2. A Space may require Tor for all network actions; this policy overrides a Persona's ordinary-route preference and cannot be weakened by a chat. Whether an existing ordinary Space may be upgraded while Devices are offline remains an open security decision; do not claim immediate protection for stale Devices. Include a revocation-race acceptance case: an offline candidate authored under an apparently valid grant but presented after revocation is revalidated at its actual authority-acceptance point; an invalid candidate remains recoverable locally but is not projected as an accepted shared change. Delivery order alone neither grants nor revokes earlier valid acceptance. Epic 5 applies the same grants to Note/Section sharing.

### Epic 4: Add and update Space applications safely

The user can activate a bundled or external declarative App in a Space, use its resources, inspect its dependencies and update it without implicit grants or disruption of other Apps. A conformant sample package makes this independently usable before full Core Apps exist.

**FRs covered:** FR-29, FR-30, FR-31, FR-32.

**Implementation considerations:** Decide signed manifest/trust, capability, dependency, schema-migration and compatibility contracts before package fan-out. Package logic always calls Core authorization and operation APIs; publication, discovery, download and activation are distinct. The conformant sample package must reuse the Epic-2 generic Resource, signed-operation, authorization and receipt path, not a demo-only data model. It must render and operate on Android, iOS, Windows and Web under each platform's approved profile; iOS Release 1 executes declarative capabilities only, and the browser execution profile remains a proof gate.

### Epic 5: Capture, collaborate on and find knowledge and files

The user can keep rich Notes and reusable Files in a Space, share a precise Note or Section, collaborate on text, inspect revisions/conflicts and find only accessible local Resources. It needs no later Messenger or Project.

**FRs covered:** FR-8, FR-13, FR-14, FR-15, FR-19, FR-20, FR-21.

**Implementation considerations:** File metadata/bytes, Relations, local search and sharing build on generic Resources/ACL. The simple Notes created in Epics 1–2 remain the same Resources with the same IDs and content as Epic 5 adds richer capabilities through versioned schema evolution; there is no user-driven import or duplicate Note. Loro/Automerge/Yrs editor comparison must pass F01–F14 including Android/iOS/Windows/Web live cursors/selections before editor selection; File storage must pass availability, durability and safe-GC proofs. Browser offline reopen, quota/eviction and file-download constraints are explicit Web cases, not inherited from native.

### Epic 6: Communicate and run projects in one connected context

The user can converse privately or in groups, keep structured Forum Topics, manage Tasks in List/Board and link discussions, knowledge and Files without duplication. Chat/Forum and Project are grouped because each uses the other's Resource/Relation context; this avoids two incomplete dependent epics.

**FRs covered:** FR-9, FR-10, FR-11, FR-16, FR-17, FR-18.

**Implementation considerations:** Within this epic, expand APIs from Message/Chat to Forum/Topic to Project/Task/workflow, using one operation and ACL path. Deliver Messenger/Forum before Project internally, but close the epic only when the linked experience works end-to-end on Android, iOS, Windows and Web. Publication/delivery receipts, tombstones and denied moves remain truthful; a closed Web tab makes no background-delivery promise.

**Internal acceptance milestones (not separate epics or scope cuts):** (1) Messenger supports a usable direct/group conversation; (2) Forum provides persistent topics and replies linked to conversation context; (3) Project provides tasks/views linked to the same Chats, Topics, Notes and Files without duplication. Each milestone has its own user-visible acceptance cases, while Epic 6 closes only after the combined flow and platform conformance pass.

### Epic 7: Keep contacts and arrange meetings

The user can maintain independent Contact Cards in Personal/Shared Spaces, optionally import system contacts to Personal Space, create Calendar Events and invite an existing VIDA Persona without granting Space membership. This can proceed after Epic 3/4 in parallel with Epic 5/6, using the same Core API.

**FRs covered:** FR-4, FR-37, FR-38.

**Implementation considerations:** Contact identity is not membership. Invitation scope, time zones, recurrence and re-RSVP use Core authorization/operation semantics. Web supports manual Contact Cards and synced cards, not system-address-book or `.vcf` import; closed-tab reminders are not guaranteed. Provider export or two-way external calendar sync remains outside Release 1.

### Epic 8: Talk securely by audio and video

The user can start and receive 1:1 and small-team E2EE audio/video calls from an authorized conversation, see participants and recover from a network change or receive a clear failure. This depends on Epic 6 conversation identity, not on later automation.

**FRs covered:** FR-12.

**Implementation considerations:** Select media/signaling/keying adapter only after direct-1:1/LiveKit comparison and F01–F18 proof across physical Android/iOS/Windows and supported-browser Web, including Web↔native/Web↔Web E2EE, permission, key and active/closed-tab lifecycle. Maximum eight participants, optimized for four to six. No screen sharing or recording in Release 1.

### Epic 9: Approve consequential actions and automate routine work

The user can configure named approval processes, let an App create one deterministic derived Resource and perform irreversible effects only after the required outcome is confirmed. This builds on existing Space/App/Resource behavior and has no future epic dependency.

**FRs covered:** FR-26, FR-27, FR-28.

**Implementation considerations:** Core owns approval counting, vote deduplication, outbox/idempotency and unknown-outcome state. An App may extend its own lifecycle hooks but may not override governance or rerun a command on remote sync.apply.

### Epic 10: Start confidently, use VIDA in your language and keep control of data

The user can complete first-run choices for the fully available Core Apps, use the product in the agreed locale, assemble a privacy-preserving support bundle and take an encrypted portable copy to an independent reader. Epic 1 already offers basic Persona setup; this epic completes the Release-1 adoption and exit journey without a future dependency.

**FRs covered:** FR-33, FR-34, FR-35, FR-36.

**Implementation considerations:** Internationalization, accessible states and diagnostic redaction are cross-cutting requirements of every earlier epic, not deferred technical work. This epic closes their full-surface UX and conformance proofs, including static Web origin/code-delivery trust, supported-browser matrix, direct/relay-path outage messaging, encrypted export/restore and user-visible browser storage limits. Export and independent reader cover all shipped Resource types; store/privacy/interoperability gates remain mandatory for public release.

**Cross-cutting exit criteria for every epic:** applicable NFR-1–NFR-16, UX-DR-1–UX-DR-11 and FR-39 Web parity are tested in that epic's touched flows; this is not permission to defer security, accessibility, localization, offline truth or platform conformance to Epic 10. UJ-2W supplies the first-slice Web behavioral addendum; detailed Web layouts and conformance evidence remain open.
