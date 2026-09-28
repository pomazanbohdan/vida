# Rubric review — full Owner membership removal

Verdict: **accepted decision is consistently propagated, with two boundary ambiguities; independent implementation remains gated.** AD-6, ADR-0002/0003, REQ-ACL-009/014/017 and SpaceMembership agree on complete Space membership removal by another current Owner, without co-owner approval or `M-of-N`. They preserve at least one Owner, deny Admin removal/demotion, require authority acceptance/audit/key-epoch advance, and distinguish compliant-client cache locking after receipt from impossible offline remote wipe.

## Findings

1. **P1 — Higher-level hard-deny boundary remains ambiguous.** ADR-0001 §3 (lines 53–61) gives explicit hard deny precedence across `Platform → Space → ...`, but its new exception names only resource permission matrix and app/container/resource hard deny. AD-6 likewise says Owner removal works regardless of *app/resource* hard deny. A Space- or Platform-level hard deny of `manage_members` or governance action could still be read to defeat the unconditional Owner-only removal right. Specify that this operation is a reserved Space governance capability, not a configurable ACL cell at any policy level, while preserving authority checks, active Owner status and the ≥1-Owner invariant. If a platform-level safety suspension may override it, define that separately rather than relying on generic hard deny.

2. **P1 — Admin policy-management preset can indirectly cross the Owner boundary.** ADR-0002 §2 line 58 still gives Admin broad “policies” management; §7 line 106 says policy must not bypass Owner-only removal; OQ-0052 explicitly defers Admin governance-policy changes. Until OQ-0052 closes, narrow the preset to **non-governance/content policies** and reject any Admin mutation that changes Owner-removal authorization, Owner role definitions, or governance-policy ownership semantics. The current prose communicates intent but does not define a clear enforceable policy namespace.

3. **P2 — Atomic full-removal transition is not yet specified.** SpaceMembership line 40 requires `MemberRemoved` together with revoking every Space grant, maintaining ≥1 Owner, audit and affected epoch changes, but does not yet say whether these are one authority-accepted control transaction or an ordered multi-event bundle. A partially applied state could leave residual access or an ownerless projection. `OQ-0031` already blocks independent implementation; its eventual answer must cover crash-point, replay, concurrent cross-removal and key-envelope tests for the entire transition.

4. **P2 — Notification-surface promise needs platform scope.** ADR-0003 line 110 and SpaceMembership line 65 say notifications stop displaying managed Space data once a compliant client receives effective removal. App-owned inbox/search/recents are controllable; previously delivered OS-owned notification banners may not be retractable. Clarify that the guarantee applies to client-managed surfaces and future notifications, while best-effort removal of prior OS banners is platform-gated. This avoids overstating the post-receipt guarantee without weakening cache locking.

## Positive trace and open gates

- ADR-0002 §7, ADR-0003 §9 and SpaceMembership §Contract apply the same Owner-only authorization to all aliases that cause Owner loss; successful removal leaves **no lower-role membership** or residual Space grants.
- ADR-0003 §6 and §7, REQ-ACL-014/016 and OQ-0051 consistently separate local hiding after a client learns revocation from the open offline-read freshness question and the impossibility of remote erasure before delivery.
- OQ-0052 correctly keeps Admin appointment of new Owners and governance-policy editing undecided; ordinary non-Owner member administration remains allowed.
- OQ-0031 correctly gates serialization and cross-removal; no finding here should be mistaken for implementation readiness.
