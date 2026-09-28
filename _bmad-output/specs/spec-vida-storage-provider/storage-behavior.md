# OQ-0036 storage behavior and provider evaluation — draft

Approved requirements and architecture take precedence over this draft kernel and its companions. This companion separates **required observable behavior** from **candidate implementation** and **unresolved decisions**.

## Storage boundary

| Concern | Runtime/Core owner | Provider obligation |
|---|---|---|
| Origin command | Validate actor, schema, causal/base conditions; assign stable operation ID and signing context. | Atomically persist the supplied validated operation, outbox intent, dedupe marker and origin-durable frontier. No domain decision. |
| Received operation | Check signature, rights, causality and acceptance evidence; never reissue command. | Persist validated/pending history and apply checkpoint idempotently. |
| Read model | Declare minimum frontier and whether state is tentative or accepted. | Return committed data and checkpoint; projection may rebuild. |
| Blob | Authorize reference and control pin/retention policy. | Stage bytes, verify expected identity, commit manifest/reference only with recoverable payload or explicit remote availability; report download failure without fabricating local bytes. |
| Backup/migration | Select permitted logical version and recovery policy. | Provide atomic checkpoints/integrity evidence and explicit failure; never invent semantic conversion. |

Port charter must version atomicity and consistency, idempotency, cancellation/timeout, typed errors, metadata exposure and fault injection as required by architecture AD-14. A replacement provider passes the **same** conformance fixtures. A physical database layout is private and cannot redefine the public operation/schema contract.

## Commit and recovery invariants

1. Before durable commit: UI may show an in-progress state, not “збережено локально”; retry must use the same logical operation ID when a result is unknown.
2. After durable commit: operation, outbox, dedupe and origin frontier are recoverable together. Replica receipt, delivery receipt, authority acceptance and external-effect result remain separate axes.
3. After restart: load committed log and frontier, resume pending outbox and incomplete downloads, then rebuild/checkpoint projections; mark runtime ready only after the required replay frontier. A missing UI callback must not cause a second command.
4. Quota/disk-full/corruption: return an explicit typed failure. Never turn a failed write into a saved badge or silently prune a pending candidate to make room.
5. Files: local-origin payload+manifest+outbox must have a recoverable staging path. Inbound manifest without bytes is a valid **not downloaded** state; it is not a completed local attachment.
6. Physical schema migration and AppPackage domain schema migration are separate. Historical operation versions remain legible; unknown fields round-trip or fail explicitly, never disappear.

## Candidate evaluation, not a library decision

| Candidate | Why test it | Evidence required before lock |
|---|---|---|
| SQLite with indexed columns/FTS and separate immutable blob store | Research recommends it for local transactions, projections and search; broad Android/iOS/Windows availability. | One-writer lifecycle, commit/fsync mode, encrypted-at-rest variant, backup API, migration, WAL checkpoint, disk-full and kill/power-loss fixtures in release builds. |
| Alternative provider behind the same port | Proves portability claim rather than only a trait shaped around SQLite. | Same golden history/frontier hashes and error semantics; measured cost of rebuild, query and blob staging. |

The [storage research](../../../research/storage-format-architecture-research.md) proposes PDM, deterministic CBOR, SQLite, BLAKE3 and conditional Zstandard. Those are **different decisions**: canonical wire bytes, projection engine, content identity and compression cannot be approved as a bundle merely because they appear in one research note. The [stack brief](../../../docs/03-architecture/stack-selection-brief.md) calls SQLite/FTS an initial projection and leaves provider swap OQ-0036 open. Current SQLite documentation notes that WAL `synchronous=NORMAL` can lose a committed transaction after power failure, whereas `FULL` synchronizes the WAL on each commit; the exact VIDA “saved” guarantee therefore needs measured settings and fault tests ([SQLite PRAGMA](https://sqlite.org/pragma.html)). Multiple attached SQLite databases are not a power-loss-atomic unit in WAL mode, so the design must not assume a transaction across separate vault DB files ([SQLite temporary files](https://sqlite.org/tempfiles.html)).

## Initial conformance matrix

The [fault-injection recipes](fault-injection-cases.md) give semantic cut points and post-restart oracles for this matrix. They specify required observations, not a physical DB schema or a claim that any fixture has passed.

| ID | Fault or workflow | Pass condition |
|---|---|---|
| STO-01 | Kill before/after each origin transaction boundary. | Published logical operation/outbox/dedupe/origin frontier are all present or all absent; unreferenced staged bytes may remain for safe collection. No false saved state. |
| STO-02 | Kill after commit before UI receipt; retry same intent. | Provider recovers the same operation ID/frontier; runtime proves one domain apply and no originating-command replay; UI recovers the durable result. |
| STO-03 | Duplicate inbound delivery with missing causal dependency. | One validated record, dependency remains pending, no new business command. |
| STO-04 | Projection corrupted or behind log. | Rebuild to declared frontier or explicit not-ready/error; signed history unchanged. |
| STO-05 | Disk fills during origin file stage or inbound download. | Origin never publishes dangling ref; inbound file remains not-downloaded with retry path. |
| STO-06 | Two incompatible file variants, offline peer returns late. | Both variants remain recoverable until proven safe frontier; local cleanup does not decide domain winner. |
| STO-07 | Backup/restore with pending outbox, old schema and unknown extension. | Stable IDs/history/pending/unknown fields preserved or explicit compatibility failure; no invented backup claim. |
| STO-08 | Interrupted physical migration and process restart. | Either previous readable layout or resumable forward state; no partially activated domain schema. |
| STO-09 | Shared Space right revoked while local projection/search exists, or no authority-confirmed rights refresh for seven days; include restart, stale head and clock rollback. | Read gate blocks unauthorized content under `NFR-SEC-005` for every role, including Owner; storage ciphertext presence is not access permission. |
| STO-10 | Same fixtures on alternate provider and three installed platforms. | Equivalent logical history/frontiers and typed outcomes; resource budgets recorded separately. |
| STO-11 | Process kill versus abrupt device power loss after an acknowledged commit, for each selected journal/sync policy. | Published durability tier matches recovered history; a mode that can lose the last acknowledged transaction cannot back the same “збережено локально” claim. |
| STO-12 | Live backup while writes continue; interrupt backup, then restore. | Restore reaches one internally consistent frontier; incomplete backup never replaces the last verified backup; pending outbox and keys follow the documented recovery path. |
| STO-13 | Inject disk-full/IO error at write, commit, temporary-file, checkpoint and backup boundaries. | Runtime reopens and completes recovery to its required frontier before querying authoritative persisted state by stable operation or backup ID; only then is absence a typed failure or presence a recovered committed result. A partial origin operation is never published. |
| STO-14 | Insert, edit, delete and rebuild searchable private text. | Search projection matches the committed log at its declared frontier; stale index terms and plaintext artifacts are checked against the vault privacy policy. |
| STO-15 | Lock/unlock, wrong key, tampered page and restored encrypted backup on each target platform. | No private plaintext in DB/WAL/journal/temp/log artifacts within the stated threat model; wrong key and tampering fail explicitly; key custody/recovery is verified separately from DB encryption. |
| STO-16 | Competing windows/processes and platform suspend/resume around one local writer. | No split writer, duplicate commit, stale frontier publication or unrecoverable lock; effective runtime settings are recorded per Android/iOS/Windows build. |
| STO-17 | After a newer persisted-state version and valid rights/control head have been committed, attempt to open or restore an older otherwise-valid backup or provider layout. | Stale restore/downgrade cannot silently replace the newer recoverable state or reopen expired/revoked shared data; unsupported recovery fails explicitly before runtime readiness. The exact monotonic proof and recovery path remain open under OQ-0053/OQ-0037. |
| STO-18 | Race blob collection with publication of a new authorized pin/reference; inject crashes before and after metadata and byte removal. | A committed live pin always retains recoverable verified bytes; otherwise publication fails without a dangling reference. Restart repeats collection safely and retains conflict variants until their separate safe frontier. |
| STO-19 | Attempt to prune old operation history, outbox/dedupe records, unknown extensions and blob references while a peer is offline; crash during the prune. | Prune proceeds only for records proven unnecessary by the selected safe-frontier/retention rule; otherwise it defers. Restart preserves pending work, replay/idempotency evidence and every required recovery path. This fixture does not select the OQ-0034 frontier algorithm. |
| STO-20 | Create an autonomous Persona; inject failure before/during/after staging of stable Persona/Space IDs, keys, recovery material and Owner authority; repeat at separate-storage confirmation and restart. | Reopen shows either no bootstrap, the same protected `setup_pending` unit, or the same active Persona/Space with durable confirmation and one `PersonaCreated`. A partial grant, regenerated secret, false active state or protected work before confirmation fails. This fixture is semantic and does not select the provider, key format or encryption profile. |

Security review uses [OWASP MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) for sensitive local storage and [OWASP Key Management](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html) for key lifecycle. These sources do not prescribe a VIDA encryption crate or prove that SQLCipher, an OS key store or any backup design is sufficient. The exact threat model, key rotation, backup custody and anti-rollback behavior remain decision gates.
