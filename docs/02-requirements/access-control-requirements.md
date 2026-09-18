---
id: REQ-ACCESS-001
status: approved
last_updated: 2026-09-18
source_refs:
  - ../01-product/composable-workspace-model.md
  - ../../research/vida-current-architecture.md
  - ../../research/Новий Text Document (10).txt
  - https://linear.app/docs/members-roles
  - https://linear.app/docs/private-teams
  - https://linear.app/docs/conceptual-model
  - https://support.monday.com/hc/en-us/articles/360019222479-Permissions-on-monday-com
  - https://support.monday.com/hc/en-us/articles/360019180359-Workspace-permissions
decision_refs:
  - ../03-architecture/decisions/ADR-0001-layered-access-control.md
  - ../03-architecture/decisions/ADR-0002-default-role-presets.md
  - ../03-architecture/decisions/ADR-0003-offline-revocation.md
---

# Вимоги до access control

## Підтверджені вимоги

### REQ-ACL-001 — Права за контекстом

Додавання контакту до shared Space `MUST NOT` автоматично надавати однаковий повний доступ до всіх даних Space.

### REQ-ACL-002 — Базова комунікація

Учасник `MUST` мати можливість спілкуватися в доступних йому чатах, якщо конкретна політика чату не забороняє надсилання повідомлень.

### REQ-ACL-003 — Матриця дій

Owner або уповноважений адміністратор `MUST` мати змогу визначати права щонайменше на:

- `create`;
- `read`;
- `update`;
- `delete`.

Схеми можуть додавати доменні actions: `comment`, `share`, `assign`, `approve`, `publish`, `moderate`, `export`, `manage_structure`, `manage_members` та інші.

### REQ-ACL-004 — Різні права для різних capabilities

Один учасник може одночасно:

- писати в доступних чатах;
- читати notes без права редагування;
- створювати notes без права видалення;
- редагувати tasks лише у визначеному project/container;
- не мати доступу до інших apps, containers або resources.

### REQ-ACL-005 — Schema-driven authorization

Permission model `MUST` працювати не лише з наперед зашитими Messenger, Notes і Projects. App schema `MUST` декларувати власні resource types та actions, до яких застосовується спільний policy engine.

### REQ-ACL-006 — Membership class і role

Система `MUST` зберігати membership class окремо від role. Канонічні membership classes v1: `Member`, `Guest`, `ServicePrincipal`.

`Guest` `MUST` бути scope-bound і може отримати `Contributor`, `Commenter` або `Viewer`, але не `Owner` чи `Admin`. `ServicePrincipal` використовує explicit capabilities, а не human role preset.

### REQ-ACL-007 — Built-in role presets

Vida v1 `MUST` надавати незмінні built-in presets: `Owner`, `Admin`, `Manager`, `Contributor`, `Commenter`, `Viewer`. Custom role створюється клонуванням preset та `MUST NOT` перевищувати maximum capabilities суб'єкта, який її створює або призначає.

### REQ-ACL-008 — Видалення власного draft

`Contributor` `MAY` видалити власний resource, лише доки він залишається draft, не опублікований, не призначений іншому учаснику та не включений в активний workflow/approval. Після цього система `MUST` запропонувати archive, withdraw або deletion request замість hard delete.

Кожне видалення `MUST` створювати audit event. Schema/container policy `MAY` повністю заборонити Contributor delete.

### REQ-ACL-009 — Кількість Owners

Personal Space `MUST` мати рівно одного Owner. Shared Space `MUST` мати одного або більше Owners. Система `MUST NOT` залишати shared Space без Owner і `MUST` аудитувати кожну зміну ownership.

### REQ-ACL-010 — Момент набуття чинності revocation

Revocation `MUST` вважатися effective лише після прийняття authority відповідного scope. UI `MUST` розрізняти `pending` та `effective` revocation. Нормативний порядок визначає `acceptedControlSequence`, а не client timestamp.

### REQ-ACL-011 — Control plane перед data plane

Під час reconnect peer `MUST` синхронізувати membership, grants, revocations і current key epochs до domain-data sync. Peer без актуального control head `MUST NOT` отримувати нові scope data або надсилати authority-accepted mutations.

### REQ-ACL-012 — Перевірка operation і quarantine

Кожна operation `MUST` містити `grantId`, `scopeId`, `policyVersion` і `keyEpoch`; executor `MUST` перевіряти їх під час прийняття. Operation, не прийнята до effective revocation, `MUST` бути відхилена незалежно від client timestamp та збережена локально в `RevokedOperationQuarantine` з export/copy/request-reapply recovery actions; вона `MUST NOT` автоматично merge-итися назад.

### REQ-ACL-013 — Scope key epoch rotation

Effective revocation `MUST` змінювати `keyEpoch` affected scopes. Нові operations, snapshots і attachments `MUST` використовувати новий epoch key, доступний лише чинним members/devices. Старий key `MUST NOT` використовуватися для нових записів.

### REQ-ACL-014 — Межа гарантії revocation

Vida `MUST` пояснювати, що revocation припиняє майбутній доступ, але не може дистанційно стерти вже отримані plaintext, screenshots, exports або ciphertext разом із ключами.

### REQ-ACL-015 — Повторне надання доступу

Re-grant після revocation `MUST` створювати новий `grantId`, нові key envelopes і прив'язку до current epoch. Відкликаний grant `MUST NOT` реактивуватися.

### REQ-ACL-016 — Offline-доступ до синхронізованих даних

Повністю синхронізовані локальні дані `MUST` бути доступні без мережі за замовчуванням у всіх Apps. Offline lease `MUST` обмежувати лише вік control state та можливість подальшого authority acceptance для offline mutations, а не локальне читання вже отриманих даних. Platform задає maximum; Space або protected scope `MAY` його скоротити чи заборонити local persistence, але `MUST NOT` збільшувати.

### REQ-ACL-017 — Authority для змін доступу

У Personal Space зміну доступу `MUST` ініціювати Owner; у shared Space — Owner або Admin із `manage_members` у відповідному scope. Authority service `MUST` повторно перевірити права, записати команду до control log і підписати результат. За замовчуванням одна чинна авторизація є достатньою; critical policy `MAY` вимагати `M-of-N` quorum до набуття зміною чинності.

### REQ-ACL-018 — Offline mutation acceptance windows

Maximum window без оновлення control state `MUST` становити: `standard` — 30 днів, `protected` — 7 днів, `critical` — 24 години. Membership, role, permission, policy та key-management mutations `MUST` проходити online authority validation. Прострочена offline mutation `MUST` переходити до `RevokedOperationQuarantine`.

## Галузеві орієнтири

### Linear

- workspace roles: Owner, Admin, Member, Guest;
- delegated Team Owner;
- Guest бачить лише явно надані teams, але всередині них переважно діє як Member;
- private teams обмежують видимість issues, projects і documents;
- окремий issue можна share поза team без відкриття всього team;
- projects об'єднують issues, documents, milestones, views, updates і кілька teams;
- workflows, cycles і частина permissions належать team.

Linear є сильним орієнтиром для швидкої project execution model, але його permission model грубіша за потрібну Vida.

### monday.com

- permissions мають рівні account, workspace, board, column, dashboard і document;
- board roles включають Owner, Editor, Contributor, Assigned Contributor і Viewer;
- action categories окремо охоплюють items, subitems, updates, columns, groups, views і forms;
- private/shareable content визначає visibility до застосування action permissions;
- custom roles наслідують базову роль і можуть додатково обмежуватися.

monday.com ближчий до цільової granularity Vida, але Vida має узагальнити модель на довільні schema-defined resources і actions.

## Прийнята модель

Щоб Owner не налаштовував сотні прапорців вручну:

1. role preset задає стартовий набір capabilities;
2. policy успадковується вниз від Space;
3. Owner бачить і може змінити permission matrix;
4. override уточнює права для app, container, schema/resource type або конкретного resource;
5. явний hard deny має перевагу;
6. UI показує effective permissions і джерело кожного правила;
7. backend повторно перевіряє кожну command незалежно від UI.

## Основна granularity UX

Owner відкриває permission matrix у контексті app або container, наприклад `Knowledge у Project X`. Рядки матриці є schema/resource types, колонки — actions. Конкретний resource отримує override лише як виняток. Нормативна precedence та enforcement визначені в `ADR-0001`.

Стартові membership classes і role presets визначені в `ADR-0002`.
