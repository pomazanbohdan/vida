---
review: persona-controller-adversarial
artifact: ../ARCHITECTURE-SPINE.md
date: 2026-09-28
verdict: adopted-conflict-direction-consistent-one-semantic-invariant-missing-production-proof-gated
---

# Persona controller adversarial review — 2026-09-28

## Verdict

AD-37/AD-40, the recovery spec and REC-F13/F17/F18 agree on signed causal history, provisional offline restoration, retained incompatible branches, and no clock/Device winner. A disputed **new** grant cannot gain shared access before signed reconciliation. One narrower authority outcome is still unspecified: the effective rights of an **already active** grant targeted by a concurrent revoke. The signer, freshness proof and wire mechanics for reconciliation are explicitly open production gates, not adopted behavior.

## Counterexample 1 — offline grant versus revoke of the same Device

Start at verified frontier `H0`: Device C has active grant `G0`; A and B may manage the Persona. While partitioned, A signs a replacement/renewal `G1` for C based on `H0`; B signs a revoke of C's existing grant/lineage based on `H0`. On reunion, both implementations retain both signed events and frontiers, mark `G1` disputed, and refuse to let `G1` confer shared access. Neither picks a clock or Device winner.

| Compliant implementation after detecting the conflict | Divergent observable result |
|---|---|
| A keeps pre-existing `G0` effective until a reconciliation explicitly revokes it; the sibling revoke has no accepted cut yet. | C continues to receive new protected bytes and may submit operations under `G0`. |
| B suspends C's `G0` for new shared reads, envelopes and accepted operations until reconciliation; previously obtained plaintext remains outside remote erasure. | C receives no new protected bytes or accepted operations. |

AD-40 decides the disputed *new* grant but does not say whether an unresolved revoke suspends a pre-existing grant, nor how a renewal and subject/lineage revoke match. AD-37 requires current controller verification; it does not define this conflict projection. ADR-0003 separates pending from effective revocation and requires current control checks, yet does not resolve this Persona controller conflict. This is a **missing semantic invariant**, because A and B expose different data from the same verified event set. Choose the effective-rights rule for the target Device at an unresolved grant/revoke conflict, including protected reads, key envelopes and acceptance; preserve already copied plaintext and local drafts. Add a permutation fixture with `G0`, `G1`, revoke, restart and late branch discovery. This finding does not assert that either proposed outcome has been approved.

## Counterexample 2 — two restores from one kit

All old Devices are lost. D and E independently restore from the same secret/bundle checkpoint `H0` without a fresh frontier. Each creates fresh Device keys, a local provisional grant, Notes and an incompatible recovery/rotation transition. Both implementations keep the signed branches and Notes, publish neither branch as verified shared authority, and require a reconciliation referencing both heads.

| Compliant implementation | Divergent result when E is lost before reconciliation |
|---|---|
| A accepts a reconciliation signed by the still-held recovery credential after checking both heads. | D can complete reconciliation alone. |
| B requires signatures from both branch Devices, or another proof unavailable to D. | D stays provisional until E or another qualifying proof returns. |

AD-40 requires a signed reconciliation referencing the conflicting branches but leaves its signer/proof and stale-history check to `OQ-0022/0024`; the recovery spec explicitly defers concurrent restore and current-controller proof. Thus the availability/security tradeoff above is a **deliberately open production gate**, not a contradiction to fix by inventing a winner. A stolen kit and a later revoked kit must be negative fixtures before either rule becomes production conformance. REC-F14 already prevents a proven revoked kit from publishing its local operations.

## Evidence

- `ARCHITECTURE-SPINE.md:326-330,344-348`: AD-37 provisional recovery and AD-40 causal branches, disputed new grant, open signer/proof.
- `SPEC.md` Constraints and Open Questions; `recovery-cases.md` REC-F07/F13/F14/F17/F18: provisional work, conflicting restores and grant/revoke fixtures.
- `docs/04-specifications/identity-domain-contract.md` controller-history paragraph; `docs/00-governance/open-questions.md:32-34`: approved boundary and open protocol.
- `docs/03-architecture/decisions/ADR-0003-offline-revocation.md:67-99,111-115`: pending/effective revoke, current control and plaintext limit.
