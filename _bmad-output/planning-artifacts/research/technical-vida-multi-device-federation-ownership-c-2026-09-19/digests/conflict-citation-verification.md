# Independent citation verification: concurrent offline edits

Verified 2026-09-19 against the four primary publishers cited in `research.md` section “Поглиблення: що роблять референси з паралельними офлайн-правками.” Scope is semantic support, not an endorsement of the proposed VIDA design.

## Verdict

Three table rows are supported as written: Automerge [6], Linear [26], CouchDB [28]. CloudKit [27] needs one qualification: `serverRecordChanged` is the result of the `ifServerRecordUnchanged` save policy (the documented default), **not** every save with a stale `recordChangeTag`. CloudKit's `changedKeys` and `allKeys` policies do not compare change tags. Add a direct Apple citation for `savePolicy` or qualify the row inline.

The recommendation is correctly labelled as a **VIDA project inference**, not an observed common protocol. None of these sources proves that `baseRevision`, causal frontier, pending-conflict UX, or operation-family rules are the only/standard implementation. The examples support the *problem and option space*, not a specific VIDA contract.

## Claim checks

| Source | Checked support | Nuance / potential mismatch |
|---|---|---|
| [6] [Automerge Conflicts](https://automerge.org/docs/reference/documents/conflicts/) | Concurrent updates of the same property yield one deterministic visible winner while conflicting values remain accessible via `getConflicts`; “last writer” uses internal operation ID (counter then actor ID), not wall-clock. The table is accurate. | “Scalar” is a narrower example than the source's same-property case; this causes no overclaim. The source does **not** establish which value best represents human intent. |
| [26] [Linear: Real-time sync and offline](https://linear.app/docs/get-the-app) | Changes that cannot reach the backend are stored locally and retried, including after restart. Linear explicitly says it does not check each change's creation date before updating data and offline edits can overwrite another teammate's description/status changes. The table is accurate. | Do not infer Linear uses global LWW, a specific CRDT, or any same-person precedence rule. Its example is other *team members*, though the generic hazard can also arise across one person's devices. |
| [27] [Apple `serverRecordChanged`](https://developer.apple.com/documentation/cloudkit/ckerror/serverrecordchanged) + [Apple `recordChangeTag`](https://developer.apple.com/documentation/cloudkit/ckrecord/recordchangetag) + [Apple `savePolicy`](https://developer.apple.com/documentation/cloudkit/ckmodifyrecordsoperation/savepolicy) | A record carries a change tag. Under `ifServerRecordUnchanged`, a stale tag causes save rejection with `serverRecordChanged`; the error supplies client, server, and ancestor copies, and Apple instructs resolving against the server copy then retrying. | The draft's unqualified “при застарілій версії сервер повертає конфлікт” overgeneralizes. `changedKeys` and `allKeys` do not compare tags. Proposed replacement: “За політики `ifServerRecordUnchanged` (типової) сервер порівнює `recordChangeTag`; за застарілої версії повертає `serverRecordChanged` із client/server/ancestor для злиття та повторної спроби.” Source [27] alone mentions old change tags, but the direct `savePolicy` page makes the conditional explicit. |
| [28] [Apache CouchDB replication and conflicts](https://docs.couchdb.org/en/stable/replication/conflicts.html) | Divergent revisions replicate; peers choose the same deterministic visible winner; losing revisions remain available as conflicts; applications can present or merge them. The table is accurate. | CouchDB also rejects stale `_rev` with 409 on a *single node*, distinct from conflicts produced by cross-node replication. If used as a model, keep these two paths separate. |

## Recommendation audit

- “No global later timestamp wins” is a reasonable VIDA policy recommendation. Automerge's winner is operation-ID-based; Linear cautions that ignoring creation dates can overwrite changes; CouchDB's deterministic winner is not human-intent resolution. These observations support concern, **not** a universal ban on timestamp ordering.
- Carrying `baseRevision` / causal frontier to tell whether an operation observed another is a sound **design inference**. Apple `recordChangeTag` and CouchDB `_rev` exemplify conditional writes, but do not establish a generic federated causal-frontier format.
- “If second action saw first, sequential; otherwise concurrent” is a causal-model interpretation and should remain an explicitly stated VIDA rule. Wall-clock order or the fact that one Persona owns both devices does not supply that observation relation.
- “Both operations remain in history” is a **proposed VIDA requirement**, not a universal reference behavior: CloudKit's conditional save rejects the stale write; Automerge/CouchDB retain alternatives in their own models. Wording should not imply every reference preserves rejected operations.
- “Pending conflict with explicit confirmed new intent” is a VIDA UX/algorithm proposal. CouchDB suggests surfacing conflicting versions; Apple suggests merge/retry. Neither mandates this exact workflow.
- “For low-value fields, schema-specific deterministic LWW; for status/booking/effects, no silent LWW” is domain-specific product judgement. It is not directly dictated by [6], [26], [27], or [28], and the report already labels it a recommendation.

## Suggested minimal patch to parent report

1. Qualify CloudKit row as above and cite the [Apple save policy documentation](https://developer.apple.com/documentation/cloudkit/ckmodifyrecordsoperation/savepolicy) alongside [27].
2. In the recommendation, retain the “проєктна рекомендація, ще не рішення” label. Optionally change “обидві операції лишаються в історії” to “у пропонованій VIDA обидві операції лишаються в історії” to avoid accidentally attributing that behavior to CloudKit.
3. Keep Linear as a documented caution, not as a canonical merge design.
