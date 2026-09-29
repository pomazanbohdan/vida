---
id: REQ-IDENTITY-001
status: approved
last_updated: 2026-09-26
source_refs:
  - ../../_bmad-output/planning-artifacts/research/technical-vida-identity-modes-2026-09-18/research.md
  - ../../_bmad-output/planning-artifacts/research/technical-vida-multi-device-federation-ownership-c-2026-09-19/research.md
  - ../../_bmad-output/planning-artifacts/research/technical-persona-bootstrap-recovery-reference-pat-2026-09-26/research.md
  - ../../research/Новий Text Document (5).txt
decision_refs:
  - ../03-architecture/decisions/ADR-0004-multi-axis-identity-model.md
---

# Вимоги до identity modes

## Підтверджені вимоги

### REQ-ID-001 — Автономне створення

Vida `MUST` дозволяти створити `LocalVault` і щонайменше одну Persona без phone, email, public handle або реєстрації на node.

### REQ-ID-002 — Незалежні axes

Identity scope, hosting, discoverability, verification, network privacy та authorization `MUST` зберігатися як незалежні властивості. Взаємовиключний `AccountType = Anonymous | Federated | Public` `MUST NOT` бути canonical domain field.

### REQ-ID-003 — Стабільна Persona

`PersonaId` `MUST` залишатися стабільним при зміні handle, device, endpoint або federated node, якщо користувач явно обрав identity continuity.

### REQ-ID-004 — Контекстна приватність

Anonymous interactions `MUST` підтримувати pairwise/contextual identifiers and keys. Network data `MUST NOT` автоматично розкривати, що дві Persona або contexts належать одному `LocalVault`.

### REQ-ID-005 — Service binding

Підключення Persona до federated node `MUST` створювати окремий revocable `ServiceBinding`. Node account, address, mailbox, quotas і retention `MUST` належати binding, а не визначати root Persona.

### REQ-ID-006 — Portability

Federated service contract `MUST` дозволяти export/migration identity state і зашифрованих user data. Відкликання старого binding `MUST NOT` автоматично відкликати Persona.

### REQ-ID-007 — Явна публікація

`PublicProfile` і `AliasBinding` `MUST` створюватися лише після явної згоди. Preview `MUST` показувати поля та contact entry points, які стануть публічними.

### REQ-ID-008 — Anonymous-to-public transition

Під час переходу anonymous → public система `MUST` за замовчуванням запропонувати нову Persona. Link/merge `MAY` бути доступним лише з окремим підтвердженням і попередженням про незворотну correlation.

### REQ-ID-009 — Межа депублікації

Вимкнення public profile `MUST` припиняти подальшу Vida-controlled publication. UI `MUST` пояснювати, що зовнішні копії, screenshots, caches і раніше встановлені linkage можуть зберегтися.

### REQ-ID-010 — Device separation

`PersonaId`, `DeviceId`, `EndpointId` та `ServiceAccountId` `MUST` бути різними. Кожний device `MUST` мати окремі keys і revocable `DeviceGrant`.

Підписані controller-події кожного Device `MUST` зберігати причинні посилання й обидві гілки одночасного конфлікту. Два сумісні grants різних Devices `MUST` сходитися автоматично. Peer, який виявив несумісні renewal/revoke чинного Device, `MUST` призупинити для нього нові захищені читання/записи, key envelopes і прийняття операцій до явного підписаного рішення власника на неспірному чинному Device; попередній plaintext не обіцяється стерти. Device-enrollment QR, файл і текст `MUST` бути представленнями короткоживучого одноразового наміру, прив'язаного до ключа нового Device, який не надає доступу без явного підтвердження trusted Device. Відновлення після втрати всіх Devices — окремий provisional flow; proof для partition лишається `OQ-0022/0024`.

### REQ-ID-011 — Recovery boundary

Recovery через trusted device або recovery package `MUST` відновлювати controller authority без непомітної заміни Persona node operator-ом. До затвердження recovery contract Vida `MUST` чесно попереджати про незворотну втрату після втрати всіх trusted devices і recovery material.

### REQ-ID-012 — Preset UX

Clients `MAY` пропонувати presets `Autonomous anonymous`, `Federated private` і `Public`, але `MUST` показувати та змінювати їхні незалежні privacy/hosting/discoverability наслідки.

### REQ-ID-013 — Окремий корпоративний акаунт

Реєстрація в корпоративному federated node `MUST` дозволяти створити окремий акаунт і робочий контекст без використання приватного/анонімного ідентифікатора користувача як ідентифікатора цього акаунта. Користувач `MUST` мати змогу перемикатися між ним та іншими своїми акаунтами. Локальне зберігання кількох контекстів або повторне використання спостережуваного transport endpoint `MUST NOT` саме по собі розкривати вузлам їхню спільну власність.

### REQ-ID-014 — Обмежене блокування

Блокування або відкликання корпоративного node account `MUST` припиняти доступ, наданий через цей акаунт, до відповідних робочих Spaces. Воно `MUST NOT` автоматично блокувати інші анонімні, персональні чи публічні акаунти користувача, їхні незалежні Spaces або їхні ідентифікатори. Модель controller для окремої корпоративної Persona лишається відкритою; це правило фіксує межу ефекту блокування, а не право компанії на всі ідентичності людини.

### REQ-ID-015 — Обов'язковий матеріал відновлення автономної Persona

При створенні автономної Persona клієнт `MUST` створити двокомпонентний owner-held recovery kit: випадковий secret і версійний зашифрований bundle з окремою recovery authority та необхідними data-key envelopes, але без приватних signing keys звичайних Devices. UI `MUST` запропонувати зберегти secret і bundle **окремо від поточного пристрою та один від одного** і вимагати підтвердження користувача перед завершенням setup. UI `MUST` пояснювати, що підтвердження не доводить наявності копій; втрата всіх пристроїв разом із будь-якою потрібною частиною комплекту може бути незворотною. Recovery secret `MUST NOT` автоматично передаватися федеративному вузлу чи вбудовуватися в bundle. Для повернення історії `MUST` існувати ще доступна encrypted resource copy. Byte format, crypto/KDF/nonce profile, current-controller proof, rotation і restore validation лишаються відкритими в `OQ-0024`.

Підтверджене 2026-09-26 правило VIDA: до надійного запису підтвердження Persona і Personal Space залишаються `setup_pending`; захищену роботу, зокрема створення нотаток, `MUST NOT` дозволяти. Після переривання setup клієнт `MUST` відновити ті самі Persona/Space IDs і recovery material без непомітної генерації заміни. Лише після підтвердження `MAY` повідомляти `PersonaCreated` і відкривати Personal Space. Це продуктове рішення VIDA, а не вимога OWASP чи поведінка, доведена для Delta Chat.

### REQ-ID-016 — Додаткові місця зберігання recovery

Клієнт `MUST` дозволяти власникові portable export зашифрованого bundle без secret; `MAY` пропонувати додаткову копію bundle у вибраному користувачем сховищі ОС або хмарному сховищі, зокрема на Windows/Android. Такий канал `MUST NOT` бути єдиним документованим способом відновлення або непомітним escrow приватних ключів вузлом; доступність bundle і encrypted resources після втрати пристрою `MUST` перевірятися окремо для кожної платформи. Відновлення виданого компанією акаунта є незалежним процесом і `MUST NOT` відкривати приватну Persona.

### REQ-ID-017 — Чесна межа анонімності

Автономна Persona `MUST` створюватися без реєстрації на зовнішньому сервері та `MAY` не мати прив'язки до публічної особи. Product/UI claims `MUST NOT` називати це повною мережевою невідстежуваністю: peers, relay, push/provider або мережевий оператор можуть бачити transport metadata, визначені відповідним profile. Телеметрія й журнали `MUST NOT` автоматично корелювати таку Persona з іншими Personas користувача.

### REQ-ID-018 — Push boundary для anonymous/public Persona

`Autonomous anonymous` Persona `MUST NOT` реєструвати push token, стабільний wake handle або call-routing binding у Apple, Google чи іншого зовнішнього push provider. Повідомлення та вхідний дзвінок для неї можуть бути отримані лише коли сумісний пристрій безпосередньо reachable; UI `MUST` чесно показувати цю межу. Push `MAY` бути ввімкнений лише для явно активованого `Public` profile після preview metadata exposure та окремої згоди на кожному device. Push registration, keys і logs `MUST` бути Persona-scoped та `MUST NOT` корелювати або повторно використовувати anonymous Persona context.

## Acceptance evidence

- offline-first onboarding без server registration;
- private federation без directory listing;
- opt-in publication preview і audit event;
- node migration зі стабільною Persona;
- multiple devices з незалежним revoke;
- automated schema check, що забороняє canonical three-value `AccountType`;
- metadata test не знаходить спільного parent identifier між anonymous contexts.
- customer-facing anonymous-mode text не обіцяє network untraceability і називає допустимі metadata boundaries.
- окремий корпоративний акаунт показує тільки доступні йому Spaces; після його блокування цей доступ зникає, але незалежні акаунти/Spaces користувача продовжують працювати.
- нова автономна Persona вимагає окремого збереження secret і encrypted bundle; тест втрати всіх пристроїв розрізняє authority restore за комплектом і Note-content restore лише за додатковою доступною encrypted resource copy.
- перерваний setup повертає ту саму `setup_pending` Persona; створення захищеної нотатки до підтвердження відхиляється, а `PersonaCreated` з'являється лише після надійного запису підтвердження.

## Deferred requirements

До окремих ADR/prototypes відкладено: DID method, byte/crypto format погодженої causal controller history та proof reconciliation/freshness, contextual Persona granularity, recovery cryptosystem, alias registry governance та node metadata visibility.
