# Independent rubric and input reconciliation — AD-19 first accepted task status

## Gate verdict

**Pass as a product-level invariant with implementation gates; do not treat it as an implemented conflict algorithm.** AD-19 gives the user's “Перша вдала синхронізація має пріоритет” a security-consistent interpretation for competing **task-status** transitions: the first valid authority-accepted synchronization wins, not the first packet, local write, mailbox receipt, or client timestamp. `OQ-0033`/`OQ-0034` correctly block implementation until a shared ordering and receipt contract exists. One interpretation should be explicitly confirmed if the prior user discussion did not already limit the answer to task status.

## Critical findings

None in the reviewed contour.

## High findings

### H1 — “Successful synchronization” is interpreted as authority acceptance, not mere arrival

- **Evidence:** user statement is “Перша вдала сінхронізація має пріоритет”. AD-19 requires “successfully synchronized **and accepted under the applicable authority policy**” and excludes `delivery.accepted`, mailbox storage, raw receive and local pending state. This matches AD-4's separate origin-durable and authority-accepted frontiers, AD-5's delivery/authority separation, ADR-0012's committed-fact boundary, and `REQ-SYNC-004`.
- **Assessment:** this is the only safe interpretation if task state must converge under current rights and business preconditions. A network packet arriving first may be invalid or stale. The rule therefore **ratifies**, rather than weakens, AD-4/5.
- **Disposition:** **Confirm if not already confirmed.** The user should understand that an earlier transport arrival can lose to a later valid authority acceptance if it fails authorization/preconditions. No change is needed if that was the intended meaning of “вдала”.

### H2 — Scope is task-status transitions, not an all-data first-writer-wins policy

- **Evidence:** AD-19 binds the task-status operation family and concurrent mutually exclusive proposals from the same accepted base; it explicitly excludes rich text, independent fields, messages and bookings. `REQ-SYNC-004` has the same scope. The user sentence alone does not name the data type, though the surrounding contour appears to concern task status.
- **Assessment:** narrow scope is architecturally sound. Applying “first sync wins” to messages, document text, counters or booking inventory would cause loss or double-booking and would contradict the per-operation-family policy boundary in AD-4/13.
- **Disposition:** **Input reconciliation.** If the user's answer was to a task-status example, pass. If they intended all concurrent changes, ask for one explicit clarification rather than broadening AD-19 silently.

## Medium findings

### M1 — Autonomous/multi-authority ordering is not yet implementable

- **Evidence:** AD-4 allows the origin transaction to be authority when policy says so. AD-18 allows local Personal-Space Owner acceptance. Two disconnected devices could each locally accept a proposal from the same task base. AD-19 cannot identify a globally “first” accepted proposal in that condition without an agreed sequencing/merge/receipt rule.
- **Disposition:** **Safely gated, not a new product decision.** Spine implementation gates block domain mutation acceptance on `OQ-0033` and independent `SyncLog` on `OQ-0034`; AD-19 itself requires authoritative ordering, receipt, stale handling and fixtures. Ensure those questions explicitly cover disconnected Personal Space devices, federated authority failover and two peers racing over direct and mailbox paths. No implementation may claim AD-19 conformance before that proof.

### M2 — The non-winning intention needs a concrete recovery contract

- **Evidence:** AD-19 and `SPEC-SYNC-LOG-001` require the stale proposal to remain visible/recoverable and not silently overwrite; AD-18 forbids age-only candidate discard. They defer exact rebase and repeat-submission to `OQ-0033`/`OQ-0034`.
- **Disposition:** **Safely gated.** Conformance fixtures should prove that a losing proposal retains author, original intended status and reason, does not become accepted or trigger post-commit effects, and can lead only to a new deliberate transition from the accepted state. No auto-rebase that silently changes business intent.

### M3 — “First” must exclude invalid proposals and retries of the same operation

- **Evidence:** AD-4 requires stable operation ID and dedupe; AD-18 requires current rights and preconditions at acceptance; AD-19 says first *accepted* proposal wins.
- **Disposition:** **Safely covered conceptually.** Add fixtures under `OQ-0033`/`OQ-0034` for invalid-first/valid-second, duplicate direct+mailbox delivery, accepted receipt loss/retry, and a valid later intentional transition. Client timestamp and transport arrival must have no priority effect.

## Good-spine checklist

| Criterion | Result | Note |
|---|---|---|
| Fixes real divergence | **Pass** | Independent clients cannot silently choose last-writer-wins or client-clock priority for competing task statuses. |
| Enforceable Rule prevents its divergence | **Pass with gate** | Observable winner/loser behavior is clear; authority order and receipt fixtures are deferred but explicitly block implementation. |
| No deferred implementation divergence | **Pass with gate** | `OQ-0033`/`OQ-0034` block acceptance/sync implementations; their fixtures must cover multi-device and failover. |
| Ratifies inherited AD-4/5/18 | **Pass** | Origin/delivery ACK is not authority; old candidates are retained but may lose after valid acceptance. |
| Scope matches input | **Conditional** | Correct if user was choosing task-status precedence; not a universal conflict strategy. |
| Operational/environmental dimension | **No new issue** | Authority failover and receipt recovery belong to the already open authority/sync gates. |

## Input reconciliation

- **User's priority:** first *successful* synchronization wins — represented as first valid authority acceptance, with earlier packet/local timestamp insufficient.
- **User's unlimited-age offline candidate:** preserved. An old proposal is not rejected for age; it can lose because its base/preconditions became stale after another valid task-status transition.
- **Task conflict UX:** losing intent remains inspectable, not silently overwritten; a fresh deliberate transition from the accepted result is allowed.
- **No global conflict rule:** AD-19 does not settle rich text, independent fields, messages or bookings.

## Recommended handoff

Keep AD-19. Before implementation, make `OQ-0033`/`OQ-0034` fixtures explicit for disconnected Personal-Space devices, multi-authority/failover races, invalid-first/valid-second, duplicate delivery, receipt loss and a later intentional transition. Confirm user scope/meaning only if the surrounding conversation did not already establish task status and authority-accepted success.
