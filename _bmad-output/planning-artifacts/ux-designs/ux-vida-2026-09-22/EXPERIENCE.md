---
name: VIDA Experience
description: Information architecture, behavior, states and journeys for VIDA Release 1.
status: discovery
updated: 2026-09-25
sources:
  - ../../prds/prd-vida-2026-09-22/prd.md
  - ../../prfaq-vida.md
  - ../../../../docs/01-product/v1-replacement-bundle.md
  - ../../../../docs/02-requirements/native-client-requirements.md
  - ../../../../docs/02-requirements/platform-nfr.md
  - ../../../../docs/01-product/composable-workspace-model.md
  - ../../../../docs/02-requirements/access-control-requirements.md
  - ../../../../docs/03-architecture/decisions/ADR-0002-default-role-presets.md
  - ../../../../docs/03-architecture/decisions/ADR-0021-static-web-client-in-release-1.md
  - ../../../../docs/04-specifications/sync-presence-status-model.md
---

# VIDA Release 1 experience

The [DESIGN.md](DESIGN.md) spine owns visual identity, including the selected product-wide graphite/terracotta direction A. This document owns the screens, behavior and words shown to a person. Where the PRD or an approved requirement settles a behavior, that decision takes precedence. The primary navigation model, theme behavior and adaptive pane count were approved on 2026-09-25. This is a cross-App concept/behavior handoff: per-screen layout, exact tokens and platform-specific presentation are to be worked through with each App during implementation, without silently changing these approved behaviors.

## Foundation

- Surfaces: installed Flutter clients on Android, iOS and Windows plus a full static Flutter Web client in Release 1. They share the same Core authorization, Resource and synchronization semantics, with platform-specific input, permission, storage and lifecycle behavior. Windows supports keyboard, system menus, tray and screen readers; Web is an authorized equal Device, not a read-only companion.
- The active Persona is always apparent when viewing or creating content. Switching Persona changes the visible profile, Spaces, contacts and available actions; it never silently combines two Personas.
- A Space defines membership, rights and synchronization. Routine navigation, creation and search stay inside the selected Space. “All Spaces” is only a Space launcher/status list, not a mixed resource browser or creation/search scope.
- Bundled Messenger, Notes/Knowledge and Project are available at installation but activated in a Space by user choice. Contacts and Calendar are mandatory Core capabilities; the Calendar menu entry may be shown or hidden for a Space independently of Project. Files, selected-Space search, identity, recovery and synchronization are Release-1 capabilities.
- The interface describes results at the level the person can act on: a local save, successful replication, delivery, reading, business approval and an external effect are separate facts.

## Information Architecture

| Surface | Main content | Entry and destination |
|---|---|---|
| First run | Persona/Profile, Personal Space, recovery material, bundled component choice, contact connector choice and optional Android high-availability choice | New installation → first usable personal resource; recovery material also opens a separate restore path on a replacement Device |
| Persona and profile | Current Persona, public profile fields, its online device count, device management | Switch Persona; open own or another visible profile |
| Personal Space | Private chats, notes, personal projects and Files; resource-scoped sharing | Open selected component or create a personal resource |
| Shared Space | Members, role-aware settings, enabled Apps, projects, conversations, knowledge and Files | Invitation or explicit Space creation |
| Messenger | Direct/group conversations, replies, reactions, voice messages, pins and attached Files | Conversation → message detail, contextual resource, call or forum |
| Forum | Topics linked to a group or project, with durable structured discussion | Group/project → topic → related resource |
| Knowledge | Notes, sections, documents, history, attachments and backlinks | Note → edit, discuss, link, share or inspect history |
| Project | Projects, list/board, tasks/subtasks, assignee, status, due date and linked context | Task → discussion, note, forum topic or File |
| Files | Per-Space files with versions and permissions | File → preview, links, revisions or permitted copy |
| Contacts | Space-owned VIDA Contact Cards; system-contact import into Personal Space; members and contacts are different entities | Auxiliary Contacts route → scoped card, communication, event, share or import preview |
| Calendar | Mandatory Core Personal/Shared Space events, simple recurrence/reminders, existing-VIDA-only invitations and RSVP; external calendar sync deferred | Contact card or Calendar → event in current Space by default → scoped invite/RSVP status |
| Search | Local results from the selected Space only, with permitted App/type/resource filters | Space search → source resource only after access check |
| Activity and sync | Pending work, replication state, conflicts, attention and errors | Status → affected resource and available next action |
| Web Device enrollment | Static-origin trust, browser-profile storage/recovery limits and explicit equal-Device authorization | Existing trusted Device → authorize Web → same permitted Personal Space and Note; lost profile/keys → new enrollment, not inherited access |
| Settings and Apps | Component activation, per-user Calendar menu visibility, system-following or manual light/dark theme, Space settings, permissions, contact import, diagnostics and recovery | Explicit configuration → preview → apply |

### Approved navigation model; exact sizes and breakpoints awaiting prototype

**[APPROVED — header identity and scope.]** The shell keeps the active Persona, Space/current scope, and this Device's connection/synchronization state visible. Tapping the active Space name opens an authorized Space picker; changing Space does not aggregate content. Persona switching shows the account name/type and organizational domain when present before changing context, never blending data across Personas. Exact dimensions and icon treatment remain open. Resource-level “Синхронізовано” remains distinct from the Device connection state.

**[APPROVED — activity and Space are distinct axes.]** A primary destination selects an activity within the selected Space; a separate selector changes the Space inside the active Persona. Persona selection changes the account and its accessible data. Routine navigation, creation and search are Space-local. “All Spaces” lists accessible Spaces and their allowed status for choosing one; it does not mix their chats, notes or tasks, or offer cross-Space creation/search. Opening a related resource keeps its OwnerSpace identifiable and a return path to its source. The older global-search wording is superseded in the final PRD and canonical client/product requirements.

**[APPROVED — reopen and fallback.]** On relaunch, VIDA restores the last selected authorized Space for the active Persona. If that Space no longer exists or is inaccessible, VIDA opens the current Persona's Personal Space without revealing the unavailable Space. Within the Space, it restores the last authorized resource/screen when possible; otherwise it opens the Space start. First-run behavior follows the onboarding flow.

**[APPROVED — local work and attention.]** A Note is created from the Notes section of one concrete Space, never from an All Spaces launcher. Search runs only inside the current Space. In-Space notifications remain in that context; a separate cross-Space attention indicator may alert to authorized events elsewhere without changing the current list or disclosing unauthorized names. Opening such a notification switches to its authorized source Space; Back restores the prior screen and unsent draft when still permitted.

**[APPROVED — Contacts and Apps.]** A first-party Core entry is named “Контакти” and remains independently reachable when Messenger is inactive, but lives in auxiliary navigation rather than a permanent primary/bottom item. A Contact Card added in a work Space belongs to that Space, separate from personal contacts and from Space membership; it is visible to permitted team members, not private to its author by default. Explicit cross-Space copy previews fields, creates a new ID and does not subscribe to future source changes. Contact search starts in the selected Space and allows explicit switching with authorized Space labels. Connected Apps appear automatically in the App menu of the Space where activated; visibility is configurable. Hiding an App removes only its navigation entry: activation, sync and notifications continue. Deactivation is a separate action.

**[APPROVED — personal presentation preferences.]** Theme follows the operating system unless the person chooses light or dark manually in settings. Calendar is shown in the current Space menu by default; each person may hide its menu entry without disabling events, reminders, sync or another participant's view. The preference is not a Space governance action.

**[APPROVED — destination and pane composition.]** The following table fixes destination roles and responsive pane behavior; exact icons, dimensions, bar/rail implementation and breakpoints remain prototype questions.

| Layer | Android/iOS compact window | Windows / wide window |
|---|---|---|
| Identity and scope | Approved visible header: Persona, current Space/scope, Device connection/sync state; exact packing awaits prototype | Same three facts remain visible; exact pane/header arrangement awaits prototype |
| Primary destinations | Bottom destinations for activated Chat, Notes and Project plus “Ще”; inactive Apps do not occupy dead slots. Space switching is separate in the header. “Ще” opens current-Space Calendar, Files, Contacts and other activated Apps; Contacts is not a permanent bottom item. | The same destinations in adaptive left navigation, expandable/collapsible with window width; Persona and Space remain separate selectors. |
| Find and act | Current-Space search and creation only in the relevant Space section; sync state opens Activity | Current-Space search and creation in the relevant section; shortcuts are additive, never the sole route |
| Browse and work | One main pane: list → full-screen resource detail; Back restores list position | Medium: list and detail; wide: optional third context pane only when enough width remains for the main content; resizing preserves selected Space/resource and Back path |
| Related context | Note/task discussion, relations and attachments open as a contextual sheet, keeping a return path to the source | Optional right context pane; it collapses before the main editor/detail becomes cramped |

This shell does **not** require a separate Home dashboard: after first setup, reopen the last authorized destination in the last authorized Space, falling back to that Space's start if needed. Forum topics live inside their group or Project; Files has a full per-Space view and appears through attachments. Contacts is a mandatory Core service reached through “Ще”, independent of Messenger. Connected third-party Apps appear automatically in their activated Space's configurable App menu, but not every App enters the bottom bar. An inactive bundled App has a clear activation path, not a broken tab. The older options remain as historical comparisons in [.working/navigation-options-2026-09-23.md](.working/navigation-options-2026-09-23.md); the 2026-09-25 selection here wins.

The proposed composition is grounded in [Apple tab guidance](https://developer.apple.com/design/human-interface-guidelines/tab-bars), [Android compact navigation guidance](https://developer.android.com/develop/ui/compose/components/navigation-bar), and [Microsoft adaptive list/detail guidance](https://learn.microsoft.com/en-us/windows/apps/develop/ui/controls/list-details). These references support stable top-level navigation and adaptive browse/detail layouts; they do not mandate a VIDA brand, exact tab count for every configuration or a Flutter widget choice.

Three illustrative screens for review are in [.working/layout-concepts-2026-09-22.md](.working/layout-concepts-2026-09-22.md): mobile group chat, mobile task detail and wide Windows Knowledge editor. They test this composition only; their color, typography, example content and control details are not approved design decisions.

The user preferred the denser mobile conversation in [.working/chat-skill-v3.png](.working/chat-skill-v3.png) over the larger v2; see the [comparison slide](.working/chat-skill-comparison-2026-09-22.png). This resolves chat density only. The two-axis navigation, mobile destination roles, header facts and calm-neutral visual character are approved; exact visual tokens in DESIGN.md remain open.

The user then selected [mobile chat direction 1](.working/mobile-chat-selected-2026-09-22.png) as the reference for two additional [Knowledge and Project screen concepts](.working/mobile-direction-selected-2026-09-22.md), and approved their information density and block order. These concepts test continuity of the mobile shell and cross-resource navigation. The later 2026-09-25 navigation decision supersedes their unapproved tab details; approval still does not settle exact visual tokens or prove working Flutter behavior, accessibility or responsive layouts. The Knowledge and Project mockups place a resource-specific “Синхронізовано” label below the resource header, separate from the shell's device “Онлайн” label.

### Surface closure

UJ-1 lands in First run, Personal Space and Knowledge; UJ-1R returns through recovery to the same Persona. UJ-2 lands in Device management, Activity/sync and Project; UJ-2W covers first Android↔Web Note continuity and browser-specific failure states. UJ-3 lands in Shared Space, Project, Messenger and Forum. UJ-4 lands in Knowledge, share preview and the recipient’s resource view. UJ-5 covers Messenger/Project call entry and the call surface; UJ-5R covers incoming-call acceptance. UJ-6 enters from Contact Card or the current Space's Calendar and lands in Event detail, scoped invitation and RSVP. Product scope is fixed in the PRD; detailed screen states and error paths remain UX work.

### Visual coverage — discovery audit

The spines define every Release-1 surface; the images are illustrative, not implemented or accessibility-tested screens. “Spine-only” does not remove a surface from Release 1.

| IA surface | Current visual evidence | Remaining load-bearing view/state |
|---|---|---|
| First run | Spine-only | Persona, recovery and component activation sequence |
| Persona and profile | Spine-only | Switcher that cannot mix Personas; device count |
| Personal Space | Spine-only | Private scope and resource-level sharing entry |
| Shared Space | Partial context in selected mobile screens | Membership, role and App activation |
| Messenger | Selected compact mobile group-chat reference | Direct chat and call entry |
| Forum | Spine-only | Topic list and topic discussion |
| Knowledge | Approved mobile note composition; Windows editor is an unapproved proposal | Share preview and collaborative edit states |
| Project | Approved mobile task composition | Task list/board and conflict comparison |
| Files | Spine-only | Per-Space list, versions and local download state |
| Contacts | Spine-only | Card, connector preview and identity binding privacy |
| Calendar | Spine-only | Personal/Shared event list, detail, invitation, RSVP, rescheduling and scoped guest view |
| Search | Spine-only | Current-Space results, offline coverage and denied-result treatment |
| Activity and sync | Spine-only | Pending, conflict and replication details |
| Web Device enrollment | Spine-only | Origin/storage warning, equal-Device grant, lost-profile recovery and revocation route |
| Settings and Apps | Spine-only | Activation, update and permission review |

Candidate key visual references for finalization are first run, share preview and conflict comparison, because their layout carries recovery or access meaning. The user must confirm which spine-only surfaces need mocks; do not treat these candidates or the proposed shell as approved layouts.

## Voice and Tone

- Use short, direct Ukrainian labels that state the present fact and the next available action. Localize the same meaning across approved locales; avoid technical protocol names in everyday messages.
- Do not use “надіслано”, “синхронізовано”, “доставлено” or “підтверджено” as synonyms.
- Example: “Збережено на цьому пристрої. Синхронізуємо, коли з’явиться зв’язок.”
- Example: “Синхронізовано з іншим вашим пристроєм.” only after the required application receipt; a single Device with no replica remains “Збережено локально”.
- Example: “Зміни різняться. Перегляньте обидва варіанти.” for a conflict; never imply that a variant was discarded.
- Example: “Доступ до цього Space треба перевірити. Під’єднайтеся до мережі.” after the seven-day shared-read limit.
- Explain a refused permission in terms of the requested action; do not expose hidden resource titles or variants.

## Component Patterns

### Group conversation

Keep Persona and owning Space identifiable, then show the group, Chat/Forum choice, chronological messages, linked resources and composer without expanding every message into a large card. Incoming and own messages must remain distinguishable. A linked Task or File opens in its authorized Space without granting access through the link alone. Device reachability belongs to the shell; message-local publication, delivery (`N/M` recipient Personas for a group) and read facts belong to the message. Compactness must not remove the action label, sender, relevant time, accessibility name or a usable touch target.

### Resource header and relations

Show resource title/type, owning Space, author when permitted, current user actions and a separate state indicator. A relation opens its target only if the current Persona can read it. Private backlinks and their titles remain absent from a shared note. A task, note or file can have a contextual discussion without duplicating the resource or creating a second Messenger model.

### Share preview

Before granting access to a note or section, show the recipient, exact scope, role/permissions, explicitly included files and whether future changes are shared. A link to a private resource does not disclose its title. A personal project offered to a team leads to an explicit Shared Space choice; the Personal Space is never made shared by implication.

### Space membership and ownership

Creating a Shared Space starts private; the creator is its initial Owner. The invitation flow shows the proposed member’s preset and any narrower resource permissions before confirmation. Only an Owner can add or remove another Owner. An Admin may manage other members within their grants but cannot display an Owner-governance action as available. Removing a member shows the Space-wide loss of access and the limit of revocation on an unsynchronized or exported copy.

The invitation and permission views show membership class, scope and role separately, then preview effective access before the invite is sent. Built-in presets are immutable; a custom role is a separately named clone. Available controls follow the effective permission, not the preset label alone. The baseline examples below must show both a permitted and a refused action; scoped overrides may narrow access, but never bypass Owner governance.

| Preset or class | Example available action | Example refused action in the baseline preset |
|---|---|---|
| Owner | Manage Space ownership | Cannot remove the last Owner without an atomic successor |
| Admin | Manage an authorized non-Owner member or App | Cannot add, remove or demote an Owner |
| Manager | Manage a Project workflow in the granted scope | Cannot change Space security or roles |
| Contributor | Update an own/assigned Task or send in an accessible Chat | Cannot edit another person's unassigned content by default |
| Commenter | Comment or send in an accessible discussion | Cannot edit the main Note/Task content |
| Viewer | Read a granted Resource | Cannot send, edit or share by default |

Guest is a scope-bound membership class, not a seventh role: the view must not reveal unrelated Space navigation or names. ServicePrincipal has explicit capabilities, not a human preset or a human device count. A submitted membership change shows pending until its Core governance operation is accepted; only then may the UI describe the access change as effective.

### Contact connector

Contact Cards are owned by a concrete Space: a work card is not the same resource as a personal card, even if both reference the same person. A Space member appears in membership automatically but is not a Contact Card or CRM client/contractor; creating a card is optional. A work card is visible to current participants permitted to read its App/Project scope, without per-author privacy by default. Cross-Space copy previews included fields, creates an independent ID and does not make a live link. Contact search starts in the current Space and may explicitly switch to another accessible Space, displaying its label; it must not disclose a hidden Space or merge cards by name, phone or external account ID. The approved card hierarchy is name/photo/Space then message, call and create-event actions, followed by contact methods, notes, permitted related resources and import provenance. Future Jira/Azure DevOps identifiers stay in details, not as the primary display name; see [focused reference research](../../research/technical-vida-contacts-calendar-interop-2026-09-24/research.md).

Release 1 includes mandatory Core Calendar Events in Personal and Shared Spaces, with time zones, one-off and simple daily/weekly recurrence, reminders and RSVP. The Calendar menu entry is visible by default and each person can hide it for themselves without stopping events, reminders, sync or teammates' access; it does not depend on Project. Creating an event from a contact card defaults to the current Space. Only an existing reachable VIDA Persona can be invited: an unlinked contact cannot become a pending undeliverable invitee. Before sending, show the owning Space, selected VIDA invitees and exact fields/files each will receive. A non-member invitee sees title, time, time zone and place; description, files and related tasks appear only after explicit inclusion and scoped sharing. The invitee may propose another time, but only organizer or Space Owner/Admin may reschedule; Owner/Admin can continue when organizer loses access. After rescheduling, the old RSVP no longer counts and the invitee must respond again. In Release 1, editing a recurring event changes the whole simple series; individual-occurrence edits and complex exceptions are deferred. A one-off event can represent a distinct date/time without altering the series. Release 1 imports system contacts into Personal Space only; `.vcf` and `.ics` file exchange and external Google/Microsoft/OS-calendar sync are future work.

New Contact Cards can be created without a platform contact permission. Release 1 offers previewed system read/import to Personal Space only. Future export or two-way sync would require a separate choice and field-level preview; neither is a current action. VIDA does not silently merge similar cards or export anonymous/private identity bindings.

Sharing a Contact Card sends a snapshot by default. If the owner chooses a live share, the UI names the future fields that may update and offers a way to revoke it; the recipient’s own notes/tags remain theirs.

Disconnecting a system contact connector shows it as disconnected and stops future connector synchronization. It does not delete the VIDA Contact Card or the provider's contact; either deletion needs its own explicit action. The connector screen states this distinction before unlink.

### Notes collaboration

Release 1 shows authorized remote participants’ live cursors and selections on Android, iOS and Windows. A recently applied remote or offline edit briefly highlights the changed range and fades. Presence is transient, not a persisted document revision. The editor/engine choice remains OQ-2, but failure to meet its live-presence conformance gate blocks Release 1; it does not silently degrade to merge-only collaboration. Durable edits and their merge/conflict result remain visible independently of presence.

### Calls

Offer 1:1 and group audio/video for authorized conversation participants, with a maximum of 8 total participants and a layout optimized for 4–6. Show microphone/camera consent and current device route, participant changes, E2EE state, reconnecting and definite failure. No screen share or recording control is present. Autonomous anonymous Persona has no external push-assisted incoming call; reachability is shown honestly. The precise incoming-call UI awaits the OQ-3 media/platform proof.

### Project List and Board

List and Board are two views of the same Task ID and local commit, not copies. A permitted status or move made in one view appears in the other after that commit; a refused move leaves both unchanged and explains whether rights or the current workflow forbids it. Incompatible concurrent status changes open the shared Conflict comparison with both variants preserved; merely switching views cannot choose a winner.

### Apps and updates

Discovering, downloading and activating an AppPackage are separate steps. Activation shows its target Space, enabled dependencies and compatibility result. A package never obtains access to another Space from its publisher identity. Compatible automatic updates should identify what changed; an incompatible package affects only its AppInstance and gives a clear update action. iOS shows only packages that fit the approved declarative Release-1 capability profile.

## State Patterns

| User-visible state | Trigger | What the person can do |
|---|---|---|
| Збережено локально | Durable write on this device | Continue working; inspect pending sync |
| Синхронізація | Authorized path is transferring/reconciling | Continue working; open progress/details |
| Синхронізовано | Independent authorized durable replica applied and signed the frontier | Open details; do not interpret as recipient delivery or approval |
| Збережено для доставки | Encrypted mailbox accepted payload, recipient has not applied it | Wait or inspect delivery details |
| Доставлено | At least one controlled device of a recipient applied the message | In a group inspect `N/M` recipient Personas; “доставлено всім” requires every intended recipient, never every Device |
| Прочитано | Separate synchronized read fact | Show sender-visible read status only under the applicable privacy choice |
| Очікує підтвердження | Process requires domain approval | Inspect the responsible process; do not display a completed outcome |
| Конфлікт | Incompatible concurrent variants remain | Compare permitted variants and choose a normal authorized update |
| Не завантажено | File could not be stored on this device | Inspect reason, free space or retry; remote file is not silently erased |
| Не збережено у браузері | Browser storage/quota failure prevented a durable write | Keep the recoverable draft where possible, explain that the Note is not saved, and offer a retry; never label it synchronized |
| Очікує з’єднання Web | Web cannot reach a peer directly or through VIDA relay fallback | Keep durably saved local edits pending, show last proven sync state and retry on reconnection; either transport path alone never proves sync |

| Немає доступу / права слід перевірити | Revocation received or shared-read proof expired | Lock every shared read surface; keep separately permitted new local drafts |
| Пошук офлайн охоплює не все | Device has no connection or a Resource/File is not locally synchronized/indexed | State that results cover only the locally available permitted index; do not imply that an unreachable replica or undownloaded File was searched |
| Перехід задачі недоступний | Rights or the current Project workflow forbids a requested move/status change | Keep the Task unchanged and explain the refusal without disclosing hidden workflow or resource details |
| Великий File очікує вибору | File exceeds the configured auto-download threshold | Show size/availability and a separate download action; failure returns to “Не завантажено” with reason and retry |

For Note saves, Web uses the same ordinary “Збережено локально” state as other clients after a successful local transaction; do not add a browser-specific backup explanation, badge or confirmation dialog to each save. The separate one-time browser-profile/storage explanation remains at Web Device enrollment. If a Note is both replicated and in an unresolved incompatible text conflict, show “Конфлікт” as the primary badge and keep replication proof in details (product-owner decisions, 2026-09-28).

The connection indicator describes this device, not the save state of one resource. The profile online count includes only currently verified synchronizable user-controlled Devices of that Persona. The owner may see “З’єднання відновлюється”; others do not see that internal state. On iOS, a suspended, push-reachable app is not counted online.

### Per-surface state walk

This table applies the global states above to every Information Architecture surface. It defines what must remain truthful and actionable; it does not select a visual layout, animation or color. A denied Resource must not be exposed through an empty-state suggestion, cached title, result snippet or relation.

| Surface | Empty or cold start | Pending or offline | Error or denied |
|---|---|---|---|
| First run | Offer new Persona and recovery as distinct paths; no existing account is implied | Local setup can proceed without optional contacts or Android high-availability permissions | Invalid recovery material reveals no private Space; optional-permission refusal does not block a local Personal Space |
| Persona and profile | New Persona shows only its own profile and authorized Devices | Device online count uses current reachability proof; stale proof is not shown as Online | Failed switch preserves an existing active Persona; failed recovery opens none, and neither mixes their Spaces |
| Personal Space | An unactivated bundled App has an activation route; a new Space may have no Resources yet | Synchronized local Resources remain available offline; a fresh write shows local-save before replication | A failed save is not labeled saved; resource-scoped sharing never exposes the rest of Personal Space |
| Shared Space | Creator-only membership still permits an authorized invitation | Invite or governance change stays pending until accepted; offline work does not extend stale read rights | Effective revocation or expired seven-day rights check blocks managed reads without revealing Resource titles |
| Messenger | An accessible conversation with no Messages offers a permitted composer | Local draft/send, mailbox storage, recipient delivery and read remain separate facts | A failed send keeps recoverable local work; denied conversation does not reveal message content or metadata |
| Forum | An accessible group/project with no Topics can create one if permitted | A locally saved Topic or reply remains pending until accepted/synchronized | Denied Topic or parent group is not discoverable through its title or link |
| Knowledge | A new Note/section may start empty and remain locally editable | Durable edits remain visible offline; live cursors disappear when their current presence is unproven | A same-range conflict preserves variants; denied or expired shared access blocks editor and indirect reads |
| Project | A permitted Project with no Tasks offers task creation | List and Board show the same local Task commit; a later incompatible status becomes Conflict | Refused move leaves both views unchanged with a reason; hidden linked Resources remain hidden |
| Files | A Space with no Files has an add action only when permitted | Availability distinguishes metadata from downloaded bytes; a File above the threshold awaits explicit download | Disk failure leaves “Не завантажено” with reason/retry; denied File has no accessible preview or title |
| Contacts | Cards can be created with no connector or platform permission | Preview precedes optional system-contact import to Personal Space; disconnected means no new imports | Denied system permission leaves VIDA-local Cards usable; unlink does not delete either Card or provider contact |
| Calendar | A Space with no events offers creation where permitted; hidden personal menu entry is still reachable from a Contact Card or settings | Unsent/undelivered invitation remains pending; recurring-series edit previews the affected series and requires new RSVP after a time change | Non-VIDA contact cannot be selected as invitee; denied event or attachment discloses no hidden Space detail |
| Search | No result does not imply that every remote Resource was searched | Offline results declare their locally available/indexed scope | A revoked Resource is absent from results and snippets; inaccessible target fails the open-time access check |
| Activity and sync | No pending operation is not by itself proof that a sole Device is synchronized | Pending, transfer, independent replica receipt and Conflict are distinguishable | A failed or rejected operation points to its recoverable local record and next allowed action |
| Web Device enrollment | Explain static-origin trust and browser-profile storage/eviction risk before authorization; no prior Device grant is implied | After authorization, cached shell/JS/Wasm and local data allow a proven offline cold reopen; remote changes remain pending when neither direct nor relay path is reachable | Missing cached shell/data does not pretend to open offline; lost profile/keys require new enrollment and old-grant revocation from a trusted Device or recovery path; a compromised origin cannot be trusted to report its own compromise |
| Settings and Apps | Bundled Apps remain discoverable even when not activated | Download/cache, activation and update acceptance are separate states | An incompatible version affects only its AppInstance; denied management or an invalid package cannot silently activate |

Keyboard focus, screen-reader name, enlarged text and reduced-motion treatment apply to every applicable state above, including empty actions, recovery errors, permission refusals and conflict choices.

## Interaction Primitives

- Create/edit operations must survive restart after “Збережено локально”. On failure, retain the draft where possible and state clearly that save did not complete.
- Search is local over currently accessible, synchronized content. A result opens after a fresh access check; hidden resources are not revealed by their titles or snippets.
- For incompatible edits to the same field or sentence, show an inline indicator on the affected Resource and open a dedicated, keyboard-accessible comparison surface without a blocking app-wide modal. Compare variants, show who changed what when known, and offer “Прийняти поточне” or “Застосувати моє” to an authorized editor. For a conflicting File, also offer a distinct “Зберегти як ревізію” or “Створити окрему копію”; those actions have different IDs/history.
- Non-overlapping text edits merge transparently. A later unknown concurrent branch can reopen a previously resolved conflict. Equivalent outcomes may show one value while preserving both audit decisions.
- A conflict from someone else is not automatically assigned as a correction task to a third person. Any authorized participant may make a new ordinary update.
- Use links to move between chat, note, task, topic and file while preserving the Space and resource context. A link alone grants no access.
- The person can choose activation and later deactivation/visibility of bundled Apps per Space; package download, activation and access are separately explained actions.
- A diagnostic report is assembled locally. Before sending, the person sees its destination and redacted content; messages, keys and cross-Persona identifiers are not silently included.

## Accessibility Floor

- Full control with touch, keyboard and supported screen readers on every Release-1 platform. Keyboard focus is visibly indicated on all interactive controls; Windows journeys include keyboard-only navigation, system menus and tray.
- Every state change is available as text and accessible name; color or animation alone cannot convey sync, delivery, conflict, access or call state. DESIGN.md must assign contrast-compatible tokens after visual direction is chosen.
- Focus returns predictably after dialogs, conflict resolution, shares, navigation and a call ending. Errors identify the affected item and recovery action.
- Cursor presence and fading edit highlights may be reduced or disabled with reduced-motion settings; document changes remain discoverable through accessible history/status.
- Dynamic text and localization expansion must not hide primary actions or truncate a security-relevant explanation.

## Responsive & Platform

- The user approved three presentation profiles: compact mobile with one main pane, medium/tablet with list and detail, and wide/desktop with an optional third contextual pane only when the main editor/detail keeps sufficient width. They share the same Space/resource semantics. A resized Windows window or split-screen tablet may cross profiles; preserve selected Space/resource and return path while adapting by available window space and input needs, not an immutable device label. Exact measured breakpoints and minimum widths remain prototype decisions.
- Android/iOS: respect permission, contact, call, background and notification controls of the OS. Android high-availability is optional per device, offered in onboarding and settings. iOS “онлайн” requires an active controlled connection.
- Windows: installed full client, with keyboard, system menus/tray and screen reader support. Window resizing must retain the active resource and Space context.
- Web: the static client uses the same Space/Resource behavior and adapts by available window width; its exact pane/bar layout remains a prototype decision. Enrollment first identifies the code-delivery origin and browser-profile data-loss risk. Offline cold reopen is claimed only when shell, JS/Wasm and local data are actually cached. Web↔native sync tries a proven direct browser-compatible path first and uses VIDA-operated encrypted Iroh relay fallback if direct is unavailable, without paid Hosted Space. Stock Iroh/Wasm alone does not prove direct browser P2P; browser transport needs a separate prototype. A closed/suspended tab promises no background sync, incoming call or precisely timed reminder. Browser keys/profile are not a backup, and the page cannot certify that its own origin is uncompromised. Keyboard, screen-reader and focus behavior must be proven on the supported-browser matrix.
- Layout dimensions, exact responsive breakpoints, visual tokens and platform-specific bar/rail implementation remain prototype decisions. The approved destination roles, relative content density, resource state and permission rules apply on every device.
- Before a layout is accepted, prototype it on a small phone, a larger phone, landscape/tablet and a resizable Windows window. Verify safe areas, that fixed bars do not cover scroll content or focus, a visible active destination, and Back returning to the originating list with its scroll/filter/draft state. Test enlarged text, reduced motion and screen-reader order; compactness must not reduce native touch targets. Exact visual tokens and breakpoint values remain open.

## Key Flows

### UJ-1 — Олена налаштовує VIDA

1. On first launch Олена creates an autonomous Persona and chooses visible Profile fields; the app explains that this Persona is independent of other accounts.
2. VIDA creates her Personal Space. Олена chooses Messenger and Notes; Project remains available for later activation.
3. Олена separately stores the recovery secret and encrypted bundle, then confirms that she did so; VIDA does not claim to verify those external copies. She then chooses whether to connect system contacts. Import preview is the default; declining optional contact permission does not block setup.
4. On a supported Android Device, she sees the optional per-device high-availability mode. The screen explains ordinary wake/reconnect behavior versus more battery use and a possible persistent system notification; neither promises continuous Online. She can decline now or disable the mode later in Settings without losing locally saved work. This choice never applies to another Persona or Device by implication.
5. She writes a note and sees “Збережено локально”. **Climax:** after app restart, the note reopens without a server.

### UJ-1R — Олена відновлює Persona після втрати пристрою

1. On a replacement Device, Олена chooses recovery instead of creating a new autonomous Persona.
2. VIDA asks for the previously stored recovery material and distinguishes it from a federated or organizational account sign-in.
3. Invalid or missing material does not reveal private Spaces or create an apparently restored Persona.
4. After valid recovery, VIDA re-establishes only authorized local identity and begins fetching permitted replicas; unavailable content stays pending rather than being claimed restored. **Climax:** Олена reopens her Personal Space under the original Persona after a verifiable local recovery path, without the organization becoming its owner.

### UJ-2 — Андрій продовжує на ноутбуці

1. Андрій approves a new equal Device from an existing Device or recovery flow.
2. His chat, note, task and file appear as available/synchronized according to their actual receipts.
3. Offline, he edits a task, restarts the client and sees the durable local change.
4. On reconnect the change syncs. **Climax:** both Devices show the same outcome or an explicit unresolved conflict with preserved variants.

### UJ-2W — Олена продовжує ту саму Нотатку з Android у Web

1. Олена creates a Persona and one plain-text Note on Android. The Note has a stable Resource ID; the richer Notes App will later extend this same record rather than import or copy it.
2. She opens the static Web client. Before pairing, VIDA identifies the Web origin as a code-delivery trust boundary and explains that browser-profile data can be cleared or evicted. Web does not silently inherit Android's keys or Device grant.
3. From an already trusted Device, she explicitly authorizes this browser profile as an equal Device. Only permitted Personal Space data appears; “Синхронізовано” requires an application-level receipt from an independent durable replica, not a relay/connection acknowledgement.
4. She edits the Note in Web without a connection. Only after a durable browser write does it say “Збережено локально”. She closes and reopens the cached Web client offline and sees the Note and pending change; this cold reopen is available only if the complete shell/JS/Wasm and local data remain present.
5. On reconnect, Web first attempts a verified direct peer path to Android; if unavailable, it uses VIDA-operated encrypted Iroh relay fallback for the same operations. **Climax:** both Devices show the same Note ID and converged content, or an explicit preserved Conflict if incompatible edits were concurrent. The UI progresses from local save through synchronization to proven synchronized receipt; only loss of both available paths leaves work pending. A transport switch does not duplicate a Note or count as a receipt.
6. If browser quota prevents a write, VIDA states that the draft was not saved and offers a recovery action. If the profile or keys are lost, the old grant is not transferred to a fresh profile: Олена re-enrolls from a trusted Device or recovery path and can revoke the old grant. The UI does not promise recovery of bytes that existed only in the lost profile.

### UJ-3 — Олена створює командний Project

1. Олена chooses “Створити спільний Space” and activates Project with Messenger and Notes dependencies.
2. She invites Андрій, reviews his role/permissions and confirms the grant.
3. She creates a task, links a note, a forum topic and a file; the project has both a quick chat and structured forum.
4. **Climax:** Андрій opens the task and its permitted context but cannot see unrelated private Personal Space resources.

### UJ-4 — Андрій ділиться однією нотаткою

1. In his Personal Space, Андрій selects a note or section and opens share preview.
2. He chooses the recipient and reviews the precise scope, included file and absent private backlink.
3. After confirmation, the recipient opens the shared note. **Climax:** the file is present only if explicitly shared; the private backlink never discloses its title.

### UJ-5 — Олена проводить дзвінок

1. Олена starts an authorized 1:1 or small-team audio/video call from a conversation.
2. Participants grant microphone/camera access and see who has joined; the call shows its protected state.
3. A network change triggers a visible reconnecting state. **Climax:** media resumes securely or the call ends with a clear failure; the app does not imply delivery while unreachable.

### UJ-5R — Андрій приймає вхідний дзвінок

1. While reachable on a supported Device, Андрій receives an incoming 1:1 or group call from an authorized conversation and can identify the current Persona and caller/conversation before answering.
2. He chooses to answer or decline; answering requests microphone/camera access only as required by the selected media mode. A declined or unanswered call is not presented as connected.
3. **Climax:** after answering, Андрій sees the actual participants and E2EE/media state, or a definite connection failure. The exact OS-level incoming-call presentation remains dependent on OQ-3 proof.

## Open UX decisions

1. Mobile chat, Knowledge and Project have approved relative density and block order; calm-neutral character and system-following theme with manual override are approved. Literal light/dark palette, typography, shape, spacing and contrast-tested implementation tokens still need visual review before DESIGN.md can be final.
2. Persona/Space/Device facts, separate activity and Space axes, activated Chat/Notes/Project plus “Ще” mobile destinations, Space and Persona pickers, and one/two/optional-three-pane composition are approved. Exact control dimensions, breakpoints and bar/rail details need prototype validation. Selected-Space-only Search is reconciled with the final PRD and canonical requirements.
3. The visual-coverage audit above identifies spine-only surfaces. Key screens where layout carries identity, recovery, scoped sharing or conflict meaning still need selected visual references and accessibility review; validation remains an opt-in, lens-selectable BMad UX finalization gate.
4. OQ-2 may alter how mandatory live cursors/selections are implemented, not whether they ship; OQ-3 may alter incoming-call UI details without changing the approved journey.
5. The per-surface state walk above covers the approved Release-1 behavior at contract level. Its exact layouts, accessible control order and platform-specific cold-load presentations still require prototype review before UX finalization.
6. The Web first-slice behavior in UJ-2W reconciles this spine with ADR-0021 and Epics 1–2. Exact Web layout, browser storage/key mechanism, supported-browser matrix and cached-shell implementation remain prototype/conformance decisions; they do not reduce Web's full Release-1 scope.
