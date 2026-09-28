# PRD Quality Review — VIDA Release 1, Web inclusion

Reviewed: `prd.md` (637 lines) and `addendum.md` (132 lines), both updated 2026-09-25. Focus: consequences of approving a full static Web client for public Release 1. The seven judgments below follow the BMad PRD Quality Rubric; they are not a code, source-currency or browser feasibility audit. This review changes neither source document.

## Overall verdict

The product thesis, four-platform scope, test suites and explicit release blockers are unusually concrete; Web is no longer treated as a disposable demo. The PRD is **not yet safe as an unconditional Web implementation/release contract** because it overstates what a static host cannot access, leaves a browser-specific Contacts requirement ambiguous, and lets the media-selection open question retain a three-platform acceptance boundary. These are localized, repairable findings, not a reason to remove Web from Release 1.

## Decision-readiness — thin

The Web choice is unambiguous: §1 and §6 put static Flutter Web in Release 1, FR-39 requires local data and native↔Web sync without paid Hosted Space, §5 excludes a hosted private browser service, and §6.4 blocks release rather than silently shrinking scope. OQ-19 honestly leaves supported browsers, Rust/Wasm bridge, key custody and media profile for proof before Web fan-out. However, the static-origin trust trade-off is described as if hosting cannot affect plaintext confidentiality. A decision-maker comparing self-hosted versus third-party static hosting would need this risk explicitly stated before accepting deployment policy.

### Findings

- **[critical] Static host trust is overstated (§4.13 FR-39, line 505; addendum §A, lines 12–20).** The phrase “розміщення не надає host-у … доступу до відкритого вмісту” conflates no protocol authorization with the browser's trust in delivered HTML/JS/Wasm. A compromised or malicious static origin can serve modified client code that reads local plaintext or keys available to the page. *Fix:* state the narrower protocol claim, identify delivered Web code/origin as part of the trusted computing base, and make release-integrity, update authenticity, origin/XSS controls and user-facing host choice part of the Web acceptance profile. Do not promise that arbitrary static hosting is zero-knowledge merely because sync uses E2EE.

## Substance over theater — strong

The text contains domain-specific, checkable requirements rather than generic “secure/scalable” furniture: FR-23 separates local save, replication, delivery and business outcome; FR-39 names relay-only Web sync and browser storage loss; §6.3 demands distinct feature suites rather than crediting one vertical slice for the full product. The broad feature count is an explicit user-chosen Release-1 bet, and §6.4 openly makes failure block launch. The addendum usefully carries mechanisms that would otherwise swamp a product PRD.

### Findings

No separate finding. Breadth is a delivery risk already named as R-1, not in itself evidence of template padding.

## Strategic coherence — adequate

The thesis in §1/§1.1 is connected communication, knowledge and project work under one local-first Space/access/sync model; the Web decision serves that thesis by extending an authorized peer, rather than creating a separate hosted product. The Android↔Web vertical slice in §6.2 is a valid early proof of the key cross-platform claim. SM-1, SM-2 and SM-7 test conformance, integrity and openness; SM-C1–C3 guard against engagement/telemetry/feature-count theater. SM-3 retains a product-value check without behavioral tracking, although its target is deliberately pending OQ-8.

### Findings

No Web-specific finding. The PRD distinguishes a thesis-critical proof order from optional scope (§6.4), which matches the user's no-scope-cut decision.

## Done-ness clarity — thin

FR-39 provides useful observable outcomes and §6.3 requires a four-platform conformance matrix. Still, two edge contracts let teams interpret Web “full-featured” differently: system-contact import and offline relaunch. The three-platform call-profile question also conflicts with Web release gating even though FR-39 itself mentions Web E2EE calls.

### Findings

- **[high] Web Contacts parity has no testable product boundary (§4.1 FR-4, lines 145–158; §4.11 FR-33, lines 441–449; §6.3, line 545; §4.13 FR-39, lines 500–509).** The PRD requires user-selected **system-contact import** in Release 1, declares Web full-featured, and demands every FR's suite, but does not say whether a supported browser must provide device-address-book import, may offer a consented browser capability only where supported, or uses manual Contact Cards as its Web-equivalent. File-based `.vcf` import is separately excluded (§5), so it cannot silently fill the gap. *Fix:* make the Web acceptance row explicit: either name the supported browser/contact-import capability and behavior when unavailable, or scope system-address-book import to installed clients while requiring full canonical Contact Card CRUD/sharing on Web. This is a product parity decision, not merely a connector choice.
- **[medium] Offline Web reopen is not bounded at the user-observable level (FR-39 line 507; NFR-3/5 lines 569–571).** The PRD promises offline local data/actions and durability after Web reopen when the profile remains, but does not state whether the static shell and Wasm assets must be launchable after an online first load. One implementation could persist data yet show a blank/offline browser page. *Fix:* state whether a supported browser profile can reopen the app offline after a completed first load and access local data/actions; test that behavior separately from cleared-site-data/quota-eviction failure. Keep the service-worker/cache mechanism in the addendum/architecture.

## Scope honesty — thin

§5 clearly excludes paid Hosted Space, server-rendered private Web, City Portal, external calendar sync and WinUI. FR-39 explicitly says browser/site-data loss is not remote backup and Web does not promise direct P2P. The critical static-host wording above is the remaining overclaim. It matters especially because the product promises independent static hosting and anonymous/private Personas; a deployment operator's ability to change code cannot be disguised as a mere transport detail.

### Findings

- **[high] Web origin/privacy consequence is absent from user-facing deployment choice (§1 line 21; §4.13 lines 500–509; NFR-7 line 573).** Even after correcting FR-39's protocol sentence, the PRD should not leave a user to infer whether hosting their own static files versus loading a third-party origin changes the trust/metadata exposure. *Fix:* add a concise Web trust statement and acceptance proof for code provenance/update integrity and Persona-isolated storage/origin behavior; defer exact browser mechanisms to OQ-19. Treat this as the product consequence of the critical finding, not a separate request to remove static hosting.

## Downstream usability — adequate

The document provides stable FR-1–FR-39, UJ-1–UJ-6 and NFR-1–NFR-16 sequences, named core nouns in §3, feature groupings, standalone verifiable consequences, and §6.3 suites for story extraction. The addendum separates technology candidates from product behavior and lists R-2 contracts. A downstream media team could nevertheless read the open OQ-3 question as satisfied by Android/iOS/Windows only, despite the new Web release promise.

### Findings

- **[high] OQ-3 still asks for a three-platform E2EE profile (§10 line 621; addendum §G/§I).** The question says “на трьох платформах” while FR-39, §6.2, §6.3 and NFR-6/14 require Web as a fourth Release-1 client, including calls. The addendum's media-selection evidence lists F01–F17 without explicitly making browser media/key/lifecycle proof part of the profile choice. *Fix:* change OQ-3 and the addendum media gate to Android, iOS, Windows **and Web**, with mixed Web↔installed 1:1/group E2EE, browser media permissions and reload/lifecycle failure evidence before profile selection. Keep Web-specific mechanism selection under OQ-19, but do not let a native-only OQ-3 closure authorize Web calls.

## Shape fit — adequate

This is a chain-top, multi-surface consumer/team product, so named-protagonist UJs, product FRs, cross-cutting NFRs, release proof and an addendum are appropriate. Existing UJ-2 can encompass a browser as the second Device; a separate Web journey is optional, not required for shape. If the user reviews a Web-specific onboarding or recovery flow later, that belongs in UX/story work once the trust and browser-profile constraints above are fixed.

### Findings

No separate finding. The PRD is long because it is a broad public Release-1 contract; its structure still supports extraction.

## Mechanical notes

- ID scan: 39 FR headings form FR-1–FR-39; six UJ headings form UJ-1–UJ-6; 16 NFR headings form NFR-1–NFR-16, with no numbering gaps found.
- OQ-19 correctly records the 2026-09-25 Web product decision and retains technical proof before Web fan-out; OQ-3's “three platforms” is the concrete cross-reference drift to repair.
- `prd.md` and `addendum.md` both mark `status: final`; in this project that means accepted product intent, not implementation readiness (§10). The findings above should be fixed in the existing PRD/addendum, not by silently downgrading the Release-1 Web commitment.
- No inline `[ASSUMPTION]` or Assumptions Index entries were found in the reviewed Web addition; no round-trip mismatch was observed.
