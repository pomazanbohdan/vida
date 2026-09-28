---
id: ADR-0004
status: accepted
last_updated: 2026-09-19
source_refs:
  - ../../../_bmad-output/planning-artifacts/research/technical-vida-identity-modes-2026-09-18/research.md
  - ../../../research/Новий Text Document (5).txt
  - ../../../_bmad-output/planning-artifacts/research/technical-vida-multi-device-federation-ownership-c-2026-09-19/research.md
  - https://atproto.com/specs/did
  - https://atproto.com/specs/account
  - https://atproto.com/guides/account-migration
  - https://www.w3.org/TR/did/#did-correlation-risks
  - https://simplex.chat/docs/simplex.html
  - https://signal.org/docs/specifications/sesame/
supersedes: []
superseded_by: []
---

# ADR-0004: Multi-axis identity model

## Context

Vida має підтримувати повністю автономну роботу без реєстрації, підключення до одного або кількох federated nodes, opt-in public identity, кілька пристроїв, recovery та міграцію між providers. Один enum `AccountType = Anonymous | Federated | Public` змішав би незалежні властивості identity, hosting і discoverability та створив би несумісні переходи між режимами.

## Decision drivers

- автономний користувач не залежить від реєстрації або постійного provider;
- node надає сервіс, але не володіє root identity користувача;
- federation і publicity можуть комбінуватися незалежно;
- anonymous contexts не повинні автоматично корелюватися між собою;
- device, transport endpoint і service account мають окремі lifecycle та revocation;
- міграція між nodes не повинна примусово змінювати Persona.

## Considered options

1. Взаємовиключний `AccountType`: простий UX, але хибна domain model і складні migrations.
2. Server-bound identity: простіше routing/moderation, але provider стає identity authority.
3. Один глобальний public key: прості signatures/discovery, але високий cross-context correlation risk.
4. Multi-axis model із стабільною Persona та окремими bindings/projections.

## Decision

### 1. Persona є root identity

`Persona` `MUST` бути стабільною application/cryptographic identity. `Phone`, `email`, handle, domain, node-local account, `DeviceId` та transport `EndpointId` `MUST NOT` бути кореневим `PersonaId`.

`LocalVault` `MAY` містити кілька не пов'язаних між собою Persona. Така локальна спорідненість `MUST NOT` автоматично розкриватися в network data.

### 2. Identity modes є незалежними axes

Canonical domain model `MUST NOT` містити взаємовиключний `AccountType = Anonymous | Federated | Public`.

Система `MUST` моделювати незалежно щонайменше:

- identity scope;
- hosting/service bindings;
- discoverability;
- verification credentials;
- network privacy;
- authorization.

UX `MAY` показувати presets `Autonomous anonymous`, `Federated private` і `Public`, але preset `MUST` розкладатися на незалежні policy values.

### 3. Autonomous anonymous

Autonomous Persona `MUST` створюватися локально без phone, email або node registration. Вона `MAY` використовувати direct/LAN/P2P transport чи opaque relay, але relay `MUST NOT` ставати identity authority.

Anonymous interaction `MUST` підтримувати pairwise/contextual identifiers and keys без on-wire parent link. Конкретна default granularity — contact, room або Space — лишається окремим рішенням.

### 4. Federated node

Registration на node `MUST` створювати revocable `ServiceBinding`, а не нову root identity. Binding `MAY` містити `remoteAccountId`, address, mailbox, sync endpoint, quota, retention та federation policy.

У корпоративному сценарії користувач `MAY` спершу створити **окрему Persona** в локальному vault, а тоді зареєструвати для неї новий node-owned `ServiceAccount` і `ServiceBinding`. Цей робочий контекст `MUST NOT` непомітно повторно використовувати ідентифікатор приватної/анонімної Persona або розкривати зв'язок між ними. Це не робить `ServiceAccountId` кореневим `PersonaId`; точний controller окремої корпоративної Persona потребує окремого рішення.

Деактивація `ServiceAccount` або binding `MUST` припиняти лише access grants і sessions, що залежать від цього акаунта, включно з наданими компанією Spaces. Вона `MUST NOT` відкликати інші Persona у vault, незалежні bindings або Spaces. Client/read API `MUST` фільтрувати всі звичайні поверхні читання — список Spaces, пошук, recent items, previews сповіщень та export — за активним account context і чинними grants, а не за фактом спільного локального vault. Cross-account aggregate `MAY` існувати лише як окремий явно авторизований режим. Після блокування node account залежні views/indexes `MUST` бути недоступними у звичайному UI; це не обіцяє фізичного видалення раніше реплікованого plaintext.

Node `MAY` видавати attestation, але `MUST NOT` непомітно замінювати Persona controller. Зміна node `MUST` бути можливою через створення нового binding і відкликання старого зі збереженням Persona, якщо користувач явно обрав continuity.

### 5. Public identity

Public identity `MUST` бути opt-in `PublicProfile` із окремим `AliasBinding`. Публікуються лише явно вибрані profile fields, contact entry points, credentials та resources.

Перехід anonymous → public `MUST` за замовчуванням створювати нову Persona. Link або merge `MAY` виконуватися лише як окрема підтверджена дія з попередженням, що correlation може бути незворотною.

Вимкнення public profile `MUST` припиняти майбутню directory/web publication, але `MUST NOT` обіцяти видалення вже скопійованих даних.

### 6. Devices і transport

`PersonaId`, `DeviceId`, `EndpointId` і node `ServiceAccountId` `MUST` бути різними identifiers.

Кожний пристрій `MUST` отримувати окремий signed `DeviceGrant`, device signing key і key-agreement key. `EndpointBinding` `MUST` пов'язувати device з transport endpoint без прирівнювання endpoint до користувача.

Інакше не пов'язані приватний і корпоративний account contexts `MUST NOT` повторно використовувати спостережуваний `EndpointBinding`/endpoint identity, якщо це розкриває їхню спільну належність одному vault; потрібні окремі bindings і privacy fixture. Це не визначає controller корпоративної Persona (`OQ-0049`).

## Rationale

Модель відділяє durable identity від змінних адрес, providers, devices і publicity. Вона дозволяє autonomous, federated-private, self-hosted-public та інші комбінації без міграції між несумісними account types.

## Consequences

- clients і services мають працювати з Persona та bindings, а не з одним account record;
- directory, verification, hosting і authorization отримують окремі policy/lifecycle;
- один користувацький vault може мати кілька Persona без автоматичного link;
- корпоративна реєстрація може створити нову окрему Persona та node account без link з приватною Persona; блокування одного node account не блокує інші;
- migration і recovery потребують versioned controller state;
- UI presets не визначають canonical storage model.

## Risks і mitigations

- Прихована кореляція через keys/endpoints → contextual keys, metadata threat model і tests.
- Node фактично стає controller → signed binding contract і окремий controller state.
- Втрата всіх keys → чесне irreversible-loss повідомлення до вибору recovery contract.
- Alias squatting/abuse → registry не запускається до окремої governance policy.
- Помилкове відчуття депублікації → UI пояснює межі видалення зовнішніх копій.

## Acceptance evidence

- Persona може існувати й локально працювати без node account;
- Persona підключається до node і мігрує між nodes без зміни `PersonaId`;
- federated Persona може залишатися private;
- public profile створюється й вимикається окремо від service binding;
- anonymous contexts не мають спільного видимого parent identifier;
- device revocation не видаляє Persona;
- блокування корпоративного ServiceAccount прибирає тільки доступ через нього; паралельні анонімна/публічна Persona і їхні Spaces залишаються доступними;
- schema не містить canonical `AccountType` із трьома взаємовиключними значеннями.

## Open questions

- метод `PersonaId` і canonical controller history;
- default granularity anonymous Persona;
- recovery package/KDF/storage UX;
- public alias registry governance;
- node-visible metadata та compliance boundary.
- controller і recovery policy для окремої корпоративної Persona та policy переносимості її node-owned ServiceAccount.
