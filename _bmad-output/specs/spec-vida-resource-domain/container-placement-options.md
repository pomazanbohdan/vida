# Container placement options — OQ-0003

**Status: proposal, not an approved VIDA rule.** The approved baseline is that Space owns membership, keys and sync; Project is normally a Container inside Space; ACL has a Container scope; a Relation never grants access to its target. This brief asks how a Resource is structurally placed. It does not decide cross-Space Relation replication or ContactCard placement.

## Business example

Olena writes one Note, “Architecture”, in a Knowledge section. Project A and Project B both need to show it. A participant in Project B has no grant to read the Knowledge section. Should the Note become a member of both projects and inherit both permission sets, or remain in one home while both projects show a link that opens only for readers who already have access?

| Option | What Olena sees | Permission consequence | Assessment |
|---|---|---|---|
| **A — one owning Container + Relations/views** | The Note has one home; authorized Project A and B readers see references to the same Note ID. | One structural inheritance path. A project link does not grant read; resource-scoped exceptions remain bounded by upper maximum capabilities and hard deny. | **Recommended for approval.** Reuse stays possible without ambiguous inherited grants. |
| **B — several structural parents** | The same Note appears as a direct child of Knowledge, Project A and Project B. | Must define whether parent grants combine by union, intersection or precedence; moving/removing one parent may silently change access. | Do not adopt without a separate, testable multi-parent authorization rule. |
| **C — no structural Container; tags/views only** | “Project” is just a filter or tag. | Removes the already approved Container permission scope, so this would require changing ADR-0001 and the product model. | Not compatible with the current baseline. |

The recommendation is an **inference for VIDA**, not a claim that another product proves its security. [Notion Page](https://developers.notion.com/reference/page) and [Block](https://developers.notion.com/reference/block) expose a parent in their object models. [Linear Projects](https://linear.app/docs/projects) states that an issue can be associated with only one project at a time, while project views and links offer other ways to present context. Neither reference defines VIDA's ACL. [OWASP Authorization](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) and [IDOR Prevention](https://cheatsheetseries.owasp.org/cheatsheets/Insecure_Direct_Object_Reference_Prevention_Cheat_Sheet.html) support checking each object access, including when its ID came from a link; they do not choose the Container model for us.

## Separate choice: is Container itself a Resource?

| Shape | Benefit | Cost/open boundary |
|---|---|---|
| Container is a typed Resource with stable ID, schema and history | A Project or Note Section could be linked, searched and audited using the same Resource machinery. | Move semantics still need a decision; edits to its content differ from Core-governed membership/ACL changes, and no App handler may override Owner governance. |
| Container is a structural ACL scope only | Keeps grouping and permissions separate from ordinary content records. | Needs its own identity, sync/history and relation target rules anyway; a Project page may require a second Resource representing its content. |

No choice here changes the approved Space boundary. A Container cannot gain access to another Space merely because a Relation points there. Copy/move inheritance, cross-Space edges and ContactCard ownership remain separate open questions.

## Questions for approval

1. For the “Architecture” Note shown in two projects, choose **one home + references** (recommended) or **true multiple structural parents**? If a Project B reader lacks Note access, VIDA hides that Note reference and its metadata; Project membership alone must not open it.
2. Should a Project or Note Section be a **typed Resource with its own stable ID/history** (recommended for consistent linking), or a **separate structural ACL scope** with another Resource for any visible content?
