# Independent good-spine review — AD-22 status/file conflict

## Gate verdict

**Conditional pass.** AD-22 ratifies the approved explicit multi-value, no-winner outcome for incomparable task-status and file-replacement changes, preserves comparable first acceptance, and leaves proof/wire/races open. Two high-priority seam gaps could still let compliant subteams disagree on which variants enter the conflict and whether both file payloads remain recoverable.

## High findings

### H1 — “Durable alternatives” is not an authorization/acceptance criterion

- **Evidence:** [AD-22](../ARCHITECTURE-SPINE.md) says two causally incomparable incompatible changes form one conflict when reconciled, but does not qualify each branch against applicable authority policy. AD-18 requires current rights/grant/epoch/business-precondition revalidation and keeps revoked/invalid candidates recoverable **without** auto-merge. [ADR-0003](../../../../../docs/03-architecture/decisions/ADR-0003-offline-revocation.md) quarantines an unaccepted operation after effective revocation; [SyncLog](../../../../../docs/04-specifications/sync-log-contract.md) distinguishes validated/pending from `authority.accepted`.
- **Divergence:** one implementation could elevate a revoked actor's locally durable candidate into the shared unresolved conflict and display it as a valid alternative; another could quarantine it. This affects authorization and accepted projections, not merely UX.
- **Disposition:** **Clarify AD-22 and ADR-0017:** only alternatives that satisfy the eventual `OQ-0033` authority/acceptance contract can form an accepted shared conflict. Rejected or insufficiently proven candidates stay separately recoverable/pending under current access rules and never become an accepted variant by being received or stored. Do not pre-decide the authority proof; keep `OQ-0033` as a blocking gate. Add revoked-vs-valid and missing-proof fixtures.

### H2 — File conflict durability must bind BlobStore retention as well as SyncLog

- **Evidence:** AD-22's **Binds** omits `BlobStore`; its Rule requires both “durable alternatives” but does not say the two file payloads/manifests and pins survive. Approved [BlobStore contract](../../../../../docs/04-specifications/blob-store-contract.md) explicitly forbids losing either concurrent replacement and requires both verified payload variants/pins until authorized resolution under retention policy. AD-7 separates large bytes from operations.
- **Divergence:** a SyncLog team could preserve two operation references while a BlobStore/GC team deletes the non-current blob after the first local acceptance. The UI would show a conflict whose second file cannot be inspected or recovered.
- **Disposition:** **Autofix the spine:** bind `BlobStore` in AD-22 and state that conflict references and both underlying file variants remain protected under the approved retention/rights policy; blob availability/transfer is separate from domain acceptance. Add a crash/restart/GC fixture. Exact pin/retention mechanics remain `OQ-0029`/`OQ-0034`.

## Medium findings

### M1 — The accepted source is absent from the spine's source registry

- **Evidence:** spine frontmatter `sources` ends at ADR-0016; AD-22 cites no ADR-0017. The accepted [ADR-0017](../../../../../docs/03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md) is the direct decision source, and the BlobStore contract is a material companion for file replacement.
- **Disposition:** **Autofix:** add ADR-0017 to `sources`; include the BlobStore contract in `companions` or the AD-22 binding map. This avoids the next reviewer treating the new policy as an unsupported spine invention.

### M2 — The gate does not explicitly test AD-22's newly fixed outcome

- **Evidence:** spine Implementation Gates block `OQ-0033` and `OQ-0034` generally, but do not name the required no-winner status/file fixture. [ADR-0017](../../../../../docs/03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md), [REQ-SYNC-004/007](../../../../../docs/02-requirements/transport-sync-requirements.md), [SyncLog](../../../../../docs/04-specifications/sync-log-contract.md), and the [device-sync session](../../../../../docs/04-specifications/device-sync-session.md) already require order/platform permutations and both variants after partition/rejoin.
- **Disposition:** **Autofix:** make the `OQ-0033`/`OQ-0034` gate require the approved no-winner, two-variant, currently authorized resolution fixture for status and file replacement; keep simultaneous resolution races, proof and wire/capability undecided.

## Good-spine checklist

| Criterion | Result |
|---|---|
| Real divergence fixed | **Pass:** no CRDT/ID/arrival winner for these two operation families. |
| Enforceable Rule prevents divergence | **Conditional:** H1's eligibility and H2's payload-retention seam need explicit wording. |
| Deferred cannot silently diverge | **Pass with gate strengthening:** `OQ-0033`/`OQ-0034` remain honest blockers; no concurrent race algorithm is smuggled in. |
| Source and capability consistency | **Conditional:** ADR-0017/requirements agree on no winner, but H2/M1 omit BlobStore/source linkage. |
| Inherited invariants | **Pass:** AD-19 comparable first acceptance, AD-21 device parity, AD-18 candidate retention, AD-7 blob separation remain compatible if H1/H2 are clarified. |
| Operational envelope | **No new issue:** existing provider/recovery/rollout gates still apply. |
| Named tech currentness | **Not assessed:** AD-22 names no new mandatory technology. |

Do not replace the explicit conflict with a deterministic visible winner or infer final acceptance from `delivery.accepted`, packet arrival, client time, replica ID or a designated device.
