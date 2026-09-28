# Adversarial review — full Owner removal

Reviewed 2026-09-19. Scope: Architecture Spine AD-6; ADR-0001/0002/0003; REQ-ACL-009/010/013/014/017; SPEC-SPACE-MEMBERSHIP-001; OQ-0031/0051/0052. Verdict: **the full-removal outcome and Owner-only rule are now coherent, but one policy-precedence conflict and one role-identity loophole remain before independent implementations can safely agree**. Concurrency, offline-read freshness and Admin Owner appointment are explicitly gated, not silently decided.

## 1. Space/Platform hard deny versus unconditional Owner power — high, normative conflict

- ADR-0001 §3 says an explicit hard deny always wins along `Platform → Space → ...`; its new exception names only AppInstance/Container/Resource permission matrices. AD-6 likewise says Owner removal prevails regardless of *app/resource* hard deny. ADR-0002 §7 and REQ-ACL-009 unconditionally say every current Owner may remove every other Owner.
- Compliant implementation A treats `Space: deny owner_remove` as a hard deny and rejects the Owner command. Compliant implementation B treats Owner removal as a reserved governance primitive and accepts it despite that deny. The accepted documents do not select A or B. A Platform-level emergency restriction poses the same question and should not be silently equated with an editable resource deny.
- Required resolution: define precedence for Space-level and Platform-level Owner-governance restrictions explicitly. Either reserve the operation outside mutable permission matrices (with any immutable platform safety exception separately named), or qualify the unconditional Owner entitlement. Add a fixture for each policy level.

## 2. Custom Owner-derived role can blur “current Owner” — high, normative authorization gap

- ADR-0002 §4 permits custom roles by cloning built-in presets, including no stated exclusion for `Owner`; Owner preset includes all capabilities. ADR-0002 §7 and AD-6 restrict removal to a “current Owner,” but do not define whether that means an authoritative built-in Owner membership state or a custom role carrying equivalent capabilities. REQ-ACL-007 likewise does not exclude an Owner clone.
- Implementation A counts `CustomRole(base=Owner)` as Owner for cardinality and removal authorization. Implementation B does not count it and rejects removal by that principal. Both can cite the current role model; A can create a governance delegation path that the Owner-only sentence appears intended to exclude.
- Required resolution: define `Owner` as a non-delegable, Space-level membership status for governance and cardinality, or explicitly permit custom-role delegation and specify its bounds. If non-delegable, cloning may copy ordinary permissions but not Owner governance powers, and fixtures must reject custom-role and ServicePrincipal impersonation of Owner.

## 3. Admin policy/edit pathway — explicitly gated by OQ-0052, not currently a grant

- ADR-0002 gives Admin broad `apps, policies, schemas, settings`, but also forbids command/policy aliases that remove or demote an Owner. ADR-0003 and REQ-ACL-017 deny Admin `manage_members` as a path to Owner removal and do not infer Owner appointment from it. OQ-0052 leaves Admin Owner appointment/governance-policy editing open.
- Thus Admin direct removal is **not** a compliant divergence. The remaining divergent implementations would let Admin edit the control-plane policy, signing authority or role-to-Owner mapping, versus limiting Admin to app/resource policy. That choice is not accepted and must remain blocked under OQ-0052; encode the reserved governance namespace and negative tests before rollout.

## 4. Visibility, keys and concurrency — accepted guarantee, deferred mechanics

- After effective `MemberRemoved` reaches a compliant client, ADR-0003 §6 and the SpaceMembership contract require managed Space data to disappear from UI/API/search/notifications/export; grants and key envelopes are invalidated. An implementation that continues displaying managed cached content after receipt is not compliant. Physical erasure of offline devices or pre-copied plaintext is explicitly *not* promised; whether a device without fresh control head may continue offline reading remains OQ-0051.
- Owner removal revokes all Space grants and advances key epochs of every scope from which the removed member could otherwise decrypt *future* content. A root-only rotation that leaves a usable child key for future writes violates ADR-0003 §5, even if the root epoch changed. Exact key topology and atomic causal cut need conformance fixtures; OQ-0031 and the SpaceMembership deferred key protocol gate implementation.
- Two Owners concurrently removing one another must not produce an ownerless Space; the accepted control sequence, stale-command handling and tie-break are OQ-0031. “First accepted wins” is a candidate, not yet a normative rule. Do not present it or physical cache deletion as an accepted outcome.

## Resolved since preceding review

- ADR-0002 now says full Space membership removal rather than demotion, and the contract uses `MemberRemoved`; no outcome ambiguity for a successful Owner-removal operation.
- Semantic aliases and demotion paths must pass the same Owner-only check; Admin cannot bypass by renaming a command.
- Critical `M-of-N` does not apply to Owner removal, while authority acceptance and at-least-one-Owner invariants remain mandatory.
