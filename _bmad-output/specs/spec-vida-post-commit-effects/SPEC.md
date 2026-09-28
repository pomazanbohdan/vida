---
id: SPEC-VIDA-POST-COMMIT-EFFECTS
status: draft
companions:
  - effect-conformance-cases.md
  - ../../../docs/02-requirements/effect-execution-constraints.md
  - ../../../docs/03-architecture/decisions/ADR-0009-managed-application-logic.md
  - ../../../docs/03-architecture/decisions/ADR-0010-instance-scoped-app-logic.md
  - ../../../docs/03-architecture/decisions/ADR-0011-instance-trigger-precedence.md
  - ../../../docs/03-architecture/decisions/ADR-0012-command-event-sync-boundary.md
  - ../../../docs/04-specifications/operation-finality-contract.md
  - ../spec-vida-approval-process/SPEC.md
sources: []
---

> **Draft BMad contract.** Approved reaction boundaries are distilled into planned checks. The executor, rule-version binding and external-provider retry mechanism are not selected; no fixture has run.

# VIDA post-commit reactions and external effects

## Why

One accepted task event may create a Note, notify people or invoke an outside service while several equal Devices receive the same history. Users need one shared result and an honest effect status, not a new business command each time another Device synchronizes. The Core must preserve this behavior across offline operation and package updates without granting App logic authority over security or synchronization.

## Capabilities

- **CAP-1**
  - **intent:** An AppInstance can react to an accepted domain fact while other Devices apply that fact without rerunning its originating command.
  - **success:** Reconnect, duplicate delivery, replay and projection rebuild change local views but do not submit a second `message.send`, `notes.update` or external effect from the received operation.
- **CAP-2**
  - **intent:** One accepted source fact can produce one shared logical resource under its bound rule revision.
  - **success:** Three authorized Devices, including mixed App versions, converge on one derived Note with one logical ID; a pending or rejected source candidate produces no accepted derived Note, and a later package update does not reinterpret the old fact.
- **CAP-3**
  - **intent:** An irreversible effect can wait for sufficient domain confirmation and expose a separate execution result.
  - **success:** A locally saved or merely delivered `Done` does not send the letter; a known unresolved conflict blocks it; an eligible confirmed transition can progress through effect-specific receipt/status independently of sync or authority state.
- **CAP-4**
  - **intent:** The user can see an unknown external result without VIDA fabricating success or repeating an unsafe request.
  - **success:** A lost provider response without supported lookup or safe same-key retry leaves `effect.unknown`; the same intent is not blindly sent twice and its evidence remains available for reconciliation.
- **CAP-5**
  - **intent:** After a completed effect, an actor whose previously unseen, authority-accepted branch opens a conflict can review a correction without erasing the effect history.
  - **success:** The completed effect remains in audit; the actor of the valid outside-frontier branch sees `correction_needed`, while several incomparable valid outside-frontier actors are not collapsed into an arbitrary single assignee; another authorized user's normal update is not misrepresented as that correction.
- **CAP-6**
  - **intent:** Human and ServicePrincipal automation can make ordinary domain changes under the same action rights, with explicit treatment of conflict context.
  - **success:** A bot's ordinary permitted update follows normal sync; a conflict-covering operation also requires access to all bound variants and explicit conflict-context automation opt-in, without a separate `resolve_conflict` ACL capability.

## Constraints

- A committed domain fact means accepted under the relevant Space/object policy. Local pending projection, delivery ACK and `sync.apply` are not a new fact or permission to rerun a business command.
- Managed logic is activated for one AppInstance in one Space. An instance handler outranks the base handler at the same supported extension point, while Core still validates authorization, acceptance, integrity, keys and sync.
- The accepted source fact binds its effective rule revision. Neither a second Device nor a later package version may turn that fact into an additional logical resource; the exact binding point and ID derivation remain open.
- An irreversible external effect waits for authority confirmation and absence of a **known** unresolved conflict. It binds a stable idempotency key and causal confirmation frontier; this cannot prove that an unknown late branch will never arrive or that an external provider executed only once.
- External result, domain outcome, replication and delivery are separate evidence axes. Provider-specific lookup/retry is allowed only if its contract makes that path safe; otherwise the effect remains unknown.
- Fully autonomous Personal Space may defer an external effect until an owner's Device returns. An identity/relay-only node may forward an opaque signal but does not gain plaintext, keys or execution authority from deployment.
- Conflict detection is Core-wide. A normal new update is not a resolution of previous variants; an operation causally covering conflicting heads requires the ordinary domain-action right plus read access to every covered variant. Automation in that context needs explicit opt-in, not a special ACL role.
- Security verification follows [OWASP Authorization](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) for deny-by-default and per-action permission checks; [OWASP AI Agent Security](https://cheatsheetseries.owasp.org/cheatsheets/AI_Agent_Security_Cheat_Sheet.html) additionally informs scoped tools and high-impact checks for LLM-backed agents. Neither defines VIDA's peer executor or adds an agent-only ACL capability.

## Non-goals

- Selecting a single executor, lease/quorum topology, exact trigger phases, handler continuation, Rhai/Wasm runtime or iOS downloadable-code profile here.
- Selecting the rule-version binding point, derived ID algorithm, old-handler retention/fallback or provider-specific retry/compensation scheme without `OQ-0045/0048` decisions.
- Replacing the separate approval-process, notification/read-state or exclusive-authority contracts; this spec does not add City booking to Release 1.

## Success signal

In a two-Persona, three-Device test, one accepted Task fact creates exactly one shared Note despite replay and mixed package versions. An irreversible effect waits for confirmation, keeps an honest independent result, and a later valid authority-accepted incomparable branch opens an attributable correction without deleting the original effect. [EFF-F01–F17](effect-conformance-cases.md) are planned checks, not implementation evidence.

## Open Questions

- Which authorized executor and recovery protocol claims an effect in a peer-only Space after offline, crash or partition without making a physical Device master?
- Where is the effective rule revision bound, how are historical handlers retained, and what happens when no compatible Device can execute one?
- Which named trigger phases, base-handler continuation, error semantics and controlled host API are available across Release-1 platforms, including iOS's approved declarative profile?
- Which provider-specific status lookup, idempotency window and user-visible reconciliation path close `effect.unknown`? No generic exactly-once guarantee is approved.
- How do normal automation opt-in, conflict-context opt-in, principal/delegation, audit, revocation and execution budgets map into the general ACL model?
