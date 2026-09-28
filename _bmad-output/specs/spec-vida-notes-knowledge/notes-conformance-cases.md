# Notes and Knowledge conformance cases

These scenarios trace the approved [Release-1 bundle](../../../docs/01-product/v1-replacement-bundle.md), [composition contract](../../../docs/01-product/composable-workspace-model.md) and `REQ-CLIENT-003/007/014`. They supplement, not replace, [collaborative document F01–F14](../../../docs/04-specifications/crdt-editor-conformance.md). No CRDT/editor result is claimed here.

| ID | Scenario and observable result | Source |
|---|---|---|
| NOTE-F01 | Create a personal note and structured document; restart without network. Both reopen with their committed content and stable resource identity. | REQ-CLIENT-003/007/014 |
| NOTE-F02 | Activate Notes in Personal Space later than Messenger. Existing Space and other AppInstances remain usable; Notes uses the same Space identity and rights. | PROD-V1-BUNDLE-001; PROD-COMPOSITION-001 |
| NOTE-F03 | Write rich text, reorder blocks, attach a File resource and link the same File from a task. The file retains one resource identity; no payload copy is required by the link. | REQ-CLIENT-014; PROD-V1-BUNDLE-001; REQ-COLLAB-002 |
| NOTE-F04 | Two authorized peers edit different ranges offline, restart and reconnect. Changes converge without loss and preserve a viewable revision history. | REQ-CLIENT-003; REQ-COLLAB-008/009 |
| NOTE-F05 | Two authorized peers make incompatible overlapping destructive edits. The UI exposes both user-relevant variants rather than silently choosing an engine-rendered winner. | REQ-COLLAB-008; REQ-SYNC-006 |
| NOTE-F06 | Connected peers edit together. Live cursors/selections appear; remote changes briefly highlight. Disconnect expires presence and never inserts cursor state into durable history. | PROD-V1-BUNDLE-001; REQ-COLLAB-004/012 |
| NOTE-F07 | Share only one personal note or section with a Guest. The Guest cannot read or infer unrelated Personal Space content through search, backlinks, attachments or notifications. | PROD-COMPOSITION-001; REQ-ACL-020 |
| NOTE-F08 | Link a note to task, chat, forum topic and file. Authorized targets open in context; a relation to an unreadable target does not grant access or reveal its private title. | PROD-COMPOSITION-001; REQ-COLLAB-005 |
| NOTE-F09 | Search local synchronized notes across accessible Spaces, filtering by Space/App/type. An inaccessible note never appears; private content is not sent to an external search provider by default. | PROD-V1-BUNDLE-001 |
| NOTE-F10 | Restore an older note revision. The restored content is a new durable operation; the intervening revision remains in authorized history. | REQ-COLLAB-009 |
| NOTE-F11 | Apply an update with wrong Space/Resource/Document binding, stale ACL epoch or forged actor. The update is rejected before visible content, search or history apply. | REQ-COLLAB-001/005; editor F14 |
| NOTE-F12 | A client unable to safely write a newer Notes schema is read-only for the affected AppInstance and prompts update; Messenger and other compatible Apps still work. | REQ-COLLAB-011; OQ-0040 |

## Evidence and preservation boundary

- Run the existing editor fixture matrix F01–F14, including IME/accessibility, kill/restart, unauthorized presence, compaction and mixed-version cases, before selecting the CRDT/editor pair.
- The approved product sources require composable Notes, scoped sharing and Release-1 collaboration. They do **not** choose a section storage type, cross-Space projection mechanism or candidate library. Those remain open in the kernel.
- Generic database views, formulas and complex tables are explicitly outside the Release-1 gate. Historical research examples are not silently promoted to requirements.
