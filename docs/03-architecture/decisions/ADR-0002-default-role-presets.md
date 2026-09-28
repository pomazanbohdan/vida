---
id: ADR-0002
status: accepted
last_updated: 2026-09-20
source_refs:
  - ./ADR-0001-layered-access-control.md
  - ../../02-requirements/access-control-requirements.md
  - https://linear.app/docs/members-roles
  - https://linear.app/docs/private-teams
  - https://www.notion.com/help/sharing-and-permissions
  - https://www.notion.com/help/whos-who-in-a-workspace
  - https://docs.github.com/en/organizations/managing-user-access-to-your-organizations-repositories/managing-repository-roles/repository-roles-for-an-organization
  - https://support.monday.com/hc/en-us/articles/360019222479-Permissions-on-monday-com
  - https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html
  - https://cheatsheetseries.owasp.org/cheatsheets/Business_Logic_Security_Cheat_Sheet.html
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
| `Admin` | Операційне адміністрування | Members і ролі нижче Owner, apps, operational policies, schemas, settings і весь content; без Owner-governance policy, Owner-removal/transfer/delete Space |
| `Manager` | Керування роботою в призначеному scope | Structure, workflows, projects, resources, assignments і moderation; без Space security та role administration |
| `Contributor` | Звичайна активна робота | Read; create; update own/assigned; comment; react; chat; без structure, policies та видалення чужого content |
| `Commenter` | Обговорення без зміни основного content | Read; comment; react; chat у доступних каналах |
| `Viewer` | Споживання інформації | Read only; без comment, chat send, edit, share та export за замовчуванням |

### 3. Scope ролі

- `Owner` і `Admin` є Space-level roles.
- `Manager`, `Contributor`, `Commenter` і `Viewer` `SHOULD` призначатися на Space, AppInstance або Container.
- Одна людина `MAY` мати різні ролі в різних containers.
- Effective permission для app/resource actions обчислюється за ADR-0001; роль не обходить hard deny. Space-governance права чинного Owner призначити іншого Owner або видалити іншого Owner з п. 7 не є resource actions і не вимикаються жодним deny ресурсної permission matrix.

### 4. Custom roles

- Owner/Admin `SHOULD` створювати custom role через clone-and-edit існуючого preset.
- Custom role `MUST` мати stable ID, name, description, base preset і version.
- Custom role `MUST NOT` перевищувати maximum capabilities того, хто її створює або призначає.
- Built-in presets `MUST NOT` редагуватися; їх можна лише клонувати.
- Clone від preset `Owner` `MUST NOT` надавати Space-governance статус Owner, будь-які reserved Owner-governance дії (призначення/видалення Owner, ownership transfer, керування Owner-governance policy, key authority або видалення Space) чи враховуватися в Owner cardinality. Ці дії має лише explicit built-in `Owner` у чинному підписаному Space control state; клон може отримати тільки явно дозволені non-governance capabilities.

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
- Початковий Owner нового Space визначається підписаним `WorkspaceGenesis` створювача, а не `RoleAssigned` від неіснуючого попереднього Owner; подальші призначення підпорядковуються правилам нижче.
- Кожен чинний Owner shared Space `MAY` одноосібно видалити будь-якого іншого Owner зі Space; згода чи голосування інших Owners для цього `MUST NOT` вимагатися. Це повне припинення Space membership, а не автоматичне пониження до нижчої ролі.
- Лише чинний Owner shared Space `MAY` додати нового Owner; це не дозволяє створити другого Owner у Personal Space. Owner `MAY` призначати також Admin і нижчі ролі; Admin із Space-level `manage_members` `MAY` додавати учасників із роллю Admin або нижчою, але `MUST NOT` призначати Owner чи підвищувати себе або іншого учасника до Owner. Перенесення власності Personal Space — окрема atomic replacement/recovery operation, не додавання другого Owner.
- Лише чинний Owner `MAY` ініціювати видалення іншого Owner. Admin `MUST NOT` видаляти чи понижувати Owners, навіть із `manage_members`; зміна назви команди або governance policy `MUST NOT` обходити цю межу. Admin `MUST NOT` редагувати Owner-governance policy через загальне право на operational settings.
- Будь-який шлях, що позбавляє учасника статусу Owner (відкликання/заміна ролі, demotion або видалення membership), `MUST` проходити ту саму Space-level перевірку; в прийнятій моделі успішний результат для іншого Owner — повне видалення membership.
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
- Один Owner може одноосібно усунути інших Owners → це свідоме спрощення governance, а не захист від зловмисного співвласника; потрібні audit та підтвердження authority, але не co-owner veto.
- Custom-role sprawl → clone from preset, usage count, archive замість delete, versioned changes.

## Acceptance evidence

- role catalog UX test без попереднього навчання;
- Guest+Contributor бачить лише granted scope;
- Contributor не керує structure або policies;
- Contributor видаляє лише власний незалучений draft; після публікації, assignment або workflow backend відхиляє delete;
- Manager керує project workflow, але не Space security;
- Admin не може передати ownership або видалити Space як обхід Owner-only governance;
- Personal Space завжди має одного Owner; shared Space ніколи не залишається без Owner;
- чинний Owner shared Space може видалити всіх інших Owners без їхнього схвалення, зберігаючи щонайменше одного Owner та audit trail; колишні Owners не зберігають membership або grants цього Space;
- лише чинний Owner може призначити нового Owner; Admin може призначити Admin або нижчу роль, але не Owner, зокрема не собі;
- Admin може додавати/керувати non-Owner учасниками, але не може прямо чи через іншу команду видалити або понизити Owner;
- custom role не перевищує assigner's maximum capabilities;
- custom role, навіть клон `Owner`, не може видаляти Owner і не задовольняє інваріант щонайменше одного Owner;
- effective-access preview відповідає backend authorization result.

## OWASP cross-check (2026-09-20)

[OWASP Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) радить deny-by-default і перевірку повноважень на кожному запиті; [OWASP Business Logic Security Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Business_Logic_Security_Cheat_Sheet.html) окремо наголошує на перевірці поточного owner/role та допустимості адміністративно заданої ролі. Тому authority `MUST` повторно перевіряти built-in Owner-статус у момент прийняття `RoleAssigned(Owner)` і відхиляти stale invitation, replay та спробу Admin передати собі роль Owner. Саме правило «лише Owner може призначити Owner» є рішенням VIDA, а не буквальним приписом OWASP.
