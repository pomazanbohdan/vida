---
review: recursive-resolution-adversarial
artifact: ../ARCHITECTURE-SPINE.md
focus: AD-22, ADR-0017, REQ-SYNC-009, SyncLog, BlobStore, authorization
date: 2026-09-20
verdict: recursive-rule-direction-sound-one-cross-document-conflict-and-four-gated-risks
---

# Adversarial review — recursive status/file resolution

## Verdict

The amendment correctly refuses to crown either independently accepted offline resolution by device or arrival order. **One normative scope conflict needs fixing now:** AD-22 mandates a new conflict for *distinct operations*, while ADR-0017 limits that rule to resolutions with *different outcomes* and explicitly defers semantically identical ones. Authorization of variant viewing, late-branch retention and effect eligibility remain unsafe to infer from the recursive rule; keep `OQ-0033/0034`, blob and storage gates closed. No production-conformance claim is justified yet (spine lines 224, 249, 252–254; `sync-log-contract.md` line 80).

## Findings

### H1 — Same result, different resolution IDs: conflict or no conflict? [fix now]

Conflict `{A,B}` is visible to two disconnected, authorized actors. R1 and R2 independently choose A, each as a new, valid operation covering both heads; their authority acceptances are incomparable. AD-22 says **two distinct** resolution operations create a new conflict (spine line 224). ADR-0017 says two actors resolve **differently** and leaves semantically identical resolutions open (ADR lines 28, 34); `REQ-SYNC-009` says “two different resolutions,” which can mean distinct IDs or different values (requirements line 52). Peer X recurses to `{R1,R2}`; peer Y coalesces the same result. Both avoid a device winner, but their projections and future resolution preconditions diverge.

**Disposition:** align the three normative texts on the trigger. Either explicitly adopt operation-identity conflict even for equal results, or keep equal-result behavior undecided and narrow AD-22; do not let two implementations guess semantic equivalence. Test equal value with different metadata/side-effect intent as well as different values.

### H2 — “Authorized peer” need not have read access to both variants [security; clarify before UI lock]

The permission matrix separates `read` from `update` (`access-control-requirements.md` lines 32–41). Actor C may be allowed to update a task/file in one scope but denied read/export of a protected prior version. ADR-0017 says *all authorized peers* show both variants (line 25), while AD-22 requires a currently authorized actor to resolve (spine line 224). If a client treats update/resolve permission as permission to fetch and render both file payloads, conflict UI or a plugin/API leaks the protected branch. AD-19 already qualifies stale-proposal visibility by authorization *for that view* (spine line 206); BlobStore tickets must not bypass Space authorization (`blob-store-contract.md` lines 41, 54).

**Disposition:** define conflict-metadata visibility, per-variant read/fetch/export rights and `resolve` capability separately. A write-authorized actor unable to read a variant must not gain its plaintext merely to resolve; specify whether resolution is blocked, blind-reference-only, or delegated. After effective removal, neither old nor recursive conflict payload appears in managed UI/API. Add role/ACL and revocation fixtures.

### H3 — Local resolution may unpin a branch needed by a later recursive conflict [data-loss gate]

Peers hold accepted file variants A/B. P accepts R1 selecting A and interprets “pin until authorized resolution” as permission to unpin/GC B. Offline Q has independently accepted R2 selecting B. When R2 arrives, the rule requires a new `{R1,R2}` conflict and both resolution payloads recoverable (spine line 224; `REQ-SYNC-009` line 52; `blob-store-contract.md` lines 56–57), but P has already collected B; Q may be unavailable or itself need repair. A local accepted resolution is not proof that no incomparable resolution head remains in another partition. Log convergence alone therefore does not prove payload recoverability.

**Gate closure:** define a safe release frontier for losing-branch pins, transitive ownership of reused blobs by resolution operations, durable operation↔manifest↔payload/pin recovery after crash, and an honest unavailable-payload UI while repair is pending. Test R1/rekey/GC/restart, then late R2 and offline Q; do not promise physical erasure or retain access after revocation.

### H4 — Recursive acceptance must revalidate control state, not just reference heads [security gate]

R1 and R2 each cite `{A,B}` while partitioned. An Owner revokes R2's actor or DeviceGrant before its *applicable* authority acceptance cut. Treating its local signed/durable receipt as accepted creates a false recursive conflict and may expose R2's payload; rejecting it leaves only R1 plus a recoverable invalid candidate. AD-18 requires current rights/epoch at actual acceptance (spine line 200); `SyncLog.Accept` is only local history, not `authority.accepted` (`sync-log-contract.md` lines 42–44). The amendment's premise “both authority-accepted” is safe only once `OQ-0031/0033` supplies a shared proof/cut.

**Gate closure:** test R2 accepted before cut, R2 merely durable before cut, and R2 invalid after cut; compare authorization and accepted-head sets after reordered control/data delivery. Never use the stale grant, device ID, client timestamp or delivery ACK as proof.

### M1 — Two legitimate resolutions can each trigger an external effect before conflict [effect gate]

R1 chooses `Done`, R2 chooses `Cancelled`; both may be locally authority-accepted offline. Their later recursive conflict cannot undo an email, booking or other external action already executed from each branch. The rule that `sync.apply` does not replay the *original* command (spine line 224; ADR-0017 line 27) prevents duplicate replay, not two distinct branch effects. ADR-0012 separately requires a controlled post-commit executor/idempotency contract (`ADR-0012-command-event-sync-boundary.md`, Decision and Consequences); that contract is still open.

**Gate closure:** operation-family policy must say when a resolution fact is effect-eligible, how effects are deduped or compensated across incomparable accepted branches, and what provisional UI may claim. Include partition/rejoin and repeated recursive-resolution fixtures.

## Acceptance boundary

Keep the accepted product decision—different, valid, incomparable outcomes produce a new no-winner conflict—but do not infer finality from a local resolution. Before independent implementations, prove N-way/late-third-head behavior, transitive causal coverage through snapshots/pruning, per-variant authorization, payload retention/repair and effect semantics under cross-vendor replay (`OQ-0033/0034`, `OQ-0029/0036`).
