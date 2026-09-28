---
id: SPEC-vida-contact-core
status: draft
companions:
  - contact-conformance-cases.md
  - contact-connector-contract.md
  - ../../../docs/02-requirements/contact-card-requirements.md
  - ../../../docs/04-specifications/identity-domain-contract.md
  - ../../../docs/02-requirements/access-control-requirements.md
sources: []
---

> **Decision-gated contract.** The approved ContactCard behavior is assembled here. Its canonical owning scope and exact connector conflict behavior remain open; this draft does not choose them implicitly.

# VIDA Contacts Core

## Why

People need one reliable contact card for communication across VIDA identity modes while retaining control over which fields leave the device or a Persona context. Contacts must work without a platform address-book account, and optional synchronization must not merge people, expose private identifiers or make an external provider the owner of VIDA data.

## Capabilities

- **CAP-1**
  - **intent:** A user can create and retain a canonical VIDA ContactCard independently of system and cloud address books.
  - **success:** The card keeps its VIDA ID and data after a connector is denied, disconnected or loses its provider token; an external record never replaces its identity.
- **CAP-2**
  - **intent:** A user can maintain contact details and several typed VIDA identity bindings with explicit provenance and visibility.
  - **success:** One card can carry name, organization, communication fields, photo, notes, tags, relations and anonymous/private/public/federated bindings; a view or export exposes only fields allowed in its current context.
- **CAP-3**
  - **intent:** A user can choose whether each address-book connector reads into VIDA, exports from VIDA or synchronizes both ways.
  - **success:** First connection defaults to read/import; export and two-way require an explicit choice and change preview, and refusal of platform permission leaves local Contacts usable.
- **CAP-4**
  - **intent:** A user can exchange compatible contact fields with supported platform and standards-based address books.
  - **success:** Standard fields round-trip through an applicable connector or vCard 4.0 profile without exporting anonymous/private VIDA bindings or writing unsupported fields into another provider's record.
- **CAP-5**
  - **intent:** A user can share selected card fields as a fixed snapshot or an explicitly enabled live share.
  - **success:** The recipient sees exactly the previewed fields and bindings; later owner edits never change a snapshot, live updates stop at revocation, and recipient notes/tags cannot alter the owner card.
- **CAP-6**
  - **intent:** A user can manage a connector link, a VIDA card and an external provider record as separate objects.
  - **success:** Each action names its target before confirmation; unlinking does not delete either card or provider record, and provider-token expiry reconciles without losing the VIDA card.

## Constraints

- Contacts is a mandatory Core service, not an optional AppPackage. A system, Google or other provider record is neither the canonical ContactCard nor its sole source of truth.
- Core never deduplicates, merges or suggests merging cards from a matching name, phone, email, profile URL or provider record; manual card editing is not an automatic matching subsystem.
- A vCard `UID` or OS aggregate contact ID may identify a source object but cannot by itself verify a VIDA Persona, merge distinct canonical cards or correlate otherwise unlinked Personas.
- Anonymous/private bindings stay VIDA-local and never enter a system/platform contact or vCard. Active account context and resource grants restrict contact reads, search, previews and export.
- Connectors need explicit, minimal, revocable permission; a preview precedes a user-approved external write action or selected batch. Denial, partial permission or revocation cannot disable local card management.
- Android, Apple, Windows and Google adapters use only fields and synchronization semantics their provider actually supports. CardDAV may be a connector, not VIDA's canonical sync protocol.
- Contact access and permission flows must satisfy the [OWASP MASVS-PRIVACY-1](https://mas.owasp.org/MASVS/controls/MASVS-PRIVACY-1/) and [MASWE-0066](https://mas.owasp.org/MASWE/MASVS-PRIVACY/MASWE-0066/) gate in the adopted requirements.
- The [connector contract](contact-connector-contract.md) separates canonical card, link metadata and provider record; it defines projection, reconcile and platform evidence gates without choosing the unresolved card scope or conflict winner.

## Non-goals

- Generic import of notes, tasks, chats and files; automatic contact matching or merging.
- Choosing the canonical ContactCard owning scope, cross-Space live-share projection identity or a provider-specific conflict algorithm without the pending decision and connector prototypes.
- Treating vCard/CardDAV, an OS address book or a cloud account as the VIDA synchronization or identity authority.

## Success signal

With two isolated Personas, a user creates a card carrying a local-only anonymous binding, optionally imports a system contact, and shares only selected ordinary fields. A second device receives the permitted snapshot or live update; the private binding never appears in system contacts, vCard, the other Persona or the recipient view. Disconnecting the provider leaves the canonical card intact. The [conformance cases](contact-conformance-cases.md) make each part observable.

## Open Questions

- Does a canonical card belong to each Persona's Personal Space or to a separate encrypted Persona Contacts vault (`OQ-0003`)? The two accounts must remain isolated either way.
- For each connector, which exact field mapping, partial-permission UX and concurrent provider-edit resolution passes platform conformance?
- For a live share across Spaces, what owns and authorizes the recipient projection without turning a link into an access grant?
- How should an imported vCard `UID` be retained, if at all, and which identifier should VIDA export as `UID` without correlating separate Personas or exposing its canonical card ID?
