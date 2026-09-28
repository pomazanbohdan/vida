# BMad architecture reviewer gate — ten-decision amendment

Date: 2026-09-20. Scope: `ARCHITECTURE-SPINE.md` AD-13/16/22–25, ADR-0003/0017, approved decision batch, related requirements, `SyncLog`, `BlobStore`, `SpaceMembership`, and implementation gates. This is a review only; it does not approve unresolved protocol mechanics.

## Verdict

**Revise before calling the ten-decision package fully reconciled.** The architecture spine is largely coherent and appropriately gates authority proof, conflict semantics, blob retention, external effects, AppInstance version binding, and seven-day freshness. No new device arbiter, silent winner, age-only candidate expiry, or irreversible-effect-on-delivery rule was found. Three cross-document inconsistencies remain: one approval-scope mismatch and two stale/unsafe `SyncLog` acceptance statements. These are document corrections, not reasons to reverse the user's decisions.

## Critical

None found. The unresolved peer-equal authority and safe-retention mechanisms are explicitly blocked from independent/production implementation by spine lines 270–274.

## High

1. **Agent opt-in is narrowed from action *type* to an ambiguous individual action.** The approved answer says an agent may act after explicit permission **for that type of action** (`docs/00-governance/decision-batch-device-core-2026-09-20.md:47`), and `REQ-EFFECT-009` uses “відповідного виду дії” with a separate permission for a file-conflict action (`docs/02-requirements/effect-execution-constraints.md:28`). AD-25 and its gate instead say `per-action opt-in` (`ARCHITECTURE-SPINE.md:242,273`), echoed in `docs/00-governance/open-questions.md:55`. This can mean approval of every individual resolution, changing the accepted product decision, or permission for a class of actions, creating incompatible implementations. Specify **explicit opt-in per action type** as the accepted minimum, with per-instance confirmation left as a distinct optional policy/`OQ-0045` question if desired. Preserve current-rights, variant-read, principal and audit checks.

2. **`SyncLog` admits a merely eligible branch into the accepted conflict projection.** The contract says two `authority-eligible/accepted` variants become the shared multi-value conflict (`docs/04-specifications/sync-log-contract.md:68`). The slash conflates a candidate that *could* be accepted with one that *has* authority acceptance; AD-22 requires accepted branches and excludes pending invalid/revoked candidates (`ARCHITECTURE-SPINE.md:224`), as do `REQ-SYNC-004/007/009` (`docs/02-requirements/transport-sync-requirements.md:47,50,52`) and the same spec's local-vs-authority frontier distinction (`sync-log-contract.md:42,44`). Replace the phrase with “authority-accepted variants”; stage eligible-but-unaccepted alternatives separately until proof arrives. Add a negative fixture for eligible-but-unaccepted, not only revoked/invalid pending.

## Medium

3. **A stale `SyncLog` fixture reopens outcomes the user already settled.** The file-conflict fixture says “однакові outcomes, proof і пізня третя гілка лишаються `OQ-0034`” (`docs/04-specifications/sync-log-contract.md:70`). Equivalent-resolution visible outcome and late accepted incompatible branch were approved (`decision-batch-device-core-2026-09-20.md:43–44`) and specified immediately below (`sync-log-contract.md:72–73`; `REQ-SYNC-010/011` at `transport-sync-requirements.md:53–54`; AD-23 at spine line 230). Only exact equivalence predicate, proof and wire mechanics remain deferred. Amend line 70 accordingly so implementers cannot treat the two outcomes as undecided.

## Rubric checks / preserved decisions

- **Decision quality:** AD-13/16/22–25 each names the boundary, divergence prevented and enforceable rule. Rule-version binding, peer-equal acceptance, recursive conflicts and no-winner projection remain separate from packet delivery and local durability.
- **Input reconciliation:** ADR-0003/`REQ-ACL-016–019` and AD-16 agree on seven-day shared-read lock including Owner, no Personal Owner lock, and no age-only expiry for pending mutations; `SpaceMembership` carries the read/draft behavior (`docs/04-specifications/space-membership-contract.md:59–63`).
- **Conflict preservation:** ADR-0017, AD-22/23, `REQ-SYNC-009–013` and `BlobStore` agree on recursive incompatible conflicts, equivalent visible results with dual audit, late-branch reopening, per-variant read rights and no early blob GC (`docs/04-specifications/blob-store-contract.md:47,56–59`).
- **Effects and derived state:** AD-24/`REQ-EFFECT-008` wait for applicable authority confirmation and no known conflict; AD-13/`REQ-EFFECT-007/010` bind source-fact rule version and one logical derived resource. Remaining proof/executor/compensation/identity details are genuinely open, not silently selected.
- **Safe gates:** spine lines 270–274 and 279 block domain acceptance, independent `SyncLog`, irreversible effects, agent auto-resolution, AppInstance derived-resource production and rights freshness until the relevant `OQ-0033/0034/0045/0048/0053` contracts and fixtures exist. The gates should use the clarified **action-type** wording from finding 1.

## Acceptance for re-review

Revise the three cited `per-action` opt-in locations consistently; restrict accepted conflict variants to proven authority-accepted branches and test the pending eligible case; remove stale deferred-outcome language from `SyncLog` line 70. Keep the current implementation gates in place. No new user decision is necessary unless the team intends to require individual approval for every agent resolution in addition to the already approved action-type opt-in.
