# OQ-0037 compatibility matrix and activation gate — draft

The tables are **candidate evidence design**, not an approved version window or executable test bundle. [Package requirements](../../../docs/02-requirements/app-package-requirements.md), [runtime baseline](../../../docs/04-specifications/app-package-runtime-baseline.md), [schema evolution](../../../docs/04-specifications/schema-evolution-contract.md), [binding kernel](../spec-vida-platform-bindings/SPEC.md) and [signed-envelope kernel](../spec-vida-operation-envelope/SPEC.md) remain load-bearing.

## Axes to declare separately

| Axis | Candidate manifest/installed-artifact evidence | Why one package version is insufficient |
|---|---|---|
| Provenance | Package identity, immutable digest, source/publisher/channel, signed metadata version/expiry and artifact size/hash. | A matching name may identify substituted, stale or wrong-channel bytes. |
| Host/binding | Flutter/Dart build, Rust `vida-sdk`/`vida-contracts` revision, generated binding and target ABI/profile. | A valid AppPackage can still call an incompatible installed native library. |
| App dependencies | Required/optional capabilities, target package/contract IDs and supported version ranges. | A dependency declaration is not a grant; a missing or widened contract changes behavior. |
| Data | Resource/operation `ContractId`/`SchemaId`, current schema, converter IDs/digests and source→target paths. | Two versions may parse the same bytes yet produce different authorized outcomes or lose unknown fields. |
| Protocol | ALPN/profile major, compatible minor features, mandatory/optional feature set and signed-envelope profile. | Transport connection is not proof that peers can validate each other's operations. |
| Activation | Space/AppInstance, proposed version, migration ID/input frontier/output digest and accepted authority receipt. | Local download or database migration does not make a shared schema current. |

These are fields to **test**, not final wire tags, JSON keys or a selected trust framework. [TUF](https://theupdateframework.github.io/specification/v1.0.31/) addresses authenticated/fresh retrieval, rollback, freeze and mix-and-match attacks; it does not define VIDA package semantics or prove a converter. [Protocol Buffers compatibility guidance](https://protobuf.dev/programming-guides/proto3/#updating) illustrates unknown-field and field-identity hazards; VIDA has not chosen Protobuf as its codec.

## Candidate gate sequence

1. Distinguish native host update from AppPackage discovery; discovery changes no active Space state.
2. Verify trusted source/channel, signed metadata freshness, monotonic anti-rollback state, artifact digest/signature and bounds **before** unpacking or migration.
3. Check host/binding tuple, declared dependencies, platform capabilities, protocol major/features and schema/converter paths. Unknown mandatory semantics fail explicitly.
4. Preflight migration and UI/handler compatibility on an isolated staged state at a named input frontier. Missing blob/converter, invalid legacy record or insufficient storage leaves old activation intact.
5. Submit activation through current Space authority; only accepted activation changes current package/schema/converter pointers. The approving actor and exact receipt remain open.
6. Reconcile older Devices by declared safe conversion or explicit incompatible/pending/read-only state for the affected AppInstance. Never erase an unknown field or re-sign altered operation bytes.

The sequence is a candidate decomposition of existing approved boundaries, not approval of its specific manifest or authority mechanism. `security-auto` still passes the same gate; its semantic eligibility is undecided.

## Candidate conformance fixtures — not executed

| ID | Input | Required observable result |
|---|---|---|
| CMP-F01 | Discovery reports a new first-party or external release. | Active package/schema/grants stay unchanged; external source provenance remains visible. |
| CMP-F02 | Stale/expired metadata, wrong channel, substituted artifact, oversized payload or rollback/mix-and-match attempt. | Rejected before preflight; previous active version remains. |
| CMP-F03 | Generated Dart binding and native Rust artifact have incompatible mandatory ContractId/ABI. | Initialization fails before unsafe call; no operation is committed. |
| CMP-F04 | Unknown mandatory host/UI/protocol feature or incompatible ALPN/profile major. | Explicit compatibility failure; no silent downgrade or fallback rendering. |
| CMP-F05 | Compatible optional field/defaulted schema addition. | Legacy read has correct projection; next edit writes current schema, preserving historical operation. |
| CMP-F06 | Old client receives new required field or unsupported converter. | It cannot perform a lossy write; affected AppInstance is pending/incompatible or read-only/update-required, not the entire VIDA shell. |
| CMP-F07 | Missing blob/converter, invalid record, no disk space or failed UI/handler fixture during preflight. | Prior version remains active, or first activation remains inactive; authorized manager sees a specific failure. |
| CMP-F08 | Two Devices replay one accepted migration/activation at same input frontier. | Same active version/output digest; no second logical migration or partial pointer switch. |
| CMP-F09 | Actor lacks activation right or loses it during preflight. | Authority rejects activation regardless of local preflight success; staging confers no grant. |
| CMP-F10 | Long-offline old client submits a candidate after new schema activation. | Safe conversion and current rights may accept; otherwise candidate remains recoverable/pending, never age-only discarded or silently merged. |
| CMP-F11 | Package update expands cross-App dependency operations or data scope but claims `compatible-auto`. | Classification fails; prior grant is not widened by update metadata. |
| CMP-F12 | Positive and negative host × package × schema × protocol tuples across Android, iOS and Windows. | The approved support window has explicit pass/fail expectations and records exact artifact/toolchain revisions; no untested tuple is claimed supported. |

## Decision gate still needed

- Choose a measured mixed-version support window and its cutoff: which host, binding, schema and protocol versions must coexist during offline rollout?
- Choose activation actor/proof with OQ-0039 and serverless Space authority OQ-0033; a local install cannot settle it.
- Define whether old writes are converted at producer or recipient and how incompatible pending work is surfaced and repaired.
- Approve the mandatory test matrix and `security-auto` eligibility. `N/N-1` and pairwise coverage are evaluation options, not approved policy.
