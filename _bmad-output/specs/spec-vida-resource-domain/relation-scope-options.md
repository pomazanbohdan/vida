# Relation scope and cross-Space visibility — OQ-0003

**Status: decision brief, not an approved cross-Space protocol.** Same-Space typed Relations are already required. A Relation never grants target access; unreadable target identity, title, snippet, preview and backlink must not leak. Space remains the ownership, key, policy and sync boundary. See [domain cases](domain-cases.md), [ACL requirements](../../../docs/02-requirements/access-control-requirements.md) and [identity contract](../../../docs/04-specifications/identity-domain-contract.md).

## Business example

Olena owns a Note in her Personal Space and works on a Task in a team Shared Space. She links the Note to the Task. Bohdan has a scoped grant to read the Note; another teammate, Taras, may read the Task but not the Note. Olena also wants the link on her own devices even when she has not shared it with anybody.

| Shape | Where the link belongs | What others receive | Assessment |
|---|---|---|---|
| **A — private overlay** | A relation record in Olena's Personal Space, bound to her active Persona and the two Resource IDs. | Nobody else receives this relation merely by reading the team Task. | Useful default for personal organization; not a shared project link. |
| **B — explicitly published scoped relation** | A relation operation in the source Task's Space, visible only to an authorized audience; target Resource remains in its own Space. | Bohdan may see/open after both source and target checks. Taras receives no target locator or existence hint. | **Recommended product direction**, conditional on proving selective sync/encryption and revocation with equal peers. Publishing is distinct from sharing the Note. |
| **C — global Relation graph outside Spaces** | One graph replicated independently of both Spaces. | Graph membership itself can disclose private resources or correlate Personas. | Conflicts with the approved Space/privacy boundaries; not recommended. |

For B, two audience policies remain to choose: **B1** publish only after every reader of the source Task has target access; or **B2** allow a subset such as Bohdan and hide the relation from Taras. B2 preserves fine-grained sharing but requires a proven audience-scoped operation/projection rather than putting a plaintext target ID in the team-wide log. A mere UI filter is insufficient because a peer may inspect synchronized records. Any actor creating a published link needs authority to edit the source and inspect the target; exact command/receipt fields remain a protocol decision.

A backlink on the target is not automatically a second, target-owned Relation or a grant to inspect the source. A device that can read both endpoints may derive it locally. To make a backlink visible to other target readers, the source side would need a separately authorized publication/projection; neither a private overlay nor a source-only link silently reveals the team Task.

Reference boundary: [Linear private-team sharing](https://linear.app/docs/private-teams) illustrates explicit item sharing and warns that integrations/webhooks can leak private titles. [GitHub cross-repository references](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-an-issues-only-repository) may show minimal reference information to unauthorized viewers; VIDA's approved `RES-11` no-existence-hint rule is stricter. [OWASP Authorization](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) and [IDOR Prevention](https://cheatsheetseries.owasp.org/cheatsheets/Insecure_Direct_Object_Reference_Prevention_Cheat_Sheet.html) require per-object checks and do not choose a replication design.

## Questions for approval

1. Should a cross-Space link start as **Olena-only** and become shared only after a separate “publish link” action (recommended), or should linking the Note to a team Task immediately publish the link to eligible teammates?
2. If only Bohdan has access to the linked Note, may he see its link on the team Task while Taras sees no trace (**B2**, recommended if conformance proves it), or must Olena first share the Note with every Task reader (**B1**)?
