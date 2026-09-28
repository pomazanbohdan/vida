---
id: SPEC-vida-messenger-forum
status: draft
companions:
  - messaging-cases.md
  - conversation-ownership-options.md
  - ../../planning-artifacts/prds/prd-vida-2026-09-22/prd.md
  - ../../planning-artifacts/ux-designs/ux-vida-2026-09-22/EXPERIENCE.md
  - ../../../docs/01-product/v1-replacement-bundle.md
  - ../../../docs/04-specifications/operation-finality-contract.md
  - ../../../docs/04-specifications/durable-delivery-contract.md
  - ../../../docs/04-specifications/sync-presence-status-model.md
  - ../spec-vida-call-control/SPEC.md
sources: []
---

> **Release-1 product kernel.** Approved Messenger/Forum scope is captured here without selecting an unapproved direct-chat ownership model, moderation policy or OQ-0065 publication wire profile.

# VIDA Messenger and Forum

## Why

People coordinating personal and team work need quick conversation and durable, structured discussion beside their notes, tasks and files. VIDA must provide both within its Space model, remain usable offline, and show honestly whether a message is saved, published, delivered or read.

## Capabilities

- **CAP-1**
  - **intent:** Authorized people can hold direct and group conversations across their VIDA devices.
  - **success:** Text, voice message and file composition survive a confirmed offline local save and restart, then synchronize without a duplicate logical message.
- **CAP-2**
  - **intent:** A sender can understand the actual publication and delivery state of each message.
  - **success:** UI uses the distinct evidence thresholds in `messaging-cases.md`: durable origin commit for local save, independent authorized application-replica receipt for “synchronized”, mailbox retention only for “stored for delivery”, qualifying recipient receipt for “delivered”, and a separate privacy-permitted Persona read fact; publication has its own proof. Written time remains available in details while the main time/order reflects publication.
- **CAP-3**
  - **intent:** Participants can reply, react, edit, delete and pin messages within their ordinary rights.
  - **success:** Edits to an author's message show “edited”; delete-for-all synchronizes a tombstone on managed replicas; reconnect/duplicate delivery converges without promising erasure of exports or unsynchronized copies.
- **CAP-4**
  - **intent:** Every active group and Project context offers quick Chat and lasting Forum Topics with replies.
  - **success:** A topic has stable identity and history; its conversation and Chat remain available in the same owning context and link to permitted Project, Note, Task and File resources.
- **CAP-5**
  - **intent:** Conversation content can reference shared Files and other VIDA resources without duplicating their payload or widening access.
  - **success:** A link opens its target only after current authorization; a participant lacking target read permission sees neither protected content nor private title/snippet.
- **CAP-6**
  - **intent:** A person can find accessible Messages and Topics locally.
  - **success:** Search filters by Space, author and type; offline coverage is explicit; revocation removes inaccessible content and metadata from results.
- **CAP-7**
  - **intent:** Conversation participants can initiate or join 1:1 and group E2EE audio/video calls.
  - **success:** Call controls obey the separate call-control/media conformance contract, support up to eight total participants and contain no screen-sharing or recording action in Release 1.

## Constraints

- Messenger and Forum are bundled App capabilities composed inside Spaces and Project; they reuse Core identity, membership, ACL, Files, Relations, search and sync rather than inventing a separate authority model.
- A locally durable message is not thereby published, delivered or read. Iroh/QUIC ACK and mailbox ciphertext storage cannot be promoted to recipient or domain evidence.
- The primary displayed message time is publication time after synchronization. Canonical order follows causal/topological publication, with a deterministic operation-ID tie-break for incomparable publications; local sender time and recipient arrival order do not select the order.
- Group delivery is `N/M` logical recipient Personas against the immutable audience revision, not a count of their devices. Read is a separate Persona-level synchronized fact subject to privacy policy.
- Search, link previews and notifications must enforce current read rights without leaking hidden resource titles, snippets or metadata. AppPackage handlers cannot bypass Core governance or revocation.
- Transcripts, local search indexes, previews and temporary attachment copies are sensitive local data: storage and platform integration must protect them at rest and avoid leaking them through backups or other apps. Authorization is deny-by-default and rechecked for each action; no particular encryption/storage product is selected here.

## Non-goals

- City/business booking conversations, full Telegram/Discord feature parity, screen sharing and call recording in Release 1.
- Choosing direct-chat Space/container topology, forum moderation or retention defaults without an explicit decision.
- Defining OQ-0065 receipt bytes, operation envelope serialization or the E2EE media implementation in this product spec.

## Success signal

On Android, iOS and Windows, two people can exchange a direct message and use a Shared Project's Chat and Forum; one drafts offline, restarts, reconnects, observes publication and recipient delivery accurately, links a permitted Task/File, finds the topic locally, and completes an E2EE call. Revoked content does not appear in search or previews, and no unsent message is shown as delivered.

## Open Questions

- Where does a new direct 1:1 conversation live, and how are its initial membership, encryption and Space rights created without exposing either person's Personal Space?
- Which forum taxonomy, moderation actions, pin/archive rules and retention defaults should Owner/Admin and ordinary participants have?
- Should a Project-linked 1:1 chat be a separate private conversation linked to the Project, or a resource governed inside the Project Space?
- Which exact OQ-0065 application receipt fields, epoch/frontier and unread-frontier rules implement the already approved publication ordering?
