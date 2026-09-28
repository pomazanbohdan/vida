# Approval process conformance cases — planned, not executed

Approved behavior comes from `REQ-EFFECT-012–014` and `REQ-FINALITY-001/007/011`. The [operation-semantics catalog](../../../docs/04-specifications/operation-semantics-catalog.md) is a draft implementation map: its examples illustrate Core classes but do not select `OQ-0033` authority mechanics. `APR-F13` is decision-gated until that proof exists.

| ID | Setup / action | Expected observable result |
|---|---|---|
| APR-F01 | AppPackage declares recommended defaults for two named processes in one AppInstance. | Each process exposes its own effective policy; default is visible as a recommendation, not a fixed minimum. |
| APR-F02 | Space Admin selects `single-approver` for Process A and `2-of-3` for Process B; a normal Member tries the same configuration. | Only the authorized Admin action changes the targeted process; Process B is unchanged when A changes; Member cannot mutate policy. |
| APR-F03 | Requester creates a candidate offline and later obtains transport/replication ACK only. | UI shows local save or synchronization as applicable, but not business confirmation; candidate remains pending until matching authority outcome. |
| APR-F04 | Set Process A to `single-approver`; one authorized approver votes once and a non-approver votes. | Eligible decision can satisfy the configured profile; unauthorized vote does not change tally or reveal protected content. |
| APR-F05 | Configure `manager → finance` stages; finance tries to approve before manager. | Later stage cannot close the process first; Core records authorized decisions and outcome only after the configured stage order is satisfied. |
| APR-F06 | Configure `2-of-3` among three distinct Personas; one approver votes from phone and laptop, another Persona votes once. | Same Persona contributes one logical vote; outcome requires the second distinct eligible approval. |
| APR-F07 | Configure `3-of-3` as `M=N`; only two of three eligible Personas approve and no rejection/expiry outcome is issued. | Result remains pending until the third eligible approval; no hidden lower threshold applies. |
| APR-F08 | An approver votes for revision R; requester changes protected data to R+1 before completion. | R vote stays in history but cannot satisfy R+1; the new revision needs its own applicable decisions. |
| APR-F09 | Replay a signed vote or outcome, or submit one for the wrong `RequestId`/revision/frontier. | Duplicate is idempotent and wrong binding is rejected; no second effect or extra vote is produced. |
| APR-F10 | Authority issues signed `accepted`, `rejected` or explicitly policy-driven `expired` outcome; requester is offline then reconnects. | Matching terminal outcome is durable and arrives as a separate synchronized fact; age alone is not an implicit expiry rule. |
| APR-F11 | Configure `auto-approve`, then submit a request with revoked rights, stale precondition or invalid semantic class. | No manual vote is requested, but failed Core checks prevent accepted outcome. |
| APR-F12 | Package attempts to use approval configuration to remove an Owner, change key revocation or replace the Core resolver. | Governance and resolver boundaries remain fixed; package/Admin configuration cannot grant that behavior. |
| APR-F13 | An `exclusive-claim` process uses `auto-approve` while two incompatible requests contend for one resource. | At most one authoritative accepted outcome may exist for the same protected frontier; a partition without serialized authority stays pending. Exact serverless proof awaits `OQ-0033`. |

Business examples are illustrative, not bundled Release-1 Apps: one reviewer signs off a document (`single-approver`); a project decision needs lead then finance (`sequential-stages`); two of three committee members approve a proposal (`M-of-N`). A resource reservation adds `exclusive-claim` scarcity on top of its approval profile, never a shortcut around it. Security review must include denied, stale and cross-Space attempts under [OWASP Authorization](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html); OWASP does not validate the unfinished peer authority protocol.
