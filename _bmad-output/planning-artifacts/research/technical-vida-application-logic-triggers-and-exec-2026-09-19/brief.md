# VIDA application logic triggers and execution phases — research brief

- Decision served: define a candidate trigger/phase contract for application logic in a Space-scoped `AppInstance`, then discuss it with the user before any approval.
- Type/shape: technical / explore.
- Topology/effort: focused inline research, three dimensions, at most two evidence rounds per dimension; no subagents. Primary official documentation first; no performance numbers without independent verification.
- Question 1 — entry points: commands from UI/bot/API, data mutation, timers, inbound integration events, package lifecycle, sync replay. Which should invoke app logic and which must not?
- Question 2 — phases: validation/transformation before durable commit versus reaction after commit; events, durable workflows, host capabilities, error handling, retries and idempotency.
- Question 3 — composition: same named action/trigger in base and instance, explicit override versus additive action, handler ordering, suppression/continuation, versioning and multi-device authority.
- Reference classes: official docs for PostgreSQL triggers/outbox and CDC, automation products, durable workflow engines, extension/plugin systems, and sandbox/capability contracts. External sources are evidence; project files shape questions only.
- Existing VIDA context to preserve: ADR-0009 managed logic, ADR-0010 AppInstance scope, ADR-0011 instance precedence, OQ-0045 open; Iroh transport not scripting engine; trusted core authorization/storage/sync remain authoritative. Prior raw research is not proof of external behavior.
- Deliverable: cited `research.md` and offline briefing for discussion; no accepted ADR or normative spec edits before user decision.
