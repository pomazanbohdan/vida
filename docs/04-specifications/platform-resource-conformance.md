---
id: SPEC-PLATFORM-RESOURCE-CONFORMANCE-001
status: approved-method
implementation_status: baseline-required
last_updated: 2026-09-25
requirement_refs:
  - ../02-requirements/platform-nfr.md
fixture_ref:
  - fixtures/platform-resource-v1.yaml
---

# Platform resource and performance conformance

## Decision

OQ-4 is split into two gates. The native Android/iOS/Windows G0 method is approved; adding Release-1 Web requires a browser-specific G0 extension before Web implementation fan-out. The supported-browser/OS/device matrix remains OQ-0075, so this file does not claim that Web G0 has passed. Numeric product budgets remain `TBD-from-baseline` until representative builds are measured. Store ceilings are separate compliance limits, never VIDA targets.

## Scope

The contract covers release/profile builds of the Flutter/Rust Android, iOS and Windows clients and static Flutter Web/Rust-Wasm client with Messenger, Notes/Knowledge, Project, Files, Contacts and Calls enabled. Web has its own browser profiles; it cannot inherit native measurements. Hosted services require a separate later profile.

## Required profiles and workloads

- Devices: Android low/mid/high; iPhone oldest-supported/mid/current; Windows x64 minimum/mid/high; Web on the supported browser/OS/device matrix selected by OQ-0075. Every run records model, OS/build, CPU, RAM, free storage, power mode, thermal/charging state where available, build hash and tool versions; Web additionally records browser version, storage persistence/quota and service-worker/cache state.
- Networks: offline→reconnect, LAN/direct for native and separately proven Web peers, broadband/direct and relay fallback, cellular, Wi-Fi↔cellular, constrained loss/latency/jitter and UDP-blocked fallback. Web direct sync needs its own browser-compatible transport evidence; do not report native direct-path results as Web evidence. Verify no relay data forwarding on the direct test, encrypted forwarding on fallback, and no duplicate apply after route migration.
- Data: empty, representative and large Spaces from versioned deterministic generators.
- Journeys: cold/warm startup to shell and meaningful interaction; search; large rich document; Project board; local commit/sync/recovery; file transfer/low disk; 1:1 call; approved 8-participant call; baseline and opt-in high-availability background runs.

## Measurement rules

1. Use physical devices, release/profile builds and fixed datasets. Web uses the final static production build in supported browsers on representative physical devices; debug builds are diagnostic only.
2. Reset or record cache, process, network, power and thermal state; separate cold and warm runs.
3. Run the fixture-defined repetitions; publish raw traces and distributions, not a single best result.
4. Record p50/p95 and tail samples where the platform tool supports them. A missing measurement is `not-measured`, never zero or pass.
5. Calls record join/reconnect duration, RTT, jitter, packet loss/discard, bitrate, frames/fps/drops/freezes, audio concealment and quality-limitation reason together with CPU, memory, network and power.
6. Results must be reproducible from pinned source, lockfiles, build artifacts, dataset and runner configuration. Payload, keys and stable identity must not enter traces.

## Platform toolchain

- Flutter: DevTools Performance/Memory, profile mode, `integration_test`, `--analyze-size` and store-processed delivered size.
- Android: Macrobenchmark Startup/Frame/Memory metrics, Perfetto/System Trace and Android vitals. PowerMetric is supporting system evidence only, not exact per-app truth.
- iOS: XCTest performance metrics in Release configuration, Instruments, Organizer/MetricKit (or current successor APIs), and physical-device Power Profiler with charging disabled.
- Windows: Flutter DevTools, WPR/WPA and Visual Studio CPU/Memory profiling on Release builds.
- Web: Flutter DevTools plus supported-browser performance, memory, network, storage/quota and accessibility traces on the final static build; distinguish initial download from cached reload and measure offline cold reopen separately.
- Calls: W3C WebRTC stats or a semantically equivalent adapter export, correlated per endpoint by opaque call/run ID.

## Gate sequence

| Gate | Required evidence | Result |
|---|---|---|
| G0 method freeze | This spec + valid fixture + pinned runner schema; Web additionally needs its approved supported-browser matrix and browser runner | closes the method/profile blocker separately per platform |
| G1 baseline | Complete reproducible raw runs for all required profiles; no invented thresholds | permits numeric target proposal |
| G2 budget freeze | Approved absolute budgets, regression tolerances and documented exceptions derived from G1 | required before feature-complete |
| G3 release regression | CI/lab comparison against G2 plus current store/vitals compliance | pass/fail release evidence |

A failed or unavailable profile cannot be averaged away. Thresholds may differ by platform/device class, but all supported profiles need an explicit budget or approved exclusion. Any change to runtime, media stack, persistence format or bundled feature set that invalidates a baseline triggers re-baselining.

## Official method references

- [Flutter Performance view](https://docs.flutter.dev/tools/devtools/performance), [Memory view](https://docs.flutter.dev/tools/devtools/memory), [app-size tooling](https://docs.flutter.dev/perf/app-size)
- [Android Macrobenchmark](https://developer.android.com/topic/performance/benchmarking/macrobenchmark-overview), [metrics](https://developer.android.com/topic/performance/benchmarking/macrobenchmark-metrics), [Android vitals](https://developer.android.com/topic/performance/vitals)
- [Apple performance tests](https://developer.apple.com/documentation/xcode/writing-and-running-performance-tests), [shipping-app metrics](https://developer.apple.com/documentation/xcode/analyzing-the-performance-of-your-shipping-app), [Power Profiler](https://developer.apple.com/documentation/xcode/measuring-your-app-s-power-use-with-power-profiler)
- [Windows performance planning](https://learn.microsoft.com/en-us/windows/apps/develop/performance/planning-measuring-performance), [profiling tools](https://learn.microsoft.com/en-us/windows/apps/develop/performance/profiling-tools)
- [W3C WebRTC Stats](https://www.w3.org/TR/webrtc-stats/)
