---
type: ux-reference-and-decision-update
status: discovery-open-questions
date: 2026-09-24
scope: Contacts, Space navigation, App menu and adaptive clients
---

# Navigation decision update, 2026-09-24

The user narrowed routine work to one selected Space. A Note is created only in that Space's Notes area; Search is also Space-local. In-Space activity and notifications keep their Space context, while an authorized event elsewhere can raise a separate attention indicator. A missing last Space falls back to the active Persona's Personal Space. Reopening the last authorized screen is preferred, falling back to the Space start. Activated bundled Apps are visible destinations. Connected Apps automatically enter a configurable App menu. Contacts is an independent Core destination named “Контакти”. These are decisions; precise compact placement is not.

## Primary-reference check

| Reference | Observed behavior or guidance | VIDA consequence — inference, not source requirement |
|---|---|
| [Microsoft Teams People](https://support.microsoft.com/en-US/Teams/calls-devices/manage-your-contacts-with-the-people-app-in-teams) | A separate People app manages contacts and can be pinned to desktop navigation; its cards and list distinguish local and directory contacts. | A dedicated VIDA Contacts entry is credible, but the user's Ukrainian label “Контакти” better names VIDA's Core Contact Cards. Teams does not determine VIDA's card ownership or mobile placement. |
| [Telegram Contacts](https://telegram.org/blog/contacts-local-groups/tr?setln=en) and [Telegram contact notes](https://telegram.org/blog/comments-in-video-chats-threads-for-bots/kk?setln=en) | Telegram exposes Contacts, can add a correspondent from a chat, and supports a private note on a contact. | A VIDA Contact Card can be entered both from Contacts and contextual communication, without making Messenger the owner of the card. This is a UI analogy, not a data-model import. |
| [Outlook People pane](https://support.microsoft.com/en-us/outlook/what-is-the-outlook-people-page) | A contact pane can show associated messages, files and past/future events. | VIDA could show permission-filtered relations to chats, tasks, notes and events. Event creation is an open product/UX decision; a Calendar App must not be assumed in Release 1. |
| [Android adaptive navigation](https://developer.android.com/develop/ui/views/layout/build-responsive-navigation) and [canonical list-detail](https://developer.android.com/develop/ui/views/layout/canonical-layouts) | Compact, medium and expanded windows use different navigation affordances; list/detail can occupy separate or simultaneous panes. | Five or more core/App destinations should not be forced into a crowded mobile bar; medium and wide VIDA layouts can expose more context without changing resource identity. |
| [Apple split views](https://developer.apple.com/design/human-interface-guidelines/split-views) | iPad windows are resizable; split views adapt between compact and regular width and preserve hierarchy/selection. | Tablet is a presentation profile, not a fixed device category. Moving between one and multiple panes must preserve the current Space, selection and Back path. |
| [Flutter adaptive guidance](https://docs.flutter.dev/ui/adaptive-responsive/best-practices) and [general approach](https://docs.flutter.dev/ui/adaptive-responsive/general) | Layout branches should use available app-window width or parent constraints, not hardware type or orientation alone; state should survive resize. | Define compact/medium/wide behavior and test phone, split-screen tablet and resized Windows with one navigation state model. Exact breakpoints are not decided by this reference check. |

## Reconciliation before UX finalization

2026-09-24 follow-up decisions: “All Spaces” is a launcher/status list only, not aggregate resource browsing. Contacts is auxiliary navigation, and work-Space cards stay separate from personal cards; cross-Space import needs an explicit design. Search can show authorized Space association for cards. An App is shown in the menu only where activated; hiding its menu entry does not deactivate it or stop sync/notifications. A cross-Space notification opens its source Space, with Back restoring the prior screen/draft. Compact uses one pane, medium list/detail, and wide may add context; resize preserves navigation state. Real calendar events with invitees/RSVP are now a requested capability; [focused research](../../../research/technical-vida-contacts-calendar-interop-2026-09-24/research.md) records standards and unresolved scope. The App placement interpretation should be confirmed if the dictated user phrase meant something else.

- The previous two-axis prototype allowed an explicit All Accessible Spaces aggregate view. It is now narrowed to a Space launcher/status list only. Do not silently re-enable aggregate resource browsing, creation or Search.
- [PRD FR-21](../../../prds/prd-vida-2026-09-22/prd.md), [Release-1 bundle](../../../../../docs/01-product/v1-replacement-bundle.md) and [native client REQ-CLIENT-022](../../../../../docs/02-requirements/native-client-requirements.md) still require global cross-Space Search. The user now explicitly wants Space-local Search. Reconcile upstream requirements before finalizing specifications or tests.
- The user says connected Apps display automatically in a configurable menu of their activated Space, not necessarily as permanent bottom tabs. Contacts is independently reachable in auxiliary navigation even when Messenger is off; exact auxiliary placement and App ordering remain open.
- Contact-card event/time actions now mean a real calendar event with invited contacts and responses. Calendar scope and implementation boundaries must enter PRD/spec reconciliation; do not substitute a reminder or dated Task.
