# Digest: contact interoperability

- claim: Android Contacts Provider агрегує raw contacts з різних account types; VIDA може мати власний account/sync adapter і custom MIME data rows, але не повинна записувати власні поля у raw contacts інших провайдерів. source: https://developer.android.com/identity/providers/contacts-provider publisher: Android Developers pub_date: current accessed: 2026-09-22 confidence: high class: compatibility
- claim: Google People API підтримує read/manage/sync, ETag для concurrent update і incremental sync tokens; sync token спливає через 7 днів, а incremental sync не гарантує read-after-write. source: https://developers.google.com/people/api/rest/v1/people.connections/list publisher: Google for Developers pub_date: current accessed: 2026-09-22 confidence: high class: compatibility
- claim: Apple Contacts надає permission-gated fetch/save, limited contact access, change notification/history, але не загальну довільну схему розширень. source: https://developer.apple.com/documentation/contacts publisher: Apple Developer pub_date: current accessed: 2026-09-22 confidence: high class: compatibility
- claim: Windows ContactStore підтримує contact lists, change tracking, aggregate contacts і provider properties/annotations. source: https://learn.microsoft.com/en-us/uwp/api/windows.applicationmodel.contacts.contactstore publisher: Microsoft Learn pub_date: current accessed: 2026-09-22 confidence: high class: compatibility
- claim: vCard 4.0 є IETF interchange format; CardDAV є стандартним server protocol for address books, але жоден із них не замінює внутрішню VIDA provenance, identity binding та merge model. source: https://www.rfc-editor.org/info/rfc6350/ publisher: RFC Editor pub_date: 2011-08 accessed: 2026-09-22 confidence: high class: protocol

Leads: окремо специфікувати canonical ContactCard, identity bindings, field provenance, connector cursors, consent і conflict UI.

