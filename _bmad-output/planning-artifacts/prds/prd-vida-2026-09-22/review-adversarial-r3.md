# Adversarial PRD review r3 — VIDA Release 1

- **Reviewed:** current `prd.md` (updated 2026-09-23) and `addendum.md` against approved v1 bundle, native-client requirements, platform NFR, platform-resource conformance, and the accepted architecture/requirements cited below.
- **Verdict:** **not ready for implementation fan-out**. Product scope broadly agrees with approved Release-1 requirements, but the claimed proof coverage and two addendum decision references would let a release gate pass on incomplete or contradictory evidence. This is a review only; no source decision is changed here.

## High

### H1 — The vertical slice claims coverage it does not exercise

`prd.md:476–478` specifies only a generic Chat, Note, Task, File, conflict and *an* E2EE call, then says that one slice proves `FR-5–FR-10`, `FR-12–FR-14` and `FR-18–FR-26`. The scenario does not explicitly exercise note/section Guest sharing (`FR-8`, `prd.md:178–185`), Forum topics (`FR-10`, `prd.md:200–206`), both 1:1 **and** eight-person group calls (`FR-12`, `prd.md:218–226`), live cursors/selections (`FR-14`, `prd.md:238–245`), conflicting File revision/copy choices (`FR-20`, `prd.md:293–299`), or irreversible-effect confirmation (`FR-26`, `prd.md:349–355`). `prd.md:482–488` has other suites, but the sentence at 478 still overstates what §6.2 itself proves; `SM-1` repeats that assertion at `prd.md:519`. The approved complete journey expressly includes scoped sharing, forum links and both call types (`docs/01-product/v1-replacement-bundle.md:108`; `docs/02-requirements/native-client-requirements.md:38–39`). **Impact:** false-green release traceability. **Fix:** list only scenario-observed FR consequences as §6.2 coverage, and attach separate named fixtures/gates for every remaining mandatory consequence; do not infer an eight-person call from an unspecified “E2EE call.”

### H2 — Iroh architectural selection is incorrectly described as undecided

`addendum.md:23` says Architecture will fix the exact Iroh version only after compatibility proof. Accepted `ADR-0005` already mandates **Iroh core 1.2** as primary transport (`docs/03-architecture/decisions/ADR-0005-iroh-transport-foundation.md:2–3,33–39`). The ADR separately requires the release lockfile/SBOM to pin exact dependency versions and checksums (`ibid.:45–52`). **Impact:** implementers could treat the core 1.2 architectural selection as open and drift to another core series without an ADR revision. **Fix:** distinguish “core 1.2 is selected” from “exact patch/build and transitive dependency pins are set by lockfile after compatibility proof”; if 1.2 is no longer intended, amend/supersede ADR-0005 explicitly.

### H3 — Media decision gate omits two current security/finality fixtures

`addendum.md:74,91` makes final media-profile selection depend on `F01–F15`. Current normative `docs/04-specifications/e2ee-calls-conformance.md:60–68` requires `F01–F17`. The omitted fixture `F16` tests immutable owning Space/AppInstance/conversation context, wrong-Space joins and revoked grants; `F17` tests durable-versus-transient signaling, replay and one terminal projection (`docs/04-specifications/fixtures/e2ee-calls-v1.yaml:94–101`). **Impact:** selecting a call stack after only F01–F15 could miss unauthorized media access or duplicate/incorrect ringing. **Fix:** update both addendum mentions to the current complete fixture set (or a versioned manifest reference), and require no hard-gate failure.

## Medium

### M1 — Fan-out gate inventory is incomplete across §9 and §10

`prd.md:539` says versioned operation envelope/serialization, storage/GC, Rust↔Flutter ABI and AppPackage compatibility contracts must freeze **before implementation fan-out**. Yet `prd.md:546,550–559` presents OQ-2/3 as the unresolved fan-out blockers and does not give those other R-2 contracts owners, artifacts or closure evidence. The sentence does not logically make OQ-2/3 the *only* blockers, but the table invites that interpretation. **Impact:** teams can claim the gate closed when two prototypes pass while fundamental wire/persistence/ABI contracts remain unapproved. **Fix:** add explicit gate rows or a linked normative fan-out checklist covering every R-2 contract, and state whether OQ-2/3 are necessary but not sufficient.

### M2 — A deterministic ID algorithm is asserted before its decision

`addendum.md:56` says derived Resources use deterministic logical IDs. The approved requirement fixes the **outcome**—one logical Resource per accepted fact—but leaves exact ID derivation, executor and delayed-rule fallback open (`docs/02-requirements/effect-execution-constraints.md:26,29,35,45`; `docs/00-governance/open-questions.md:58`). **Impact:** the addendum prematurely excludes otherwise conformant implementations and masks OQ-0048. **Fix:** say “devices converge on one logical derived Resource with one stable identity”; mark deterministic derivation versus authority-issued identity and executor as TBD under OQ-0048.

### M3 — Contact Card glossary may pre-decide the unresolved owning scope

`prd.md:92` defines every Resource as a record **in Space**, while `prd.md:94` names Contact Card a Core Resource. `docs/00-governance/open-questions.md:13` explicitly keeps ContactCard owning scope open, including a Persona Contacts vault distinct from Personal Space. **Impact:** a glossary inference could silently rule out a candidate architecture before user decision. **Fix:** describe Contact Card as a canonical Core contact entity/service record and explicitly defer its owning/replication scope to OQ-0003; after that decision, align the glossary and FR-4.

## Currentness note — prior rubric finding is stale

`review-rubric-v2.md:17–19` flags contradictory OQ-4 budget gates, but the **current** PRD says G0 method before implementation fan-out and G1-derived numeric G2 budgets before feature-complete (`prd.md:508,546,553`), exactly the sequence in `docs/04-specifications/platform-resource-conformance.md:46–55` and `docs/02-requirements/platform-nfr.md:55–56`. Do **not** carry that old high finding forward. Its older assertion that OQ-1 is still open (`review-rubric-v2.md:7`) is also superseded by `prd.md:550`.

## Scope check

No current contradiction found for Android+iOS+Flutter Windows Release 1, browser/hosted deferral, required Messenger/Notes/Project bundle, live cursors, eight-participant call cap, or G0→G1→G2→G3 resource sequence (`prd.md:21,224–245,450–508`; `docs/01-product/v1-replacement-bundle.md:50,65,106–110`; `docs/02-requirements/native-client-requirements.md:15–20,33–39`; `docs/04-specifications/platform-resource-conformance.md:46–55`). This check does not certify all PRD claims or resolve open architecture decisions.
