---
id: SPEC-PERSONA-CONTROLLER-RECOVERY-WIRE-V1
status: experimental-headless-proof
date: 2026-09-29
---

# Persona controller and recovery wire v1 (headless proof)

This document describes the bytes emitted by `vida-core` 0.1.0. It is an experimental Core profile, not a production protocol approval or closure of OQ-0022/0024. Integers are unsigned little-endian. IDs are 16 opaque bytes; digests and public keys are 32 bytes. No wall clock or Device ordering selects a conflict winner.

## Controller event

The Ed25519 signature covers `signed_bytes`, in this exact order:

| Field | Bytes |
|---|---:|
| ASCII `VIDA-CTRL-EVENT-v1` and NUL | 19 |
| Persona ID, signer Ed25519 public key, sequence | 16 + 32 + 8 |
| Parent count and lexically sorted parent event IDs | 1 + 32 × count; count ≤ 64 |
| Controller epoch, command ID | 8 + 16 |
| Action tag and payload | below |

Action tags: `0` Genesis = Device ID + Device public key + recovery public key (16+32+32); `1` Grant = ASCII `VIDA-ENROLL-INTENT-v1` + NUL + intent Persona ID + Device ID + new public key + Space scope + nonce + expiry u64 + 64-byte new-key Ed25519 signature + approved-at u64; `2` Revoke = Device ID + scope; `3` RecoveryGrant = Device ID + public key; `4` Rotate = new recovery public key + SHA-256 of encrypted bundle bytes; `5` Resolve = Device ID + scope + two lexically sorted branch event IDs + presence byte (`0` deny; `1` followed by 32-byte chosen key). The event signature follows `signed_bytes` as 64 bytes. Event ID = SHA-256(`signed_bytes || signature`). Scope zero is controller scope. The new-key intent signature covers its domain through expiry, excluding its own signature; expiry is checked against the trusted Device's supplied approval time. `command ID` and intent nonce are rejected on replay.

The root Genesis is self-signed. Subsequent events must reference known causal parents and have a signer authorized on that parent closure. Recovery-signed grants remain provisional. Grants for different `(Device ID, scope)` pairs compose. Concurrent incomparable transitions for the same pair yield `Conflict`; a trusted unaffected Device can sign a Resolve event referring to the current conflicting heads and selecting only a key already bound by one of those heads. Causally repeated enrollment nonces are rejected; identical signed intents approved concurrently are retained as branches. One signing key cannot be causally rebound to another Device ID, and concurrent cross-Device reuse fails closed as `Conflict`. Event maps and parent lists are sorted, and merge checks each signature and parent closure. The public `History` structure must be validated after decoding; callers must pin the expected Genesis/Persona independently. `ControllerState` is derived from events. `frontier_hash` = SHA-256(ASCII `VIDA-CTRL-FRONTIER-v1` + NUL + lexically sorted maximal event IDs).

Golden deterministic event (`tests/conformance.rs::golden_event_v1`): Ed25519 seed = 32 × `07`, Persona = 16 × `11`, command = 16 × `22`, Device = 16 × `33`, recovery public key = 32 × `44`, no parents, sequence/epoch zero. Exact `signed_bytes` hex:

```text
564944412d4354524c2d4556454e542d76310011111111111111111111111111111111ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c0000000000000000000000000000000000222222222222222222222222222222220033333333333333333333333333333333ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c4444444444444444444444444444444444444444444444444444444444444444
```

Signed event ID hex = `3a8c3dade6b173e01da2a688221dcebacea1e2f7464b0cf86c6ce8ac277c56e3`.

Exact golden encrypted bytes are in [bundle-v1.hex](../../crates/vida-core/tests/fixtures/bundle-v1.hex) and [note-v1.hex](../../crates/vida-core/tests/fixtures/note-v1.hex); `recovery::wire_tests::golden_bundle_and_note_v1` compares every byte and authenticates/decrypts both. The fixture uses secret = 32 × `09`, recovery signing seed = 32 × `08`, data key = 32 × `0a`, bundle salt = 16 × `0d`, bundle nonce = 24 × `0e`, Note nonce = 24 × `0f`, Note ID = 16 × `0b`, Note resource frontier = 32 × `0c`, Note content = ASCII `golden`. Fixed entropy is test-only; production exports draw fresh entropy from the OS CSPRNG.

## Secret, bundle and resource bytes

The recovery secret is exactly 32 bytes from the OS CSPRNG and is held separately from the bundle. HKDF-SHA-256 with a fresh 16-byte random bundle salt and info `VIDA-bundle-aead-v1` derives a 32-byte bundle key. The bundle contains a distinct Ed25519 recovery signing seed and 32-byte data key; it contains no Device private key. The Note key uses HKDF-SHA-256(data key, salt = Persona ID, info `VIDA-note-aead-v1`). Bundle and Note use XChaCha20-Poly1305 with independent fresh random 24-byte nonces. Entire headers below are AEAD AAD. Ciphertexts include the 16-byte tag.

| Object | Header offsets (zero-based) | Limit |
|---|---|---|
| Bundle | `[0..5]` ASCII `VDBN1`; `[5]` mandatory version `01`; `[6..22]` Persona ID; `[22..38]` salt; `[38..62]` nonce; `[62..66]` ciphertext length u32; ciphertext starts at 66 | plaintext ≤ 4 MiB |
| Note | `[0..5]` ASCII `VDNT1`; `[5]` mandatory version `01`; `[6..22]` Persona ID; `[22..38]` Note ID; `[38..70]` covered resource operation frontier hash; `[70..94]` nonce; `[94..98]` ciphertext length u32; ciphertext starts at 98 | content ≤ 1 MiB |

Bundle plaintext uses pinned `bincode` 1.3.3 fixed-width little-endian serde field order, with trailing bytes rejected: `History { Persona ID, BTreeMap<Event ID, Event> }`, controller frontier hash, controller epoch u64, 32-byte data key, 32-byte recovery signing seed, `Vec<Coverage { Note ID, resource frontier hash }>` manifest, pending-rotation bool. Serde Vec/map lengths are u64. This headless profile permits at most one covered Note; multiple entries are rejected until multi-resource restore is implemented. History validation, controller frontier, epoch, Persona, Genesis and recovery signer binding occur before restore publishes any state. The Note ciphertext must match its manifest ID/frontier pair. The resource frontier is caller-supplied and authenticated by the export, but its provenance from a real operation log is not yet proven. Unknown mandatory versions and truncated lengths fail without partial import. The implementation zeroizes the secret, derived keys, bundle plaintext buffer and decoded sensitive seed/key fields when feasible; caller-managed Note plaintext and exported bytes need platform custody policy.

`restore` creates a new Device ID and Ed25519 key and appends a recovery-signed provisional grant. It reports authority, key recovery, optional content, and the actual content frontier separately. If the authenticated manifest claims a Note and its ciphertext is absent, restore returns `MissingResource` before any new grant or import; an empty manifest permits Persona-only restore. A bundle prepared for rotation is only importable with a validated post-commit controller history whose signed Rotate event names its exact ciphertext hash. Old bundles fail against such a history. If fresh history is unavailable, an older initial bundle can create only a provisional branch; a prepared rotation bundle currently returns `Stale`. This is a deliberate proof gap for OQ-0022/0024, not an availability guarantee.

## Storage and crash profile

`StateStore` and `MemoryStore` are injectable reference abstractions, not OS power-loss guarantees. `StateStore` validates a candidate controller event, stages the new history, and promotes it atomically in memory; a reopened crash cut retains the prior state and a repeat of the committed command is idempotent. Export stages bytes, reopens a copy, runs a caller-supplied authenticated parse/coverage oracle, then promotes a `VerifiedSnapshot`; the prior verified snapshot is retained. Cuts before write, after write, after readback and before promotion never replace the prior verified state. Destinations without readback report export only. An independent off-Device witness must be supplied separately; local readback never proves it. The REC-F23 oracle actually decrypts the exported Note with its bundle and checks Note ID/content/resource frontier; tamper and readback failure retain the prior verified snapshot. An edit with a later resource frontier remains locally durable but outside that backup claim, even when the controller frontier is unchanged.

Normal rotation prepares a new secret/bundle, requires readback and owner custody confirmation, then signs a Rotate event only if the frontier and controller epoch are unchanged. The old kit remains valid before that event. A changed frontier requires a new export and confirmation. `repair_from_trusted` can prepare another kit from a data key held by a still-trusted Device; its commit uses the same guard and never reactivates a revoked kit. `RotationHealth` starts `Pending` after a signed commit, runs an authenticated restore test, and projects any failure as sticky `Critical`; the headless fixture verifies this state. Data-key epoch is not independently modeled yet; rotation under an external key-epoch change must be rejected by an adapter until this proof is extended. Platform UI presentation and the actual release-gate runner remain unproven.

## Evidence and remaining gates

`cargo test --workspace --all-targets` writes `target/rec-case-matrix.json` with every REC-F01–F29 plus REC-F09a, each as `passed`, `failed` or `unproven` with a reason. A test panic records `failed` and fails the command. The headless fixtures cover intent/replay, tamper/version, fresh-profile Note roundtrip, provisional concurrent restore, grant/revoke conflict and reconciliation, independent scoped grants, backup frontier, readback failure, snapshot retention, and guarded rotation. REC-F08 remains `unproven`: its in-memory crash-cut test runs, but an actual persistent restart adapter does not exist. No `unproven` row is counted as passed. `cargo check --target wasm32-unknown-unknown` is a separate gate when the target is installed; otherwise its status is `unproven`.

Security review map: [OWASP Key Management](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html) → CSPRNG, distinct key purposes, rotation/custody and compromise response; [OWASP Cryptographic Storage](https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html) → maintained AEAD, random nonces, authenticated metadata and encrypted copies; [libsodium AEAD nonce guidance](https://doc.libsodium.org/secret-key_cryptography/aead) → never repeat a nonce with one key. The crate uses RustCrypto implementations, with no custom cipher. Independent crypto review, platform secure storage, Android/Web custody, durable provider atomicity, real independent replica/readback, partition invitation single-use, authenticated current frontier, compromise cut, data-key epoch guard, broader resource coverage, and end-to-end release restore remain open. OQ-0022/0024 retain their production status.
