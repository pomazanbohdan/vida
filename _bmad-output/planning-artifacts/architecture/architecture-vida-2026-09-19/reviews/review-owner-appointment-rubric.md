# Rubric review — Owner appointment boundary

Verdict: **directionally consistent, but two scope ambiguities must be closed before implementation.** AD-6, ADR-0002 §7, ADR-0003 §9, REQ-ACL-017, SpaceMembership and resolved OQ-0052 all express the same initiator rule: only an explicit current built-in Owner may appoint another Owner; Admin with `manage_members` may assign Admin or lower but may not promote self/others to Owner. Owner-cloned custom roles do not inherit this governance right. Independent membership implementation remains gated by OQ-0031.

## Findings

1. **P1 — Personal-Space cardinality conflicts with unqualified appointment wording.** ADR-0002 §7 says a Personal Space `MUST` have *exactly one* Owner (line 104), then says a current Owner `MAY` appoint a new Owner without limiting that rule to shared Spaces (line 107). AD-6 likewise does not scope the appointment clause. A literal add-Owner operation would create two Personal Owners. State explicitly: adding a co-Owner applies only to shared Spaces; Personal owner succession/recovery must atomically replace the sole Owner without any two-Owner or zero-Owner accepted state.

2. **P1 — ACL/hard-deny carve-out covers removal but not appointment.** ADR-0001 §3 line 61 reserves *removing* another Owner outside the resource-policy chain. ADR-0002 §3 line 69 and AD-6 similarly carve out removal, while appointment is newly Owner-only. Define `assign Owner` as the same class of reserved Space-governance operation, not a `manage_members` resource-ACL cell; otherwise a Platform/Space `deny` or delegated capability mapping could inconsistently block a legitimate Owner or enable an Admin through an alias. Authority still validates explicit current built-in Owner and recipient eligibility.

3. **P2 — Initiator and approval threshold are different rules.** “Only an Owner may appoint” settles who can initiate; it does not settle whether one Owner authorization suffices in every critical Space. ADR-0003 §9 permits optional `M-of-N` for critical membership operations and exempts only *Owner removal*. State explicitly whether appointment remains subject to a configured quorum or is also unilateral. Until then, OQ-0033 should keep the threshold question visible; do not infer a no-quorum appointment from OQ-0052 resolution.

4. **P2 — Simultaneous succession and concurrent assignment/removal need a control-state fixture.** ADR-0002 §7 line 110 permits last-Owner leave only with simultaneous successor appointment; SpaceMembership specifies atomic accepted transition for Owner removal but not the coupled `RoleAssigned` + last-Owner leave/recovery case. OQ-0031 should cover transaction boundary, crash/replay and ordering for concurrent Owner appointment, Owner removal and last-Owner exit, with cardinality checked at every accepted state.

5. **P2 — Accepted amendment has stale revision metadata.** Spine `updated` is `2026-09-19`; ADR-0002, ADR-0003 and ACL requirements retain `last_updated: 2026-09-18`; SpaceMembership retains `2026-09-19`, while this Owner-appointment decision is being accepted on `2026-09-20`. Refresh revision dates/change evidence so readers can distinguish the original accepted role model from the new amendment. OQ-0052 can remain resolved once normative document revision is traceable.

## Positive trace

- ADR-0002 custom-role rule and SpaceMembership explicitly prevent an Owner-cloned role from obtaining governance privileges.
- ADR-0003 and SpaceMembership require authority revalidation from current signed control state, so stale Admin grants or a local role change cannot self-promote.
- Admin `manage_members` remains useful for Admin-and-lower appointments; it cannot assign, demote or remove Owner through command aliases.
