---
review: persona-controller-adversarial-r2
artifact: ../ARCHITECTURE-SPINE.md
date: 2026-09-28
verdict: prior-conflict-gap-closed-same-device-grant-semantics-open-production-proof-gated
---

# Persona controller adversarial review — round 2

## Verdict

The prior renewal/revoke gap is closed: a peer that sees the conflict suspends the existing Device's new protected reads/writes, key distribution and accepted operations. AD-40 also decides that independently valid grants for **different** Devices merge. It does not decide how two independently approved grants for the **same** Device/key combine. Separately, global single-use and reconciliation/freshness proofs remain expressly gated by `OQ-0022/0024`; a local prototype cannot claim production enrollment conformance.

## Product gap — two distinct approvals for one new Device

At verified `H0`, trusted A and B independently create distinct short-lived intents `I1` and `I2`, each bound to new Device D's same key. Each intent is used once and explicitly approved on its own trusted Device. A approves scope S1; B approves scope S2. Both approvals are valid, signed and causally concurrent. No grant/revoke or recovery conflict exists; the AD-40 automatic merge rule explicitly names grants for *different* Devices.

| Compliant implementation | Shared result after synchronization |
|---|---|
| A treats the two grants as additive scope grants for one Device key. | D gains S1 and S2; both approval events remain auditable. |
| B requires one effective grant/intent lineage per Device key and classifies the overlapping approvals as unresolved. | D gains neither new scope until owner resolution or a fresh consolidated approval. |

Both obey key binding, explicit approval, per-intent single use, signed causal history and no clock/Device priority. The divergent access result is a **missing product/semantic invariant**, not a cryptographic choice: define whether multiple live grants for one `DeviceId`/key may coexist, whether scopes union/intersect/conflict, and whether `DeviceId` reuse across independent intents is legal. If scope is always Persona-wide, state that explicitly and replace this fixture with equal-scope duplicate approvals. Add permutations for `I1/I2`, same key, unequal/equal scopes, crash/retry and a later revoke of one grant. Do not silently infer a scope union from the adopted different-Device rule.

## Explicit production gate — one intent consumed on two partitions

At `H0`, one typed intent `I` bound to D's key reaches trusted A and B through copied QR/file/text representations. Both are partitioned, both see `I` locally unused, and the owner explicitly approves identical payload/scope on each. Neither approval by itself proves that no other partition consumed `I`.

| Compliant implementation at reunion | Divergent result |
|---|---|
| A coalesces the identical signed approvals by intent ID/key/payload into one effective grant and retains both events. | D enrolls once. |
| B treats concurrent consumption as ambiguous and requires a new intent/approval. | D remains pending; no grant is accepted. |

Neither creates a *second* accepted grant or silently enrolls a different key. The accepted single-use behavior during partition, anti-replay evidence and current-controller proof are explicitly deferred to `OQ-0022/0024` in AD-40 and the recovery spec. This is a **production proof gate**: specify the canonical consume record, equivalence predicate, partition reconciliation and negative vectors for changed payload/key, expiry, replay and stale/revoked approvers before claiming interoperability. Do not label either A or B an adopted outcome.

## Evidence

- `ARCHITECTURE-SPINE.md:326-330,344-348`: current grant boundary; different-Device merge, conflict suspension, owner choice and invitation gate.
- `SPEC.md` approved Constraints and Open Questions; `recovery-cases.md` REC-F17–F20: approved conflict/restore/enrollment behavior and candidate fixtures.
- `docs/02-requirements/identity-requirements.md:55-58`; `docs/04-specifications/identity-domain-contract.md:86-88`; `docs/00-governance/open-questions.md:32-34`: cross-document boundary and open proof.
