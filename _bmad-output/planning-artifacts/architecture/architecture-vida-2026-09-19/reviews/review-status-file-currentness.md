# AD-22 / ADR-0017 — status and file conflict currentness review

Reviewed: 2026-09-20. Scope: `ARCHITECTURE-SPINE.md:218-222`, `ADR-0017-concurrent-status-file-conflict.md:18-34`, and the cited Automerge, Loro and OWASP primary sources. This is an evidence/fit check, not a library selection or implementation review.

## Verdict

**Conditional pass.** The approved rule—no canonical current winner for two causally incomparable incompatible task-status/file-replacement operations, with an explicit authorized resolution—is a VIDA policy, not an asserted feature of any named CRDT. Its cited sources are current and compatible as *patterns*. The implementation must not project library defaults as the VIDA business state; two adapter/conformance risks remain below.

## Findings

### P2 — A generic Automerge assignment can erase its exposed conflict without a VIDA resolution

The [Automerge conflict reference](https://automerge.org/docs/reference/documents/conflicts/) confirms that same-property concurrent values are available via `getConflicts`, while the normal property read returns a deterministic visible winner. It also states that the **next assignment to that property considers the conflict resolved and removes it from `getConflicts`**. Thus `getConflicts` alone is not a durable VIDA conflict ledger or proof that the authorized resolution operation named both branches. If Automerge is prototyped, keep VIDA variant IDs/branch evidence in validated history, gate all writes to the affected status/file projection on the new authorized resolution operation, and add a negative fixture: a generic write or replay must not clear the unresolved domain conflict. `ADR-0017:25-26` already forbids the library winner as current and requires a new causal resolution; this is a concrete adapter-fit gate, not a contradiction in the ADR.

### P2 — Loro sync convergence does not supply a multi-value register for these fields

The cited [Loro sync reference](https://www.loro.dev/docs/tutorial/sync) supports delivery-order/duplicate-independent convergence for the same updates. It does **not** establish that a map field retains both visible business alternatives. [Loro Map](https://www.loro.dev/docs/tutorial/map) uses Lamport LWW for conflicting keys; its example makes the larger peer ID the visible winner, while the sync guide warns that a concurrently created regular child container under one map key may be hidden by that rule. If Loro is evaluated for task status or file pointers, require a separate explicit multi-value variant model and prove both alternatives survive import, snapshot/rebuild and client projection. Cite the Map behavior alongside Sync in the fit/prototype notes; never treat `toJSON()`'s single map value as the approved current status/file.

## Source and authorization boundary

- [OWASP Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) supports deny-by-default and permission checks for every new action, including object-specific checks. Applying that rule at VIDA's **authority-acceptance** point, rather than trusting a stale client grant or UI check, is a VIDA design inference consistent with `AD-18` and `ADR-0017:26,32`; OWASP does not define distributed acceptance proof, causal conflict resolution, retention or who may view old variants.
- `AD-22` and ADR-0017 correctly keep the unresolved conflict distinct from `AD-19`'s first acceptance where a shared verifiable order exists. They also leave simultaneous resolution races, capability mapping, wire form and provisional UX in `OQ-0033`/`OQ-0034`; the cited libraries cannot close those decisions.
- No Automerge or Loro release version is selected here, so this check makes no claim that a particular release/adapter already passes VIDA's conformance fixtures. Pin and verify exact versions if a prototype or release binds either library.
