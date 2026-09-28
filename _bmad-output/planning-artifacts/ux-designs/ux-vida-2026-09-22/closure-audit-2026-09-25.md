---
type: targeted-ux-closure-audit
date: 2026-09-25
status: concept-handoff-ready
---

# VIDA Release 1 UX — concept handoff after visual selection

## Verdict

The user approved all ten 2026-09-25 UX recommendations and then selected candidate A (graphite and terracotta) as the common visual concept for all Core surfaces and Apps. The earlier compact Chat, Knowledge and Project previews remain composition/density references; their literal blue/violet colors are not the selected palette. `DESIGN.md` and `EXPERIENCE.md` remain `discovery` in the BMad sense because detailed tokens, screen mocks and validation are intentionally deferred to per-App implementation. The cross-App concept and interaction patterns are ready to hand to epic/story planning; `status: final` would overstate visual proof.

## Decisions now closed

1. Calm neutral, content-first visual character with restrained warm accents; literal colors are not yet chosen.
2. System-following light/dark theme with manual override.
3. Mobile bottom destinations for activated Chat/Notes/Project plus “Ще”; Contacts, Calendar, Files and other Apps remain auxiliary within the selected Space.
4. Header Space picker changes scope without combining Spaces.
5. Persona switch previews account type/domain before changing context.
6. Calendar menu visibility is a personal preference, not a Space-wide disablement.
7. Event pre-send preview shows owning Space, VIDA invitees and exact shared fields/files.
8. Simple recurring events are edited as a whole series in Release 1; per-occurrence exceptions are deferred.
9. Conflict starts with an inline resource indicator and opens a dedicated comparison surface, not an app-wide blocking modal.
10. Phone one-pane; medium list/detail; wide optional third context pane without crowding the main content.
11. Candidate A is the product-wide visual concept in light and dark; exact colors, type, shape and screen details will be selected while implementing each App.

## Deferred implementation-level UX work (not a block to epic planning)

- **Visual tokens:** derive literal light/dark graphite and terracotta values, multilingual typography, shape, spacing, elevation and icon language from the selected concept; verify contrast and non-color status cues for each implementation.
- **Per-App screens:** elaborate Chat/Forum, Knowledge, Project, Files, Contacts, Calendar and Core onboarding/sharing/conflict surfaces one by one. Existing images prove relative density/composition, not production layouts.
- **Responsive/accessibility proof:** test phone, tablet/split-screen and resizable Windows, text scaling, screen-reader order, keyboard/focus, reduced motion, safe areas and pane collapse without lost draft/selection as those surfaces are built.
- **BMad UX finalization:** offer the opt-in reviewer gate and confirm mock coverage when implementation-level design artifacts exist; no independent UI review or implementation pass is claimed now.

## Next BMad document

The side-by-side visual set remains in `.working/visual-theme-comparison-2026-09-25.html`; A is selected as concept, not as literal token contract. The PRD and shared architecture spine are already `final` and accepted, while no epic/story plan was found under planning artifacts. Per the installed BMad method route, the next planning document is the ordered epics and stories list (`bmad-create-epics-and-stories`), with explicit implementation stories for App-by-App UI elaboration and proof. Do not restart PRD or architecture solely to decide pixels. If a future UX choice changes cross-App behavior or architecture, reconcile it before building the affected story.
