---
review: first-sync-adversary
artifact: ../ARCHITECTURE-SPINE.md
focus: AD-19, REQ-SYNC-004, SyncLog, ADR-0003, AD-18
date: 2026-09-20
verdict: fix-one-cross-rule-ambiguity-then-keep-implementation-gated
---

# Adversarial review — first accepted competing task status

## Verdict

AD-19 captures the accepted product preference for conflicting task-status changes: a competing stale intent cannot silently overwrite a status already synchronized and accepted under the applicable authority policy. It correctly rejects transport arrival, mailbox storage, local pending state and client clocks as precedence evidence. The exact ordering/receipt/state-transition mechanism is properly blocked by `OQ-0033`/`OQ-0034`. One cross-rule ambiguity should be fixed **now** before calling the policy coherent: AD-18 permits local authority acceptance in an Owner's Personal Space, while AD-19 gives priority to the first **synchronized and accepted** competing proposal. Two Personal devices can each be locally “accepted” before either synchronizes; the text does not say whether one accepted state is provisional or whether AD-19 excludes that Space mode. Do not silently choose one interpretation.

## Independent-implementation attacks

### C1 — Two offline Personal devices can each truthfully claim local acceptance [FIX NOW]

**Evidence:** AD-18: Personal Space “can accept under its Owner policy locally without other members or server approval.” AD-19: first proposal “successfully synchronized and accepted under the applicable authority policy” has precedence. `SyncLog` distinguishes origin-durable and authority-accepted frontiers, but allows them to coincide when origin is authority.

**Construction:** Owner uses laptop L and phone P, both offline with task `status=Open` at accepted base R. L changes it to `Done`; P changes it to `Cancelled`. Each implementation treats its own Owner device as local Personal authority and marks its status authority-accepted. Months later L syncs first to a peer. Implementation A gives L precedence by network-first synchronization and demotes P's prior accepted state to conflict. Implementation B treats both prior local authority receipts as irrevocable and cannot demote either; it chooses a deterministic tie-break or forks the task. Both can cite AD-18 and AD-19, but cannot share one accepted frontier.

**Required decision:** state whether (a) Personal Space multi-device local acceptance of competing status is provisional until common authority serialization, (b) the first authoritative local commit already has a globally comparable receipt/order even offline, or (c) AD-19 applies only to Space profiles with a serializing authority. This is a product-level invariant, not merely a codec detail. Once chosen, `OQ-0033`/`OQ-0034` can define receipts and fixtures. Do not make every offline Personal edit unexpectedly pending by implication; surface that consequence for user confirmation.

### H1 — “First synchronized” still admits two incompatible definitions of success [GATED]

**Evidence:** AD-19's conjunction “successfully synchronized and accepted” excludes raw receive, yet leaves the authoritative acceptance event/actor open in `OQ-0033`. A sender can sync to a relay, mailbox, peer replica or Space authority; those events may not be ordered the same way.

**Construction:** A reaches one peer first; B reaches the Space authority first. Client C says A “synced successfully” after peer persistence and treats A as winner; node N requires an authority receipt and gives B priority. C would violate AD-19 if it equates peer persistence with acceptance, but the current text lacks the canonical receipt needed to test the distinction. A second pair can both wait for receipts from different authority replicas and still disagree on which receipt is globally first.

**Closure:** `OQ-0033` must define one authority ordering domain, receipt signer/sequence/frontier, quorum behavior and failure/retry semantics. `OQ-0034` must bind task-status transition validation to that order. Add race fixtures with reordered direct/mailbox delivery and replicated authority failover.

### H2 — Stale intent is protected from overwrite, but “visible” must respect later revocation [FIX NOW]

**Evidence:** AD-19 says the losing proposal remains “recoverable and visible as non-accepted/conflicted.” ADR-0003 §4/§6 says a fully removed member must not see protected Space content through quarantine/recovery UI and a compliant client must hide managed Space data after receiving effective removal.

**Construction:** A former member's offline `Done` loses to accepted `Cancelled`, then authority removes that member. Client C obeys AD-19 literally by showing the conflict with task title/previous status; client N obeys ADR-0003 by hiding it. C leaks revoked Space content even though the proposal itself remains recoverable in encrypted local storage.

**Disposition — autofix:** qualify AD-19's visibility: the stale intent is recoverable but shown only through an actor-authorized recovery surface; after effective removal, no protected Space content appears. This does not choose the conflict algorithm.

### H3 — Retry/dedupe versus a genuinely competing status needs a logical-intent identity [GATED]

**Evidence:** AD-4 keeps one stable operation ID across direct/durable retry; AD-19 compares competing proposals; `SyncLog.Accept` is idempotent. `OQ-0028` leaves operation-ID derivation/envelope open, and `OQ-0033`/`OQ-0034` leave stale-intent rebase/re-sign open.

**Construction:** client C resends `Done` via direct and mailbox; node N sees two envelopes and treats the second as a competing status because they were re-signed after key rotation. Conversely, two distinct devices send `Done` and `Cancelled` from base R with a deterministic ID derived only from task/base, causing N to dedupe a real conflict. A rebase that reuses the losing ID may be ignored forever; a rebase with a new ID may create an unintended second business command.

**Closure:** define immutable candidate/intent identity, operation revision or replacement linkage, idempotency key scope, signature domain, and explicit user-originated new transition versus automatic retry. Golden vectors must cover identical retry, distinct competing intents, old-epoch re-sign and new-base intentional update.

### H4 — “Same prior accepted base” and “mutually exclusive” must be machine-checkable [GATED]

**Evidence:** AD-19 scopes the rule to concurrent mutually exclusive task-status proposals sharing a prior accepted base, but a canonical base revision/frontier, transition graph and concurrency relation are deferred to `OQ-0034`.

**Construction:** client C treats `Done`/`Cancelled` as mutually exclusive and compares full task revision R. node N compares only status-field revision, or considers `Done→Cancelled` a valid sequential transition after accepting A. Both implement the English rule but disagree whether B is stale or a legitimate next transition.

**Closure:** specify a versioned status operation family: base-state identity, causal comparison, legal transitions, authority-time validation, stale/recoverable result code, and a separate explicit command for an intentional transition from the new accepted state. Cross-client replay must produce one accepted status and one preserved conflicting intent.

## Fix-now versus deferred summary

| Scope | Finding | Action |
|---|---|---|
| Fix now | AD-18 Personal local authority acceptance conflicts with AD-19 first synchronized acceptance for concurrent offline Personal devices | Ask/select product semantics; then align both AD rules |
| Fix now | “Visible” stale proposal can conflict with ADR-0003 post-revocation hiding | Add authorization qualifier; preserve encrypted/recoverable candidate |
| `OQ-0033` | Authority order/receipt, serialized acceptance, failover and Personal authority implementation | Keep independent acceptance blocked |
| `OQ-0034` | Status base/concurrency/transition/conflict/rebase fixtures | Keep independent `SyncLog` blocked |
| `OQ-0028` + `OQ-0033/34` | Operation ID, signature, retry and replacement linkage | Cross-path/idempotency conformance before claim |

## Stack-brief consistency

`docs/03-architecture/stack-selection-brief.md` accurately repeats `REQ-SYNC-004`, keeps receipt/order in `OQ-0033`/`OQ-0034`, and rejects CRDT winner selection as a substitute for authority acceptance. No material stack-brief mismatch found. Its `STACK-G2` remains a test gate, not evidence that the receipt/order contract already exists.

## Acceptance fixtures required at gate closure

1. A and B from the same accepted status base; B arrives at a relay first, A gets the first valid authority receipt: A wins.
2. A reaches one authority replica first, B reaches another; only one globally valid ordering survives failover/replay.
3. Two offline Personal devices each create a candidate from base R; the chosen Personal authority model yields one verifiable outcome and honest local UI before convergence.
4. Losing actor is revoked before reconnect; conflict is preserved securely but not displayed via managed Space UI.
5. Same logical A arrives via direct/mailbox/retry/key-rotation; one apply. B is a distinct competing intent and is not deduped away.
6. After A is accepted, actor explicitly submits a new transition based on A's accepted state; it is evaluated as new intent, not the old stale B silently replayed.
