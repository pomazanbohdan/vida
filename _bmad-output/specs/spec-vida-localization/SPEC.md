---
id: SPEC-vida-localization
status: draft
companions:
  - localization-conformance-cases.md
  - ../../../docs/02-requirements/localization-requirements.md
  - ../../../docs/02-requirements/platform-nfr.md
sources: []
---

> This kernel distills approved `REQ-LOCALIZATION-001` into a Release-1 test plan. It neither changes the 18-language/21-profile set nor claims that installed clients have passed the tests.

# VIDA localization conformance

## Why

One shared Rust core and three installed Flutter clients must not produce different language, fallback, search or warning behavior. A translated screen is insufficient when RTL navigation, CJK input, signed IDs or recovery notices fail.

## Capabilities

- **CAP-1**
  - **intent:** Every bundled Release-1 workflow is usable in each approved locale profile on Android, iOS and Windows.
  - **success:** The 21-profile × three-client release matrix covers shell, Messenger, Notes/Knowledge, Projects, Contacts, Files, Forums, Calls and system/error flows; missing critical translations fail the gate.
- **CAP-2**
  - **intent:** Locale choice and fallback are deterministic without changing identity or domain data.
  - **success:** BCP 47 resolution, explicit language/country separation, missing-key fallback and locale switching give the same declared result across clients; no path silently falls back to Russian or changes Persona, legal regime, data location or signed identifiers.
- **CAP-3**
  - **intent:** Layout, input, search and language-dependent values remain correct beyond English-shaped strings.
  - **success:** Arabic RTL, CJK glyph/wrapping/IME/search, plural and variable forms, dates/numbers/units/sorting, pseudo-localized overflow and representative accessibility fixtures pass.
- **CAP-4**
  - **intent:** Optional AppPackages and release-critical messages expose truthful language coverage.
  - **success:** A package declares supported locales and fallback in its manifest; host shows the actual fallback language; store listing, onboarding, privacy/security, recovery, permissions and destructive-action warnings have reviewed translations before public release.

## Constraints

- The approved source of truth is `REQ-L10N-001–009`: 18 languages, 21 initial profiles, Ukrainian required, Russian excluded. Chinese language support is not a mainland-China distribution decision.
- Locale changes are data/configuration changes, not forks of domain logic. Schema, permission and operation IDs and signatures remain language-neutral.
- The tests run against installed Android, iOS and Windows release builds; a CI catalog check alone does not prove visible text, screen readers or input-method behavior.
- Translation-source locale, review ownership and workflow remain implementation-plan choices; this spec does not invent them.

## Non-goals

- Adding locales, selecting a translation vendor or claiming GDPR/store approval.
- Translating user-authored messages, notes, files or third-party AppPackage content not supplied for a locale.
- Choosing Rust/Dart localization libraries or a concrete ICU bundle before representative build validation.

## Success signal

The same traceable `L10N-F01–F10` suite runs on all three installed release clients; per-locale evidence names the artifact/version, translated surfaces, fallback path and reviewed critical notice. No suite has run yet.

## Open Questions

- Which canonical source locale, review owners and translation workflow will the implementation plan assign?
- Which representative device/font/IME matrix and measured resource budgets will the platform conformance plan pin?
