---
type: technical
shape: select
topic: VIDA UI platform and Rust binding strategy
decision: Select the UI-shell and Rust interoperability strategy without assuming Swift, Kotlin, or one universal UI framework
preset: deep
validation: high
red_team: on
created: '2026-09-19'
---

# Research brief

## Decision frame

Choose an implementation strategy for Android, iOS, desktop, and browser shells above the accepted headless Rust `vida-core` / `vida-runtime` architecture. No UI toolkit is preselected.

## Hard gates

- Rust remains the canonical shared product/runtime implementation.
- Platform shells cannot fork domain, authorization, protocol, or persistence semantics.
- Local-first/offline behavior, encryption, Iroh integration, accessibility, background work, notifications, secure storage, diagnostics, and independent release compatibility remain possible.
- The strategy must expose deterministic headless APIs and support the common Conformance Plane.
- Browser and desktop may use different shell technologies when evidence favors it; one universal UI framework is not a requirement.

## Scenario frame

Because v1 platform priority is not yet fixed, score recommendations separately for:

1. mobile-first production;
2. all-platform delivery;
3. maximum native UX/platform capability;
4. smallest team and fastest delivery;
5. lowest long-term lock-in and migration cost.

## Candidate screen

- Native UI: SwiftUI/UIKit + Kotlin/Compose/Views.
- Kotlin Multiplatform / Compose Multiplatform.
- Flutter.
- React Native.
- Tauri 2 and webview-based shells where applicable.
- Web/PWA and desktop-web hybrids.
- Credible Rust-native UI approaches only if current maturity passes screening.
- Mixed-shell strategy: best UI toolkit per platform over one Rust contract/core.

## Research dimensions

1. Current landscape, platform coverage, maturity, and credible five-year outlook.
2. Rust interoperability: UniFFI, C ABI/codegen, JNI, Swift/C++ interop, WASM, serialization boundaries.
3. Runtime semantics: threading, async, cancellation, memory ownership, callbacks/streams, errors, crash diagnostics.
4. Platform reality: lifecycle, background work, notifications, secure storage, accessibility, native UX, app-store constraints.
5. Desktop/browser integration, Iroh/runtime limits, WASM/companion/gateway options.
6. Testing, conformance, release independence, ABI/API compatibility, migrations, rollback, and observability.
7. Developer productivity, ecosystem health, operational burden, lock-in, and exit cost.
8. Weighted scenario matrix, runner-up conditions, reversibility hedge, and prototype plan.

## Acquisition plan

- Breadth-first fan-out in up to three rounds.
- Three researchers per wave due available concurrency; additional waves if novelty remains.
- Maximum 12 distinct sources per dimension/round, primary sources first.
- High validation: two-source verification for versions/compatibility, performance, and decisive comparison cells.
- Fresh-context red team against the emerging recommendation.

