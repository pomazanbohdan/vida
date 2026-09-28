# Targeted reviewer gate — command/fact/sync update (2026-09-19)

Scope: AD-13 amendment and ADR-0012, not a re-review of unchanged AD-1–AD-15. Independent subagents were not used; this is a sequential fallback review.

## Rubric

Verdict: pass. A received operation is no longer ambiguous with a newly issued command. The rule binds core/runtime, AppInstance handlers and SyncLog while leaving effect-executor authority and exact hook APIs explicitly open. AD-4/AD-5 idempotent delivery and separate `authority.accepted` remain intact.

## Technology/currentness

Verdict: pass for this delta. No new library, framework, version or starter was selected. The supporting research is recorded in `technical-vida-application-logic-triggers-and-exec-2026-09-19/research.md`; its external references remain pattern evidence, not implementation mandates.

## Two compliant units / adversarial scenario

Unit A (Messenger) and Unit B (Notes) both receive duplicate operations after reconnect. Both must update local projections without invoking the originating command handler again. They can still diverge on who executes a committed-fact external effect: ADR-0012 explicitly leaves this to OQ-0045 and prohibits each receiver independently producing the effect. Thus this ambiguity is a declared implementation gate, not silently compatible production behavior.

Finding: no contradiction in the changed architecture rule; do not claim effect execution or v1 trigger API is production-ready before OQ-0045 is resolved.
