---
title: "VIDA Release 1 PRD — Repository Reconciliation"
status: review
created: 2026-09-22
inputs:
  - prd.md
  - addendum.md
scope:
  - docs/02-requirements
  - docs/03-architecture/decisions
---

# Repository reconciliation

## Result

`prd.md` and `addendum.md` are directionally aligned with the accepted platform, browser, transport, identity, package, localization and Windows decisions, but they are not yet a lossless product-level projection of the approved repository requirements. The following gaps must be reconciled before the PRD is treated as implementation-authoritative.

## 1. Canonical access model is collapsed into a non-canonical `User` role

**PRD/addendum state**

- `prd.md:151-158` defines only `Owner`, `Admin` and `User`, plus CRUD permissions.
- `addendum.md:45-46` preserves only the Owner/Admin governance split and the seven-day revalidation window.

**Approved repository state**

- `docs/02-requirements/access-control-requirements.md:57-65` requires membership classes `Member`, `Guest`, `ServicePrincipal` and immutable presets `Owner`, `Admin`, `Manager`, `Contributor`, `Commenter`, `Viewer`.
- `docs/02-requirements/access-control-requirements.md:32-55` requires schema-defined domain actions and different rights by app/container/resource, not CRUD alone.
- `docs/03-architecture/decisions/ADR-0002-default-role-presets.md:55-79` makes the six presets, scoped roles and custom-role constraints an accepted decision.

**Required reconciliation**

- Remove `User` as a canonical role from FR-6.
- Add membership class, six built-in presets, scoped/custom-role behavior and schema-defined actions to the PRD or explicitly normatively incorporate `REQ-ACL-001..009` and ADR-0001/0002.
- Preserve the existing Owner/Admin invariants; do not replace the approved richer model with a three-role shortcut.

## 2. “Full access to own Resources” conflicts with mandatory authorization

**PRD/addendum state**

- `prd.md:348-355` says an AppInstance “повністю керує власними Resources” by default.
- `addendum.md:51-54` says an AppInstance has full access to its own Resources.

**Approved repository state**

- `docs/02-requirements/app-package-requirements.md:29-31` requires Space-scoped permissions and forbids a package/link from granting access by installation.
- `docs/02-requirements/app-package-requirements.md:36-44` requires every package action to pass authorization and forbids package logic from bypassing Core governance.
- `docs/03-architecture/decisions/ADR-0007-declarative-app-packages.md:35-41` explicitly denies rights obtained merely from package installation and keeps grants within the Space contract.

**Required reconciliation**

- Replace “full access” with: an AppInstance owns its schema/configuration namespace, while every read/write/effect is authorized against current Space, instance, container and resource policy.
- State that declared cross-App dependencies expose contracts, not implicit grants.

## 3. Release-1 Contact Cards omit approved user-visible behavior

**PRD/addendum state**

- `prd.md:130-137` covers connector modes, no automatic deduplication and local treatment of unsupported identifiers.
- `addendum.md` contains no Contact Card contract.

**Approved repository state**

- `docs/02-requirements/contact-card-requirements.md:19-24` requires a canonical stable VIDA card, field provenance/visibility, typed identity bindings, explicit connector modes and provider metadata.
- `docs/02-requirements/contact-card-requirements.md:25-30` defines platform/Google constraints, vCard 4.0 baseline, field-scoped sharing and separation of connector unlink from card/provider deletion.
- `docs/02-requirements/contact-card-requirements.md:31-33` requires explicit snapshot/live-share modes, snapshot as default and read/import as the first-connection default.

**Required reconciliation**

- Expand FR-4 acceptance with canonical card identity, provenance/visibility, preview-before-write, connector unlink semantics, vCard boundary and snapshot/live sharing.
- Add the contact connector conformance gate instead of leaving these approved behaviors implicit.

## 4. “Openness” is named, but its accepted release evidence is missing

**PRD/addendum state**

- `prd.md:406-412` requires portable export.
- `prd.md:450` reduces openness to MIT Core, public specifications, compatibility tests and export.
- `prd.md:437-460` gates Release 1 only on the official three-platform vertical slice.

**Approved repository state**

- `docs/02-requirements/open-interoperability-requirements.md:19-23` requires independent client/node implementation, full wire/canonical-signing coverage, third-party AppPackage conformance, public byte-exact/negative-security fixtures and a public change process.
- `docs/02-requirements/open-interoperability-requirements.md:24-26` requires explicit non-code licenses, an independent reference reader, cross-vendor evidence and no hidden paid dependency.
- `docs/03-architecture/decisions/ADR-0013-open-interoperability.md` is the accepted architectural decision behind those gates.

**Required reconciliation**

- Expand NFR-8/SM-1 with independent client+node, third-party package, public fixture/suite, reference-reader and license-manifest evidence.
- Distinguish “specification published” from “interoperability demonstrated”; only the latter satisfies the accepted release claim.

## 5. The PRD release gate does not incorporate the approved platform conformance matrix

**PRD/addendum state**

- `prd.md:437-454` defines one vertical slice and broad NFR statements.
- `addendum.md:69-75` records call/background decisions but leaves timings to experiments.

**Approved repository state**

- `docs/02-requirements/platform-nfr.md:20-23` requires injected crash-boundary recovery, exactly-one domain apply, reconnect pull/reconcile and node-loss/restore drills.
- `docs/02-requirements/platform-nfr.md:27-31` requires negative security tests and seven-day rights expiry across all shared-Space roles and all read surfaces.
- `docs/02-requirements/platform-nfr.md:42-56` requires per-platform suspend/process-death/network/keystore, offline app, Windows accessibility, background/retry, store-policy, resource-budget and contact-connector conformance.

**Required reconciliation**

- Make the approved `platform-nfr.md` matrix a normative Release-1 gate referenced by SM-1/SM-4.
- Add explicit evidence mapping from every platform NFR to Android, iOS and Windows; the current generic vertical slice is insufficient.

## Reconciliation priority

1. Fix the authorization contradictions in findings 1-2 before architecture decomposition.
2. Expand release acceptance for interoperability and platform conformance in findings 4-5 before marking the PRD complete.
3. Expand Contact Cards in finding 3 before UX and story decomposition.

