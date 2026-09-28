# OQ-0028 wire-profile decision record — draft

This companion separates **accepted VIDA semantics** from **unselected byte-level choices**. It is not a wire protocol. The adopted contracts in `SPEC.md` frontmatter govern if this inventory is incomplete.

## Semantic inventory already required

| Boundary | Existing requirement | Byte-level decision still needed |
|---|---|---|
| Operation identity | Stable `OperationId`; direct and mailbox use the same signed inner operation; origin log/outbox commit atomically. | ID bytes, derivation/randomness, collision handling and signed inner/outer frame boundary. |
| Author and scope | Author/device binding reference and Space/context; endpoint is not identity or authorization. | Exact Persona, Device, Space, AppInstance and optional Resource/Process field types and visibility. |
| Causality | Dependencies and frontier keep out-of-order input pending; authority acceptance is separate. | Canonical parent/frontier encoding, digest construction and limits. |
| Control and schema | Current grant/epoch, schema/policy version and feature set are verified before visible apply. | Canonical control proof reference, schema registry IDs, mandatory-feature encoding and extension rules. |
| Content protection | Payload/domain-sensitive fields stay E2E encrypted; mailbox routes ciphertext without authority. | Clear routing minimum, ciphertext/content digest, encryption context and key-epoch binding. |
| Evidence | Replication, delivery, authority and effect receipts bind to operation/request and frontier, but are distinct facts. | Receipt byte schema, signature domain, exact operation digest reference and profile negotiation. |

These are semantic requirements from [SyncLog](../../../docs/04-specifications/sync-log-contract.md), [DurableDelivery](../../../docs/04-specifications/durable-delivery-contract.md), [Operation Finality](../../../docs/04-specifications/operation-finality-contract.md) and [SpaceMembership](../../../docs/04-specifications/space-membership-contract.md). A draft field name in this table must not be mistaken for an assigned wire tag.

## Candidate standards and unresolved choices

- The existing [storage-format research](../../../research/storage-format-architecture-research.md) recommends a typed Platform Data Model (PDM) with a deterministic, schema-aware CBOR profile; the current [stack selection brief](../../../docs/03-architecture/stack-selection-brief.md) still labels this a candidate pending OQ-0028 proof. PDM is the semantic model; CBOR would be one binary representation, not a competing alternative to PDM.
- [RFC 8949 §4.2](https://www.rfc-editor.org/rfc/rfc8949.html#section-4.2) provides core deterministic CBOR requirements: preferred shortest encodings, definite lengths and lexicographic ordering of encoded map keys. It leaves VIDA to define allowed types/tags, field IDs, numeric and Unicode normalization, absence/null/default semantics, duplicate-key rejection and bounds. Older length-first map ordering is a different profile; mixing both would break byte-exact signatures. A candidate strict decoder must inspect raw entries **before** a generic map decoder can collapse duplicate keys or discard unknown raw bytes.
- [RFC 9052](https://www.rfc-editor.org/rfc/rfc9052.html) defines COSE structures with protected/unprotected headers, `crit` and external authenticated data. It is a possible construction to assess, **not** an accepted VIDA choice. If used, authority-critical context cannot live only in unprotected headers; VIDA must define unambiguous external-AAD bytes and verify protected-header semantics, algorithm/key identity and exact Space/control/payload binding. COSE does not decide domain authorization.
- [RFC 8610](https://www.rfc-editor.org/rfc/rfc8610.html) CDDL can describe the chosen CBOR shape and support generated test input; it does not define VIDA access rules or canonical signing bytes by itself. Iroh's [ALPN configuration](https://docs.rs/iroh/latest/iroh/endpoint/struct.Builder.html) selects a transport protocol, not an operation schema or capability intersection.
- [OWASP Key Management](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html) and [OWASP Authorization](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) inform key custody and per-operation access checks. Neither specifies VIDA's codec, signature domain or distributed authority ordering.

## Recommendation for review, not an adopted wire decision

Use **PDM for typed operation meaning** and prototype **RFC 8949 core deterministic CBOR with schema-aware numeric maps** as the first byte profile. This is narrower than selecting one format for every local database row, file, app package and transport frame. The [storage study](../../../research/storage-format-architecture-research.md#6-platform-cbor-profile-v1) proposed an object envelope and semantic-type rules, but explicitly left golden operation bytes unassigned. Reusing its ideas does not authorize reusing its provisional `PlatformEnvelope` as the signed operation layout.

| Candidate | Why consider it | What remains unproved for signed operations |
|---|---|---|
| PDM + deterministic CBOR | Standardized deterministic base; numeric field IDs; portable constrained subset; aligns with current storage research. | Exact canonical PDM value, schema binding, signed/encrypted boundary, Rust/Flutter/independent vectors and hostile-input behavior. |
| PDM + MessagePack profile | Compact runtime encoding and mature serializers may perform well in hot paths. | Map ordering, semantic extensions and canonicalization would be VIDA-specific; speed and size advantage must be measured on actual operations, not assumed. |
| PDM + Amazon Ion profile | Rich semantic types and useful shared-symbol behavior in homogeneous batches. | Signed single-operation determinism, symbol-table availability offline, duplicate-field handling and cross-language maintenance need stronger proof. |

**Proposed selection gate:** choose the least custom profile that passes identical semantic round-trip, byte-exact signed-input, malformed-input, mixed-version and crash/replay fixtures in Rust plus an independent implementation. Compare single operations and batched transport separately; synthetic size tables alone do not establish latency, allocation, security or signing correctness. If CBOR passes this gate without a material measured disadvantage, adopt it; otherwise record the alternative and the decisive evidence. BLAKE3 revision identity, COSE signatures and any concrete crypto suite each need their own explicit decision and vectors.

## Decision sequence before implementation lock

1. Define the typed PDM operation and receipt subsets, with field meaning, required/optional status, public routing visibility and maximum sizes. Do not assign final numeric tags before the meanings are fixed.
2. Compare a deterministic CBOR profile and signature construction against the same canonical values, malformed encodings and cross-language implementations. Record exact codec/library versions and raw bytes.
3. Specify the signed/encrypted boundary, domain-separation string or structure, key identity/epoch, hash suite and immutable outer-versus-inner wrapper rule. Verify wrong-Space, wrong-AppInstance and wrong-Resource replay cannot succeed.
4. Specify ALPN/profile major and compatible minor negotiation, required/optional feature handling, unknown-field round-trip, downgrade resistance and OQ-0037 activation/migration boundary.
5. Publish a versioned schema, positive byte-exact vectors, negative-security vectors and independent reader results; only then update `OQ-0028` and the affected contracts from draft/candidate to an approved profile.

## Minimum fixture families to design

| ID | Observable assertion |
|---|---|
| ENV-F01 | Same typed operation yields identical raw bytes, digest and signing input in Rust and an independent reader under one declared profile. No byte literal is normative before OQ-0028 selects the profile. |
| ENV-F02 | Non-shortest integer, indefinite length, wrong encoded-key order, duplicate key, invalid UTF-8 and forbidden tag/float/null form fail **before** lossy normalization under the chosen profile. |
| ENV-F03 | Excessive envelope size, nesting depth, map entries or dependency count fails within bounded work and memory; numeric limits are OQ-0028. |
| ENV-F04 | Substituting Space, AppInstance, Resource, author/device, control/key epoch, protocol major or payload digest invalidates the signed operation. |
| ENV-F05 | Direct and mailbox paths preserve the identical signed inner operation despite distinct outer routing wrappers; changing authenticated routing context fails, while harmless wrapper changes do not change `OperationId`. |
| ENV-F06 | Duplicate/reordered delivery applies once; missing causal dependencies remain pending, then repair applies once. Transport ACK alone creates no application receipt. |
| ENV-F07 | An unaccepted candidate with stale grant after effective revocation does not become accepted; an operation accepted before the removal cut remains valid history. |
| ENV-F08 | Unknown mandatory feature or incompatible ALPN/profile major fails explicitly; profile downgrade does not occur silently. |
| ENV-F09 | Permitted optional extension survives decode/encode without silent drop-and-resign or changed proof semantics. Exact raw-byte preservation strategy remains open. |
| ENV-F10 | If COSE is selected: unprotected-header substitution, duplicate protected/unprotected label, unknown `crit` and altered external-AAD context fail verification. If another signature frame wins, equivalent domain-substitution cases apply. |
| ENV-F11 | Receipt for another operation, request revision, audience or frontier cannot raise the current operation's replication, delivery or authority axis. |
| ENV-F12 | Every local commit/outbox crash cut is recoverable; blob reference binds declared digest/authorization context without embedding large bytes in the operation log. |
| ENV-F13 | Positive mixed-version case: two peers on one compatible major but with different optional-feature support derive the same declared permissible exchange profile when all required features are supported, then validate the same signed inner operation. Both peer orderings agree; substituting or downgrading the negotiated profile/features is rejected. Exact intersection algorithm, transcript bytes and support window remain open with OQ-0028/OQ-0037. |

The rows are a fixture **plan**, not executable golden vectors. ENV-F13 is a VIDA interoperability test proposal, not behavior supplied by CBOR, COSE or Iroh ALPN. OQ-0033/OQ-0034 still own authority topology and merge outcomes; this record only prevents OQ-0028 from inventing those semantics implicitly.
