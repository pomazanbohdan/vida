---
id: SPEC-vida-local-search
status: draft
companions:
  - local-search-conformance-cases.md
  - index-provider-evidence.md
  - ../../../docs/02-requirements/native-client-requirements.md
  - ../../../docs/04-specifications/identity-domain-contract.md
  - ../../../docs/02-requirements/access-control-requirements.md
  - ../../../docs/04-specifications/space-membership-contract.md
  - ../../../docs/02-requirements/localization-requirements.md
  - ../../planning-artifacts/ux-designs/ux-vida-2026-09-22/EXPERIENCE.md
sources: []
---

> **Decision-gated contract.** The Release-1 search behavior below is approved; searchable field depth and the local indexing provider remain open. Conformance cases are requirements, not executed evidence.

# VIDA Local Search

## Why

People need to find a message, note, task, forum topic, file or contact across their own linked work without losing the local-first and scoped-access guarantees of VIDA. Search must remain useful offline and must not reveal a different Persona or a resource after access is revoked.

## Capabilities

- **CAP-1**
  - **intent:** A user can search locally across synchronized Resources they may currently read.
  - **success:** Available Messenger, Notes, Project, Forum, Files and Contacts entries appear only within the active authorized context; unavailable or unsynchronized content is not represented as searched.
- **CAP-2**
  - **intent:** A user can narrow results by active Persona/actor, Space, AppInstance, author or Resource type.
  - **success:** Each filter changes the result set consistently without exposing inaccessible names, counts or snippets.
- **CAP-3**
  - **intent:** A user can understand the coverage of an offline search.
  - **success:** The UI identifies that results cover only locally available synchronized content and does not claim to include an unreachable replica or a file not downloaded on this device.
- **CAP-4**
  - **intent:** A user can open a result in its source context when access still exists.
  - **success:** Navigation checks current read access again; a revoked or expired grant yields a safe unavailable state without revealing a protected title or excerpt.
- **CAP-5**
  - **intent:** The system can keep search results consistent with authorized Resources after edits, deletion, revocation and account changes.
  - **success:** Reconciliation and index rebuild do not resurrect a removed Resource, a stale snippet or a result from another Persona.

## Constraints

- Search is a shared Core capability and a derived local projection, not an authority over canonical Resources. A link or index entry grants no read access.
- The active searchable index must exclude inaccessible Resources. Persisted index remnants are a separate at-rest risk and may never bypass the current read check.
- Effective revocation or expiry of the seven-day shared-Space rights reconciliation interval closes managed search for that Space, including cached results. Personal Space follows its own local identity policy.
- Active account context and explicit grants bound query, filters, result count, title, snippet, preview and open action. A resource-scoped Guest must not infer unrelated Space content through search or backlinks.
- Private search content is not sent to an external search service by default. Local index protection, revocation and crash/rebuild behavior require negative conformance proof before release.
- Arabic and CJK search input/tokenization must pass the approved localization conformance profile on Android, iOS and Windows.

## Non-goals

- Default cloud/remote full-text indexing, automatic cross-Persona aggregate search or using a result as an authorization proof.
- Selecting a search engine, promising OCR/full-file-content extraction or a numeric latency target before the corresponding scope and platform evidence gate.

## Success signal

While offline, a user finds a locally synchronized note, message and task, narrows by Space and opens a permitted result. After a second Persona is selected or shared access expires, the former results, snippets and counts cannot be used to recover that content, including after restart and index rebuild.

## Open Questions

- Which fields of Files, attachments and ContactCards are searchable in Release 1? Is OCR or full file-content indexing required, or only supported metadata and text Resources?
- Which local index/storage provider and encrypted-at-rest/rebuild approach passes cross-platform, revocation and localization fixtures?
