---
type: technical
topic: VIDA contact cards and mobile-store resource constraints
decision: Define contact interoperability and mobile release constraints
status: complete
updated: 2026-09-22
verified_claims: 13
unverified_claims: 0
---

# VIDA Contacts and Mobile Store Constraints

## Executive summary

VIDA should own a canonical `ContactCard` resource and treat Android Contacts, Apple Contacts, Windows Contacts, Google People and CardDAV as optional connectors. Platform records are not rich or uniform enough to be the source of truth for multiple VIDA identifiers, visibility modes, verification and field provenance. General note/task/file import stays outside current scope; contact synchronization is a first-class v1 capability with explicit consent.

The super-app must not set its performance target from store maximum package sizes. Release gates should measure delivered size, startup, memory, battery/background behavior and sync recovery on representative devices. iOS cannot promise continuous background Iroh availability; Android high-availability mode remains opt-in and policy-constrained.

## Contact model and interoperability

Android aggregates raw contacts from multiple account types and explicitly supports service-owned sync adapters and custom MIME rows; it also warns not to attach unknown data kinds to another provider's account because they will not sync reliably [1]. Google People adds its own ETag concurrency and expiring sync-token rules [2]. Apple exposes permission-gated, partially selectable contacts plus save/change APIs but no general custom schema [3]. Windows provides a system ContactStore, aggregate contacts, change tracking and provider annotations [4].

Therefore the canonical VIDA card should contain a stable `contact_card_id`, human fields, per-field provenance, links to external records and an array of VIDA `identity_bindings` with type, identifier, visibility/persona context, verification state and source. A phone number or email is evidence for matching, not sufficient proof that two cards are the same person.

vCard 4.0 should be the baseline interchange representation [5]. CardDAV may be a connector protocol [6]. Provider-specific IDs, sync cursors, ETags and tokens remain connector metadata, never portable VIDA identity.

## Store and resource constraints

Apple requires efficient power use and restricts background execution to intended modes [7]. iOS background work is scheduled/opportunistic; silent push and BG tasks do not make a permanently connected peer a valid guarantee [8]. Android 14+ requires declared and justified foreground-service types [9]. Google Play also treats excessive wake locks, crashes, ANRs and memory as store-quality signals [10].

The current distribution ceilings are 200 MB compressed per-device APK delivery on Google Play [11] and 4 GB uncompressed for iOS apps, with a 500 MB executable text ceiling [12]. They are compliance ceilings, not VIDA budgets. Flutter deferred components help Android/web only [13], so v1 must obtain most savings from shared runtime, asset discipline, lazy data/file download and inactive-feature runtime suspension.

## Recommendations

1. Add `ContactCard` as a Core resource and a Contacts App/view; keep platform connectors optional, permission-scoped and reversible.
2. Synchronize VIDA identity bindings only through VIDA; export compatible public fields through vCard and supported platform fields without leaking private/anonymous bindings.
3. Start contact sync in preview mode: show creates/updates/merges/deletes before first two-way write; record field provenance and provider version tokens.
4. Establish store compliance now, then set numeric VIDA budgets after a representative Messenger+Notes+Project+Calls vertical slice. Budgets must be stricter than store maxima.
5. Keep iOS reachability semantics already adopted: online only with a live verified connection; push-reachable is not online.

## Open questions

- Is Contacts a separate bundled App or a Core shared service with a first-party view?
- Is system sync initially one-way into VIDA, one-way out, or previewed two-way per account?
- Which identity bindings may be exported into platform cards or vCard?
- What representative devices and percentile thresholds define VIDA size/startup/memory/battery budgets?
- Does the newly proposed browser offer replace ADR-0014, and is it free local-only, paid hosted-only, or both?

## Source appendix

| Ref | Finding | Publisher | Pub date | Accessed | Confidence |
|---|---|---|---|---|---|
| [1] | Android raw contacts, custom data and sync adapters | [Android Developers](https://developer.android.com/identity/providers/contacts-provider) | current | 2026-09-22 | high |
| [2] | Google contact sync tokens and concurrency | [Google for Developers](https://developers.google.com/people/api/rest/v1/people.connections/list) | current | 2026-09-22 | high |
| [3] | Apple Contacts permissions and store APIs | [Apple Developer](https://developer.apple.com/documentation/contacts) | current | 2026-09-22 | high |
| [4] | Windows ContactStore capabilities | [Microsoft Learn](https://learn.microsoft.com/en-us/uwp/api/windows.applicationmodel.contacts.contactstore?view=winrt-26100) | current | 2026-09-22 | high |
| [5] | vCard 4.0 | [RFC Editor](https://www.rfc-editor.org/info/rfc6350/) | 2011-08 | 2026-09-22 | high |
| [6] | CardDAV | [RFC Editor](https://www.rfc-editor.org/info/rfc6352/) | 2011-08 | 2026-09-22 | high |
| [7] | Apple energy and background review rules | [Apple Developer](https://developer.apple.com/app-store/review/guidelines/uk/) | 2026 | 2026-09-22 | high |
| [8] | Apple background strategy limits | [Apple Developer](https://developer.apple.com/documentation/BackgroundTasks/choosing-background-strategies-for-your-app) | current | 2026-09-22 | high |
| [9] | Google Play foreground-service declarations | [Google Play](https://support.google.com/googleplay/android-developer/answer/13392821) | current | 2026-09-22 | high |
| [10] | Android vitals | [Android Developers](https://developer.android.com/topic/performance/vitals) | current | 2026-09-22 | high |
| [11] | Google Play package size ceiling | [Google Play](https://support.google.com/googleplay/android-developer/answer/9859152) | current | 2026-09-22 | high |
| [12] | Apple build size ceilings | [Apple Developer](https://developer.apple.com/help/app-store-connect/reference/app-uploads/maximum-build-file-sizes) | current | 2026-09-22 | high |
| [13] | Flutter deferred components | [Flutter](https://docs.flutter.dev/perf/deferred-components) | current | 2026-09-22 | high |

## Staleness map

- Store policy, foreground-service and size limits: re-check before each release submission and at least monthly during active release work.
- Platform Contacts APIs and Google sync behavior: re-check when raising minimum OS/API versions or changing connector behavior.
- IETF vCard/CardDAV baselines are stable; re-check only when adopting an extension or successor specification.
