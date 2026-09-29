---
title: 'Executable Persona controller and recovery proof'
type: 'feature'
created: '2026-09-29'
status: 'done'
route: 'dispatch'
review_loop_iteration: 0
baseline_commit: '9c4ffd0cd626585536877b9f77ab9cb3e18a70fe'
---

<frozen-after-approval reason="human-owned intent - do not modify unless human renegotiates">

## Intent

**Problem:** Epic 1–2 describe Persona enrollment/recovery, but OQ-0022/0024 have no executable proof. Stories 1.1/1.2 are not production-ready.

**Approach:** Build a headless Rust Core kernel and conformance harness for signed controller history, scoped grants, separate secret/bundle/resource ciphertext and crash-safe recovery. This precedes Android/Web adapters; it does not close the production OQs.

## Boundaries & Constraints

**Always:** Equal Devices; causal authority, never clock/device priority. Same-Device grants compose across Space scopes but revoke independently. Restore creates new Device keys; bundle contains a distinct recovery credential, never old Device signing keys. Stale/partitioned authority stays provisional until signed reconciliation. Secret, bundle and resource ciphertext remain separate; local save is neither backup nor sync. Rotation keeps old kit valid until signed commit; changed frontier/epoch requires new export/custody confirmation. Unknown mandatory bundle version causes no partial import.

**Never:** Secret escrow, implicit server authority, plaintext/key logging, homegrown crypto, silent deletion of verified snapshots, or mock-only release proof. Do not change website/presentation assets.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|---|---|---|---|
| Enrollment | trusted Device, intent bound to new key/Space | scoped grant; second scope separately grantable | replay/foreign/expired intent rejected |
| Partition | concurrent revoke/renew or restores | retain branches; provisional until signed reconciliation | no clock/arrival winner |
| Recovery | secret, bundle, encrypted Note copy | new Device keys; restore Note ID/content | wrong secret, tamper, missing bytes or version → no partial import |
| Backup | verified frontier F; edit F+1 | readback/open/coverage prove F only | failed export retains prior snapshot; edits continue |
| Rotation | crash around signed commit | old kit valid before; stale restore provisional after | failed restore test → critical risk; trusted-Device repair, no rollback |

</frozen-after-approval>

## Code Map

- `docs/00-governance/open-questions.md` — OQ-0022/0024 production proof remains open; do not mark resolved prematurely.
- `_bmad-output/specs/spec-vida-persona-recovery/{SPEC.md,recovery-cases.md,recovery-bundle-contract.md}` — approved behavior, REC-F01–F29 candidates and custody/coverage rules.
- `docs/04-specifications/operation-finality-contract.md` — local durability, replication and authority are separate evidence axes.
- `Cargo.toml`, `crates/vida-core/` — new workspace/kernel; no product Rust/Flutter runtime exists yet.

## Tasks & Acceptance

**Execution:**
- [x] `Cargo.toml`, `crates/vida-core/Cargo.toml` — create workspace with pinned maintained crypto/codec dependencies and a platform-neutral API.
- [x] `crates/vida-core/src/controller.rs` — versioned signed event bytes, causal parents, scoped grants/revocations, deterministic merge, provisional reconciliation, replay and epoch validation.
- [x] `crates/vida-core/src/storage.rs` — atomic commit/export abstraction, readback oracle and injectable crash cuts; failed replacement retains the prior verified snapshot.
- [x] `crates/vida-core/src/recovery.rs` — 32-byte CSPRNG secret; purpose-separated KDF keys; authenticated versioned bundle, encrypted Note snapshot, new-Device restore and rotation prepare/commit/repair. Zeroize sensitive buffers where feasible.
- [x] `crates/vida-core/tests/` — executable REC-F mapping, tamper/replay/partition/rotation permutations and golden round trips. Platform-only cases report `unproven`, never `passed`.
- [x] `docs/04-specifications/persona-controller-recovery-wire-v1.md` — actual byte layout, signature/KDF/AEAD/nonce/AAD profile, size limits, compatibility, threat assumptions, fixture mapping and unproved platform gates.

**Acceptance Criteria:**
- Given the REC-F inventory, when `cargo test --workspace --all-targets` runs, then a machine-readable case matrix records `passed|failed|unproven` and reasons; skipped cases never count as proof.
- Given a new Persona and Note, when bundle and ciphertext are exported separately and restored in a fresh profile, then Note ID/content match and verified versus provisional authority is explicit.
- Given merge, replay and crash permutations, when events are reconciled, then outcomes agree without clock or Device priority.
- Given the Wasm target is installed, when Core is checked for `wasm32-unknown-unknown`, then compilation succeeds; without the target, the gate reports `unproven`.
- Given security review, when the proof is reported, then OWASP key/storage checks are mapped and Android/Web custody, independent review and end-to-end release gates remain open.

## Implementation Notes

- Headless Rust Core and conformance harness implemented against the approved I/O matrix. `target/rec-case-matrix.json` is generated by a running test: 14 `passed`, 16 `unproven`, 0 `failed` on this host. REC-F08 is `unproven` until a persistent restart adapter exists; its in-memory crash-cut model runs as a separate test.
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace --all-targets` passed. `wasm32-unknown-unknown` is not installed here, so the Wasm compile gate remains `unproven`.
- The storage proof uses an in-memory crash-cut model; Android/Web durable storage, independent recovery custody, actual multi-peer reconciliation, external crypto review, and end-to-end release gates remain open. This does not make Stories 1.1/1.2 production-ready.

## Spec Change Log

## Review Triage Log

| Finding | Verdict and evidence | Route |
|---|---|---|
| BH-01 expiry backdating | `maybe-false`: a signed approver can assert an earlier `approved_at`; the proof assumes a trusted approving Device, and a trusted-time witness would be needed to distinguish a dishonest clock from delayed delivery. | defer: distributed trusted-time model in OQ-0022 |
| BH-02 concurrent nonce | `high`: `History::insert` rejects a nonce already present even when two grants are causally concurrent, so `merge` drops a valid signed branch. | patch: allow concurrent branch, reject causal reuse |
| BH-03 concurrent rotations | `medium`: two rotation heads make `epoch_at` fail and no rotation-specific Resolve exists. The approved proof covers commit crash cuts, not multi-rotation reconciliation. | reject: production distributed-rotation gate remains outside this headless proof |
| BH-04 arbitrary Resolve key | `high`: a controller signer can select a key absent from both referenced branches and grant verified access without its enrollment intent. | patch: constrain selected key to a signed branch key |
| BH-05 stale Resolve refs | `medium`: `Resolve` checks subject but not that both IDs are current conflicting heads; it can revive an obsolete branch. | patch: require current conflict heads |
| BH-06 reused Device key | `medium`: one public key can be assigned to two Device IDs while `signer_device` picks only one. | patch: reject cross-Device key reuse |
| BH-07 event cap | `low`: the 4096-event proof bound eventually refuses all operations; no production runtime or compaction promise is in this build. | reject: bounded headless profile, production rollover gate open |
| BH-08 multiple Notes | `high`: manifest accepts multiple Notes while `restore` verifies only one resource; it can report success with missing covered data. | patch: enforce this proof's single-Note coverage limit |
| BH-09 unrelated known history | `high`: matching Persona/recovery key does not prove the bundle and supplied history share a Genesis. | patch: compare Genesis identity |
| BH-10 rotation resource readback | `medium`: `verify_export` authenticates the bundle, not every resource; actual independent resource custody is explicitly a separate unproven production gate. | reject: outside the approved headless rotation-commit proof |
| BH-11 arbitrary Note frontier | `medium`: `export_note` accepted an unvalidated history and a caller-supplied frontier; the headless proof can authenticate a supplied frontier but cannot establish a missing operation log. | patch: validate history and state the provenance limit |
| BH-12 snapshot risk coverage | `medium`: `MemoryStore::risk` ignores resource coverage, permitting a false no-risk result. | patch: remove the misleading unused projection |
| EC-01 multiple Notes | `high`: same missing-coverage outcome as BH-08 at `restore`. | patch: same single-Note guard |
| EC-02 reused Device key | `medium`: same ambiguous signer binding as BH-06. | patch: same unique-key guard |
| EC-03 RecoveryGrant ID collision | `high`: a recovery grant can name an existing Device ID and replace its verified controller state. | patch: reject existing Device ID |
| EC-04 arbitrary Resolve key | `high`: same unauthorized selected-key outcome as BH-04. | patch: same branch-key guard |
| EC-05 mutated export after readback | `high`: `PreparedRotation.export.bytes` is public and mutable after verification, so commit may sign an unverified hash. | patch: bind commit to the verified hash |
| EC-06 direct Rotate | `maybe-false`: a fully trusted signing Device can emit Rotate directly; preventing a malicious signer from making an unsafe declaration would require a separate authority/proof protocol, not just an API guard. | defer: trusted-signer threat boundary in OQ-0024 |
| EC-07 REC-F08 label | `medium`: an in-memory crash cut does not prove persistent restart behavior, yet REC-F08 was marked passed. | patch: mark REC-F08 unproven; keep model test separately |
| VG-01 rotation guard test | `medium`: all commit tests already confirm readback/custody, so removing either guard would leave tests green. | patch: negative commit tests |
| VG-02 Passed transition test | `medium`: no test asserted `RotationProofStatus::Passed` from Pending. | patch: successful post-commit test |
| VG-03 restored key isolation test | `medium`: fresh restore checked Device ID but not that the new signing key differs from both prior Device and recovery keys. | patch: key-distinctness assertions |

## Design Notes

Controller authority, bundle availability, resource coverage and replica synchronization need separate witnesses. Use Ed25519 signatures, HKDF-SHA-256 with distinct context labels, and XChaCha20-Poly1305 with a fresh random 192-bit nonce per encryption from maintained RustCrypto crates; pin versions, authenticate all header/context fields as AAD, and document exact golden bytes. Partitioned peers cannot prove global invitation single-use: isolated enrollment remains provisional until histories reconcile, without creating a central arbiter. Review against [OWASP Key Management](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html), [OWASP Cryptographic Storage](https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html), and [libsodium's AEAD nonce guidance](https://doc.libsodium.org/secret-key_cryptography/aead).

## Verification

**Commands:**
- `cargo fmt --all -- --check` — formatting.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean lint.
- `cargo test --workspace --all-targets` — conformance and fault tests.
- `cargo check --target wasm32-unknown-unknown` — only when target is installed; absence is reported, not passed.
