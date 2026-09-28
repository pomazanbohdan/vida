# Independent good-spine review — AD-19 / AD-21 peer equality

## Gate verdict

**Pass for the peer-equality decision, conditional on source reconciliation.** AD-19 correctly limits “first accepted” precedence to a verifiable common order; AD-21 forbids a preferred device and leaves incomparable concurrency to `OQ-0033`/`OQ-0034`. No reviewer-selected tie-breaker is warranted. The spine as a whole has one high-priority stale source conflict unrelated to the peer-equality choice.

## High finding

### H1 — AD-16 still treats the shared-read interval as undecided

- **Evidence:** [spine AD-16](../ARCHITECTURE-SPINE.md) says the numeric interval and offline-expiry behavior remain `OQ-0051`; the same claim recurs in Implementation Gates and Deferred. [Open questions](../../../../../docs/00-governance/open-questions.md) marks `OQ-0051` resolved at **7 days**, with shared read blocked after expiry. The approved [SpaceMembership contract](../../../../../docs/04-specifications/space-membership-contract.md) and [ADR-0003](../../../../../docs/03-architecture/decisions/ADR-0003-offline-revocation.md) require that behavior.
- **Divergence:** a client team following the spine could leave shared reads available indefinitely or choose its own expiry, while another implements the approved seven-day lock.
- **Disposition:** **Autofix in the spine:** bind the approved seven-day value and post-expiry shared-read lock in AD-16; remove `OQ-0051` as an implementation blocker. Preserve `OQ-0053` for freshness/anti-rollback proof and `OQ-0063` for the unresolved shared-Owner exception. Do not change the indefinite *mutation-candidate* lifetime in AD-18.

## Medium findings

### M1 — AD-21's “identical valid operation set” needs an acceptance boundary

- **Evidence:** [AD-21](../ARCHITECTURE-SPINE.md) promises identical state/conflict variants for an identical valid operation set. [SyncLog](../../../../../docs/04-specifications/sync-log-contract.md) distinguishes locally validated/pending history from `authority.accepted`, and AD-4/AD-13 distinguish origin-durable and authority-accepted frontiers. A domain operation's eligibility also depends on signed control history, grants and epochs.
- **Divergence:** two teams could count different operations as “valid” or compare domain operations without the same control frontier, then both claim peer convergence.
- **Disposition:** **Clarify, not decide concurrency:** define the convergence fixture over identical validated control+domain history, applicable contract version/policy and acceptance evidence; compare accepted state separately from pending/conflict variants. `OQ-0033`/`OQ-0034` still own proof and concurrent-resolution semantics.

### M2 — Offline peer authoring must not imply final acceptance

- **Evidence:** AD-21 says every authorized device may durably author/exchange operations “under current grants,” whereas AD-18 allows offline candidates to remain pending and requires current-grant revalidation at the actual authority-acceptance point.
- **Divergence:** a shell could present an offline locally durable operation as finally accepted because the device had an old grant when it authored it.
- **Disposition:** **Clarify AD-21:** equal peers may durably author *candidates*; no device's local commit, endpoint or first ACK supplies global acceptance. Existing AD-4/5/18 gates remain binding.

### M3 — Regression fixture and source traceability are incomplete

- **Evidence:** [ADR-0016](../../../../../docs/03-architecture/decisions/ADR-0016-equal-device-peers.md) requires partitioned peer groups, delivery-order permutations and phone/laptop swaps, but it is absent from the spine's `sources` list. AD-21 states the invariant without an explicit peer-equality fixture in Implementation Gates.
- **Disposition:** **Autofix:** add ADR-0016 as a source and require the concrete peer-swap/partition/rejoin fixture under the `OQ-0033`/`OQ-0034` sync gate. Keep the concurrent schema-level rule open.

### M4 — Structural seed contradicts accepted mobile toolkit

- **Evidence:** the spine's `shells/ios` and `shells/android` comments say “toolkit open,” while AD-20 and accepted [ADR-0015](../../../../../docs/03-architecture/decisions/ADR-0015-v1-device-platforms-and-mobile-flutter.md) bind Flutter for both; only Windows toolkit is open.
- **Disposition:** **Autofix:** label both mobile presentation shells Flutter; retain Windows `OQ-0008`.

## Checklist

| Criterion | Result |
|---|---|
| Real divergence points and enforceable rules | **Pass with M1/M2 clarification:** no device rank; comparable vs incomparable acceptance is separated. |
| Deferred gaps | **Pass with gates:** `OQ-0033`/`OQ-0034` block domain acceptance and independent sync; do not silently select a winner. |
| Source consistency / brownfield ratification | **Conditional:** H1 and M4 contradict approved sources; M3 weakens traceability. |
| Capability coverage | **Pass for peer parity:** client, core, runtime, SyncLog, membership and conformance are bound. |
| Operational/environmental envelope | **Pass:** AD-15 and rollout gates cover owners, environments and recovery; no peer-specific new deployment decision is required. |
| Named technology currentness | **Not assessed in this semantic lens;** version-fit reviewer should verify Iroh 1.2. |

No critical peer-equality defect found. The unresolved concurrent operation-family rule is a deliberate gated decision, not a license for first-arrival, client-clock or device-priority behavior.
