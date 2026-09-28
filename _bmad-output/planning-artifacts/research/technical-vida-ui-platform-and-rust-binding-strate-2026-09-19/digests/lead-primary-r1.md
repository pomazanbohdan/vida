# Lead primary-source digest — round 1

Accessed: 2026-09-19

## Findings

1. **Flutter is a credible single UI toolkit across the candidate platform set.** The current supported-platform matrix covers Android, iOS, Windows, macOS, Linux, and modern web browsers; the current page identifies Flutter 3.47.2. This proves support breadth, not equal feature parity or equal platform fit.  
   Source: https://docs.flutter.dev/reference/supported-platforms — Flutter — current page — high — class: version/compatibility.

2. **Flutter mobile and desktop can call a Rust library through a C ABI; Flutter web cannot use the same native FFI plugin path.** Flutter's official interop guide states that `dart:ffi` binds C symbols on mobile/desktop and that this feature is not supported for web plugins. A Flutter-all-platform UI therefore still needs a separate WASM/JS contract for browser use.  
   Source: https://docs.flutter.dev/platform-integration/android/c-interop — Flutter — current docs — high — class: integration/compatibility.

3. **Flutter does not eliminate platform-specific integration.** Flutter documents native plugins and platform-specific code as the fallback when its core packages or community plugins do not cover an OS capability.  
   Source: https://docs.flutter.dev/platform-integration — Flutter — current docs — high — class: architecture.

4. **Background execution needs an explicit lifecycle contract, not just a Flutter isolate.** Flutter isolates have independent memory and communicate by message passing. On web there are no isolates and `compute()` runs on the main thread; background isolates also cannot receive unsolicited host-platform messages through platform channels. Persistent mobile work uses OS schedulers/plugins.  
   Sources: https://docs.flutter.dev/perf/isolates and https://docs.flutter.dev/packages-and-plugins/background-processes — Flutter — current docs — high — class: platform/runtime.

5. **Accessibility is supported but must be acceptance-tested per target.** Flutter exposes a Semantics tree to assistive technologies. On web it builds an accessible DOM layer; current docs say accessibility can require explicit enablement and custom widgets need explicit roles.  
   Sources: https://docs.flutter.dev/ui/accessibility and https://docs.flutter.dev/ui/accessibility/web-accessibility — Flutter — current docs — high — class: platform/accessibility.

6. **Flutter/Dart SDK versions are part of the compatibility tuple.** Flutter's FAQ states that Flutter and Dart do not provide ABI compatibility across compiler releases, so the app and generated/native artifacts must be built with matching SDK versions.  
   Source: https://docs.flutter.dev/resources/faq — Flutter — current docs — high — class: version/compatibility.

7. **`flutter_rust_bridge` is active and broad, but it is a generated bridge rather than the normative VIDA contract.** Stable 2.13.0 was published in August 2026 and lists Android, iOS, Linux, macOS, web, and Windows. Its docs support sync/async calls, streams, errors, two-way calls, and multiple integration backends.  
   Sources: https://pub.dev/packages/flutter_rust_bridge/versions and https://cjycode.com/flutter_rust_bridge/ — cjycode/pub.dev — 2026 — medium-high — class: version/compatibility.

8. **Cancellation is not safely inferred from generated async bindings.** The current bridge cancellation guide describes application-owned cancellation tokens and notes that its simple implementation is copied from an unmerged PR, or developers can use Tokio cancellation. VIDA must define cancellation semantics in `vida-contracts` and test them independently of bridge codegen.  
   Source: https://cjycode.com/flutter_rust_bridge/guides/how-to/cancel — flutter_rust_bridge — current docs — medium-high — class: integration risk.

9. **Tauri is a coherent Windows shell over a Rust runtime, but it is a webview UI architecture.** Tauri uses HTML rendered in the OS webview and message passing between the frontend and Rust. Commands can be typed, async, error-returning, and channel-based. This aligns naturally with Rust ownership but does not provide Flutter widget/UI reuse.  
   Sources: https://v2.tauri.app/concept/architecture/ and https://v2.tauri.app/develop/calling-rust/ — Tauri — updated 2026/current — high — class: architecture/integration.

10. **Windows deployment inherits WebView2 as an operational dependency.** Tauri uses Edge-based WebView2 and its Windows installer can ensure or bootstrap a required runtime version; installers can be MSI or NSIS. The compatibility manifest must therefore include the WebView2 floor.  
    Sources: https://v2.tauri.app/reference/webview-versions/ and https://v2.tauri.app/distribute/windows-installer/ — Tauri — current docs — high — class: deployment/compatibility.

11. **Tauri has first-party desktop lifecycle facilities, but every capability remains an explicit plugin/security surface.** Official documentation exposes updater, notification, autostart, single-instance, process, stronghold, and window-state plugins plus capability/permission controls. Each used plugin must be pinned and included in security/conformance review.  
    Sources: https://v2.tauri.app/plugin/updater/, https://v2.tauri.app/plugin/notification/, https://v2.tauri.app/concept/architecture/ — Tauri — current docs — high — class: platform/security.

12. **Tauri 3 exists only as alpha in the current release feed.** The official GitHub release page shows 3.0.0-alpha releases in September 2026. A production recommendation should remain on the current stable Tauri 2 line until a separately verified stable migration decision.  
    Source: https://github.com/tauri-apps/tauri/releases — Tauri GitHub — 2026-09 — high — class: version/maturity.

## Early synthesis

- The user hypothesis is technically coherent: Flutter can own mobile UI, Tauri can own Windows UI, and Rust can remain the shared implementation.
- It creates **two presentation stacks and two boundary technologies**: Dart FFI/codegen for Flutter and Tauri IPC/commands for Windows. The common contract must sit above both bridge mechanisms.
- Flutter Desktop is the strongest counterfactual because it removes the second UI stack; Tauri must justify itself through Windows-native desktop workflows, smaller deployment/runtime shape, web-ecosystem UI reuse, or Rust-host integration—not merely because Rust is present.
- Browser remains a third delivery profile: native FFI does not carry over, and either Rust/WASM or a gateway/companion is required.

## Leads for round 2

- Verify production evidence for Flutter + Rust at super-app scale and bridge upgrade burden.
- Compare Flutter Windows versus Tauri Windows on accessibility, text-heavy productivity UI, tray/background/service integration, updater and packaging.
- Define one `vida-contracts` API model that can generate Dart FFI DTOs and Tauri IPC DTOs without exposing bridge-specific types.
- Prototype cancellation, stream backpressure, shutdown/restart, crash recovery, and mixed-version compatibility across both bridges.

