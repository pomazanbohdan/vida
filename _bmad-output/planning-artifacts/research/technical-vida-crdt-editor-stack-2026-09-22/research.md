---
title: VIDA CRDT and collaborative-editor stack
date: 2026-09-22
status: completed-research-prototype-required
scope: OQ-2 / OQ-0073
---

# VIDA CRDT and collaborative-editor stack

## Executive conclusion

Документація звужує CRDT core до **Loro 1.x**, **Automerge 3** і контрольного **Yrs**. Вона не доводить готовність жодної пари `CRDT + Flutter editor` для Release 1. Рекомендований порядок прототипування: Loro першим через найближчу до VIDA форму даних; Automerge як обов'язковий контроль explicit-conflict/history; Yrs як обов'язковий ecosystem-контроль.

Фінальний lock заборонено робити лише за feature list. Переможець має пройти однакові fixtures на Android, iOS і Windows. Поточний OQ-2 лишається відкритим до такого доказу.

## Вже затверджена межа

- Flutter Android/iOS/Windows є Release-1 shells; доменна семантика живе в Rust Core.
- Iroh транспортує opaque versioned payload; transport ACK не визначає merge, authority або finality.
- CRDT застосовується лише до mergeable document structures. Task status, booking, approval, file replacement та ownership лишаються VIDA domain operations/state machines.
- Durable document operations, transient presence і blobs є різними data planes.
- SQLite/FTS/search/UI state є відтворюваними projections, а не канонічним Notes state.
- Live cursor/selection уже є обов'язковою Release-1 вимогою `REQ-CLIENT-021`; попередня PRD-умова про optional cursors була суперечністю.

## Candidate evidence

| Candidate | Сильна відповідність VIDA | Межа/ризик |
|---|---|---|
| Loro 1.x, MIT | First-class Rust API; rich text; stable Cursor; Tree/MovableList; version DAG/time travel; snapshots, updates, shallow snapshots; EphemeralStore; існує малий Iroh+Loro demo | LoroMap є LWW; немає офіційного Dart/Flutter editor binding; iroh-loro — лише demo на 2 peers/plain text |
| Automerge 3, MIT | Rust core; rich-text marks/block markers; stable cursors; explicit concurrent property values; compact history/storage; transport-agnostic sync | Офіційний Rust API названо low-level/недодокументованим; editor bindings переважно JS; Dart/Flutter bridge відсутній |
| Yrs, MIT | Rust Yjs implementation; YText/XML, StickyIndex, Awareness, snapshots/diffs; найбільший editor ecosystem; AppFlowy є референсом Flutter+Rust+Yrs | Map conflict semantics LWW; official Flutter binding відсутній; schema migration лишається прикладною |
| cr-sqlite | SQLite locality | Rich-text і causal/history model не закривають VIDA gate; mobile extension packaging додає ризик |
| RxDB/Electric | Сильні централізовані replication/data patterns | Не відповідають headless Rust+Iroh autonomous P2P core; не editor CRDT для VIDA |

LWW-map не є автоматичною дискваліфікацією Loro/Yrs: VIDA не делегує task/file/business conflicts CRDT map. Водночас Automerge `getConflicts` також не замінює operation-specific authority і UX.

## Architecture under test

```text
Flutter Editor
  ↕ EditorIntent / EditorPatch (versioned VIDA port)
Rust CollaborativeDocumentPort
  ↕ CRDT engine adapter
Signed VIDA operation log + encrypted snapshots
  ↕ opaque update frames
Iroh transport / durable mailbox
```

Canonical proposal for the prototype:

- one Note resource = one independently authorized collaborative document;
- stable block IDs + rich-text container per block + relation IDs for embeds;
- comments use stable range anchors, never raw integer offsets;
- presence/cursors/selections are encrypted transient messages with TTL and never durable history;
- editor emits intents and consumes patches; Dart never becomes the merge authority;
- restore of an old revision creates a new operation;
- rejected/revoked operations never enter the visible document; sealed quarantine may exist outside the CRDT document;
- engine-native sync does not bypass `SPEC-OPERATION-FINALITY-001` or VIDA ACL/current-epoch checks.

## Editor candidates are a separate decision

CRDT and editor must not be scored as one library.

- AppFlowy Editor: block-oriented Flutter UI and a real Flutter+Rust/Yrs reference; dual MPL-2.0/AGPL license requires dependency/license discipline; its collaboration layer is not a reusable VIDA contract.
- Flutter Quill: MIT, Android/iOS/Windows editor and Quill Delta model; collaboration adapter and block-tree mapping must be built and tested.
- SuperEditor/Fleather: admissible prototype controls; neither may own canonical merge state.
- A custom editor adapter is considered only after published candidates fail the same fixtures.

## Decision method

### Hard gates

1. deterministic convergence across three replicas, all delivery permutations and duplicate replay;
2. kill/restart safety at commit/export/import/apply/compaction boundaries;
3. ACL/current-epoch rejection before visible apply, search or history;
4. mixed v1/v2 clients preserve unknown data and do not perform lossy writes;
5. Iroh adapter resumes idempotently and never creates a business winner;
6. Android/iOS/Windows IME and screen-reader workflows have no composition loss, caret jumps or semantic-tree break.

Any hard-gate failure rejects the candidate pair.

### Weighted comparison after hard gates

| Dimension | Weight |
|---|---:|
| Merge semantics | 22 |
| Rich-text/block model | 14 |
| Anchors/history | 12 |
| Persistence/compaction | 12 |
| Mixed-version/schema | 10 |
| Rust↔Flutter patch path/performance | 10 |
| Editor IME/accessibility | 10 |
| Iroh integration | 5 |
| Security/ACL boundary | 5 |

Selection requires score `>=80/100`, no hard-gate failure. A lead `<=10` is inconclusive until repeated on three representative device classes.

## Prototype fixture set

The normative draft is `docs/04-specifications/crdt-editor-conformance.md`; machine-readable scenarios are `docs/04-specifications/fixtures/crdt-editor-v1.yaml`.

F01 concurrent text/marks; F02 move-delete-edit block; F03 cursor/comment anchors; F04 presence expiry; F05 history restore; F06 mixed schema; F07 revoke/key epoch; F08 Iroh reorder/duplicate/reconnect; F09 kill points; F10 compaction plus stale replica; F11 mixed CRDT wire versions; F12 Rust↔Flutter patch burst; F13 IME/accessibility.

## Prototype status

No runtime bake-off has been accepted yet. A delegated harness intentionally did not mutate the already dirty shared worktree under the BMad Build safety gate. Therefore no candidate has measured PASS, serialized-size, memory, latency, mobile, IME or accessibility evidence. This is a proof gap, not permission to infer a winner.

## Primary references

- [Automerge rich text](https://automerge.org/docs/reference/documents/rich-text/), [conflicts](https://automerge.org/docs/reference/documents/conflicts/), [storage](https://automerge.org/docs/reference/under-the-hood/storage/), [Rust status](https://github.com/automerge/automerge)
- [Loro Rust](https://docs.rs/loro/latest/loro/), [Text/Cursor](https://loro.dev/docs/tutorial/text), [Ephemeral Store](https://loro.dev/docs/tutorial/ephemeral), [Loro FFI](https://github.com/loro-dev/loro-ffi), [iroh-loro demo](https://github.com/loro-dev/iroh-loro)
- [Yrs feature/FFI matrix](https://github.com/y-crdt/y-crdt), [Yjs Awareness](https://docs.yjs.dev/api/about-awareness), [AppFlowy Collab reference](https://github.com/AppFlowy-IO/AppFlowy-Collab)
- [AppFlowy Editor](https://github.com/AppFlowy-IO/appflowy-editor), [Flutter Quill](https://github.com/singerdmx/flutter-quill)

