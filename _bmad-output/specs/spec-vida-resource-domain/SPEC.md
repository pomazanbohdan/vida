---
id: SPEC-vida-resource-domain
status: draft
companions:
  - domain-glossary.md
  - domain-cases.md
  - container-placement-options.md
  - contact-card-placement-options.md
  - relation-scope-options.md
  - resource-transfer-options.md
  - ../../planning-artifacts/prds/prd-vida-2026-09-22/prd.md
  - ../../../docs/01-product/composable-workspace-model.md
  - ../../../docs/03-architecture/decisions/ADR-0001-layered-access-control.md
  - ../../../docs/02-requirements/access-control-requirements.md
  - ../../../docs/02-requirements/contact-card-requirements.md
  - ../../../docs/04-specifications/app-package-runtime-baseline.md
  - ../../../docs/04-specifications/schema-evolution-contract.md
  - ../../../docs/04-specifications/blob-store-contract.md
sources: []
---

> **Decision-gated OQ-0003 kernel.** Approved Resource/Space/Relation behavior is assembled here. Container identity, ContactCard placement, cross-Space edges and copy/move inheritance remain open; this draft does not settle them by naming a type.

# VIDA Resource domain

## Why

Messenger, Notes, Projects, Files and external Apps must point to the same typed objects without copying content or creating incompatible permission models. A shared domain contract lets a task reference a note or file, supports offline continuity and prevents a link, search result or App dependency from leaking a private Resource.

## Capabilities

- **CAP-1**
  - **intent:** Every Resource can be addressed and revisited under a stable identity and declared schema.
  - **success:** A Message, Note, Task or File keeps its identity through offline replay, sync, projection rebuild and compatible schema evolution; repeat delivery creates no second logical Resource.
- **CAP-2**
  - **intent:** People can organize Resources in containers while configuring inherited access at useful scopes.
  - **success:** Effective permissions are explainable across Space, AppInstance, Container, ResourceType and Resource; a lower rule never overrides an upper maximum or hard deny.
- **CAP-3**
  - **intent:** Apps can relate Resources without copying their content or implicitly sharing the target.
  - **success:** A Task can link a Note, Forum Topic and File; each destination is checked independently on open, backlink, preview and search, with no hidden title/snippet disclosure.
- **CAP-4**
  - **intent:** The same File can serve multiple conversations, notes and tasks.
  - **success:** Several attachments reference one stable File identity and verified payload; versions and local availability remain visible without multiplying the File on each attach.
- **CAP-5**
  - **intent:** A Resource remains readable and editable across supported schema revisions without silent loss.
  - **success:** Historical payload retains its writer schema/version; reads use a supported converter, while the next edit writes current schema, materializes safe defaults or requires an absent mandatory value.
- **CAP-6**
  - **intent:** A person can find and carry authorized related data across Apps.
  - **success:** Local search and portable export include only accessible Resources, Relations and Files; revoked or unavailable targets yield no protected metadata.

## Constraints

- Space is the approved top-level boundary for ownership, membership, keys, policies and synchronization. Resource author and Space Owner are different concepts; AppPackage publisher identity gives no Resource access.
- App schemas declare Resource types and actions; Core applies the same permission engine to bundled and external Apps. A Relation, backlink, attachment, search hit or package dependency never grants target read/write by itself.
- Knowing or guessing a Resource ID never substitutes for object-level authorization: every read, write, backlink, preview, search and export path checks the exact actor, scope and action, denying by default.
- Access inheritance follows Platform → Space → AppInstance → Container → ResourceType → Resource → Field → Action, with upper maximum and hard-deny precedence. Reserved Owner-governance actions are outside this Resource ACL chain.
- Durable operations bind a schema identity/version. Historical operations are not rewritten by package upgrades; deprecated fields and unknown data cannot silently vanish from retained history or a mixed-version write.
- File metadata and Resource relations are distinct from immutable blob bytes; BlobStore integrity, local download failure and safe retention remain under its separate contract.

## Non-goals

- Choosing whether Container is itself a Resource, allowing multiple structural parents, or prescribing a tree/database schema without a decision.
- Defining cross-Space Relation replication/ownership, ContactCard vault placement, exact ID/serialization bytes or copy/revision ACL inheritance by implication.
- Giving linked content, external Apps or search providers additional access merely because a Resource is referenced.

## Success signal

On two devices, a user creates a Note, Task and File offline, links the Task to both, restarts and synchronizes. The same IDs and one File appear on both devices; a participant who can read the Task but not the Note sees no Note title or backlink. After a compatible schema update, the Note opens and its next valid edit preserves historical data.

## Open Questions

- Is Container a Resource with its own schema/history or a structural ACL scope? Can a Resource have multiple structural parents?
- Where does a canonical ContactCard live relative to Persona and Personal Space, especially when selected fields are shared?
- Who owns/authorizes a Relation, and how do cross-Space edges and backlinks synchronize without disclosing an inaccessible endpoint?
- How do move, copy and revision actions determine destination scope and metadata/permission inheritance without silent widening?
