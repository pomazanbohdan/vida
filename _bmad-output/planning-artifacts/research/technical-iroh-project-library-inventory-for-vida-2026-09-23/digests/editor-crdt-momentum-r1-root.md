# Editor CRDT candidates: development and issue evidence

Snapshot: 2026-09-23 UTC. Scope: the upstream Rust-capable libraries used by Iroh application references, not an assertion that either has passed VIDA's [editor fixtures](../../../../../docs/04-specifications/crdt-editor-conformance.md). Counts come from GitHub REST `commits` on the default `main` branch and `search/issues` with `is:issue` or `is:pr`; 30 days start 2026-08-24 UTC, 90 days start 2026-06-25 UTC. Counts include merges, release commits and automation; issue counts are separate from PRs.

| Candidate | Default-branch commits 30d / 90d | Open issues / open PR | Issues closed 90d | Interpretation |
|---|---:|---:|---:|---|
| [Loro](https://github.com/loro-dev/loro) | 29 / 58 | 39 / 12 | 7 | Recent fixes for UTF-16 text cursors, snapshot history and concurrent import, plus WASM tree/deep-read features; versioning commits are also in the count. |
| [Automerge](https://github.com/automerge/automerge) | 31 / 143 | 71 / 23 | 40 | Recent rich-text patch, author tracking and sync/change-graph fixes; dependency bumps and release commits are also in the count. |

Sources for repeatable count queries: [Loro commits](https://github.com/loro-dev/loro/commits/main/), [Loro issues](https://github.com/loro-dev/loro/issues), [Loro PRs](https://github.com/loro-dev/loro/pulls), [Automerge commits](https://github.com/automerge/automerge/commits/main/), [Automerge issues](https://github.com/automerge/automerge/issues), [Automerge PRs](https://github.com/automerge/automerge/pulls), [GitHub REST issues semantics](https://docs.github.com/en/rest/issues/issues#list-repository-issues). These are point-in-time API-query results, not quality rankings or sustained-response measurements.

## Implemented functionality relevant to VIDA

| Function | Loro evidence | Automerge evidence | VIDA gate |
|---|---|---|---|
| Concurrent text/rich text | [Text and marks](https://loro.dev/docs/tutorial/loro_doc) | [Text and rich-text marks/blocks](https://automerge.org/docs/reference/documents/rich-text/) | Same F01–F13 editor scenarios, including same-sentence overlap and mark boundaries. |
| Stable cursor/selection anchors | [Cursor](https://loro.dev/docs/tutorial/cursor) | [Rust Cursor and patch API](https://automerge.org/automerge/automerge/) | Cross-device cursor movement, Unicode and deleted-range restoration. |
| Incremental sync and history | [Export/import updates and history](https://loro.dev/docs/tutorial/sync) | [Rust sync protocol and historical heads](https://automerge.org/automerge/automerge/) | Iroh session adaptation, duplicate/reordered delivery and restart; Automerge's documented sync protocol assumes a reliable in-order stream. |
| Structured document data | [Map, list, tree, movable list](https://loro.dev/docs/tutorial/loro_doc) | [Nested maps/lists](https://automerge.org/automerge/automerge/) | Notes block-tree model and migrations; do not assume equivalent move semantics. |
| Exposing conflicting values | [Map LWW and mergeable-child caveat](https://loro.dev/docs/tutorial/map) | [Deterministic winner plus `get_all`](https://automerge.org/automerge/automerge/) | VIDA must surface applicable unresolved variants; neither library's default visible value decides task/booking authority. |

The table establishes **four common capability families** (text, cursor, sync/history, structured data) and one conflict-behavior comparison, not five equivalent capabilities. Loro's documented Map LWW behavior may hide a concurrently created regular child; Automerge exposes conflicting values through `get_all`. Neither result is a VIDA fixture pass: no F01–F13 suite was executed here. A broader API does not compensate for failing a required data-integrity fixture.

## Issue-response examples, not a tracker-wide SLA

- [Loro #1068](https://github.com/loro-dev/loro/issues/1068) reports a panic when importing an update depending on shallow-snapshot-folded operations (opened 2026-08-11). Two visible comments through 2026-09-02 are from accounts labeled `NONE`, not a visible maintainer response; the issue remained open in this snapshot. This is a high-relevance recovery risk to test, not proof all Loro bugs are ignored.
- [Automerge #825](https://github.com/automerge/automerge/issues/825) reports path-splice handling of `/` (opened 2023-12-15). A contributor answered within about nine minutes and supplied a workaround; a collaborator kept the issue open for a proper fix. Fast initial response and a long-lived unresolved issue can both be true.

Before selection, sample more severe and ordinary issues across both projects, inspect linked fixes, release notes and the exact versions used by Synesis/echo/Kith. Recount at ADR time. No `<1.0` version is a rejection criterion; compatibility, license, security and fixture results remain independent gates.
