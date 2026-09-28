---
id: SPEC-INDEX
status: review
last_updated: 2026-09-23
---

# Technical specifications

Специфікації створюються після відповідного decision gate. Вони описують реалізаційний контракт, але не замінюють rationale з ADR або product requirements.

| Specification | Scope | Status | Implementation |
|---|---|---|---|
| [SPEC-IDENTITY-001](identity-domain-contract.md) | Persona, bindings, public projection, devices і transitions | approved | unplanned |
| [SPEC-NODE-HOST-001](vida-node-host-contract.md) | Iroh endpoint/router і platform network lifecycle | approved | unplanned |
| [SPEC-DURABLE-DELIVERY-001](durable-delivery-contract.md) | Offline encrypted store-and-forward та ACK state | approved | unplanned |
| [SPEC-SPACE-MEMBERSHIP-001](space-membership-contract.md) | Space grants, revocation та key epochs | approved | unplanned |
| [SPEC-SYNC-LOG-001](sync-log-contract.md) | Signed durable operations, repair та projections | approved | unplanned |
| [SPEC-OPERATION-FINALITY-001](operation-finality-contract.md) | Evidence axes, Application Receipts, delivery, authority outcome і bounded finality | approved | unplanned |
| [SPEC-vida-operation-envelope](../../_bmad-output/specs/spec-vida-operation-envelope/SPEC.md) | BMad draft kernel OQ-0028: signed operation identity, semantic field dictionary for inner operation/receipts, wire/security decisions і conformance plan; codec/signature profile не обрано | draft | decision-gated |
| [SPEC-vida-platform-bindings](../../_bmad-output/specs/spec-vida-platform-bindings/SPEC.md) | BMad draft kernel OQ-0035: Flutter↔Rust semantic facade, lifecycle, ownership, cancellation, events, errors і three-platform fixtures; FFI bridge не обрано | draft | prototype-required |
| [SPEC-vida-windows-shell](../../_bmad-output/specs/spec-vida-windows-shell/SPEC.md) | BMad Release-1 Windows shell kernel: повні три Apps, keyboard-only, system menu/tray і screen-reader installed-build fixtures; WinUI не входить у v1 | draft | release-build-proof-required |
| [SPEC-vida-accessibility](../../_bmad-output/specs/spec-vida-accessibility/SPEC.md) | BMad Release-1 accessibility kernel: approved Windows keyboard/screen-reader gate + candidate Android/iOS floor, A11Y-F01–F12; formal level/AT matrix open | draft | decision-and-build-proof-required |
| [SPEC-vida-diagnostics](../../_bmad-output/specs/spec-vida-diagnostics/SPEC.md) | BMad decision-gated diagnostics kernel: локальний preview, явне надсилання, redaction і негативні privacy fixtures; channel/retention `OQ-0079` відкриті | draft | decision-gated |
| [SPEC-vida-local-search](../../_bmad-output/specs/spec-vida-local-search/SPEC.md) | BMad Release-1 local search kernel: cross-App authorized results, filters, offline coverage, revocation/rebuild і SEARCH-F01–F11; searchable File fields та provider open | draft | scope/prototype-required |
| [SPEC-vida-storage-provider](../../_bmad-output/specs/spec-vida-storage-provider/SPEC.md) | BMad draft kernel OQ-0036: atomic local commit, recovery, blob staging, provider seam і crash fixtures; SQLite/FTS лише кандидат | draft | prototype-required |
| [SPEC-vida-signed-log-engine](../../_bmad-output/specs/spec-vida-signed-log-engine/SPEC.md) | BMad draft kernel OQ-0033/0034: signed history, causal repair, receipt boundaries і однакове оцінювання Irokle/iroh-db/Knot/reference; provider не обрано | draft | prototype-required |
| [SPEC-vida-approval-process](../../_bmad-output/specs/spec-vida-approval-process/SPEC.md) | BMad Core approval kernel: per-process Admin policy, single/staged/M-of-N, revision-bound votes, separate outcome і APR-F01–F13; serverless authority proof open | draft | decision-gated |
| [SPEC-VIDA-POST-COMMIT-EFFECTS](../../_bmad-output/specs/spec-vida-post-commit-effects/SPEC.md) | BMad AppInstance reaction kernel: accepted fact → one derived resource, authority-gated irreversible effect, unknown result, correction та agent opt-in; EFF-F01–F17 planned | draft | executor-and-rule-binding-decision-gated |
| [SPEC-vida-messenger-forum](../../_bmad-output/specs/spec-vida-messenger-forum/SPEC.md) | BMad Release-1 kernel: direct/group chat, forum, publication/delivery states, content actions, relations, search і calls; direct-chat topology/moderation open | draft | design-gated |
| [SPEC-vida-resource-domain](../../_bmad-output/specs/spec-vida-resource-domain/SPEC.md) | BMad OQ-0003 kernel: Resource/Container/Relation glossary, identity, ACL, Files, schema evolution і conformance; structural choices open | draft | decision-gated |
| [SPEC-vida-contact-core](../../_bmad-output/specs/spec-vida-contact-core/SPEC.md) | BMad Contacts Core kernel: canonical ContactCard, identity bindings, opt-in connectors, vCard, sharing, lifecycle і conformance; owning scope open | draft | decision-gated |
| [SPEC-vida-notes-knowledge](../../_bmad-output/specs/spec-vida-notes-knowledge/SPEC.md) | BMad Release-1 Notes/Knowledge kernel: structured content, offline collaboration, scoped sharing, history, relations/search і conformance; CRDT/editor pair open | draft | prototype-required |
| [SPEC-vida-project-app](../../_bmad-output/specs/spec-vida-project-app/SPEC.md) | BMad Release-1 Project kernel: tasks/subtasks, list/board, dedicated team Space, composed discussions/knowledge/files, offline/conflicts і conformance; authority proof open | draft | decision-gated |
| [SPEC-vida-files](../../_bmad-output/specs/spec-vida-files/SPEC.md) | BMad Release-1 Files kernel: per-Space File resource, cross-App attachments, staged offline save, verified download, revisions/conflicts і conformance; crypto/retention open | draft | decision-gated |
| [SPEC-vida-portable-export](../../_bmad-output/specs/spec-vida-portable-export/SPEC.md) | BMad Release-1 FR-36 kernel: authorized encrypted portable data, public schemas/versioned format, independent reader і PORT-F01–F11; encoding/crypto/recovery details open | draft | decision-gated |
| [SPEC-vida-first-run](../../_bmad-output/specs/spec-vida-first-run/SPEC.md) | BMad Release-1 onboarding kernel: autonomous Persona, recovery warning, bundled App choice, optional Contacts/Android mode and ONB-F01–F13; UX/implementation proof open | draft | conformance-required |
| [SPEC-vida-call-control](../../_bmad-output/specs/spec-vida-call-control/SPEC.md) | BMad draft kernel OQ-0072: call context, multi-device answer, authority proof, token/key admission і crash fixtures; media profile не обрано | draft | prototype-required |
| [SPEC-COLLABORATIVE-DOCUMENT-CONFORMANCE-DRAFT](crdt-editor-conformance.md) | CRDT/editor boundary, live cursors, presence, hard gates, weighted rubric і cross-platform fixtures | draft | prototype-required |
| [SPEC-E2EE-CALLS-CONFORMANCE-DRAFT](e2ee-calls-conformance.md) | Core/media boundary, E2EE key epochs, OS lifecycle, hard gates, rubric і cross-platform call fixtures | draft | prototype-required |
| [SPEC-PLATFORM-RESOURCE-CONFORMANCE-001](platform-resource-conformance.md) | Android/iOS/Windows device/network profiles, workloads, measurement tools, evidence schema і staged performance gates | approved-method | baseline-required |
| [SPEC-BLOB-STORE-001](blob-store-contract.md) | Immutable encrypted blobs, manifests, transfer та GC | approved | unplanned |
| [SPEC-DEVICE-SYNC-SESSION-001](device-sync-session.md) | Пряма сесія пристроїв: контроль, causal repair, прийняття та ACK; serverless ordering відкритий | review | unplanned |
| [SPEC-SYNC-PRESENCE-STATUS-DRAFT](sync-presence-status-model.md) | Чернетка позначок зв'язку, sync і лічильника онлайн-пристроїв у профілі користувача | draft | unplanned |
| [SPEC-APP-PACKAGE-001](app-package-runtime-baseline.md) | Package/runtime baseline, керована логіка, репозиторії та AppInstance | review | unplanned |
| [SPEC-PRIVACY-RELEASE-EVIDENCE-DRAFT](privacy-release-evidence.md) | Матриця `REQ-PRIV-001–009` → докази → негативні launch checks; докази ще не зібрані | draft | evidence-not-produced |
| [SPEC-SCHEMA-EVOLUTION-DRAFT](schema-evolution-contract.md) | Current schema, versioned converters, migrations, mixed-version safety та rollback gates | draft | research-backed-proposal |
