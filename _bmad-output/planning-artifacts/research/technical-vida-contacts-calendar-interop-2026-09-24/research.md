---
type: technical-research
status: focused-evidence; decisions-pending
date: 2026-09-24
scope: Contact Card UX, Space ownership, calendar invitation interoperability
---

# Contacts and calendar interoperability for VIDA

## Question and outcome

How can a Space-scoped VIDA contact and a real invitation/RSVP calendar flow stay portable across mobile devices and future Google, Microsoft, Jira or Azure DevOps integrations?

The user decided that a card added in a work Space belongs to that Space and is separate from personal contacts. Copying/importing between Spaces may be offered explicitly; no automatic cross-Space merge is approved. Contacts is an auxiliary destination, not a permanent primary/bottom tab. A calendar event is a real event with invitees and responses, rather than a dated Task or reminder. The detailed ownership, sharing, connector and Release-1 boundaries remain open.

## Reference findings

| Source | Verified capability | VIDA interpretation; not a source mandate |
|---|---|---|
| [RFC 6350 — vCard 4.0](https://www.rfc-editor.org/rfc/rfc6350.html) | Portable contact-card format with identity and contact properties. | Keep vCard 4 as the open exchange baseline; VIDA Space ownership, field provenance and private identity bindings remain internal extensions. |
| [RFC 9553 — JSContact](https://www.rfc-editor.org/rfc/rfc9553.html), [RFC 9555 — conversion](https://www.rfc-editor.org/rfc/rfc9555.html) | JSON contact representation and vCard conversion. | Candidate for a JSON interchange adapter; not an automatic replacement for the approved VIDA ContactCard schema. |
| [Microsoft Teams People](https://support.microsoft.com/en-us/teams/calls-devices/manage-your-contacts-with-the-people-app-in-teams), [Outlook People](https://support.microsoft.com/en-us/outlook/what-is-the-outlook-people-page) | Separate contact surface, quick communication actions and contextual related items/events. | Use a compact card header with explicit actions and permission-filtered sections; keep Contacts reachable from an auxiliary route. |
| [RFC 5545 — iCalendar](https://www.rfc-editor.org/rfc/rfc5545.html) | VEVENT, UID, organizer, attendees, time zones and recurrence. | Open calendar-event exchange baseline; distinguish VIDA event ID from external provider IDs. |
| [RFC 5546 — iTIP](https://www.rfc-editor.org/rfc/rfc5546.html) | REQUEST, REPLY and CANCEL scheduling messages. | Invitation and RSVP need explicit states and receipt semantics, not only a shared Event row. |
| [RFC 4791 — CalDAV](https://www.rfc-editor.org/rfc/rfc4791.html), [RFC 6638 — scheduling](https://www.rfc-editor.org/rfc/rfc6638.html) | Calendar access and scheduling interoperability. | Candidate standards adapter for future interoperable sync; not a requirement to run a CalDAV server in Release 1. |
| [Android CalendarContract](https://developer.android.com/reference/android/provider/CalendarContract.html), [Apple EventKit](https://developer.apple.com/documentation/eventkit/accessing-the-event-store) | Platform calendars/events/attendees with OS-specific permissions. | OS calendar connector is opt-in, previewed and separate from VIDA canonical data; iOS/Android permission flows differ. |
| [Google Calendar Events](https://developers.google.com/workspace/calendar/api/v3/reference/events), [Google incremental sync](https://developers.google.com/workspace/calendar/api/guides/sync), [Microsoft Graph event](https://learn.microsoft.com/en-us/graph/api/resources/event?view=graph-rest-1.0) | Provider event/attendee state, external IDs and incremental update mechanisms; Graph also documents transactionId for idempotent create. | Future connectors need per-provider ID, cursor/version, mapping and loop prevention; provider state must not silently override VIDA's Space rights. |
| [Jira Cloud Users API](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/), [Azure DevOps Graph Users](https://learn.microsoft.com/en-us/rest/api/azure/devops/graph/users/get?view=azure-devops-rest-7.1) | Product-user identities, not portable address books; fields may depend on visibility. | Future links are provenance-bearing external identities on a ContactCard, not proof that two cards are the same person or a Release-1 sync commitment. |

## Proposed model for discussion

1. VIDA owns the canonical `ContactCard` and `CalendarEvent` in an explicit Persona/Space scope. External providers are adapters, not authorities over VIDA membership or ownership. This is an architectural inference awaiting approval.
2. Contact list/card UI candidate: name/avatar and Space badge; quick actions for message/call/event; then identity/contact methods, notes, related resources, connector provenance and sharing controls. Never reveal a private field or another Space's title merely because a linked external identity matches.
3. Calendar UI candidate: personal or Space calendar, event organizer, invitees, response status, time zone, recurrence and links to permitted resources. An invite grants access to the invitation details only, not automatically to the whole Space. The latter is a proposed privacy boundary, not yet approved.
4. Copy/import between Spaces should be an explicit reviewable operation creating a separate card, with field visibility and provenance retained. Whether to offer live linkage instead is an open decision.
5. Do not conflate event's local save, delivery of invitation, attendee RSVP, organizer acceptance or provider synchronization. These need separate user-facing facts.

## Open decisions

- Work ContactCard visibility within a Space; role and private-note boundaries.
- Copy vs live link vs move between Spaces; cross-Space contact search semantics.
- Calendar ownership, invitation access, organizer authority, offline conflicting edits and recurrence.
- Release-1 scope of calendar UI and platform/provider connectors.
- Exact card layout, quick-action order and compact/medium/wide behavior.

## Staleness and limits

Standards are stable references; Google, Microsoft, Apple and Android APIs and store policies change. Recheck provider permissions, event ID/response behavior and platform policy immediately before implementation. This note is source-backed research, not approval of all proposed behavior.
