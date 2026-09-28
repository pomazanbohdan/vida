# Flutter mobile + shared Rust core + Tauri Windows: technical selection digest

Accessed: 2026-09-19

## Verdict

**Conditional GO.** The combination is coherent for a local-first encrypted application **only as a shared-Rust-core architecture, not as a shared-UI architecture**. Flutter mobile and Tauri Windows should be treated as two independent presentation shells over one Rust application runtime. The Rust core must not depend on Flutter, `flutter_rust_bridge` (FRB), Tauri, JavaScript, or OS UI types; thin target facades own every bridge and platform integration.

Default decision rule:

- Choose **Flutter mobile + Tauri Windows** when Windows needs an HTML/DOM ecosystem, an existing TypeScript design system/team, web-style automation, Tauri's tray/updater/plugin surface, or substantially different desktop information architecture.
- Choose **Flutter mobile + Flutter Windows** when cross-device UI parity, one presentation codebase, one state-management stack, lower staffing/QA cost, or delivery speed matters more. This is the lower-complexity default.
- A browser product is **not** obtained “for free” from either native shell. A Tauri frontend can be reused in a browser only behind a transport abstraction; the native Rust core then needs a separately proven WASM build or a server/local-agent API.
- An always-on Windows component that must run before login is **not** a tray app. Split it into a real service/daemon with authenticated local IPC; keep Tauri as the user-session UI. Official Tauri material establishes tray/autostart/sidecar facilities, not a Windows Service lifecycle.

Confidence: **high** on structural coherence and duplication costs; **medium** on production readiness until the prototype proves lifecycle, packaging, accessibility, and secret-handling on target hardware.

## Current version/compatibility snapshot

| Component | Verified current signal | Compatibility implication |
|---|---|---|
| Flutter | Windows stable manifest reports **Flutter 3.47.5 / Dart 3.13.4**, released 2026-09-18. | Current Dart exceeds FRB 2.13's Dart 3.9 floor. Pin exact Flutter/Dart in CI. |
| flutter_rust_bridge | pub.dev reports **2.13.0 stable**, **2.14.0-beta.2 prerelease**, Android/iOS/Windows/web labels, async Rust, streams, and zero-copy large arrays. | Use 2.13.x stable for the spike; do not base release architecture on 2.14 beta. “web” support still means a separate WASM path, not native FFI. |
| Tauri | Official release index reports **tauri 2.11.5** (2026-07-01), CLI 2.11.4, JS API 2.11.1. | Pin the coordinated 2.11 line and lock plugin versions; do not assume all ecosystem crates share one patch number. |
| Rust floor | Official Tauri plugin pages list Rust **1.77.2+** for updater/autostart/notification; FRB-generated/native code can impose a newer floor through deps. | Pick one workspace MSRV after `cargo msrv`/CI validation; the highest target dependency wins. |

## Required architecture boundary

```text
Flutter/Dart UI ── mobile_app_api (FRB-generated facade) ─┐
                                                         ├─ app_runtime
Tauri/TS UI ───── desktop_app_api (commands/channels) ───┤  domain + use cases
                                                         │  sync + encrypted repository
Browser UI ────── web_app_api (HTTP/WS or WASM) ─────────┘  platform ports
                                                                  │
                    Android/iOS/Windows adapters ──────────────────┘
                    keystore, notifications, background jobs,
                    filesystem, network reachability, clock
```

Workspace rule:

- `core-domain`: pure domain types/invariants; no async runtime, persistence, bridge, UI, or OS deps.
- `app-runtime`: use cases, single-writer state machine, sync orchestration, encrypted repository traits, durable job journal; owns `Arc`-backed runtime state.
- `platform-ports`: narrow Rust traits for secure key wrapping, clock, randomness, network, notification intent, and lifecycle.
- `mobile-app-api`: FRB annotations/generated code and DTO mapping only.
- `desktop-app-api`: Tauri `#[command]`, `State`, `Channel`, capability manifest, and DTO mapping only.
- Optional `service-host`: separate Windows Service/daemon host; never make the WebView process the privileged service.
- Optional `web-app-api`: either WASM-safe facade or client/server transport; no silent fallback to Tauri APIs.

The same `app-runtime` crate and migration code must be linked into both native products from the same Git commit. Platform cfgs belong in adapters, not scattered through domain/use-case code.

## Binding contract

Use a deliberately small, versioned application protocol even though both integrations can expose rich native types.

1. **Three surfaces only:** commands, queries, events. Commands are explicit mutations; queries are side-effect free; events are observations, never required for correctness.
2. **Stable envelopes:** `schema_version`, `request_id`, `operation_id`, `actor/device_id`, `expected_revision`, and structured `error { code, retryable, details_version }`. UI text is localized in the shell, not returned by Rust.
3. **Idempotency:** every durable mutation accepts an idempotency key. Retries after mobile suspension, updater restart, or IPC loss must not duplicate work.
4. **Revisioned streams:** each event carries monotonic `sequence` and aggregate/database `revision`. A detected gap triggers `get_snapshot(since_revision)`; no UI assumes streams are lossless.
5. **Explicit cancellation:** `start_* -> operation_id`; `cancel(operation_id)` drives a Rust `CancellationToken`. Dropping a Dart `StreamSubscription`, navigating away, or discarding a JS Promise is not the cancellation contract. FRB's current cancellation guide explicitly describes user-supplied/Tokio tokens rather than automatic bridge cancellation.
6. **Backpressure:** coalesce progress/presence events, bound every queue, and emit an overflow/resync marker. Do not stream database rows one object per IPC message.
7. **Bulk data:** keep routine DTOs small; use bounded byte buffers, a Rust-owned opaque handle, or an authenticated temporary file for large attachments. Tauri documents optimized binary responses because JSON serialization is slow for large data.
8. **Lifecycle:** `open_profile`, `lock`, `prepare_suspend(deadline)`, `resume`, `flush`, and `shutdown` are first-class operations. The durable journal, not a live Dart isolate/WebView listener, is the source of truth.
9. **Ownership:** Rust owns DB connections, encryption contexts, sync sessions, and migration locks. Dart/JS receives immutable DTO snapshots and opaque IDs, never DB handles or long-lived key material.
10. **Generated-code discipline:** generated FRB code is reproducible and checked for drift in CI. Tauri TypeScript types should be generated from the same facade schema or validated by contract tests; do not hand-maintain parallel DTO definitions.

## Async, streams, and lifecycle findings

- FRB 2.13 maps Rust `async fn` to Dart futures and `StreamSink`/iterators to Dart streams. Synchronous Dart bindings exist but can block the UI; reserve them for tiny deterministic getters.
- FRB stream closure/subscription semantics and cancellation are not a durable task model. Keep tasks in `app-runtime`; bridges subscribe by operation ID and can reconnect.
- Tauri async commands run as async tasks; `Channel` is the documented streaming mechanism. This maps cleanly to the same operation/event protocol, but it is a second adapter, not reusable FRB glue.
- Flutter background work uses isolates/callback dispatch and WorkManager-style scheduling. Mobile OSes may suspend/kill the process; therefore the core must checkpoint quickly and resume idempotently. Continuous socket/sync operation while backgrounded is not a safe baseline requirement.
- A hidden Tauri window/tray process can remain active in a logged-in user session. Autostart is a login convenience, not a boot service, privileged daemon, or availability guarantee. If the product requires pre-login processing, machine-wide policy, or survival independent of the user's session, adopt `service-host` and authenticated named-pipe/local IPC.

## Security boundary for a local-first encrypted product

1. **Assume the WebView can be compromised.** Tauri's own capability docs say capabilities reduce frontend-compromise impact but do not protect against insecure Rust, lax scopes, incorrect command checks, WebView zero-days, or supply-chain compromise.
2. **Keep plaintext/key operations in Rust.** JS/Dart should pass intent and opaque record IDs. Return display data only when unlocked; zeroize Rust key buffers; avoid secret-bearing exceptions, logs, analytics, clipboard, DOM attributes, Dart objects, or Tauri events.
3. **Use an envelope-key design.** Generate the data-encryption key in Rust; wrap it with Android Keystore/Apple Keychain/Windows DPAPI or an approved hardware-backed credential. Define recovery/export separately. Platform key stores are adapters, not the database encryption primitive.
4. **One writer per vault.** The runtime owns transaction/migration/sync serialization. If a Windows service is introduced, the service becomes the sole vault owner and Tauri becomes a client; never allow service and UI to open the encrypted DB independently.
5. **Tauri least privilege:** package only local frontend assets; strict CSP; no remote scripts; one labeled main WebView; explicit command manifest; per-window/plugin capabilities; narrow filesystem scopes; no generic shell command; validate every path, URL, size, enum, authorization state, and optimistic revision in Rust.
6. **Bridge memory is part of the threat model.** “Zero-copy” is a performance property, not proof of secret erasure. Prototype heap/crash/log behavior and minimize plaintext crossings.
7. **Signed release chain:** code-sign Windows installers and binaries; keep updater private keys offline/CI-protected. Tauri updater signatures are mandatory, but updater signing is separate from Windows Authenticode/SmartScreen reputation.

## Windows/Tauri product surface

- **WebView2:** Tauri uses the system Evergreen WebView2 by default. Windows 11 includes it; Tauri installers can download/bootstrap, embed an offline installer (~127 MB), or pin a fixed runtime (~180 MB). A local-first/offline promise requires an offline installer or a documented prerequisite, not the default download bootstrapper.
- **Commands/state/plugins:** async commands, managed state, binary responses, and channels are adequate for the desktop facade. Registered custom commands still need their own authorization/validation; plugin capabilities alone are not a domain authorization layer.
- **Updater:** MSI/NSIS artifacts and mandatory updater signatures are supported. Windows exits the app during install. The core must flush/lock before install; migrations need forward compatibility and a tested rollback policy.
- **Notifications:** the official plugin supports Windows, but notifications work only for installed apps. Portable zip builds cannot be treated as equivalent.
- **Tray/background:** tray and autostart are official surfaces. They support a user-session resident app, not a Windows Service. Test close-to-tray, fast-user-switching, logout, update, and multiple-instance behavior.
- **Accessibility:** Tauri inherits DOM/ARIA and WebView2 behavior, not automatic accessibility. This research found no Tauri-specific conformance guarantee. Require semantic HTML, keyboard-complete flows, 200%/400% zoom, high contrast, reduced motion, and live NVDA + Narrator testing. WebView2 evergreen updates can change behavior, so include preview-channel compatibility smoke tests.

## Browser implications

There are three defensible choices:

| Browser option | Core reuse | Security/offline consequence | Decision |
|---|---:|---|---|
| No browser product | None required | Simplest; native encrypted vault only. | Preferred until a browser requirement exists. |
| Tauri web UI reused as browser + remote API | UI/component reuse; Rust runs server-side | Browser holds session/display plaintext; offline vault requires IndexedDB/WebCrypto design; transport auth/sync conflicts are new. | Viable for connected companion UI. |
| Rust core compiled to WASM | Potential algorithm/domain reuse | `dart:ffi` is unavailable on web; filesystem, threads, SQLite, OS keystore, background execution, and secure key persistence need replacements. | Prototype-only until every core dependency passes `wasm32-unknown-unknown`. |

Never import `@tauri-apps/api` directly in components. Define a TypeScript `AppClient` port with `TauriClient`, `HttpClient`, and optional `WasmClient` implementations. Likewise, Flutter web would require conditional web bindings; native FRB/FFI code cannot simply run in the browser.

## Counterfactual: Flutter Desktop for Windows

| Dimension | Flutter Windows | Tauri Windows |
|---|---|---|
| Presentation reuse | Reuses widgets, navigation, state, localization, design tokens, and much mobile UI. | Reuses none of the Flutter presentation layer; requires a full TS/web UI and duplicate view models/tests. |
| Rust boundary | Same FRB/FFI model as mobile; one binding style across native targets. | Rust is native host; Tauri commands/channels are simpler on desktop but constitute a second contract implementation. |
| Desktop fit | Custom-rendered Flutter controls; Fluent packages/native host customization available. | DOM/CSS, mature editors/data grids, browser devtools, ARIA, and web automation can be strong for data-dense productivity UI. |
| Runtime/distribution | Bundles Flutter engine/assets; Windows packaging is available but updater/tray solutions are less unified. | Small app shell when using system WebView2; official updater/tray/autostart/notification surface; WebView2 deployment policy becomes part of release engineering. |
| Security | No HTML/XSS/remote-script surface, but FFI and plugin boundaries remain. | Extra WebView/IPC/XSS/supply-chain boundary; Tauri capabilities/CSP reduce, not remove, risk. |
| Browser leverage | Flutter web can reuse Dart widgets but still cannot use native FFI. | Web components can be reused naturally if all Tauri calls are behind `AppClient`; Rust still needs server/WASM adaptation. |
| Team cost | One UI language/toolchain. | Dart/Flutter + TS/web + two UI test stacks + Cargo. |

**Use Tauri instead of one Flutter UI only if at least one benefit is material and measured:** existing production web UI/design system; desktop-first UX diverges enough that widget reuse is low; critical DOM ecosystem components; browser UI reuse is planned; or official Tauri desktop lifecycle/update capabilities save more effort than the second UI stack costs. Otherwise Flutter Desktop is the rational baseline.

## Incompatibilities and coupling risks

- FRB's generated facade cannot be reused as a Tauri command layer. Share Rust use cases, not bridge code.
- Dart models/state stores and TypeScript models/state stores will drift without one versioned schema and contract tests.
- Flutter mobile release cadence (Gradle/Xcode/store review) and Tauri desktop updater cadence differ. Database/schema changes must support an explicit compatibility window; never require synchronized client updates.
- Native Rust deps must build for Android ABIs, iOS devices/simulators, and Windows MSVC. Crates that assume desktop filesystem/process/thread behavior can break mobile or WASM.
- Native Assets support exists in FRB, while Cargokit remains a supported/default path depending on integration choice. Changing the backend affects packaging/CI even when the Dart/Rust API stays constant; pin one after a reproducible signed build.
- Tauri's system WebView reduces bundled runtime size but introduces evergreen-runtime drift. Fixed/offline runtime modes increase installer size and patching responsibility.
- A separate service creates a third host, IPC authentication, installer privileges, service update sequencing, and multi-user vault ownership. Do not add it unless requirements demand it.
- Two UIs duplicate routing, adaptive layout, validation display, localization plumbing, accessibility work, analytics instrumentation, screenshots, component QA, and support documentation. Only domain rules belong in Rust; presentation duplication remains real.

## Prototype exit criteria

Proceed with the split architecture only when all gates pass on release builds and representative low-end devices:

1. **One-core proof:** the same `app-runtime` commit passes Rust contract tests and is linked into Android, iOS, and Windows Tauri artifacts; no domain fork and no bridge types in core crates.
2. **Contract parity:** a golden suite runs the same command/query/event scenarios through direct Rust, FRB, and Tauri adapters; results/error codes/revisions match byte-for-byte after canonicalization.
3. **Stream correctness:** 100,000 sequenced events with forced slow consumers produce no silent loss/reordering; overflow yields a resync marker; snapshot recovery converges.
4. **Cancellation:** canceling CPU and I/O jobs from both shells stops observable work within a team-defined budget, releases resources, and never commits partial durable state unless the operation contract permits it.
5. **Crash/lifecycle:** kill at every transaction/sync/migration phase and run 100 restart cycles; vault integrity, idempotency, and resumability hold. Android/iOS background/foreground and OS-kill tests are included.
6. **Secret audit:** no master key/plaintext appears in JS/Dart persistence, DOM, logs, analytics, panic/crash reports, temp filenames, or updater metadata. Lock clears UI data and invalidates opaque handles.
7. **Windows lifecycle:** installed build survives 24 h close-to-tray, sleep/resume, network changes, second launch, logout/login, and signed update. If work must occur before login, a separate service POC must pass authenticated IPC and least-privilege review.
8. **Offline install/update:** chosen MSI/NSIS mode works on a clean supported Windows image without assuming network for WebView2; update signature failure is safely rejected; app flushes before installer exit; migration rollback/recovery is proven.
9. **Accessibility:** core journeys pass keyboard-only, NVDA, Narrator, high contrast, 200%/400% scaling, and automated DOM checks on Tauri; TalkBack/VoiceOver and text scaling pass on Flutter mobile. No release on “framework support” alone.
10. **Performance budgets:** predeclare and meet p95 cold start, unlock, first-query, scroll/update latency, memory-at-idle, battery/background budget, bridge throughput, and installer-size targets. Compare the same workloads against a Flutter Windows spike.
11. **Release reproducibility:** CI emits signed Android AAB, iOS archive, Windows MSI/NSIS + updater signatures from one core lockfile/SBOM; generated FRB/TS bindings are clean and reproducible.
12. **Counterfactual win:** the Tauri spike must demonstrate a material desktop benefit that a Flutter Windows spike does not—component capability, UX quality, browser reuse, operational integration, or measured team throughput. If the gain is marginal, choose Flutter Windows.

## Evidence ledger (12 primary source records)

### E1 — Current Flutter/Dart release

- claim: The official Windows release manifest identified Flutter 3.47.5 stable with Dart 3.13.4 on 2026-09-18.
- URL: https://storage.googleapis.com/flutter_infra_release/releases/releases_windows.json
- publisher: Flutter/Google release infrastructure
- pub_date: 2026-09-18
- accessed: 2026-09-19
- confidence: high
- class: primary release manifest
- contrary evidence: package/tool compatibility can lag the SDK even when version floors match.
- gaps: full FRB 2.13 matrix was not executed in this research pass.
- leads: pin 3.47.5 and run FRB Android/iOS/Windows CI from clean agents.

### E2 — FRB current version and feature surface

- claim: pub.dev reported FRB 2.13.0 stable and 2.14.0-beta.2 prerelease; its maintained package page documents Android/iOS/Windows/web labels, async Rust, streams, errors, large-array zero-copy, and Rust-to-Dart calls.
- URL: https://pub.dev/packages/flutter_rust_bridge
- publisher: Dart/Flutter pub.dev; package publisher cjycode.com
- pub_date: 2026-08 (2.13.0; registry showed 26 days before access)
- accessed: 2026-09-19
- confidence: high for registry/version; medium-high for feature claims until workload tests
- class: primary package registry + maintainer documentation
- contrary evidence: generator convenience does not remove lifecycle, cancellation, and schema design; “web” is not native FFI.
- gaps: no independent production benchmark for this product's payloads.
- leads: spike stable 2.13.0; keep 2.14 beta out of release baseline.

### E3 — FRB packaging backends

- claim: FRB supports Cargokit and Native Assets/build-hook integration; the latter changes build/bundling while the application API remains the same and requires a supporting Flutter/Dart SDK.
- URL: https://cjycode.com/flutter_rust_bridge/manual/integrate/builtin
- publisher: flutter_rust_bridge maintainers
- pub_date: undated live docs
- accessed: 2026-09-19
- confidence: high
- class: primary maintainer documentation
- contrary evidence: backend availability is not proof of reproducible signed builds for all ABIs.
- gaps: no clean-agent iOS simulator/device, Android ABI, or Windows MSVC build was run.
- leads: compare Cargokit vs Native Assets on build time, artifact contents, symbols, and CI maintenance; then pin one.

### E4 — FRB cancellation is application-defined

- claim: FRB's cancellation guide recommends an explicit cancel token, including Tokio's `CancellationToken`; cancellation is not described as automatic on Dart future/stream abandonment.
- URL: https://cjycode.com/flutter_rust_bridge/guides/how-to/cancel
- publisher: flutter_rust_bridge maintainers
- pub_date: undated live docs
- accessed: 2026-09-19
- confidence: high
- class: primary maintainer documentation
- contrary evidence: small short-lived calls may not need cancellation.
- gaps: subscription-drop behavior must be verified against 2.13 generated code.
- leads: standardize `operation_id` + explicit cancel and test races at commit boundaries.

### E5 — Flutter FFI and browser boundary

- claim: Flutter documents direct Dart FFI for C ABIs including Rust on native targets, while explicitly stating FFI is unavailable on web and JS interop/package:web is used instead.
- URL: https://docs.flutter.dev/resources/architectural-overview
- publisher: Flutter/Google
- pub_date: undated live docs
- accessed: 2026-09-19
- confidence: high
- class: primary framework documentation
- contrary evidence: FRB can generate a WASM bridge, but that is a separate compilation/runtime path with browser-specific adapters.
- gaps: core crate WASM dependency audit not performed.
- leads: `cargo check --target wasm32-unknown-unknown` plus feature/dependency inventory before promising browser parity.

### E6 — Flutter background execution

- claim: Flutter background execution uses isolates/callback dispatch; WorkManager supports persistent scheduled work through app restarts/reboots. This is scheduled platform work, not proof of an always-running Rust runtime.
- URL: https://docs.flutter.dev/packages-and-plugins/background-processes
- publisher: Flutter/Google
- pub_date: undated live docs
- accessed: 2026-09-19
- confidence: high on mechanism; medium on product-specific feasibility
- class: primary framework documentation
- contrary evidence: supported OS background modes can enable bounded special-purpose work.
- gaps: iOS entitlements/modes and Android OEM battery policies depend on exact use cases.
- leads: prototype each required background job with OS-native scheduling and hard deadlines; reject continuous-sync assumptions.

### E7 — Current Tauri line

- claim: Tauri's official ecosystem index listed tauri 2.11.5, CLI 2.11.4, JS API 2.11.1, and Wry 0.56.0 as the current 2026 release line.
- URL: https://v2.tauri.app/release/
- publisher: Tauri Programme
- pub_date: 2026-07-01 for tauri 2.11.5
- accessed: 2026-09-19
- confidence: high
- class: primary release index
- contrary evidence: plugin versions and MSRVs evolve independently.
- gaps: selected frontend/framework plugin compatibility not yet resolved.
- leads: generate a locked dependency graph/SBOM and test upgrades as a coordinated release train.

### E8 — Tauri commands, async, binary responses, channels

- claim: Tauri documents typed Rust commands, async commands, managed state access, optimized binary responses, and `Channel` as the recommended streaming mechanism.
- URL: https://v2.tauri.app/develop/calling-rust/
- publisher: Tauri Programme
- pub_date: undated live docs
- accessed: 2026-09-19
- confidence: high
- class: primary framework documentation
- contrary evidence: Tauri command typing/serialization does not create a cross-shell versioned domain protocol automatically.
- gaps: throughput/backpressure for this workload is unmeasured.
- leads: benchmark canonical JSON DTOs, binary transfer, channel fan-out, and resync.

### E9 — Tauri security capabilities

- claim: Tauri capabilities constrain plugin/core access by window/WebView, but the docs explicitly exclude protection from malicious Rust, lax scopes, bad command validation, WebView vulnerabilities, and supply-chain compromise; custom registered commands require separate manifest treatment.
- URL: https://v2.tauri.app/security/capabilities/
- publisher: Tauri Programme
- pub_date: undated live docs
- accessed: 2026-09-19
- confidence: high
- class: primary security documentation
- contrary evidence: a locked local-only frontend with strict permissions materially reduces attack surface.
- gaps: no threat model or CSP/capability diff exists for the proposed app.
- leads: threat-model WebView compromise; deny remote content and shell; generate least-privilege capabilities per window.

### E10 — Windows WebView2 and packaging

- claim: Tauri Windows ships MSI/NSIS and uses WebView2; installer modes range from download/bootstrap to offline (~127 MB) and fixed runtime (~180 MB). System Evergreen WebView2 reduces bundle size but makes runtime updates external to the app.
- URL: https://v2.tauri.app/distribute/windows-installer/
- publisher: Tauri Programme
- pub_date: undated live docs
- accessed: 2026-09-19
- confidence: high
- class: primary distribution documentation
- contrary evidence: Windows 10/11 commonly supply WebView2, so offline bundling may be unnecessary for connected consumers.
- gaps: supported Windows floor and offline-enterprise requirements are undecided.
- leads: install on clean supported VMs with network denied; test Evergreen and preview runtimes.

### E11 — Tauri updater semantics

- claim: Tauri updater supports signed MSI/NSIS artifacts; signature verification cannot be disabled; Windows exits the app during installation; update signing keys are distinct release assets that must be protected.
- URL: https://v2.tauri.app/plugin/updater/
- publisher: Tauri Programme
- pub_date: undated live docs
- accessed: 2026-09-19
- confidence: high
- class: primary plugin documentation
- contrary evidence: mandatory updater signatures do not replace Authenticode, OS trust, database migration safety, or rollback design.
- gaps: update server, key custody, staged rollout, and rollback policy undefined.
- leads: prototype failed signature, interrupted download/install, pre-exit flush, and N-1 database compatibility.

### E12 — Tauri official desktop feature surface

- claim: The official plugin table lists Windows support for autostart, notifications, single-instance, updater, window state, and system tray; plugin pages use capability permissions and generally require Rust 1.77.2+.
- URL: https://v2.tauri.app/plugin/
- publisher: Tauri Programme
- pub_date: undated live docs
- accessed: 2026-09-19
- confidence: high for availability; medium for product-quality edge cases
- class: primary plugin catalog
- contrary evidence: feature presence does not imply Windows Service semantics, accessibility conformance, or zero operational bugs.
- gaps: notification activation, tray z-order, multiple users, logout, and accessibility remain live-test items.
- leads: build one installed tray app and run the complete lifecycle/accessibility matrix before selection.

## Contrary evidence and unresolved gaps

- A single Flutter UI is already a credible Flutter+Rust production pattern; the split cannot claim validation merely because Flutter+Rust apps exist. The decisive evidence must be the split prototype and team economics.
- FRB's maintained web label is encouraging but does not prove that a local encrypted database, OS keystore, background scheduler, or every Rust dependency works in a browser.
- Tauri has the primitives required for a resident tray application, yet official plugin coverage is not evidence of a Windows Service. Service need = separate architecture decision.
- No official Tauri-specific accessibility conformance statement was found in the selected primary source set. Web semantics can be excellent, but only app-level NVDA/Narrator evidence closes the gap.
- No source proves that “smaller Tauri bundle” means lower total installed footprint in offline mode; fixed/offline WebView2 can erase that advantage.
- No measured comparison yet exists for team velocity, memory, cold start, editor/data-grid capability, or accessibility between Tauri and Flutter Windows.

## Research leads

1. Two-week paired spike: one representative Windows workflow in Tauri and Flutter Windows, same Rust runtime and golden contract suite.
2. FRB 2.13 clean builds on Android arm64/x64, iOS device+simulator, and Windows; compare Native Assets and Cargokit once.
3. Threat model: vault unlock, WebView compromise, bridge DTOs, logs/crash dumps, updater key custody, and optional service IPC.
4. Rust portability audit: target triples, native dependencies, SQLite/encryption library, WASM blockers, MSRV, binary size, licenses/SBOM.
5. Accessibility lab: NVDA, Narrator, keyboard, high contrast, scaling, TalkBack, VoiceOver; retain recordings and defects as selection evidence.
6. Release rehearsal: signed MSI/NSIS, offline WebView2, updater, forced process exit, schema migration, downgrade/rollback, and portable-build decision.
