---
id: SPEC-vida-platform-bindings
status: draft
companions:
  - binding-behavior.md
  - ../../planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md
  - ../../../docs/02-requirements/native-client-requirements.md
  - ../../../docs/02-requirements/platform-nfr.md
  - ../../../docs/04-specifications/vida-node-host-contract.md
  - ../../../docs/04-specifications/operation-finality-contract.md
  - ../../../docs/03-architecture/decisions/ADR-0021-static-web-client-in-release-1.md
  - ../spec-vida-storage-provider/storage-behavior.md
  - ../spec-vida-storage-provider/fault-injection-cases.md
sources: []
---

> **Decision-gated OQ-0035 kernel.** This draft defines observable Flutter↔Rust behavior, not a selected generator, ABI, exact DTO schema or passed release gate. Read the adopted companions for complete platform and domain requirements.

# VIDA platform binding contract

## Why

Release 1 has full Flutter clients on Android, iOS, Windows and static Web but one headless Rust implementation of VIDA semantics. A bridge that works only in a demo can still lose committed work, misreport delivery, expose secrets or change behavior after OS suspension. The client/runtime boundary therefore needs one testable contract before independent platform implementation expands.

## Capabilities

- **CAP-1**
  - **intent:** Each installed shell invokes the same authorized domain command and query contract.
  - **success:** The headless reference harness and Android, iOS, Windows and Web release builds return equivalent IDs, transitions and denials for the same seeded case without shell-side authorization or domain mutation.
- **CAP-2**
  - **intent:** A shell submits and tracks durable asynchronous work without depending on a widget or process lifetime.
  - **success:** Cancellation, process death and reconnect preserve every whole action proven durably committed, recover its outcome by stable ID after readiness and never cause a second domain apply for the same operation; an ambiguous result stays pending until verified.
- **CAP-3**
  - **intent:** A shell observes bounded state changes and recovers after missing an event.
  - **success:** Observer loss, queue overflow and restart produce an explicit catch-up or snapshot/reconciliation path; the UI never treats an unverified stale projection as current.
- **CAP-4**
  - **intent:** Boundary values, errors and resources have safe, consistent ownership and meaning.
  - **success:** Cross-platform conformance finds no borrowed-memory use, leaked secret, uncaught Rust panic, UI-thread stall, stale-handle access or collapsed transport/authorization/durability error.
- **CAP-5**
  - **intent:** Platform lifecycle adapters suspend, resume and reopen the runtime without changing product semantics.
  - **success:** Mobile suspend/kill/network change and Windows multiwindow/restart recover durable state with one effective writer for each local vault and truthful online/pending status.
- **CAP-6**
  - **intent:** A client knows whether its generated binding and native artifact can safely operate together.
  - **success:** Declared compatible artifact tuples pass version fixtures; mismatched or unknown mandatory contract versions fail explicitly before unsafe calls.

## Constraints

- Release 1 consists of full installed Flutter Android, iOS and Windows clients plus a full static Flutter Web client with one shared Rust core. Web uses a separately proven Rust/Wasm facade and browser platform profile; WinUI and Tauri are not Release-1 alternatives.
- The first-party shell calls only the `vida-sdk` facade. `vida-core` owns domain authorization, operation validation and transitions; the shell owns UI and OS-specific lifecycle, notifications, permissions and secure-store integration.
- Local durability, replication, recipient delivery, domain authority and external effect remain separate evidence axes. A bridge event, transport ACK or dropped observer cannot promote one axis into another.
- “Збережено локально” requires a durable result for every required component of the user action, including staged attachments; a missing callback or pre-commit event is insufficient. A post-commit ambiguity stays unknown until runtime replay/readiness and authoritative lookup by the same stable operation ID.
- Crossing values are owned, bounded and versioned; Rust references, endpoint secrets, private keys, allocator-owned buffers without release, executor internals and database handles do not become Dart DTOs.
- A mobile bridge cannot guarantee background execution or permanent reachability contrary to OS policy. Durable reconciliation and honest degradation are required.
- `VidaNodeHost` owns Iroh endpoint lifecycle and typed transport failures; its secret material stays behind its port. The Conformance Plane owns boundary `ContractId`, version and compatibility evidence.
- `flutter_rust_bridge` is a first prototype candidate, not an approved VIDA API or ABI. A different physical bridge may pass the same semantic contract; production binding choice remains OQ-0035.

## Non-goals

- Selecting FRB versus direct Dart FFI/C ABI, generated code layout, exact signatures, binary framing or platform package manager before a release-build prototype.
- Defining the signed operation wire format (OQ-0028), production mixed-version window (OQ-0037), storage provider (OQ-0036) or post-Release-1 WinUI bindings.
- Giving Flutter or a bridge generator authority to resolve domain conflicts, sign on behalf of another Persona or run network/background work the OS forbids.

## Success signal

One fixture set runs through the headless reference and Android, iOS, Windows and static Web release builds: commands, results, errors, restart recovery and event resubscription agree within each platform's declared lifecycle; hostile, stale-handle and mixed cached Web-asset inputs fail safely; each tested binding/artifact/toolchain tuple is recorded. Until those fixtures run, this document is a prototype contract, not OQ-0035 closure.

## Open Questions

- **Product behavior approved 2026-09-29:** [D5–D6, D9](../../planning-artifacts/implementation-readiness-epics-1-2-2026-09-29.md): browser tabs do not become extra Devices or receipts, restart resolves an unknown commit by OperationId, and direct Android↔Web means non-relay application payload in release-build evidence. Bridge and transport implementations remain unselected.

- Which tested bridge and generated-code/native packaging tuple pass the three-platform release-build fixtures?
- Which tested Rust/Wasm bridge and atomic Flutter/JS/Wasm asset tuple pass Web offline-reopen, storage and compatibility fixtures?
- What exact versioned command/event DTOs, handle/cancellation operations and bulk-data path implement this behavior without leaking secrets or duplicating domain logic?
- Which supported API/FFI compatibility window and activation policy will OQ-0037 approve for mixed-version releases?
