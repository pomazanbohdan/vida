---
id: SPEC-VIDA-ACCESSIBILITY
status: draft
companions:
  - accessibility-conformance-cases.md
  - ../../../docs/02-requirements/native-client-requirements.md
  - ../../../docs/02-requirements/platform-nfr.md
  - ../spec-vida-windows-shell/SPEC.md
sources: []
---

> **Decision-gated draft.** Windows keyboard/screen-reader acceptance is approved. The broader Android/iOS floor below is distilled from draft PRD and discovery UX, not yet an approved conformance level. All cases are unexecuted.

# VIDA Release-1 accessibility

## Why

VIDA Release 1 has full installed clients on Android, iOS and Windows, with private conversations, knowledge and work in one shell. A feature is not usable if a person cannot reach it with their input method, perceive its state or recover from an error. Windows keyboard and screen-reader behavior is already an explicit release gate; a common cross-platform accessibility contract is needed before implementation evidence can be judged consistently.

## Capabilities

- **CAP-1**
  - **intent:** A Windows user operates Messenger, Knowledge/Notes and Projects/Tasks, including errors and dialogs, with a keyboard alone.
  - **success:** An installed Flutter Windows release build completes keyboard-only journeys with visible focus, no pointer-only action or keyboard trap, and operable system menu/tray actions.
- **CAP-2**
  - **intent:** A Windows screen-reader user understands and operates the same Apps and their save, sync, conflict and failure states.
  - **success:** Real assistive-technology inspection of the installed build finds meaningful names, roles, values/actions, reading order and state announcements, without relying on Flutter framework support as proof.
- **CAP-3 — candidate floor**
  - **intent:** Android and iOS users operate the Release-1 journeys with touch, platform screen readers and applicable keyboard input.
  - **success:** Installed-build TalkBack and VoiceOver scenarios cover first run, Messenger, Notes, Project, Files, calls, sharing, errors and conflicts; each blocked action has an accessible explanation.
- **CAP-4 — candidate floor**
  - **intent:** A person can understand changing sync, delivery, access, conflict and call state without relying on color, motion or a transient visual cue.
  - **success:** State text/names and privacy-safe announcements convey the same outcome; focus returns predictably after dialogs, sharing, conflict actions and calls.
- **CAP-5 — candidate floor**
  - **intent:** People using enlarged text, resized windows, RTL/long translations or reduced motion retain the primary action and current Persona/Space context.
  - **success:** Representative small/large/landscape/Windows-resize fixtures retain reachable controls and legible state; reduced-motion treatment does not hide a document change or failure.

## Constraints

- `REQ-CLIENT-006`, `NFR-PLAT-006` and `ADR-0020` require Windows installed-build keyboard, menu/tray and screen-reader proof; a framework demo, widget test or Rust-domain test cannot substitute.
- The Android/iOS floor and exact interaction rules originate in draft PRD NFR-6 and discovery UX. They are candidate acceptance checks until the user approves their scope; this document does not turn them into approved requirements.
- Accessibility output cannot reveal a title, variant or content that the active Persona is not authorized to read. Accessible and visual status must agree on local save, remote sync and conflict distinctions.
- [W3C WCAG 2.2](https://www.w3.org/TR/WCAG22/) offers testable technology-neutral criteria for web content; [Flutter accessibility](https://docs.flutter.dev/ui/accessibility) and [Windows accessibility guidance](https://learn.microsoft.com/en-us/windows/apps/design/accessibility/accessibility-overview) inform native-client checks. None is silently declared the VIDA conformance level.

## Non-goals

- This draft does not approve a WCAG level, EN 301 549 mapping, specific screen-reader/OS versions, visual tokens or exact control sizes.
- It does not redesign individual Apps, replace the [Windows shell](../spec-vida-windows-shell/SPEC.md) or collaborative-editor conformance contracts, or claim that Flutter semantics automatically pass an installed-client audit.
- Live-call captioning or transcription is not presumed from the approved E2EE calling scope.

## Success signal

The approved Windows gate is demonstrable only when a traceable installed release build passes keyboard, menu/tray and screen-reader journeys alongside `WIN-F01–F08`. After the broader floor is approved, the same release candidate must pass the cross-platform [A11Y-F01–F12](accessibility-conformance-cases.md) matrix on named OS and assistive-technology configurations. No such builds or results are attached yet.

## Assumptions

- Candidate CAP-3–CAP-5 reflect the current draft PRD and discovery UX direction, not final user approval; implementation must not treat their exact coverage matrix as closed.

## Open Questions

- Do you approve the candidate Android/iOS floor: TalkBack and VoiceOver across the whole Release-1 journey, keyboard where supported, text/state announcements, enlarged text and reduced motion?
- Which formal standard/level, native-app mapping, OS/assistive-technology versions and exception process define the release acceptance matrix?
- What accessible alternative, if any, is required for live audio/video calls (for example captions), given the current approved E2EE calls scope?
