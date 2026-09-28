---
title: "VIDA Release 1 — PRFAQ Reconciliation"
status: review-required
created: 2026-09-22
inputs:
  - ../../prfaq-vida.md
  - ../../prfaq-vida-distillate.md
  - prd.md
  - addendum.md
---

# PRFAQ → PRD reconciliation

## Result

`prd.md` + `addendum.md` preserve most approved Release-1 decisions, but are not yet source-complete. One scope contradiction must be removed before PRD approval; four material contracts need restoration; four qualitative decisions need explicit acceptance language or traceability.

## Blocking contradiction

### RC-01 — Marketplace and external repositories moved into Release 1

- **Source decision:** external repositories and the first-party marketplace are **future distribution channels**; Release 1 contains AppPackage runtime and bundled Apps, not public package distribution ([distillate, line 68](../../prfaq-vida-distillate.md#L68)).
- **Current PRD:** FR-32 promises a built-in Marketplace and trusted external repositories, and §6.1 lists both in Release 1 ([prd.md, line 375](prd.md#L375), [prd.md, line 434](prd.md#L434)).
- **Impact:** unapproved scope expansion adds publisher trust, catalog, repository, dependency-resolution and store-policy work to the critical path.
- **Required correction:** remove public Marketplace/external-repository use from Release 1. Retain only the AppPackage/runtime contracts needed for bundled Core Apps and dependency conformance. Move catalog/repository distribution to post-v1; keep signed manifest/update metadata as an architecture requirement for that future contour.

## Material source gaps

### RC-02 — Contact connector and ContactCard sharing contract is incomplete

- **Source decision:** first connector use defaults to read/import-only; export or two-way sync requires a later explicit choice. Contact sharing supports one-time snapshot and revocable live share; snapshot is default; live recipients cannot edit the owner's card; private/anonymous bindings are never exported to device contacts ([PRFAQ, line 61](../../prfaq-vida.md#L61), [PRFAQ decision extract, line 272](../../prfaq-vida.md#L272)).
- **Current PRD:** FR-4 lists read/export/two-way choices but does not define the safe first-use default, snapshot/live sharing, recipient read-only semantics, local recipient notes/tags, revoke behavior, or the unconditional prohibition on exporting private/anonymous bindings ([prd.md, line 130](prd.md#L130)).
- **Impact:** onboarding and privacy behavior can diverge by platform; an implementation could expose a private binding or make two-way sync appear to be the initial default.
- **Required correction:** add testable FR-4 acceptance outcomes for the default connector mode and ContactCard sharing lifecycle. Keep `private/anonymous binding → VIDA-local only` as a Core invariant, not a best-effort format fallback.

### RC-03 — User-visible operation states collapse delivery and business acceptance

- **Source decision:** UI distinguishes durable local save, sync/publication/acceptance, delivery and conflict; transport delivery, durable storage, business acceptance and terminal outcome are separate concepts ([distillate, line 38](../../prfaq-vida-distillate.md#L38), [distillate, line 54](../../prfaq-vida-distillate.md#L54)).
- **Current PRD:** FR-22 defines `saved`, FR-23 defines a single `synchronized` state after protocol receipt, and FR-25 defines conflict. No functional requirement exposes recipient delivery, business acceptance or terminal outcome as distinct states ([prd.md, lines 283–325](prd.md#L283)).
- **Impact:** the product can incorrectly present transport receipt as recipient delivery or accepted business result—the exact reliability ambiguity the source rejected.
- **Required correction:** define a Core state vocabulary and UI mapping for at least `saved locally`, `pending`, `synchronized/published`, `delivered where provable`, `accepted/rejected where applicable`, `outcome unknown`, and `conflict`. Every state must name its proof and must not imply a stronger guarantee.

### RC-04 — Independent reference reader is no longer a concrete Release-1 gate

- **Source decision:** open-source Core, public versioned specifications, portable encrypted export **and an independent reference reader** are mandatory Release-1 continuity gates ([PRFAQ, line 77](../../prfaq-vida.md#L77), [PRFAQ decision extract, line 276](../../prfaq-vida.md#L276)).
- **Current PRD:** NFR-8 omits the reader artifact; FR-36 only states that an unspecified compatible independent client *can* read the export ([prd.md, line 406](prd.md#L406), [prd.md, line 450](prd.md#L450)).
- **Impact:** interoperability can be asserted from a format document without an executable independent proof.
- **Required correction:** restore a distributable reference reader/conformance implementation as an explicit gate, with a fixture proving authorized recovery of schemas, resources, relations, files and recovery metadata from the export.

### RC-05 — Unapproved Iroh version pin in the addendum

- **Source decision:** Iroh is the primary transport foundation; the exact transport delivery/VIDA semantics boundary remains to be specified. No Iroh version is approved in either source ([distillate, line 54](../../prfaq-vida-distillate.md#L54)).
- **Current addendum:** pins `Iroh 1.2` as the transport building block ([addendum.md, line 23](addendum.md#L23)).
- **Impact:** a draft addendum silently converts an implementation choice into a product constraint and can become stale before architecture selection.
- **Required correction:** say `Iroh (version selected and locked by Architecture after compatibility proof)` or cite the missing approved decision that authorizes 1.2.

## Dropped qualitative decisions

### RC-06 — Per-Space Files view is not required

- **Source decision:** every Space has a Files view; one File resource may be related to multiple resources without duplicate identity ([distillate, line 25](../../prfaq-vida-distillate.md#L25)).
- **Current PRD:** FR-19 preserves stable identity, versions and relations, but does not require the Files view ([prd.md, line 260](prd.md#L260)).
- **Correction:** add the per-Space Files view to FR-19 or to the later UX artifact with a PRD trace.

### RC-07 — Selective activation lacks the “inactive means unloaded” product constraint

- **Source decision:** bundled Apps are selectively activated and runtime does not keep inactive capabilities active without need ([PRFAQ, line 93](../../prfaq-vida.md#L93)).
- **Current PRD:** onboarding lets a user select Apps, but no FR/NFR constrains inactive App resource/background behavior.
- **Correction:** add an NFR/acceptance consequence that an inactive App does not run handlers, request its optional permissions, maintain background work, or materially consume runtime resources.

### RC-08 — Resource-scoped Guest semantics are implicit, not modeled

- **Source decision:** sharing a Note/Section uses a resource-scoped Guest grant and reveals only explicitly included resources ([PRFAQ, line 69](../../prfaq-vida.md#L69)).
- **Current PRD:** FR-8 preserves visibility boundaries but says only “holder of permission”; FR-6 names Owner/Admin/User and does not define Guest/resource-scoped membership ([prd.md, lines 143–175](prd.md#L143)).
- **Correction:** define whether Guest is a role, a grant subject, or a non-member capability; then add authorization and revoke tests so implementations do not accidentally create full Space membership.

### RC-09 — Two numeric success thresholds are source-free assumptions

- **Source position:** product-value proof must be privacy-preserving and cannot rely on raw downloads alone; method/sample remain open. Quantitative gates should not be invented before evidence ([distillate, lines 115–116](../../prfaq-vida-distillate.md#L115)).
- **Current PRD:** SM-3 invents `80%` unassisted completion and SM-6 invents `90%` diagnostic-redaction pass rate, albeit marked assumptions ([prd.md, lines 460–468](prd.md#L460)).
- **Correction:** keep the measurement methods as proposed validation plans, but move numeric targets to the Assumptions/Open Questions section until a baseline or explicit product decision approves them. For secret/content redaction, evaluate whether Release 1 should require zero known leaks rather than a 90% pass target.

## Source-aligned items confirmed

- Core product hero, Release-1 feature boundary and explicit City Portal/Hosted/Browser/WinUI deferral are preserved.
- Flutter Android/iOS/Windows + shared Rust Core, equal device peers, seven-day shared-rights revalidation, offline durability and explicit Conflict lifecycle are preserved.
- Messenger/Forum, Notes/Knowledge, Project, Files/Relations, E2EE calls, localization, diagnostics, approval modes and schema evolution are materially represented.
- MIT Core, public specifications, portable export and public-release legal/security blockers are represented, subject to RC-04.

## Approval condition

PRFAQ reconciliation passes after RC-01 is removed, RC-02–RC-05 become testable requirements/constraints, and RC-06–RC-09 are either restored with traceability or explicitly rejected by a new product decision.
