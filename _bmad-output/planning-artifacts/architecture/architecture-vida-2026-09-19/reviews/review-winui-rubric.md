# Independent rubric review — WinUI 3/C# deferred Windows candidate

## Gate verdict

**Pass.** WinUI 3/C# appears only as an explicitly deferred Windows shell candidate, not an adopted technology or a change to the shared Rust core. `OQ-0008` gates the Windows shell and supported OS matrix; `OQ-0035` gates the binding contract. The comparison documents are clearly drafts with a common release-build vertical slice, not permission to select a framework by preference alone.

## Critical findings

None.

## High findings

None.

## Medium findings

### M1 — Add candidate-document traceability to the spine

- **Evidence:** spine Deferred names “Flutter Windows versus WinUI 3/C#, and Tauri only if WebView is allowed under `OQ-0008`” (line 297), but the spine frontmatter sources/companions do not point to `docs/03-architecture/windows-client-options.md` or `stack-selection-brief.md`. Those documents contain the comparison, evidence limitations and common gate.
- **Disposition:** **Autofix, non-blocking.** Add the Windows options discussion as a companion/source so future reviewers can see the candidate rationale without mistaking the Deferred line for a selected stack. Do not promote a draft to normative status.

### M2 — “Native” remains a user decision, correctly bounded

- **Evidence:** AD-20 says the supported OS targets and whether Tauri WebView qualifies remain `OQ-0008`; neither shell nor production OS matrix is locked. `windows-client-options.md` distinguishes installed app from Windows XAML native controls and notes C# is managed .NET, not Rust/C++ native code.
- **Disposition:** **Safely deferred.** WinUI 3 is a plausible candidate for native Windows UI, but this review does not infer that all three candidates satisfy the user's precise meaning of “native”. The current gate prevents a premature Tauri/Flutter/WinUI commitment.

### M3 — C#↔Rust bridge and AppPackage renderer are not silently assumed

- **Evidence:** `windows-client-options.md` labels `WinUI/XAML → C# adapter → C ABI → Rust vida-sdk` as a *prototype path*, identifies ABI, ownership, async/cancellation, threading, x64/ARM64 and versioning proof under `OQ-0035`, and requires the same AppPackage semantics. AD-11 keeps domain/authorization/sync in first-party Rust core/runtime; the spine blocks production bindings until `OQ-0035`.
- **Disposition:** **Pass.** The candidate does not create a second C# domain engine, a C# Iroh implementation, arbitrary executable packages, or direct shell access to core/runtime internals. The production binding remains gated.

### M4 — Windows choice still needs one measured release target

- **Evidence:** the Windows options and stack brief propose comparing Flutter Windows, WinUI 3/C#, and conditional Tauri on the same core, data, fixtures, offline/restart, package renderer, rich editor/grid, accessibility, lifecycle and security cases. They recommend one release shell but do not claim measured results.
- **Disposition:** **Pass.** This is appropriate discussion material. No quantitative winner or stack lock has leaked into AD-20, the Stack table, or an accepted ADR.

## Good-spine checklist

| Criterion | Result | Note |
|---|---|---|
| Fixes cross-team divergence | **Pass** | Rust core/runtime, single SDK facade and installed-client boundary remain invariant; Windows selection has a named decision gate. |
| Rules enforceable | **Pass** | AD-11/20 and `OQ-0035` prevent C# semantic forks and unsupported production binding claims. |
| Deferred cannot silently diverge | **Pass** | WinUI/Flutter/Tauri are explicitly candidates; `OQ-0008` blocks shell/OS lock and `OQ-0035` blocks binding. |
| Ratifies prior decisions | **Pass** | ADR-0014, open-protocol independent clients, native-only first-party distribution and offline base Apps remain intact. |
| Named tech verified-current | **Not binding yet** | Official WinUI/.NET/Rust references support candidate feasibility; exact Windows App SDK/runtime version should be pinned only after selection. |
| Input coverage | **Pass** | Candidate rationale, risks and common prototype gate are in the two draft discussion documents. |
| Operational/platform envelope | **Pass for this update** | Packaging/updates, process death, local one-writer, vault recovery, platform a11y/security are part of the gate. |

## Input reconciliation

- User's WinUI 3/C# suggestion is **considered**, not approved as implementation.
- Windows XAML UI could satisfy a stricter no-WebView reading; managed C# does not change Rust ownership of product semantics.
- Flutter Windows remains a candidate for mobile UI reuse; Tauri remains conditional on the user allowing WebView as an installed Windows shell.
- The first-party VIDA browser/PWA client is still excluded; a City Portal public web or independent protocol implementation is not a first-party VIDA release target.

## Handoff

No semantic blocker. Optionally add the two discussion docs to spine traceability. Keep the Windows shell and OS matrix undecided until `OQ-0008`, and keep the C#↔Rust ABI a prototype until `OQ-0035`.
