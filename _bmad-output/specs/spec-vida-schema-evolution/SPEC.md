---
id: SPEC-vida-schema-evolution
status: draft
companions:
  - ../../../docs/04-specifications/schema-evolution-contract.md
  - ../../../docs/02-requirements/app-package-requirements.md
  - ../spec-vida-app-packages/SPEC.md
  - ../../../docs/03-architecture/decisions/ADR-0007-declarative-app-packages.md
  - ../../../docs/03-architecture/decisions/ADR-0012-command-event-sync-boundary.md
sources: []
---

> **Decision-gated OQ-0040 kernel.** `REQ-APP-017/018` and the resolved parts of OQ-0040 are binding. Converter topology, shared activation authority and `SCHEMA-F01–F08` remain draft proposals in the adopted companion.

# VIDA schema evolution and activation

## Why

Messenger, Notes and Projects must retain readable data while their AppPackages evolve across Devices that may be offline for different periods. An update must not silently lose legacy fields, partially migrate a Space or rerun business actions merely because a new schema is installed.

## Capabilities

- **CAP-1**
  - **intent:** A user can read retained resources created under older schema versions after the AppInstance updates.
  - **success:** A legacy resource shows a current projection without rewriting immutable history; a required new value without a safe default appears as missing rather than as fabricated valid data.
- **CAP-2**
  - **intent:** A user or authorized automation can edit a legacy resource under the current AppInstance schema.
  - **success:** A safe default is materialized on the next write; without one, save is visibly blocked until the required value is supplied, and no unknown field is silently discarded.
- **CAP-3**
  - **intent:** An AppInstance can activate a compatible package/schema update without exposing a partially migrated version.
  - **success:** Mandatory compatibility and migration preflight pass before activation; an invalid record or insufficient storage leaves the former version active (or first activation inactive), preserves data and shows an actionable error. Retry repeats the gate after correction.
- **CAP-4**
  - **intent:** A Device returning after an offline period can reconcile its old-version writes without corrupting the active schema.
  - **success:** A provably safe conversion retains the write and unknown values; otherwise the affected write is explicitly pending/incompatible and only that AppInstance requires an update while other Apps remain usable.
- **CAP-5**
  - **intent:** Authorized Devices can converge on one accepted logical schema migration after reconnection.
  - **success:** Candidate conformance tests can show the same migration identity/input frontier produces one logical result under duplicate delivery and crash recovery; the authority receipt and converter protocol needed for this result remain open.

## Constraints

- Accepted: one `AppInstance` has one active current schema for new writes; obtaining a package does not activate that schema or rewrite existing data. Each durable resource/operation has a schema version.
- Accepted: `compatible-auto` is limited to forward-compatible data, workflows, dependencies, host capabilities and authorized outcomes. A breaking converter, new mandatory capability or expanded data scope needs another update classification.
- Accepted: migration/compatibility preflight precedes activation; failure preserves the prior active version without partial rewrite. Post-activation downgrade/rollback is outside the baseline.
- Draft conformance target: shared Resource-schema activation and Device-local store migration are different operations. Local SQLite success does not itself make a shared schema current; immutable original `ContractId/SchemaId` remains a proposed payload invariant.
- Draft conformance target: unknown fields round-trip or cause explicit incompatibility; field identities are not silently reused; renames/deprecations retain audit provenance. Exact converter/retention profile still needs adoption.
- Draft conformance target: migration replay does not invoke AppInstance command handlers or external effects; package provenance alone does not prove conversion correctness.

## Non-goals

- Selecting a wire codec, converter ABI, hub-and-spoke implementation, authority topology, mixed-version duration or background materialization schedule in this kernel.
- Bulk rewriting all legacy records merely because an optional/defaulted field was added, or adding a generic bulk-update UI for this case.
- Treating a signed package or successful local database transaction as proof that a shared migration is correct and accepted.

## Success signal

On two Devices, one kept offline with v1, VIDA stages v2, rejects a failing migration without changing the active AppInstance, then accepts a corrected compatible update. A legacy read stays available; its next edit writes v2 or explicitly requests a missing required value. The old Device reconnects without deleting new fields or replaying business effects. `SCHEMA-F01–F08` are candidate fixtures, not executed evidence.

## Open Questions

- Who may activate a new schema in a shared Space, and which signed receipt makes it current for all Devices?
- How long may old clients write after activation, and when is the affected AppInstance read-only/update-required?
- Which converter ABI and rules cover partial blobs, unknown extensions and writes from an old offline Device?
- Which pre-activation fixtures are mandatory for UI/handler behavior, low-space diagnostics, retry and complete re-preflight?
