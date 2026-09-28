---
title: "PRFAQ: Vida"
status: "complete"
created: "2026-09-18"
updated: "2026-09-22"
stage: 5
inputs:
  - "research/Новий Text Document (2).txt"
  - "research/vida-current-architecture.md"
  - "docs/README.md"
  - "docs/00-governance/working-agreement.md"
  - "docs/00-governance/research-traceability.md"
  - "https://easyweek.com.ua/"
  - "https://support.booksy.com/hc/en-gb/articles/16486745644306-What-is-Online-Booking-and-how-do-I-enable-it"
  - "https://www.fresha.com/pricing"
  - "https://play.google.com/store/apps/details?id=com.kyivdigital"
  - "https://digitalstate.gov.ua/projects/govtech/diia"
  - "https://github.com/pomazanbohdan/city-portal"
  - "https://github.com/pomazanbohdan/city-portal/blob/master/docs/1-analysis/brief.md"
  - "https://github.com/pomazanbohdan/city-portal/blob/master/docs/2-plan/prd.md"
  - "https://github.com/pomazanbohdan/city-portal/blob/master/docs/3-solutioning/architecture.md"
  - "https://commission.europa.eu/law/law-topic/data-protection/information-business-and-organisations/principles-gdpr_en"
  - "https://commission.europa.eu/law/law-topic/data-protection/reform/rules-business-and-organisations/application-regulation/who-does-data-protection-law-apply_en"
  - "https://www.edpb.europa.eu/sme/be-compliant/be-compliant_en"
---

# VIDA об'єднує спілкування, знання та проєкти в одному супер-апі

## Персональний і командний простір поєднує Messenger, Notes/Knowledge та Project App із задачами, чатами, форумами, файлами й дзвінками — на всіх ваших пристроях

**Україна** — VIDA представляє перший публічний реліз супер-апу для людей, які ведуть особисті справи й працюють над спільними проєктами. Замість окремих месенджерів, нотатників, task managers і файлових обговорень користувач отримує пов'язаний простір, де розмова, документ і задача не втрачають спільний контекст.

Сьогодні одна справа часто розпадається між кількома сервісами: рішення залишається в чаті, пояснення — у нотатці, файл — в іншому сховищі, а відповідальність — у task manager. Людина витрачає час не на роботу, а на пошук актуальної версії, перенесення контексту й повторне пояснення того самого команді.

VIDA дає кожному Personal Space і дозволяє активувати потрібні базові компоненти. Messenger забезпечує приватні й групові чати, форуми, файли, голосові повідомлення та аудіо- й відеодзвінки. Notes/Knowledge зберігає особисті й спільні знання. Project App поєднує задачі, підзадачі, строки, виконавців, дошки, нотатки, чати, форуми й файли. Дані лишаються корисними на власних пристроях без мережі, а користувач визначає, які Spaces і ресурси поширювати та з ким.

> «Людина не повинна збирати одну справу з фрагментів у п'яти застосунках. VIDA зберігає розмову, знання, рішення й виконання поруч — від особистої нотатки до командного проєкту — і не робить центральний сервіс єдиною умовою доступу до власної роботи».
> — Засновник Vida

### Як це працює

Новий користувач створює Persona, налаштовує профіль і персональну сторінку, а VIDA пояснює призначення bundled Messenger, Notes/Knowledge і Project App та пропонує активувати потрібні компоненти. Особистий проєкт залишається в Personal Space. Користувач може поділитися конкретною нотаткою або її розділом, а для командного проєкту — явно створити окремий Shared Space. Далі він підключає другий пристрій, запрошує учасників і працює з задачами, нотатками, чатами, форумами, файлами та дзвінками в одному контексті.

> «Раніше ми обговорювали роботу в чаті, фіксували рішення в нотатках і окремо переносили задачі на дошку. У VIDA я відкриваю проєкт і одразу бачу розмову, документ, файл та наступну дію — навіть якщо зараз немає мережі».
> — Учасник проєктної команди

### Як почати

Встановіть VIDA на Android, iOS або Windows, створіть Persona й профіль, оберіть базові компоненти та створіть перший Personal Space. Потім підключіть інший пристрій, поділіться окремою нотаткою або створіть Shared Space для групи чи командного Project. City Portal, каталог локального бізнесу й booking розвиватимуться як наступні застосунки платформи та не входять до першого релізу.

---

## Customer FAQ

### 1. Чому мені переходити з Telegram/Signal, Notion/Keep, Linear/Trello та Drive, якщо вони вже працюють?

VIDA не обіцяє одразу перевершити кожен спеціалізований продукт у кожній функції. Її перевага — один пов'язаний контекст: повідомлення, форум, нотатка, задача і файл є взаємопов'язаними ресурсами одного Space, працюють local-first/offline і не залежать від обов'язкового центрального сервера. Користувач може створити автономну Persona без реєстрації на зовнішньому сервері та без прив'язки до публічної особи. Це не є обіцянкою мережевої невідстежуваності: peers, relay або мережевий оператор можуть бачити мінімально необхідні metadata. Optional hosted services додають доступність, а не забирають контроль.

### 2. Як я перенесу до VIDA наявні нотатки, файли, задачі та контакти?

Загального імпорту нотаток, чатів, задач і файлів у поточному релізі не буде. Contacts є обов'язковим Core service: картку можна створити вручну або, після явної згоди, пов'язати із системною адресною книгою Android/iOS/Windows чи Google Contacts. Перше підключення за замовчуванням лише читає/імпортує; export або two-way sync користувач вмикає явно. Картку можна надіслати як разовий snapshot або як live share; snapshot є default, а live share доставляє дозволені майбутні зміни до revoke. Одержувач не редагує картку власника, але може мати власні локальні нотатки й теги. VIDA не виконує дедублікацію контактів; private/anonymous bindings лишаються локальними й не експортуються в device contacts.

### 3. Що працюватиме, коли немає мережі або доступний лише один мій пристрій?

Ви можете читати вже синхронізовані дані та продовжувати дозволену роботу без мережі. VIDA надійно зберігає зміни на пристрої й синхронізує їх після відновлення зв'язку. Надсилання іншій людині або дія, що потребує її підтвердження, залишається в очікуванні — локальне «збережено» не видається за віддалене «отримано».

### 4. Якщо я поділюся однією нотаткою, чи побачить людина інші дані мого Personal Space?

Ні. Resource-scoped Guest grant відкриває лише зазначену нотатку або розділ і явно додані ресурси. Прикріплений файл включається лише після підтвердження sharing; приватні backlinks, назви закритих ресурсів і решта Personal Space не розкриваються.

### 5. Чи справді VIDA може видалити повідомлення або забрати доступ із чужого офлайн-пристрою?

Після синхронізації VIDA прибере повідомлення або закриє ресурс на сумісних керованих пристроях. Але жоден цифровий сервіс не може відкликати вже зроблений screenshot, export або скопійований текст; VIDA не створює неправдивої обіцянки такого стирання.

### 6. Що буде з моїми даними, якщо компанія VIDA припинить роботу?

Open-source core, відкриті versioned specifications, portable encrypted export і незалежний reference reader є обов'язковим release gate v1. Вони мають дозволити сумісному клієнту або fork продовжити роботу з даними без VIDA commercial service. Пакет охоплює schemas, resources, relations, files і recovery metadata у межах прав користувача. Точні ліцензії та production conformance ще мають пройти окреме юридичне й технічне затвердження; до цього гарантію не можна рекламувати як уже доведену.

### 7. Скільки це коштує і які можливості залишаться безплатними?

Open core працює без підписки: локальні дані, базові Apps і direct device/peer sync не потребують VIDA Cloud. Платна підписка дозволяє прив'язати Space до постійно доступного VIDA node, який може дати durable relay/mailbox, replica/backup і blob storage. Ціна, квоти, SLA та різні trust profiles ще не визначені; підписка додає availability і recovery, але не робить відкритий формат або локальну роботу платними.

### 8. Чому немає браузерної версії і що станеться, якщо я не можу встановити застосунок?

VIDA матиме два browser modes після native v1. Free local-only mode зберігає дані у storage поточного browser profile й не обіцяє server/device sync або remote backup; UI показує, коли це єдина копія, запитує persistent storage і дає one-click encrypted export. Paid hosted mode підключається до Hosted Space і синхронізується через сервер; він також є post-v1. Android/iOS/Windows clients залишаються повноцінним першим публічним релізом.

### 9. Наскільки приватні й надійні аудіо- та відеодзвінки?

Release requirement — E2EE для 1:1 і групових аудіо-/відеодзвінків; relay може бачити мінімально необхідні мережеві metadata, але не медіавміст. Якщо production media stack не доводить E2EE і надійне відновлення для обох режимів, публічний v1 не проходить release gate. Screen sharing і recording у v1 відсутні; точні group limits і стек ще відкриті.

### 10. Чи не стане супер-ап занадто складним і важким?

Базові Apps постачаються узгодженим комплектом, але користувач активує лише потрібні, а runtime не тримає неактивні можливості без потреби. Кожен реліз вимірює delivered size, startup, memory, battery/background activity, crash/ANR і ріст локальних даних на Android/iOS/Windows. Apple/Google store ceilings — лише абсолютні межі; власні жорсткіші VIDA budgets будуть затверджені після representative vertical slice і стануть CI release gates.

---

## Internal FAQ

### 1. Якою є стратегія складання v1 без «великого вибуху»?

VIDA готується одразу як завершений public v1 без окремих customer-facing alpha або beta releases. Внутрішні збірки, automated tests, prototypes, store pre-release channels і conformance gates використовуються лише як інженерна перевірка; вони не є скороченими публічними версіями продукту.

### 2. Який один vertical slice першим доведе життєздатність VIDA Core?

Перший наскрізний доказ охоплює Persona, двох користувачів, три пристрої, Personal і Shared Space, чат, нотатку, задачу, файл, offline changes, reconnect, merge/conflict і повторну синхронізацію. Він є implementation milestone, але не окремим релізом для користувачів.

### 3. Які технічні невідомі мають отримати time-boxed prototypes до основної розробки?

Оптимальна залежнісна послідовність: мінімальний Rust API/Flutter binding smoke test → durable local store, identity і operation envelope → Iroh connection та sync двох peers → CRDT merge для notes/tasks/message log і file metadata → E2EE 1:1 call spike → group-call/keying spike → повний vertical slice. Ризикові spikes виконуються рано, але їх порядок підпорядкований залежностям, а не довільному паралелізму.

### 4. У якій послідовності створюються Android, iOS і Windows clients?

Android, iOS і Windows входять у Release 1 як окремі Flutter platform targets зі спільною UI/domain integration logic та Rust core. Кожен target має власні specifications, permissions/lifecycle, packaging/signing/distribution і conformance suite у монорепозиторії.

### 5. Коли і за якими доказами обирається Windows shell та Rust binding?

Release 1 постачає окремо запакований Flutter Windows client. Його gate перевіряє packaging, Rust FFI, accessibility, keyboard/menu/tray, startup, memory і повний core journey. Окрема нативна WinUI 3/C# реалізація можлива після Release 1; Tauri не є обраним Release-1 stack.

### 6. Яка команда реально доступна для реалізації, а не бажана в майбутньому?

Основна розробка виконується агентною системою; користувач не встановлює попереднього ліміту кількості agent roles. Це не закриває людські й зовнішні responsibilities: product authority, store accounts/signing, фізичні test devices, legal accountability і незалежна security assessment мають бути явно призначені.

### 7. Який доступний runway і який часовий горизонт до internal alpha, beta та public v1?

Грошовий runway і staffing не використовуються як обмеження scope, оскільки реалізація agent-driven. Точні календарні строки не затверджуються до dependency-based implementation plan; зовнішні витрати на Apple/Google accounts, devices, infrastructure, legal і security work все одно мають бути обліковані.

### 8. Які можливості можна перенести після v1, якщо critical path не вкладається в ресурси?

Весь уже затверджений core scope Release 1 реалізується без продуктового скорочення. Browser, Hosted Space commercial service, City Portal і WinUI 3 вже належать post-v1 та не можуть непомітно повернутися у critical path. Якщо реалізація складніша, змінюється план і строк, а не затверджений core scope.

### 9. Які security gates обов'язкові до передачі реальних приватних даних користувачів?

Security, OWASP requirements і privacy-by-design закладаються від першого architecture contract. Threat model, secure defaults, dependency/supply-chain checks, negative tests, MASVS/ASVS-aligned verification, key/recovery drills і release evidence входять у definition of done. Зовнішній pentest — незалежна ручна перевірка release-like clients та business logic; crypto-review — окрема експертна перевірка E2EE, key lifecycle, recovery і protocol implementation. Vendor і обов'язковість pre-release gate ще не затверджені; рішення приймається після стабілізації security-critical vertical slice.

### 10. Яка open-source ліцензійна й governance модель захищає і відкритість, і життєздатність бізнесу?

Open-source код VIDA публікується під MIT License. Це свідомо дозволяє використання, модифікацію, комерційне розповсюдження та hosted forks за умовами MIT. Окремі ліцензії нормативних специфікацій, документації, fixtures, trademarks і contribution governance ще мають бути юридично визначені.

### 11. Для яких юрисдикцій запускається v1 і хто відповідає за privacy/legal boundary?

Google не публікує універсальний top-10 ranking, тому VIDA об'єднує global-reach і Europe-first набори. Release 1 підтримує 18 мов: Ukrainian, English, Spanish, Portuguese, Hindi, Indonesian, Arabic, German, French, Japanese, Korean, Turkish, Chinese, Polish, Italian, Romanian, Czech і Dutch; російська виключена. Regional profiles для Spanish, Portuguese і Chinese дають 21 початковий locale profile. На поточному етапі VIDA не має призначеної legal entity/controller або store-account owner і не виконує store launch; відповідальні ролі мають бути призначені до production processing чи публікації.

### 12. Звідки прийдуть перші 100 активних користувачів core v1 без City Portal?

Перші користувачі залучаються напряму через тематичні форуми, open-source та інші релевантні спільноти. Конкретні communities, owner каналу, message, funnel і критерій «активного користувача» ще не визначені.

### 13. Яким вимірюваним результатом beta доведе, що пов'язаний Messenger + Notes + Projects кращий за набір окремих Apps?

VIDA не вводить централізоване behavioral tracking лише заради PRFAQ metrics. Базовий зовнішній показник після store launch — кількість встановлень/завантажень. Для crash/feedback проєктується local-first канал: diagnostic bundle спочатку зберігається локально, користувач переглядає/redacts його і явно надсилає як E2EE attachment до вбудованого Support Contact у Messenger; якщо Messenger зламаний, доступний encrypted export. Automatic background upload лишається окремим opt-in рішенням, а support infrastructure ще не реалізована.

### 14. Хто й як підтримуватиме протокол, сумісність, security updates і користувацькі інциденти після релізу?

Відповідальність залишається за командою VIDA, а recurring compatibility, dependency, vulnerability та incident workflows мають бути автоматизовані в агентній системі. Людина/організація, яка юридично приймає release та incident decisions, канали disclosure і service levels ще не визначені.

### 15. Який доказ змусить зменшити scope або зупинити поточний підхід замість нескінченно продовжувати розробку?

Founder не приймає kill criterion або продуктове скорочення: ціль має бути реалізована. Невдалий prototype є сигналом змінити implementation, dependency або architecture, але не відмовитися від продукту. Це свідомо означає невизначений строк і відсутність економічного stop-loss.

---

## The Verdict

### Concept strength

VIDA пройшла PRFAQ як **сильна продуктова концепція, готова до PRD, але ще не готова до реалізації без додаткової формалізації**. Головна цінність сформульована чітко: не копіювати окремо Messenger, Notion і Linear, а створити local-first середовище, де розмова, знання, задача, форум і файл є пов'язаними ресурсами одного Personal або Shared Space. Product boundary Release 1, post-v1 напрями й чесні обмеження автономності вже відокремлені.

### Forged in steel

- **Конкретний користувач і проблема.** Головний користувач веде особисті справи та командні проєкти й зараз втрачає контекст між розмовами, документами, файлами та задачами.
- **Диференціація.** VIDA поєднує пов'язаний контекст, local-first/offline роботу, автономні Personas, децентралізацію та відкритий Core; hosted services додають доступність, але не є умовою володіння даними.
- **Композиційна модель.** Personal/Shared Spaces, resource-scoped sharing і залежні AppInstances дають одну модель для Messenger, Notes/Knowledge та Project App без трьох несумісних permission/sync систем.
- **Чесні customer promises.** Документ не обіцяє абсолютної мережевої анонімності, миттєвого стирання офлайн-копій, підтвердження недоставлених дій або availability, якої не дозволяє ОС.
- **Release boundary.** Release 1 включає Flutter Android/iOS/Windows, три Core Apps, files, forums, calls і multi-device offline sync; Browser, Hosted commercial service, City Portal і WinUI 3 явно відкладені.
- **Відкритість.** MIT для open-source коду, portable encrypted export, versioned specifications і незалежний reader/fork визначені як основа неперервності.

### Needs more heat

- **PRD acceptance model.** Потрібні перевірювані functional requirements, user journeys, release gates і traceability для всього погодженого scope, а не лише список можливостей.
- **Числові NFR.** Startup, memory, battery, sync latency/success, crash/ANR, call quality, storage growth і locale quality потребують baseline від representative vertical slice та подальших release thresholds.
- **UX contract.** Onboarding, activation Apps, Persona/Space switching, sync/conflict states, recovery, permission prompts, shared editing і diagnostics потребують окремого UX artifact після PRD.
- **Adoption proof.** Store downloads не доводять, що пов'язаний контекст корисніший за набір окремих Apps; PRD має визначити privacy-preserving qualitative/opt-in evidence без централізованого behavioral surveillance.
- **Localization operations.** 18 мов/21 locale profiles затверджені як scope, але source locale, translation workflow, reviewers і release ownership ще не визначені.

### Cracks in the foundation

- **Integration scale.** Один Release 1 одночасно охоплює messaging, collaborative notes, project management, files, forums, E2EE calls, offline multi-device sync і три ОС. Agent-driven implementation не усуває integration, security, device-lab і release-governance bottlenecks. Відмова скорочувати scope означає, що schedule мусить залишатися невизначеним до architecture/prototype evidence.
- **Незакриті Core contracts.** Operation envelope/finality, CRDT/editor, storage/GC, Rust bindings, media/keying, AppPackage compatibility та open specification governance ще можуть змінити реалізацію кількох підсистем одночасно.
- **Public-release authority відсутня.** Legal controller/entity, store-account owner, signing custody, incident authority, disclosure channels і support service levels не призначені. Це не заважає плануванню й прототипам, але блокує production processing та публічну поставку.
- **Security assurance decision відкладено.** Threat modeling і OWASP verification є безперервними вимогами, але обов'язковість незалежного pentest/crypto-review ще не затверджена.
- **Немає stop-loss.** Founder свідомо не приймає product kill criterion. Тому architecture має підтримувати заміну залежностей і поетапні докази; інакше невдалий підхід може споживати ресурси без об'єктивної межі.

### Verdict

**Needs more heat — proceed to PRD.** Концепція достатньо чітка, щоб перейти до формальних product requirements. PRD не повинен повторно відкривати вже прийняті product decisions або скорочувати scope без окремого рішення. Він має перетворити широкий Release 1 на dependency-ordered, testable requirements і явно позначити prototype, architecture, legal та security gates, без яких public release неможливий.

<!-- coaching-notes-stage-1
concept_type: Комерційна open-core двостороння платформа; city portal дає дистрибуцію й довіру, B2B-функції монетизуються.
first_customer: Мешканець українського міста, якому потрібно записатися на послугу локального бізнесу.
problem_hypothesis: Дзвінки й розрізнені месенджери не показують надійну актуальну доступність; підтвердження, зміни та нагадування губляться.
solution_hypothesis: Знайти бізнес у міському контексті, побачити реальні слоти, забронювати, отримати підтвердження та оновлення в одному застосунку.
assumptions_challenged:
- "Платформа для всіх" не є стартовим сегментом.
- Super-app не може бути первинною ціннісною пропозицією.
- Онлайн-запис сам по собі не є диференціатором: EasyWeek, Booksy та Fresha вже закривають основний сценарій.
direction_selected:
- Beachhead — booking у локального appointment-based бізнесу через міський портал.
- City portal — acquisition driver; business space і повторна взаємодія — retention/monetization path.
- Consumer-facing category — супер-ап; «платформа» є архітектурним, а не головним маркетинговим визначенням.
- Core consumer promise — замінити кілька розрізнених щоденних застосунків одним узгодженим середовищем; replacement bundle для v1 ще не визначено.
- Functional bundle визначено: communications (DM, group chats, forums), knowledge (notes, documents, knowledge base), work management (personal/team tasks and projects, configurable privacy).
- Відкинуто трактування bundle як трьох окремих застосунків або лінійного journey. Це композиційні capabilities: shared project/workspace включає chats, knowledge, documents і tasks; document та task можуть мати власні context-bound discussions.
- Research підтвердив модель Space: кожен користувач має Personal Space, може створювати додаткові Spaces і стає їх Owner; shared-доступ видається контактам через roles та fine-grained grants. Space є межею ownership/security/sync, а global views можуть агрегувати дозволені ресурси з кількох Spaces.
- CRM не входить до прийнятого bundle; користувач виправив цей напрям на project/work management.
key_research_findings:
- EasyWeek майже повністю покриває базовий booking flow; Vida має відрізнятися міською дистрибуцією, довірою та довготривалим business context.
- Kyiv Digital і Diia сформували високі очікування до якості українських digital products.
- Найбільший ризик — двосторонній cold start; B2B utility має існувати до повної marketplace density.
- Confirmation та notification delivery є reliability-critical.
- Booking data потребують data minimization, retention/deletion, audit і чітких controller/processor roles.
product_context_worth_preserving:
- Один бізнес має стабільну ідентичність і business Space навіть із філіями в кількох містах.
- City partner може керувати placement/moderation, але не отримує автоматичного доступу до приватних чатів або бронювань.
- research/ є evidence; docs/ є canonical лише після explicit decision gate.
- City Portal уже розробляється як окрема centralized SaaS у приватному репозиторії pomazanbohdan/city-portal з approved Product Brief, PRD та architecture.
- Vida охоплює mobile/desktop clients, shared core, app runtime, sync і platform services; City Portal є одним із застосунків усередині Vida, а не визначенням усієї платформи.
- City Portal App взаємодіє з головним portal backend через окремий integration service/sidecar; deployment, protocol, identity mapping і source-of-truth contract лишаються відкритими.
-->

<!-- coaching-notes-stage-2
scope_correction_2026-09-21:
- Попередній city-booking beachhead із Stage 1 не є scope першого релізу; City Portal, local business і booking відкладені після core v1.
- Головний користувацький контекст v1 — людина, яка керує власними справами та/або працює в проєктній команді.
- Core v1: Persona/Profile onboarding, Messenger, Notes/Knowledge, Project App, device sync, groups/forums/files і audio/video calls.
- Notes v1 не прагне повного Notion parity; Projects v1 відкладає Gantt, time tracking, budgeting і складну аналітику.
- Технічний media stack для calls не обрано: iroh-live та WebRTC/Iroh є лише дослідними референсами.
- Customer-facing privacy promise покращено без непідтвердженої обіцянки абсолютної анонімності або доступності.
- City pilot metrics відхилені як критерій v1; потрібні окремі core-product success signals.
rejected_framings:
- Мешканець міста як головний герой першого релізу.
- Booking як перший десятихвилинний шлях і release proof.
- City/business functionality у v1.
-->

<!-- coaching-notes-stage-3
questions_opened_2026-09-22:
- Switching cost та import baseline.
- Sustainability/open-core portability and shutdown path.
- Free versus paid boundary.
- Installed-only client trade-off.
- Calls privacy/reliability guarantees.
- Measurable weight/complexity limits.
decisions_already_available:
- Main user is a person managing personal affairs and team projects.
- Bundled Apps are selectively activated.
- Personal Project stays in Personal Space; team Project uses explicit Shared Space.
- A note/section can be shared by resource-scoped access.
- Group/Project has chat stream plus forum topics.
- Calls include 1:1/group audio-video but exclude screen sharing and recording.
- Message edit marker and delete tombstone are required.
- Collaborative Notes use remote cursors/selections plus fading changed-range highlights.
- Search is local across authorized synced Spaces; each Space has Files view.
- Demonstrable core release journey is approved; numeric NFR remains.
decisions_2026-09-22:
- Differentiator доповнено децентралізацією й autonomous anonymous Persona без обов'язкового зовнішнього сервера.
- General import відкладено; Contacts виділено у canonical ContactCard + consented platform/Google connectors.
- Offline, scoped sharing і чесна межа revoke/delete підтверджені.
- Open continuity потребує forkable code/specs і portable data; exact licenses/conformance лишаються gate.
- Paid model — optional always-online Space node для relay/replica/backup/blob; open core не залежить від підписки.
- Calls потребують E2EE для 1:1/group; media contract лишається open.
- Store/resource policy researched; numeric VIDA budgets потребують representative build evidence.
decisions_closed_2026-09-22:
- Contact sharing підтримує snapshot і live share; snapshot default; live recipient read-only щодо owner card; перший connector mode — read/import-only.
- Customer promise для anonymous Persona не обіцяє network untraceability.
- Browser modes є post-native-v1; local-only mode показує ризик єдиної копії, просить persistence і дає encrypted export.
- Hosted node default — encrypted/zero-knowledge; managed replica з keys — явний opt-in Owner для конкретного Space.
- Portable encrypted export, public schemas і independent reference reader — release gate v1.
- Відсутність production-ready E2EE 1:1/group calls блокує public v1.
stage_3_result: Customer FAQ complete; ready for explicit transition to Stage 4 Internal FAQ.
-->

<!-- coaching-notes-stage-4
questions_opened_2026-09-22:
- Delivery strategy, vertical slice and prototype gates.
- Platform sequencing, Windows shell/binding decision and real team capacity.
- Runway, milestone horizon and explicit scope cuts.
- Security, licensing/governance and jurisdiction/legal ownership.
- First-100 adoption, measurable beta proof and operational ownership.
- Founder-avoidance question: explicit evidence for scope reset or stop.
initial_risk_assessment:
- Public v1 simultaneously promises Messenger, Notes/Knowledge, Projects, files, forums, E2EE calls, offline multi-device sync and Android/iOS/Windows; without staged internal delivery this is not yet schedulable.
- Critical implementation contracts remain open across operation envelope, sync finality, storage, bindings, media, collaborative editor, AppPackage compatibility and open governance.
- Hosted/browser/City Portal are post-v1 and must not absorb v1 capacity.
decisions_2026-09-22:
- Один public Release 1 без customer-facing alpha/beta; internal evidence builds лишаються обов'язковими.
- Перший vertical slice затверджено; prototype order визначено залежностями.
- Android/iOS/Windows — Flutter targets Release 1 зі спільним Rust core та окремими platform profiles; WinUI 3/C# later.
- Agent-driven development не обмежує scope; external legal/security/store/device responsibilities лишилися відкритими.
- Core scope не скорочується; implementation failure змінює architecture/plan, не product objective.
- Security/OWASP/privacy-by-design є continuous definition of done; independent assessment owner open.
- Code license — MIT; spec/docs/trademark/contribution governance open.
- Release 1 localization approved: 18 languages/21 locale profiles; Ukrainian mandatory, Russian excluded; Chinese distribution/compliance remains separate.
- No legal controller/entity or store-account owner is assigned at the documentation stage; public production remains gated on assigning them.
- Crash/feedback infrastructure is local-first and manual-preview by default, with E2EE Support Contact delivery and encrypted-export fallback; recipient governance remains open.
- Distribution starts through forums/communities; centralized behavioral telemetry rejected, store downloads remain baseline signal.
- Operations belong to VIDA team and agent system; human/legal incident authority remains open.
-->
