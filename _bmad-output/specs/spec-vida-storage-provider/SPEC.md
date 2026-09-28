---
id: SPEC-vida-storage-provider
status: draft
companions:
  - storage-behavior.md
  - fault-injection-cases.md
  - provider-evidence.md
  - ../../planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md
  - ../../../docs/02-requirements/native-client-requirements.md
  - ../../../docs/02-requirements/platform-nfr.md
  - ../../../docs/02-requirements/transport-sync-requirements.md
  - ../../../docs/04-specifications/sync-log-contract.md
  - ../../../docs/04-specifications/blob-store-contract.md
  - ../../../docs/04-specifications/schema-evolution-contract.md
  - ../../../research/storage-format-architecture-research.md
  - ../../../docs/03-architecture/stack-selection-brief.md
sources: []
---

> **Decision-gated OQ-0036 kernel.** This draft states required storage behavior and a provider conformance plan. It does not approve SQLite, an encryption scheme, a snapshot format, a backup format or power-loss durability settings. Approved requirements and architecture control if this draft or an evidence companion conflicts with them.

# VIDA local storage provider and recovery

## Why

Messenger, Notes, Projects and Files must continue offline, survive process death and synchronize later. A storage library can be fast and still lose a committed outbox entry, advance the wrong frontier or report an incomplete attachment as saved. VIDA needs a replaceable mechanism with one testable meaning of local durability.

## Capabilities

- **CAP-1**
  - **intent:** Commit a local operation and the recovery data needed to send it as one logical transaction.
  - **success:** At every injected crash boundary, either the complete operation, outbox, dedupe and origin-durable frontier are recovered under one stable ID or none are presented as saved; Persona bootstrap and activation recover as complete semantic states under STO-20.
- **CAP-2**
  - **intent:** Rebuild state after restart without repeating business commands or external effects.
  - **success:** Replay yields the same validated log/frontier and derived-state hashes; tentative, delivered and authority-accepted states remain distinct.
- **CAP-3**
  - **intent:** Recover attachments without dangling references or silent loss of variants.
  - **success:** An incomplete local file write reports failure; an inbound failed download remains not-downloaded; retained conflict payloads remain recoverable until a separately proven safe cleanup frontier.
- **CAP-4**
  - **intent:** Replace the physical provider without changing product behavior.
  - **success:** A common fault-injection suite produces equivalent operation IDs, recoverability, typed errors and read-frontier behavior across candidate providers.
- **CAP-5**
  - **intent:** Upgrade, back up and restore persisted data without silently dropping history, unknown fields, pending work or keys.
  - **success:** Mixed-version and restore fixtures either preserve those items with verifiable integrity or fail explicitly before claiming readiness.

## Constraints

- `vida-runtime` owns transaction/recovery orchestration, one local writer and frontier publication; the provider owns durable primitives, not authorization, conflict resolution or domain acceptance.
- “Збережено локально” follows a verified durable commit, never an in-memory enqueue, transport ACK or projection update. Quota/disk/permission/corruption failures are typed and visible.
- Signed operations/snapshots are durable history; SQL/search/UI projections are rebuildable, frontier-tagged derivatives. Replaying a received operation never reissues its originating command.
- Pending candidates have no age-only expiry; compaction/GC cannot silently evict them, conflict alternatives or the last recoverable blob copy.
- Sensitive local data and keys need a platform-aware protection boundary. Encryption, key custody, recovery material and secure deletion claims require a separate threat-model/OWASP verification.
- SQLite/FTS is the **first provider prototype candidate**, as proposed in the storage research and stack brief; it is not the adopted logical model or an approved provider lock.
- The provider comparison is evidence for prototyping, not a choice: SQLite/SQLCipher, redb and Fjall have different durability, query and protection surfaces. `provider-evidence.md` records the dated source check and common gate.

## Non-goals

- Choosing PDM/CBOR, BLAKE3, Zstandard, SQLCipher or a specific Rust database crate by implication from the research.
- Defining authority ordering (OQ-0033), operation wire bytes (OQ-0028), safe conflict frontier (OQ-0034) or package migration authority (OQ-0040).
- Promising remote erasure, guaranteed backup that has not been made, or power-loss safety solely because a database call returned successfully.

## Success signal

The Android, iOS, Windows and Web vertical slices and a headless harness pass applicable provider fault matrices: crash/restart, disk-full/quota, Persona bootstrap, interrupted blob staging, migration/restore and projection rebuild produce no false “saved” claim, lost pending operation, dangling published reference or duplicate domain effect. Web has a distinct browser provider and loss/eviction limits under ADR-0021. Until the provider tuples and fixtures pass, OQ-0036 remains open.

## Open Questions

- Which power-loss durability tier, filesystem assumptions and SQLite configuration pass the physical-device crash/power-cut gate?
- Which encryption/key/backup design protects each Persona vault and restores it after device loss without weakening revocation?
- Which snapshot/compaction proof, provider ABI/version and one-writer/multi-window lifecycle pass the cross-platform suite?
