# Candidate evaluation and conformance plan

This is a comparison protocol, **not a ranking or library decision**. Recheck repository heads, releases, issues and dependency locks immediately before an ADR. Snapshot claims below refer to the [2026-09-23 inventory](../../planning-artifacts/research/technical-iroh-project-library-inventory-for-vida-2026-09-23/research.md).

## Evaluation card (one card per candidate)

| Dimension | Record | Interpretation |
|---|---|---|
| Development trajectory | Dated 30/90-day substantive default-branch commits, active weeks, merged PRs, release dates, maintainer count, direction of work | Count fixes, features and tests separately from bot/dependency churn; inspect cadence and continuity, not only lifetime total or `pushed_at`. |
| Functionality | For each applicable VIDA fixture: implemented + tested / applicable total; platform support and recovery/migration paths | Record pass/partial/absent/unknown per fixture. Breadth matters, but unrelated features cannot compensate for a missing critical invariant. |
| Issues | Open and closed issue counts separately from PRs; severity, age distribution, first-response time, closure/fix evidence and security advisories | Report volume and handling. Zero public issues may mean no users or no triage; an active backlog may reflect adoption. Do not treat count alone as quality. |
| Integration gates | Resolved Iroh/dependency tuple, builds on release platforms, license, security model, maintainability of adapter | A failed gate is a concrete blocker even for a very active project; `<1.0` alone is not. |

Capture each metric with observation date, repository/ref or commit SHA, query method, sample limits and unknowns. Avoid an opaque weighted score: compare the four primary dimensions side by side, then document explicit hard-gate failures and why an ADR chooses or rejects a candidate. A later gate can be reopened when development evidence changes.

## Current comparison set

| Candidate | Relevant pattern from inventory | Proof still required |
|---|---|---|
| Irokle | Signed Ed25519/BLAKE3/postcard history; optional Iroh/Fjall; active 30-day commit sample | VIDA membership/epoch validation, repair, receipt separation, platform and crash fixtures. |
| iroh-db | Typed CRDT/redb/store/sync workspace; open PR activity despite zero merged default-branch commits in sampled window | Resolve Iroh 1.0.2/pinned blobs fork against VIDA tuple; review PR age/response, authority/conflict and restart fixtures. |
| Knot | Revision DAG and custom merge as a pattern, Iroh 1.1 | Determine reusable library boundary, test coverage and whether conflict semantics match VIDA. |
| Minimal VIDA reference | Small implementation of the published log contract, not presumed production winner | Same suite, maintenance burden and feature-completeness comparison. |

The [GitHub REST sample](../../planning-artifacts/research/technical-iroh-project-library-inventory-for-vida-2026-09-23/digests/development-momentum-r1-root.md) and [deeper storage/sync pass](../../planning-artifacts/research/technical-iroh-project-library-inventory-for-vida-2026-09-23/digests/storage-sync-r3-luna.md) captured the following on 2026-09-23. “Commits” includes merges, automation and dependency bumps; issues and PRs are separate GitHub search categories.

| Candidate | Default-branch commits / 30d, 90d | Open issues | Open PRs | Immediate follow-up |
|---|---:|---:|---:|---|
| Irokle | 481, 523 | 0 | 0 | Repair/ack PRs are relevant, but classify substantive source changes and test/release cadence; zero reported issues is not defect proof. |
| iroh-db | 0, 24 | 0 | 10 | Open PRs are largely dependency automation, not ten defects; review queue age, compatibility and resumed maintenance before inferring inactivity. |
| Knot | 27, unknown | 0 | 0 | Recount 90-day activity and check how much work is reusable signed-log logic versus surrounding app/sync code. |

These counts are **not** an apples-to-apples quality score. Closed-issue history, severity and response times were not established in the sample; mark them unknown until the pre-ADR refresh rather than inventing a rating.

## Identical fixtures for every candidate

| ID | Input | Required observation |
|---|---|---|
| LOG-01 | Same signed operation delivered twice over separate sessions | One logical append, one projection change, stable ID. |
| LOG-02 | Child operation arrives before causal parent | Child pending, parent requested; both validate and converge after repair. |
| LOG-03 | Tampered payload/signature or revoked epoch | Rejected without frontier advancement or data disclosure. |
| LOG-04 | Two equal peers partition, each writes an incompatible task status | Both branches preserved; no arrival-time/device-ID winner; conflict surfaced after reunion. |
| LOG-05 | Two peers edit disjoint document regions, then overlapping region | Disjoint edit merges; overlap follows explicit document conflict policy, retaining variants. |
| LOG-06 | Two offline claims target one exclusive resource | Neither becomes final from peer transport alone; named authority/current-frontier proof required. |
| LOG-07 | Crash between local log, outbox and frontier writes; then restart | Either fully durable operation or explicit unsaved failure; no ghost frontier or repeated external command. |
| LOG-08 | Mailbox holds ciphertext; no recipient applied it | Only “stored for delivery”; no replicated/delivered/domain-accepted receipt. |
| LOG-09 | First independent authorized durable replica applies operation | Signed application receipt advances replication evidence; same-Persona second device is not a second approval vote. |
| LOG-10 | Offline old client returns after schema/feature change | Unknown mandatory feature fails closed or affected AppInstance becomes read-only; history remains recoverable. |
| LOG-11 | Two distinct, valid concurrent resolutions cover identical conflict heads and produce identical canonical state **and** effects | One visible result; both signed operation IDs and actors remain in audit history. Predicate is a candidate for `OQ-0034`, not yet frozen. |
| LOG-12 | Concurrent resolutions show the same status but differ in covered heads, effects or schema/policy version | Do not silently coalesce; preserve explicit conflict or require a new causally covering operation. |
| LOG-13 | Conflict resolved against all known heads; an accepted, previously unknown incomparable branch arrives after long offline interval | Reopen explicit conflict; prior resolution and late candidate remain recoverable. |
| LOG-14 | Two append-only messages have identical body but distinct signed operation IDs | Keep both entries; content equality is not duplicate delivery. |
| LOG-15 | Two devices of one Persona submit approval of the same action | Count one human approver; independent replication receipts remain distinct from approval votes. |
| LOG-16 | Three configured authority replicas partition 2+1; alternate test changes membership/revokes one replica during partition | Under a quorum candidate, minority cannot finalize; no stale-epoch certificate becomes valid after reunion. This tests an option, not an approved topology. |
| LOG-17 | Old offline peer presents an operation concurrent to the start of a shallow snapshot; then a file-conflict variant is needed by another peer | Rebootstrap/reconcile before applying; retain pending candidate and unresolved blob reference; no silent overwrite or last-copy GC. Exact safe frontier remains `OQ-0033`/`OQ-0034`. |
| LOG-18 | Two accepted file resolutions yield byte-identical content but differ as same-resource revision versus separate copy, ACL inheritance, metadata or downstream effects; one actor lacks read access to a variant | Matching digest/display alone cannot coalesce different resource identities, rights or effects. Preserve both signed operations; deny a resolution that needs an unreadable variant without leaking its contents. Exact equivalence stays `OQ-0034`. |
| LOG-19 | After one resolution, two or more previously unknown accepted incomparable heads arrive in every transfer permutation; some appear equivalent, others are incompatible or cover different causal heads | Every peer derives the same active head/conflict set. No pairwise arrival order prematurely closes the conflict; the next resolution causally covers every current head. |
| LOG-20 | A stale peer needs pre-snapshot history, but the full archive/rebootstrap source or required frontier proof is unavailable; alternate run has an unresolved file blob unavailable | Fail closed without partial snapshot/frontier advancement or variant/blob GC. Keep the candidate and unresolved references recoverable or explicitly unavailable until verified recovery; never infer a safe prune from age or a shallow snapshot alone. |

Run the same fixture data with direct and relay transport, duplicate/reordered packets, restart checkpoints and a lock-resolved dependency graph. Record pass/fail/unsupported separately; do not rewrite VIDA invariants to make a candidate pass.

LOG-18/19 test VIDA's accepted [file/conflict semantics](../../../docs/03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md), not an upstream library's visible winner. [Automerge documents](https://automerge.org/docs/reference/documents/conflicts/) that one deterministic value can be displayed while other concurrent values remain in a conflicts object and a later assignment clears that object. LOG-20 tests the [Loro shallow-snapshot limitation](https://loro.dev/docs/tutorial/encoding) as a candidate hazard: updates concurrent with the snapshot start cannot be imported by that path. These sources motivate fixtures; they do not choose Automerge, Loro, a retention frontier or a VIDA equivalence predicate.

## Decision-gated semantics to prototype, not yet VIDA policy

For `OQ-0033`, compare (A) a named logical authority service, (B) a configured Space-replica quorum with intersecting certificates, and (C) optimistic multi-head history that leaves exclusive operations pending until a common authority step. A fixed favored Device is excluded by `ADR-0016`. Under a partition, neither a transport receipt nor first local write establishes globally serialized exclusive finality; the [Gilbert–Lynch result](https://www.cs.princeton.edu/courses/archive/spr22/cos418/papers/cap.pdf) explains the availability/linearizability tradeoff, while [Irokle](https://github.com/arunaengine/irokle) and [iroh-db](https://github.com/holon-technologies/iroh-db) supply useful causal/store patterns but not VIDA's domain policy. Which profile Release 1 needs is still a product/architecture decision.

For `OQ-0034`, candidate visible-result equivalence requires the same target/semantic family, exact conflict heads covered, canonical resulting state **and effects**, and schema/policy version after valid authority acceptance. This is a proposed comparison rule; the trigger/effect idempotency-key definition remains open. Same `OperationId` is deduplication; two different `OperationId`s never lose audit attribution merely because their visible state coalesces. [Automerge conflicts](https://automerge.org/docs/reference/documents/conflicts/) preserve concurrent values but choose a deterministic visible property winner; VIDA cannot inherit that winner as business truth. [Loro shallow snapshots](https://loro.dev/docs/tutorial/encoding) can discard earlier operations and cannot import updates concurrent with the snapshot start, so stale-peer rebootstrap must be proved before any pruning choice.

A snapshot is only an acceleration checkpoint, not an authority certificate. Candidate snapshot evidence should bind Space/control epoch, schema/semantic version, causal frontier digest, projection hash, operation-ID/dedupe summary, unresolved heads, receipts/effect ledger and blob references. Pending offline candidates, unresolved variants and the last recoverable blob are never pruned merely by age. Exact replica acknowledgement/rebootstrap proof and pruning eligibility depend on the unresolved `OQ-0033` authority/current-frontier topology; these fields are evaluation hypotheses, not a released wire format.
