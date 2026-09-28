---
id: REQ-ACCESS-001
status: approved
last_updated: 2026-09-22
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

Vida v1 `MUST` надавати незмінні built-in presets: `Owner`, `Admin`, `Manager`, `Contributor`, `Commenter`, `Viewer`. Custom role створюється клонуванням preset та `MUST NOT` перевищувати maximum capabilities суб'єкта, який її створює або призначає. Custom role, навіть клон `Owner`, `MUST NOT` отримувати built-in Owner-governance статус або зараховуватися як Owner Space.

### REQ-ACL-008 — Видалення власного draft

`Contributor` `MAY` видалити власний resource, лише доки він залишається draft, не опублікований, не призначений іншому учаснику та не включений в активний workflow/approval. Після цього система `MUST` запропонувати archive, withdraw або deletion request замість hard delete.

Кожне видалення `MUST` створювати audit event. Schema/container policy `MAY` повністю заборонити Contributor delete.

### REQ-ACL-009 — Кількість Owners

Personal Space `MUST` мати рівно одного Owner. Shared Space `MUST` мати одного або більше Owners. Система `MUST NOT` залишати shared Space без Owner і `MUST` аудитувати кожну зміну ownership.

Кожен чинний Owner shared Space `MUST` мати право одноосібно видалити всіх інших Owners зі Space без їхнього схвалення. Це `MUST` повністю припиняти їхнє membership, усі Space grants і делегації, похідні від цього membership, а не понижувати роль. Кожна зміна `MUST` зберігати щонайменше одного Owner. Admin `MUST NOT` видаляти або понижувати Owner, навіть із `manage_members`.

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

Після отримання effective membership removal сумісний клієнт `MUST` припинити показ і доступ через усі свої керовані поверхні до локальних даних видаленого Space, інвалідувати Space grants/keys і очистити керований кеш настільки, наскільки дозволяє платформа. Офлайн-пристрій до отримання revocation, вже показані системні банери та раніше зроблені копії не входять у гарантію негайного приховування.

### REQ-ACL-015 — Повторне надання доступу

Re-grant після revocation `MUST` створювати новий `grantId`, нові key envelopes і прив'язку до current epoch. Відкликаний grant `MUST NOT` реактивуватися.

### REQ-ACL-016 — Offline-доступ до синхронізованих даних

Повністю синхронізовані локальні дані Personal Space `MUST` залишатися доступними без мережі за замовчуванням, доки локальний identity context активний; logout/local lock лишаються окремими правилами. У shared Space всі Apps і керовані поверхні `MUST` застосовувати єдиний, незалежний від класу ризику, інтервал узгодження прав `rightsReconciliationInterval = 7 діб` із authority-confirmed control state. До спливу цього інтервалу втрата зв’язку сама собою `MUST NOT` приховувати вже відкритий документ або вимагати нового online-підтвердження для відкриття синхронізованого `critical` вмісту. Після 7 діб без успішного authority-confirmed узгодження сумісний клієнт `MUST` блокувати читання кешу shared Space, включно з уже відкритим вмістом, до нового підтвердження прав. Отримане effective revocation `MUST` блокувати доступ негайно; локальний lock/logout і явна заборона local persistence залишаються окремими правилами.

Узгодження прав `MUST` охоплювати UI, runtime/API, локальні проєкції та plugin/automation reads: ні effective revocation, ні сплив семиденного інтервалу без підтвердження не можна обходити через інший шлях читання. Перезапуск, відкат годинника, старий control head або локальна активність `MUST NOT` удавати нове authority-confirmed узгодження. Семиденний інтервал перевірки прав для *читання* не є строком придатності offline mutation candidate з `REQ-ACL-018`; точний міжплатформний proof/anti-rollback — `OQ-0053`.

### REQ-ACL-017 — Authority для змін доступу

У Personal Space зміну доступу `MUST` ініціювати Owner; у shared Space звичайну зміну non-Owner доступу — Owner або Admin із відповідним Space-level `manage_members` для Space-role assignment. Authority service `MUST` повторно перевірити права в момент прийняття, записати команду до control log і підписати результат. За замовчуванням одна чинна авторизація є достатньою; critical policy `MAY` вимагати `M-of-N` quorum до набуття зміною чинності.

Повне видалення іншого Owner чинним Owner shared Space є винятком: `M-of-N` або згода інших Owners `MUST NOT` вимагатися. Admin `MUST NOT` ініціювати Owner removal чи demotion. Лише чинний Owner shared Space `MAY` додати іншого Owner; Admin зі Space-level `manage_members` `MAY` призначати Admin і нижчі ролі, але `MUST NOT` призначати Owner або підвищувати себе до Owner. Personal Space `MUST NOT` мати другого одночасного Owner. Це не скасовує перевірки authority і правила, що shared Space не може лишитися без Owner.

### REQ-ACL-018 — Безстрокове очікування offline mutation candidate, перевірка при прийнятті

VIDA `MUST NOT` відхиляти, приховувати або поміщати в quarantine збережений offline mutation candidate **лише через його вік**: немає maximum 30 днів / 7 днів / 24 години за risk class. При reconnect authority/executor `MUST` перевірити чинні права актора для Space/об'єкта/дії, grant/epoch, signature, schema, причинні й бізнесові preconditions; candidate `MAY` бути прийнятий, якщо всі перевірки пройшли, інакше `MUST` залишитися recoverable без автоматичного merge. Чинний Owner свого Personal або shared Space може локально працювати без мережі необмежено за віком candidate, з урахуванням окремого семиденного shared-read lock з `REQ-ACL-016`; у Personal Space локальне прийняття визначає власна authority policy, а у shared Space навіть Owner не обходить її перевірку і не отримує автоматичного acceptance лише від часу чи доставки. Membership, role, permission, policy та key-management mutations `MUST` проходити authority validation перед набуттям чинності, хоча їх кандидати теж не мають age-only expiry. Конкретний rebase/re-sign і conflict contract лишається `OQ-0033`/`OQ-0034`; це не встановлює безстроковий security grant і не скасовує семиденну межу читання за `REQ-ACL-016`.

### REQ-ACL-019 — Нова чернетка після блокування shared-read

Після спливу семи діб без підтвердження прав клієнт `MUST` дозволяти створити **новий** локальний draft/candidate у shared Space, якщо це не потребує читання заблокованого вмісту. Така чернетка `MUST` залишатися локальною й pending; вона `MUST NOT` набувати спільної чинності або відкривати старі дані до нового rights reconciliation та перевірки права `create`, grant/epoch і schema preconditions. Раніше durable-committed pending work `MUST NOT` втрачатися через read lock. Owner власного Personal Space не потребує зовнішнього підтвердження своїх прав; Owner **shared** Space підпадає під той самий семиденний read lock, що й інші учасники (`OQ-0063` закрито).

### REQ-ACL-020 — Resource-scoped sharing із Personal Space

Owner Personal Space `MUST` мати змогу надати контакту доступ до конкретного resource або container, наприклад однієї нотатки чи розділу нотаток, без доступу до решти Personal Space. Core `MUST` представити такого отримувача scope-bound `Guest` membership і explicit grant, застосувати звичайні key epoch/revocation rules та заборонити inference через search, backlinks, attachments або notifications до ресурсів поза grant scope.

Клієнт `MUST` показати точний поширений scope і effective permissions до підтвердження share. Такий share `MUST NOT` неявно перетворювати Personal Space на Shared Space. Для спільного командного Project клієнт `MUST` створити або використати окремий Shared Space.

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
