# Adversarial review — Owner appointment and Admin delegation

Reviewed 2026-09-20. Scope: Architecture Spine AD-6, ADR-0001/0002/0003, access-control requirements, SpaceMembership contract, product Space genesis, and OQ-0031/0033/0052. Verdict: **the direct rule is coherent: only a current explicit built-in Owner may appoint an Owner; Admin may appoint Admin/lower and cannot promote self to Owner. Three boundary cases still admit divergent implementations and need explicit gates/fixtures.**

## Finding 1 — Owner invitation can become a latent privilege escalation (high; implementation gate)

- Evidence: SpaceMembership lists `InviteIssued`, `MemberJoined`, and `RoleAssigned`, but defers invite-token replay policy. ADR-0003 requires authority to recheck current rights when accepting a command. It does not say whether an invitation to become Owner is an accepted Owner assignment at `InviteIssued` or only at `MemberJoined`/`RoleAssigned`.
- Two implementations: A authorizes Owner A when `InviteIssued` is accepted; after A is removed, the invitee joins as Owner using the still-valid invitation. B rechecks A's current Owner status at join and rejects it. Both can cite “only current Owner assigns Owner” by placing the assignment at different control events. A lets a removed Owner leave behind latent governance grants.
- Required resolution: define the exact authority-accepted event that commits Owner appointment, the control frontier used to validate sponsor status, and revocation/expiry of outstanding Owner invitations when a sponsor loses Owner status. Acceptance fixture: Owner invite issued, sponsor removed, invite accepted later, with one canonical result. This is within the already deferred invite-token and OQ-0031 control-ordering work, not permission for either behavior now.

## Finding 2 — Space-role appointment scope is underspecified (medium; enforceable authorization boundary)

- Evidence: ADR-0002 makes `Owner` and `Admin` Space-level roles, but ADR-0003 §9 and REQ-ACL-017 speak of Admin with `manage_members` “in the corresponding scope.” REQ-ACL-003 also allows app schemas to define `manage_members`, and ADR-0001's explicit resource-ACL exception names Owner *removal* but not Owner *appointment*.
- Two implementations: A accepts a container-scoped `manage_members` grant when assigning a new Space-level Admin, or lets an app-level hard deny block Owner appointment. B treats both appointments as Space-governance operations, with Admin's `manage_members` evaluated only at Space scope and Owner identity checked against Space control state. The outcomes diverge even though target roles are Space-wide.
- Required resolution: define both Owner and Admin appointment as Space-level control operations outside app/resource action aliases; require the Admin actor's `manage_members` capability at the *target role's Space scope*. A child-scope grant cannot authorize a Space role. Explicitly state whether an Owner appointment may be limited by a separate Space-governance policy; do not let resource ACL supply the answer. Conformance should test Space versus child grants/denies, expired actor grants, and `RoleAssigned(Owner)` aliases. This does not challenge the accepted ability of Admin to appoint another Admin.

## Finding 3 — AD-6 may apply the no-quorum exception to Owner appointment (medium; textual conflict)

- Evidence: AD-6 combines “Owner may assign another Owner or unilaterally remove one ... without co-owner approval or M-of-N” in one sentence. ADR-0003 §9 explicitly exempts *removal* from optional critical-space quorum, while other critical membership operations may require it. ADR-0002's appointment rule determines the actor, not the approval threshold.
- Two implementations: A parses AD-6 as exempting both assignment and removal from M-of-N; B exempts only removal and lets critical policy require quorum before Owner appointment. Both can claim conformance.
- Required resolution: attach “without co-owner approval/M-of-N” solely to removal in AD-6, unless the product owner explicitly approves unilateral Owner appointment as well. Keep appointment quorum under the generic OQ-0033 policy gate until selected. Actor identity and approval threshold are different dimensions.

## Finding 4 — negative conformance fixtures incomplete (low; test specification)

- Accepted prose already excludes Owner-clone custom roles and Admin from Owner *removal*, and only explicit built-in Owner may *assign* Owner. The SpaceMembership acceptance list tests Owner-clone removal but not Owner-clone appointment; it tests Admin self-promotion but not promotion of a different account or assigning an Owner role via a custom role/`RoleAssigned` replacement alias.
- Add negative fixtures for: custom `base=Owner` appointing Owner; Admin appointing another person Owner; Admin assigning `Admin` to an existing Owner as an implicit demotion; Guest/ServicePrincipal Owner appointment. The normative rule points to rejection, so these are missing tests rather than a fresh product decision.

## Not findings

- Initial Owner bootstrap is supplied by signed `WorkspaceGenesis` in the product Space lifecycle, so “only current Owner appoints Owner” does not make new Spaces impossible; the genesis transaction remains a special creation path.
- OQ-0031 still gates concurrent control ordering. OQ-0052 is resolved for the actor allowed to appoint Owner; it does not by itself close invitation activation or optional quorum semantics.
- ADR-0001 now expressly excludes Owner removal from the resource ACL matrix at all listed policy levels; no residual hard-deny conflict was found for the removal operation.
