# UI landscape and Rust interoperability — research digest R1

Decision date: 2026-09-19  
Access date for every source: 2026-09-19  
Scope: Android, iOS, Windows/macOS/Linux desktop, browser; local-first; Rust shared core/runtime; no assumption that Swift/Kotlin or one universal UI toolkit must own every surface.  
Method: current official documentation plus production retrospectives; no project files or ambient repository context read.

## Verdict

**Leader: mixed shell — Flutter on Android/iOS; a shared web UI in Tauri 2 on Windows and as the browser/PWA surface; Rust as a UI-agnostic local-first core.** This is the best risk-adjusted fit when mobile quality matters, Windows is a first-class installed product, browser delivery must remain independent, and Rust already owns durable domain/runtime behavior.

The hypothesis is not a universal-UI bet. It is a deliberate two-UI-system bet:

1. Flutter/Dart owns mobile presentation.
2. One DOM-based web frontend owns both browser and the Tauri window on Windows.
3. Rust owns domain rules, local data/sync, deterministic jobs, and portable services.
4. Each shell owns OS lifecycle, permissions, notifications, secure storage, background scheduling, deep links, and platform UI integration.

This boundary is decisive. Putting lifecycle/background/secure-storage abstractions inside the portable Rust core would turn OS policy differences into leaky emulation and erase the main advantage of mixed shells.

**Challenger A: native SwiftUI/UIKit + Kotlin Compose/Views over Rust via UniFFI.** Choose it instead when platform-native interaction fidelity, accessibility edge cases, extensions/widgets, or long-running background/device integration dominate staffing cost. It has the lowest platform-policy risk and highest duplicated UI/release cost.

**Challenger B: Kotlin Multiplatform shared logic with Compose Multiplatform UI.** Strongest for a Kotlin-heavy organization and now credible for iOS, but it adds a second portable runtime beside Rust, makes Rust↔Kotlin/Native/JNI interop an extra seam, and still has a less mature browser story. It is not the natural center of gravity for a Rust-core product.

**Do not select React Native, Tauri-mobile/PWA-primary, or a Rust-native GUI as the default all-surface shell in R1.** They remain tactical options, not architecture leaders.

Confidence: **medium-high**. Mobile and desktop conclusions have good official and production evidence. Browser accessibility, Flutter↔Rust bridge longevity, and five-year Tauri mobile maturity remain evidence gaps.

## Screen: leaders, challengers, wildcard, cuts

Scores: 1 weak → 5 strong. “Release” measures ability to evolve a surface independently without forcing all other surfaces to ship together. Scores are synthesized judgments, not measurements.

| Candidate | Coverage | Native UX/a11y | Lifecycle/device | Ecosystem | Rust fit | Independent release | 5-year regret | Screen |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| **Flutter mobile + shared Web/PWA + Tauri Windows + Rust core** | 5 | 4 mobile / 4 web | 4 | 4 | 5 | 5 | 4 | **Leader** |
| Native iOS + native Android + Rust/UniFFI | 3 | 5 | 5 | 5 | 4 | 3 | 4 | Challenger A |
| KMP logic + Compose Multiplatform UI | 4 | 4 | 4 | 4 | 2 | 3 | 3 | Challenger B |
| Flutter on every surface | 5 | 3 | 4 | 5 | 4 | 3 | 3 | Challenger C |
| React Native mobile + community desktop/web | 4 | 4 mobile / 3 elsewhere | 4 mobile | 5 mobile | 2 | 4 | 3 | Cut as default |
| Tauri/Web/PWA on every surface | 5 nominal | 2–3 mobile | 2–3 mobile | 4 web / 3 mobile | 5 | 5 | 2 | Cut as mobile default |
| Rust-native UI (Dioxus native/Slint/iced/egui class) | 3 nominal | 2 | 2 | 2 | 5 | 3 | 1 | Wildcard; cut for R1 |

### Why the mixed leader wins

- Flutter 3.47.2 officially supports Android, iOS, Windows, macOS, Linux, and major browsers, so Flutter-all is technically possible. The leader nevertheless limits Flutter to mobile because browser DOM semantics and desktop conventions are stronger long-term constraints than source-code reuse. [S1; confidence: high; class: official/current]
- Flutter executes native binaries but generally draws its own widget scene. On web it translates its Semantics tree into an accessible HTML DOM layer because the visible UI is canvas-rendered. Therefore “natively compiled” must not be read as “uses the platform’s native widget tree.” [S1; confidence: high; class: official/current]
- eBay Motors reported shipping a production Flutter app on schedule and continuing rapid feature delivery; its retrospective supports mobile delivery velocity, not browser or desktop equivalence. [S3; confidence: medium; class: first-party production retrospective]
- Tauri is structurally aligned with a Rust core: Rust is linked in-process, the webview calls Rust through commands/events/channels, and official plugins cover notifications, biometric authentication, persistent storage, SQL, deep links, and other OS integrations. The same frontend can remain a normal web/PWA build. [S4; confidence: high; class: official/current]
- A browser/PWA gives the cleanest independent release path, but browser background work is capability- and browser-policy-dependent; silent push is not supported. It is a surface, not a substitute for mobile OS integration. [S9; confidence: high; class: standards/reference]

## Candidate findings

### 1. Native SwiftUI/UIKit + Kotlin Compose/Views

**Strengths.** Direct platform APIs are the reference implementation for VoiceOver/TalkBack, text input, navigation behavior, notifications, Keychain/Keystore, widgets/extensions, background task classes, and lifecycle restoration. Apple explicitly exposes background modes and keychain accessibility classes; Android explicitly models Activities, Services, providers, process death, permissions, and lifecycle-specific components. [S11; confidence: high; class: platform-vendor documentation]

**Costs.** Two UI codebases, two specialist toolchains, duplicated visual QA, and asynchronous feature delivery. Rust interop is credible through UniFFI for Kotlin and Swift, but the generated binding does not package, sign, schedule, or lifecycle-manage the library for the target; the team still owns those integrations. [S10; confidence: high; class: official project documentation]

**Five-year regret.** Lowest framework abandonment risk, highest permanent staffing/coordination cost. Best escape hatch if mixed-shell mobile UX or accessibility becomes unacceptable.

**Screen:** Challenger A, not default.

### 2. Kotlin Multiplatform / Compose Multiplatform

Compose Multiplatform 1.8.0 made iOS Stable in May 2025, including accessibility support and SwiftUI/UIKit embedding. JetBrains reports production apps and benchmark results, but those performance claims are vendor-produced. The same roadmap described Compose for Web/Kotlin-Wasm as Beta and suitable first for pioneers shipping small-to-medium apps, so browser parity should not be assumed. [S5; confidence: high for declared stability, medium for comparative performance; class: vendor primary]

STRV’s January 2026 production retrospective used KMP for shared business logic with a fully native SwiftUI interface. It found the split effective but called out sealed-class translation, cross-runtime concurrency assumptions, and debugging. It explicitly treated shared Compose UI as a separate decision. [S6; confidence: medium-high; class: practitioner production retrospective]

For a Rust-centered product, KMP is awkward strategically: either Kotlin duplicates domain ownership or Rust must cross JNI on Android and Kotlin/Native/Swift interop on iOS. Compose is a compelling UI option for Kotlin organizations, not a simplifier of a Rust core.

**Screen:** Challenger B; reconsider if Kotlin becomes the dominant client skill and browser is separate.

### 3. Flutter

**Coverage/maturity.** Flutter’s current support matrix is the broadest single-vendor matrix in the study. Its mobile support and tooling are mature; desktop and web are supported, not merely experimental. [S1; confidence: high; class: official/current]

**UX/accessibility.** Flutter provides semantics and assistive-technology support, but a custom-rendered scene requires discipline. On web, standard widgets can populate semantics automatically while custom components need explicit roles; this is a measurable a11y QA obligation, not a theoretical concern. [S1; confidence: high; class: official/current]

**Lifecycle/background/native features.** Flutter can run Dart background callbacks and use platform service integrations, but iOS/Android scheduling rules still govern execution. Notifications, secure storage, biometrics, and unusual device features are plugin/platform-channel concerns; the shell must retain native escape hatches. [S1; confidence: high; class: official/current]

**Rust interop.** `flutter_rust_bridge` 2.13.0 was the current stable registry release at access time, with Android, iOS, desktop, and web listed; 2.14.0-beta.2 was prerelease. This is unusually good ergonomic coverage, but it is a community bridge rather than a Flutter/Rust platform guarantee. Pin it, own generated-code review, and maintain a manual C-ABI fallback design. [S2; confidence: high for registry state, medium for longevity; class: package registry/community]

**Independent release.** Android and iOS still ship through stores. Flutter does not eliminate platform release governance. Feature flags and data-driven presentation may decouple some behavior, but executable/UI changes remain binary releases.

**Screen:** Leader on mobile; Challenger C across every surface.

### 4. React Native

React Native’s primary promise is real native platform views from React primitives, and its mobile ecosystem is deeply production-proven. Official positioning, however, describes Windows, macOS, and Web as community initiatives “beyond Android and iOS,” not one parity-guaranteed platform surface. [S7; confidence: high; class: official/current]

Shopify migrated two large apps with hundreds of screens and more than 40 native modules to the New Architecture while keeping weekly releases. Its retrospective also reports dependency audits, patched/removed unmaintained libraries, production-only hangs from legacy modules, and the need for phased rollout. This is evidence of maturity and operational cost at the same time. [S8; confidence: high; class: first-party production retrospective]

For this decision RN adds JavaScript↔native modules plus Rust↔native bindings; Windows/Web still need distinct ecosystem choices. It does not beat Flutter mobile on Rust bridge simplicity or the mixed web/Tauri plan on desktop/browser reuse.

**Screen:** Cut as default; retain only if React staffing and native-view reuse dominate Rust integration concerns.

### 5. Tauri 2 and Web/PWA hybrids

Tauri’s architecture is a Rust process plus HTML in an OS webview, connected by message passing. Commands can return typed serialized values, while channels/binary responses are preferable for streams or large payloads. Official plugins cover key product capabilities, but plugin availability is not proof of equal behavior across every OS. [S4; confidence: high; class: official/current]

For Windows, Edge WebView2 plus direct Rust linkage is a strong local-first shell: good DOM accessibility, mature frontend tooling, small packaging compared with bundling a browser, and no extra FFI between shell and core. Webview/runtime variance and IPC authorization are the main engineering obligations.

For mobile, Tauri 2 has nominal Android/iOS support and useful plugins, but the study did not find production evidence comparable to Flutter/RN at large scale. A small-app retrospective found fast prototyping and repeatable Android builds, but also manual UI testing, compile/storage overhead, and an unimplemented Android directory selector; the author had not tested iOS. [S4; confidence: medium; class: practitioner retrospective]

PWA is excellent for reach and independent deployment. It is not a reliable owner for unrestricted background work, silent push, OS-secure secret handling, or extension-grade integration. [S9; confidence: high; class: standards/reference]

**Screen:** Leader on Windows and browser; cut as primary mobile shell.

### 6. Credible Rust-native UI

Dioxus 0.7 documents web, webview-backed desktop/mobile, and an experimental native renderer (Blitz). That is an explicit maturity boundary: the “native Rust UI” path is experimental, while the production-shaped path returns to webviews. [S12; confidence: high; class: official project documentation]

Slint is the most credible further spike because it publishes mobile/desktop docs and accessibility properties, but R1 lacks large-scale, independent evidence for consumer mobile lifecycle, notifications, secure storage, screen-reader edge cases, and plugin breadth. iced/egui-style toolkits have still weaker evidence for a cross-platform consumer super-app.

**Screen:** Wildcard; cut for R1. Revisit only after an accessibility + IME + lifecycle spike on physical Android/iOS/Windows devices.

## Recommended Rust binding and ownership strategy

```text
Flutter mobile shell              Web browser/PWA          Tauri Windows shell
  Dart UI + OS plugins              DOM UI                   same DOM UI
  lifecycle/background owner        browser APIs             Windows integration owner
            | FRB generated calls       | wasm-bindgen/API       | direct Rust commands/channels
            +---------------------------+------------------------+
                                        |
                                Rust application facade
                         versioned commands + events + snapshots
                                        |
                       domain / local DB / sync / crypto primitives
```

### Binding choices

1. **Flutter mobile → `flutter_rust_bridge` 2.x**, generated from a narrow Rust application facade. Use async calls, streams only for coarse events, and byte buffers/file handles for bulk transfer. Do not expose internal Rust object graphs or lifetimes.
2. **Tauri Windows → direct crate dependency**, with thin `#[tauri::command]` adapters and channels. There is no reason to insert a C ABI between Tauri and the Rust core.
3. **Browser → Rust/Wasm only for code that pays for Wasm.** Keep browser storage, notifications, network reachability, and service-worker control in TypeScript. If DB/sync code depends on native files/threads, expose a browser-specific Rust feature set instead of pretending it is identical.
4. **Native fallback → UniFFI for Swift/Kotlin.** Preserve a facade that UniFFI can express, even if native shells are not initially built. This is the practical exit strategy from Flutter. [S10; confidence: high; class: official project documentation]

### Contract rules

- Version the **semantic command/event contract**, not a shared UI model.
- Use stable IDs, tagged enums, explicit error codes, pagination, cancellation tokens, and resumable job IDs.
- Never let the UI hold the only copy of durable job state.
- Make Rust jobs restartable after process death; shells schedule/wake them but do not contain domain truth.
- Keep secure tokens in Keychain/Keystore/Windows credential facilities; pass short-lived credentials or opaque handles to Rust.
- Keep notification rendering/action registration in the shell; Rust emits domain events and consumes user actions.
- Keep database migrations in Rust and test forward/backward opening across N−1/N app versions.
- Add golden serialization fixtures, generated-binding compile tests, physical-device smoke tests, and contract compatibility CI for every shell.
- Avoid high-frequency chatty FFI/IPC. Batch state snapshots; use event coalescing; move large media/file transfer out of JSON.

These rules make the UI replaceable and contain the principal five-year risk: bridge/toolkit churn.

## Release topology

| Artifact | Release owner | Can ship independently? | Compatibility obligation |
|---|---|---|---|
| Android Flutter app | mobile team / Play | Yes | Rust facade + DB schema N−1 |
| iOS Flutter app | mobile team / App Store | Yes | Rust facade + DB schema N−1; Apple lifecycle policy |
| Browser/PWA | web team / web deploy | Yes, fastest | Browser feature detection; backend/sync protocol |
| Windows Tauri app | desktop team / installer/store | Yes | Rust facade; WebView2 floor; updater rollback |
| Rust core | runtime team, embedded per artifact | No standalone user release | semver + migrations + per-shell generated adapters |

Do not distribute one prebuilt “universal core binary” to every shell. Publish versioned Rust crates plus reproducible target builds. The browser target will necessarily have different capabilities from native targets.

## Five-year regret register

| Risk | Probability / impact | Early signal | Mitigation / exit |
|---|---|---|---|
| Flutter mobile misses new platform UX/a11y behavior | M / H | repeated native exceptions; assistive-tech defects | native-view escape hatches; UniFFI-ready facade; annual native-shell reassessment |
| `flutter_rust_bridge` churn/bus factor | M / H | incompatible generator changes; slow releases | pin toolchain; generated-code checks; C ABI proof-of-concept; narrow facade |
| Three toolchains overwhelm team | M / H | feature skew, QA lag, release coupling | exactly two UI systems; reuse web UI in Tauri; shell owners; shared contract tests |
| Tauri/WebView regression or capability gap | M / M | OS/WebView-specific incidents | capability matrix; minimum WebView; Rust-side integration tests; native Windows module escape hatch |
| Browser treated as equal background runtime | H / H | missed sync/push after suspension | durable server/native scheduling; visible degraded-mode UX; never promise silent background work |
| Rust core absorbs OS policy | M / H | `cfg(platform)` spreads through domain crates | strict port/adaptor layer; shell-owned lifecycle/security; architecture lint/review |
| Compose Multiplatform overtakes Flutter for mobile | M / M | better iOS adoption/tooling, Kotlin staffing advantage | UI-agnostic Rust facade makes shell replacement possible |
| Native Rust UI matures rapidly | M / L | stable mobile renderers, a11y certifications, production case studies | revisit in 18–24 months; do not pre-optimize now |

## Contradictions resolved

1. **“Flutter is native” vs “Flutter is canvas-rendered.”** Both can be true: code is ahead-of-time compiled and calls native APIs, while most UI pixels come from Flutter’s renderer rather than UIKit/Android widgets. The distinction matters for platform look changes and accessibility testing. [S1]
2. **“Compose iOS is production-ready” vs “use native UI where it counts.”** JetBrains stabilized Compose iOS APIs; a practitioner still chose native SwiftUI and reported cross-runtime friction in shared logic. Stability is not the same as the best organizational boundary. [S5, S6]
3. **“React Native is native UI” vs “one codebase for all surfaces.”** RN primitives use native mobile views, but official desktop/web coverage is described as ecosystem initiatives. Native rendering does not guarantee uniform platform coverage. [S7]
4. **“Tauri supports mobile” vs “Tauri should not lead mobile.”** Supported build targets and plugins establish feasibility; they do not supply Flutter/RN-scale evidence for mobile UX, lifecycle, and ecosystem maturity. [S4]
5. **“PWA receives push while closed” vs “PWA cannot be background runtime.”** Push can wake a service worker, but silent push is unsupported and background APIs remain browser-controlled. [S9]

## Gaps requiring spikes before commitment

1. **Flutter accessibility bake-off:** VoiceOver, TalkBack, switch control, dynamic type/font scaling, keyboard-only, RTL, text selection, reduced motion; compare one representative workflow with native.
2. **FRB soak test:** 24-hour sync, cancellation, process kill/restart, 100 MB transfer, memory pressure, crash symbolication, release-mode iOS/Android builds.
3. **Tauri Windows proof:** WebView2 minimum, offline startup, auto-update rollback, tray/background behavior, notifications/actions, secure secret storage, screen reader, high-DPI/multi-window.
4. **Browser degradation proof:** quota/eviction, offline migration, push after long idle, no-silent-push UX, IndexedDB corruption/recovery, multi-tab locking.
5. **Release rehearsal:** ship incompatible-looking but contract-compatible mobile/web/Windows versions in every ordering; confirm N−1 DB/protocol behavior.
6. **Staffing model:** quantify Dart + TypeScript + Rust ownership. The mixed leader is invalid if the organization cannot sustainably own two UI systems.

## Decision gates

Adopt the mixed leader only if all gates pass:

- Mobile a11y/interaction spike has no P0/P1 gap without maintainable native escape hatches.
- FRB bridge survives kill/restart, cancellation, and bulk-data tests with pinned reproducible builds.
- The browser UI runs unchanged inside Tauri except for a typed platform adapter.
- Secure storage, notifications, and background scheduling stay shell-owned.
- Core contract remains expressible through FRB, Tauri commands, Wasm, and UniFFI fallback.
- Team accepts two UI stacks and independent release/QA pipelines.

If any of the first two gates fail, choose native mobile + UniFFI. If web-in-Tauri reuse fails, compare Flutter Windows against native Windows/Tauri as a separate desktop decision rather than forcing a universal shell.

## Evidence ledger — 12 source records

### S1 — Flutter official platform/accessibility/background documentation

- URLs: https://docs.flutter.dev/reference/supported-platforms ; https://docs.flutter.dev/resources/faq ; https://docs.flutter.dev/ui/accessibility/web-accessibility
- Publisher: Flutter / Google
- pub_date: n.d., live documentation; platform page observed at Flutter 3.47.2
- accessed: 2026-09-19
- class: official current documentation
- supports: six-platform support matrix; native-code integration/background execution; semantics-to-DOM accessibility model
- confidence: high for declared support/capabilities; medium for real-world parity

### S2 — flutter_rust_bridge registry state

- URL: https://pub.dev/packages/flutter_rust_bridge/versions
- Publisher: pub.dev package registry; package maintained by community authors
- pub_date: 2026-08, relative registry timestamp; stable 2.13.0, prerelease 2.14.0-beta.2 at access
- accessed: 2026-09-19
- class: package registry / community implementation
- supports: current version, minimum Dart SDK, listed Android/iOS/desktop/web targets
- confidence: high for snapshot; medium for five-year maintenance

### S3 — eBay Motors Flutter production retrospective

- URL: https://innovation.ebayinc.com/stories/ebay-motors-accelerating-with-fluttertm/
- Publisher: eBay Tech / Innovation
- pub_date: 2021-02 (page/search metadata)
- accessed: 2026-09-19
- class: first-party production retrospective
- supports: schedule, team velocity, continued feature delivery, high mobile code sharing
- confidence: medium; successful case, old and self-reported

### S4 — Tauri 2 architecture, Rust calls, plugins, and mobile retrospective

- URLs: https://v2.tauri.app/concept/architecture/ ; https://v2.tauri.app/develop/calling-rust/ ; https://v2.tauri.app/plugin/ ; https://blog.erikhorton.com/2025/10/05/4-mobile-apps-with-tauri-a-retrospective.html
- Publisher: Tauri project; Erik Horton for retrospective
- pub_date: architecture updated 2026-06-30; retrospective 2025-10-05; other docs n.d.
- accessed: 2026-09-19
- class: official current documentation + independent practitioner retrospective
- supports: webview/Rust message architecture; command/channel model; official capability list; small-app mobile experience and gaps
- confidence: high for architecture; low-medium for mobile production maturity

### S5 — Compose Multiplatform stability and roadmap

- URLs: https://blog.jetbrains.com/kotlin/2025/05/compose-multiplatform-1-8-0-released-compose-multiplatform-for-ios-is-stable-and-production-ready/ ; https://kotlinlang.org/docs/multiplatform/kotlin-multiplatform-roadmap.html
- Publisher: JetBrains / Kotlin
- pub_date: 2025-05-06; roadmap update 2025-08
- accessed: 2026-09-19
- class: vendor primary
- supports: iOS Stable, accessibility/native interop, Web Beta status, vendor performance claims
- confidence: high for release status; medium for benchmark/generalized production claims

### S6 — KMP production retrospective

- URL: https://www.strv.com/blog/kotlin-multiplatform-in-production-what-worked-what-didn-t
- Publisher: STRV
- pub_date: 2026-01-06
- accessed: 2026-09-19
- class: practitioner production retrospective
- supports: KMP logic + native SwiftUI boundary; concurrency/type/debugging costs
- confidence: medium-high; concrete project, one consultancy case

### S7 — React Native official positioning

- URL: https://reactnative.dev/
- Publisher: React Native project / Meta ecosystem
- pub_date: n.d., live documentation
- accessed: 2026-09-19
- class: official current documentation
- supports: native primitives; Android/iOS center; Windows/macOS/Web as ecosystem initiatives
- confidence: high

### S8 — Shopify React Native New Architecture migration

- URL: https://shopify.engineering/react-native-new-architecture
- Publisher: Shopify Engineering
- pub_date: 2025-09-05
- accessed: 2026-09-19
- class: first-party large-scale production retrospective
- supports: production scale, weekly releases, migration tactics, dependency/native-module issues, phased rollout
- confidence: high

### S9 — PWA offline/background reference

- URL: https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps/Guides/Offline_and_background_operation
- Publisher: MDN / Mozilla contributors
- pub_date: n.d., live reference
- accessed: 2026-09-19
- class: standards-oriented reference documentation
- supports: service workers, sync/push model, lack of silent-push browser support
- confidence: high for web model; platform-specific behavior still requires device testing

### S10 — UniFFI guide

- URL: https://mozilla.github.io/uniffi-rs/
- Publisher: Mozilla / UniFFI project
- pub_date: n.d., live documentation
- accessed: 2026-09-19
- class: official project documentation
- supports: generated Rust bindings for Kotlin and Swift; boundary/packaging limitations
- confidence: high

### S11 — Native platform lifecycle and secure-storage references

- URLs: https://developer.apple.com/documentation/uikit/preparing-your-ui-to-run-in-the-background ; https://developer.apple.com/documentation/security/restricting-keychain-item-accessibility ; https://developer.android.com/guide/components/fundamentals ; https://developer.android.com/topic/architecture
- Publisher: Apple; Google Android Developers
- pub_date: n.d., live platform documentation
- accessed: 2026-09-19
- class: platform-vendor primary documentation
- supports: OS-specific lifecycle, background modes/components, keychain access classes, process death, services and permissions
- confidence: high

### S12 — Dioxus renderer maturity

- URL: https://dioxuslabs.com/learn/0.7/beyond/project_structure/
- Publisher: Dioxus project
- pub_date: n.d., 0.7 documentation
- accessed: 2026-09-19
- class: official project documentation
- supports: web/webview renderers and explicit experimental status of native Blitz renderer
- confidence: high for declared status; low for broader Rust-GUI generalization

## Leads for R2

- Slint mobile + accessibility physical-device evaluation: https://docs.slint.dev/latest/docs/slint/guide/platforms/mobile/general/ and https://docs.slint.dev/latest/docs/slint/reference/common/
- Tauri v2 external security audit and IPC boundary review.
- Current Flutter engine/platform-view accessibility issues and real iOS/Android production postmortems newer than eBay.
- Current Compose for Web production cases after Beta.
- FRB maintainer/bus-factor, release cadence, generated ABI compatibility, and iOS App Store production cases.

