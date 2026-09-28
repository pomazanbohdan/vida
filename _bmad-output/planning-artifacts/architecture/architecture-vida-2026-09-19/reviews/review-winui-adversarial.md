---
review: winui-adversarial
artifact: ../ARCHITECTURE-SPINE.md
companion: ../../../../../docs/03-architecture/windows-client-options.md
focus: Flutter mobile versus WinUI Windows shell seam
date: 2026-09-20
verdict: architecture-consistent-with-two-unowned-acceptance-details
---

# Adversarial seam review — Flutter mobile and WinUI Windows

## Verdict

The WinUI 3 + C# proposal is correctly a **candidate**, not a selected stack. The shared Rust `vida-core`/`vida-runtime`/`vida-sdk` rules block a second C# domain engine; `OQ-0008` blocks the Windows-shell choice and `OQ-0035` blocks production FFI. Both documents preserve installed-only VIDA clients and do not mistake WinUI, Tauri, or Wasm plugin execution for a browser VIDA client. Two acceptance details still need explicit owners before comparing or releasing two shells: canonical `AppPackage` UI-renderer behavior, and a named cross-OS minimum offline-action set. They are not solved merely by choosing WinUI or defining C ABI calls.

## High findings

### H1 — Same package/domain outcomes can still produce incompatible renderer behavior

**Evidence:** ADR-0007 and `REQ-APP-005` require the same package/instance semantics and authorized outcomes; AD-2/AD-12 require versioned conformance and observable contracts. `windows-client-options.md` says WinUI must interpret the same schemas/UI ports/capability manifest, but exact UI-port schema, fallback rules, validation timing, rich-document representation, accessibility semantics and renderer feature negotiation are not yet normative. `OQ-0008` chooses supported shell/OS; `OQ-0035` chooses binding behavior; neither fully owns UI-port semantics. `OQ-0040`/`OQ-0043` concern package compatibility/manifest and could own them if expanded explicitly.

**Two-compliant-team construction:** one external package declares a task form with a relation field, conditional required rule and rich Notes block. Flutter mobile renders a relation picker, validates before issuing the command, and exposes the block to screen readers. WinUI uses a plain text field and `RichEditBox`, sends a different normalized value through the same Rust SDK, and hides the unsupported block while still considering the package installed. Rust authorization may reject the bad command, so neither team has duplicated domain authority, yet the package is not functionally portable. Another mismatch is a field omitted from visual UI but present in keyboard/accessibility or search surfaces.

**Required closure:** assign one open question/contract to versioned UI-port and capability semantics: required versus optional component behavior, value normalization, relation/query and editor model, validation/error presentation, fallback/unsupported state, privacy visibility, accessibility and per-platform golden interaction fixtures. Run the **same external package** on Flutter and WinUI release builds. Unknown mandatory declarations must fail before activation, never degrade silently. This is a cross-shell AppPackage contract, not a choice of UI toolkit.

### H2 — “Offline local action” permits incomparable three-App slices unless a minimum action matrix is fixed

**Evidence:** AD-20, `REQ-CLIENT-002`–`005` and `NFR-PLAT-003` require Messenger, Knowledge/Notes and Projects/Tasks to work offline; `native-client-requirements.md` explicitly leaves the list of offline commands open. `windows-client-options.md` proposes one vertical slice but names only categories “offline read/local action” and richer UX examples. Without a shared minimum operation list and fixtures, a mobile team can demonstrate rich edit/task transitions while Windows demonstrates only draft creation; each honestly claims some permitted local action.

**Two-compliant-team construction:** Flutter lets the user edit an existing note and change a task status offline. WinUI permits composing a new chat and creating a new empty note/task draft but disables editing existing resources until connected, citing lack of editor/grid support. Both keep pending operations in Rust and pass a loosely worded one-action-per-App test, but users experience different product capabilities.

**Required closure:** before the Windows bake-off/release claim, name v1 minimum offline commands and local-read states per App, shared authorization/pending/conflict UX, and expected result for unavailable/not-yet-cached resources. Bind a common scenario manifest to every supported installed OS. `OQ-0008` can own the platform commitment, but the product command matrix should be a normative requirement, not a UI-spike preference.

## Gated implementation risks, not contradictions

### G1 — Flutter FRB and WinUI C ABI can diverge at the binding boundary (`OQ-0035`)

One team may map an ID/time/decimal/error as Dart-owned bytes and the other as UTF-16/.NET-owned memory; callback-after-disposal, cancellation races, reentrant events and x64/ARM64 ABI differences can yield different commands or crashes. AD-12 requires versioned DTO and normalization rules; AD-11 requires the same SDK facade; `OQ-0035` correctly blocks production until ownership, threading, lifetime, cancellation, error mapping and supported version window are specified. The WinUI document labels C ABI + P/Invoke a **prototype path**, not an accepted bridge. Test bidirectional values, unknown fields, concurrent callback/cancel, process exit and mixed-version artifacts. Do not treat successful compilation as conformance.

### G2 — Presentation-side caches can undermine revocation even if Rust core rechecks commands

Flutter and WinUI may both call the Rust authorization engine for writes, yet one shell may retain plaintext search results, preview thumbnails, OS notification text, navigation recents or clipboard data after effective revocation. AD-16, `NFR-SEC-005` and `REQ-CLIENT-005` already prohibit managed-surface bypass and require cross-platform evidence; ADR-0003 acknowledges unavoidable OS banners and copies. The Windows vertical slice should explicitly include C#/XAML view-model caches, Windows Search/notification integration and cross-Persona switch, not only “attacker package cannot bypass Rust facade.” This is a test-coverage tightening, not a new architecture decision.

### G3 — Offline durability must stay in Rust runtime/storage, not C# or Dart queues

A WinUI team might cache an unsent edit in a C# view-model and recreate it after restart, while Flutter commits to Rust outbox. The former would violate AD-4/AD-11/AD-13 if it is the sole durable copy. Existing NFR requires process-death, backup/restore and reconnect tests. The candidate comparison must inject a kill before/after SDK acceptance and prove the same origin-durable frontier, dedupe key and recovery result on both platforms. No gap in the AD rule; `OQ-0036` still owns provider transaction/recovery details.

### G4 — One Rust core does not imply one identical OS lifecycle

Windows sleep/activation, mobile suspend/process death, secure storage and notifications differ. AD-9 and `NFR-PLAT-001`–`005` correctly require separate results. The option document lists single-instance/one-writer and signed update/rollback as spike items. `OQ-0008` must select OS/architecture/package targets and thresholds; `OQ-0035` must select binding support. Neither should be silently inferred from WinUI's native-XAML capability.

## Non-findings and boundaries preserved

- WinUI 3 + C# does **not** require C# Iroh or domain logic; AD-1/AD-11 place them in Rust runtime/core.
- A Windows-native XAML UI is an eligible **candidate**, not an approved Windows release stack. Flutter Windows and conditional Tauri remain comparison options under `OQ-0008`.
- A WinUI client is an installed application, not a VIDA browser client under AD-20. A managed Wasm App executor, if later chosen, is also not a browser client.
- A different presentation framework is allowed if it produces the same protected domain outcomes and meets platform-specific UX/accessibility/security evidence. “Same core” does not demand pixel-identical UI.
- The current stack-selection brief has been updated to include WinUI and a three-candidate comparison; no material cross-document selection mismatch was found.

## Closure order

1. Keep `windows-client-options.md` and stack brief as discussion documents until `OQ-0008` selects the supported OS matrix and one Windows shell.
2. Give H1's renderer contract and H2's offline command matrix explicit normative owners and fixtures; include them in the common candidate bake-off.
3. Close `OQ-0035` for Flutter and WinUI bindings independently against one public SDK semantic contract, then run the same package/offline/security scenarios on release builds.
