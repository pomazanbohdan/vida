# ContactCard and connector contract

This companion details [CAP-1–CAP-6](SPEC.md) without selecting the canonical card's owning scope (`OQ-0003`), a vCard UID mapping or a provider-edit conflict winner. It derives product obligations from [REQ-CONTACT-001–015](../../../docs/02-requirements/contact-card-requirements.md); provider facts below are evidence for adapter prototypes, not proof that an adapter exists.

## Three separate records

| Record | Required boundary | Source |
|---|---|---|
| `ContactCard` | Stable VIDA ID; human fields, per-field provenance/visibility and typed `identity_bindings[]`. Its owner scope remains a parameter until `OQ-0003` is resolved. | REQ-CONTACT-001–003 |
| `ConnectorLink` | For one selected provider/account, retain external-record reference, revision/change token, sync cursor, last-sync evidence and field mapping separately from the card. Link removal changes neither card nor provider record. | REQ-CONTACT-005–006, 012 |
| Provider record | Exists under Android, Apple, Windows, Google or CardDAV rules; it is neither the canonical VIDA ID nor an identity proof for a Persona. Deleting it is an operation distinct from unlinking or deleting the card. | REQ-CONTACT-001, 004, 012 |

One `ContactCard` may have more than one identity binding. A matching phone, email, name, provider ID or vCard UID never creates a link or merge between cards or Personas automatically. The unresolved external-ID mapping must not be disguised as the canonical VIDA ID. [REQ-CONTACT-003–004, 010](../../../docs/02-requirements/contact-card-requirements.md).

## Authorization and projection gate

Apply the active account context, effective resource grant and field/binding visibility **before** producing any projection. This obligation is independent of whether the card eventually resides in a Personal Space or a Contacts vault. [SPEC-ID-009](../../../docs/04-specifications/identity-domain-contract.md), [REQ-ACL-020](../../../docs/02-requirements/access-control-requirements.md), REQ-CONTACT-002–003/011.

| Surface | Evidence to demonstrate |
|---|---|
| Card list, detail, recent and local search | No card or field from an inactive/ungranted account; no cross-Persona linkage inferred from co-location in one LocalVault. |
| Share preview, snapshot and live projection | Show only explicitly selected fields/bindings; a snapshot never follows later edits; revoked live access stops future updates. Recipient-only notes/tags never write back. |
| Notification preview, backlink and attachment | No name, identifier or private field leaks through a secondary surface outside the exact grant. |
| vCard/platform export | Project only compatible ordinary fields that the destination supports. Anonymous/private bindings never leave VIDA; no unsupported custom field is smuggled into a provider record. |

## Connector transition obligations

These are observable obligations, **not** a chosen persisted state enum or provider conflict algorithm.

| Event | Required result |
|---|---|
| First connect | Ask for the smallest applicable permission and begin in `read/import`. Local cards remain usable if permission is denied. |
| Choose `export` or `two-way` | Record the explicit selection; preview provider, target records and changed fields before the user approves an external write action or selected batch. One preview can cover its listed batch; it does not authorize later unrelated writes. Unsupported fields remain in VIDA, not silently truncated. |
| Permission narrows or is revoked | Stop operations outside the current grant. Do not infer that a no-longer-visible provider contact was deleted; keep the VIDA card and surface the connector's limited/blocked state. |
| Revision mismatch, invalid cursor or lost change history | Re-read/reconcile the provider's authorized scope; preserve the canonical card and local edits. Show unresolved divergence instead of a silent overwrite. The exact business resolution policy is open. |
| Partial external write or uncertain response | Keep per-record outcome evidence and do not report all records synchronized. Retry only the operations whose result can be established safely by that provider; otherwise present an unresolved result. |
| Unlink/delete | Distinguish `unlink ConnectorLink`, `delete ContactCard` and `delete provider record` by exact target and confirmation. |

## Provider evidence matrix

For every adapter and each enabled direction (`read/import`, `export`, `two-way`), the prototype must record supported fields, permission scope, external-write preview, provider record reference, revision check, incremental cursor, full-reconcile trigger, retry/partial-failure behavior and unsupported cases. The table identifies source-backed hazards; it does not approve a mapping or assert round-trip across providers.

| Provider | Source-backed hazard and prototype check |
|---|---|
| Android Contacts Provider | A visible `Contact` aggregates account-owned `RawContacts`; aggregation can change aggregate IDs. `SOURCE_ID` belongs to its source account and may be empty for a new raw contact; raw `VERSION` can support optimistic checks. Verify VIDA-owned raw writes stay in the VIDA account, foreign raw data is untouched, and reaggregation does not change the VIDA card ID. [Android provider](https://developer.android.com/identity/providers/contacts-provider), [RawContacts](https://developer.android.com/reference/android/provider/ContactsContract.RawContacts). |
| Google People API | `resourceName` can change after source linkage; `metadata.previousResourceNames` is conditional, and a CONTACT source has its own source ID/ETag. `connections.list` sync tokens expire after seven days; writes can lag incremental reads. Update masks replace or clear named fields rather than merge them. Verify remapping, ETag preconditions, expiry→full reconcile, masked-write safety and no immediate read-after-write assumption. [Person resource](https://developers.google.com/people/api/rest/v1/people), [connections.list](https://developers.google.com/people/api/rest/v1/people.connections/list), [updateContact](https://developers.google.com/people/api/rest/v1/people/updateContact). |
| Apple Contacts | Authorization may expose only a selected subset; fetched contacts may contain only requested keys. Change-history tokens are device-local and can become invalid/expired, requiring a rebuild. Access to the `note` field needs Apple's special entitlement and approval. Verify limited→changed selection, denial, requested-key handling, note entitlement and token reset without interpreting lost visibility as provider deletion. [Accessing the contact store](https://developer.apple.com/documentation/contacts/accessing-the-contact-store?changes=_11_2&language=objc), [TN3149](https://developer.apple.com/documentation/technotes/tn3149-fetching-change-history-events). |
| Windows ContactStore | `AppContactsReadWrite` covers app contacts; ordinary all-contacts access is read-only, while `AllContactsReadWrite` needs special provisioning. Change tracker can report `ChangeTrackingLost`. Verify declared capability/consent, app-owned write target, unsupported global write and full reconcile after tracking loss. [Access types](https://learn.microsoft.com/en-us/uwp/api/windows.applicationmodel.contacts.contactstoreaccesstype), [change tracking](https://learn.microsoft.com/en-us/uwp/api/windows.applicationmodel.contacts.contactchangetracker), [change types](https://learn.microsoft.com/en-us/uwp/api/windows.applicationmodel.contacts.contactchangetype). |
| CardDAV | Address-book object `UID`, resource URL and strong ETag have distinct roles; conditional `If-Match` protects writes. A sync token can be invalidated; a full collection pass then replaces incremental tracking. Verify stale ETag rejection, token fallback and no assumption of a server-side preview API. [RFC 6352](https://www.rfc-editor.org/rfc/rfc6352.html), [RFC 6578](https://www.rfc-editor.org/rfc/rfc6578.html). |

## Decision and proof boundary

- `OQ-0003`: card owning scope; all tests above run with that scope parameterized.
- `OQ-0074`: exact provider ID/field mapping, permission UX, concurrent-edit outcome and vCard UID policy remain open. This matrix identifies cases the choice must survive, not the choice itself.
- No adapter is accepted by documentation alone: run [CONTACT-F01–F23](contact-conformance-cases.md) on applicable platform/provider profiles and check [OWASP MASVS-PRIVACY-1](https://mas.owasp.org/MASVS/controls/MASVS-PRIVACY-1/) and [MASWE-0066](https://mas.owasp.org/MASWE/MASVS-PRIVACY/MASWE-0066/) for least privilege, informed permission and graceful denial.
