# AD-18 / ADR-0003 — currentness and security-source review

Date: 2026-09-20. Scope: [ARCHITECTURE-SPINE.md](../ARCHITECTURE-SPINE.md) AD-18, [ADR-0003](../../../../../docs/03-architecture/decisions/ADR-0003-offline-revocation.md) §7, [REQ-ACL-018](../../../../../docs/02-requirements/access-control-requirements.md), associated NFR and open questions. Lens: official OWASP guidance versus VIDA's product-specific offline-candidate policy. This review does not change the decision or close open implementation contracts.

## Verdict

**Conditional pass; one P1 document blocker.** The adopted rule correctly separates *indefinite retention of an unaccepted local candidate* from a credential, current authorization, or guaranteed eventual acceptance. The normative acceptance path requires fresh Space/object/action authorization and operation preconditions. Official OWASP sources support those authorization checks but prescribe **no numeric age limit** for a pending offline mutation. ADR-0003 still contains one contradictory legacy sentence about a risk-class-based `maximum lifetime`; remove or narrow it before treating the ADR as internally consistent.

## Findings

### P1 — Legacy risk-class `maximum lifetime` contradicts the adopted rule

ADR-0003 line 133 states: “Точний maximum lifetime визначається risk class scope”. This is in the same section where line 127 explicitly cancels the 30-day / 7-day / 24-hour risk-class candidate windows. It conflicts with AD-18, REQ-ACL-018 and resolved OQ-0019/OQ-0054. Implementers could interpret it as permission to age-expire, hide or quarantine a valid candidate, making cross-client behavior inconsistent. Remove that first clause. Retain the separate security point that a write grant/capability may expire and that an indefinitely valid write grant is not the default; explicitly identify its lifetime as a *credential lifetime*, not a *candidate lifetime*. No new numeric policy is implied.

### Verified — Authorization is checked at acceptance, not at authoring time alone

AD-18 and ADR-0003 line 127 require current Space/object/action rights, control state, grant/epoch, signature, schema, causal and business preconditions at the actual acceptance point. They distinguish creation and delivery from `authority.accepted`, and place invalid/revoked operations in recoverable quarantine without merging. This fits [OWASP Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) (“Deny by Default”, “Validate the Permissions on Every Request”) and [OWASP API1:2023 Broken Object Level Authorization](https://api-security.owasp.org/editions/2023/en/0xa1-broken-object-level-authorization/) (authorization for the requested action on the specific object in each relevant function). Those web/API guides do not literally define a distributed offline acceptance protocol; applying their per-request/object principle to authority acceptance is a VIDA architectural inference.

### Verified — No OWASP mandate for a pending-candidate TTL

ADR-0003 line 131 correctly labels the absence of age-only expiry as a VIDA product decision, not a numerical OWASP requirement. Neither of the OWASP authorization sources above gives a maximum pending-offline-operation age. [OWASP MASWE-0024](https://mas.owasp.org/MASWE/MASVS-AUTH/MASWE-0024/) addresses session expiry/termination and risk-appropriate session timeouts; it must not be cited as a candidate-age rule. Separate local storage, logout/session, offline *read* and rights-reconciliation controls in ADR-0003 §7 / AD-16 remain distinct; AD-18 does not decide OQ-0051's interval or offline-read behavior.

### Implementation gates, not defects in AD-18

- An expired grant/old epoch must never be treated as a still-valid credential. Candidate content stays recoverable; acceptance requires a defined reauthorization/re-sign/rebase path or rejection with recoverable evidence. OQ-0033/OQ-0034 remain open; AD-18 and the spine explicitly block production acceptance semantics until they close.
- Personal Space's Owner-policy local acceptance is scoped to an active local identity context. It is not a blanket bypass for shared-Space authority or for membership, role, permission, policy and key-management changes.
- “No age-only expiry” is a retention/decision rule, not a promise that a six-month-old mutation will be accepted. Current rights, causal conflict and business preconditions may reject it. Client wall-clock timestamps cannot prove entitlement.

## Required correction and proof

1. Replace ADR-0003 line 133's `maximum lifetime` clause with an explicit contrast between candidate retention and grant/capability expiry. Do not reinstate 30-day / 7-day / 24-hour candidate windows by implication.
2. In OQ-0033/OQ-0034 acceptance fixtures, cover: six-month candidate with still-valid rights and unchanged grant/epoch; revoked member; expired grant but still-current member rights requiring a safe renewal path; old epoch and key rotation; causal/business conflict; duplicate replay; forged timestamp or clock rollback; shared control-plane mutation staged offline but ineffective until authority acceptance; persistence across restart/restore. For each, assert both acceptance outcome and recoverability of an unaccepted local candidate.

## Primary sources checked

- [OWASP Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html), authorization on each request and default denial.
- [OWASP API1:2023 Broken Object Level Authorization](https://api-security.owasp.org/editions/2023/en/0xa1-broken-object-level-authorization/), object/action authorization on every relevant endpoint.
- [OWASP MASWE-0024 — Improper Session Invalidation](https://mas.owasp.org/MASWE/MASVS-AUTH/MASWE-0024/), session lifecycle guidance, not a pending-mutation TTL.
