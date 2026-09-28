---
id: SPEC-vida-notes-knowledge
status: draft
companions:
  - notes-conformance-cases.md
  - ../../../docs/01-product/v1-replacement-bundle.md
  - ../../../docs/01-product/composable-workspace-model.md
  - ../../../docs/04-specifications/crdt-editor-conformance.md
  - ../../../docs/02-requirements/transport-sync-requirements.md
sources: []
---

> **Decision-gated contract.** Release-1 Notes behavior is approved; the CRDT/editor pair, section representation and cross-Space projection mechanics are not selected here.

# VIDA Notes and Knowledge

## Why

A person needs notes that remain useful alone and become part of a shared project's knowledge without changing products or losing offline work. A note may be discussed, related to a task and shared narrowly, while the rest of the Personal Space stays private.

## Capabilities

- **CAP-1**
  - **intent:** A user can create and organize personal or shared notes, structured documents and knowledge sections in an authorized Space.
  - **success:** Each item is addressable in its owning context, reopens after restart and is available to Project knowledge where authorized.
- **CAP-2**
  - **intent:** A user can write rich text and attach or relate files without making duplicate file resources.
  - **success:** Formatting, ordered content and authorized file references persist across devices; one File resource can be linked from multiple notes or tasks.
- **CAP-3**
  - **intent:** Authorized collaborators can edit the same note online or offline and understand one another's changes.
  - **success:** Independent edits converge without loss; incompatible overlapping edits retain visible variants; live cursors/selections and fading change highlights appear when relevant, but presence is not document history.
- **CAP-4**
  - **intent:** A Personal Space owner can share one note or section with another Persona without opening the rest of the Space.
  - **success:** The recipient sees only the granted content and separately authorized attachments; search, backlinks and notifications reveal nothing outside that grant.
- **CAP-5**
  - **intent:** A user can connect a note to a task, chat, forum topic, document or file and find it through knowledge search.
  - **success:** Explicit relations navigate only to readable targets; local search indexes available synchronized content without sending private content to an external search service by default.
- **CAP-6**
  - **intent:** A user can inspect a note's history and restore an earlier version without erasing later audit.
  - **success:** Restore creates a new durable change; earlier authorship and revisions remain inspectable under current access rights.

## Constraints

- Bundled Notes/Knowledge is a schema-driven `AppInstance` in a Space, composable with Messenger and Project. Its package logic cannot redefine Core authorization, keys or synchronization.
- Offline read and permitted edit are Release-1 requirements on Android, iOS and Windows. “Saved locally” requires durable commit; remote acceptance/delivery are distinct states, and committed edits survive restart.
- Note Resource, collaborative Document and Space identities remain bound. CRDT merge or a relation never grants access, decides task/file status or turns transport receipt into domain confirmation.
- Current ACL governs content, history, search, attachments, backlinks, presence and scoped sharing. Incompatible edits must not be hidden merely because an engine can render a deterministic value.
- Live cursors/selections and fading change highlights are Release-1 client requirements; presence is transient, not part of durable document history. Candidate CRDT/editor pairs must pass the existing cross-platform [conformance gate](../../../docs/04-specifications/crdt-editor-conformance.md).

## Non-goals

- Full Notion-style database views, formulas and complex tables as Release-1 gates.
- A second, isolated knowledge store inside Project; silent conversion of Personal Space into Shared Space.
- Selecting Loro, Automerge, Yrs or a Flutter editor without the published prototype evidence.

## Success signal

On Android, iOS and Windows, two authorized Personas edit a shared note, including an offline change followed by restart and reconciliation. Both can see one authorized result or an explicit user-relevant overlap, a readable history and linked Project context. A Guest invited to one personal note cannot infer another. The [conformance cases](notes-conformance-cases.md) and existing [editor fixtures](../../../docs/04-specifications/crdt-editor-conformance.md) define the observable evidence.

## Open Questions

- Which CRDT engine and Flutter editor adapter pass the same Android/iOS/Windows fixtures (`OQ-0073`)?
- How is a shareable section represented and authorized, and how are cross-Space links or projections handled without inheriting access (`OQ-0003`)?
- What exact mixed-version editing and schema migration contract applies when a Notes `AppInstance` is upgraded (`OQ-0040`)?
