---
id: SPEC-COLLABORATIVE-DOCUMENT-CONFORMANCE-DRAFT
status: draft
implementation_status: prototype-required
last_updated: 2026-09-22
requirement_refs:
  - ../02-requirements/native-client-requirements.md
  - ../02-requirements/transport-sync-requirements.md
decision_refs:
  - ../03-architecture/decisions/ADR-0016-equal-device-peers.md
  - ../03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md
  - ../03-architecture/decisions/ADR-0020-flutter-windows-in-release-1.md
research_refs:
  - ../../_bmad-output/planning-artifacts/research/technical-vida-crdt-editor-stack-2026-09-22/research.md
---

# Collaborative document and editor conformance gate

## Status and purpose

Цей draft визначає однаковий доказ для CRDT engine та Flutter editor adapter. Він **не обирає** Loro, Automerge, Yrs або editor widget до виконання fixtures.

## Normative boundaries

| ID | Requirement |
|---|---|
| REQ-COLLAB-001 | Один Note resource `MUST` мати окремий collaborative document identity й authorization boundary; engine document ID не замінює VIDA Resource ID. Canonical `(SpaceId, ResourceId, DocumentId)` binding is immutable and signed; a same `DocumentId` in another Space never authorizes apply. |
| REQ-COLLAB-002 | Canonical Note model `MUST` мати stable block IDs, ordered/movable block structure, rich-text content, embeds як Resource relations і stable range anchors для comments/selections. |
| REQ-COLLAB-003 | Rust `CollaborativeDocumentPort` `MUST` бути єдиним canonical owner merge/history semantics. Flutter editor `MUST` надсилати versioned intents і застосовувати patches; повна canonical state заміна з Dart заборонена. |
| REQ-COLLAB-004 | Presence, cursor і selection `MUST` бути transient encrypted state з TTL, не входити до durable document/history/snapshot і не впливати на authority/finality. |
| REQ-COLLAB-005 | CRDT update and transient presence `MUST` bind signed Space, Resource and Document IDs and pass signature, current ACL/control epoch, schema and document checks before visible apply. A relation or local document lookup never grants target read/write. Rejected bytes `MUST NOT` потрапити в visible history/search; sealed quarantine зберігається поза document state. |
| REQ-COLLAB-006 | Engine sync/update bytes `MUST` транспортуватися через versioned VIDA/Iroh adapter і `MUST NOT` створювати `Synchronized`, Delivered або AuthorityOutcome без application receipts за `SPEC-OPERATION-FINALITY-001`. |
| REQ-COLLAB-007 | CRDT merge `MUST NOT` визначати task status, exclusive claim, approval, ownership або file-replacement winner. Ці operation families лишаються VIDA domain semantics. |
| REQ-COLLAB-008 | Independent non-overlapping text/block edits `MUST` converge without loss. Overlapping destructive edits `MUST` retain enough anchored transaction evidence to render both user-relevant variants when `REQ-SYNC-006` classifies them incompatible. Engine materialization alone is not proof of this UX. |
| REQ-COLLAB-009 | Restore revision `MUST` create a new durable operation; history/audit `MUST NOT` be rewritten. Comments and anchors `MUST` have deterministic behavior after delete, move, restore and compaction. |
| REQ-COLLAB-010 | Snapshot/compaction/GC `MUST` preserve recovery and declared stale-replica compatibility. A peer older than retained history receives an explicit compatible snapshot path or explicit unsupported-version result, never silent divergence. |
| REQ-COLLAB-011 | Mixed schema/client versions `MUST` round-trip unknown fields and avoid lossy writes. A client unable to edit safely becomes read-only for the affected AppInstance, not for all VIDA. |
| REQ-COLLAB-012 | Live remote cursor/selection is a Release-1 gate on Android, iOS, Windows and Web when peers are connected; fading remote-change highlight is local ephemeral UX. |

## Candidate set

- CRDT cores: Loro first-run candidate; Automerge mandatory control; Yrs ecosystem control.
- Flutter editors: AppFlowy Editor, SuperEditor, Fleather і Flutter Quill як окремі comparable adapters; custom adapter only after documented failure of published candidates.
- Prototype `MUST` evaluate the declared CRDT×editor pair matrix. Every skipped pair requires recorded incompatibility evidence; candidate versions `MUST` be pinned in lockfiles and result records.

## Hard gates

`G1` convergence; `G2` crash durability; `G3` ACL/current epoch; `G4` schema/mixed client; `G5` Iroh boundary; `G6` Android/iOS/Windows/Web editor input and accessibility (including browser keyboard, IME and focus); `G7` encryption, signature and document binding for durable updates and transient presence. Any failure rejects the pair.

## Weighted rubric

`merge 22 + rich/block 14 + anchors/history 12 + persistence 12 + schema 10 + Rust↔Flutter 10 + IME/a11y 10 + Iroh 5 + security 5 = 100`. Each dimension receives `1.0` only with complete passing evidence, `0.5` for an explicitly safe limitation with all required evidence and no hard-gate failure, or `0` for failure/missing evidence; total is `sum(weight × value)`.

Winner: `>=80`, no hard-gate fail. Lead `<=10` remains inconclusive until repeated on three representative device classes.

## Required fixtures

Machine-readable source: [crdt-editor-v1.yaml](fixtures/crdt-editor-v1.yaml).

| Fixture | Required proof |
|---|---|
| F01 | Concurrent insert/delete/mark on same and different ranges; convergence and retained user intent |
| F02 | Peer A moves a block while peer B deletes/edits it; deterministic no-loss result or explicit variant |
| F03 | Comment/cursor anchors survive insert, inside-delete and block move with declared fallback |
| F04 | Presence expires, disconnects and rejoins; forged, wrong-document and unauthorized presence is rejected; wire payload remains encrypted; presence never appears in durable history |
| F05 | Old revision restore is a new operation and converges with concurrent edit |
| F06 | v1/v2 optional/default/rename/unknown-field clients avoid lossy write |
| F07 | Grant/revoke/key epoch, signature and document binding block stale, forged, cross-document visible apply and resurrection |
| F08 | Iroh loss/reorder/duplicate/reconnect produces one convergent apply and no business winner |
| F09 | Fifty deterministic kill/restart cut points across local commit/export/import/apply/compaction |
| F10 | Compact snapshot plus stale replica reconnect has explicit safe path |
| F11 | Supported old/new CRDT wire versions interoperate or fail explicitly |
| F12 | 1,000 local edits plus remote patch burst crosses Rust↔Flutter without semantic loss and records raw frame/input/patch/memory data. Until OQ-4 sets numeric budgets, measurements are evidence-only and do not use an invented pass threshold. |
| F13 | Gboard, SwiftKey, iOS, Windows IME; emoji/CJK/RTL; TalkBack/VoiceOver/Narrator |
| F14 | Same `DocumentId` in two Spaces, wrong Resource binding and cross-Space replay fail closed without leaking target metadata or altering history |

## Evidence required to change status

The YAML defines canonical seed/documents/principals, durable and presence envelopes, delivery defaults, assertion result schema and scoring formula. Canonical fixture serialization currently references the project-wide deterministic CBOR decision `OQ-0028`; until that profile and an executable operation/assertion registry exist, the YAML remains a scenario contract rather than runnable proof. Candidate adapters translate canonical inputs without weakening assertions.

Status may become `approved` only after a reproducible result bundle contains source, lockfiles, exact toolchain/devices, canonical input hash, schedules, raw fixture results, failure logs, size/latency/memory measurements, accessibility evidence and an ADR selecting both engine and editor adapter boundary.
