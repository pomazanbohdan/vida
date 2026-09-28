# Spotify mobile architecture evidence digest — round 1

- Topic: Spotify precedent for moving from React Native to native clients while retaining shared product logic
- Decision served: define a platform boundary between stable shared product logic and native clients
- Accessed: 2026-09-19
- Scope: Spotify primary sources first; one Spotify-supervised university thesis is used where Spotify has not published equivalent implementation detail
- Epistemic note: no retrieved Spotify source says that the main Spotify iOS/Android app migrated from React Native. This digest therefore treats “React Native → native” as the proposed decision context, not as Spotify history.

## Decision digest

The defensible Spotify precedent is **shared native core + platform-specific application/UI layers**, not a documented React Native exit. Historical, Spotify-supervised evidence identifies a C++ core shared across most platforms, with Android Java/SDK code above it and JNI between layers; current Spotify recruiting material still describes Client C++ as building core client parts such as playback and OS/partner integrations. Spotify’s iOS UI evidence is explicitly native and component-driven. Networking and at least some caching were designed behind a common C++ interface backed by native platform HTTP implementations. The boundary is therefore broader than “pure business rules”: it has included playback, networking, caching, persistence-oriented data processing, and OS integrations.

For a new architecture, the safest transferable pattern is: keep native UI, navigation, lifecycle, accessibility, permissions, and platform integrations in Swift/Kotlin; put deterministic domain state transitions, playback/domain orchestration, wire/domain models, sync rules, and cross-platform persistence semantics in a stable shared core; place actual OS storage and HTTP primitives behind narrow ports when native facilities are advantageous. Do **not** claim Spotify evidence for Rust, UniFFI, generated Swift/Kotlin bindings, or an RN strangler migration: these remain open design choices.

## Findings

### F1 — Spotify’s documented shared runtime was native C++, not JavaScript/React Native

- claim: **Explicit.** A 2016 master’s thesis conducted on Spotify’s Android client under a Spotify AB supervisor states that the application had a native C++ core shared among most supported platforms. Android-specific parts were Java using the Android SDK; the Android NDK deployed the core and JNI connected the Java and C++ layers.
- source URL: https://liu.diva-portal.org/smash/get/diva2:910711/FULLTEXT01.pdf
- publisher: Linköping University; author Jens Green Olander; Spotify AB supervisor Robert Nissa Holmgren
- pub_date: 2016-03-08
- accessed: 2026-09-19
- confidence: high for the 2016 architecture; medium for present-day generalization
- class: shared-core-language-runtime; explicit; historical-primary-adjacent

### F2 — Current public Spotify material still identifies a Client C++ core

- claim: **Explicit.** Spotify’s current Engineering careers page says its Client C++ discipline builds “the core parts for our clients,” including playback and OS/partner integrations. This confirms continued organizational ownership of a C++ client-core domain, although it does not enumerate which mobile features use it or prove that all mobile shared logic remains C++.
- source URL: https://www.lifeatspotify.com/find-your-team/job-categories/engineering
- publisher: Spotify
- pub_date: n.d. (live page retrieved 2026-09-19)
- accessed: 2026-09-19
- confidence: high for the stated current responsibility; medium for mobile-specific scope
- class: shared-core-current-signal; explicit

### F3 — The shared boundary has included infrastructure-heavy logic, not only pure domain rules

- claim: **Explicit + inference.** The 2016 client description says network access was mostly implemented in the native core. Current Spotify material assigns playback and OS/partner integrations to Client C++. Therefore Spotify’s shared core is not evidence for a narrowly “pure business logic only” kernel; it is evidence for a stable reusable client engine that may include stateful and platform-facing capabilities behind interfaces.
- source URL: https://liu.diva-portal.org/smash/get/diva2:910711/FULLTEXT01.pdf ; https://www.lifeatspotify.com/find-your-team/job-categories/engineering
- publisher: Linköping University / Spotify
- pub_date: 2016-03-08 / n.d.
- accessed: 2026-09-19
- confidence: high that networking/playback have lived in core; medium for the recommended boundary, which is an inference
- class: domain-boundary; mixed-explicit-inference

### F4 — Spotify has shipped native, platform-specific iOS UI above model/content layers

- claim: **Explicit, historical.** Spotify’s Hub Framework is described as a toolkit for native, component-driven iOS UI, used in production in the Spotify iOS app. It was written in Objective-C, consumable from Objective-C or Swift, and was designed to decouple model code from UI through content operations. Existing `UIView` implementations could be adopted without wholesale rewrite.
- source URL: https://spotify.github.io/HubFramework/
- publisher: Spotify Open Source; authors include John Sundell, Aron Cedercrantz, and Robin Goos
- pub_date: 2016 (page copyright 2015–2016; exact release date not stated)
- accessed: 2026-09-19
- confidence: high for the historical iOS implementation; low for current framework use
- class: native-ui; explicit; historical

### F5 — Spotify has used common networking semantics with native platform transports

- claim: **Explicit, historical.** NFHTTP, developed at Spotify from 2019–2022, exposed a common cross-platform C++ HTTP interface while interfacing with platform HTTP systems. Spotify states that it wanted native-backed networking for battery efficacy and a caching layer consistent across platforms. The project was discontinued by Spotify and handed to new maintainers in January 2023, so it is architectural evidence, not a current technology recommendation.
- source URL: https://github.com/nativeformat/NFHTTP/blob/master/README.md
- publisher: Spotify-origin project, now nativeformat maintainers
- pub_date: 2023-01 (handover statement; original development 2019–2022)
- accessed: 2026-09-19
- confidence: high
- class: networking-ownership; explicit; historical

### F6 — Feature persistence and synchronization were implemented as shared client architecture with platform storage details still visible

- claim: **Explicit + bounded inference.** In the Liked Songs redesign, metadata was downloaded in batches, stored on device or SD card, sorted views were precomputed and stored on disk, and later launches read those tables while checking Spotify servers for deltas. Spotify says the work changed roughly 100,000 lines in the shared client codebase and also required Android and iOS changes. It is reasonable to infer that shared code owned the cross-platform data semantics while platform apps retained integration responsibilities; the article does not name the database, storage adapter API, or exact ownership seam.
- source URL: https://engineering.atspotify.com/2020/5/spotify-modernizes-client-side-architecture-to-accelerate-service-on-all-devices
- publisher: Spotify Engineering; Carl Engström
- pub_date: 2020-05-28
- accessed: 2026-09-19
- confidence: high for the behavior and migration size; medium for ownership inference
- class: persistence-sync-boundary; mixed-explicit-inference

### F7 — Android FFI was JNI; no Spotify evidence retrieved for iOS binding mechanics or binding code generation

- claim: **Explicit + gap.** The Spotify-supervised thesis explicitly identifies Android NDK packaging and JNI bindings between the Java client layer and C++ core. The retrieved Spotify sources do not specify whether iOS used Objective-C++, a C ABI, handwritten wrappers, generated bindings, or another mechanism; nor did they identify schema/code generation for shared-core APIs.
- source URL: https://liu.diva-portal.org/smash/get/diva2:910711/FULLTEXT01.pdf
- publisher: Linköping University; author Jens Green Olander; Spotify AB supervisor Robert Nissa Holmgren
- pub_date: 2016-03-08
- accessed: 2026-09-19
- confidence: high for Android JNI; high that the retrieved corpus leaves iOS/codegen unanswered
- class: ffi-codegen; explicit-plus-gap

### F8 — Cross-layer native code creates observability and debugging costs

- claim: **Explicit, historical.** The thesis reports that JVM stack inspection was insufficient because many calls crossed JNI into native core components, especially networking; Android method tracing could not cover native C++ code. Spotify’s instrumentation therefore used system logs and source/line attribution to diagnose behavior. This is a direct warning that a shared native core requires cross-boundary tracing, crash symbolication, and ownership-aware diagnostics.
- source URL: https://liu.diva-portal.org/smash/get/diva2:910711/FULLTEXT01.pdf
- publisher: Linköping University; author Jens Green Olander; Spotify AB supervisor Robert Nissa Holmgren
- pub_date: 2016-03-08
- accessed: 2026-09-19
- confidence: high for the observed constraint; medium for the modern operational implication
- class: implementation-cost-observability; explicit-plus-inference

### F9 — Spotify’s historical Android conformance testing combined system automation with model-based testing

- claim: **Explicit, historical.** Spotify’s Android test-automation tools ran system tests continuously in CI on debug builds and physical devices. JUnit test drivers controlled the app over USB/TCP, including UI interactions and network toggles. The thesis also records model-based testing with application states as vertices and input actions as edges.
- source URL: https://liu.diva-portal.org/smash/get/diva2:910711/FULLTEXT01.pdf
- publisher: Linköping University; author Jens Green Olander; Spotify AB supervisor Robert Nissa Holmgren
- pub_date: 2016-03-08
- accessed: 2026-09-19
- confidence: high for the 2016 test system
- class: testing-conformance; explicit; historical

### F10 — The Liked Songs migration used behavioral inventory, parity checks, parallel development, and staged rollout

- claim: **Explicit.** Spotify spent about a year on the Liked Songs client-architecture change, mapped every way a user could like a song, required the replacement to reproduce existing behavior, tracked ongoing changes in the live architecture, and developed/testing in parallel. Exposure grew from 30 employees to 1,000, then about 1% of users, then 50%, before full switch-over.
- source URL: https://engineering.atspotify.com/2020/5/spotify-modernizes-client-side-architecture-to-accelerate-service-on-all-devices
- publisher: Spotify Engineering; Carl Engström
- pub_date: 2020-05-28
- accessed: 2026-09-19
- confidence: high
- class: migration-strategy; explicit

### F11 — Spotify’s build-system migration supplies a stronger dual-run/differential-validation playbook

- claim: **Explicit; analogous rather than RN-specific.** During the iOS Bazel migration, Spotify ran Xcode and Bazel side by side, generated more than 2,000 Bazel build files from an existing declarative project model (only 30–50 manually), produced optimized builds on the same cadence, and compared packaged assets, configurations, and binary symbols for unknown differences. Rollout proceeded through employee, alpha, and beta cohorts while the prior build path remained available for several releases.
- source URL: https://engineering.atspotify.com/2023/10/switching-build-systems-seamlessly
- publisher: Spotify Engineering; Patrick Balestra
- pub_date: 2023-10-17
- accessed: 2026-09-19
- confidence: high for the migration mechanics; medium for transfer to an RN-to-native rewrite
- class: migration-strategy-differential-conformance; explicit-plus-analogy

### F12 — Spotify’s current release practice supports feature flags and early-cycle isolation for high-risk changes

- claim: **Explicit.** Spotify’s release process uses trunk-based development; large infrastructure changes are merged early in a release cycle for more test time, and teams are encouraged to hide risky changes behind feature flags. The release team can route users away from a failing experiment through backend configuration while a client fix waits for the next release.
- source URL: https://engineering.atspotify.com/2025/4/how-we-release-the-spotify-app-part-1
- publisher: Spotify Engineering
- pub_date: 2025-04-17
- accessed: 2026-09-19
- confidence: high
- class: migration-release-safety; explicit

### F13 — Shared native libraries impose binary-size and packaging constraints

- claim: **Explicit.** Spotify Lite included a native shared playback library. Spotify used linker/compiler techniques including lld, link-time optimization, and disabling RTTI, stored the shared library unpacked in the APK, and added continuous-delivery checks against size growth. This is evidence that a shared native core must carry explicit size budgets and packaging tests.
- source URL: https://engineering.atspotify.com/2020/12/how-we-built-it-spotify-lite-one-year-later
- publisher: Spotify Engineering; Erik Ghonyan, Slava Savitskiy, Tommy Tynjä
- pub_date: 2020-12-03
- accessed: 2026-09-19
- confidence: high
- class: native-core-operational-cost; explicit-plus-inference

### F14 — Spotify’s Android state-management tooling demonstrates a pure/testable core separated from UI adapters, but it is Android-specific

- claim: **Explicit.** Mobius separates state evolution and side effects, emphasizes testability, ships a self-contained pure-Java core plus a separate test module, and treats the Android module as an adapter for connecting a Mobius loop to Android UI. Spotify says Mobius is used in production Spotify Android applications. This supports a boundary pattern—pure state machine + effects + platform adapter—but does not establish cross-iOS sharing.
- source URL: https://github.com/spotify/mobius
- publisher: Spotify Open Source
- pub_date: n.d. (live repository retrieved 2026-09-19)
- accessed: 2026-09-19
- confidence: high for module structure and Android production claim; high that it is not cross-platform proof
- class: domain-state-boundary-testing; explicit

## Recommended platform boundary derived from the evidence

The following is an **architectural inference**, not a description Spotify publishes as a single canonical diagram.

| Shared stable core | Native Swift/Kotlin clients | Port/adapter seam |
|---|---|---|
| Domain entities and invariants | Rendering, layout, animation, accessibility | Typed request/response/event DTOs |
| Deterministic state transitions and commands | Navigation and deep-link routing | Async cancellation and lifecycle contract |
| Sync, ordering, pagination, retry, conflict rules | App lifecycle, permissions, notifications | Clock, randomness, feature flags |
| Playback/product orchestration independent of OS media APIs | OS media session/audio integration | Playback device/service interfaces |
| Cross-platform persistence semantics and migrations | Keychain/Keystore and filesystem locations | KV/database/blob store interfaces |
| Network policy, endpoint/domain mapping, caching semantics | Native HTTP engine and reachability where beneficial | HTTP transport interface + canonical errors |
| Analytics event definitions and sampling rules | Platform-specific event sources and privacy prompts | Telemetry sink |

Boundary rule: shared code may decide **what** should happen and preserve product invariants; native code owns **how the OS presents or performs it**. Spotify evidence shows that performance-sensitive capabilities may move below this line into a reusable native engine, but each such move increases FFI, debugging, packaging, and rollout costs.

## Contradictions and cautions

1. **The prompt’s implied Spotify RN migration is unsupported.** Searches of Spotify Engineering, Spotify Open Source/GitHub, Spotify careers, and author/institutional material did not produce a primary source saying the main mobile app was React Native or migrated from React Native to native.
2. **“Shared business logic” is too narrow for the observed Spotify core.** Retrieved evidence puts networking, caching, playback, and some OS/partner integrations in or adjacent to Client C++; Spotify also says backend teams build business logic and protocols across devices. Product logic is therefore distributed across backend, shared client core, and native clients.
3. **Historical sources may not describe the 2026 implementation.** The precise C++/Java/JNI diagram is from 2016; Hub Framework evidence is from 2015–2016; NFHTTP left Spotify stewardship in 2023. Current Spotify material corroborates a Client C++ responsibility but not the complete modern mobile layering.
4. **No evidence of automatic Swift/Kotlin binding generation.** The Android JNI seam is documented; iOS FFI and codegen remain unverified.
5. **No public shared-core contract test suite was found.** Public evidence covers Android system/model-based tests, staged cohorts, dual-build comparisons, release flags, and Mobius unit-test support, but not cross-platform golden vectors proving identical Swift/Kotlin-facing behavior.

## Leads worth chasing

- Carl Engström, Felipe Oliveira Carvalho, Marcus Fredriksson, Niklas Björnberg, Oscar Andersson, and Marcus Vesterlund: original participants named in the Liked Songs migration; conference talks or personal posts may expose the persistence/core seam.
- John Sundell, Aron Cedercrantz, Robin Goos: Hub Framework authors; possible talks/posts may clarify iOS model/core integration.
- Spotify Client C++ job archives and conference talks: may document current mobile binding, ownership, or code-generation systems.
- Bazel mobile talks by Patrick Balestra: may identify language targets and test topology beyond the engineering post.
- Public symbols/artifacts in Spotify Android/iOS releases could technically reveal native library names, but reverse engineering was out of scope and would still not establish intended architecture.

## Gaps

- Exact current shared-core language mix and runtime topology on iOS/Android.
- Whether Spotify ever used React Native in its main mobile clients.
- iOS core binding mechanism and whether bindings are handwritten or generated.
- Public IDL/schema ownership, ABI/versioning policy, threading model, cancellation semantics, and memory ownership across FFI.
- Exact database technology and whether storage engines are shared or native adapters.
- Cross-platform contract/golden testing, replay harnesses, fuzzing, and parity metrics.
- Feature-by-feature migration sequencing from a JavaScript/RN layer to native screens.
- Quantified ongoing cost of the shared core: build time, crash attribution, developer staffing, and release coupling.

## Search log / negative evidence

- Searched Spotify Engineering for `React Native`, mobile shared logic, shared client, C++ core, native UI, Swift/Kotlin, networking, persistence, testing, releases, and migration.
- Searched Spotify GitHub/Open Source for C++ mobile libraries, JNI, NFHTTP, Mobius, Hub Framework, Android/iOS SDKs, and shared core.
- Searched Spotify careers for current Client C++ and mobile ownership descriptions.
- Followed original authors/institutional sources where official posts did not expose mechanics.
- Excluded community “Spotify clone” repositories and third-party React Native wrappers as evidence about Spotify’s internal architecture.

## Sources consulted

1. Spotify Engineering — [Spotify Modernizes Client-Side Architecture to Accelerate Service on All Devices](https://engineering.atspotify.com/2020/5/spotify-modernizes-client-side-architecture-to-accelerate-service-on-all-devices), 2020-05-28.
2. Linköping University / Spotify AB supervised thesis — [Optimizing Communication Energy Efficiency for a Multimedia Application](https://liu.diva-portal.org/smash/get/diva2:910711/FULLTEXT01.pdf), 2016-03-08.
3. Spotify Engineering — [Switching Build Systems, Seamlessly](https://engineering.atspotify.com/2023/10/switching-build-systems-seamlessly), 2023-10-17.
4. Spotify Engineering — [How We Built It: Spotify Lite, One Year Later](https://engineering.atspotify.com/2020/12/how-we-built-it-spotify-lite-one-year-later), 2020-12-03.
5. Spotify Engineering — [A Behind-the-Scenes Look at How We Release the Spotify App (Part 1)](https://engineering.atspotify.com/2025/4/how-we-release-the-spotify-app-part-1), 2025-04-17.
6. Spotify Careers — [Engineering / Client C++](https://www.lifeatspotify.com/find-your-team/job-categories/engineering), live page.
7. Spotify Open Source — [Hub Framework reference](https://spotify.github.io/HubFramework/), 2015–2016.
8. Spotify-origin open source — [NFHTTP README](https://github.com/nativeformat/NFHTTP/blob/master/README.md), developed 2019–2022; handed over 2023-01.
9. Spotify Open Source — [Mobius repository](https://github.com/spotify/mobius), live repository.
