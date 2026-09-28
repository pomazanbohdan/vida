# PRD/UX → Architecture reconciliation (2026-09-22)

Scope: VIDA Release 1 PRD FR-1–36 and the current EXPERIENCE.md discovery spine. This is a coverage audit, not a new product decision. The Architecture Spine's AD rules bind shared implementation; the linked requirements and specifications define detailed behavior.

| PRD capability | Architecture owner / invariant | UX surface | Open proof |
|---|---|---|---|
| FR-1–4 Persona, Devices, recovery, Contacts | `vida-core` identity/Contacts and `vida-runtime` lifecycle; AD-3, AD-11, AD-20, AD-37, AD-38 | First run, Persona/profile, Contacts | OQ-0024 recovery/key custody and connector conformance |
| FR-5–8 Spaces, roles, revocation, scoped share | SpaceMembership, Core ACL and Resource graph; AD-6, AD-16, AD-21, AD-36 | Shared Space, membership, share preview | OQ-0031 authority order; OQ-0053 freshness proof; denied-relation fixtures |
| FR-9–11 chat, forum, search | Common signed operations and per-Space resources; AD-4, AD-13, AD-27 | Messenger, Forum, Search | OQ-0065 publication encoding; forum moderation contract |
| FR-12 calls | Core-bound owning call context, exclusive answer proof/key epochs and replaceable MediaSessionAdapter; AD-35 | Call surface | OQ-3/OQ-0072 authority and physical E2EE/media selection; OQ-4 quality budgets |
| FR-13–15 Notes and collaboration | Core Resource/log and signed Space/Resource/Document binding; AD-7, AD-13, AD-36 | Knowledge/editor/history | OQ-2 CRDT/editor prototype and cross-Space replay fixture |
| FR-16–18 Project work | Core operations and comparable/incomparable conflict rules; AD-19, AD-22, AD-23 | Project list/board/task | OQ-0033/0034 authority, merge and safe frontier |
| FR-19–21 Files, relations, global search | BlobStore + Core Resource/Relation/ACL projection; AD-7, AD-11, AD-13, AD-36 | Files, relation links, Search | Blob manifest/GC and search/relation ACL fixtures |
| FR-22–26 offline, sync, conflicts, effects | Runtime single mutation path; AD-4, AD-13, AD-18–28; SPEC-OPERATION-FINALITY-001 | Activity/sync, conflict comparison | Finality vectors; OQ-0033/0034/0045; G1 resource baselines |
| FR-27–28 approvals and automation | Core semantic class and process policy; AD-24, AD-25, AD-31, AD-34 | Approval status, Apps/settings | OQ-0045 execution/idempotency and OQ-0048 rule identity |
| FR-29–32 packages and schemas | AppPackage runtime, dedicated Space, no-grant dependencies, forward activation; AD-29–33, AD-39 | Apps/settings, activation/update | OQ-0037/0040/0056 compatibility, migration and denied-cross-App fixtures |
| FR-33–36 onboarding, localization, diagnostics, export | Platform shells + Core contracts + Conformance Plane; AD-11, AD-12, AD-15, AD-17, AD-20 | First run, settings, diagnostic preview | Locale ownership, security/operational release gates |

## Conclusions

- The 2026-09-22 PRD does not reopen adopted Iroh, peer equality or Core ownership decisions.
- ADR-0020 supersedes the Spine's older Windows-toolkit ambiguity: Flutter Windows is in Release 1; WinUI 3 is later.
- Approved `SPEC-OPERATION-FINALITY-001` supplies receipt semantics. OQ-2/OQ-3 still choose CRDT/editor and media profiles by measured conformance.
- OQ-4 G0 measurement method is closed; G1 physical baseline and G2 numeric budgets remain work for the vertical slice and feature-complete gate.
- EXPERIENCE.md is a behavior draft. Its navigation placement and DESIGN.md tokens are not yet architecture invariants.

## Next document handoff

After OQ-2/OQ-3 evidence and UX review, generate epics/stories from PRD FR IDs and this architecture mapping. Every story must cite its contract and acceptance fixture; do not elevate a candidate library or a discovery UX assumption into an approved decision.
