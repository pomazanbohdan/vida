# Messenger and Forum behavior cases

These cases preserve the approved product behavior; they do not choose the unresolved direct-chat ownership, moderation or wire format.

| ID | Case | Required observation |
|---|---|---|
| MSG-01 | User writes to a group at 10:00 offline, local save succeeds, client restarts, message first publishes at 13:00 | Draft survives restart; main timeline shows publication at 13:00; written-at 10:00 is detail/audit; no premature Delivered. |
| MSG-02 | Two offline messages publish concurrently without a causal order | All clients eventually show the same deterministic operation-ID tie-break; individual recipient arrival and unsynchronized device clocks do not order them. |
| MSG-03 | Ciphertext reaches a mailbox, but no recipient device has applied it | Sender sees only “stored for delivery”; neither Synchronized nor Delivered is inferred. |
| MSG-04 | One of a recipient's two devices decrypts, validates and durably stores the message | Qualifying signed receipt makes that recipient Delivered; the second device is backfill, not a new recipient or approval vote. |
| MSG-05 | Group message has a fixed audience of three logical recipients; two have qualifying receipts | Show Delivered 2/3. Later device addition/removal does not rewrite the historical audience or reveal device counts. |
| MSG-06 | Recipient opens a message with sender-visible read receipts disabled | Local read may be recorded under applicable policy, but sender cannot infer “Read” from delivery or device ACK. |
| MSG-07 | Author edits and then deletes own message while another device is offline | Edit has visible marker; tombstone eventually removes managed content after sync; no claim that an exported or yet-unreached copy was erased. |
| MSG-08 | Project has chat plus forum; participant links a topic to a task, note and file | Topic retains stable ID/history; each target opens only if independently readable; inaccessible titles do not appear in preview/search. |
| MSG-09 | Access to a conversation or linked resource is revoked | Current read checks remove inaccessible content, snippets and metadata from local search, previews and notifications; offline shared-read lease follows Core policy. |
| MSG-10 | File/voice payload cannot be saved locally because storage is full | Explicit local failure or “not downloaded” state; no false saved/published claim and no silent deletion of a remote copy. |
| MSG-11 | Group call spans an authorized conversation | E2EE, capacity and lifecycle are proved by the adopted call-control/media conformance suite; no screen-share/recording UI is exposed. |
| MSG-12 | A transcript, local search index, preview or temporary attachment is present on a mobile device | Inspect local storage, OS backups and app-to-app exposure: protected content is not readable outside the authorized VIDA context; revoked content is absent from managed search/previews. |
| MSG-13 | Origin committed the message and Iroh/QUIC acknowledged transmission, but no independent authorized application replica has validated and durably applied it | Preserve “збережено локально” or transfer-in-progress status; do not show “синхронізовано”, “доставлено” or publication merely from transport evidence. |

## State boundaries

| User-facing state | Evidence boundary |
|---|---|
| Збережено локально | Atomic local operation/outbox durability only. |
| Синхронізація | Authorized transfer/reconciliation in progress. |
| Синхронізовано | First independent authorized durable application replica receipt; not a recipient read or business approval. |
| Збережено для доставки | Mailbox durably retained ciphertext; recipient has not necessarily applied it. |
| Доставлено | At least one recipient-controlled device durably decrypted/validated/persisted the exact operation. |
| Прочитано | Separate privacy-permitted Persona-level read fact after presentation. |

Publication proof is a distinct open OQ-0065 contract. Do not infer it from any row above merely because a network packet was accepted.

Security cross-check: [OWASP MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) covers sensitive local data, while the [OWASP Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) requires deny-by-default and permission checks on every request. These are verification references, not a choice of cipher, database or key-store design.
