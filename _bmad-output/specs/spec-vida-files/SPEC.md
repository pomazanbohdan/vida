---
id: SPEC-vida-files
status: draft
companions:
  - file-conformance-cases.md
  - ../../../docs/01-product/v1-replacement-bundle.md
  - ../../../docs/01-product/composable-workspace-model.md
  - ../../../docs/02-requirements/native-client-requirements.md
  - ../../../docs/02-requirements/access-control-requirements.md
  - ../../../docs/04-specifications/blob-store-contract.md
  - ../../../docs/03-architecture/decisions/ADR-0001-layered-access-control.md
  - ../../../docs/03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md
  - ../../../docs/02-requirements/transport-sync-requirements.md
sources: []
---

> **Decision-gated contract.** File behavior approved for Release 1 is assembled here; cryptographic byte profile, download defaults and safe retention frontier remain open.

# VIDA Files

## Why

People need files that belong to their Spaces and can be reused in messages, notes and projects without fragmented copies. Offline capture and resumable transfer must not create a false “saved” state or lose a conflict variant.

## Capabilities

- **CAP-1**
  - **intent:** A user can browse the Files view of an authorized Space and manage each file as a distinct resource.
  - **success:** Each file retains a stable ID, versions and permissions, and the client distinguishes available local payload from metadata-only or failed-download state.
- **CAP-2**
  - **intent:** A user can attach the same file to several messages, notes or tasks.
  - **success:** Relations reuse one File resource without duplicating its payload; each reader still needs target File access, and unauthorized attachment previews/search cannot disclose protected File content or metadata.
- **CAP-3**
  - **intent:** A user can add a file offline and send it when a route becomes available.
  - **success:** “Saved locally” appears only after recoverable staged bytes, manifest reference and pending intent commit together; the committed attachment survives restart and later transfers without claiming early delivery.
- **CAP-4**
  - **intent:** A user can control whether a large remote file downloads to a device.
  - **success:** Metadata remains visible, a payload above the configured auto-download limit waits for explicit confirmation, and a failed save shows “not downloaded” with a reason without changing the shared File.
- **CAP-5**
  - **intent:** A user can receive a file despite interrupted transfer.
  - **success:** The client verifies the manifest and chunks, rejects corruption and resumes only from verified bytes under current authorization.
- **CAP-6**
  - **intent:** A user can compare incompatible file replacements and keep an alternative as a revision or independent copy.
  - **success:** Incomparable accepted variants remain recoverable and visibly unresolved; a revision retains the File ID, while a copy gets a new ID and separate rights check. Neither silently replaces the primary version.

## Constraints

- Immutable large payload transfer is separate from the signed operation log. The manifest binds digest, byte count/chunks, encryption/key reference, Space/resource authorization and pin/retention ownership.
- Possession of bytes or a stale transfer ticket never grants future fetch access. Fetch, revision, copy and removal require current Space/resource/action authorization.
- A readable message, note or task containing a File relation never grants access to that File. The exact creation and recipient-display UX when rights differ is still open; clients cannot infer a sharing grant from the link.
- Sensitive stored payloads must satisfy [OWASP MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/); resource actions must pass the [OWASP Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) deny-by-default and per-request checks. Exact cryptography remains prototype-gated.
- The auto-download size setting controls admission to download, not permission to discard a retained variant. Disk/quota/I/O/verification failure is a local “not downloaded” state and cannot mutate shared manifests or justify remote GC.
- Conflict payloads and pins remain recoverable until a safe retention frontier. Local conflict resolution or removal of one reference alone does not prove a variant is safe to collect.
- Deletion and revocation cannot promise physical erasure of already exported, replicated or backed-up plaintext. Convergent encryption is not a default without an explicit leakage decision.

## Non-goals

- General-purpose OS folder synchronization like OneDrive in Release 1.
- A fixed numerical auto-download threshold or a selected digest, chunk, key-wrap and ticket format before the corresponding decision gates.
- Guaranteed immediate erasure of all copies after deleting a reference.

## Success signal

On two devices, a user attaches a photo offline to a note and task, restarts, then transfers it over an interrupted connection. The receiving device verifies the bytes and the same File ID remains linked in both contexts. A second, incompatible replacement does not erase either version; the user sees an explicit conflict and may keep the alternative as a revision or separately authorized copy. The [conformance cases](file-conformance-cases.md) make each result observable.

## Open Questions

- Is the auto-download threshold per device or profile, and what default/mobile-data/low-space policy applies (`OQ-0066`)?
- Which digest, encryption, key wrapping, manifest and transfer-ticket format pass the blob security prototype (`OQ-0029`)?
- Which comparable authority proof orders or exposes conflicting File acceptances without treating sync arrival as authority (`OQ-0033`)?
- What retention frontier and proof permit GC after conflicting revisions, and how do copies inherit metadata and ACL (`OQ-0034`)?
- In a readable group chat, Богдан attaches a private File that Олена cannot read: should VIDA offer Богдан an explicit grant to the chat audience, allow an inaccessible relation, or require another action before attaching? What access-safe placeholder, if any, should Олена see (`OQ-0083`)?
