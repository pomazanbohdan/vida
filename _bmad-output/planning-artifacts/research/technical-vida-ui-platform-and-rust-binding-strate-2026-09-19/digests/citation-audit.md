# Independent citation audit — 2026-09-19

## Verdict

The decision-critical technical claims sampled in `research.md` are supported after narrow citation and provenance corrections. The report properly keeps Flutter mobile + Rust + Tauri Windows provisional and requires a Flutter-vs-Tauri Windows bake-off. The matrix and proposed facade are architectural inferences, not measurements or vendor promises. `claims.json` records **42 checked claims**: 24 external and 18 internal/corpus claims. The latter preserve the distinction between approved canonical requirements, accepted working principles, user intent, proposals, historical snapshots and explicit retractions.

## Corrections made

1. Flutter's old `/platform-integration/android/c-interop` URL now redirects to a **legacy `plugin_ffi` page**; current Flutter guidance recommends `package_ffi` with build hooks. Replaced decision-critical citations with the [current FFI guide](https://docs.flutter.dev/platform-integration/bind-native-code). This does not settle FRB versus manual FFI.
2. The historical [Iroh 0.32 browser alpha post](https://www.iroh.computer/blog/iroh-0-32-0-browser-alpha-qad-and-n0-future) alone is insufficient to assert current browser behavior. The [current first-party browser/Wasm example](https://github.com/n0-computer/iroh-esp32-examples/blob/main/wasm-gui/README.md) explicitly uses relay-only paths because raw UDP/QUIC sockets are unavailable. Report language now limits the assertion to that example and acknowledges possible future alternative transport.
3. The old `rustwasm.github.io/docs/wasm-bindgen` domain says its guide is no longer maintained. Replaced citation targets with the [maintained wasm-bindgen guide](https://wasm-bindgen.github.io/wasm-bindgen/reference/rust-targets.html), which supports the `std::fs`/`std::net` limitation.
4. The Kotlin Multiplatform candidate row now cites the [current supported-platform table](https://kotlinlang.org/docs/multiplatform/supported-platforms.html) for Web Beta instead of treating an older roadmap as current state.
5. The business/city row now attributes centralized SaaS/GEO intent to the user's statement at `(2).txt:46`, while multi-entry IDs, public website and placements are labeled derived proposals from later assistant text. This avoids turning a proposal into user approval.

## Incremental raw-corpus and package-feasibility audit

- Recounted `research/`: 18 text/Markdown files totaling **846,433 bytes**, plus one PNG excluded from the text audit. The [corpus digest](research-corpus-cases.md) identifies 15 additional/sharper case groups and source locations. The report's status labels match the source passages sampled across all 15 groups.
- Verified explicit reversals: Dioxus is retracted as an inherited UI choice; Chatmail/Delta Chat core adoption is retracted after the user rejects email. Older .NET/Blazor/MAUI/sidecar descriptions and a Flutter visual prototype are historical/provisional, not shell decisions.
- Verified case provenance: portable provider-neutral packages and capability rechecks appear as accepted *working principles* in `vida-current-architecture.md`; package lifecycle, declarative presentation split, node profiles, browser alternatives, reminder executors and Windows process topology remain proposals/open decisions. The 50k group and latency figures are scenarios/targets, not measured capacity. Repository-index projects are inspection leads, not endorsements.
- Verified the new executable-package caveat against first-party [Flutter package guidance](https://docs.flutter.dev/packages-and-plugins/using-packages) and [deferred-component guidance](https://docs.flutter.dev/perf/deferred-components): platform plugin code is built into the app; deferred Android/web imports are predeclared; adding Android code requires rebuilding/uploading the whole AAB. A downloadable declarative VIDA package is therefore a separate feasibility target from arbitrary runtime code plugins.

## Mechanical link check

- Local Markdown links: 15 unique; all 15 resolve to existing files from the research folder. `../../../../docs/...` correctly reaches `C:\project\vida\docs\...`; changing it to three parent traversals would break it.
- External Markdown links after integration: 62 unique; HTTP HEAD returned 200 for 61. The JetBrains Compose article returned 403 to HEAD, but its content was available in browser search; this is **not** evidence of a broken link. The check covers reachability, not factual support.
- Semantic/source-to-claim review: 42 claims in `claims.json`; no unsupported architecture approval or quantitative VIDA SLO was found.

## Remaining evidence limits

- The current Iroh example proves that example's relay-only browser path, not a universal impossibility of every future browser transport. Browser transport/version and egress require a VIDA prototype.
- Flutter support listings and FRB registry versions change; recheck the pinned tuple at implementation time.
- Vendor APIs demonstrate feasibility, not VIDA security, accessibility, mobile background delivery, Windows effort/performance or City Portal booking correctness.
- Apple documentation was validated through the publisher's indexed text because the page's normal web view requires JavaScript; the indexed article explicitly says delivery is not guaranteed and can be throttled.
- Windows Flutter-versus-Tauri ranking remains a provisional research judgment pending identical release-build tests and predeclared thresholds.
