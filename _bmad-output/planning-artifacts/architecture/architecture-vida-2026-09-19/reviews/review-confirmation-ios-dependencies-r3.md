# Reviewer gate — architecture rubric r3

Verdict: PASS with three explicit open decisions.

- AD-31 separates local durability, transport delivery, authority decision and requester receipt; each transition is implementable and testable.
- `auto-approve` still executes Core authorization/precondition/semantic checks and therefore cannot mean blind acceptance.
- Governance is invariant: package logic cannot change Owner/Admin, membership revocation, key revocation/rotation or control-log rules.
- `compatible-auto` now has a negative boundary: dependency expansion, new data scope, breaking converter or mandatory capability cannot silently auto-activate.
- iOS wording distinguishes declarative packages, Apple 4.7 mini apps and store-delivered native features; it does not claim Rhai/Wasm approval.
- Cross-App composition is preserved through versioned typed contracts, while OWASP least privilege blocks implicit unrestricted data access.
- Open gates remain explicit: Space/App cardinality (`OQ-0070`), v1 approval profiles (`OQ-0071`) and iOS 4.7 profile (`OQ-0068`).

