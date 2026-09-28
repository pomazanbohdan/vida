---
title: "PRFAQ Distillate: Vida"
type: llm-distillate
source: "prfaq-vida.md"
created: "2026-09-22"
purpose: "Token-efficient context for downstream PRD creation"
---

# Product thesis

- VIDA is an open-core local-first super-app for a person who manages personal affairs and participates in team projects.
- Core value: messages, forum topics, notes/documents, tasks and files are related resources in one Space instead of fragments across unrelated apps.
- Primary differentiators: connected context, offline usefulness, multi-device sync, autonomous Personas, decentralization, portable data and forkable open Core.
- VIDA is customer-facing as a super-app; platform/runtime is an architecture description, not the main marketing proposition.
- City Portal, local-business catalog and booking are future Apps/integrations, not Release-1 scope.

# Release 1 scope

- Installed Flutter clients for Android, iOS and Windows share Rust Core but have separate platform profiles, lifecycle/permission adapters, packaging/signing and conformance suites.
- Core services: Persona/Profile, Personal Space, Shared Space, Contacts, Files, search, identity, rights, sync, recovery and AppPackage runtime.
- Bundled Apps: Messenger, Notes/Knowledge and Project App; user activates only needed Apps.
- Messenger: direct/group chats, forums, files, replies, reactions, edit/delete, search, pins, voice messages, E2EE 1:1/group audio-video calls; no screen sharing or recording.
- Notes/Knowledge: simple and structured notes, rich text, attachments, relations/backlinks, history, offline use and shared editing; full Notion database/formula parity is out.
- Project App: tasks, subtasks, statuses, assignees, dates, list/board views and links to notes/chats/forums/files; Gantt, time tracking, budgeting and advanced analytics are out.
- Each Space has Files view; one File resource can be linked from multiple resources without duplicate identity.
- First vertical slice: two users, three devices, Personal/Shared Space, chat, note, task, file, offline writes, restart, reconnect, merge/conflict and repeat sync.
- Public release is one complete Release 1; internal prototypes, evidence builds and store pre-release channels remain mandatory engineering stages.

# Product and UX decisions

- Onboarding: create Persona and profile/personal page, create Personal Space, explain/activate Core Apps, then offer second-device link, contact or group.
- Personal project stays in Personal Space; inviting people to a team project creates or uses explicit Shared Space.
- A specific note or note section can be shared without exposing the rest of Personal Space.
- Group/Project supports both fast chat stream and durable forum topics.
- Search is local across authorized synchronized Spaces; private content is not sent to an external search service by default.
- Message edit shows edited marker; delete-for-all creates synchronized tombstone but cannot revoke screenshots/exports/copied text.
- Shared rich text should support remote cursors/selections where chosen references allow it, plus fading changed-range highlights.
- UI distinguishes local durable save, sync/publication/acceptance, delivery and conflict; it does not claim global finality prematurely.
- Contacts are canonical VIDA ContactCards with optional consented device/Google connectors; Core does not deduplicate contacts.

# Identity, access and offline invariants

- Identity axes are separate: autonomous anonymous Persona, public Persona and federation-issued account; blocking one federation account does not destroy unrelated Personas/Spaces.
- Devices of one user are equal peers; no device wins because it is phone/desktop/owner device.
- Owner may remove another Owner; Admin cannot remove or appoint Owner and operates below Owner.
- Shared-Space cached reading locks after seven days without successful rights verification; Personal Space is not subject to that shared revocation window.
- Locally durable data may remain pending indefinitely; acceptance rechecks current rights and preconditions.
- Concurrent mergeable edits merge through Core libraries; incompatible concurrent states create explicit conflict rather than arbitrary device priority.
- Later unseen branches reopen a resolved conflict; equivalent outcomes preserve every operation in history.
- Conflict UI allows accept current, resubmit own change as a new authorized update and, where meaningful, save own variant as copy/revision.

# Sync, operations and automation

- Iroh is the primary transport foundation; transport delivery, durable storage, business acceptance and terminal outcome are separate concepts.
- Synced state application must not recursively emit a new business command merely because it arrived by sync.
- Same deterministic rule/event consequence uses stable logical IDs to converge to one derived resource.
- Irreversible external effects wait for sufficient accepted state; exactly-once cannot be promised without idempotency/status support.
- Apps use Core operation semantics; conflict resolution is system-wide behavior, not a special ACL permission.
- Approval profiles supported in v1: single approver, sequential stages and threshold M-of-N; Admin selects a profile per named process from Core-supported modes.
- Schema evolution is lazy/background; editing an old record writes current schema and missing required-without-default fields block save.

# AppPackage and openness

- An App may own a Space and depend on Messenger, Notes and Projects AppInstances; a smaller extension may attach to an existing Space.
- Published App controls its own Space data/logic but cannot override Core governance, membership, ownership, keys or revocation.
- App updates support compatible-auto, security-auto, manual and pinned policies; incompatible schema/client combinations affect only that AppInstance, not all VIDA.
- iOS Release 1 accepts declarative packages using built-in VIDA capabilities; arbitrary downloaded Rhai/Wasm/JavaScript execution is out pending separate Apple-policy design.
- External repositories and first-party marketplace are future distribution channels with signed manifests and update metadata.
- Open-source code license is MIT; normative specifications, docs, fixtures, trademarks, patents/IPR and contribution governance remain open.
- Portable encrypted export, public versioned schemas/specifications and an independent reader/fork are Release-1 continuity requirements.

# Platform, localization and diagnostics

- Flutter Windows is in Release 1; separate native WinUI 3/C# may follow after Release 1; Tauri is not the selected Release-1 stack.
- Browser modes are post-v1: free local-only browser storage and paid Hosted-Space sync.
- Release 1 localization is 18 languages/21 locales: uk, en, es-419, es-ES, pt-BR, pt-PT, hi-IN, id-ID, ar, de-DE, fr-FR, ja-JP, ko-KR, tr-TR, zh-Hans, zh-Hant, pl-PL, it-IT, ro-RO, cs-CZ, nl-NL; Russian is excluded.
- Chinese language support does not imply mainland-China distribution; country distribution/compliance is separate.
- Diagnostics are local-first, bounded and Persona-separated; user previews/redacts and explicitly sends an encrypted bundle to Support Contact; encrypted export is fallback.
- Automatic background diagnostics remain separate opt-in and disabled by default until governance/infrastructure are approved.

# Business model and post-v1

- Open Core works without subscription: local data, Core Apps and direct peer/device sync remain usable.
- Paid future service attaches a Space to an always-online node for relay/mailbox, managed replica/backup and blob storage.
- Hosted default is zero-knowledge; managed decrypting replica is explicit Owner opt-in per Space.
- Pricing, quotas, SLA, billing subject, retention/deletion and S3 ownership are post-v1 decisions.
- Initial distribution is through relevant forums, open-source and communities; specific channels/funnel/active-user definition are open.
- Centralized behavioral surveillance is rejected; store downloads alone do not prove product value.

# Technical constraints and ordered proofs

- Prototype order: Rust API/Flutter binding smoke test; durable local store/identity/operation envelope; Iroh two-peer sync; CRDT merge for notes/tasks/message log/file metadata; E2EE 1:1 call; group-call/keying; full vertical slice.
- Unresolved Core contracts: operation envelope/finality, CRDT/editor, storage/GC, Rust binding ABI/threading/cancellation, media/signaling/keying, AppPackage compatibility and open-spec governance.
- Numeric NFR budgets for size/startup/memory/battery/network/crash/ANR/storage/sync/calls/localization follow representative builds and become CI release gates.
- Android continuous sync uses allowed push/wake/reconnect/reconcile; optional per-device high-availability mode is explicit. iOS is online only with active controlled connection.
- All security cases must remain aligned to OWASP MASVS/ASVS and documented threat models.

# Rejected framings

- City resident booking local business is not the Release-1 hero or first journey.
- City Portal and booking are not Release-1 scope.
- VIDA is not three independent bundled apps; capabilities compose within Spaces and resources.
- CRM was corrected to personal/team project management.
- Absolute anonymity, guaranteed background online state and guaranteed remote deletion are rejected promises.
- General import of existing notes/chats/tasks/files is not promised in Release 1.
- Browser client is not a substitute for installed Android/iOS/Windows Release-1 clients.
- WinUI 3 is not Release-1 Windows implementation.

# Verdict and mandatory PRD actions

- Verdict: needs more heat, proceed to PRD; concept is shaped, implementation is not yet authorized.
- PRD must preserve approved scope and convert it to testable functional requirements, dependency-ordered journeys, release gates and traceability.
- PRD must distinguish user-visible completion from internal prototype/architecture/legal/security gates.
- PRD must not invent schedule or quantitative NFR values before prototype evidence.
- PRD must explicitly mark public-release blockers: legal controller/entity, store-account/signing owner, incident authority and support/disclosure channels.
- PRD must define privacy-preserving product-value evidence beyond raw store downloads.
- PRD must hand significant UI flows to a later UX artifact and cross-component technical choices to architecture.

# Open questions

- Exact media/signaling/keying stack and tested group-call limits.
- Canonical rich-text CRDT/editor and presence cursor protocol.
- Rust↔Flutter binding contract, threading, cancellation and compatibility window.
- Operation envelope, receipt/finality proof, storage/GC and file-retention contracts.
- Specification/doc/fixture license, trademark/IPR and governance.
- Translation source locale, workflow, reviewer ownership and release QA.
- Independent pentest/cryptography review as mandatory Release-1 gate and future vendor.
- Support Contact recipients, diagnostics retention, redaction schema and abuse controls.
- Legal entity/controller, store-account owner, signing custody, incident authority and launch jurisdictions.
- Acquisition channels, active-user definition and privacy-preserving success evidence.
- No founder-approved product kill criterion; failed prototypes change implementation/architecture rather than product objective.
