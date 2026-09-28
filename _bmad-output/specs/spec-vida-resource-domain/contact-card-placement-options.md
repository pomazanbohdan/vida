# ContactCard placement options — OQ-0003

**Status: proposal, not an approved VIDA rule.** Approved requirements make Contacts a mandatory Core service with canonical VIDA IDs, separate connector metadata, no automatic deduplication, field-scoped sharing and no automatic correlation of Personas. The missing decision is where the canonical card belongs for ownership, keys, sync and ACL. See [Contacts requirements](../../../docs/02-requirements/contact-card-requirements.md) and [Identity contract](../../../docs/04-specifications/identity-domain-contract.md).

## Business example

Bohdan uses a private Persona and a separately registered company Persona on one phone. In his private book, “Olena” has a phone number and a locally known anonymous VIDA binding. The company book may contain the same person's work email. The device must not silently combine these cards, reveal the anonymous binding to the company, or erase one book when the other account is blocked.

| Option | Canonical home | Consequence | Assessment |
|---|---|---|---|
| **A — Core ContactCard in each Persona's Personal Space** | Each Persona owns its cards in its own Personal Space; other Spaces receive only explicitly shared fields or projections. | Reuses existing Space sync, keys and ACL while preserving account-context filtering. The same real person can have separate card IDs in two Personas; no automatic merge. | **Recommended for approval.** Smallest new domain boundary, consistent with current contracts. |
| **B — separate encrypted Persona Contacts vault** | Persona owns a dedicated contact store outside any Space. | Keeps contacts out of the visible Personal Space, but requires another sync, recovery, ACL and export boundary with equivalent guarantees. | Possible only if product needs distinct contacts policy beyond Space. |
| **C — one LocalVault-wide address book** | Cards are shared across every Persona stored on a device. | Easy local reuse but risks correlating private and company contexts, conflicts with account-filtered search/export and independent revocation. | Not compatible with the approved identity separation by default. |

Under A, Core ownership does **not** mean Contacts is an optional AppPackage or that a connected Google/Android/iOS address book becomes canonical. Connector IDs and tokens stay adapter metadata. An anonymous/private binding remains local-only under `REQ-CONTACT-010`; ordinary sharing still uses the approved snapshot default or an explicitly enabled live share with a field/binding preview. Exact cross-Space share transport and recipient projection ID remain open; this brief does not equate a Relation with a permission grant.

## Question for approval

For Bohdan's private and company accounts, should VIDA keep **separate canonical ContactCards in each Persona's Personal Space** (recommended), even when he manually enters the same phone number twice? A user may explicitly share selected fields, but opening the company account must never reveal the private card or anonymous binding automatically.
