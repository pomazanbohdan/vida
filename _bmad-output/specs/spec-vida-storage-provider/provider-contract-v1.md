---
id: VIDA-STORAGE-PORT-V1-CANDIDATE
status: candidate-for-bounded-prototype
updated: '2026-09-29'
---

# StorageProvider v1: спільна поведінка, різні механізми

Derived from this workspace's `.memlog.md`, inherited architecture AD-13/14, STO-01–20 and approved D4–D6. This is a semantic prototype contract, **not** an approved production ABI, provider tuple or cryptosystem. Approved requirements take precedence. The same logical contract does not require Android and Web to use the same physical database.

## 1. Ownership

| Unit | Owns | Does not own |
|---|---|---|
| `vida-core` | validation, signatures, rights, causal/domain rules | disk, browser API, database, writer election |
| `vida-runtime` | stable intent ID, transaction composition, recovery readiness, outbox, projections | changing domain rules to suit a provider |
| Provider | atomic persistence, idempotent lookup, consistent reads, local writer exclusion | signing, authorization, conflict winner, remote acceptance |
| Platform custody port | key handles/unwrapping and platform lifecycle | granting Space access merely because it can decrypt local bytes |

Current code has only `StateStore` and `MemoryStore` in `crates/vida-core/src/storage.rs`. Their synthetic crash cuts are reusable semantic oracles, not persistent adapters. Introduce persistence outside Core; do not make Core depend on SQLCipher, IndexedDB or Flutter. Storage records carry already-validated opaque bytes; controller-event v1 encoding is not silently promoted to generic Note wire encoding.

## 2. Candidate port operations

Names below are semantic, not frozen Rust/Dart signatures. Byte limits, codec and binding representation are recorded in the prototype tuple before execution; public wire fields remain OQ-0028/0035.

| Operation | Input | Required outcome |
|---|---|---|
| `open_and_recover` | vault scope, custody handle, supported local layout | verified committed checkpoint and recoverable jobs, or typed failure; never empty-vault substitution on corruption |
| `acquire_writer` | vault scope | exclusive local handle/generation or `Busy`; generation checked atomically on subsequent writes |
| `commit_origin` | writer generation, expected local checkpoint, stable ID, validated operation bytes, outbox intent and dedupe identity | all authoritative components and new origin checkpoint commit together; no authorization/merge performed by provider |
| `lookup_operation` | scope and original ID, required recovery checkpoint | committed result, proven absence at that checkpoint, or not-ready/unknown |
| `read_history_page` | scope, cursor, required checkpoint, bounded page size | consistent committed history/checkpoint or explicit lag/unsupported error |
| `write_projection` | derivation checkpoint and derived records | rebuildable checkpointed cache; cannot manufacture history, receipts or authority |
| `release_writer` / `close` | live handle | release local resources; committed work remains; stale handles cannot write |

`Frontier` is opaque causal evidence, **not** a clock value or a provider row counter. A local writer generation is only a fencing token for one vault; it gives no network Device priority. A DB checkpoint, transport ACK and independently authorized application receipt are different facts.

No origin transaction spans multiple Persona DBs by assumption. Cross-Space/domain composition and required files follow existing STO-01/05 and BlobStore publication rules; staged unreferenced bytes may remain, published dangling references may not.

## 3. Commit result and retry

| Semantic result | Condition | Runtime/UI behavior |
|---|---|---|
| `Committed` | complete atomic commit verified under the measured provider profile | retain original ID and origin checkpoint; ordinary «Збережено» is allowed; no implicit sync, delivery or backup |
| `ProvenNotCommitted` | known pre-write rejection, confirmed transaction abort, or complete authoritative lookup after recovery | report safe typed error; retry the same logical intent/ID where retryable |
| `OutcomeUnknown` | process/callback/IO interruption cannot determine outcome | retain ID; reopen/recover and lookup; never issue the same action under a fresh ID |
| `NotReady` | recovery or required checkpoint incomplete | do not infer absence or publish an authoritative projection |

Same ID plus identical validated content returns the existing durable result without another domain effect. Same ID plus different content is `IdentityMismatch`, not an overwrite. IDs originate in runtime before submission; the provider cannot turn a retry into a new command. Inbound replay does not generate an origin command/outbox intent.

Error categories: `QuotaOrDiskFull`, `Io`, `Busy`, `WriterFenced`, `KeyUnavailable`, `AuthenticationOrCorruption`, `UnsupportedLayout`, `CheckpointMismatch`, `IdentityMismatch`, `NotReady`. Exact enum mapping belongs to bindings; safe category and original correlation ID survive it. A low-level error alone cannot convert an ambiguous commit into proven failure.

Before every write, atomically verify current writer generation and expected local checkpoint. A suspended old writer cannot commit after takeover. A contender either waits/attaches or gets `Busy`; no wall-clock expiry alone grants writer authority. Cancellation before proven commit may stop work; after commit it cannot erase a saved operation. Durability evidence stays internal/diagnostic, not a new per-save warning.

## 4. Android first prototype profile — not provider adoption

- First native candidate: SQLCipher-backed SQLite in the application-private vault, one writer, one DB transaction for operation/outbox/dedupe/origin checkpoint. Prototype `journal_mode=WAL`, `synchronous=FULL`, file-based temporary stores disabled. Read back effective settings and record library revision/build flags; spelling a PRAGMA is not proof it took effect.
- SQLite documents WAL FULL commit synchronization; VFS/filesystem/storage behavior still requires installed-device and power-cut proof. Plain SQLite may test atomicity with synthetic non-sensitive fixtures, but cannot pass private-data protection merely by those tests. [SQLite PRAGMA](https://sqlite.org/pragma.html)
- SQLCipher protects DB/WAL/journal pages; other transient files require memory-only temporary storage. Inspect search, temp, crash/log and backup artifacts independently. Database encryption does not supply blob encryption or key custody. [SQLCipher design](https://www.zetetic.net/sqlcipher/design/)
- Candidate custody: a recoverable vault key wrapped by a device-local Android Keystore key. Keystore material itself is nonexportable; fresh-device restore needs the recovery path, not the old OS handle. Record actual hardware security level rather than promise universal StrongBox. Exact key hierarchy/backup encoding remains a separate profile gate. [Android Keystore](https://developer.android.com/privacy-and-security/keystore)

## 5. Chromium first prototype profile — not provider adoption

- First comparison candidate: authenticated-encrypted records in IndexedDB; one `readwrite` transaction covers authoritative operation, outbox, dedupe and local checkpoint. Prepare ciphertext before opening the transaction; do not await unrelated asynchronous crypto inside its active request window.
- Request `durability: strict`; declare the observed browser/version behavior. It is a user-agent **hint**, not a guarantee against every physical power loss or user clearing browser data. Wait for transaction completion, not an individual request's success. [IndexedDB 3.0](https://w3c.github.io/IndexedDB/)
- Coordinate the primary writer/sync worker with Web Locks. Every write also checks a persisted writer generation/checkpoint in the same IndexedDB transaction. Do not steal a live lock by elapsed time; test close/crash, suspended leader and stale resume. Two tabs still represent one logical Device, never two replicas. [Web Locks](https://www.w3.org/TR/web-locks/)
- Ask for persistent storage during enrollment; denial leaves Web usable and ordinary saves unchanged. Expose persistence/recovery risk in setup/settings; actual quota failure is a typed write error. Origin/profile clearing loses that local copy and requires restore/re-enrollment, not silent reuse of absent keys.
- Key custody preserving approved automatic reopening remains unproven. Encrypting records while retaining a usable key in the same origin cannot claim protection against malicious same-origin code or a substituted static host bundle. Optional local lock is allowed; mandatory recovery-secret entry on each launch is not introduced here.
- OPFS-backed SQLite is the comparative alternative if IndexedDB fails measured workloads/requirements. OPFS VFS variants differ in worker/concurrency requirements; neither reuse of SQL nor choice of OPFS proves browser encryption, custody or durability. [SQLite Wasm persistence](https://sqlite.org/wasm/doc/tip/persistence.md)

## 6. Security and promotion gate

Protect payloads, keys and derived private search data under a declared threat model; purpose-separated DB/recovery/signing/transport keys are not interchangeable. Never copy recovery encryption's key/nonce rules into record encryption without a separately recorded profile. Log no payload, secret, raw recovery material or identifying vault path. [OWASP MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/), [OWASP Key Management](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html)

Promotion requires the relevant STO/REC/BND results and a reviewed exact provider/custody tuple. Native and browser adapters must reproduce the same logical history and typed outcomes; their different physical loss boundaries remain declared. Redb/Fjall remain viable comparator candidates and are not rejected because of a version below 1. No benchmark, documentation citation or in-memory test closes OQ-0036.
