---
title: "VIDA PRD adversarial cross-document review R4"
date: 2026-09-23
scope: "Current prd.md and addendum.md against approved requirements, accepted ADRs, current specifications, fixtures and open questions"
verdict: "not-ready-for-finalization"
---

# Adversarial cross-document review R4

## Verdict

The PRD remains correctly marked `draft`. OQ-4's staged G0/G1/G2 gate and the F01–F17 calls fixture range now agree with their current specifications. No missing local Markdown link target was found in `prd.md` or `addendum.md`. Three material cross-document/gate findings remain; they do not justify declaring the PRD or implementation fan-out ready.

## High — accepted AppPackage requirements still mandate Release-1 external packages

`prd.md:414-419,457-459` limits Release 1 to bundled packages and explicitly excludes public Marketplace and external repositories. This matches the later PRFAQ distillate (`prfaq-vida-distillate.md:62-68`), but conflicts with still-`approved` `docs/02-requirements/app-package-requirements.md:26-35`: `REQ-APP-003` requires a compatible external package to load on an unchanged client, and `REQ-APP-007` requires a built-in marketplace plus an external repository in a release build. Accepted `ADR-0007-declarative-app-packages.md:35-37` also states the external-package ability as a v1 `MUST`; `ADR-0008-package-distribution-channels.md:20-26,41-46` retains both distribution channels and acceptance evidence. A team implementing the approved REQ/ADR set would deliver a different release from the PRD.

Disposition: explicitly reconcile the release scope in the normative REQ/ADR set, recording whether external-package loading itself is Release 1 while its distribution channels are deferred, or whether both are post-v1. Do not silently treat the later PRFAQ as superseding still-accepted ADRs.

## High — R-2 fan-out inventory omits the serverless authority gate

`prd.md:548,553-561` says the addendum lists the remaining R-2 contract gates. `addendum.md:98-109` includes envelope, storage/GC, editor, FFI, media and package contracts but does not name `OQ-0033` for non-media domain authority. Yet `docs/04-specifications/sync-log-contract.md:52,77,83` explicitly blocks independent SyncLog implementations until `OQ-0033`/`OQ-0034` define serverless authority topology, equivalence, deterministic operation-family transitions and frontier/snapshot rules; `docs/00-governance/open-questions.md:43-44` keeps these open. An engineer could read the R-2 inventory as exhaustive, close its named rows and incorrectly fan out sync/domain implementation without a common authority contract.

Disposition: add an explicit domain-authority/signed-log row or amend the envelope row and R-2 risk to name `OQ-0033` alongside `OQ-0034`, with required serverless authority/current-frontier and convergence evidence. Distinguish the already-approved receipt/finality semantics from the still-open authority mechanism.

## High — approved privacy/compliance baseline is missing from explicit public-release gates

`prd.md:496,508-522,545-565` names platform, OWASP, store and operational ownership gates, but does not make the approved `REQ-PRIV-001–009` baseline an explicit public-release gate. `docs/02-requirements/privacy-compliance-requirements.md:10-22` requires processing-flow records, legal basis/roles, data-subject rights, incident notification, DPIA where high-risk, EEA transfer assessment, distinct local/peer/push/diagnostics flows and EU applicability review; `REQ-PRIV-009` expressly blocks public store publication until controller/operator/store-account responsibilities are named. R-3 covers only some of this. The PRD's broad “store-policy” and “privacy” phrases cannot prove these obligations.

Disposition: add a product-level release gate referencing the approved baseline and its required artifacts, with legal decisions kept open rather than invented. Keep country/locale choice separate from GDPR applicability.

## Medium — basic AppInstance activation has two incompatible defaults

`prd.md:49,423-429` and `docs/02-requirements/native-client-requirements.md:30` let a new Personal Space activate only selected bundled Apps. However `REQ-APP-002` (`app-package-requirements.md:27`) and accepted `ADR-0007:37,63` say every standard Space receives three AppInstances. This may be resolvable by defining provisioned/inactive versus active instances, but the current wording does not. The distinction affects NFR-13 (inactive Apps consume no material resources) and onboarding acceptance.

Disposition: align the normative requirements with the approved selectable onboarding, explicitly defining package-bundled, instance-provisioned and instance-active states without dropping any Core App from installation.

## Medium — FR-29 preselects an activation authority still marked open

`prd.md:380-388` states that Owner or Admin with `manage_apps` activates AppPackages. The approved role preset gives Admin broad apps management (`ADR-0002-default-role-presets.md:59-60`), but `docs/00-governance/open-questions.md:49` and `docs/02-requirements/app-package-requirements.md:48` expressly leave exact Owner/Admin activation, deactivation, removal, schema activation and capability-grant authority open. The PRD's role statement can be read as a resolved permission decision.

Disposition: either ratify the role/action matrix in an accepted source and close that portion of `OQ-0039`, or mark FR-29's precise activation actor as pending while retaining the user-facing requirement that only authorized Space managers can activate.

## Checks and limits

- Rechecked current `prd.md`, `addendum.md`, `docs/00-governance/open-questions.md`, accepted ADR-0002/0005/0007/0008, approved REQ-APP/REQ-PRIV and current sync, media, editor, platform and package specification references.
- Local Markdown target check: all relative path targets in `prd.md` and `addendum.md` exist. This does not validate every heading anchor or remote URL.
- No source document was edited by this reviewer.
