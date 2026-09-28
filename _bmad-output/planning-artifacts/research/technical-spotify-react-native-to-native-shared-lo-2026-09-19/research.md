---
title: 'Technical research: Shopify native migration, Spotify shared core, and the VIDA client boundary'
type: technical
topic: 'Shared headless core with native platform clients'
decision: 'Define the VIDA boundary between stable shared product logic, Iroh/platform infrastructure, and native client implementations'
source: 'native web research'
status: complete
preset: standard
validation: normal
created: '2026-09-19'
updated: '2026-09-19'
verified_claims: 7
unverified_claims: 0
---

# Shopify native migration, Spotify shared core, and the VIDA client boundary

**Decision this research serves:** Define the VIDA boundary between stable shared product logic, Iroh/platform infrastructure, and native client implementations.

## Executive summary

The remembered story combines two different precedents. **Shopify**, not Spotify, announced on 2026-09-10 that it is moving mobile apps from React Native to native Swift and Kotlin. Shopify does not call React Native a failure; its argument is that coding agents reduced the economic advantage of one shared UI implementation. It maintains parity through shared specifications, tests, review checkpoints and release gates. Its new architectural principle is that business logic must be decoupled from UI, runnable headlessly on desktop and operable through a CLI.[1][2]

**Spotify** is a different precedent: independently released platform clients that share selected libraries.[3] Historical evidence documents a native C++ core, a shared sequencing module, and substantial shared-client logic for Liked Songs, but does not support a Spotify React Native exit.[4][5][7]

For VIDA, the evidence supports **Headless Shared Core + Native/Platform Shells + Conformance Plane**. This is more precise than saying that everything outside the domain is a “replaceable adapter.” The Rust core should be genuinely shared executable code. Iroh core is a strategic dependency within the shared runtime. Native UI, OS lifecycle and device integrations remain platform-owned. Replaceability applies only to explicitly defined provider seams—storage engines, durable-delivery topology, Address Lookup providers and optional pre-1.0 Iroh higher-level libraries—and only behind compatibility tests.

The largest caveat is that Shopify has not publicly specified the implementation language or binary boundary of its headless business-logic layer. The VIDA Rust-core recommendation therefore combines Shopify’s agent-addressable architecture with 1Password’s deployed Rust-core pattern, Mozilla’s documented Rust packaging and proposed orchestration split, and VIDA’s existing Iroh/Rust direction.[8][10][11]

## Proposed Architecture Spine

> **Headless local-first shared core with native/platform shells and ports-and-adapters at external seams.**

Governing rules:

1. `vida-core` is the single normative executable implementation of cross-platform domain and protocol semantics.
2. `vida-runtime` is the shared Rust host for Iroh, delivery, sync, blob and storage coordination.
3. Iroh core is strategic and mandatory; only its integration version and optional higher-level adapters are replaceable.
4. Native/platform shells own UI and OS policy and may release independently.
5. Language-neutral specifications, golden vectors, replay fixtures and parity tests are authoritative across all implementations.
6. Storage/delivery/discovery providers are replaceable only through explicit ports, migrations and conformance gates.
7. The headless core and runtime expose a CLI/test harness suitable for developers and coding agents.

**Confidence:** high for the Shopify/Spotify correction and the boundary pattern; medium-high for applying a shared Rust kernel to VIDA, pending successful FFI, browser/WASM, and storage prototypes.

## 1. What actually happened

### Shopify: React Native to native

Shopify is migrating its mobile apps from React Native to native Swift and Kotlin; the Shop app specifically was rebuilt with SwiftUI and Jetpack Compose. Shopify says React Native delivered its intended benefits and could be fast; the changed variable was the cost of maintaining two implementations once agents could translate, test and review them. For the Shop migration, feature parity was enforced through shared behavior specifications, tests, analytics checkpoints, visual comparison and release process.[1][2]

Shopify’s agent-addressable architecture has three relevant properties:

1. business logic is completely decoupled from UI;
2. that logic can run headlessly on desktop;
3. a CLI exposes state and actions so humans and agents can test without a simulator.[1]

This is not evidence that Shopify retained one shared executable business-logic core. The public text explicitly emphasizes two native implementations plus shared specifications and verification artifacts.

### Spotify: native clients with a selected shared core

Spotify’s current public release model treats Android, iOS and Desktop as independent release tracks that share some common libraries.[3] Historical evidence identifies a native C++ core shared across most platforms and a shared sequencing module with C++ tests.[5][7] Separately, Spotify Wrapped used native Android and iOS animation implementations.[6]

Spotify’s Liked Songs modernization changed about 100,000 lines in a shared client codebase and also required Android and iOS changes. Shared code owned metadata batching, on-device persistence/sorting and checks for newly liked tracks on Spotify’s servers; the clients integrated the behavior and UI.[4] This provides strong precedent for the proposed VIDA boundary, but it is not a React Native migration.

The older Android boundary connected a Java application layer to the native C++ core through JNI, with documented performance and debugging implications.[7] For VIDA, this evidence motivates treating FFI, cancellation, memory ownership, tracing, crash symbolication and version skew as first-class design risks; those additional risks are VIDA inferences, not claims made by the source.

## 2. Comparable shared-core/native-shell architectures

1Password consolidated server communication, database handling, permissions, cryptography, search and other feasible non-UI behavior into a shared Rust Core, stopping short of UI. Android and iOS use platform-native frontends, while other platforms may choose different shells.[8] 1Password also uses generated types between its Rust core and frontends and compiles Rust to WASM for browser use.[9]

Mozilla’s Sync Manager design proposes reusable Rust components below platform-specific Swift/Kotlin orchestration, with the platform layer owning embedding-app policy.[10] Separately, Mozilla documents shipping compiled Rust to iOS through XCFramework/Swift Package artifacts and generated bindings.[11]

These examples confirm that VIDA can share a substantial Rust implementation without forcing one cross-platform UI runtime.

## 3. Recommended VIDA boundary

### Layer A — Conformance Plane: normative and language-neutral

This is the normative source of cross-platform consistency:

- schemas, IDs, operation envelopes and protocol versions;
- authorization, sync and migration specifications;
- golden wire vectors, deterministic replay fixtures and negative security cases;
- parity scenarios for Messenger, Notes, Projects and system Apps;
- visual/accessibility expectations where platform behavior must match;
- independently versioned client compatibility matrix.

Native clients and the shared core both conform to this layer. It survives replacement of any implementation.

### Layer B — `vida-core`: shared headless Rust product kernel

This layer should run without a UI, simulator, network connection, or specific database:

- domain types, invariants and commands;
- deterministic state transitions and conflict rules;
- Space membership/capability evaluation;
- signed operation construction and validation;
- causal/frontier, snapshot and migration semantics;
- encryption/key-envelope logic and canonical serialization;
- projection rules that must behave identically everywhere;
- test clock, deterministic randomness and replay harness;
- headless API/CLI for humans, tests and agents.

It decides **what the product means**, not how an OS presents or schedules it.

### Layer C — `vida-runtime`: shared Rust infrastructure runtime

This layer is shared where technically viable and accesses external capabilities through explicit ports:

- Iroh 1.2 Router, Endpoint and VIDA ALPN implementations;
- direct/durable delivery orchestration;
- sync-log and blob transfer engines;
- storage transactions, migrations and recovery coordination;
- telemetry events and typed failure mapping;
- platform-neutral background job graph.

Iroh core is **not an interchangeable product plugin**. It is the adopted transport foundation of `vida-runtime`. The replaceable elements are the integration layer used across version changes and optional higher-level libraries such as `iroh-blobs`, `iroh-gossip` and `iroh-docs`.

### Layer D — native/platform shells

Swift/SwiftUI, Kotlin/Compose, desktop and browser shells own:

- UI, navigation, accessibility, animation and native design language;
- OS lifecycle, background execution, notifications and deep links;
- permissions and privacy UX;
- Keychain/Keystore handles and platform secure-storage integration;
- network reachability, push wake-up and energy policy;
- filesystem/database primitives exposed through runtime ports;
- App Store packaging, platform diagnostics and release rollout.

The shell decides **how the platform performs or presents an action**. It must not reimplement domain invariants already owned by `vida-core`.

### Explicit replaceable seams

| Seam | Stable part | Replaceable part |
|---|---|---|
| Iroh | VIDA ALPNs, operation semantics, endpoint profiles | pinned Iroh integration version and optional higher-level adapters |
| Storage | transaction, migration, snapshot, encryption and recovery contracts | SQLite/RocksDB/native database or service-side engine |
| Durable delivery | envelope, ACK, deduplication, retry and privacy semantics | managed, federated or self-hosted topology |
| Discovery | endpoint/address record and trust rules | Address Lookup provider order and caches |
| Platform | headless commands/events and conformance fixtures | Swift, Kotlin, desktop and browser shell implementation |

Replacement is allowed only when the candidate passes the same fixtures and migration tests. “Adapter” is therefore a controlled compatibility boundary, not permission to swap foundational technology casually.

## 4. Development and migration model

Shopify’s transferable lesson is not merely “use native.” Its published process supports headless CLI access, small ordered checkpoints, behavior tests, visual review, adversarial review, human approval and parity review.[1][2] The following is VIDA’s synthesis of those practices with its own protocol, accessibility and release requirements:

1. write an approved behavior/specification slice;
2. record approval against an immutable specification revision;
3. implement the headless command/state path first;
4. run deterministic core and golden-vector tests;
5. implement each platform shell against the same slice;
6. run behavioral, analytics, accessibility and visual parity checkpoints;
7. use independent adversarial reviews and human acceptance;
8. release behind flags with staged cohorts and rollback evidence.

Spotify’s migrations add parallel or side-by-side old/new paths, phased cohorts and crash/performance monitoring while the prior path remains temporarily available.[4][12] VIDA should extend this precedent with its own artifact and behavior-differential checks.

## 5. Contrary evidence, risks, and unresolved decisions

- Shopify’s decision is not proof that React Native is generally inferior. Shopify explicitly says its React Native investment was successful; its new conclusion depends on its team, apps and 2026 agent capabilities.[1]
- Spotify’s detailed C++/JNI evidence is partly historical. Current sources confirm shared libraries and independent releases, but not the complete 2026 boundary.[3][7]
- Spotify’s historical JNI experience demonstrates performance/debugging costs, while Mozilla’s packaging work demonstrates the need for XCFramework, C-header and generated-binding infrastructure.[7][11] VIDA should additionally test symbolication, threading, cancellation and independent-release compatibility as project-specific risks.
- Maximizing shared code is not the goal. UI, permissions, lifecycle and OS integrations should remain native even when duplicating some orchestration produces a better platform fit.
- Do not assume that the browser can use the full native runtime. WASM coverage and relay-only limitations require a separate conformance profile.

### Unresolved decisions

1. Which binding strategy becomes canonical: UniFFI, a narrow C ABI with generated Swift/Kotlin wrappers, or another IDL?
2. Does `vida-core` own only storage semantics, or also a portable embedded storage implementation?
3. Which `vida-runtime` capabilities compile to browser WASM, and which require a companion/gateway?
4. Does desktop use native shells, web shells, or platform-dependent choices above the same core?
5. Which behavior belongs in shared executable code versus shared specification only?
6. What ABI/schema compatibility window permits independently released client versions?

## Source appendix

| Ref | Claim/finding supported | Publisher | Published | Accessed | Confidence |
|---:|---|---|---|---|---|
| [1] | Shopify native decision, rationale, headless logic/CLI, shared specs/tests/reviews | [Shopify Engineering](https://shopify.engineering/back-to-native) | 2026-09-10 | 2026-09-19 | high |
| [2] | Shop migration mechanics, native parity, Tardis/checkpoint workflow | [Shopify Engineering](https://shopify.engineering/shop-app-migration) | 2026-09-10 | 2026-09-19 | high |
| [3] | Spotify shared libraries with independent Android/iOS/Desktop releases | [Spotify Engineering](https://engineering.atspotify.com/2026/2/how-we-release-the-spotify-app-part-2) | 2026-02-09 | 2026-09-19 | high |
| [4] | Spotify shared-client Liked Songs architecture and staged migration | [Spotify Engineering](https://engineering.atspotify.com/2020/5/spotify-modernizes-client-side-architecture-to-accelerate-service-on-all-devices) | 2020-05-28 | 2026-09-19 | high |
| [5] | Shared Spotify sequencing module and C++ property-based test code | [Spotify Engineering](https://engineering.atspotify.com/2015/6/rapid-check) | 2015-06-25 | 2026-09-19 | high historical |
| [6] | Native Android/iOS animation implementations for 2023 Wrapped | [Spotify Engineering](https://engineering.atspotify.com/2024/1/exploring-the-animation-landscape-of-2023-wrapped) | 2024-01-24 | 2026-09-19 | high scoped |
| [7] | Spotify Android/C++ core and JNI boundary | [Linköping University / Spotify-supervised thesis](https://liu.diva-portal.org/smash/get/diva2:910711/FULLTEXT01.pdf) | 2016-03-08 | 2026-09-19 | high historical |
| [8] | Shared Rust Core stopping short of UI | [1Password](https://1password.com/blog/1password-8-the-story-so-far) | 2021-08-12 | 2026-09-19 | high historical |
| [9] | Generated Rust/frontend types and WASM core use | [1Password](https://1password.com/blog/passkey-crates) | 2023 | 2026-09-19 | high |
| [10] | Proposed shared Rust components below platform orchestration | [Mozilla Application Services](https://mozilla.github.io/application-services/book/design/sync-manager.html) | n.d. | 2026-09-19 | medium-high |
| [11] | Rust XCFramework/Swift Package and generated bindings | [Mozilla Application Services](https://mozilla.github.io/application-services/book/design/swift-package-manager.html) | n.d. | 2026-09-19 | medium-high |
| [12] | Spotify side-by-side build migration, phased rollout and telemetry monitoring | [Spotify Engineering](https://engineering.atspotify.com/2023/10/switching-build-systems-seamlessly) | 2023-10-17 | 2026-09-19 | high analogous |

## Staleness map

| Claim class | Re-check |
|---|---|
| Shopify 2026 direction and migration state | 2027-09-10 |
| Current Spotify release boundary | 2028-02-09 |
| Spotify C++ scope | historical; current scope remains partially unverified |
| 1Password/Mozilla comparable implementation details | verify again before selecting the VIDA binding toolchain |

Earliest scheduled refresh: **2027-09-10**. Historical sources remain valid as precedents but must not be presented as complete current architectures.
