# Files conformance cases

These scenarios trace the approved [Release-1 bundle](../../../docs/01-product/v1-replacement-bundle.md), [client requirements](../../../docs/02-requirements/native-client-requirements.md), [BlobStore contract](../../../docs/04-specifications/blob-store-contract.md) and [file-conflict ADR](../../../docs/03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md). They are acceptance targets, not an implemented transfer claim.

| ID | Scenario and required observation | Source |
|---|---|---|
| FILE-F01 | Open Files view in each authorized Space. A File has one stable ID, versions and permissions; metadata-only and local-payload-available states differ. | REQ-CLIENT-023; PROD-V1-BUNDLE-001 |
| FILE-F02 | Attach the same File to message, note and task. Each relation points to that File without duplicating payload; a reader lacking File access cannot fetch it through a link. | REQ-CLIENT-023; SPEC-BLOB-STORE-001 |
| FILE-F03 | Add a photo while offline; kill the process after confirmed local save. Recoverable staged payload, manifest reference and outbox intent reopen together. | REQ-CLIENT-007/009; SPEC-BLOB-STORE-001 |
| FILE-F04 | Fill disk before recoverable payload commit. The UI reports save failure, never “saved locally”, and no published manifest references missing local bytes. | REQ-CLIENT-009; SPEC-BLOB-STORE-001 |
| FILE-F05 | Receive metadata for a file above configured auto-download size. Payload does not fetch until explicit user action; confirmation still rechecks current authorization. | SPEC-BLOB-STORE-001 |
| FILE-F06 | Inbound payload write fails from quota, disk or I/O. Shared manifest and other replicas stay unchanged; local state is “not downloaded” with reason. | SPEC-BLOB-STORE-001 |
| FILE-F07 | Interrupt transfer after some chunks and reconnect. Resume uses only verified chunks; corrupt chunk or manifest is rejected. | REQ-SYNC-003; SPEC-BLOB-STORE-001 |
| FILE-F08 | Expire or leak a transfer ticket, or revoke File access before fetch. The stale capability cannot reveal the current payload merely because the caller has bytes or a ticket. | SPEC-BLOB-STORE-001; OWASP authorization gate |
| FILE-F09 | Two incompatible replacement versions have incomparable valid authority acceptances. All authorized peers show unresolved conflict with both retained payloads and no automatic winner. | REQ-SYNC-007; ADR-0017 |
| FILE-F10 | Resolve an alternative as revision or separate copy. Revision keeps original File ID; copy receives a new ID and independent current-rights check; neither silently replaces primary. | REQ-SYNC-013; ADR-0017 |
| FILE-F11 | Two offline incompatible resolutions later reconcile or a previously unknown accepted branch arrives. Required variants/pins survive and the conflict reopens where acceptance is incomparable. | REQ-SYNC-009/011; ADR-0017 |
| FILE-F12 | Remove one relation or resolve one local branch. GC does not collect a live or potentially needed variant merely because that peer removed a pin; storage pressure is visible without silent deletion. | REQ-BLOB-002/003; SPEC-BLOB-STORE-001 |
| FILE-F13 | Delete a File reference or revoke access. A formerly authorized actor cannot fetch after effective revocation; UI does not claim deletion of prior exports, offline replicas or backups. | SPEC-BLOB-STORE-001; REQ-MEM-003 |
| FILE-F14 | Inspect retained sensitive payload and local temporary chunks on Android, iOS and Windows release builds; validate protected storage and current authorization for fetch/copy/revision after revocation. | REQ-CLIENT-005/006; OWASP MASVS-STORAGE-1; Authorization Cheat Sheet |
| FILE-F15 | A reader may open a message, note or task linked to a File but lacks File read rights. Fetch through every relation and a stale ticket is denied; preview, search and notification paths reveal no protected File content or metadata. The fixture does not choose whether the relation is rejected, hidden or shown as an access-safe placeholder. | ADR-0001; REQ-ACL-020; SPEC-BLOB-STORE-001; SPEC-vida-resource-domain CAP-3/6; OQ-0083 |
| FILE-F16 | A File was previously authorized, locally indexed and visible in global search. After effective revocation, repeat search in Files, Messenger, Notes and Project contexts, including after process restart. The inaccessible File and its name/content snippets no longer appear; a stale index entry cannot bypass the current read gate. This does not decide whether an unrelated, readable relation gets an access-safe placeholder. | REQ-CLIENT-022; REQ-ACL-020; OQ-0083 |

## Evidence and preservation boundary

- Use the adopted BlobStore manifest, interface and acceptance contract for byte-level implementation. These cases add cross-App and client observations but do not replace its verified-transfer, pin or GC rules.
- The sources approve per-Space Files views, stable File identity, versioning, one payload with multiple relations, offline staged save, download admission, failure status and explicit conflict variants. They do **not** choose cryptographic primitives, numeric thresholds, replication defaults or safe GC frontier.
- The exact conflict acceptance proof is still `OQ-0033/0034`; transport arrival, local commit and a CRDT-rendered value cannot substitute for it.
- `FILE-F15` asserts the existing access boundary, not a sharing flow. Sender grant prompts, relation creation and recipient placeholder behavior remain an `OQ-0083` product decision.
- Unless a scenario is explicitly OS-specific, these cases run on installed Android, iOS and Windows release builds. An unexecuted platform path is not a pass (`REQ-CLIENT-005/006`).
