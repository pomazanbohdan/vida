# Operation and receipt semantic field dictionary — draft

This is a **meaning inventory**, not a CBOR/CDDL schema, numeric-tag assignment, signature input or cleartext routing layout. `Required` below means that the approved contracts need the information to validate a durable operation or receipt; it does **not** mean every value must be an inline field rather than a verifiable referenced control record. Exact bytes and placement remain `OQ-0028`.

## Signed inner operation

The same inner operation is carried by direct and mailbox paths; route-specific outer wrappers may differ without changing its stable identity ([ADR-0006](../../../docs/03-architecture/decisions/ADR-0006-durable-delivery-and-operation-envelope.md)).

| Semantic element | Required when | Validation purpose / approved boundary | Wire and visibility decision still open |
|---|---|---|---|
| Stable `OperationId` | Every durable operation | Deduplication and one idempotent apply across retry/reorder/restart (`SPEC-SYNC-LOG-001`; ADR-0006). | Raw ID bytes, creation rule, collision policy and whether ID derives from signed content. |
| Author Persona and Device binding | Every durable operation | Validate actor, signing key and current action grant; two Devices of one Persona are not two authority votes. | Key/identifier representation and what, if anything, a mailbox may see. |
| Space and operation context | Every durable operation | Prevent wrong-Space replay and select the applicable schema, policy and authority. | Exact Space/context fields and authentication/encryption placement. |
| AppInstance, Resource, process or request scope | When the operation acts on one of these | Prevent cross-instance/resource replay and bind an approval candidate to its named process. | Typed IDs and optional/required representation for membership, system or App operations. |
| Causal metadata / dependencies and frontier reference | Every durable operation | Missing dependencies stay pending; even an initial operation has a verifiable causal context rather than inferred packet arrival order. | Empty/genesis representation, parent list versus frontier digest, encoding, size bounds and canonical digest. |
| Current grant/control state and epoch reference | Every authority-relevant validation | Reject stale/revoked candidates at acceptance while preserving valid historical acceptance before a proven revocation cut. | Inline proof versus referenced verified control log; epoch bytes and partition proof (`OQ-0033`). |
| Protocol/schema/semantic-class and feature identifiers | Every durable operation | Fail closed for unknown mandatory semantics; validate against the applicable schema/policy and compatible protocol major. | Field IDs, optional-feature encoding, ALPN binding, compatibility transcript and support window (`OQ-0028/0037`). |
| Protected operation-body/payload binding | Every durable operation | Bind the exact operation body to identity and signature; relay/mailbox must not learn private content merely by forwarding. | Representation for contentless operations, E2E ciphertext boundary, digest, algorithm, key epoch, large-blob reference and minimal clear routing metadata. |
| Signature/authentication evidence | Every durable operation | Detect tampering and verify author/device/scope binding before visible apply. | Signature suite, signing-input bytes, domain separation, protected headers and key identity. |
| `RequestId` and request revision | Authority-gated request only | Correlate durable candidate with a later distinct `AuthorityOutcome`; candidate sync is not approval. | Whether represented in the operation body, request-specific extension or verifiable reference. |

`SyncLog.Accept` accepts a record into local validated/pending history; it is **not** `authority.accepted`. Local origin commit, replica apply, recipient delivery, authority outcome and external effect are separate evidence axes ([SyncLog](../../../docs/04-specifications/sync-log-contract.md), [Finality](../../../docs/04-specifications/operation-finality-contract.md)).

## Application Receipt common meaning

The approved [Operation Finality contract](../../../docs/04-specifications/operation-finality-contract.md) requires each receipt to bind the following semantics. The labels are explanatory, not assigned wire names or tags.

| Semantic element | Required condition | Validation purpose | Open representation question |
|---|---|---|---|
| Receipt identity, version and kind | Every receipt | Idempotent replay and kind-specific verification. | Raw ID/version/kind encoding and profile negotiation. |
| Referenced `OperationId`; optional `RequestId` + revision | Every receipt; request pair when applicable | Prevent evidence for another operation or revision from changing this operation's state. | Exact ID/digest references and whether inner operation bytes are needed to verify. |
| Issuer Persona/Device/Service binding and signature domain | Every receipt | Verify that this issuer is allowed to assert this specific evidence kind; transport or storage role alone is not authority. | Key binding, signature suite, protected headers and context bytes. |
| Space and AppInstance; Resource/process when applicable | Space and AppInstance on every Application Receipt; Resource/process when applicable | Prevent cross-Space, cross-AppInstance or wrong-resource evidence substitution. | Representation for Core/system receipts and confidentiality placement; optional Resource/process binding. |
| Recipient principal, Device/replica and immutable audience revision | Delivery-related receipt when applicable | Bind delivery to the intended recipient and audience; sender-visible aggregation must not expose exact recipient Device topology. | Exact audience/recipient representation and privacy-preserving routing. |
| Control epoch, schema/semantic-class/policy version | Every receipt's applicable validation context | Recheck relevant control and interpretation before raising an evidence axis. | Inline versus referenced proofs, version negotiation and stale-control handling. |
| Causal frontier digest | Every receipt | Prove the state up to which evidence applies, including all causal dependencies for replica `applied_at_frontier`. | Canonical frontier bytes and digest suite. |
| Evidence state/decision and payload/outcome/result digest | Every receipt, kind-specific | Prevent a receipt for one state/result from being reinterpreted as another. | Tagged union shape, allowed kind/state pairs and content-digest algorithm. |
| `issued_at` audit metadata | Every Application Receipt | Display and audit only; cannot establish authority order. | Timestamp precision/encoding; no clock-based winner. |
| Cryptographic signature | Every receipt | Authenticate all authority-critical bindings. | Exact signing input, algorithm, key epoch and canonicalization. |

## Kind-specific proof boundary

| Receipt / outcome kind | Issuer and claim required by approved semantics | It does **not** prove |
|---|---|---|
| `ReplicationReceipt` | Authorized application replica attests durable validation/persistence; `applied_at_frontier` additionally attests apply of all dependencies to its named frontier. Independence is checked separately when evaluating Replication Policy. | A transport ACK, `validated_persisted` alone or ciphertext-only mailbox storage is not `Synchronized`. |
| `DeliveryReceipt` | `stored_for_delivery` may be mailbox ciphertext storage; `delivered` requires a current recipient-controlled Device to decrypt, validate and durable-save. | Mailbox storage is not recipient delivery; one Device receipt is not delivery to every Device or a business approval. |
| `AuthorityOutcome` | Named logical Resource Authority decides `accepted`, `rejected` or `expired` for the exact `RequestId`, revision and frontier. | Replication, Iroh ACK or co-location with a node does not make the node authority; `conflicted` is reconciliation of accepted branches, not an arbitrary outcome decision. |
| `EffectReceipt` | Executor/provider result binds a stable effect/idempotency key and causal confirmation frontier; an uncertain unsupported external result is `unknown`. | Source authority acceptance is not proof the outside API ran; a stable key alone is not a provider exactly-once guarantee. |

`Read` is a separate Persona-level domain operation after actual display/open, not a transport-level delivery receipt. Sender-facing read evidence follows the privacy policy. `issued_at` and client timestamps never choose a winner.

## Boundaries that remain unselected

1. PDM type subset, exact field IDs, canonical encoding, duplicate-key and unknown-extension behavior, bounds, hashes, signature domain and encryption suite (`OQ-0028`).
2. Which semantic references are inline versus dereferenced from verified control/snapshot state, and which minimal outer routing fields remain visible to a mailbox; the private resource payload must not become mailbox plaintext.
3. ALPN/profile and mandatory/optional feature transcript with downgrade protection and mixed-version activation (`OQ-0028/0037`).
4. Serverless serialized Resource Authority, grant/revocation cut, resolver/frontier proof and safe history pruning (`OQ-0033/0034`); this dictionary does not decide them.

Use the planned `ENV-F01–F13` in [wire-profile-decisions.md](wire-profile-decisions.md) to turn this semantic inventory into byte-exact positive/negative vectors after a profile is selected. No listed fixture has been executed.
