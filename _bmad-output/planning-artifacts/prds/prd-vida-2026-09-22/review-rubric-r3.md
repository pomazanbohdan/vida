# PRD Quality Review — VIDA Release 1 (r3)

**Reviewed:** `prd.md` (564 lines) and `addendum.md` (109 lines), current state 2026-09-23. **Rubric:** BMad PRD Quality Rubric. **Gate:** fair for a chain-top launch PRD; bounded UX/architecture work can continue, but implementation fan-out still depends on the explicitly open OQ-2/OQ-3 and the findings below.

## Overall verdict

The PRD has a specific local-first connected-resource thesis, named journeys, explicit Release-1 boundaries, and unusually clear separation of local save, synchronization, delivery, and business outcome. Its main remaining weakness is proof precision: §6.2 credits a narrow vertical slice with whole-feature validation, while FR-30 names update modes without defining their observable behavior. OQ-4 is **not** a current contradiction: its G0-before-fan-out and G1→G2-before-feature-complete sequence agrees across the PRD and addendum.

## Decision-readiness — adequate

The document makes real choices: Release 1 includes all three Core Apps and calls across Android/iOS/Windows (§§1, 5–6), excludes hosted/browser/Marketplace features (§5), and blocks release rather than silently cutting scope if a capability proof fails (§6.4). The OQ table names owners, evidence, and gates (§10). OQ-2/OQ-3 remain legitimate before-fan-out blockers, not rhetorical questions; operational ownership and independent assurance remain later release gates (R-3/R-4, OQ-5/OQ-6). The addendum preserves technical alternatives instead of presenting a selected CRDT or media stack as a settled product choice (addendum §§C, G, I).

OQ-4 is internally consistent: NFR-10 (§7, line 508) puts the G0 method/profiles before implementation fan-out, requires a physical-device G1 baseline before numerical G2 budgets, and makes G2 mandatory before feature-complete; §10 (lines 546, 553) repeats that gate; addendum §I (line 93) defers numbers until G1. The older r2 review's contrary finding should not be carried forward.

## Substance over theater — strong

The vision names a distinct resource/Space/access/sync job rather than generic productivity benefits (§§1–2), and UJ-1–UJ-5 exercise materially different decisions (offline setup, multi-device, shared Project, scoped Note share, E2EE call). NFR-3/4/8/10/12/14/15 express product-specific durability, convergence, openness, staged measurement, and conformance gates (§7); they are not just adjectives. The comparator rationale is deliberately bounded rather than a novelty claim (§1.1; addendum §J).

## Strategic coherence — adequate

Messenger, Notes, Project, Files and Relations follow one connected-context thesis (§§1, 4, 6.4). Primary SM-1/SM-2 verify technical viability, and counter-metrics reject engagement/telemetry growth that would contradict privacy (§8). SM-3 is correctly opt-in rather than behavioral tracking, but its unresolved target weakens the product-value release decision.

### Findings

- **[medium] Connected-context success has no decision threshold** (`prd.md` §8, lines 519–522; §10 OQ-8, line 557) — “completion та assistance rate” is named, but no task definition, sample/method, baseline decision rule, target, or date by which the target becomes a release gate is fixed. OQ-8 says only “before PRD metric target freeze,” which is not tied to a downstream gate. *Fix:* preregister the moderated Chat→Note/Task/File task and denominator, set a target from the baseline, and tie approval of that target to a named gate before public-release readiness; keep individual telemetry off.

## Done-ness clarity — thin

All FR-1–FR-36 have at least one “Перевірні наслідки,” and important edge states are explicit (FR-22–FR-26). Two acceptance gaps nevertheless permit a false-green result: the vertical-slice coverage statement is broader than its described scenario, and the four update modes are listed without enough behavior to test each mode.

### Findings

- **[high] Vertical slice overclaims whole-FR proof** (`prd.md` §6.2, lines 474–478 versus FR-9 lines 191–198, FR-12 lines 218–226, FR-13/14 lines 230–245) — the slice names Chat, Note, Task, File, offline/reconnect and “E2EE call,” then says it “доводить” whole FR-9, FR-12–FR-14 and other multi-part FRs. That scenario alone does not prove voice messages, reactions, delete/tombstone, group calls up to eight, structured Notes or live cursors. *Fix:* change the coverage statement to the specific behaviors actually exercised and attach separate named feature/conformance cases for every remaining consequence; require both suites before declaring an FR passed.
- **[high] Update modes lack observable acceptance behavior** (`prd.md` FR-30, lines 389–396; `addendum.md` §E, line 58) — `compatible-auto`, `security-auto`, `manual`, and `pinned` are named, but only compatible-auto and first-party defaults have consequences. A team cannot verify what security-auto may change, whether manual/pinned ever install automatically, how a user changes modes, or how a failed update is surfaced. *Fix:* specify a short behavior table for all four modes, including trigger, user confirmation, compatibility/data-access boundary, failure state, and interaction with FR-29 preflight; keep package/signature mechanism in the addendum/spec.
- **[medium] Non-Owner role presets are not testable from this PRD handoff** (`prd.md` FR-6, lines 158–167; §6.3, line 483) — the document names Manager, Contributor, Commenter and Viewer, but states no allowed/denied domain actions for them and does not link the exact normative permission matrix in the FR. “Access-control suite доводить FR-6” is weaker than a trace to expected behavior. *Fix:* add an explicit normative link and version to the approved access-control requirements/matrix beside FR-6, plus a small number of role-specific positive/negative acceptance cases; do not duplicate the entire matrix in the PRD.

## Scope honesty — adequate

§5 is unusually explicit about non-goals, and §6.4 says failed proof blocks release instead of silently lowering scope. Draft status and OQ-2/OQ-3 correctly signal that the document is not ready for implementation fan-out (§10). The assumptions index needs bookkeeping repair, but there is no evidence that OQ-4 is being used to hide an invented budget.

## Downstream usability — adequate

The glossary and stable ID families make source extraction practical (§3; FR-1–FR-36; UJ-1–UJ-5; NFR-1–NFR-15; SM-1–SM-7 and SM-C1–SM-C3; OQ-1–OQ-10). Named journeys carry protagonist context inline. The technical addendum separates Iroh, CRDT, ABI, migration and media choices from product behavior; referenced PRFAQ, research paths and platform-NFR file resolve in the current workspace. The §6.2 overclaim above must be corrected before translating FR coverage into stories or release tracking.

## Shape fit — strong

This is a multi-surface consumer/team product feeding UX, architecture and story work. Five named journeys are load-bearing but do not swamp the feature groups; FRs are grouped by actual product capabilities; security, platform conformance, openness and release gates match the launch stakes. The addendum holds technical detail that would otherwise turn the PRD into an architecture document.

## Mechanical notes

- FR IDs 1–36, UJ IDs 1–5, NFR IDs 1–15 and OQ IDs 1–10 are contiguous in the current document; SM primary/secondary IDs 1–7 and counter-metrics C1–C3 are distinct.
- **[low] Assumptions Index does not round-trip** (`prd.md` §11, lines 561–564) — it lists FR-14 and NFR-10 as assumptions, but neither location has an inline `[ASSUMPTION]` tag; the memlog also records a usability/diagnostic-threshold assumption not indexed there. *Fix:* mark genuine unconfirmed assumptions inline and index each exactly once, or relabel resolved/gated items as decisions/open questions and remove them from the Assumptions Index.
- The PRFAQ `source` path in frontmatter, both OQ-2/OQ-3 research links, and the `docs/02-requirements/platform-nfr.md` normative path exist in the current workspace.
- OQ-4 check: no open high finding; `prd.md` NFR-10/§10 and `addendum.md` §I consistently stage G0 → G1 → G2 → G3.
