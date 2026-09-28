---
id: REQ-APP-PACKAGE-001
status: approved
last_updated: 2026-09-23
source_refs:
  - ../01-product/composable-workspace-model.md
  - ../03-architecture/decisions/ADR-0007-declarative-app-packages.md
  - ../03-architecture/decisions/ADR-0008-package-distribution-channels.md
  - ../03-architecture/decisions/ADR-0009-managed-application-logic.md
  - ../03-architecture/decisions/ADR-0010-instance-scoped-app-logic.md
  - ../03-architecture/decisions/ADR-0011-instance-trigger-precedence.md
  - ../03-architecture/decisions/ADR-0012-command-event-sync-boundary.md
decision_refs:
  - ADR-0007
  - ADR-0008
  - ADR-0009
  - ADR-0010
  - ADR-0011
  - ADR-0012
---

# Вимоги до пакетів застосунків VIDA

| ID | Вимога | Критерій перевірки |
|---|---|---|
| REQ-APP-001 | VIDA `MUST` представляти застосунок як `AppPackage` зі schemas, relations, commands, workflows, permission declarations, UI/data ports і, за потреби, керованою прикладною логікою, що використовує підтримувані можливості runtime. | Тестовий пакет запускається на незміненому клієнті; невідома capability відхиляється явно. |
| REQ-APP-002 | Стандартний Space `MUST` отримувати підготовлені `AppInstance` Messenger, Knowledge/Notes і Projects/Tasks із пакетів, що постачаються разом із VIDA. `Підготовлений` означає, що instance має Space-scoped identity/configuration, але це саме по собі не вмикає його для користування, не показує в навігації й не надає grants. Під час першого запуску Personal Space користувач обирає, які з них увімкнути й показати; решту `MAY` увімкнути пізніше без повторного встановлення VIDA. | Новий стандартний Space має три підготовлені instances без hard-coded моделі прав/даних. Відкладена активація Notes не видаляє instance і не змінює Messenger; сам факт підготовки не відкриває ресурси. |
| REQ-APP-003 | Сумісний зовнішній package `MUST` завантажуватися й підключатися без нового релізу VIDA-клієнта, якщо всі його declarations підтримані runtime. | Пакет з новими формами й ресурсами підключається до незміненого release build. |
| REQ-APP-004 | Кожен `AppInstance` `MUST` мати Space-scoped configuration/data/permissions/sync; встановлення package або link `MUST NOT` саме надавати доступ до ресурсів. | Два Spaces використовують той самий package без витоку даних; direct forbidden command відхиляється. |
| REQ-APP-005 | Базові й зовнішні пакети `MUST` використовувати той самий версійний контракт семантики та conformance; presentation різних shell-профілів `MUST` зберігати однакові авторизовані outcomes. | Однакові golden scenarios проходять для bundled та external packages на підтримуваних профілях. |
| REQ-APP-006 | V1 `MUST NOT` виконувати довільний код пакета з прямим доступом до ОС, ключів, сховища або transport internals; керована прикладна логіка дозволена лише через схвалений runtime/host API. | Package loader відхиляє непідтримуване payload; обробник не отримує прямого доступу до системних ресурсів. |
| REQ-APP-007 | VIDA `MUST` постачати вбудований керований маркетплейс та `MUST` дозволяти підключати зовнішні репозиторії сумісних package. | Release build показує VIDA catalog і окремо підключає тестовий external repository. |
| REQ-APP-008 | Discovery `MUST` відрізняти application package від extension package («плагіна») і вказувати цільовий базовий застосунок для розширення. | Каталог відображає тип і target host тестових packages; невідомий target не активується мовчки. |
| REQ-APP-009 | Repository metadata `MUST` дозволяти виявити доступну нову версію package, зберігаючи origin/provenance; update discovery `MUST NOT` самостійно змінювати installed package чи `AppInstance`. | Публікація нового release змінює status на «оновлення доступне»; data/state активного instance незмінні. |
| REQ-APP-010 | Source connection, package discovery, package acquisition і Space activation `MUST` бути різними станами; жоден із перших трьох `MUST NOT` сам надавати grants на Space-ресурси. | Підключення недовіреного тестового source та отримання package не надає доступу до приватного Space. |
| REQ-APP-011 | App/extension package `MUST` уміти задавати прикладну логіку над schema-defined ресурсами та додавати, замінювати або вимикати визначену поведінку базового застосунку через explicit runtime contract. | Тестове розширення змінює названу дію Notes без зміни Rust core; невідома точка розширення відхиляється явно. |
| REQ-APP-012 | Усі дії й ефекти пакетної логіки `MUST` проходити trusted authorization, operation acceptance, data integrity та sync controls; рішення обробника `MUST NOT` саме надавати доступ. | Пряма заборонена команда з обробника відхиляється, навіть якщо UI або сам обробник вважає її дозволеною. |
| REQ-APP-013 | Активація і зміна прикладної поведінки `MUST` обмежуватися цільовим `AppInstance` у конкретному Space; встановлення/оновлення package `MUST NOT` змінювати базовий пакет або інші instances глобально. | Extension змінює Notes A у Space X; Notes B у Space X і Notes у Space Y лишаються незмінними. |
| REQ-APP-014 | На тому самому оголошеному прикладному тригері процес цільового `AppInstance` `MUST` мати пріоритет над базовим handler; композиція власних незалежних дій належить розробнику застосунку, без окремого Owner-level вибору чи платформного арбітражу бізнес-конфліктів. | Instance handler для збереження Notes отримує пріоритет; незалежна post-save дія працює за композицією розробника без діалогу вибору для Owner. |
| REQ-APP-015 | Runtime `MUST` відрізняти нову named business command від committed fact, timer/external signal та UI projection; отримання або replay синхронізованої операції `MUST NOT` повторно викликати command handler чи породжувати новий бізнес-намір. | Два пристрої отримують одну операцію `message.send`; дубль доставки й reconnect оновлюють projection, не створюючи другого повідомлення або ефекту. |
| REQ-APP-016 | VIDA `MUST` мати marketplace-level роль `Developer` для публікації та супроводу `AppPackage`. Вона `MUST` бути окремою від Space roles `Owner`/`Admin` і сама не надавати доступу до жодного `AppInstance` або даних користувачів. Та сама Persona `MAY` незалежно бути Developer у marketplace та Owner/Admin у конкретному Space. | Developer публікує пакет, але не може прочитати Space клієнта. Активувавши пакет у власному Space як його Owner, ця сама Persona керує instance лише через свою Space role. |
| REQ-APP-017 | Нова package/schema version `MUST NOT` активуватися, доки migration preflight і всі mandatory compatibility/conformance checks не завершилися успішно. Failure `MUST` лишати попередню версію активною без часткового переписування та активно показувати діагностичну помилку уповноваженому керівнику AppInstance. | Fixture з невалідним legacy record або браком місця не змінює active version/schema; Owner/Admin бачить actionable error. Успішна activation можлива лише після усунення причини й повторної повної перевірки. |
| REQ-APP-018 | `compatible-auto` update `MAY` змінювати UI, додавати optional/defaulted fields або оптимізувати managed logic лише якщо старі дані, workflows, dependency contracts, supported host API й authorized outcomes лишаються forward-compatible. Нові mandatory capabilities, breaking converters, розширення dependency contract або новий data scope `MUST` вимагати іншої update classification. | Conformance fixtures автоматично приймають UI fix та defaulted field, але відхиляють auto-activation release з новою mandatory capability, breaking migration або розширеним доступом. |
| REQ-APP-019 | AppPackage/handler `MUST NOT` змінювати або обходити Core-governance: Owner/Admin invariants, membership revocation, key rotation/revocation, hard deny та control-log ordering. | Пакет, який намагається надати Admin право видалити Owner або скасувати revocation, відхиляється незалежно від manifest/handler result. |
| REQ-APP-020 | Складний App `MAY` запропонувати уповноваженому користувачу створити dedicated Space і використовувати Messenger, Knowledge/Notes та Projects/Tasks як залежні `AppInstance` у цьому Space. Користувач `MUST` підписати звичайний `WorkspaceGenesis` і стати початковим Owner; AppPackage/Developer `MUST NOT` отримати ownership через цю дію. | Тестовий «Міста України» створює Space лише після дії користувача, активує три стандартні залежності та не реалізує окремий несумісний messenger/notes/tasks stack; початковим Owner є користувач. |
| REQ-APP-021 | iOS v1 `MUST` виконувати лише декларативні AppPackages, що посилаються на вбудовані VIDA capabilities; завантажувані Rhai/Wasm/JavaScript handlers `MUST` відхилятися цим profile. Apple 4.7 mini-app runtime не входить у v1. | iOS v1 запускає package із schemas/forms/workflows, але повертає явну compatibility failure для code-bearing payload; Android/Windows capability не маскується як підтримана на iOS. |

Точні repository/package manifest, Developer/publisher verification, підписи, право активації та semantic classification `security-auto` відкриті. Post-activation rollback не входить у baseline; недопущення дефектного release є mandatory gate, а recovery після невиявленого production defect ще не затверджено. Так само не обрано Rhai/Wasmtime, повний hook protocol, sandbox і поведінку базового handler після instance override. Cross-App dependency grants та співвідношення Space/AppInstance відкриті в `OQ-0070`.
