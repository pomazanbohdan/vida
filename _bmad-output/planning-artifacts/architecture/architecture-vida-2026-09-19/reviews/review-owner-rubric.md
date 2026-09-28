# Rubric review — unilateral Owner removal

Verdict: **conditionally coherent, not implementation-ready**. The user-approved one-Owner authorization is reflected in the spine and companion documents; an inherited ACL conflict and operation-path ambiguity still need resolution. `OQ-0031` correctly blocks independent implementation of concurrent membership changes.

## Findings

1. **P1 — Unconditional Owner governance conflicts with general hard-deny precedence.** [ADR-0001](../../../../../docs/03-architecture/decisions/ADR-0001-layered-access-control.md) §3 says lower policy cannot exceed a higher maximum and explicit hard deny always wins; [ADR-0002](../../../../../docs/03-architecture/decisions/ADR-0002-default-role-presets.md) §3 says a role never bypasses hard deny, yet §7 and spine AD-6 promise that *every current Owner* can unilaterally revoke another Owner. A policy hard deny of `manage_members` or equivalent could therefore make the promise false. Clarify that Owner-role revocation is a reserved Space control-plane governance action outside app/container/resource ACL overrides; ordinary content permissions remain subject to hard deny. Preserve authority validation, audit, and the nonempty Owner set.

2. **P1 — Equivalent mutation paths lack a unified guard.** [SpaceMembership](../../../../../docs/04-specifications/space-membership-contract.md) exposes both `RoleRevoked` and `MemberRemoved`; ADR-0002 also names `remove` and `demote`. ADR-0003 broadly permits an Admin with `manage_members` to initiate shared-Space access changes, but the Owner-removal exception speaks only of a current Owner revoking another Owner's role. Define one rule for every command that causes an existing Owner to lose Owner status, including member removal, demotion, role replacement and bulk changes; otherwise an alternate command can sidestep the one-Owner/no-quorum/last-Owner checks. Whether Admin can initiate such a change is not settled by the user's specific statement and should be confirmed or explicitly reserved to Owners.

3. **P2 — Target's post-removal membership state is unspecified.** User wording “remove other Owners” could mean removing only the Owner role, demoting to another human role, or removing Space membership. Current normative text says `RoleRevoked` but does not state whether the target remains a Member, what role replaces Owner, or whether all grants are revoked. This affects key rotation, UI, audit, and authorization fixtures. Do not infer a destructive membership deletion from an ownership-role decision.

4. **P2 — Cross-removal ordering remains an intentional gate.** Two current Owners may concurrently remove each other. [OQ-0031](../../../../../docs/00-governance/open-questions.md) correctly remains open and spine implementation gates prevent independent control-plane implementation. Specify authority serialization, first-accepted-versus-rejected behavior, idempotent retries, and convergence fixtures before claiming the model is executable. Every accepted intermediate state must retain at least one Owner.

## Positive trace

- ADR-0002 §7, ADR-0003 §9, REQ-ACL-009/017, SpaceMembership contract and spine AD-6 all express single-Owner authorization without co-owner approval or `M-of-N` for revoking another Owner.
- ADR-0003 preserves online authority acceptance, signed control ordering, future-access/key-epoch semantics; unilateral approval does not imply unvalidated local effect.
- Acceptance evidence covers audit and nonempty Owner cardinality; OQ-0031 keeps unresolved concurrency visible.
