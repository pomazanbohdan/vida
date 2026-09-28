---
id: PROD-COMPOSITION-001
status: approved
last_updated: 2026-09-22
source_refs:
  - ./v1-replacement-bundle.md
  - ../../research/vida-current-architecture.md
  - ../../research/Новий Text Document (6).txt
  - ../../research/Новий Text Document (10).txt
  - ../../research/Новий Text Document (12).txt
  - ../../_bmad-output/planning-artifacts/prfaq-vida.md
decision_refs:
  - ADR-0007
  - ADR-0008
  - ADR-0009
  - ADR-0010
  - ADR-0011
---

# Композиційна модель Vida

## Прийняте рішення

Messenger, knowledge та work management — не три окремі застосунки й не послідовні етапи одного процесу. Це повторно використовувані функціональні можливості, які утворюють персональні та спільні контексти Vida.

`Space` є верхньою логічною межею ownership, membership, keys, policies, configuration та synchronization. `Project` зазвичай є контейнером усередині Space. Спеціалізований `project Space` допускається як окремий вид Space, коли весь простір належить одному проєкту. Аналогічно складний опублікований App може створити власний dedicated Space: у користувацькому сенсі це «Space застосунку», але технічно він залишається звичайним Space із тими самими правилами ownership, membership, keys і sync.

## Життєвий цикл Space

1. Після створення облікового запису користувач має персональний Space.
2. Будь-який користувач може створити додатковий Space.
3. Створювач підписує `WorkspaceGenesis` і стає початковим `Owner`.
4. Новий Space початково є приватним.
5. Owner може зробити його shared, запросивши контакти та видавши їм ролі й обмежені grants.
6. Membership, roles, grants, revocations і key epochs належать до control plane Space.

Термін `Owner` є канонічним для керування Space. `Author` використовується для авторства конкретного ресурсу й не є синонімом Owner.

Personal Space має рівно одного Owner. Shared Space може мати кількох Owners, але не може залишитися без жодного Owner.

Owner Personal Space `MAY` надати іншій Persona scope-bound `Guest` access до конкретної нотатки, розділу нотаток або іншого resource/container без відкриття решти Personal Space. У користувацькому інтерфейсі це «поділитися нотаткою», хоча Core все одно створює перевірний membership/grant у межах OwnerSpaceId.

Перший персональний Project створюється в Personal Space. Запрошення учасників до командного Project `MUST` вести до явного створення окремого Shared Space або вибору вже наявного Shared Space; клієнт `MUST NOT` неявно перетворювати Personal Space на shared.

## Застосунки й схеми Space

Під час створення стандартного Space `MUST` формуватися базовий функціональний комплекс:

- Messenger;
- Knowledge/Notes;
- Projects/Tasks.

Кожна capability реалізується як schema-driven app package та конкретний `AppInstance` у Space. Пакет визначає schemas, relations, commands, workflows, permission declarations, UI і data ports. Конфігурація та додаткові apps можуть відрізнятися між Spaces.

### Dedicated Space складного застосунку

Складний App `MAY` під час активації запропонувати уповноваженому користувачу створити dedicated Space, у якому його головний `AppInstance` є продуктовим центром, а Messenger, Knowledge/Notes і Projects/Tasks підключаються як залежні `AppInstance`. Користувач підписує звичайний `WorkspaceGenesis` і стає початковим Owner; пакет або його Developer не стає security Owner. Наприклад, App «Міста України» може мати власний Space з міськими ресурсами, чатами мешканців, нотатками/документами та проєктними задачами. Залежні instances не копіюються у нові несумісні реалізації: вони використовують стандартні package contracts VIDA, а їхні ресурси, ролі й sync залишаються в межах цього dedicated Space.

Отже «Space = App» є правилом продуктової композиції для складного застосунку, а не твердженням, що будь-який технічний `AppInstance` завжди дорівнює окремому Space. Простий extension або додатковий інструмент може лишатися AppInstance в уже наявному Space.

## Доставлення пакета і активація застосунку

`AppPackage` є декларативним описом застосунку: schemas, relations, commands, workflows, permission declarations та UI/data ports звертаються до підтримуваних можливостей спільного VIDA runtime. Bundled Messenger, Knowledge/Notes і Projects/Tasks та окремо завантажені зовнішні пакети використовують той самий package contract; різниться спосіб доставлення, а не модель даних або прав. Сумісний зовнішній пакет можна підключити без нового релізу клієнта, якщо потрібні primitives уже є в runtime.

`AppInstance` є окремою активацією пакета в Space. Один пакет може мати різні instances у різних Spaces; ресурси, grants і sync не змішуються. Сам факт завантаження пакета не дає доступу до жодного Space чи ресурсу. Довільний код із прямим доступом до ОС або внутрішніх механізмів ядра не входить до цього v1 package contract. Нормативне рішення: [ADR-0007](../03-architecture/decisions/ADR-0007-declarative-app-packages.md); вимоги: [REQ-APP-PACKAGE-001](../02-requirements/app-package-requirements.md).

VIDA містить власний керований маркетплейс і дозволяє підключати зовнішні репозиторії пакетів. Обидва канали можуть поширювати застосунки й пакети-розширення базових застосунків; метадані репозиторію повідомляють про доступні нові версії. Discovery, отримання пакета та активація в Space — різні дії. Канал розповсюдження не надає виконуваній логіці додаткових прав. Нормативне рішення: [ADR-0008](../03-architecture/decisions/ADR-0008-package-distribution-channels.md).

Користувацький застосунок може визначати власну прикладну логіку над схемами та ресурсами, зокрема повторно використовувати Notes/Knowledge й через контрольований runtime додавати, замінювати або вимикати визначену поведінку базового застосунку. Це не дає плагіну влади над permissions, фізичним сховищем, ключами або sync. Iroh залишається транспортом; Rhai і Wasm/Wasmtime — досліджені, але не обрані виконавці цієї логіки. Нормативне рішення: [ADR-0009](../03-architecture/decisions/ADR-0009-managed-application-logic.md).

Така логіка діє лише в цільовому `AppInstance` конкретного Space (Workspace). Вона не переписує базовий застосунок і не змінює інші instances того самого або іншого Space. Встановлення пакета саме по собі не активує його логіку. Нормативне рішення: [ADR-0010](../03-architecture/decisions/ADR-0010-instance-scoped-app-logic.md).

Якщо процес застосунку й базовий handler використовують той самий прикладний тригер (наприклад, створення, зміна чи збереження), процес цільового `AppInstance` має пріоритет. Розробник застосунку компонує незалежні дії без окремого арбітражу Owner-а; це не скасовує перевірку доступу й приймання операцій ядром. Нормативне рішення: [ADR-0011](../03-architecture/decisions/ADR-0011-instance-trigger-precedence.md).

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

Проєктний/груповий контекст має два паралельні режими комунікації: безперервний chat stream для швидкої взаємодії та forum topics для структурованих довготривалих обговорень.

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
