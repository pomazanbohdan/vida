---
id: SPEC-IDENTITY-001
status: approved
implementation_status: unplanned
last_updated: 2026-09-26
requirement_refs:
  - ../02-requirements/identity-requirements.md
decision_refs:
  - ../03-architecture/decisions/ADR-0004-multi-axis-identity-model.md
source_refs:
  - ../../_bmad-output/planning-artifacts/research/technical-vida-identity-modes-2026-09-18/research.md
  - ../../_bmad-output/planning-artifacts/research/technical-vida-multi-device-federation-ownership-c-2026-09-19/research.md
  - ../../_bmad-output/planning-artifacts/research/technical-persona-bootstrap-recovery-reference-pat-2026-09-26/research.md
---

# SPEC-IDENTITY-001: Identity domain contract

## Purpose

Визначити мінімальний реалізаційний контракт, який не дозволяє clients, nodes і Apps побудувати несумісні identity models.

## Scope і non-goals

У scope: domain entities, identifiers, binding/projection boundaries і нормативні transitions. Не у scope: wire format, DID method, cryptographic algorithms, recovery package format, alias registry protocol та node metadata policy.

## Terminology

| Entity | Contract |
|---|---|
| `LocalVault` | Локальний encrypted container для Persona, devices і recovery material; не network identity |
| `Persona` | Stable application/cryptographic subject |
| `ControllerState` | Versioned authority, яка може authorize/revoke devices і transitions |
| `DeviceGrant` | Signed authorization конкретного device |
| `ServiceBinding` | Revocable relationship Persona ↔ federated service account |
| `ServiceAccount` | Node-issued account with its own lifecycle; not a root Persona |
| `AliasBinding` | Mutable human-readable address → stable Persona reference |
| `Credential` | Selectively disclosed verified claim |
| `PublicProfile` | Explicit public projection Persona |
| `EndpointBinding` | Device ↔ transport endpoint relationship |

## Requirements

- `SPEC-ID-001`: `PersonaId != DeviceId != EndpointId != ServiceAccountId` `MUST` бути структурним інваріантом.
- `SPEC-ID-002`: `Persona` `MUST NOT` вимагати `ServiceBinding`, `AliasBinding`, `Credential` або `PublicProfile`.
- `SPEC-ID-003`: `ServiceBinding`, `AliasBinding`, `Credential`, `PublicProfile`, `DeviceGrant` і `EndpointBinding` `MUST` мати власні IDs, state та lifecycle.
- `SPEC-ID-004`: один `PersonaId` `MAY` мати кілька concurrent `ServiceBinding`; один binding `MUST` належати рівно одній Persona.
- `SPEC-ID-005`: один `LocalVault` `MAY` містити кілька Persona; network protocol `MUST NOT` виводити їхню спорідненість із факту локального co-location.
- `SPEC-ID-006`: canonical persistence `MUST NOT` кодувати presets як взаємовиключний account type.
- `SPEC-ID-007`: корпоративна реєстрація `MAY` створити окрему Persona перед створенням `ServiceAccount`/`ServiceBinding`; вона `MUST NOT` автоматично використовувати приватну Persona чи розкривати зв'язок між ними на wire.
- `SPEC-ID-008`: блокування `ServiceAccount` `MUST` інвалідувати sessions і Space grants, що залежать від нього, без revocation інших Persona, bindings або незалежних Spaces.
- `SPEC-ID-009`: client/read API `MUST` фільтрувати Spaces, пошук, recent items, previews сповіщень і export за активним account context та чинними grants, а не з належності Persona одному `LocalVault`; cross-account aggregate потребує окремої явної авторизації.
- `SPEC-ID-010`: приватний та корпоративний контексти, які не були явно пов'язані, `MUST NOT` використовувати спільний on-wire endpoint/binding identifier, що видає їхню кореляцію.
- `SPEC-ID-011`: автономне `PersonaCreated` `MUST` супроводжуватися створенням випадкового owner-held recovery secret, версійного encrypted recovery bundle і надійним записом UX-підтвердження, що власник зберіг обидві частини окремо від Device та одна від одної. Bundle містить окремий recovery-authority credential і необхідні data-key envelopes, але `MUST NOT` клонувати приватні signing keys звичайних Devices чи містити secret. До підтвердження staged Persona/Personal Space `MUST` залишатися `setup_pending` без захищеної роботи; після переривання setup `MUST` відновлювати ті самі IDs і комплект без непомітної заміни. Підтвердження не доводить фактичного існування копій. Новий Device `MUST` створити власні ключі й отримати чинний `DeviceGrant`; історія `MUST NOT` вважатися відновленою без доступної encrypted resource copy. Платформа `MAY` пропонувати додатковий OS/cloud backup, але не як єдиний шлях; byte/crypto/current-controller proof залишаються `OQ-0024`.

## Data and state model

```text
LocalVault
  └─ Persona ─┬─ ControllerState ──authorizes──> DeviceGrant ──> EndpointBinding
              ├─ ServiceBinding[] ─────────────> federated nodes
              ├─ AliasBinding[] ───────────────> exact/public addresses
              ├─ Credential[]
              └─ PublicProfile? ───────────────> directory/web/search
```

Мінімальні relation constraints:

- child record посилається на `personaId`, але public/contextual representation `MAY` приховувати root reference;
- device і binding revocation не видаляють Persona;
- alias mutation не змінює `PersonaId`;
- public profile deletion/disable не видаляє private Persona data.

## Interfaces and protocols

Потрібні абстрактні commands/events:

- `CreatePersona` / `PersonaCreated`;
- `AuthorizeDevice` / `DeviceGrantIssued`;
- `RevokeDevice` / `DeviceGrantRevoked`;
- `AttachService` / `ServiceBindingCreated`;
- `MigrateService` / `ServiceBindingReplaced`;
- `PublishProfile` / `PublicProfilePublished`;
- `DisablePublicProfile` / `PublicProfileDisabled`;
- `LinkPersonas` / `PersonaLinkDeclared` — лише після explicit irreversible-correlation acknowledgement.

Конкретні payload schemas лишаються окремою protocol specification після вибору controller history.

## Lifecycle and failure behavior

| Transition | Required behavior |
|---|---|
| Autonomous → Federated | Створити `ServiceBinding`; Persona continuity лише за явним вибором |
| Node A → Node B | Створити binding B, перенести дозволений state, переключити address, revoke binding A |
| Private → Public | Preview → explicit consent → `PublicProfile` + optional alias |
| Anonymous → Public | Default: new Persona; link/merge: explicit warning + audit evidence |
| Public → Private | Stop future publication; do not promise erasure of external copies |
| Device lost | Revoke `DeviceGrant`; rotate affected envelopes/epochs; preserve Persona |
| Corporate account suspended | Зупинити sessions і доступ до Spaces, наданий через `ServiceAccount`/binding; не змінювати інші Persona або їхні grants |

Failure `MUST` be atomic at the control-state level: partially created binding/profile `MUST NOT` appear active. Retry `MUST` be idempotent by command ID.

## Security and privacy

- contextual identifiers and keys `MUST` minimize cross-context correlation;
- shared hidden root keys `MUST NOT` appear in public aliases, endpoints or ordinary message signatures;
- node attestation `MUST NOT` grant controller authority unless an explicit managed-identity policy later defines it;
- publish/link operations `MUST` create audit evidence;
- secrets and recovery material `MUST NOT` enter logs or public projections.

## Compatibility and migration

Service export `MUST` distinguish portable Persona/controller state, encrypted user data and node-owned operational metadata. Importer `MUST` validate controller continuity before activating a migrated binding.

Legacy records that encode a single account type `MUST` be migrated into independent Persona, binding and profile records; ambiguous cases require explicit user choice.

## Observability and operations

Telemetry `MUST` distinguish Persona, device, service-binding and publication lifecycle events without emitting private keys, recovery material or cross-Persona vault linkage.

## Acceptance criteria

- contract tests cover every transition table row;
- migration keeps `PersonaId` stable when continuity is selected;
- revoking a device or binding preserves Persona and unrelated bindings;
- корпоративний акаунт може використовувати окрему Persona; його блокування прибирає тільки залежні Spaces, а перемикання на інший акаунт лишається доступним;
- після перемикання чи блокування корпоративного акаунта пошук, recent items, previews сповіщень і export не показують залежні корпоративні ресурси у приватному контексті; збережений plaintext не оголошується фізично видаленим;
- public profile state changes independently of federation state;
- privacy test confirms no root-link field in contextual wire representation;
- persistence/schema lint rejects canonical three-state `AccountType`.
- network privacy fixture не виявляє спільного endpoint/binding identifier між непов'язаними приватним та корпоративним контекстами одного LocalVault.

## Test and conformance plan

1. Domain invariant/property tests.
2. Multi-device authorize/revoke tests.
3. Node migration and rollback tests.
4. Publication consent/depublication tests.
5. Metadata-correlation analysis for contextual identities.
6. Legacy account migration fixtures.

## Open questions

Tracked as `OQ-0021`–`OQ-0026` and corporate controller question in governance.
