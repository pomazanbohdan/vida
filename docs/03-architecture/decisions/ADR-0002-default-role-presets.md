---
id: ADR-0002
status: accepted
last_updated: 2026-09-18
source_refs:
  - ./ADR-0001-layered-access-control.md
  - ../../02-requirements/access-control-requirements.md
  - https://linear.app/docs/members-roles
  - https://linear.app/docs/private-teams
  - https://www.notion.com/help/sharing-and-permissions
  - https://www.notion.com/help/whos-who-in-a-workspace
  - https://docs.github.com/en/organizations/managing-user-access-to-your-organizations-repositories/managing-repository-roles/repository-roles-for-an-organization
  - https://support.monday.com/hc/en-us/articles/360019222479-Permissions-on-monday-com
supersedes: []
superseded_by: []
---

# ADR-0002: Стартовий каталог ролей

## Context

ADR-0001 прийняв layered access control: role preset, inherited permission matrix на app/container і виняткові resource overrides. Потрібен стартовий каталог ролей, достатній для персональних, командних, клієнтських і корпоративних Spaces без десятків майже однакових варіантів.

## Research summary

- Linear розділяє Workspace Owner, Admin, Team Owner, Member і Guest; Guest обмежується явно наданими teams.
- Notion відокремлює membership від доступу до сторінки та використовує `Full access`, `Can edit`, `Can edit content`, `Can create`, `Can comment`, `Can view`.
- GitHub використовує зрозумілу драбину `Read → Triage → Write → Maintain → Admin`.
- monday.com поєднує Owner, Editor, Contributor, Assigned Contributor і Viewer з action categories та granular board/column permissions.

Спільний сильний патерн: невелика role ladder, окремий зовнішній membership boundary і точніші permissions нижче за ієрархією.

## Decision drivers

- ролі мають пояснюватися без документації;
- governance не можна змішувати з редагуванням контенту;
- зовнішній Guest не повинен бути окремим синонімом Viewer;
- presets мають покривати більшість випадків без ручної matrix;
- custom roles і overrides мають закривати нестандартні випадки.

## Decision

### 1. Membership class окремо від role

Vida `SHOULD` розділяти:

- `Member` — звичайний учасник Space;
- `Guest` — зовнішній або обмежений учасник лише явно наданих scopes;
- `ServicePrincipal` — bot, agent або integration з explicit capabilities.

`Guest` не є рівнем редагування. Guest може отримати роль Contributor, Commenter або Viewer у дозволеному scope. Guest `MUST NOT` бути Owner або Admin і `MUST NOT` отримувати global discovery поза наданими scopes.

### 2. Шість human role presets

| Role | Призначення | Стартові права |
|---|---|---|
| `Owner` | Остаточне володіння Space | Усі capabilities; ownership transfer; security, keys, audit/export; видалення Space |
| `Admin` | Операційне адміністрування | Members, roles, apps, policies, schemas, settings і весь content; без transfer/delete Space за замовчуванням |
| `Manager` | Керування роботою в призначеному scope | Structure, workflows, projects, resources, assignments і moderation; без Space security та role administration |
| `Contributor` | Звичайна активна робота | Read; create; update own/assigned; comment; react; chat; без structure, policies та видалення чужого content |
| `Commenter` | Обговорення без зміни основного content | Read; comment; react; chat у доступних каналах |
| `Viewer` | Споживання інформації | Read only; без comment, chat send, edit, share та export за замовчуванням |

### 3. Scope ролі

- `Owner` і `Admin` є Space-level roles.
- `Manager`, `Contributor`, `Commenter` і `Viewer` `SHOULD` призначатися на Space, AppInstance або Container.
- Одна людина `MAY` мати різні ролі в різних containers.
- Effective permission обчислюється за ADR-0001; роль не обходить hard deny.

### 4. Custom roles

- Owner/Admin `SHOULD` створювати custom role через clone-and-edit існуючого preset.
- Custom role `MUST` мати stable ID, name, description, base preset і version.
- Custom role `MUST NOT` перевищувати maximum capabilities того, хто її створює або призначає.
- Built-in presets `MUST NOT` редагуватися; їх можна лише клонувати.

### 5. Invite UX

Invite flow `SHOULD` вимагати:

1. membership class;
2. scope;
3. role preset;
4. optional permission adjustments;
5. preview effective access до надсилання invitation.

### 6. Видалення Contributor-owned drafts

`Contributor` `MAY` видалити resource лише тоді, коли одночасно виконані всі умови:

- Contributor є автором/власником resource;
- resource має стан `draft` або schema-defined еквівалент;
- resource не опубліковано;
- resource не призначено іншому учаснику;
- resource не увійшов до активного workflow або approval;
- container policy не містить deny.

Після виходу зі стану draft Contributor `MUST NOT` виконувати hard delete. Доступні дії: archive, withdraw або deletion request до Manager/Admin — відповідно до schema policy. Видалення `MUST` залишати audit event і, де можливо, підтримувати recovery window.

### 7. Cardinality Owners

- Personal Space `MUST` мати рівно одного Owner — власника персонального облікового контексту.
- Shared, team, project, organization, community, city і business Space `MUST` мати щонайменше одного Owner та `MAY` мати кількох Owners.
- Система `MUST NOT` дозволяти remove, demote, revoke або leave для останнього Owner shared Space без одночасного призначення наступника.
- Додавання, видалення та передавання Owner role `MUST` створювати audit event.
- Втрата доступу до Personal Space вирішується recovery-механізмом identity/device, а не додаванням другого Owner до персональних даних.

## Why this is balanced

- Менше ролей прибирає Manager або Commenter і змушує постійно редагувати matrix.
- Більше ролей дублює permissions та створює незрозумілий каталог.
- Шість presets дають два governance-рівні, один delegated-management рівень і три collaboration-рівні.
- Guest як membership class усуває комбінації на кшталт окремих `Guest Viewer`, `Guest Commenter` і `Guest Editor` roles.

## Consequences

- UI має показувати membership class і role окремо.
- Guest access завжди scope-bound.
- Service principals не використовують human presets.
- Apps розширюють actions, але не створюють паралельну систему ролей.
- Назви presets мають бути локалізовані, а internal IDs — стабільні.

## Risks і mitigations

- `Manager` може бути надто широким → завжди scope-bound і без security/role administration.
- `Contributor` може випадково змінити чужі дані → default лише own/assigned; matrix розширює явно.
- Guest може побачити global data → deny global discovery та explicit scope grants.
- Custom-role sprawl → clone from preset, usage count, archive замість delete, versioned changes.

## Acceptance evidence

- role catalog UX test без попереднього навчання;
- Guest+Contributor бачить лише granted scope;
- Contributor не керує structure або policies;
- Contributor видаляє лише власний незалучений draft; після публікації, assignment або workflow backend відхиляє delete;
- Manager керує project workflow, але не Space security;
- Admin не може transfer/delete Space без explicit Owner grant;
- Personal Space завжди має одного Owner; shared Space ніколи не залишається без Owner;
- custom role не перевищує assigner's maximum capabilities;
- effective-access preview відповідає backend authorization result.
