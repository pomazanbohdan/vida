# Independent rubric and input reconciliation — AD-18 offline candidates

## Gate verdict

**Revise before finalizing the AD-18 update.** AD-18, `REQ-ACL-018`, `SPEC-SYNC-LOG-001`, and `OQ-0019`/`OQ-0054` mostly preserve the accepted rule: an offline candidate has no age-only expiry, but current authority, rights, grant/epoch, signature and operation preconditions govern acceptance. A contradictory sentence remains in accepted ADR-0003. Two scope ambiguities could let clients implement different handling for owned shared Spaces and staged control operations.

## Critical

### C1 — ADR-0003 still imposes a risk-class maximum lifetime

- **Evidence:** ADR-0003 §7 first says the prior `standard` 30-day, `protected` 7-day and `critical` 24-hour candidate windows are cancelled and age alone cannot reject a candidate (line 127). But line 133 still says: **“Точний maximum lifetime визначається risk class scope; довгоживучі безстрокові write grants `MUST NOT` бути default.”** AD-18 line 193, `REQ-ACL-018` and resolved `OQ-0019`/`OQ-0054` all reject an age-only candidate maximum.
- **Divergence:** one implementation applies a risk-class expiry to pending edits; another keeps them pending indefinitely. Both can cite an accepted paragraph of ADR-0003.
- **Disposition:** **Autofix.** Remove or replace the stale first clause with an explicit separation: *the candidate itself has no age-only maximum; individual credentials/grants may expire and are revalidated at acceptance*. Preserve the second clause about not defaulting to indefinite write grants if still desired. Do not reintroduce a class-based candidate TTL.

## High

### H1 — “Own Space” was narrowed to `Personal Space` without an explicit equivalence decision

- **Input:** parent task states the user accepted unlimited candidate age if current rights exist **or own Space**.
- **Evidence:** AD-18 says “Own Personal Space can accept under its Owner policy locally”; ADR-0003 §7 and `REQ-ACL-018` similarly mention Personal Space. In a shared Space created/owned by the actor, AD-18 instead says the candidate requires applicable Space authority policy.
- **Divergence:** “own Space” may mean every Space in which the actor is Owner, whereas the documents grant special local authority only to Personal Space. An Owner of an offline shared Space might expect their edits to be accepted locally; another implementation would stage them until an external authority receipt. Neither age nor ownership alone should bypass current Space policy, but the owner case needs an unambiguous rule.
- **Disposition:** **Discuss if user intent is not already explicit; otherwise autofix wording.** State whether “own Space” means strictly Personal Space or any Space where the actor is current Owner, and how shared-Space authority/quorum is preserved. Current-rights acceptance in shared Spaces should not become an implicit bypass of `OQ-0033`.

### H2 — AD-18 may exclude staged control mutations from no-age protection

- **Evidence:** AD-18 binds “every schema-driven App” and says “offline **domain** mutation candidate” can remain pending indefinitely; its final sentence requires authority acceptance for membership/role/policy/key changes. ADR-0003 §7 and `REQ-ACL-018` say **all mutation classes may be staged** and do not limit the no-age rule to data operations.
- **Divergence:** a client could age-expire a staged permission or membership command while retaining an old Notes edit; another would retain both candidates pending current-rights checks. Acceptance must remain authority-gated, but staging durability should have one meaning.
- **Disposition:** **Autofix.** Say “offline mutation candidate, including staged control mutations” for the no-age retention rule, then retain the explicit “no control effect before authority acceptance” restriction. If control candidates are intentionally excluded, that is a new policy choice requiring user review.

## Medium

### M1 — ADR-0001 still describes OQ-0019 as open

- **Evidence:** ADR-0001 line 109 says “maximum offline lease lifetime залишається відкритим у `OQ-0019`”, but `OQ-0019` is resolved/superseded by the no-age-only decision.
- **Disposition:** **Autofix traceability.** Refer to ADR-0003 §7 and explain that candidate age has no hard maximum; individual grant expiry and authority checks remain separate.

### M2 — Re-sign/rebase across a revoked or expired grant is correctly gated, but recovery semantics need exact fixtures

- **Evidence:** ADR-0003 §3 rejects an operation not accepted before effective revocation; §8 requires a new grant ID on re-grant. AD-18 and `SPEC-SYNC-LOG-001` defer old-envelope re-sign/rebase to `OQ-0033`/`OQ-0034`, with an implementation gate at spine line 220.
- **Risk:** “current rights exist” does not make an old signed envelope valid under a revoked/expired grant. An independent client could silently change the signed operation or reuse an ID, while another quarantines it forever.
- **Disposition:** **Safely gated.** Keep the no-age candidate; require eventual fixtures for authorization under the new grant, stable logical intent/operation identity, exactly-once apply, quarantine/recovery visibility and conflict resolution. No need to choose the cryptographic rebase mechanism in AD-18.

### M3 — Candidate age and offline-read reconciliation are separated correctly

- **Evidence:** AD-18 references `OQ-0051`; AD-16, ADR-0003 §7 and `REQ-ACL-016` keep the uniform rights-reconciliation interval and unknown offline-expiry behavior separate from mutation acceptance. `OQ-0054` expressly says it does not close `OQ-0051`.
- **Disposition:** **Pass.** Do not infer an offline-read timeout from the candidate rule or vice versa.

## Good-spine checklist

| Criterion | Result | Note |
|---|---|---|
| Prevents age-only divergence | **Fail pending C1** | Accepted ADR still contains a contradictory risk-class maximum sentence. |
| Enforceable AD rule | **Mostly** | AD-18 has clear acceptance checks and implementation gate; “domain” scope needs clarification. |
| Preserves inherited revocation/security | **Pass with gates** | Effective revocation, new grant/epoch, control-before-data and `OQ-0033`/`0034` are not bypassed. |
| Captures user intent | **Partial** | Unlimited pending age is captured; “own Space” may be narrower than approved. |
| Deferred items cannot diverge | **Pass with gate** | Re-sign/rebase and conflicts block production independent implementations. |
| Read/mutation distinction | **Pass** | `OQ-0051` remains open independently. |

## Finalization condition

Fix C1 and H2. Resolve whether H1's “own Space” includes current Owners of shared Spaces or only Personal Space; do not silently infer this from the existing Personal-Space wording. Update ADR-0001 traceability. The accepted no-age principle can then remain final while detailed re-sign/rebase and conflict mechanics stay gated by `OQ-0033`/`OQ-0034`.
