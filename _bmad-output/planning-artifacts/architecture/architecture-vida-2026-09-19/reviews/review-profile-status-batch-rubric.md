# BMad architecture reviewer gate — profile/status/effects batch

Date: 2026-09-20. Scope: current `ARCHITECTURE-SPINE.md` AD-24–27 and implementation gates; the user-answer ledger in `decision-batch-device-core-2026-09-20.md:54–65`; `OQ-0033/0045/0064–0066`; relevant client, effect, presence and blob contracts. Read-only assessment; no protocol or quorum decision is made here.

## Verdict

**Conditional pass for the architecture amendment; minor reconciliation needed before calling the document set final.** The new spine decisions reflect the answers and safely gate unsolved proof, privacy, publication-order, external-effect and file-download mechanics. In particular, the user's word “кворум” for synchronization of two own devices is **not** elevated to shared-Space domain authority. No critical or high architecture blocker was found. The findings below concern stale references and one term that should not be allowed to pre-decide authority semantics.

## Critical / High

None found. `ARCHITECTURE-SPINE.md:282,285–287,357` keeps peer-equal domain acceptance, same-Persona two-copy evidence, profile presence, external effects, chat publication and blob download policy behind distinct gates. `OQ-0033` remains open; AD-26 does not turn delivery or replica count into authority proof.

## Medium

1. **Publication decision is still described as wholly open in the product question ledger.** The accepted answer fixes the *relative shared order*: offline group message written at 10:00 and published at 13:00 belongs at the 13:00 publication position (`docs/00-governance/decision-batch-device-core-2026-09-20.md:60`; AD-27 at `ARCHITECTURE-SPINE.md:254`; `REQ-CLIENT-008` at `docs/02-requirements/native-client-requirements.md:21`). Yet `docs/00-governance/open-questions.md:22` (`OQ-0012`) still says the canonical publication time/order versus local time is open. Reword it to say **rule approved; exact publication proof, clock and incomparable tie-order open in OQ-0065**. Otherwise the product ledger invites a second decision on an already settled outcome.

2. **Publication proof has two open-question owners.** `REQ-CLIENT-008` points its exact shared-publication proof/clock to `OQ-0034` (`native-client-requirements.md:21`), whereas the new dedicated question and spine gate assign those mechanics to `OQ-0065` (`open-questions.md:75`; `ARCHITECTURE-SPINE.md:254,287`). Either cross-reference both with clear split (`OQ-0065` publication, `OQ-0034` shared SyncLog primitives), or make `OQ-0065` the primary owner. A conformance team should not have to guess which closure unlocks group-chat ordering.

3. **“Accepted publication event” risks implying an unapproved domain-acceptance point.** AD-27 (`ARCHITECTURE-SPINE.md:254`) says the message enters order at its *accepted publication event*. The user approved ordering by publication after sync, not the authority proof that makes publication accepted (`decision-batch-device-core-2026-09-20.md:59–60`; `OQ-0065`). The gate at spine line 287 prevents implementation now, but use a neutral “shared publication event as defined by OQ-0065” or explicitly say that this word does not make same-Persona replica ACK a shared-Space authority receipt. Preserve separate local-written-at metadata.

## Low / housekeeping

- `docs/00-governance/contour-status.md:24` still calls the requested indicators “Space-level” and the number “online devices”; the later accepted profile-level count supersedes that location (`decision-batch-device-core-2026-09-20.md:56`; AD-26). Mark the row superseded or update its current-status summary so roadmap readers do not revive a Space aggregate.
- `REQ-EFFECT-008` says “компенсація” remains open (`docs/02-requirements/effect-execution-constraints.md:27`). The later approved `REQ-EFFECT-011` at line 30 allows a separate corrective proposal, not fictitious cancellation of a completed effect. When specifying `OQ-0045`, distinguish provider reconciliation/unknown outcome from a user-visible correction to avoid interpreting “compensation” as automatic undo.

## Rubric and input-reconciliation checks

- **AD-24/effects:** authority confirmation and no known unresolved conflict precede irreversible execution; a later conflict preserves the executed audit fact and offers a separate correction (`ARCHITECTURE-SPINE.md:236`; `REQ-EFFECT-008/011`). Unknown-outcome retry is explicitly research/open, not chosen from RFC/Stripe/Temporal examples (`effect-execution-constraints.md:35–40`; gate at spine line 286).
- **AD-25/agents:** opt-in is for the action **type**, current action/read rights apply to all necessary variants, and the existing ACL/ServicePrincipal model is used rather than a new Owner/Admin approval tier (`ARCHITECTURE-SPINE.md:242`; `REQ-EFFECT-009`; `OQ-0045`). Exact mapping/delegation stays gated.
- **AD-26/presence:** count belongs to a visible profile/Persona, not all devices in a Space; online means sync-capable over direct or usable relay, not mere internet; replication, domain acceptance and effect completion stay separate (`ARCHITECTURE-SPINE.md:248,285`; `sync-presence-status-model.md:14–24`; `OQ-0064`). Anonymous/private visibility and anti-correlation are open.
- **AD-27/chat:** publication order and written-at metadata match the 10:00/13:00 user example; tie order, proof and clocks stay `OQ-0065` (`ARCHITECTURE-SPINE.md:254,287`). This should remain separate from AD-19/22 task-status conflict order.
- **File admission:** configurable auto-download limit and explicit above-limit confirmation are documented; download admission does not authorize GC of retained conflict variants (`blob-store-contract.md:45,57`; `OQ-0066` versus `OQ-0034`; spine line 287).
- **Quorum guardrail:** two durable replicas of one Persona may prove two copies, not two independent authorities or an accepted shared-Space mutation (`decision-batch-device-core-2026-09-20.md:59`; `open-questions.md:43`; spine lines 285,357). `ADR-0003`'s policy-specific membership quorum is a separate rule.

## Re-review acceptance

Align `OQ-0012`, `REQ-CLIENT-008` and the roadmap row with the approved profile/publication decisions; clarify AD-27's publication term without choosing the still-open authority proof. Keep `OQ-0033/0045/0064–0066` production gates intact. No new user choice is required for these editorial corrections.
