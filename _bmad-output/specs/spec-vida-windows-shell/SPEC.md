---
id: SPEC-vida-windows-shell
status: draft
companions:
  - windows-shell-conformance-cases.md
  - ../../../docs/02-requirements/native-client-requirements.md
  - ../../../docs/02-requirements/platform-nfr.md
  - ../../../docs/03-architecture/decisions/ADR-0020-flutter-windows-in-release-1.md
  - ../spec-vida-platform-bindings/SPEC.md
sources: []
---

> **Release-1 acceptance kernel.** This is an unexecuted Windows-shell test contract distilled from approved requirements, not a change to the Flutter decision or a claim that the client passes.

# VIDA Windows shell acceptance

## Why

Windows is a full installed VIDA client in Release 1. Equivalent Rust domain results alone do not prove that a person can operate Messenger, Notes/Knowledge and Projects/Tasks through Windows keyboard, system menu/tray and screen reader. This mandate needs its own observable client gate.

## Capabilities

- **CAP-1**
  - **intent:** Windows users can complete the three base-App journeys available on the other installed clients.
  - **success:** An installed Windows release build passes Messenger, Notes/Knowledge and Projects/Tasks flows, including files, forums, calls, offline action, restart, reconnect and visible sync/conflict state, without a companion-only feature subset.
- **CAP-2**
  - **intent:** A person can navigate and complete those journeys using only a keyboard.
  - **success:** Focus, command, dialog and recovery fixtures complete without a pointer-only action, invisible focus or keyboard trap.
- **CAP-3**
  - **intent:** Windows system menu and tray actions give useful, truthful access to the installed client.
  - **success:** Every exposed menu/tray action executes its documented client behavior; opening, hiding, resuming or closing the shell neither loses committed work nor reports an unverified sync state as complete.
- **CAP-4**
  - **intent:** A screen-reader user can understand and operate the three Apps and status/error flows.
  - **success:** Installed-build inspection finds usable names, roles, state and reading/focus order for primary controls, pending/synced/conflict messages and failures.

## Constraints

- Release 1 uses installed Flutter Windows with the shared Rust core; WinUI 3/C# remains a post-Release-1 possibility and Tauri is not the selected Release-1 shell.
- Windows must pass the same product/domain conformance as Android and iOS, plus its own keyboard/menu/tray/screen-reader checks. A framework demo or headless binding test is not Windows acceptance evidence.
- The shell presents state and dispatches intent; it does not duplicate domain authorization, acceptance or conflict resolution owned by Rust/Space authority. Storage-provider mechanics remain outside this shell spec.
- Exact menu layout, shortcut map, supported assistive-technology/version matrix and numerical performance budgets are not selected by this spec.

## Non-goals

- Designing a new Windows UI, choosing shortcuts or selecting a screen-reader vendor/version here.
- Replacing the platform-binding contract, three-App domain specs or approved Windows release decision.
- Building a WinUI client or browser companion for Release 1.

## Success signal

One traceable installed Flutter Windows release artifact passes `WIN-F01–F08` alongside the three-App and binding suites; its Windows build, OS and accessibility-test matrix are recorded. The fixtures are plans until executed on that artifact.

## Open Questions

- Which Windows versions, screen readers and input configurations will the release conformance matrix support and test?
- Which concrete menu/tray actions and shortcut assignments will the UX/implementation plan expose?
