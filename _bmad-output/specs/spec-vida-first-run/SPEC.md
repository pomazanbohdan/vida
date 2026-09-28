---
id: SPEC-VIDA-FIRST-RUN
status: draft
companions:
  - first-run-cases.md
  - ../../../docs/02-requirements/native-client-requirements.md
  - ../../../docs/02-requirements/app-package-requirements.md
  - ../../../docs/02-requirements/contact-card-requirements.md
  - ../../../docs/02-requirements/platform-nfr.md
  - ../spec-vida-persona-recovery/SPEC.md
sources: []
---

> **Draft BMad contract.** This SPEC and its companions distill approved Release-1 first-run requirements into planned verification. The PRD and UX screen proposals remain draft/discovery inputs, not approved visual designs; no fixture here has been executed.

# VIDA first run and Core App activation

## Why

A new user must reach useful private, local-first work without registering with an external server, understanding package internals or granting optional OS permissions. First run also establishes the recovery and sharing boundaries that prevent an apparently usable Persona from silently becoming unrecoverable or public.

## Capabilities

- **CAP-1**
  - **intent:** A person creates an autonomous Persona, chooses visible Profile fields and receives a Personal Space without external registration.
  - **success:** Offline setup durably stages an isolated Personal Space; it becomes usable for protected work only after the owner confirms separate storage of recovery material. Recovery is a distinct entry path and switching Personas does not combine their data.
- **CAP-2**
  - **intent:** The new Persona owner records recovery material separately from the current Device before completing setup.
  - **success:** The material is shown with a loss warning; until explicit, durably recorded confirmation of separate storage, the same pending Persona resumes after restart and cannot create a protected Note. The UI does not claim that confirmation proves a copy exists or that the secret alone restores missing history.
- **CAP-3**
  - **intent:** The person chooses which bundled Messenger, Knowledge/Notes and Projects/Tasks instances to use and show now.
  - **success:** Each choice explains user value rather than package internals; all three packages are installed and their standard instances provisioned; only selected instances become active and visible, while an unselected one can be activated later without reinstalling VIDA or recreating the Space.
- **CAP-4**
  - **intent:** The person uses local Contact Cards without an OS connector, or connects an address book in a chosen direction with informed consent.
  - **success:** Declining permission leaves local Contacts and setup usable; a new connector defaults to read/import, and export or two-way sync requires explicit selection plus preview before the first external write.
- **CAP-5**
  - **intent:** On supported Android Devices, the person can opt into or decline a more available background mode for the current Device.
  - **success:** The mode explains battery/system-notification trade-offs, can be changed later without losing saved work, does not inherit consent across Devices or Personas and never promises permanent Online state.
- **CAP-6**
  - **intent:** The person reaches a first useful local resource and can choose a next step toward another Device, contact or group.
  - **success:** A locally committed note reopens after restart without a server and is labeled “Збережено локально,” not “Синхронізовано”; optional collaboration entry does not expose Personal Space implicitly.

## Constraints

- Autonomous setup cannot require a federated node; new-Persona and restore flows must remain distinct, with no automatic cross-Persona data linkage.
- The `setup_pending` Persona and Space keep their stable IDs after interruption; only durable recovery confirmation activates them and permits protected work. This strict gate is VIDA product policy, not a requirement inferred from reference apps or OWASP.
- A provisioned `AppInstance` is not activated, visible or granted resource access merely because its package ships with VIDA. First-run activation is scoped to the Personal Space.
- Recovery handling follows the [Persona recovery contract](../spec-vida-persona-recovery/SPEC.md); key/bundle format and production proof remain `OQ-0024`, not a choice made by onboarding.
- Contacts connector and Android high-availability permissions are optional. iOS suspension or push reachability must not be represented as an active Online connection.
- A personal Project remains in Personal Space; inviting a team requires an explicit separate Shared Space, while sharing a Note/Section can be resource-scoped without sharing the Space.
- Security review checks sensitive recovery storage against [OWASP MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) and [OWASP Key Management](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html); Contacts permission choice against [OWASP MASVS-PRIVACY-1](https://mas.owasp.org/MASVS/controls/MASVS-PRIVACY-1/). These references are verification criteria, not a claim of conformance.

## Non-goals

- This spec does not choose screen layout, visual tokens, recovery cryptography, connector-specific permission UI or a permanent mobile background connection.
- City Portal, local-business booking, marketplace discovery and browser onboarding are not Release-1 first-run requirements.
- Offering second-device or group setup does not make either mandatory before the first local resource is usable.

## Success signal

On Android, iOS and Flutter Windows, a new autonomous Persona can stage setup without a server or optional Contacts permission, resume the same pending identity after interruption, and cannot create a protected Note before durable recovery confirmation. After confirmation, the owner can activate selected bundled Apps, create and reopen a local Note, then activate a deferred App without losing its Space. [ONB-F01–F13](first-run-cases.md) are planned acceptance cases, not executed evidence.

## Open Questions

- Which first-run, Persona/Profile and Personal Space screens need approved visual mocks, and what exact layout/accessibility order passes the UX review?
- Which platform-specific Contacts permission journeys and Android high-availability implementation meet the conformance tests without implying always-Online behavior (`OQ-0067`, `OQ-0074`)?
