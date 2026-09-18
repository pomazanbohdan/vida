---
id: PROD-COMPOSITION-001
status: approved
last_updated: 2026-09-18
source_refs:
  - ./v1-replacement-bundle.md
  - ../../research/vida-current-architecture.md
  - ../../research/Новий Text Document (6).txt
  - ../../research/Новий Text Document (10).txt
  - ../../research/Новий Text Document (12).txt
  - ../../_bmad-output/planning-artifacts/prfaq-vida.md
decision_refs: []
---

# Композиційна модель Vida

## Прийняте рішення

Messenger, knowledge та work management — не три окремі застосунки й не послідовні етапи одного процесу. Це повторно використовувані функціональні можливості, які утворюють персональні та спільні контексти Vida.

`Space` є верхньою логічною межею ownership, membership, keys, policies, configuration та synchronization. `Project` зазвичай є контейнером усередині Space. Спеціалізований `project Space` допускається як окремий вид Space, коли весь простір належить одному проєкту.

## Життєвий цикл Space

1. Після створення облікового запису користувач має персональний Space.
2. Будь-який користувач може створити додатковий Space.
3. Створювач підписує `WorkspaceGenesis` і стає початковим `Owner`.
4. Новий Space початково є приватним.
5. Owner може зробити його shared, запросивши контакти та видавши їм ролі й обмежені grants.
6. Membership, roles, grants, revocations і key epochs належать до control plane Space.

Термін `Owner` є канонічним для керування Space. `Author` використовується для авторства конкретного ресурсу й не є синонімом Owner.

Personal Space має рівно одного Owner. Shared Space може мати кількох Owners, але не може залишитися без жодного Owner.

## Застосунки й схеми Space

Під час створення стандартного Space `MUST` формуватися базовий функціональний комплекс:

- Messenger;
- Knowledge/Notes;
- Projects/Tasks.

Кожна capability реалізується як schema-driven app package та конкретний `AppInstance` у Space. Пакет визначає schemas, relations, commands, workflows, permission declarations, UI і data ports. Конфігурація та додаткові apps можуть відрізнятися між Spaces.

## Контексти використання

### Персональний контекст

- приватні діалоги й приватні групові чати;
- персональні нотатки, документи та база знань;
- персональні задачі й проєкти;
- ресурси залишаються приватними, доки користувач явно не надасть доступ.

### Спільний робочий контекст

Спільний проєкт або робочий простір `MUST` поєднувати:

- учасників і permissions;
- прямі та групові комунікації;
- проєктні чати й тематичні обговорення;
- документи, нотатки та базу знань;
- задачі, статуси й інші елементи керування роботою.

Такий контекст є корпоративним середовищем на кшталт інтегрованого робочого простору, а не лише task tracker.

## Membership і permissions

Space `MUST` розділяти membership classes `Member`, `Guest`, `ServicePrincipal` і human role presets `Owner`, `Admin`, `Manager`, `Contributor`, `Commenter`, `Viewer`. Детальний каталог визначено в `../03-architecture/decisions/ADR-0002-default-role-presets.md`.

Роль є шаблоном початкових capabilities, а не єдиним джерелом авторизації. Grant може обмежувати:

- доступні scopes;
- дозволені operations;
- строк дії;
- конкретний app, container, resource type, resource, field або action.

Політики застосовуються ієрархічно:

```text
Platform → Space → AppInstance → Container/Channel → Dataset/Collection
→ ResourceType → Resource → Field → Action
```

Контейнер успадковує політики Space, але може мати точніші правила. Явний hard deny має перевагу. Наявність link між ресурсами не надає автоматичного доступу до пов'язаного ресурсу.

Учасник за замовчуванням може спілкуватися в доступних йому чатах. Права на notes, documents, tasks та інші schema-defined resources налаштовуються окремо за діями, щонайменше `create/read/update/delete`. Детальні нормативні вимоги містить `../02-requirements/access-control-requirements.md`.

## Контекстні обговорення

Комунікація `MUST` бути доступною не тільки у глобальному messenger, а й у контексті конкретного ресурсу:

- документ може мати окреме обговорення;
- задача може мати окремий real-time chat;
- проєкт може мати загальні й тематичні чати;
- документ і задача можуть бути явно пов'язані;
- повідомлення, документ, задача й проєкт `MUST` посилатися одне на одного без копіювання змісту.

## Knowledge у проєкті

Кожен спільний проєкт `MUST` мати власний knowledge context: документи, нотатки, сторінки або інші матеріали, доступні відповідно до permissions проєкту.

Knowledge не є зовнішнім сховищем, прикріпленим до project management. Воно є повноправною частиною того самого робочого контексту.

## Наслідки для реалізації

- chat, document, task і project мають бути адресованими ресурсами;
- discussion має бути повторно використовуваною capability, яку можна прив'язати до різних ресурсів;
- permissions мають враховувати owning context і можливі явні винятки;
- search, notifications, relations, attachments і sync мають працювати між усіма типами ресурсів;
- клієнти `MUST NOT` реалізовувати окремі несумісні моделі чатів для messenger, project, document і task.
- кожен ресурс `MUST` мати один `OwnerSpaceId`, навіть якщо він підключений до інших Spaces через link, mount, projection або data contract;
- invitation `MUST` завершуватися створенням revocable Grant, а не передаванням довгоживучого універсального ключа;
- Space є security/sync boundary, але не обов'язково UX boundary: global Messages, Today, Search і Activity можуть агрегувати дозволені ресурси з кількох Spaces без руйнування ізоляції.

## Підтвердження в research

- `vida-current-architecture.md`: Space є межею володіння, членства, ключів, політик і конфігурації; містить projects, channels, forums, private areas та app instances.
- `Новий Text Document (6).txt`: Account → global views/workspace context → installed apps → resources; Workspace визначає ownership, security і synchronization, але не обов'язково межу UX.
- `Новий Text Document (10).txt`: `WorkspaceGenesis` містить owner key/signature; grants задають role, scopes та operations; invitations створюють Grant і key envelopes.
- `Новий Text Document (12).txt`: capability delegation і fine-grained resource/action permissions доповнюють RBAC/ABAC.

## Прийнята access-control модель

Під час додавання учасника Owner призначає role preset. Права деталізуються matrix на рівні app/container за resource types та actions. Для конкретного resource дозволений винятковий override. Див. `../03-architecture/decisions/ADR-0001-layered-access-control.md`.
