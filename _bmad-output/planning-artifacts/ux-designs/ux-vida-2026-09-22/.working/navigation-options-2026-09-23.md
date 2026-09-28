---
type: ux-research
status: historical-comparison-current-decisions-in-2026-09-24-update
date: 2026-09-23
scope: VIDA Release 1 navigation on Android, iOS and Windows
---

# VIDA navigation: reference check and alternatives

This is a historical working comparison, not a finalized navigation contract. Its earlier All-Spaces search/create and third-party-App placement proposals were superseded or narrowed by the user on 2026-09-24. Read the current [decision and reference update](navigation-decision-update-2026-09-24.md) and [EXPERIENCE.md](../EXPERIENCE.md) before reusing any option below.

## Product constraints from approved VIDA sources

- [Composition model](../../../../../docs/01-product/composable-workspace-model.md): Space is the ownership/security/sync boundary, not necessarily the UX boundary. Global views may aggregate authorized resources from several Spaces, but must label ownership. A complex App can own a dedicated Space; simple extensions can remain AppInstances inside one.
- [Release-1 bundle](../../../../../docs/01-product/v1-replacement-bundle.md): Messenger, Notes/Knowledge and Project are bundled but their visibility/activation is chosen by the user. Contacts is mandatory Core, not a Messenger App. Files are a shared resource capability. Forum belongs to group/project context.
- [EXPERIENCE](../EXPERIENCE.md): Persona isolation, resource-scoped sharing, explicit local-save vs sync vs delivery, offline state, Back returning to source, and access-checked links are existing behavioral constraints. A route may not reveal hidden resource names merely because it is in Search, Recents or a relation.

## What external references actually show

| Source | Observed pattern | VIDA inference, not source requirement |
|---|---|---|
| [Apple Tab Bars](https://developer.apple.com/design/human-interface-guidelines/tab-bars) and [Sidebars](https://developer.apple.com/design/human-interface-guidelines/sidebars) | Tabs serve top-level destinations and preserve each tab's navigation state; sidebars expose more collections on wider layouts. Apple warns against unpredictably appearing/disappearing tabs. | Keep a small, stable set of top-level destinations during ordinary navigation. Explicit user customization/activation may change the set, but switching Space should not reshuffle it. |
| [Android layouts and navigation](https://developer.android.com/design/ui/mobile/guides/layout-and-content/layout-and-nav-patterns) | Compact navigation bars hold 3–5 peer destinations; larger windows use rail/other adaptive layouts. | Do not put every App, File, Contact, Forum and Activity into the mobile bottom bar. Do not simply stretch that bar across tablet/desktop. |
| [Microsoft NavigationView](https://learn.microsoft.com/en-us/windows/apps/design/controls/navigationview) | Adaptive left navigation supports categories and hierarchy; Microsoft recommends a shallow hierarchy, with two levels ideal. | A Windows sidebar can show core destinations, selected Space and a short pinned/recent list, while deeper resources use list/detail or contextual pane. This does not require WinUI; the Release-1 Windows client is Flutter. |
| [Slack workspace navigation](https://slack.com/help/articles/212596808-Adjust-your-sidebar-preferences) | Workspace switcher is distinct from navigation and content sections; Home/Activity/Later are separate routes. | Distinguish Persona/Space scope from content destination. Activity is a useful separate attention model, but need not be a mandatory top-level VIDA tab. |
| [Notion sidebar](https://www.notion.com/help/navigate-with-the-sidebar) | Workspace switcher, private/shared/teamspace areas, recents, favorites and apps coexist. | Space hierarchy and shortcuts can live together; a resource may be easy to reach without copying it into a second Space. |
| [Linear teams](https://linear.app/docs/teams), [search](https://linear.app/docs/search) and [favorites](https://linear.app/docs/favorites) | Team/project hierarchy, cross-workspace search and pinned resources provide both context and fast return. | VIDA can offer global Search/Recent/Favorites within a Persona while keeping each result's OwnerSpace visible and rights-checked. |
| [Teams app pinning](https://support.microsoft.com/en-us/teams/apps-service/pin-an-app-in-microsoft-teams) | Apps can be discovered from a larger list and explicitly pinned; apps can also appear in a chat/channel context. | External AppPackages should not automatically seize permanent navigation slots. User-initiated pinning is a candidate for frequently used Apps. |

These are analogies, not evidence that their product architecture or security model matches VIDA.

## Four shell combinations

| Model | Phone primary navigation | Strength | Main cost for VIDA |
|---|---|---|---|
| A. Function-first | Chats · Knowledge · Project · Spaces; each module handles its own Space selection | Fast to the three bundled activities; familiar when only one Space exists | Repeats scope controls, risks different filters per module; external dedicated Apps and Core Contacts/Files are easily buried. |
| B. Space-first | Spaces · Inbox · Search · Activity; select Space before its Apps/resources | Clear ownership and good fit for dedicated App Spaces | Adds navigation steps for everyday direct messages, personal notes and cross-Space work; makes a Space feel like a hard UX boundary even though VIDA does not require one. |
| C. Two-axis hybrid (approved prototype basis) | Small, stable activity destinations plus a separate visible Persona/Space scope selector; Spaces opens the Space/App/Files hub | Supports fast global work and explicit ownership; scales to dedicated Apps without adding a tab per App | Needs very clear scope labels, create-target confirmation and cross-resource Back behavior; the exact tab set is still a user decision. |
| D. Home/Activity-first | Today/Inbox · Chats · Spaces · More; Notes/Projects reached via home cards or Space | A unified attention entry can help a busy team | Makes core Knowledge/Project less direct and creates a dashboard the user has not requested; adds a second, potentially noisy content summary. |

## Stress test against VIDA journeys

| Journey | A | B | C | D |
|---|---|---|---|---|
| Write a private note while currently in a shared project | Fast, but creation target can be ambiguous | Explicit Space switch first | Fast Knowledge route; explicit Personal Space target for create | Depends on Home shortcut; target still needs confirmation |
| Answer a direct chat while in a Project Space | Fast if Chats aggregates Spaces | Leave project, find chat's Space | Fast global Chats; row always shows owning context | Fast only if surfaced in Inbox |
| From task, open linked note, discussion, File and return | Requires context-preserving relation route, not tab hopping | Same requirement | Same requirement; context route is part of the shell contract | Same requirement |
| Open a separately published City App | Find it under Spaces/Apps | Natural Space entry | Natural Space entry, optional explicit pin | Find it under Spaces/More |
| Messenger not activated, but Contact Card needed | Fails if Contacts exists only under Messenger | Can work from Space/Core tools | Requires independent Core Contacts route; Messenger People is only a shortcut | Requires independent Core Contacts route |
| Many third-party Apps installed | Bottom bar must not expand per App | Space library scales | Space/App library scales; pinned shortlist only | More library scales but can become a catch-all |

## Approved model and prototype details still open

1. Keep the approved header facts visible without forcing full names into one cramped line. A short Persona identity, Space/scope label and meaningful Device state can expand to details; resource-level “Синхронізовано” remains separate.
2. **Approved model:** treat destination and scope as separate axes. A destination answers *what am I doing?*; the scope answers *whose/which Space's resources am I seeing?* (`current Space` or explicitly `all accessible Spaces` within this Persona). Relaunch restores the last selected authorized Space. The example destination names and fallback when that Space is unavailable remain proposals. A global result always shows its OwnerSpace; a create action chooses one target Space, never an implicit aggregate.
3. Keep top-level navigation stable while moving between resources or Spaces. App activation/visibility changes it only through an explicit setting, not as an incidental effect of opening another Space. A bundled App that is active somewhere in this Persona remains findable even if inactive in the currently selected Space.
4. Give Core Contacts a route independent of Messenger, e.g. a People entry in the Persona/Space library plus global Search; decide its exact placement by prototype. Files remain available in each Space's library and through permitted attachments/relations, with optional global search.
5. Keep Forum inside its group or Project, and discussion attached to its resource. A linked resource opens with its owning Space visible and Back returns to the exact originating chat/task/note, preserving draft, list position and filter. A link does not grant access.
6. On wide layouts, adapt the same information model to a left navigation + list/detail + optional related-context pane. Do not turn desktop into a different account/scope model. The Windows pane can expose pinned/recent resources without making them new ownership containers.
7. Do not approve the exact bottom tabs, first-run/inaccessible-Space fallback, search/activity placement, conditional tab visibility, breakpoint or visual tokens until the user reviews small-phone and Windows prototypes. Test enlarged text, screen readers, safe areas, keyboard, Back, offline scope labels and access-denied links.

## Historical decisions for the user — resolved or superseded on 2026-09-24

1. On first launch, or if the last Space is no longer accessible, which safe scope should open? The ordinary relaunch rule is already approved: restore the last selected authorized Space.
2. Should user-activated bundled Apps be shown as persistent top-level destinations for that Persona, with third-party Apps discoverable/pinnable from the Space/App library, or should every App be entered only through its Space?

The user has since approved Personal Space fallback, a separate Contacts entry, activated core destinations, automatic App-menu visibility with user configuration, in-Space creation/navigation/Search and three adaptive presentation profiles. The remaining ambiguity is whether an All Accessible Spaces browse-only overview survives alongside routine Space-local work.
