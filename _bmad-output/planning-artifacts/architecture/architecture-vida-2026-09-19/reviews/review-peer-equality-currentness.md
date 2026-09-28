# AD-21 peer equality — currentness and evidence review

Reviewed: 2026-09-20. Scope: the new peer-equality rule in `ARCHITECTURE-SPINE.md:210-214`, its `AD-19` dependency and implementation gate, plus the named references in ADR-0016 and the device-sync-session draft. This is not a review of the older spine or a library selection.

## Verdict

**Conditional pass.** The peer-equality invariant is consistent with the user's decision and the cited CRDT/authorization sources when they are treated as patterns, not VIDA authority implementations. No selected Automerge, Loro or Irokle version is claimed. Three wording/fit risks should be closed before implementation; `OQ-0033`/`OQ-0034` correctly remain gates.

## Findings

### P1 — Specify the evidence set for convergence, not only the operation set

`AD-21` says an identical **valid operation set** yields identical state/conflicts, while it also says devices can author under **current grants**. An offline device cannot prove that its locally known grant is still current. `AD-18`, `SPEC-SYNC-LOG-001` and `OQ-0033` already require revalidation at authority acceptance, but `AD-21` by itself can be read as author-time validity or as granting every signed operation final status. Two peers holding the same payload operations but different signed control/acceptance evidence could then classify them differently. Clarify that offline authorship creates origin-durable candidates under locally known grants; final convergence is defined over the same operations **and** the same verifiable control/authority-acceptance evidence, with pending/rejected variants kept distinguishable. Keep the existing `OQ-0033` proof gate. This is an architectural precision issue, not a reason to designate an arbiter device. [OWASP Authorization](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) supports per-action authorization/default deny, but does not supply a distributed acceptance proof; that latter conclusion is a VIDA design inference.

### P2 — Default CRDT winners can accidentally encode device priority

`AD-21` correctly says replica IDs are for causality/deduplication, not business rank. Yet [Loro Map](https://www.loro.dev/docs/tutorial/map) uses Lamport LWW for a conflicting map key and its example chooses the larger peer ID in a tie. [Automerge conflicts](https://automerge.org/docs/reference/documents/conflicts/) exposes all concurrent scalar values but picks a deterministic visible winner from internal operation IDs, with actor ID breaking counter ties. Thus default `LoroMap`/Automerge scalar projection is **not** evidence for VIDA's “first authority-accepted” status/file choice, and using it as final business state can make a replica identifier the de facto priority. Require operation-family fixtures that permute peer IDs as well as delivery/platform order, preserve authorized alternatives, and prove the selected `OQ-0034` resolver rather than a library default. The existing device-sync-session draft already warns that Automerge's winner is not a VIDA receipt.

### P2 — Irokle is a history/repair reference, not a Space authority adapter

The [Irokle README](https://github.com/arunaengine/irokle) supports the cited signed Merkle-DAG, membership operations, bounded causal sync and branch-bound ACK claims. It also says membership is evaluated at an operation's DAG position and identifies topic members as peer IDs. VIDA's Persona/Device separation and revalidation under current Space grants at the actual authority-acceptance point are different semantics; an Irokle ACK or topic-membership check cannot be promoted to `authority.accepted`. The device-sync-session draft already labels it a candidate for history/repair and denies ACK-as-receipt. Add an explicit adapter/prototype assertion if Irokle is evaluated: preserve VIDA's separate membership/authority proof and test revoke-vs-offline-operation races. No current production-fit endorsement follows from the repository example alone.

## Source check and boundaries

- [Automerge conflicts](https://automerge.org/docs/reference/documents/conflicts/): concurrent scalar alternatives are retained; its visible winner is deterministic, not a VIDA authority decision.
- [Loro sync](https://www.loro.dev/docs/tutorial/sync): equal update sets converge regardless of order/duplicates; this does not cover business authorization. [Loro PeerID Management](https://www.loro.dev/docs/concepts/peerid_management) explicitly warns against using a user ID for multiple devices or reusing a peer ID without its persisted state; this is a more precise citation than the generic JS API linked in `device-sync-session.md:49`.
- [OWASP Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html): deny by default and validate permissions on every request support the recheck boundary, but do not specify peer ordering, conflict resolution or finality.
- The spine's `AD-19` and `AD-21` correctly reject packet arrival, client time and an arbitrary device as a global order; `OQ-0033`/`OQ-0034` remain unresolved in `open-questions.md:43-44` and are appropriately implementation-blocking.
