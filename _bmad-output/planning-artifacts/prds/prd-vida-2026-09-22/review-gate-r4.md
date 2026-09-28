# VIDA Release 1 PRD — targeted gate recheck r4

**Review date:** 2026-09-23. **Scope:** current `prd.md` and `addendum.md` against the two r3 reviews and the cited approved requirements/architecture/specifications. This is a read-only independent review of source decisions, not an approval of unbuilt prototypes.

## Verdict

The five assigned r3 document inconsistencies are repaired or explicitly represented as open gates. **No residual high-severity contradiction was found in those five areas.** The PRD correctly remains `draft` (`prd.md:3`): OQ-2 and OQ-3 need reproducible CRDT/editor and E2EE-media selections (`prd.md:560–561`), the other R-2 contracts need approved profiles and proof (`prd.md:548,555`; `addendum.md:98–109`), and the unresolved `security-auto` classification prevents a full Release-1 update-mode proof (`prd.md:398,569`). This review does not assert implementation or release readiness.

## Recheck of assigned findings

| r3 finding | Current evidence | Result |
|---|---|---|
| Vertical-slice overclaim / SM-1 | `prd.md:481–483` now limits the slice to named integration behavior and explicitly excludes Guest sharing, Forum, voice, eight-person calls, cursors, File Conflict variants and irreversible effects; `prd.md:487–497` names separate feature/conformance suites; `prd.md:528` says SM-1 does not replace full FR proof. | Repaired as a documentation coverage claim; actual suite results remain future evidence. |
| FR-30 modes | `prd.md:395–400` makes `compatible-auto`, `manual` and `pinned` observable and defines failed-preflight behavior. `security-auto` preserves authorization/preflight and explicitly defers semantic eligibility to OQ-0043; `prd.md:569` adds OQ-11 before a security-auto release claim. This agrees with `docs/02-requirements/app-package-requirements.md:43,48` and `docs/04-specifications/app-package-runtime-baseline.md:65`. | Prior blanket untestability is repaired; see residual R1. |
| Iroh selection | `addendum.md:23` now states accepted Iroh core 1.2 and separates exact lockfile/SBOM pins; this matches `docs/03-architecture/decisions/ADR-0005-iroh-transport-foundation.md:33–35,45–52`. | Repaired. |
| Current fixtures | `prd.md:491–492,560–561` and `addendum.md:74,91,106,108` reference calls F01–F17 and editor F01–F14. These match `docs/04-specifications/e2ee-calls-conformance.md:60–64`, `docs/04-specifications/crdt-editor-conformance.md:56–75` and both fixture manifests. | Repaired. |
| R-2 inventory; ContactCard/OQ-0048 | `prd.md:548,555` says OQ-2/3 are necessary, not sufficient; `addendum.md:98–109` enumerates operation envelope, storage/GC, collaboration, FFI, calls and AppPackage contracts with remaining proof. `prd.md:94` does not preselect ContactCard placement (`docs/00-governance/open-questions.md:13`). `addendum.md:56` specifies one logical derived resource while leaving ID derivation/executor/delayed rule to OQ-0048 (`docs/02-requirements/effect-execution-constraints.md:26,35`; `docs/00-governance/open-questions.md:58`). | Repaired; owning scope and derived-resource mechanism remain explicit open questions. |

## Material residual

**R1 — `security-auto` is a tracked but unresolved Release-1 behavior, not a selected algorithm.** FR-30 says first-party Core Apps receive compatible/security updates by default (`prd.md:401`), while eligibility and automatic-activation semantics remain open (`prd.md:398,569`; `docs/00-governance/open-questions.md:53`). Release-1 conformance cannot mark the `security-auto` mode passed until OQ-11 fixes classification and positive/negative update-policy fixtures. Do not infer that a publisher's “security” label alone authorizes automatic activation. This is a decision/proof gate, not a reason to invent rules in the PRD review.

**Gate disposition:** continue bounded UX, architecture and prototypes; do not claim PRD finality, broad implementation fan-out or public-release readiness from this r4 review alone.
