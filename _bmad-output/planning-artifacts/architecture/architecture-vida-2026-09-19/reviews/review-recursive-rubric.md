# Independent BMad rubric review — recursive status/file conflict resolution

## Gate verdict

**Revise two semantic phrases before handoff; the accepted recursive no-winner outcome is otherwise coherent and safely implementation-gated.** AD-22/ADR-0017/`REQ-SYNC-009` preserve authority validation, equal device peers, both alternatives and payloads, and a new resolution based on current heads. The update does not impose this conflict policy on text or other operation families. The spine, however, says that any *distinct operations* re-conflict even if their decisions are semantically identical, while the approved input says *different resolutions*; it also sends comparable file resolutions to task-status-only AD-19.

## Critical findings

None in the reviewed contour.

## High findings

### H1 — Distinct resolution operation IDs do not necessarily mean incompatible outcomes

- **Evidence:** AD-22 says “If two **distinct** authorized resolution operations are independently authority-accepted offline and their acceptances remain incomparable, reconciliation creates a new explicit multi-value conflict.” ADR-0017 §Decision says two actors resolved the conflict **по-різному**; `REQ-SYNC-009` says two different resolutions. Both ADR-0017 and AD-22 explicitly defer *semantically identical resolution behavior* to `OQ-0033`/`OQ-0034`.
- **Divergence:** two peers may each choose the same task status or the same file digest in different signed operations. One implementation will show a second unresolved conflict because operation IDs differ; another will collapse/accept the identical semantic result. AD-22's current `MUST` effectively decides this case while claiming it is deferred.
- **Disposition:** **Autofix.** Scope the recursively required no-winner state to **different/incompatible semantic resolution outcomes** whose authority acceptances are incomparable. State that same-outcome resolutions retain both signed operations/audit evidence and their exact collapse/receipt semantics remain `OQ-0033`/`OQ-0034`; do not silently call them an unresolved business conflict. If the product actually wants every distinct operation to conflict, that needs explicit user approval.

### H2 — Comparable file resolutions cannot “follow AD-19” literally

- **Evidence:** AD-22 says comparable resolution acceptances follow AD-19. AD-19's Binds and Rule apply to the **task-status operation family**. File-replacement comparable-acceptance behavior is specified in `REQ-SYNC-007` and ADR-0017 §Decision, not AD-19.
- **Divergence:** an implementer may treat AD-19 as a newly broadened file rule, while another follows `REQ-SYNC-007` and file-specific retention/revision semantics. The current cross-reference weakens the intended per-operation-family boundary.
- **Disposition:** **Autofix.** Say comparable task-status resolutions follow AD-19/`REQ-SYNC-004`, while comparable file resolutions follow the first-accepted rule in `REQ-SYNC-007`/ADR-0017, subject to file-specific retention and authorization. Alternatively introduce a generic first-comparable-acceptance AD only with an explicit decision; do not silently broaden AD-19.

## Medium findings

### M1 — Recursive fixture is in companion specs but not the spine gate

- **Evidence:** spine Implementation Gate for independent `SyncLog` enumerates initial two-variant status/file conflict, resolution over both branches, and blob retention, but not **two incomparable accepted resolution operations**. ADR-0017 acceptance evidence, `SPEC-SYNC-LOG-001` and `device-sync-session.md` do include that recursive case.
- **Disposition:** **Autofix.** Extend the spine gate's required fixture list to include two divergent accepted offline resolutions, delivery permutations, new unresolved conflict over both *current resolution heads*, current-rights recheck, and both file resolution payloads/pins surviving restart/GC. This keeps the normative AD from outrunning implementation proof.

### M2 — Recursive file payload retention is covered but should use current-head wording consistently

- **Evidence:** AD-22 binds `BlobStore` and keeps both first-level accepted file payloads pinned until resolution; `BlobStore` acceptance criterion retains both recursive resolution payloads/pins. `REQ-SYNC-009` requires both resolutions and required payloads recoverable within rights/retention.
- **Disposition:** **Pass with wording polish.** For AD-22, make clear that each unresolved conflict level protects payloads referenced by its *current competing heads*, and that a resolution operation never permits premature GC of an accepted head still needed for reconciliation/audit under policy. Exact retention frontier remains `OQ-0034`.

### M3 — Authority proof and acceptance order remain intentionally open

- **Evidence:** AD-22 applies recursion only after both decisions have passed applicable authority policy and remain incomparable; invalid/revoked pending candidates do not become accepted variants. The spine blocks domain acceptance on `OQ-0033` and independent `SyncLog` on `OQ-0034`; the open-question register names proof, wire, late third branch and same-outcome semantics.
- **Disposition:** **Safely gated.** Do not interpret local durability, device receipt or transport delivery as authority acceptance. No new architecture choice is needed in this review.

## Good-spine checklist

| Criterion | Result | Note |
|---|---|---|
| Fixes real divergence | **Partial pending H1** | Recursive divergent resolutions correctly produce no winner; “distinct” overreaches same-result case. |
| Rule enforceable and prevents stated divergence | **Partial pending H2** | Comparable file path points to task-only AD-19. |
| Deferred cannot cause divergence | **Pass with gate** | Proof/wire/same-outcome/third-branch semantics are explicitly gated; add recursive fixture to gate. |
| Ratifies existing authority/device rules | **Pass** | No device arbiter, client clock, ACK or CRDT tie-break becomes business authority. |
| Covers approved requirements | **Pass with H1** | `REQ-SYNC-009` is represented, but “different resolutions” needs consistent wording. |
| Payload durability | **Pass** | BlobStore preserves recursive resolution payloads and pins within policy. |
| Other operation families | **Pass** | Text auto-merge and unrelated operation-family policies remain separate. |

## Preservation and handoff

- Keep AD-19's first **comparable** acceptance rule for task status and ADR-0017/`REQ-SYNC-007` for file replacement.
- Keep AD-22's no-winner conflict for genuinely incomparable incompatible first-level variants and recursively for *different* authorized resolution outcomes.
- Keep rejected/expired-grant candidates recoverable under rights policy but outside the accepted shared conflict.
- Keep each subsequent resolution as a new authorized operation causally covering both current heads; receiving/replaying it must not rerun the original command/effect.
- Resolve H1/H2 in wording and add M1 fixture. Remaining proof, wire, same-result and late-branch mechanics may stay under `OQ-0033`/`OQ-0034`.
