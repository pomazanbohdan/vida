# Reviewer gate — rubric walker r2

Verdict: PASS with explicit open gates.

- AD-28 now makes Android HA optional and iOS online proof platform-realistic without weakening online-first durability.
- AD-29 is enforceable: closed update modes, trust default, atomic activation, instance isolation, no post-activation downgrade and preserved anti-downgrade security.
- Clear fix applied: aborted pre-commit activation is distinguished from rollback.
- OQ-0068 and OQ-0069 correctly keep iOS distribution profile and Core semantic classes out of accepted decisions.
- Remaining high-impact open items: exact presence TTL/retry thresholds, serialized authority proof, update-mode compatibility definition and forward-repair UX.
