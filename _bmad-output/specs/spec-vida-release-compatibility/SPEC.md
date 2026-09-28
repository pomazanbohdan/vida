---
id: SPEC-vida-release-compatibility
status: draft
companions:
  - compatibility-matrix.md
  - ../../../docs/02-requirements/app-package-requirements.md
  - ../../../docs/04-specifications/app-package-runtime-baseline.md
  - ../../../docs/04-specifications/schema-evolution-contract.md
  - ../spec-vida-platform-bindings/SPEC.md
  - ../spec-vida-operation-envelope/SPEC.md
sources: []
---

> **Decision-gated OQ-0037 kernel.** Accepted safety boundaries are below; `compatibility-matrix.md` contains candidate fields and tests. This draft does not choose a supported mixed-version window, manifest encoding, bridge ABI or activation authority.

# VIDA Release-1 compatibility

## Why

VIDA must update bundled and external Apps while Android, iOS and Windows Devices can be on different host, package and schema versions, including after long offline periods. A retrieved package is not necessarily safe to activate, and an old client must not silently erase new data or treat unknown protocol behavior as supported.

## Capabilities

- **CAP-1**
  - **intent:** A client can discover and classify an available package release without changing its active AppInstance.
  - **success:** Discovery shows update status while the installed package, active schema, grants and current resources remain unchanged.
- **CAP-2**
  - **intent:** A client can verify whether a release and its dependencies are safe for its installed host and protocol profile before activation.
  - **success:** Stale/substituted artifacts, unsupported mandatory capability, incompatible protocol major or mismatched native binding fail explicitly before unsafe calls or migration.
- **CAP-3**
  - **intent:** An authorized Space can activate a compatible package/schema as one recoverable transition.
  - **success:** Failed preflight keeps the previous version active, or leaves a first instance inactive; successful accepted activation yields the same version/schema state across authorized Devices without partial migration.
- **CAP-4**
  - **intent:** Clients of supported different versions can continue using data without lossy writes.
  - **success:** A compatible older client round-trips required unknown data safely; otherwise the affected AppInstance reports pending/incompatible or read-only/update-required while other Apps and the VIDA shell continue.

## Constraints

- Native host and `AppPackage` have separate release channels. Discovery, acquisition and local cache do not activate an AppInstance or grant Space access.
- Approved package modes are `compatible-auto`, `security-auto`, `manual` and `pinned`; the exact eligibility of `security-auto` is still open. `compatible-auto` cannot add a mandatory capability, breaking converter, broader dependency contract or new data scope.
- Retrieval must verify provenance, freshness and artifact integrity before isolated deterministic migration preflight. TUF-style anti-rollback is required; trusted retrieval does not establish converter correctness.
- Activation requires current Space authority and successful compatibility/migration gates. A failed pre-activation update keeps the old active version; failed first activation stays inactive with an explicit error. The exact approving actor and receipt remain OQ-0039.
- Post-activation downgrade, package/schema rollback and runtime fallback are outside the current baseline. A discovered post-activation defect is release nonconformance, not permission for silent fallback.
- Unknown mandatory protocol/host/FFI semantics fail closed. Old clients may not silently drop unknown fields, reinterpret a signature or perform a lossy write; the exact supported coexistence window remains open.
- Release 1 includes installed Flutter Android, iOS and Windows shells with shared Rust semantics. iOS v1 accepts only declarative package capabilities already built into the host.

## Non-goals

- Selecting `N/N-1` or any other support window, exact manifest tags, wire codec, FFI generator, publisher trust root or migration authority in this draft.
- Treating a TUF-style verified download, transport receipt or local SQLite version bump as Space acceptance.
- Designing post-activation rollback/recovery UX that the user has deferred.

## Success signal

One public fixture set covers the approved Android/iOS/Windows host × package × schema × protocol tuples. Every approved tuple activates and safely exchanges data; rejected tuples fail before mutation or unsafe calls, and a long-offline client cannot erase a newer field. Until the window and matrix are approved and fixtures execute, this is a testable draft, not a passed release gate.

## Open Questions

- Which exact host/API/FFI/schema/protocol versions coexist, for how long, and when does an affected old AppInstance become read-only/update-required?
- Who accepts schema/package activation in a shared Space, and what signed proof makes the transition current for equal Devices?
- Which old offline writes are losslessly convertible, and which must remain pending until the client is updated?
- Which version/platform combinations are mandatory pre-activation tests, and what makes a `security-auto` release eligible without widened access?
