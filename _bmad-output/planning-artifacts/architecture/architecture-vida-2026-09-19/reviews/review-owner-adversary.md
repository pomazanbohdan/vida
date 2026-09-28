# Adversarial review — unilateral Owner removal

Reviewed 2026-09-19. Scope: Architecture Spine AD-6, ADR-0001/0002/0003, REQ-ACL-009/017, SPEC-SPACE-MEMBERSHIP-001, OQ-0031. Read-only review of the accepted user preference: a current Owner may remove other Owners without co-owner consent. Verdict: **two normative ambiguities require correction before independent implementation; OQ-0031 properly gates concurrency details**. No objection to the user's simpler one-Owner authorization model itself.

## Finding 1 — Owner right conflicts with unconditional hard-deny precedence (high; immediate correction)

- Evidence: ADR-0001 §3 states that explicit hard deny always takes precedence. ADR-0002 §3 says Owner role does not bypass hard deny. ADR-0002 §7 and REQ-ACL-009 now promise every current shared-Space Owner a unilateral right to revoke another Owner. AD-6 repeats it.
- Two conforming implementations: A permits a Space-level `deny manage_members` or `deny owner_revoke` to stop an Owner from removing another Owner. B treats Owner governance as a non-deniable platform invariant and permits the action. Both can cite accepted text, yet their security and recovery results diverge.
- Required resolution: explicitly define whether an Owner's ability to change Owner membership is a governance capability outside mutable Space/App/resource hard denies, or whether higher-level platform hard deny can block it. If the latter, replace the unconditional “each Owner may” promise with the precise exception. Conformance must cover an Owner facing a deny in the permission matrix.

## Finding 2 — Admin's power over Owner roles is undefined (high; immediate correction)

- Evidence: ADR-0002 §2 gives Admin “Members, roles”; ADR-0003 §9 and REQ-ACL-017 allow an Admin with `manage_members` to initiate shared-Space access changes. ADR-0002 §7 says an Owner may revoke another Owner but does not say whether that right is exclusive. The Admin restriction only says no transfer/delete *Space* by default.
- Two conforming implementations: A permits Admin to revoke or grant Owner role using `manage_members`; B reserves all Owner-role changes to a current Owner. In A, Admin may even assign self Owner and then remove existing Owners, bypassing the intended governance boundary.
- Required resolution: explicitly scope `manage_members` and role administration with respect to `Owner` grants, demotions, revocations and member removal. If Owner-only is intended, state it in ADR-0002/0003 and REQ-ACL-017 and add negative conformance fixtures for Admin self-escalation. If Admin is intended to act, record that as a separate product decision; the current unilateral Owner statement does not answer it.

## Finding 3 — “remove another Owner” is event-type dependent (medium; immediate product clarification)

- Evidence: the user's wording means “видалити всіх власників інших”; the updated documents instantiate this as `RoleRevoked`. SPEC-SPACE-MEMBERSHIP-001 also permits `MemberRemoved`, while ADR-0002 §7 lists remove/demote/revoke/leave and ADR-0003 §9 exempts only revoking the Owner role from optional critical quorum.
- Two conforming implementations: A revokes only the Owner role, leaving the former Owner as a Member. B removes that person from the Space entirely. Alternatively, one implementation routes `MemberRemoved` through critical `M-of-N` while another treats it as the same unilateral Owner-removal action. Both appear to satisfy parts of the text but produce different access and key-rotation outcomes.
- Required resolution: distinguish `remove Owner role` from `remove member from Space`; specify the post-action role/access and whether the no-quorum rule follows the semantic effect across `RoleRevoked`, demotion and `MemberRemoved`. Do not choose the post-action role without asking the product owner.

## Finding 4 — simultaneous cross-removal is correctly gated, not yet an accepted rule (implementation gate)

- Evidence: OQ-0031 explicitly names Owners A and B concurrently revoking one another; SPEC-SPACE-MEMBERSHIP-001 blocks independent implementations until deterministic authority serialization and causal cut exist. ADR-0003 requires authority recheck, accepted control sequence and at least one remaining Owner.
- Divergence if gate ignored: one implementation accepts A→remove B first and rejects B's stale command; another accepts both optimistic commands and must repair an ownerless state. The second would violate the at-least-one-Owner invariant. The exact tie-break/receipt/replay mechanism remains open under OQ-0031 and should not be presented as decided.
- Gate test: competing A/B removals from the same base control head converge on one accepted Owner and one non-effective attempt with an explicit conflict result; no valid replay reaches zero Owners. Decide stale-command UX alongside serialization.

## Not a finding

- ADR-0003 §9 now explicitly exempts Owner-role revocation from `M-of-N` even in a critical Space. That resolves the direct quorum conflict for `RoleRevoked`. Finding 3 concerns other operations with the same ownership-removal effect.
- ADR-0002 §7 preserves at least one Owner; a single Owner sequentially removing every *other* Owner does not itself violate cardinality. Last-Owner self-removal still requires simultaneous succession.
