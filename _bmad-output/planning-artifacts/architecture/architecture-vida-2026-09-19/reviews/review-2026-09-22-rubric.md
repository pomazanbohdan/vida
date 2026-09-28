# Architecture Spine reviewer gate — good-spine rubric, 2026-09-22

**Verdict: CHANGES REQUIRED.** Mechanical `lint_spine.py` passes (0 findings), and the core/runtime, peer-equality, conflict, receipt, call and platform boundaries are substantial. Four Release-1 seams still let independently built units make incompatible security or data-model decisions. This validates the current spine only; source documents were not changed.

## High

### R1 — Core Resource/Relation and scoped-sharing seam is absent — discuss, then fix

The spine gives Space membership rules (spine:132–136), data-plane separation (138–142), and observable DTO ownership (168–172), but its capability map (384–415) has no owner or contract for the shared Resource/Relation graph. The PRD requires one File ID across attachments, typed cross-App Relations, resource-scoped Guest sharing, explicit inclusion of attachments, and non-disclosure of private backlinks/titles (PRD:178–185, 230–236, 257–263, 283–307). UX assumes one contextual link that grants no access and checks the target at navigation time (EXPERIENCE:64–70, 111–119). Messenger, Notes, Project, Files and search teams could otherwise disagree on edge identity, ownership, dangling/private targets, or whether a link grants access. Bind a Core-owned typed relation and scoped-share invariant, including endpoint authorization and metadata filtering, with cross-App conformance fixtures; leave physical graph storage as implementation detail.

### R2 — Autonomous recovery and new-Device trust bootstrap are not bound — discuss, then fix

AD-3 governs EndpointBinding/DeviceGrant lifecycle (spine:114–118), AD-9 asks for platform recovery gates (150–154), and AD-21 establishes equal peers (222–226). None states which Core authority issues a new Device grant after existing-Device confirmation or recovery, how recovery material is verified without a server, or the isolation rule preventing a federated/corporate account from recovering a private Persona. These are Release-1 FR-1–FR-3 and the first two journeys (PRD:110–132; EXPERIENCE:138–150). Two clients could implement incompatible enrollment and key recovery while satisfying the existing endpoint rule. Bind the trust/authorization boundary and a loss-of-all-devices recovery conformance gate; leave cryptographic format choices to the relevant contract.

### R3 — Contact Card versus OS-provider authority is unassigned — fix

The Release-1 Contact Card/connector is a Core capability (PRD:134–144; EXPERIENCE:20–25, 76–80), but the spine's shell/core split (162–166), observable-contract rule (168–172) and map (384–415) never allocate card identity/provenance, provider field mapping, or connector write authority. Android, iOS and Windows adapters could silently deduplicate, export a private Persona binding, or delete a VIDA card during unlink. Bind one Core Card identity and field-provenance model; put OS contact access behind consented platform adapters; require import-only default, previewed export/two-way changes, snapshot/live-share distinction and no-delete unlink in a shared fixture suite. Provider API details can remain deferred.

### R4 — AppInstance dependency does not have a no-grant rule — fix

AD-29 governs updates (spine:270–275), AD-30 separates Developer from Space authority (277–281), and AD-32 composes Messenger/Notes/Project dependencies (289–293). Yet the PRD requires each AppInstance's schema/config namespace and every read/write/effect to pass Space-, instance-, container- and Resource-scoped authorization; a cross-App dependency is a versioned contract, never an implicit access grant (PRD:379–387). The existing generic Core authorization rule (spine:162–166) does not say what dependency activation may authorize. A Project integration could treat a Notes dependency as blanket Note access while Notes rejects it. Add an enforceable dependency/capability rule and a denied-cross-App fixture before implementation fan-out.

## Medium

### R5 — Deferred mixes unresolved gates with settled decisions and future scope — trim

Deferred:424, 426–429 restates AD-16/18/22–29 and release-scope choices; it also places cross-shell AppPackage UI-port semantics under future independent/WinUI clients (424), although Release 1 requires signed declarative package parity across Android/iOS/Windows and a common AppPackage conformance contract (PRD:407–414, 466–488). Keep only unresolved decision, owner and revisit gate; clarify whether the declarative rendering contract for the three Flutter shells is already covered by AD-2/12/33 fixtures or still needs a Release-1 gate.

### R6 — Release operations ownership is narrower than the PRD blockers — defer to named gate

AD-15 names endpoint, delivery and Space-authority operators plus promotion evidence (spine:186–190), but the PRD additionally blocks public submission on controller, store-account, signing-key, incident/disclosure ownership; independent security assurance and localization review remain unassigned (PRD:538–542, 554–556). These need not become technical ADs, but the initiative-level handoff should name their PRD gate/owner rather than let `status: final` (spine:8) be read as production readiness. `implementation_status: prototype-gated` (10) appropriately limits the present claim.

## Checklist disposition

| Criterion | Result |
|---|---|
| AD structure and deterministic lint | Pass: 0 findings |
| Core/runtime, operation, convergence, call and platform invariants | Pass for represented seams |
| Full Release-1 capability coverage | Fail: R1–R4 |
| Every Deferred item safe for independent units | Fail: R5 |
| Operational/environmental envelope | Partial: R6 |
| Named technology currentness | Not rechecked in this rubric lens; separate currentness review required |
| Brownfield ratification | Not rechecked against implementation; this pass compares spine with the current PRD and UX discovery |
