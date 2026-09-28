# OQ-0033 authority options and conformance cases — draft

The options are **comparisons, not a selected protocol**. The accepted [Operation Finality contract](../../../docs/04-specifications/operation-finality-contract.md) already requires an exact signed authority outcome for exclusive claims. This companion asks how a serverless Space obtains one shared order without privileging a Device.

## Business example

Two authorized people separately request the last appointment at 14:00 while their Devices cannot communicate. Each may see **saved locally**; a subsequent replication receipt may add **synchronized**. Neither label means **booking confirmed**. Only the calendar's named authority, or the Space authority chosen for a generic exclusive resource, may accept one exact request and reject or leave the other pending. A later correction is a new authorized action, not retroactive reinterpretation of transport order.

In a serverless three-replica Space split `2+1`, a quorum option could let the connected pair confirm while the isolated replica remains pending. A wait-for-reconnection option keeps both sides pending. These are business availability choices, not different interpretations of an Iroh ACK; neither has been selected. Two Devices belonging to one Persona are not two approval votes.

## Mechanisms to compare

| Candidate | Partition behavior | Benefit and cost | Decision state |
|---|---|---|---|
| Named continuously available authority service | Isolated clients retain candidates; reachable service serializes acceptance. Service outage delays confirmation. | Simple shared order; optional deployment cannot become mandatory for autonomous Personas. | Not selected. |
| Intersecting Space-replica quorum | Only a group with a valid current quorum certificate can accept; minority stays pending. | Can confirm without a central service while a quorum is reachable; membership, recovery, epoch and fault model become protocol-critical. Replica votes are not Owner-approval votes. | Not selected or proved. |
| Wait for a common authority step after peer reconnection | All disconnected exclusive requests remain pending until a joint serial order can be established. | Minimal false-confirmation risk; no exclusive confirmation during partition. Other mergeable work continues offline. | Not selected as universal default. |

A permanent favored phone/laptop is excluded by [ADR-0016](../../../docs/03-architecture/decisions/ADR-0016-equal-device-peers.md). Optimistic multi-value merge remains useful for [status/file conflicts](../../../docs/03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md), but cannot silently confirm two claims on one scarce resource. This availability-versus-single-order tradeoff is an inference from the [Gilbert–Lynch partition result](https://www.cs.princeton.edu/courses/archive/spr22/cos418/papers/cap.pdf), not a limitation peculiar to Iroh. [Automerge conflict behavior](https://automerge.org/docs/reference/documents/conflicts/) shows why a deterministic internal winner is not a VIDA business decision. [Iroh's relay description](https://www.iroh.computer/blog/shared-relays) describes packet forwarding without retained data; relay presence is not authority.

The [Iroh 1.2 Connection API](https://docs.rs/iroh/latest/iroh/endpoint/struct.Connection.html) explicitly distinguishes QUIC-stack receipt from remote application delivery; even transport acknowledgement cannot prove resource acceptance. The [current etcd failure guide](https://etcd.io/docs/v3.7/op-guide/failures/) illustrates the quorum trade-off: a majority can continue while a disconnected minority cannot complete writes. This is a reference pattern, not an etcd dependency or a decision to require a server. [OWASP Authorization guidance](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) supports validating authority at each action; it does not prescribe VIDA's serverless ordering or quorum membership.

For a process configured as `2-of-3`, the three authorized approvers satisfy the `ConfirmationPolicy`; they do not each become Resource Authority or determine packet order. The named authority issues one bounded terminal outcome only after the configured conditions and current rights/preconditions validate. An Admin can change supported approval conditions per process, but cannot appoint a transport relay as authority or change Core's acceptance rules.

## Candidate proof fields, not a wire profile

An independently checkable outcome would need to bind exact `RequestId`, revision, candidate `OperationId`, Resource/operation family, authority identity/capability, current Space/control epoch, policy version, accepted causal frontier and signed decision. A quorum variant additionally needs member-set/epoch and intersecting signatures or equivalent certificate. OQ-0028 chooses actual bytes and signature domain; this table does not.

## Candidate fixtures — not executed

| ID | Scenario | Required observable result |
|---|---|---|
| AUTH-F01 | Two partitioned peer groups submit incompatible exclusive claims from one frontier. | Both candidates remain durable; neither local commit nor Iroh ACK displays `authority.accepted`. |
| AUTH-F02 | Groups reconnect; accepted authority proof arrives in different packet orders. | All peers validate one same bounded outcome; duplicate delivery applies once. |
| AUTH-F03 | Replace phone with laptop while keeping actor/grants and exact operations. | Business outcome unchanged; Device type/endpoint/first contact is not a tie-breaker. |
| AUTH-F04 | Two Devices of one Persona both replicate a request. | Replication evidence is counted under its own policy; these Devices do not become two human approval votes. |
| AUTH-F05 | Candidate made before grant revocation reaches authority afterward. | Current rights/epoch recheck rejects or quarantines candidate; client timestamp cannot restore the old grant. |
| AUTH-F06 | Outcome appears to precede revocation, then both arrive from partitioned peer groups. | Preserve historical outcome only if a verified control order proves acceptance preceded the effective cut; otherwise keep the case unresolved under OQ-0033/REQ-MEM-004 rather than infer order from clocks or packets. |
| AUTH-F07 | Quorum candidate: partition `2+1`, then rotate membership/revoke one member. | Only a currently certificated intersecting group may accept; minority and stale epoch stay pending. This applies only if quorum is selected. |
| AUTH-F08 | Service candidate: authority is unavailable while peers can still sync data. | Replication may advance; exclusive domain outcome remains pending. This applies only if service is selected. |
| AUTH-F09 | Proof references another Request revision, Resource, Space, control frontier or policy. | Verification fails; receipt cannot promote the wrong candidate. |
| AUTH-F10 | Same event reaches direct and mailbox paths, with duplicate/reordered packets and process restart. | Stable request identity and pending/outcome state survive; transport receipt never substitutes for signed domain proof. |
| AUTH-F11 | A valid offline candidate remains pending for months without a domain deadline; another request has an explicit deadline. | Age alone does not expire the first; only the named authority can issue a bounded `expired` outcome for the second under its configured domain policy. |
| AUTH-F12 | `2-of-3` approvers approve one request from different Devices, while one Persona submits a duplicate vote. | The duplicate is not a second vote; configured approvals are checked before one named authority issues a signed exact outcome. |
| AUTH-F13 | Quorum candidate: during membership handoff, isolate groups that can certify under only the old or only the new roster; each attempts a conflicting outcome for the same request/frontier before, during and after transition. | At most one incompatible terminal outcome verifies. A stale or nonintersecting roster cannot certify acceptance; a completed handoff yields one comparable current frontier. Conditional on choosing quorum. |
| AUTH-F14 | Quorum candidate: after member removal or epoch advance, restart an old member from a snapshot containing a previously valid certificate and a pending request; replay the old proof and attempt a new acceptance. | Recovery cannot revive old authority or split the frontier. Preserve a historical outcome only if its exact proof independently establishes acceptance before the effective cut; otherwise keep the candidate pending or reject it under current control. Conditional on choosing quorum. |

[Raft's membership-change analysis](https://raft.github.io/raft.pdf) illustrates why old and new majorities must overlap during a transition; [etcd's reconfiguration and recovery guidance](https://etcd.io/docs/v3.6/op-guide/runtime-reconf-design/) illustrates why a lost quorum cannot simply force-remove peers without risking divergent histories. These are safety comparators for AUTH-F13/F14, **not** selection of Raft, a leader, a fixed server, or a crash-only fault model for VIDA Devices. Replica membership remains distinct from human `M-of-N` approval.

## Decision needed before OQ-0033 closure

Choose the authority **per exclusive operation family**, then choose how a serverless shared Space serializes it. For example, a business calendar may itself own appointment confirmation while a shared Project Space needs a separate Space rule. The user must decide whether confirmation may wait for reconnection or needs quorum-side availability; implementers then must prove membership changes and signed outcomes. No reference library or transport can make this product choice automatically.
