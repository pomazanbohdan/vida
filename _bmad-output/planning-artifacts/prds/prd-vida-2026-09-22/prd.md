---
title: "VIDA Release 1"
status: final
created: 2026-09-22
updated: 2026-09-25
source: "../../prfaq-vida.md"
---

# PRD: VIDA Release 1

## 0. Призначення документа

Цей PRD визначає продуктову поведінку публічного Release 1 для команд UX, архітектури, розробки, QA та security. Він спирається на завершений PRFAQ VIDA, групує можливості у Features, задає стабільні ідентифікатори UJ/FR/NFR і не дублює технічні механізми з `addendum.md` та архітектурних документів.

## 1. Бачення

VIDA — open-core, local-first суперзастосунок для людини, яка одночасно веде власні справи й працює з іншими. Повідомлення, форумні теми, нотатки, задачі та файли не розкидані між несумісними сервісами: вони є пов'язаними ресурсами в Personal Space або Shared Space.

VIDA зберігає корисність без мережі, синхронізує дозволені дані між рівнозначними пристроями, дає користувачу керувати Personas, Spaces і доступом та не вимагає єдиного зовнішнього сервера для базової локальної роботи. Для Web↔peer sync у Release 1 продукт спершу пробує пряме з'єднання, а VIDA-operated encrypted Iroh relay є fallback без платного Hosted Space. Якщо обидва шляхи недоступні, Web продовжує локальну роботу до відновлення зв'язку. Прямий Web шлях потребує окремого browser-transport доказу. Відкриті специфікації, переносні дані й MIT Core мають дозволяти незалежну сумісну реалізацію.

Release 1 доводить цю тезу одним завершеним продуктом на Android, iOS, Windows і Web: Messenger, Notes/Knowledge і Project працюють разом, а обов'язкові Core-сервіси Contacts і Calendar, Files, пошук у вибраному Space, права, recovery та AppPackage runtime утворюють пов'язаний досвід. Web — повноцінний статично розміщуваний Flutter-клієнт зі спільною семантикою Rust Core, локальним browser storage та синхронізацією з авторизованими пристроями; він не потребує платного Hosted Space. Видимий розділ «Календар» можна показати або приховати в меню Space незалежно від Project App.

### 1.1 Ринкова теза

VIDA не заявляє себе «першим суперзастосунком» або «першою децентралізованою mini-app платформою». Її перевірна відмінність — єдина open-source local-first модель типізованих пов'язаних Resources, Spaces, прав та синхронізації для комунікації, знань і проєктної роботи. Signal доводить privacy-first messaging, Delta Chat/webxdc — децентралізовані multi-profile Apps, Notion/Linear — пов'язану knowledge/project роботу, WeChat — super-app distribution; VIDA поєднує ці jobs без призначення одного vendor обов'язковим data authority.

## 2. Цільовий користувач

### 2.1 Jobs To Be Done

- Організувати особисті повідомлення, знання, задачі й файли в одному контексті.
- Перетворити особисту роботу на спільну без втрати структури та контролю доступу.
- Продовжувати читати й змінювати синхронізовані дані без мережі.
- Працювати з одного профілю на кількох рівнозначних пристроях.
- Зв'язати обговорення з документом, задачею або файлом без копіювання контенту.
- Запланувати особисту або командну подію, запросити людей та отримати їхні відповіді без відкриття всього Space.
- Використовувати автономну, публічну або видану організацією Persona без змішування їхніх даних.

### 2.2 Нецільові користувачі Release 1

- Організації, яким уже в Release 1 потрібні hosted SLA або централізований web-admin; статичний Web-клієнт не є керованим hosted service.
- Бізнеси, яким потрібні CRM, платежі, бухгалтерія, склад або міське бронювання.
- Команди, для яких обов'язкові Gantt, budgeting, time tracking чи advanced analytics.

### 2.3 Ключові User Journeys

- **UJ-1. Олена налаштовує власну VIDA.**
  - **Контекст:** Олена хоче впорядкувати особисті справи без створення акаунта на центральному сервері.
  - **Вхід:** перший запуск на телефоні.
  - **Шлях:** створює Persona → налаштовує Profile → отримує Personal Space з трьома підготовленими Core AppInstances → обирає, які Apps показати й використовувати зараз → обирає режим системних контактів та recovery.
  - **Кульмінація:** створена нотатка після перезапуску доступна локально й позначена «збережено».
  - **Результат:** підготовлений Project можна ввімкнути пізніше без перевстановлення VIDA.

- **UJ-2. Андрій продовжує роботу на другому пристрої.**
  - **Контекст:** Андрій має телефон і ноутбук з тією самою Persona.
  - **Вхід:** телефон містить чат, нотатку, задачу й файл; ноутбук щойно підключено.
  - **Шлях:** підтверджує новий пристрій → синхронізує Space → відключає мережу → змінює задачу → перезапускає застосунок → відновлює зв'язок.
  - **Кульмінація:** зміна не втрачена й однаково відображається на обох пристроях.
  - **Edge case:** несумісна паралельна зміна створює явний Conflict, а не мовчазну втрату.

- **UJ-3. Олена створює командний проєкт.**
  - **Контекст:** особиста підготовка перетворилася на спільну роботу.
  - **Вхід:** у Personal Space вже є нотатка та список задач.
  - **Шлях:** створює Shared Space → активує Project із залежними Messenger і Notes → запрошує Андрія → задає роль → пов'язує задачу з нотаткою, форумною темою та файлом.
  - **Кульмінація:** кожен учасник бачить лише дозволені ресурси й працює в одному пов'язаному контексті.
  - **Результат:** швидкі домовленості йдуть у Chat, довгі обговорення — у Forum.

- **UJ-4. Андрій поширює одну нотатку, а не весь Space.**
  - **Контекст:** нотатка потрібна зовнішньому учаснику, решта Personal Space приватна.
  - **Вхід:** нотатка містить явно прикріплений файл і приватний backlink.
  - **Шлях:** обирає share → перевіряє склад пакета → надає доступ конкретній Persona.
  - **Кульмінація:** одержувач бачить нотатку та дозволений файл, але не назву й вміст приватного ресурсу.

- **UJ-5. Олена проводить приватний командний дзвінок.**
  - **Контекст:** Олені та іншим учасникам Shared Space потрібне голосове або відеообговорення.
  - **Вхід:** учасники мають чинний доступ до розмови.
  - **Шлях:** починають 1:1 або груповий дзвінок → VIDA встановлює захищений медіасеанс → мережа одного учасника змінюється → сеанс відновлюється або чесно показує збій.
  - **Кульмінація:** медіавміст залишається E2EE; relay не отримує відкритий контент.

- **UJ-6. Олена призначає зустріч команді.**
  - **Контекст:** Олена працює в Shared Space і хоче узгодити час із колегами та зовнішнім запрошеним.
  - **Вхід:** картка контакту або календар поточного Space.
  - **Шлях:** створює подію в поточному Space → зазначає час і часовий пояс → додає запрошених → перевіряє, що саме кожен побачить → надсилає запрошення → одержує відповіді.
  - **Кульмінація:** на конкретну подію можна запросити лише наявну VIDA Persona; вона відповідає, не стаючи учасником усього Space. Контакт без пов'язаного VIDA-акаунта не є доступним адресатом запрошення.
  - **Зміна:** після перенесення часу попередня згода не вважається згодою на новий час.

## 3. Глосарій

- **VIDA Core** — спільне ядро і контракти ідентичності, даних, доступу, синхронізації, recovery та AppPackage runtime.
- **Persona** — окрема криптографічна ідентичність користувача з власними Profile, Devices і Spaces.
- **Profile** — видимі атрибути однієї Persona; не об'єднує інші Personas користувача.
- **Device** — керований користувачем клієнт Persona; Devices однієї Persona рівнозначні.
- **Space** — межа даних, ключів, членства, доступу, Apps і синхронізації.
- **Personal Space** — Space однієї Persona; ресурс може бути поширений окремо.
- **Shared Space** — Space з кількома учасниками, ролями й спільними Apps.
- **AppPackage** — підписаний пакет схем, UI-декларацій, workflows, metadata та дозволеної логіки.
- **AppInstance** — екземпляр AppPackage в конкретному Space із власною identity/configuration; може бути підготовленим, але ще не активованим для користування.
- **Developer** — Persona або publisher identity, уповноважена підписувати й публікувати AppPackages; ця роль не надає прав Owner/Admin у Space.
- **Core App** — вбудований AppPackage Release 1: Messenger, Notes/Knowledge або Project.
- **Resource** — адресований запис у Space: Message, Forum Topic, Note, Task, File або інший schema-об'єкт.
- **Relation** — типізований зв'язок між Resources без дублювання їхнього вмісту.
- **Contact Card** — канонічний контактний ресурс зі stable VIDA ID у конкретному Space та зв'язками з Persona identifiers. Картка не є Space membership: її наявність не надає власнику чи записаній людині прав у Space. Робоча й особиста картки не зливаються автоматично; картка в Shared Space доступна команді відповідно до чинного App/Project scope і прав, а не є приватною карткою її автора.
- **Calendar Event** — подія в Personal або Shared Space з часом, часовим поясом, організатором і запрошеними; створення з картки за замовчуванням використовує поточний Space.
- **Event Invitation** — обмежений доступ до конкретної Calendar Event для запрошеної Persona та її відповіді; він не дорівнює членству в Space.
- **Membership Class** — спосіб участі суб'єкта в доступі: Member, resource-scoped Guest або ServicePrincipal.
- **Role Preset** — вбудований набір дозволів: Owner, Admin, Manager, Contributor, Commenter або Viewer; custom role не може послабити Core governance.
- **Conflict** — кілька несумісних, причинно конкурентних змін одного Resource, які неможливо безпечно об'єднати автоматично.
- **Approval Process** — налаштований процес підтвердження конкретної операції: одним учасником, послідовними етапами або M-of-N.
- **Online Device** — Device, з яким VIDA зараз має підтверджену можливість виконати синхронізацію.
- **Synchronized** — перша незалежна authorized durable application replica підписано підтвердила validate/apply Operation/Frontier; single-Device Persona без replica лишається «Збережено локально». Це не означає business approval або відсутність майбутнього Conflict.
- **Delivered** — хоча б один чинний recipient-controlled Device розшифрував, перевірив і durable-зберіг Message. Для групи стан агрегується per Persona як `N/M`; sender не бачить exact Device topology. Synchronized або mailbox storage не означають Delivered.
- **Terminal Outcome** — доведений бізнес-результат operation, наприклад Accepted або Rejected; transport receipt не є Terminal Outcome.

## 4. Features і функціональні вимоги

### 4.1 Persona, Profile, Devices і Contacts

**Опис:** користувач керує кількома ізольованими Personas, підключає рівнозначні Devices, відновлює власну Persona і працює з Contact Cards без примусової дедублікації.

#### FR-1: Створення Persona та Personal Space

Користувач може створити автономну Persona, налаштувати Profile і отримати Personal Space без зовнішньої реєстрації. Реалізує UJ-1.

**Перевірні наслідки:**
- Перший Resource створюється й повторно відкривається без доступу до зовнішнього сервера.
- Окремі Personas не розкривають взаємний зв'язок іншим учасникам автоматично.

#### FR-2: Підключення рівнозначного Device

Власник Persona може авторизувати новий Device і синхронізувати дозволені Spaces. Реалізує UJ-2.

**Перевірні наслідки:**
- Device не отримує ключі або дані до підтвердження чинним Device чи recovery flow.
- Жоден Device не стає пріоритетним лише через платформу або час підключення.
- Авторизований Web-клієнт є рівнозначним Device однієї Persona, а не лише переглядачем чи платним hosted session.

#### FR-3: Recovery Persona

Власник може відновити автономну Persona за попередньо створеним recovery material.

**Перевірні наслідки:**
- Onboarding вимагає створити й підтвердити збереження recovery material.
- До надійного запису підтвердження Persona/Personal Space залишаються незавершеними: захищені нотатки недоступні, а після переривання setup відкривається той самий pending профіль без заміни ключів чи IDs.
- Федеративний або організаційний акаунт не відновлює приватну Persona без її recovery material.

#### FR-4: Space-scoped Contact Cards і системний імпорт

Користувач може створювати Contact Cards у конкретному Space та за власним вибором імпортувати системні контакти до Personal Space. Export і двостороння синхронізація з provider не входять у Release 1.

**Перевірні наслідки:**
- VIDA не виконує автоматичну дедублікацію Contact Cards.
- Імпорт системних контактів у Release 1 доступний лише там, де конформний OS/provider connector це підтримує, і потребує явного дозволу й preview; відмова не блокує ручне створення карток. Web створює картки вручну та синхронізує вже дозволені картки, але не обіцяє імпорт із системної адресної книги браузера; `.vcf` import також не входить у Release 1.
- Contact Card має stable VIDA ID, field provenance/visibility, typed identity bindings і provider metadata; сумісне відображення полів на vCard 4.0 проєктується як майбутній interchange contract, але `.vcf` import/export не входить у Release 1.
- Картка, створена в робочому Space, не зливається з карткою Personal Space. Явне копіювання між Spaces показує поля, створює новий ID і не підписує копію на майбутні зміни джерела.
- Space member і Contact Card не тотожні: наявність картки не надає Space access, а membership не створює контакт автоматично.
- Картка, створена в Shared Space, видима чинним учасникам із правом читати відповідний App/Project scope; приватних за автором нотаток у Shared Space немає. Окремий App може визначити спеціальну персональну видимість своїх об'єктів/полів (наприклад, показник мешканця для нього й менеджера) через звичайні Core ACL, не змінюючи загальну модель Notes.
- Нестандартний, private або anonymous Persona identifier лишається VIDA-local і ніколи автоматично не експортується до device/provider contacts.
- Sharing за замовчуванням створює одноразовий snapshot; revocable live share вмикається окремо, не дозволяє одержувачу редагувати картку власника й не стирає локальні notes/tags одержувача.
- Unlink connector не видаляє Contact Card або provider contact без окремої явної дії.

### 4.2 Spaces, членство та доступ

**Опис:** Space є явною межею спільної роботи. Owner керує власністю; Admin керує налаштуваннями й учасниками в межах наданих прав, але не змінює Owner.

#### FR-5: Створення Shared Space

Persona може створити Shared Space, стати його Owner і активувати потрібні Core Apps. Реалізує UJ-3.

**Перевірні наслідки:**
- Shared Space має окремі членство, ключі, Apps і права від Personal Space.
- Запрошення саме по собі не розкриває дані до успішної авторизації.

#### FR-6: Ролі й ресурсні права

Owner може керувати Membership Classes і Role Presets; уповноважений налаштовувач деталізує schema-defined domain actions на рівні AppInstance, container, Resource type або Resource.

**Перевірні наслідки:**
- Лише Owner може надати або відкликати роль Owner.
- Admin не може видалити або понизити Owner.
- Core надає незмінні presets Owner, Admin, Manager, Contributor, Commenter і Viewer; scoped/custom roles не можуть обходити Core governance.
- Нормативні дозволені/заборонені дії preset визначає [ADR-0002](../../../../docs/03-architecture/decisions/ADR-0002-default-role-presets.md) разом із [REQ-ACL-007](../../../../docs/02-requirements/access-control-requirements.md): Manager керує workflow у своєму scope, але не Space security; Contributor змінює власне/призначене, але не чуже; Commenter обговорює без редагування основного вмісту; Viewer читає без send/edit/share за замовчуванням. Acceptance перевіряє для кожної ролі щонайменше одну дозволену та одну заборонену дію.
- Member, resource-scoped Guest і ServicePrincipal мають різні membership semantics та не отримують доступ лише через наявність AppPackage або invite link.
- Зміна доступу синхронізується як Core governance operation.

#### FR-7: Відкликання доступу

Owner або уповноважений Admin може відкликати доступ учасника відповідно до його ролі.

**Перевірні наслідки:**
- Після синхронізації керований клієнт припиняє показувати відкликані Resources.
- Shared Space вимагає перевірки прав щонайменше раз на 7 діб; після спливу строку локальний кеш блокується до перевірки.
- VIDA не обіцяє відкликання screenshot, export або вже скопійованого тексту.

#### FR-8: Точкове поширення Resource

Власник дозволу може поширити окрему Note або її Section без відкриття всього Personal Space. Реалізує UJ-4.

**Перевірні наслідки:**
- До sharing входять лише явно дозволені вкладення.
- Закриті backlinks та назви недоступних Resources не розкриваються.
- Одержувач отримує revocable resource-scoped Guest grant, а не повне membership у Personal Space.

### 4.3 Messenger і Forum

**Опис:** Messenger підтримує швидку синхронну комунікацію, а Forum — довготривалі структуровані обговорення в тому самому Space.

#### FR-9: Приватні й групові Chats

Учасник може створювати direct і group Chats, надсилати текст, voice messages та Files, відповідати, реагувати, редагувати, видаляти й закріплювати власні Messages.

**Перевірні наслідки:**
- Message, створене offline, переживає перезапуск і публікується після синхронізації.
- UI окремо показує локальний час написання та канонічний час публікації.
- Видалення для всіх поширює tombstone; експортовані копії не вважаються відкликаними.

#### FR-10: Forum Topics

Учасник може створити Forum, Topics і replies, пов'язані з Project, Note, Task або File. Реалізує UJ-3.

**Перевірні наслідки:**
- Chat і Forum можуть співіснувати в одному Shared Space.
- Topic має стабільний ID, історію та Relations до інших Resources.

#### FR-11: Пошук у Messenger і Forum

Користувач може локально шукати доступні Messages і Topics поточного Space із фільтрами за автором і типом.

**Перевірні наслідки:**
- Результати не розкривають вміст або metadata Resources, до яких право read відкликано.
- Offline search працює лише по локально синхронізованому й дозволеному індексу та явно показує цю межу.

### 4.4 Calls

#### FR-12: E2EE audio/video calls

Чинний учасник розмови може почати й прийняти 1:1 або груповий audio/video call. Реалізує UJ-5.

**Перевірні наслідки:**
- Медіавміст має E2EE; relay не може розшифрувати audio/video.
- Group call підтримує до 8 total participants; UX оптимізований для 4–6; більші групи не входять у Release 1.
- Зміна мережі має відновити сеанс або показати однозначний terminal failure.
- Screen sharing і call recording відсутні в Release 1.

### 4.5 Notes/Knowledge

#### FR-13: Особисті й shared Notes

Користувач може створювати rich-text і structured Notes, Sections, вкладення, Relations та backlinks у Personal Space і Shared Space.

**Перевірні наслідки:**
- Note доступна offline після синхронізації.
- Attachment є Relation до File, а не неявним дублем.

#### FR-14: Спільне редагування Notes

Кілька уповноважених учасників можуть редагувати Note із прозорим merge, presence й видимим позначенням щойно зміненого фрагмента.

**Перевірні наслідки:**
- Незалежні зміни різних фрагментів об'єднуються без втрати.
- Несумісні зміни створюють Conflict із доступними варіантами.
- Live remote cursors і selections є Release-1 gate на Android, iOS, Windows і Web; presence є transient і не входить до durable history. Вибір engine/editor має пройти `SPEC-COLLABORATIVE-DOCUMENT-CONFORMANCE-DRAFT`.

#### FR-15: Історія Note

Користувач із правом read може переглянути історію доступних revisions; відновлення revision створює нову зміну, а не стирає аудит.

**Перевірні наслідки:**
- Історія не показує вміст revision, на який користувач не має чинного read.
- Відновлена revision синхронізується як нова operation та може створити Conflict із concurrent change.

### 4.6 Project

#### FR-16: Projects, Tasks і Subtasks

Користувач може створювати Projects, Tasks і Subtasks зі status, assignee, dates та Relations.

**Перевірні наслідки:**
- Project працює особисто в Personal Space або спільно в окремому Shared Space.
- Task може посилатися на Chat, Forum Topic, Note і File.

#### FR-17: List і Board views

Користувач може керувати Tasks у list і board views без створення різних копій Task.

**Перевірні наслідки:**
- Зміна Task в одному view відображається в іншому після того самого локального commit.
- Move, заборонений правами або workflow, не змінює Task і показує причину.

#### FR-18: Task workflow

AppInstance може задавати доступні statuses і переходи; чинне право update визначає, хто може створити зміну.

**Перевірні наслідки:**
- Одночасні несумісні status changes обробляються загальною Conflict system.
- Gantt, time tracking, budgeting і advanced analytics не входять у Release 1.

### 4.7 Files, Relations і пошук у поточному Space

#### FR-19: Files як спільні Resources

Користувач може додати File до Space, переглянути його статус, versions і Relations та прикріпити той самий File до кількох Resources.

**Перевірні наслідки:**
- File має стабільний ID і не дублюється через повторне прикріплення.
- Кожен Space має Files view для доступних Files, їхніх versions, availability і Relations.
- Поріг автозавантаження налаштовується; більші Files потребують явної дії.
- Помилка збереження залишає статус «не завантажено» з причиною та retry.

#### FR-20: Варіанти конфліктного File

Під час Conflict користувач може залишити один варіант основним, зберегти інший revision або створити окрему копію з власним ID.

**Перевірні наслідки:**
- Вибір main variant не видаляє інший варіант до виконання safe-GC rule.
- Окрема копія успадковує лише явно визначені metadata/permissions і отримує новий stable ID.

#### FR-21: Локальний пошук у вибраному Space

Користувач може шукати по локально доступних і синхронізованих Resources вибраного Space з фільтрами за AppInstance, автором і типом Resource. «Усі Spaces» лишається вибором Space, не пошуком по змішаному вмісту.

**Перевірні наслідки:**
- Пошуковий індекс не містить недоступних Resources.
- Приватний вміст не надсилається зовнішньому search service за замовчуванням.
- Контакти шукаються спершу в поточному Space; перехід до контактів іншого дозволеного Space є явною зміною scope, а не автоматичним злиттям карток.

### 4.8 Offline, sync і Conflict system

#### FR-22: Durable local save

VIDA показує «збережено» лише після надійного локального запису operation і потрібних bytes. Реалізує UJ-1, UJ-2.

**Перевірні наслідки:**
- Chat, Note, Task, Relation і File intent переживають штатний restart.
- Помилка диска показується; незбережена operation не позначається «збережено».

#### FR-23: Синхронізація рівнозначних Devices

VIDA синхронізує operations між досяжними Devices без platform або owner-device priority.

**Перевірні наслідки:**
- Повторна доставка idempotent і не створює дубль логічного Resource.
- UI не змішує proofs і показує лише доведені стани: `збережено локально`, `очікує`, `синхронізовано/опубліковано`, `доставлено` where provable, `прийнято/відхилено` where applicable, `результат невідомий` або `конфлікт`.
- Кожний сильніший стан вимагає окремого proof; sync receipt не означає delivery або Terminal Outcome.
- Default «Синхронізовано» потребує першої незалежної authorized durable application replica; mailbox показує лише «Збережено для доставки».
- 1:1 «Доставлено» потребує першого qualifying recipient Device receipt; group UI показує `N/M logical recipients`, а coverage усіх власних Devices доступне лише owner diagnostics.
- Profile може публічно показувати кількість Online Devices поточної Persona, не змішуючи інші Personas.

#### FR-24: Детермінований merge

VIDA автоматично об'єднує сумісні конкурентні зміни та однаково впорядковує опубліковані Messages на всіх Devices.

**Перевірні наслідки:**
- Device wall clock не є єдиним доказом причинного порядку.
- Chat показує publication time; локальний composition time доступний окремо.

#### FR-25: Явний Conflict lifecycle

VIDA створює Conflict для несумісних змін, зберігає всі варіанти й дозволяє уповноваженій людині прийняти наявний стан, застосувати свою зміну або створити копію, де це підтримано.

**Перевірні наслідки:**
- Давно offline Device може повторно відкрити Conflict новою невідомою гілкою.
- Однаковий результат двох operations показується один раз, але обидві operations лишаються в історії.
- Учасник не вирішує Conflict, якщо не може прочитати всі потрібні варіанти.
- Resolve Conflict є Core behavior, а не окремим ACL permission.

#### FR-26: Незворотні effects після підтвердження

Незворотна зовнішня дія виконується лише після достатнього підтвердження operation за правилами процесу.

**Перевірні наслідки:**
- Повторний effect використовує стабільний logical ID та idempotency contract.
- Якщо terminal outcome неможливо довести, UI показує «результат невідомий» і не повторює effect сліпо.

### 4.9 Approval Processes і automation

#### FR-27: Approval Process для конкретного процесу

Admin може обрати для кожного named process один із Core modes: один уповноважений approver, послідовні етапи або M-of-N.

**Перевірні наслідки:**
- AppPackage задає оптимальний default; Admin Space може його змінити.
- Core рахує approvals, відкидає дубль vote і фіксує outcome.
- Approval Process не змінює Core governance правил Owner/Admin.

#### FR-28: Automation у межах AppInstance і Space

Дозволена логіка AppInstance може реагувати на визначені lifecycle events, змінювати Resources у межах чинних прав і створювати детерміновані похідні Resources.

**Перевірні наслідки:**
- Одна вихідна подія створює один logical derived Resource на всіх Devices.
- Похідна operation синхронізується; remote apply не запускає нову бізнес-команду повторно.
- Bot або agent створює звичайну нову зміну; auto-resolution Conflict потребує окремої automation setting, не нового ACL permission.

### 4.10 AppPackages і schemas

#### FR-29: Активація й залежності AppPackage

Уповноважений за Space policy користувач може активувати AppPackage у Space; AppInstance може залежати від інших AppInstances через versioned contracts. Точне право Owner/Admin на активацію залишається `OQ-0039`, а не визначається цим PRD наперед.

**Перевірні наслідки:**
- Складний App може використовувати Messenger, Notes і Project як залежності.
- AppInstance володіє власним schema/config namespace, але кожна read/write/effect operation проходить чинну Space-, instance-, container- і Resource-scoped authorization.
- Cross-App dependency відкриває versioned contract, а не implicit access grant.
- Activation відбувається лише після успішного migration/conformance preflight; у разі помилки оновлення чинна версія лишається активною, а при першій активації AppInstance лишається неактивним. Уповноважений користувач бачить причину; часткової міграції немає.

#### FR-30: Оновлення AppPackage

AppInstance підтримує `compatible-auto`, `security-auto`, `manual` і `pinned` update modes.

**Перевірні наслідки:**
- Режим для конкретного AppInstance змінює лише Owner або Admin із `manage_apps`; оновлення native host binary і AppPackage залишаються різними процесами.
- `compatible-auto` може активуватися автоматично лише за збереження сумісності старих даних, workflows, dependency contracts, supported host API та authorized outcomes; нова mandatory capability, breaking migration, розширений dependency contract або новий data scope не проходять цей режим.
- `manual` не активує знайдений release без окремої дії уповноваженого користувача; `pinned` зберігає обрану версію до явної зміни налаштування/версії уповноваженим користувачем. Сам факт discovery/download не змінює active version або grants.
- `security-auto` не обходить чинну Space activation authority, migration/conformance preflight або host capability checks. Позначка видавця «security» сама не робить пакет auto-eligible: автоматично може активуватися лише перевірене сумісне виправлення без розширення capabilities, data scope або dependency access. Новий доступ чи несумісна migration вимагає окремого підтвердження уповноваженим Owner/Admin; точний proof/manifest profile лишається `OQ-0043`.
- Якщо preflight/activation не пройдено, при оновленні попередня версія лишається активною, а при першій активації AppInstance лишається неактивним; часткової міграції немає, уповноважений користувач бачить причину. Downgrade після успішної activation не входить у baseline.
- Оновлення одного AppInstance не блокує решту VIDA.
- First-party Core Apps за замовчуванням отримують compatible/security updates.
- Зовнішній repository/package за замовчуванням лишається в `manual`, доки довіру до source/publisher не налаштовано явно; виявлення чи завантаження release саме не активує його.

#### FR-31: Schema evolution

AppPackage може додавати й мігрувати schema fields через versioned migrations без мовчазної втрати даних.

**Перевірні наслідки:**
- Optional field із default читається lazy й materializes при наступному write або background migration.
- Редагування старого Resource завершує міграцію до активної schema; required field без default вимагає значення.
- Несумісний старий клієнт переводить лише affected AppInstance у read-only до update.

#### FR-32: Підписаний AppPackage contract

Developer може підписати й опублікувати AppPackage, а VIDA — перевірити manifest, заявлені publisher metadata, version, dependencies і compatibility до activation. Криптографічне підтвердження особи/ключа видавця залежить від майбутнього trust profile (`OQ-0041`/`OQ-0043`) і тут не вважається вже визначеним механізмом. Release 1 включає вбудований керований Marketplace та підключення сумісних зовнішніх repositories; точний формат і trust policy залишаються відкритими в `OQ-0043`.

**Перевірні наслідки:**
- Bundled Core Apps і зовнішні Apps використовують той самий AppPackage/AppInstance conformance contract; сумісний зовнішній declarative package з новими формами/ресурсами працює без нового release VIDA-клієнта.
- Каталог відрізняє application від extension package і показує його target; підключення repository, discovery, download і Space activation є окремими станами та самі собою не надають доступу до приватного Space.
- У release build доступний VIDA catalog і тестове підключення external repository; update discovery показує нову версію, але не змінює active instance без чинної update policy та authorization.
- Роль Developer не надає автоматичного доступу до Space, де інший користувач активував AppPackage.
- iOS Release 1 AppPackage runtime виконує сумісні bundled і зовнішні declarative packages лише через вбудовані capabilities; довільний downloadable code заборонено.

### 4.11 Onboarding, localization і diagnostics

#### FR-33: Пояснювальний onboarding

Під час першого запуску користувач створює Persona, налаштовує Profile, обирає видимість і використання Core Apps, recovery та, де доступний конформний OS/provider connector, добровільний імпорт системних контактів. На підтримуваному Android-пристрої можна добровільно ввімкнути режим підвищеної доступності; у Web контактні картки створюються вручну або приходять через дозволену синхронізацію.

**Перевірні наслідки:**
- Стандартний Personal Space одразу має підготовлені Messenger, Knowledge/Notes і Projects/Tasks AppInstances; користувач може завершити onboarding, показавши/увімкнувши лише обрані, а решту — пізніше, без перевстановлення чи повторного створення Space.
- Відмова від optional contact/high-availability permissions не блокує створення Persona та Personal Space.
- Екран пояснює різницю: звичайний режим відновлює зв'язок і синхронізацію за доступних ОС вікон, а Android-режим підвищеної доступності може показувати постійне системне сповіщення та витрачати більше батареї; жоден режим не обіцяє безперервного Online.
- Режим вмикається лише явною згодою для конкретного пристрою, вимикається згодом у налаштуваннях; після вимкнення діє звичайний режим без втрати локально збережених змін. Зовнішній push не активується для автономної анонімної Persona, а згода однієї Persona не переноситься на іншу.

#### FR-34: Localization

Користувач може обрати одну з 18 погоджених мов і відповідний locale profile; українська є базовою мовою продукту, російська не входить у Release 1.

**Перевірні наслідки:**
- Locale коректно форматує дату, час, числа, plural forms і напрям письма.
- Підтримка `zh-Hans`/`zh-Hant` не означає launch у mainland China.

#### FR-35: Privacy-preserving diagnostics

Користувач може створити локальний diagnostic bundle, переглянути й відредагувати його та явно надіслати через обраний channel.

**Перевірні наслідки:**
- Автоматична передача diagnostics вимкнена за замовчуванням.
- Bundle не містить Message/Note/File content, ключів або cross-Persona correlation identifiers.

#### FR-36: Portable encrypted export

Користувач може створити зашифрований переносний export власних schemas, Resources, Relations, Files і recovery metadata.

**Перевірні наслідки:**
- Формат описаний відкритою versioned specification.
- Сумісний незалежний client може прочитати export після належної авторизації.

### 4.12 Calendar Events і запрошення

#### FR-37: Особисті та спільні календарні події

Користувач може створювати Calendar Events у Personal або Shared Space із часом і часовим поясом; створення з картки контакту за замовчуванням використовує поточний Space. Реалізує UJ-6.

**Перевірні наслідки:**
- Подія має окремий ID і owning Space; вона не підміняється Task із датою чи нагадуванням.
- До збереження користувач бачить, у якому Space створюється подія.
- Локально збережена, але ще не доставлена подія не показується як підтверджена запрошеними.
- Calendar є обов'язковою Core capability, не залежить від активації Project; його видимий пункт меню Space можна приховати без вимкнення подій або нагадувань.
- Release 1 підтримує одноразові та прості щоденні/щотижневі повторення з нагадуваннями. Складні винятки повторюваної серії відкладено.

#### FR-38: Запрошення, відповіді й зміна часу

Організатор може запросити VIDA Personas до Calendar Event; запрошений може відповісти на конкретне запрошення або запропонувати інший час без членства в owning Space. Реалізує UJ-6.

**Перевірні наслідки:**
- Запрошена Persona поза Space бачить назву, час, часовий пояс і місце події. Опис, Files і пов'язані Tasks доступні лише коли організатор явно включив їх у запрошення і надав відповідний scoped access; решта Space не відкривається.
- Додати до запрошених можна лише існуючий VIDA-акаунт/Persona. Картку без такого зв'язку можна зберігати в Contacts, але не створювати для неї «недоставлене запрошення».
- Відповідь на старий час не підтверджує перенесену подію: після зміни часу запрошений має відповісти повторно.
- Час змінює організатор або Owner/Admin owning Space; звичайний запрошений лише пропонує інший час. Owner/Admin можуть вести подію, якщо організатор утратив доступ.

### 4.13 Web як клієнт Release 1

#### FR-39: Статичний local-first Web-клієнт із синхронізацією

Користувач може відкрити VIDA як статично розміщений Flutter Web-клієнт, створити або підключити Persona, зберігати дозволені дані локально у browser profile та синхронізувати їх з авторизованими Android/iOS/Windows/Web Devices без платного Hosted Space.

**Перевірні наслідки:**
- Web-клієнт постачається як статичний набір файлів. Hosting не є бізнесовим власником Persona/Space і не отримує ключі через протокол синхронізації, але оператор або компрометація origin може підмінити виконуваний JS/Wasm і прочитати дані, доступні запущеному клієнту. Тому Web не обіцяє zero-knowledge відносно host-а виконуваного коду; вибір власного або стороннього host-а є вибором довіри до доставки коду. UI показує поточний origin і пояснює цю межу до підключення Persona; trust, цілісність релізу, origin isolation і захист від XSS є окремим release gate.
- Авторизований браузер синхронізує принаймні Persona/Space, Chat, Note, Task і File через ті самі Core operation/authorization/receipt правила, що встановлювані клієнти; Release-1 feature suites охоплюють усі обов'язкові функції Web, а не лише демонстраційну нотатку.
- Без мережі доступні локально наявні дозволені дані й pending дії після відкритого раніше та перевірено кешованого app shell/JS/Wasm; офлайн-повторне відкриття без цих assets не обіцяється. Очищення browser/site data, quota eviction або втрата profile не подаються як безпечне зберігання чи remote backup. Користувач має перевірене відновлення/експорт і зрозумілий стан єдиної локальної копії.
- Після закриття вкладки або призупинення браузера Web не обіцяє постійного Iroh-з'єднання, вхідного дзвінка чи локального спрацювання нагадування в точний час; незавершені події відновлюються після відкриття клієнта. Активна вкладка проходить повну call/calendar suite, а фоновий delivery потребує окремого затвердженого platform profile, без зовнішнього push для автономної анонімної Persona.
- Web↔native синхронізація є direct-first із VIDA-operated encrypted Iroh relay fallback. Поточний Iroh/Wasm сам по собі relay-only, тому прямий Web↔native/Web↔Web шлях через browser-compatible WebRTC/custom transport є обов'язковим proof gate, а не затвердженою бібліотекою. Relay не є платним Hosted Space, durable mailbox або authority; коли обидва шляхи недоступні, remote sync лишається pending.
- Browser key custody, origin/XSS isolation, storage quota/eviction, lifecycle, підтримувані браузери, Rust/Wasm bridge, E2EE calls і accessibility проходять окремий Release-1 conformance gate; невдалий gate блокує реліз, а не непомітно перетворює Web на урізаний companion.

## 5. Явні non-goals Release 1

- City Portal, каталог локального бізнесу й booking.
- Розширена комерційна інфраструктура Marketplace: оплата пакетів, промоаналітика видавця та платне розміщення.
- Hosted paid Space, durable mailbox/S3 service та server-rendered private web client; прямий peer transport і транспортні VIDA-operated Iroh relay fallbacks для Web/native входять до Release 1.
- WinUI 3 client; Release 1 Windows client реалізується Flutter.
- CRM, платежі, accounting, inventory.
- Експорт або двостороння provider-синхронізація контактів і зовнішня calendar sync із Google/Microsoft/OS; у Release 1 системні контакти лише імпортуються до Personal Space за явним вибором.
- Файлові `.vcf` import/export контактів і `.ics` exchange календарних подій; це окремий майбутній interchange contour.
- Screen sharing і call recording.
- Повна Notion database/formula parity; Gantt, budgeting, time tracking, advanced analytics.
- Загальний імпорт Telegram, Signal, WhatsApp, Notion або Drive.
- Довільний downloadable executable code для iOS App Store profile.
- Гарантія постійного mobile-online, абсолютної анонімності або відкликання вже експортованих копій.

## 6. Release 1 scope і acceptance gate

### 6.1 In scope

- Android, iOS і Windows Flutter clients та статично розміщуваний Flutter Web client зі спільним VIDA Core/конформною Rust/Wasm реалізацією.
- Core Apps: Messenger, Notes/Knowledge, Project; bundled AppPackages проходять спільний conformance contract.
- Persona, Profile, Personal/Shared Space, Contacts, Calendar Events/invitations, Files, Relations, Space-local search, access, sync, recovery, AppPackage runtime.
- Forums та E2EE 1:1/group audio-video calls.
- 18 мов / 21 locale profiles.

### 6.2 Обов'язковий vertical slice

Release 1 не готовий, доки автоматизований сценарій не доведе: дві Personas, щонайменше три Devices (включно з Android і Web), Personal Space, Shared Space, Chat, Note, Task, File, offline writes, restart/reopen, reconnect, deterministic merge, explicit Conflict, повторний sync без дублювання та 1:1 E2EE call. Окремі Web↔Android сценарії перевіряють авторизоване підключення браузера, локальне durable save, прямий sync без relay data forwarding, encrypted relay fallback за недоступності direct, перемикання маршруту без дублювання та pending за втрати обох шляхів.

Це **інтеграційний доказ лише названих дій і станів**, а не pass цілих FR за згадкою Chat/Note/Task/File/Call. Наприклад, він сам не доводить Guest sharing, Forum, voice messages, восьмиособовий group call, live cursors, календарне запрошення, усі варіанти File Conflict чи незворотний effect. Кожна обов'язкова поведінка FR-1–FR-39 потребує свого feature/conformance proof із §6.3; Release 1 потребує **обох** наборів.

### 6.3 Обов'язкові feature і conformance gates

- Persona/Device/recovery suite доводить FR-1–FR-3: автономне створення, ізоляцію Personas, authorized Device enrollment, рівнозначність Devices та відновлення лише з належним recovery material.
- Contact connector/card suite доводить FR-4, включно з preview system import до Personal Space на платформах із конформним connector, ручним створенням та sync карток на Web, незалежністю карток між Spaces, явною копією, team-visible work card, snapshot/live share, private bindings і unlink semantics; `.vcf` file import/export і provider export/two-way не заявляються як Release-1 pass.
- Calendar suite доводить FR-37–FR-38: Personal/Shared ownership, часовий пояс, Core availability без Project, одноразові й прості повторювані події та нагадування, existing-VIDA-only invitations, обмежений invitee preview, RSVP, proposal часу й повторне погодження після перенесення; `.ics` file exchange не є Release-1 pass.
- Access-control/sharing suite доводить FR-5–FR-8 для Member, Guest, ServicePrincipal і всіх Role Presets, включно з точковим поширенням Note/Section та забороною відкривати Personal Space.
- Messenger/Forum suite доводить FR-9–FR-11: direct/group Chat, voice messages, replies/reactions/edit/delete/pin, локальний пошук, Forum Topics і Relations до Project/Note/Task/File.
- Calls suite доводить увесь FR-12, зокрема 1:1 і group audio/video до 8 учасників, через чинні F01–F18 у `docs/04-specifications/fixtures/e2ee-calls-v1.yaml`, включно з Web media boundary, без hard-gate failure.
- Notes/Project/Search suite доводить FR-13–FR-18 і FR-21, включно зі structured Notes, revisions, live cursors/selections, list/board views та workflow; editor pair проходить чинні F01–F14 у `docs/04-specifications/fixtures/crdt-editor-v1.yaml`.
- Files/Relations/Conflict/effects suite доводить FR-19–FR-26: повторне прикріплення без дубля, download/error states, revision проти окремої копії, durable save, proofs sync/delivery, merge, читання всіх Conflict variants і confirmation перед незворотним effect.
- AppPackage/schema/Marketplace suite доводить FR-27–FR-32, включно з failed preflight, mixed versions, migration on write, bundled/external parity, publication/discovery, external repository connection, application/extension distinction і забороною implicit grants.
- Onboarding/localization/diagnostics/export suite доводить FR-33–FR-36.
- Platform conformance matrix з `docs/02-requirements/platform-nfr.md` є нормативним gate окремо на Android, iOS, Windows і Web: crash/reopen boundary, exactly-one domain apply, direct/relay path selection та reconnect/reconcile, relay loss/restore, storage/key failures, rights expiry, accessibility та resource budgets. Native background і store-policy перевірки застосовуються лише до відповідних ОС; Web додатково проходить FR-39 і browser security/storage gate.
- Open interoperability gate вимагає public byte-exact/negative-security fixtures, third-party AppPackage conformance, independent reference reader, independent client/node proof, cross-vendor evidence та license manifest без hidden paid dependency.
- Privacy/compliance gate доводить `REQ-PRIV-001–009` для кожного фактично активованого processing flow через документований flow inventory/record (purpose, data, legal basis, retention, recipients, transfers, roles), privacy defaults, rights і breach runbooks (включно з фіксацією awareness time та оцінкою застосовного 72-годинного повідомлення), risk-based security, EU applicability review, DPIA й transfer/hosting assessment де застосовно. [Матриця доказів](../../../../docs/04-specifications/privacy-release-evidence.md) фіксує потрібні артефакти й негативні gate checks, але ще не містить pass evidence. Local-only, direct peer, push, crash/store analytics оцінюються окремо; Hosted Space проходить такий самий gate до його майбутньої activation. Відповідальні controller/operator/store-account roles мають бути призначені до production activation або public store publication. Ні E2EE, ні вибір країни не замінюють цього доказу.

### 6.4 Порядок реалізації без скорочення scope

- **Thesis-critical gates:** Persona/Spaces/access, Messenger/Notes/Project/Files/Relations, durable offline sync, deterministic merge, Conflict system і portable continuity.
- **Launch-completeness gates:** E2EE calls, signed AppPackage contract, localization, diagnostics, accessibility та store conformance.
- Обидві групи обов'язкові для Release 1; поділ визначає порядок proofs, а не optional scope.
- Якщо proof не проходить, команда замінює stack або ізолює implementation behind stable contract; публічний release блокується, доки та сама продуктова обіцянка не доведена. Скорочення обіцянки потребує нового явного продуктового рішення.

## 7. Cross-cutting NFRs

- **NFR-1 Security:** mobile surfaces проходять OWASP MASVS-aligned verification; Web та exposed APIs — OWASP ASVS-aligned verification; threat model і abuse cases є release artifacts.
- **NFR-2 Cryptography:** протоколи не винаходять власні primitives; ключі зберігаються через platform secure storage; E2EE call security має окремий review gate.
- **NFR-3 Local durability:** підтверджене локальне save не втрачається після штатного restart/process kill або Web reopen за збереженого browser profile; очищення site data/eviction є окремим явним ризиком, не доказом remote backup.
- **NFR-4 Convergence:** однаковий set operations дає однаковий user-visible state на всіх conformant Devices.
- **NFR-5 Offline:** уже синхронізовані дозволені Resources доступні без мережі; outbound operations мають явний pending state.
- **NFR-6 Accessibility:** Android, iOS, Windows і Web clients підтримують screen readers, keyboard navigation, visible focus і platform semantics; Windows також підтримує system menus і tray where applicable.
- **NFR-7 Privacy:** content telemetry і automatic diagnostic upload вимкнені; різні Personas не корелюються продуктом без явної дії користувача.
- **NFR-8 Openness:** MIT Core, versioned public specifications, public fixtures/conformance tests, portable export, license manifest та independent reference reader є release gate.
- **NFR-9 Localization:** усі 21 locale profiles проходять automated formatting tests; RTL layout перевіряється для Arabic.
- **NFR-10 Performance budgets:** native G0 method/profiles/fixtures зафіксовано в `SPEC-PLATFORM-RESOURCE-CONFORMANCE-001`; Web G0 ще потребує supported-browser matrix і runner за OQ-0075 до Web implementation fan-out. Representative builds на фізичних пристроях дають G1 baseline, після якого затверджуються числові G2 budgets для install/download size, cold/warm start, memory, battery, network, storage growth, sync convergence, crash/ANR/browser failures і call quality. G2 потрібен до feature-complete gate; G3 перевіряє regression і чинні store limits перед release. Числові пороги до G1 не вигадуються.
- **NFR-11 Store compliance:** Android/iOS packages, background behavior, privacy disclosures й dynamic-content profile проходять актуальні store-policy checks до submission.
- **NFR-12 No silent loss:** VIDA-controlled operation, migration, Conflict cleanup і File eviction не видаляють останню відновлювану копію без явного дозволеного lifecycle rule. Browser/site-data очищення поза контролем VIDA не можна фізично заборонити; Web завчасно попереджає про єдину копію й надає encrypted export/restore.
- **NFR-13 Inactive Apps:** неактивний AppInstance не запускає handlers/background work, не запитує optional permissions і не створює матеріального runtime/resource overhead.
- **NFR-14 Platform conformance:** усі обов'язкові рядки `platform-nfr.md` мають автоматизований або документований pass evidence окремо для Android, iOS, Windows і Web з platform-specific browser storage/direct-relay/security profile.
- **NFR-15 Interoperability:** публікація specification не дорівнює сумісності; Release 1 потребує distributable independent reference reader і незалежного client/node conformance proof.
- **NFR-16 Privacy/compliance:** production flow, public store publication і paid service проходять затверджений baseline `REQ-PRIV-001–009`; відсутність legal controller/operator/store-account owner лишається блокером запуску, не причиною оголосити compliance завчасно.

## 8. Success metrics

**Primary**

- **SM-1 Vertical-slice conformance:** 100% mandatory integration scenarios §6.2 pass on Android, iOS, Windows і Web, включно з Web↔Android direct sync та relay fallback; цей показник не підміняє повноту FR, яку доводять окремі suites §6.3.
- **SM-2 Data integrity:** 0 silent-loss, duplicate-effect or cross-permission disclosure defects in release-gate suites. Validates FR-7, FR-22–FR-28, FR-31.
- **SM-3 Connected-context proof:** moderated opt-in usability sessions вимірюють completion та assistance rate для Chat → Note/Task/File relation journey. `OQ-8` визначає consented method/sample; числовий target затверджується після baseline і до Release-1 product-value decision. Це доказ зручності пов'язаного контексту, а не повний conformance pass FR-10, FR-13, FR-16, FR-19 чи FR-21.

**Secondary**

- **SM-4 Platform readiness:** 100% нормативної platform conformance matrix, accessibility, localization, OWASP і store-policy gates pass.
- **SM-5 Distribution:** store download/install counts are reported only in aggregate; VIDA Core does not add behavioral tracking to manufacture an active-user metric.
- **SM-6 Supportability:** 100% negative fixtures блокують secret/content leaks; жоден known leak не допускається до release. Validates FR-35.
- **SM-7 Interoperability:** independent reference reader та independent client/node проходять public conformance fixtures без private VIDA service. Validates FR-36, NFR-8, NFR-15.

**Counter-metrics**

- **SM-C1:** Do not maximize time-in-app; VIDA should reduce coordination friction.
- **SM-C2:** Do not maximize telemetry coverage; privacy and Persona unlinkability override analytics convenience.
- **SM-C3:** Do not maximize feature count at the cost of conformance or silent data loss.

## 9. Risks і release blockers

- **R-1 Integration scale:** Messenger + Notes + Project + Calendar + calls + AppPackages across four platforms creates large integration risk. Mitigation: Android↔Web vertical slice precedes breadth; browser Rust/Wasm, direct WebRTC/custom transport, storage/key security and relay-fallback proof are explicit gates; Calendar has a separate feature suite and does not inherit a pass from the older slice.
- **R-2 Protocol contracts:** operation envelope/serialization, serverless authority/frontier and merge semantics (`OQ-0033`/`OQ-0034`), storage/GC, CRDT/editor presence, Rust↔Flutter ABI, media signaling/keying and bundled/external AppPackage compatibility must be frozen as versioned specs before implementation fan-out. Receipt/finality semantics already fixed by `SPEC-OPERATION-FINALITY-001`; усі решта contract gates і докази перелічено в [addendum §I, R-2 inventory](addendum.md#r-2-implementation-fan-out-inventory).
- **R-3 Public release authority:** legal controller, store-account owner, signing-key custody, incident authority and disclosure channel are not assigned. `REQ-PRIV-001–009` і призначення відповідальних є launch gate; production processing та public store submission заблоковані до його проходження.
- **R-4 Independent assurance:** scope and provider of external pentest/crypto-review remain unresolved; security-sensitive release claims cannot exceed completed evidence.
- **R-5 Localization operations:** translators, reviewers and update SLA for 18 languages are not assigned.

## 10. Open Questions і disposition

Продуктові рішення OQ-14–OQ-18 затверджені 2026-09-24 та узгоджені з UX, requirements і product bundle. `final` у цьому PRD означає завершений продуктовий контракт, **не** дозвіл на implementation fan-out, store submission чи security claims. Закриття OQ-2–OQ-3 **необхідне, проте недостатнє**: решта R-2 contracts також потребують затверджених версійованих profiles і доказів за [addendum §I](addendum.md#r-2-implementation-fan-out-inventory). OQ-4 G0 profiles/method зафіксовано в `SPEC-PLATFORM-RESOURCE-CONFORMANCE-001`; його числові G2 thresholds мають бути затверджені після G1 baseline і до feature-complete gate.

| ID | Питання | Owner | Потрібний доказ / артефакт | Gate |
|---|---|---|---|---|
| OQ-1 | Receipt/finality contract для `Synchronized`, Delivered і Terminal Outcome | Architecture | `SPEC-OPERATION-FINALITY-001` + `operation-finality-v1.yaml` | resolved 2026-09-22 |
| OQ-2 | Який CRDT/editor pair задовольняє merge, durable history, transient presence та live cursors? | Architecture/UX | [research](../../research/technical-vida-crdt-editor-stack-2026-09-22/research.md) + `SPEC-COLLABORATIVE-DOCUMENT-CONFORMANCE-DRAFT` + `crdt-editor-v1.yaml` F01–F14; selection pending reproducible prototype | before implementation fan-out |
| OQ-3 | Який media/signaling/key-management profile проходить E2EE calls на Android, iOS, Windows і Web? | Architecture/Security | Capacity 8 і anonymous/public push boundary затверджені; [research](../../research/technical-vida-e2ee-media-stack-2026-09-22/research.md) + `SPEC-E2EE-CALLS-CONFORMANCE-DRAFT` + `e2ee-calls-v1.yaml`; Web потребує окремих browser E2EE/permission/lifecycle/mixed-client fixtures; profile selection pending reproducible prototype/security evidence | before implementation fan-out |
| OQ-4 | Які числові performance, battery, storage, crash і call-quality budgets дають representative builds? | Architecture/QA | `SPEC-PLATFORM-RESOURCE-CONFORMANCE-001` + `platform-resource-v1.yaml` fix G0 profiles/method; G1 reproducible benchmark report yields G2 thresholds | G0 resolved 2026-09-22; G1/G2 before feature-complete gate |
| OQ-5 | Хто виконує controller, store accounts, signing custody, incident response і disclosure? | Product/Operations | named responsibility record + operational runbooks | before store submission |
| OQ-6 | Чи є independent pentest/crypto-review mandatory і хто виконаве? | Security/Product | signed review plan or explicit risk acceptance | before public security claims |
| OQ-7 | Хто володіє translation/review та SLA кожного locale? | Product/Localization | locale ownership matrix + publication workflow | before localization freeze |
| OQ-8 | Які method/sample валідовують SM-3 без behavioral tracking? | Product Research | consented research protocol, що визначає одиницю й denominator завершення Chat → Note/Task/File task та кодування assistance, + baseline report і затвердження target після baseline | before Release-1 product-value decision |
| OQ-9 | Що стається, якщо capability proof не проходить? | Product/Architecture | правило §6.4: replace stack/isolate implementation; release blocked; scope change only by new decision | resolved in this draft |
| OQ-10 | Який документ нормативний при conflict зі старим Architecture Spine? | Architecture/Governance | accepted ADR/approved requirement wins; Spine reconciliation change | before architecture handoff |
| OQ-11 | Які releases справді належать до `security-auto`, коли вони можуть активуватися автоматично і як це пояснюється Owner/Admin? | Product/Architecture/Security | Product rule у FR-30: лише перевірене сумісне виправлення без нових capabilities/data access; publisher label недостатній. `OQ-0043` signed proof profile і negative/positive fixtures лишаються технічним gate. | product rule resolved 2026-09-24; proof before security-auto release claim |
| OQ-12 | Хто саме з Owner/Admin може активувати/вимикати AppInstance і надавати його capabilities? | Product/Architecture | `OQ-0039` Space activation authority + negative ACL fixtures | before AppPackage implementation fan-out |
| OQ-13 | Який точний зовнішній repository/package format і trust/update profile забезпечить Release 1 без неявних grants? | Architecture/Security | `OQ-0043` versioned manifest/trust contract, signature/freshness/anti-downgrade та cross-platform fixtures | before external Marketplace implementation fan-out |
| OQ-14 | Чи є Contact Cards у Shared Space спільною адресною книгою, приватними картками учасників у робочому контексті або обома варіантами? | Product/Privacy | Team-visible under App/Project ACL; no per-author private Shared Space Notes; app-specific per-user resources remain possible. FR-4, `REQ-CONTACT-016`. | resolved 2026-09-24 |
| OQ-15 | Які поля Calendar Event бачить запрошена Persona поза Space і хто саме може змінити час? | Product/Access | Title/time/time zone/place baseline; other fields explicit scoped share. Organizer або Space Owner/Admin reschedule; invitee proposes. FR-38. | resolved 2026-09-24 |
| OQ-16 | Який мінімальний Release-1 calendar і contact interchange: системний імпорт, vCard та/або `.ics` файл? | Product/Interop | Consented system contacts → Personal Space only; no `.vcf` import/export, `.ics` file exchange or external provider sync in Release 1. FR-4/§5. | resolved 2026-09-24 |
| OQ-17 | Чи Space-local Search остаточно замінює раніше затверджений глобальний Search у всіх продуктових вимогах? | Product/UX | FR-21, UX, `REQ-CLIENT-022` і product bundle узгоджені на selected-Space-only. | resolved 2026-09-24 |
| OQ-18 | Чи Calendar є обов'язковою Core capability/окремою AppInstance і як вона відображається в activation/navigation? | Product/UX | Mandatory Core Calendar, hideable Space menu section, independent of Project; UJ-6/FR-37. | resolved 2026-09-24 |
| OQ-19 | Який browser support, Rust/Wasm bridge, прямий Web transport, local encrypted storage/key/recovery та Web media profile доведе FR-39 на статичному host без paid Hosted Space? | Architecture/Security/UX | Версійований Web client contract, supported-browser matrix, OWASP ASVS/XSS/storage fixtures, Web↔Android direct sync та relay-fallback/path-migration proof і E2EE calls; продуктове включення Web до Release 1 затверджено 2026-09-25. | before Web implementation fan-out and public Release 1 |
