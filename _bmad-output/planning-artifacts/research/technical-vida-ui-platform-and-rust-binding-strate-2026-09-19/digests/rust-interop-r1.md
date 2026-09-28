# Rust interoperability for UI shells — research digest r1

Date: 2026-09-19  
Scope: Android, iOS, desktop, browser; UI toolkit intentionally undecided.  
Working hypothesis evaluated: Flutter mobile + shared Rust core/runtime + Tauri Windows.  
Evidence policy: conclusions below use sources retrieved this run; no project files were read.

## Compact verdict

**Recommend one semantic Rust application API, not one physical wire ABI.** Keep a toolkit-neutral `core`/`runtime` crate and place thin transport adapters around it:

```text
Flutter/Dart ─ flutter_rust_bridge adapter ┐
Swift/Kotlin ─ UniFFI adapter              ├─ application facade ─ domain/core/runtime
Tauri JS    ─ command/channel adapter      │
Browser JS  ─ wasm-bindgen adapter         ┘
```

For the working hypothesis, **FRB is the default Flutter bridge**, **Tauri calls the Rust facade in-process**, and **wasm-bindgen is the browser bridge**. Retain an intentionally tiny, versioned C ABI only for embedding, diagnostics, or consumers not covered by generators. UniFFI is the best evidence-backed option if native Swift/Kotlin shells remain plausible, but it is not a Dart or browser unifier. Do not force Tauri through FFI merely to imitate Flutter: that adds serialization/lifetime machinery without increasing reuse.

The reusable contract is Rust types and use-case semantics: owned request/response records, stable error codes, opaque IDs/handles, explicit operation IDs, snapshots/deltas, and bounded event streams. The generated FRB/UniFFI/Tauri/wasm-bindgen surfaces are adapters and may differ idiomatically. UI objects, runtime-specific callbacks, platform contexts, view models, navigation, arbitrary closures, borrowed references, file descriptors, and executor handles must not cross the shared contract.

## Decision matrix

Scores: 5 strong; 1 weak. “Release independence” means ability to evolve/publish the Rust core and shell separately without unsafe binary mismatch.

| Architecture | Android/iOS | Desktop | Browser | Async/streams | Ownership safety | Release independence | Verdict |
|---|---:|---:|---:|---:|---:|---:|---|
| UniFFI | 5 | 3 | 2 | 4 | 4 | 3 | Best native Swift/Kotlin generator; current Web support is external/unstable; no first-party Dart |
| FRB over Dart FFI | 5 | 4 | 3 | 5 | 4 | 3 | Best fit for Flutter; Dart-specific generated surface; Web path has separate constraints |
| Narrow C ABI + generated/manual wrappers | 4 | 5 | 2 | 2 | 2 | 5 if versioned | Lowest-common-denominator escape hatch; highest engineering/test burden |
| Direct JNI + Swift C/C++ wrappers | 4 | 4 | 1 | 3 | 2 | 3 | Use only for measured hotspots or platform integration; duplicates language-specific glue |
| JNA | 3 | 3 | 1 | 2 | 2 | 3 | Convenient C mapping, but callback/thread/type traps and overhead; not preferred for a new Flutter path |
| Tauri commands/plugins/state | n/a | 5 | n/a | 4 | 5 in Rust; serialized IPC | 3 | Desktop shell adapter, not the canonical core API |
| wasm-bindgen | n/a | 3 | 5 | 4 | 3 | 4 at package boundary | Browser-native choice; design for workers and restricted `std` |
| Kotlin/Native C interop | 2 | 3 | 1 | 2 | 2 | 3 | Relevant to KMP/native, not Android JVM/Flutter; C import itself remains Beta |

## Decisive findings

### 1. Contract layering

- **Core boundary:** deterministic domain logic, validation, policy, crypto, sync/state machines, persistence orchestration, networking abstractions, search/indexing, and durable models belong in Rust. This matches 1Password’s “everything feasible short of UI”, Mozilla Application Services, Matrix Rust SDK/Element X, and Signal’s shared Rust implementation.
- **Shell boundary:** navigation, rendering, accessibility, localization formatting, permissions prompts, lifecycle, deep links, window/view state, clipboard/share sheets, push-token acquisition, and framework objects stay native to the shell.
- **Adapter boundary:** translate shell DTOs to application-facade commands; map `Result` to native errors; turn operations into futures/streams; enforce backpressure/cancellation; contain runtime-specific handles. Keep adapters thin and separately tested.
- **Why one physical ABI fails:** UniFFI’s first-party languages are Kotlin/Swift/Python; Dart needs FRB/Dart FFI; Tauri uses serde-based IPC; browser Rust uses wasm-bindgen/JS glue. Their lifetime, concurrency, and serialization models are materially different. A single C ABI can technically underlie several, but would erase richer error/async/stream semantics and shift unsafe glue into every shell.

### 2. UniFFI

- Current official guide: full first-party support is Kotlin, Swift, Python; Ruby is maintained but feature-limited; other generators are third-party. UniFFI generates bindings/scaffolding but explicitly does **not** ship/package libraries.
- Current version evidence: changelog’s latest release is **0.32.1 (2026-09-08)**. 0.32.0 changed several binding/config contracts and added synchronous zero-copy borrowed bytes; 0.31.0 had earlier breaking generator/metadata changes; 0.31.1/0.31.2 fixed Swift async memory/crash issues and Kotlin ARM32/async-return issues. Pin generator/runtime versions together and regenerate in CI.
- Data model: owned primitives, strings, byte buffers, records, enums, lists/maps, objects and errors. Most compound values/errors are lowered/serialized through `RustBuffer`; 0.32 adds pointer+length zero-copy for synchronous borrowed bytes in Kotlin/Swift/Python, but not async functions. `Result<T,E>` becomes native exceptions. Panic trapping uses `catch_unwind`, but cannot help with `panic=abort`.
- Ownership: object instances are wrapped in `Arc`; generated foreign handles clone/decrement strong counts. Exported objects must be `Send + Sync`. Foreign callback ownership is transferred through vtables/handles; legacy callback interfaces are soft-deprecated in favor of foreign traits.
- Concurrency: generated async maps Rust futures to Swift async/await and Kotlin suspend. The foreign runtime supplies polling; a Rust runtime is not inherently required. Foreign callbacks may arrive on arbitrary threads, so the foreign implementation must be thread-safe even where its language cannot prove it.
- **Cancellation:** current guide says UniFFI has no built-in future cancellation and prescribes a library-specific `cancel()`/flag channel. Therefore cancellation must be part of VIDA’s semantic contract (`operation_id`, `cancel`, terminal event), not assumed from dropping Swift/Kotlin futures.
- Streams: UniFFI has callbacks/foreign traits and async, but no universal first-class cross-language `Stream<T>` contract comparable to FRB’s generated Dart Stream. Prefer a subscription object with `next_batch()` or an event sink plus explicit `close`; document ordering, buffer bounds, overflow and terminal semantics.
- Web: UniFFI itself has no “WASM generator”; external generators are required, and the documented scaffolding flag is `wasm-unstable-single-threaded`. This is not a sound basis for making UniFFI the browser standard today.

### 3. Flutter: FRB versus raw Dart FFI

- Dart FFI is a C ABI. `ffigen` can generate declarations from C headers; Dart build hooks/code assets can build/bundle native libraries and bind them via `@Native`. `NativeCallable` distinguishes same-thread callbacks from any-thread/listener forms and must be explicitly closed; invoking a closed callback is undefined behavior.
- Raw Dart FFI is suitable for a very small stable ABI or measured zero-copy/hot paths. It leaves allocation/free pairing, callback lifetimes, isolates, async dispatch, errors, streams, codegen and packaging to the application.
- FRB generates Dart and Rust wire layers over Dart FFI. Its default Dart API is async, supports Rust async functions, maps results/errors, provides `StreamSink<T> -> Stream<T>`, and supports opaque Rust handles.
- FRB’s default CST/DCO codec minimizes copying for cases such as large byte vectors; its SSE codec serializes through a byte buffer and may help many-small-object cases. Codec choice is an adapter optimization and must not leak into the domain API.
- Opaque Rust values are Rust-owned behind Dart handles/finalizers; docs warn that native memory is not counted by Dart GC and recommend explicit disposal. Async opaque types may need `Send + Sync` because executor threads can share them.
- FRB cancellation is not automatic structured cancellation: current docs recommend an application token (including Tokio `CancellationToken`). Model it explicitly, as for UniFFI and Tauri.
- Version risk: releases show **2.13.0 (2026-08-23)** as current stable and **2.14.0-beta.2 (2026-09-12)** as latest prerelease. The prerelease includes Dart-Wasm and stream-cancellation fixes; recent releases show build-template compatibility moving with Flutter/SPM/AGP. Pin Flutter, Dart, FRB codegen/runtime, NDK, Xcode and packaging templates as one tested matrix.
- Recommendation: use FRB for Flutter, but expose only an adapter module (`api_flutter`). Keep Rust application methods free of `StreamSink`, `DartOpaque`, isolate ports and FRB attributes where possible; wrappers translate to the neutral facade.

### 4. Tauri Windows

- Tauri commands accept serde-deserializable arguments and return serde-serializable results over IPC; ordinary return values use JSON, while `tauri::ipc::Response` avoids JSON for large byte buffers. Async commands run as separate async tasks; sync commands otherwise run on the main thread.
- Managed state is appropriate for an `Arc<AppRuntime>`/service handle. Tauri plugins are for reusable shell/platform capabilities and add command permissions/scopes; do not turn every domain service into a Tauri plugin.
- Channels/resources are useful for progress and opaque handles, but resources require explicit close for early cleanup. A JS `Promise` or closed webview does not by itself define cancellation of underlying Rust work; use the same operation registry and cancellation token as Flutter.
- Recommendation: commands should call the Rust facade directly in-process. Keep `#[tauri::command]`, `State`, `Channel`, `AppHandle`, webview permissions and serde casing in `api_tauri`; they must not appear in shared domain/runtime crates.
- One contract can serve Flutter and Tauri **semantically**: the same commands, DTO meanings, error codes and event schemas. It should not be the same generated signature. FRB can expose typed Dart objects/streams; Tauri wraps the same Rust calls in IPC/serde and can choose batched or binary responses.

### 5. Browser/WASM

- Use `wasm-bindgen`/`web-sys` for first-class JS/TypeScript bindings. It handles strings, classes, JS values and closures and can generate TypeScript declarations.
- `wasm32-unknown-unknown` has inert/unsupported `std::fs` and `std::net`; inject storage/network/time/entropy capabilities or compile platform-specific implementations behind traits.
- Browser main thread cannot block. Threaded Wasm needs workers, shared memory and cross-origin isolation; the official guide documents substantial setup and ecosystem caveats. Plan a single-threaded/worker execution profile first; do not assume native Tokio/thread behavior.
- Rust/JS closure lifetimes and Wasm object memory still need explicit discipline. Weak references/finalization can reduce leaks where supported, but explicit `free`/unsubscribe remains the deterministic API.
- Browser output should be an independently versioned npm/package artifact containing `.wasm`, JS/TS glue, integrity metadata and sourcemap/debug-symbol retention policy.

### 6. C ABI, JNI/JNA, Swift/C/C++, Kotlin/Native

- A narrow `extern "C"` ABI is the only broadly consumable native denominator. Rust’s own ABI has no stability guarantee. Export fixed-width scalars, `(ptr,len)` byte slices, opaque handles and explicit alloc/free/retain/release functions; catch panics before the boundary; never let Rust/C++ unwinding cross a non-unwind ABI.
- Version it explicitly: `vida_api_version`, capability bits, sized request/response structs or versioned serialized envelopes. Hide all Rust layouts, enums, `bool`, `String`, `Vec`, trait objects, references and allocator-owned memory.
- Direct JNI is the Android choice for measured high-frequency Java/Kotlin integration. Android’s guidance is to minimize marshalling, async cross-language traffic and the number of threads touching JNI; `JNIEnv` is thread-local and native threads must attach before calling Java.
- UniFFI main’s unreleased changelog now contains an **experimental** JNI Kotlin generator and reports benchmark gains over the current mapping, but labels it very fresh/use-at-own-risk. Track it as a lead; do not base the initial architecture or release promise on it.
- JNA reduces handwritten glue and is used in Mozilla’s UniFFI Kotlin path, but Mozilla records bool-width and callback/threading hazards. It is not relevant to Flutter’s normal Dart FFI path and should not be introduced there.
- Swift imports C directly. Swift C++ interop is available from Swift 5.9 but remains evolving, has type/lifetime/exception constraints, and creates a C++ toolchain/standard-library compatibility surface. Prefer C/UniFFI for the stable public bridge; use C++ only if an existing C++ SDK or measured zero-copy path warrants it.
- Kotlin/Native `cinterop` generates C mappings but is Beta and requires manual/lexical native memory and `StableRef` patterns for callbacks. It does not replace JNI for an Android JVM UI and is not relevant to Flutter unless adopting KMP/native.

## Boundary rules

### Cross the boundary

- Owned immutable records/enums; fixed-width integers; UTF-8 strings; bounded byte buffers.
- Stable domain IDs, pagination cursors, generation numbers, timestamps with explicit unit/time zone.
- Coarse use-case commands (`open_workspace`, `apply_transaction`, `start_sync`) rather than entity getters/setters.
- Typed domain errors: stable machine code + retryability + safe user message key + optional diagnostic correlation ID.
- Snapshots and versioned deltas; batched events with sequence number and resync marker.
- Explicit lifecycle: create/open, close/dispose, subscribe/unsubscribe, operation ID/cancel/await terminal state.
- Capability/config records, with additive optional fields and negotiated versions.

### Do not cross

- UI widgets, contexts, activities, view controllers, Tauri windows/app handles, Dart isolates/ports.
- Rust references/borrowed slices retained after a call; raw pointers without an owner/free contract.
- Rust-specific generic types, trait objects, executors, channels, locks, futures or panic payloads.
- Arbitrary foreign closures or high-frequency per-item callbacks; use bounded streams/batches.
- Database connections/transactions or filesystem handles unless represented by opaque, scoped resources.
- Large object graphs serialized on every frame; use immutable snapshots, deltas, paging or binary buffers.
- Secrets in diagnostics/errors; platform keychain/keystore objects remain behind platform capability adapters.

## Async, threading, cancellation and streams contract

Recommended neutral protocol:

1. `start(request) -> OperationId` returns quickly.
2. `events(operation_id, after_seq, max_items) -> EventBatch` or adapter-native stream emits ordered, bounded events.
3. `cancel(operation_id)` is idempotent and cooperative.
4. One terminal event: `completed`, `failed(ErrorEnvelope)`, or `cancelled`.
5. Dropping a shell future/subscription triggers best-effort `cancel` + `unsubscribe`, but correctness never depends on finalizers.
6. Buffers have declared bounds and overflow behavior (`coalesce`, `drop_oldest + resync_required`, or producer backpressure).
7. No callback into UI code while holding Rust locks. Marshal shell-visible events to the shell’s executor/main isolate.

This protocol maps to FRB Stream, UniFFI callback/foreign trait or polling object, Tauri Channel/invoke, and browser async iterator/worker messages without importing any of them into the core.

## ABI/API compatibility and release independence

- **Source API:** semver the application facade and generated wrapper packages; additive fields/variants still require per-language compatibility tests because exhaustive enums can break consumers.
- **Binary ABI:** do not promise stability for Rust symbols or generator-internal C symbols. Bundle matched library + generated bindings. Only the optional narrow C ABI is externally stable.
- **Artifact lockstep:** embed core git SHA, schema/API version, target triple and bridge-generator version; validate at initialization and fail with a typed mismatch instead of crashing.
- **Independent delivery:** shell can update independently only within an advertised compatibility window. Maintain N and N-1 contract fixtures; publish a compatibility manifest. Schema evolution must tolerate unknown optional fields/events where transports permit it.
- **Coordinated breaks:** Mozilla’s production process prepares consumer PRs before a breaking shared-core change. Adopt the same gate even if artifacts live in one monorepo.

## Packaging and symbolication

- Android: package all required ABIs in AAR/app bundle (`arm64-v8a`, plus chosen simulator/test ABIs); test 16 KiB page compatibility. Publish native symbols with `ndk.debugSymbolLevel=FULL` or `SYMBOL_TABLE`; preserve build IDs and exact unstripped binaries.
- Apple: package device + simulator slices and headers/module map in an XCFramework, optionally distributed as a SwiftPM binary target. Archive matching dSYMs by build UUID for every shipped artifact.
- Flutter: choose and pin FRB’s Cargokit or native-assets build integration; do not mix ad hoc library copying with generated package metadata. Test release/profile builds because dead stripping and symbol lookup differ from debug.
- Tauri Windows: link the core as a Rust workspace dependency; publish PDBs separately to the crash-symbol store and retain the exact executable DLL/PDB build identity.
- Browser: publish `.wasm` + JS/TS glue as one immutable package; retain sourcemaps and Wasm debug/symbol artifacts keyed by content hash.
- All: include Rust panic hook/correlation IDs, but prefer typed errors. Never rely on shipped symbol names as the sole diagnostic path.

## Production patterns

- **Mozilla Application Services:** UniFFI-generated Kotlin/Swift packages, Android Gradle/AAR path, Apple XCFramework path, nightly/release promotion, and coordinated consumer changes. Pattern: core + generated bindings are a tested artifact set, not independent loose files.
- **1Password 8:** a shared Rust Core across macOS/iOS/Windows/Android/Linux/browser/web, deliberately stopping before UI; server communication, DB, permissions and crypto are shared. Pattern: maximize domain consistency, keep presentation native.
- **Matrix Rust SDK / Element X:** Rust owns encryption, sync, room state and UI-oriented SDK data; native SwiftUI/Compose apps consume bindings. Pattern: expose high-level UI-supporting models, not raw protocol primitives. Caveat: API evolution remains active.
- **Signal libsignal:** one Rust implementation exposed as Java, Swift and TypeScript through language-specific bridges/macros; bridge layers are explicitly not stable public APIs, while product-language APIs are versioned best-effort. Pattern: one semantic implementation with specialized transport surfaces.

## Contradictions and resolutions

| Conflict | Evidence | Resolution |
|---|---|---|
| Matrix 2023 says Element contributed “cancellable async bindings” to UniFFI; current UniFFI guide says no built-in cancellation | Matrix 2.0 post vs current UniFFI async guide | Treat current guide as authoritative for generic UniFFI. Assume Matrix had project-specific/generated cancellation behavior; require explicit VIDA cancellation contract. Confidence: high. |
| UniFFI appears in a WASM section, but has no WASM generator | Current UniFFI WASM page | It only configures scaffolding for external generators; first-party browser choice remains wasm-bindgen. Confidence: high. |
| “One Rust API” may suggest one generated surface | Production examples use separate Java/Swift/TS/Dart mechanisms | Share facade semantics and Rust implementation; specialize adapters. Confidence: high. |
| Generated bridges advertise safety, but releases still fix memory/crash/marshalling defects | UniFFI 0.31.1/0.31.2 and FRB release history | Pin versions, run cross-language ABI smoke tests, fuzz malformed buffers, and canary release. Confidence: high. |
| Finalizers make handles convenient, but native memory is invisible/delayed under foreign GC | UniFFI Arc handles; FRB opaque docs; wasm-bindgen weak-ref discussion | Require deterministic `close/dispose`; finalizer is fallback only. Confidence: high. |

## Gaps and validation leads

- Benchmark the actual call shapes: tiny command, 1 MiB bytes, 10k records, 60 Hz deltas, cold initialization. Compare FRB CST/DCO vs SSE and Tauri JSON vs binary response.
- Prototype cancellation propagation under Flutter hot restart, webview reload, app backgrounding and Tauri window close.
- Verify FRB 2.13 stable vs 2.14 beta against the selected Flutter/Dart/AGP/Xcode matrix before committing.
- Test browser dependencies under `wasm32-unknown-unknown`; inventory crates requiring threads, sockets, filesystem or OS entropy.
- Decide whether native Swift/Kotlin remains a credible fallback. If not, UniFFI can remain a contingency adapter rather than a first-release deliverable.
- Define resource leak tests: repeated open/close, abandoned futures, dropped streams, foreign callback exceptions, double-close and process shutdown.
- Verify symbol upload and end-to-end crash symbolication for Rust frames on Android, iOS, Windows and Wasm before release.
- Security review Tauri capability scopes and command argument validation; the core API is not an authorization boundary by itself.

## Evidence digest — 12 source groups

All accessed 2026-09-19.

1. **UniFFI current guide** — Mozilla, pub_date: living docs / undated. URLs: https://mozilla.github.io/uniffi-rs/latest/ ; https://mozilla.github.io/uniffi-rs/next/futures.html ; https://mozilla.github.io/uniffi-rs/next/internals/object_references.html ; https://mozilla.github.io/uniffi-rs/next/types/errors.html ; https://mozilla.github.io/uniffi-rs/latest/wasm/configuration.html . Claims: supported languages, generated type/error model, Arc ownership, `Send+Sync`, async mapping, no built-in cancellation, external/unstable WASM path. Confidence: high. Class: primary official documentation.
2. **UniFFI changelog** — Mozilla/uniffi-rs, pub_date: 2026-09-08 (0.32.1). URLs: https://github.com/mozilla/uniffi-rs/blob/main/CHANGELOG.md ; https://docs.rs/crate/uniffi_bindgen/0.32.1 . Claims: 0.32.1 current release (repository + registry verification); 0.32 zero-copy synchronous borrowed bytes and binding/config breaks; recent Kotlin/Swift async, memory, crash and marshalling fixes; experimental JNI Kotlin generator is unreleased and explicitly fresh. Confidence: high. Class: primary repository release record and registry artifact.
3. **Application Services build/release architecture** — Mozilla, pub_date: living docs / undated. URLs: https://mozilla.github.io/application-services/book/build-and-publish-pipeline.html ; https://mozilla.github.io/application-services/book/howtos/releases.html ; https://mozilla.github.io/application-services/book/android-faqs.html . Claims: Cargo + Gradle/AAR + XCFramework packaging, generated Swift/Kotlin artifacts, JNA/JNI tradeoffs, coordinated releases. Confidence: high. Class: primary production architecture documentation.
4. **JNI tips** — Android Developers / Google, pub_date: living docs / undated. URL: https://developer.android.com/ndk/guides/jni-tips . Claims: minimize marshalling, async crossings and threads; `JNIEnv` is thread-local; attach native threads; generator recommendation. Confidence: high. Class: primary platform documentation.
5. **Dart native interop** — Dart project / Google, pub_date: living docs / undated. URLs: https://dart.dev/interop/c-interop ; https://dart.dev/tools/hooks ; https://api.dart.dev/dart-ffi/NativeCallable-class.html . Claims: Dart FFI is C interop, ffigen/header generation, code-asset bundling, callback thread modes and explicit close. Confidence: high. Class: primary language/API documentation.
6. **flutter_rust_bridge** — FRB project, pub_date: 2026-09-12 for latest prerelease; living docs otherwise. URLs: https://github.com/fzyzcjy/flutter_rust_bridge/releases ; https://pub.dev/packages/flutter_rust_bridge/versions ; https://cjycode.com/flutter_rust_bridge/guides/miscellaneous/codec ; https://cjycode.com/flutter_rust_bridge/guides/types/translatable/stream ; https://cjycode.com/flutter_rust_bridge/guides/how-to/cancel . Claims: 2.13.0 stable and 2.14.0-beta.2 prerelease status (repository + package-registry verification), codecs, async/stream support, application-level cancellation. Confidence: medium-high (official project; some docs describe recommended/copied patterns rather than a stable built-in). Class: primary project docs/releases and registry artifact.
7. **Tauri v2 IPC and plugins** — Tauri project, pub_date: living docs / undated. URLs: https://v2.tauri.app/develop/calling-rust/ ; https://v2.tauri.app/develop/plugins/ ; https://v2.tauri.app/reference/javascript/api/namespacecore/ . Claims: serde/JSON command boundary, binary Response, async execution, managed state, channels/resources, plugin permissions/scopes. Confidence: high. Class: primary official documentation.
8. **wasm-bindgen guide** — Rust and WebAssembly working group/project, pub_date: living docs / undated. URLs: https://rustwasm.github.io/docs/wasm-bindgen/ ; https://rustwasm.github.io/docs/wasm-bindgen/reference/rust-targets.html ; https://rustwasm.github.io/docs/wasm-bindgen/print.html . Claims: JS/TS bindings, `wasm32-unknown-unknown` limitations, closure/object lifetimes, browser thread/worker caveats. Confidence: high. Class: primary project documentation.
9. **Swift/C++ and Apple binary packaging** — Swift.org + Apple, pub_date: living docs / undated. URLs: https://www.swift.org/documentation/cxx-interop/status/ ; https://developer.apple.com/documentation/xcode/creating-a-multi-platform-binary-framework-bundle ; https://developer.apple.com/documentation/xcode/building-your-app-to-include-debugging-information . Claims: Swift C++ availability/constraints, XCFramework packaging, build-UUID/dSYM retention. Confidence: high. Class: primary language/vendor documentation.
10. **Kotlin/Native C interop** — JetBrains, pub_date: living docs / undated. URL: https://kotlinlang.org/docs/native-c-interop.html . Claims: cinterop remains Beta; native allocation and StableRef callback ownership. Confidence: high. Class: primary language documentation.
11. **Matrix Rust SDK / Element X** — Matrix.org + matrix-org, pub_date: 2023-09-21 for Matrix 2.0 post; repository current. URLs: https://matrix.org/blog/2023/09/matrix-2-0/ ; https://github.com/matrix-org/matrix-rust-sdk . Claims: production Rust SDK under Element X, high-level UI support, UniFFI async history, planned Web/desktop expansion. Confidence: medium-high; cancellation wording conflicts with current UniFFI docs. Class: primary project repository and first-party architecture post.
12. **Signal and 1Password production patterns** — Signal + 1Password, pub_date: repositories current; 1Password post 2021-08-12. URLs: https://github.com/signalapp/libsignal ; https://github.com/signalapp/libsignal/blob/main/rust/bridge/README.md ; https://1password.com/blog/1password-8-the-story-so-far . Claims: language-specific Java/Swift/TS bridges over Rust; bridge API instability; shared Rust core stopping short of UI across native and Web targets. Confidence: high. Class: primary production repositories and first-party architecture post.

## Final recommendation

Adopt a **ports-and-adapters shared-Rust architecture**. First implementation path for the hypothesis:

1. `vida-core` + `vida-runtime`: toolkit-neutral Rust; no UniFFI/FRB/Tauri/wasm types.
2. `vida-app-api`: owned DTOs, typed errors, operation registry, cancellation, bounded events.
3. `vida-api-flutter`: FRB; async Dart API + Streams; explicit dispose.
4. `vida-api-tauri`: direct Rust calls behind commands/state/channels; serde and ACL stay here.
5. `vida-api-web`: wasm-bindgen worker-oriented adapter.
6. Optional `vida-api-uniffi`: native Swift/Kotlin contingency.
7. Optional `vida-c-api`: tiny versioned C ABI only when a real external consumer demands it.

Gate the decision with two spikes: (a) end-to-end operation/cancel/stream/resource lifecycle in Flutter and Tauri; (b) browser compile/worker viability. This preserves UI-toolkit optionality without paying for four production bridge stacks up front.
