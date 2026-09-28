---
title: 'Technical research: VIDA Epics 1–2 operation lifecycle'
type: technical
topic: 'Persona, Note, local commit, peer sync, application receipt, conflict and recovery'
decision: 'Approve a simple user journey and a safe Core transition contract for Stories 1.1–1.4 and 2.1–2.12'
source: 'Official project, platform, standards and OWASP documentation checked 2026-09-26'
status: product-decisions-approved-implementation-gated
preset: standard
validation: normal
verified_claims: 7
unverified_claims: 1
created: '2026-09-26'
updated: '2026-09-28'
---

# Технічне дослідження: один шлях операції для епіків 1–2

**Рішення, якому служить дослідження:** поведінка створення Persona і Note, локального збереження, синхронізації Android↔Web, підтвердження іншою replica, конфлікту й відновлення. Власник продукту погодив наведені нижче чотири уточнення 2026-09-28. Це **не виконаний proof** і не дозвіл позначити 16 сторіс production-ready.

## Погоджений продуктовий напрям

Один Core-протокол має незалежні докази, а звичайний інтерфейс лишається простим: **«Збережено локально» → «Синхронізація» → «Синхронізовано»**. На Web після успішного запису нотатку **просто зберігаємо**: немає окремої Web-позначки, пояснення про backup під нотаткою або додаткового діалогу після кожної зміни. Окремо з'являється **«Потрібна увага»** при відмові чи невдалому записі та **«Конфлікт»** при несумісному конфлікті. Мережевий ACK, факт доступності peer-а, копія шифротексту на relay і час пристрою ніколи не перетворюють локальну зміну на «синхронізовано». Це відповідає чинному [контракту VIDA](../../../../docs/04-specifications/operation-finality-contract.md), а не додає нову бізнес-семантику.

Технічна основа: атомарний локальний запис операції з outbox; підписана й версіонована операція; причинні залежності та права; direct-first обмін; перевірка і надійне застосування іншою **незалежною авторизованою** replica; підписаний application-level receipt. Для Note безпечні незалежні зміни зводить CRDT, але бізнесово несумісний конфлікт не ховається за його випадковим детермінованим winner. Референси надають частини рішення, **жоден із них не реалізує весь контракт VIDA**. [1][2][3][4][5][6]

### Що вже вирішено, а що пропонується

**Уже погоджено у VIDA:** рівність пристроїв однієї Persona; «синхронізовано» після першої незалежної authorized durable application replica; recovery secret і encrypted bundle окремо, ciphertext для історії окремо; Android і статичний Web як два повноцінні клієнти; Iroh direct-first, власний relay лише fallback; відсутність server/master Device; явний конфлікт несумісних непорівнюваних гілок. [Контракт](../../../../docs/04-specifications/operation-finality-contract.md), [readiness](../../implementation-readiness.md), [recovery research](../technical-vida-persona-recovery-crypto-and-bundle-2026-09-26/research.md).

**Погоджено тут:** конкретний маршрут операції, умови кожного доказу, мінімальний UI, порядок fault-handling та незмінність Android↔Web direct-path gate. Конкретні байти envelope, криптоалгоритм і назви SQLite/IndexedDB/Automerge як обов'язкових залежностей **не затверджуються** цим звітом.

## 1. Референси: що дійсно переносимо

| Референс | Встановлений факт | Висновок для VIDA / обмеження |
|---|---|---|
| SQLite | Транзакції можуть атомарно зберігати кілька пов'язаних змін; `WAL + synchronous=FULL` дає сильніший power-loss профіль, ніж `NORMAL`, але залежить від коректності ОС і носія. [1][2] | Кандидат Android storage: operation + outbox + frontier в **одній** транзакції; перевірити на фізичному Android. Не проголошувати вибір БД до gate OQ-0036. |
| IndexedDB + browser storage | `durability: "strict"` є запитом на сильніший commit; браузерний origin storage за замовчуванням best-effort, persistent storage можна запросити, але користувач усе одно може його стерти. [3][4] | Web «збережено» означає підтверджену транзакцію в цьому browser profile, **не** резервну копію й не гарантію від user clear/eviction. Надійний Web профіль треба довести тестами конкретних браузерів. |
| Iroh 1.2 | З'єднання може мати relay та direct path; після hole punching обидва можуть співіснувати. Custom transport/path selector позначені unstable. [5][6] | Сам факт Iroh connection **не доводить** direct app-payload path. Для VIDA потрібен enforced path gate/selector та тест native↔browser, а не лише напис «direct-first». |
| Iroh browser/WebRTC | Команда Iroh називає WebRTC custom transport суттєвою окремою роботою; proof-of-concept існує, але не є production гарантією профілю VIDA. [7][8] | Не ставити Story 2.2/2.3 у Done до Android↔Web direct proof і N0-free explicit relay configuration. Не підміняти провал прихованим relay-first. |
| Automerge | Sync порівнює heads і пересилає відсутні зміни по reliable ordered stream; concurrent same-property values збережено, хоча видимий winner обирається детерміновано й довільно. [9][10] | Добрий кандидат для Note merge, але VIDA має окремо перевіряти підписи, grants, acceptance і показувати бізнесовий Conflict. CRDT winner ≠ authority winner. |
| OWASP + W3C WebCrypto | OWASP вимагає lifecycle/recovery для ключів і відділення backup від локального захисту; W3C попереджає, що non-extractable WebCrypto key не захищає від зловмисного коду того ж origin та не гарантує фізичний key-store. [11][12][13] | Android Keystore захищає локальний Device key; recovery kit зберігається окремо. Скомпрометований Web origin вимагає revoke і нового enrollment, а не обіцянки, що «ключ неекспортований, отже безпечно». |

## 2. Один наскрізний сценарій — без зайвої складності для людини

### 2.1 Перший запуск: Persona й Personal Space (Stories 1.1–1.2)

1. Користувач обирає «Створити профіль». Core локально генерує Persona ID, Personal Space ID, **окремий Device keypair** та recovery authority/kit material; усе залишається `pending`, доки транзакція staging не завершилася. Закриття застосунку повертає той самий pending профіль, не створює іншого ID.
2. VIDA просить зберегти **два різні елементи**: recovery secret і encrypted bundle; пояснює одним реченням: «Обидва потрібні, щоб повернути керування після втрати всіх пристроїв». Не примушує реєструватися на сервері. На onboarding користувач підтверджує, що зберіг їх окремо; Core локально перевіряє здатність розшифрувати bundle. Це не доводить, що стороння копія справді залишиться доступною.
3. Після підтвердження Core окремо атомарно активує Persona і Personal Space. До цього protected Note створювати не можна. Екран показує «Профіль готовий». Пізніша перевірка на новому Device дає сильніший статус «Відновлення перевірено». Копія самих зашифрованих Note — **третя окрема передумова** повернення історії. [11][14]

**UX межа:** жодних crypto термінів на основному екрані; `secret`, `bundle`, «копія даних» пояснюються на окремому екрані відновлення. Немає фальшивої обіцянки, що кнопка «Я зберіг» сама створила backup.

### 2.2 Створення або зміна Note без мережі (Story 1.3)

1. Команда `CreateNote`/`EditNote` містить Persona/Space/Resource, `OperationId`, causal base/frontier, версії schema/policy та ідентичність Device. Core перевіряє доступ і формує canonical signed operation; Note payload шифрується незалежно від транспорту.
2. StorageProvider однією транзакцією фіксує **envelope + SyncLog + outbox intent + локальний frontier + необхідні локальні bytes**. Проєкція Note може записуватися разом або детерміновано відбудовуватися з committed log. Поки commit не підтверджено, UI не каже «збережено». При `quota`, disk-full або crash до commit — помилка; текст у редакторі можна зберегти як окремий recoverable draft тільки якщо він теж реально durable.
3. Після commit Note доступна офлайн, звичайний індикатор — «Збережено локально». Для одного Device без іншої authorized replica це чесний стан, не помилка. При restart outbox знаходить ту саму операцію, не створює нову. [1][2][3]

**Важливо:** Android фізичний power-loss профіль і Web transaction/quota/eviction профіль — окремі conformance докази. Для Web після підтвердженої локальної транзакції немає спеціального пояснення чи попередження при збереженні Note; збереження працює так само, як у загальному UX. Технічно слід перевіряти quota/persistence та не обіцяти Web durability понад гарантії браузера. Раніше погоджене пояснення ризику втрати *єдиної* browser-копії під час підключення Web Device лишається окремим onboarding-контуром, не частиною кожного save. [3][4]

### 2.3 Додавання Web як рівного Device (Story 2.1)

1. Web генерує **власний** Device key у цьому origin/browser profile; показує QR/короткий одноразовий код із запитом. Android після сканування показує зрозуміло: адресу origin, назву браузера/Device, Persona, обсяг доступу. Користувач один раз підтверджує.
2. Android підписує версіонований `DeviceGrant`, криптографічно зв'язаний із Web Device public key, origin, nonce, Persona/Space scope і поточним ControllerState frontier. Web перевіряє grant і зберігає його перед першим sync; повтор коду або підміна origin/key відхиляються.
3. Це **додавання пристрою**, не голос за зміни й не призначення Web підлеглим. Android і Web надалі мають однакові правила acceptance/merge. Web-origin compromise не виправляється non-extractable key; потрібен trusted revoke/rotation path. [12][13]

### 2.4 З'єднання й обмін тією самою операцією (Stories 2.2–2.3, 2.10)

1. Discovery/signaling може йти через явно сконфігуровану інфраструктуру VIDA. Peer-и проходять Iroh/VIDA handshake: endpoint/device identity, чинний grant/epoch, ALPN/protocol major, capabilities та anti-replay nonce. Не використовувати N0 relay як прихований default.
2. Для **app payload** спочатку вимагати доведений прямий шлях. Після обмеженої, виміряної спроби (точний timeout — тестовий platform profile, не вигадані 5/30 секунд) можна перейти на **власний зашифрований VIDA relay fallback**. Якщо шлях зміниться вже під час stream, selector/gate мусить зберігати правило; простий snapshot `paths()` перед відправкою недостатній. Це окремий feasibility gate для Iroh 1.2/WebRTC. [5][6][7][8]
3. Peers обмінюються causal heads/frontiers та запитують відсутні operations/dependencies. Транспорт може повторно доставити ту саму операцію через direct і relay; `OperationId` та digest роблять apply ідемпотентним. Не робити нову Note від `sync.apply`: це перенесення стану, не нова бізнес-команда. Automerge може реалізувати Note-diff exchange, але не визначає права чи outcome. [9]
4. Приймач спочатку перевіряє підпис, DeviceGrant, effective controller epoch, schema/policy, causal dependencies, payload digest і версії. Якщо чогось бракує — зберігає як pending dependency та просить її; якщо grant відкликано — типізована відмова. Невідомий mandatory field fail closed. Лише після **атомарного durable apply** й усіх залежностей приймач підписує `ReplicationReceipt(applied_at_frontier)` із точним operation/digest/frontier та власною authorized replica identity.
5. Відправник перевіряє receipt. Після **першої незалежної** authorized durable application replica показує «Синхронізовано». Два пристрої однієї Persona — дві копії, не два business approval голоси. Transport ACK або збереження ciphertext relay не дає цього стану. Історичний receipt не дорівнює гарантії теперішнього backup. [Контракт VIDA](../../../../docs/04-specifications/operation-finality-contract.md)

### 2.5 Якщо два пристрої редагували офлайн (Stories 2.4–2.9)

1. Два різні абзаци/незалежні фрагменти Note: CRDT-адаптер зводить зміни детерміновано; UI м'яко підсвічує нове, без діалогу. Історія зберігає обидві операції. Перевірити UX на реальних текстових структурах, не обіцяти ідеальне семантичне злиття кожного речення. [9][10]
2. Один і той самий несумісний фрагмент: якщо друга зміна створена зі **старої бази**, але її acceptance доведено відбулася після вже прийнятої зміни, перша лишається поточною, а пізній автор бачить актуальний текст і вибирає «Прийняти актуальне», «Застосувати моє як нову зміну» або «Зберегти копію/ревізію». Якщо другий автор **бачив** першу зміну й навмисно оновив її, це звичайна нова редакція, не конфлікт. Якщо обидві acceptance непорівнювані, **немає глобально доведеного першого**; показується явний Conflict без прихованого winner. Локальні мілісекунди — audit/display, не арбітр. [10][Контракт VIDA](../../../../docs/04-specifications/operation-finality-contract.md)
3. Вирішення — нова підписана операція, що посилається на всі відомі conflicting heads і проходить звичайну перевірку прав. Невідома раніше третя гілка може відкрити Conflict знову. Кілька однакових результатів не стирають історію різних рішень. Автоматичне library winner за actor ID не показувати як вибір власника. [10]

### 2.6 Втрата пристрою й відновлення (Stories 1.4, 2.12)

1. Новий Device отримує owner-held recovery secret + encrypted bundle, генерує **новий** Device key, відновлює recovery authority credential та перевіряє signed ControllerState checkpoint. Старий Device key не переноситься. Контент відновлюється лише з окремо доступної encrypted data replica/export. [11][14]
2. Якщо є peer/копія більш нового controller frontier, відновлення звіряє його. Якщо доступний лише старий bundle і всі інші Devices недоступні, **неможливо довести відсутність невидимих пізніших revocations/branches**. Погоджено дозволити відновити Persona й почати локальну роботу з попередженням про неперевірену актуальність мережевого стану. Recovery authority може видати новому Device необхідний локальний grant; інакше сценарій втрати всіх пристроїв зайшов би в глухий кут. Ця controller-гілка позначається provisional до звірки. При появі несумісної новішої гілки не оголошувати довільного переможця: зупинити подальше поширення нових grant/revoke з конфліктної гілки, зберегти локальні наміри та вимагати окремого протоколу reconciliation. Це продуктовий напрям, **не** завершений OQ-0022/24 алгоритм чи гарантія проти компрометації старого комплекту.
3. Якщо Web-origin скомпрометовано: trusted Device або recovery authority підписує revoke й новий epoch/grant; шифрування майбутнього контенту ротують за затвердженою політикою. Це не видалить plaintext, який шкідливий код уже бачив. [11][12][13]

## 3. Стани: доказ Core ↔ короткий напис людині

| Core доказ | Основний UI | Деталі за натисканням | Заборонений висновок |
|---|---|---|---|
| Commit ще не завершено | «Зберігаємо…» | локальний запис | Не показувати «збережено» |
| `origin.committed`, replica receipt немає | «Збережено локально» | звичайний стан очікування sync; без browser-specific пояснення при кожному save | Не «синхронізовано» |
| Іде передача / dependency repair | «Синхронізація» | direct/relay diagnostic для власника | Не трактувати connection/ACK як apply |
| Валідний signed `applied_at_frontier` від незалежної replica | «Синхронізовано» | coverage: операція, frontier, replica, час **отримання** receipt | Не «назавжди backed up» |
| Peer відхилив op або локальний write не вдався | «Потрібна увага» | причина; текст/intent не губиться | Не нескінченний spinner |
| Небезпечний несумісний merge | «Конфлікт — оберіть варіант» | обидві дозволені версії + історія | Не автоматичний winner бібліотеки |

У звичайній роботі лише одна компактна позначка без модальних вікон. Діагностика, шлях транспорту й frontier — у деталях для технічного розбору, не в нотатці. `online` показує фактичну досяжність, але не підвищує стан операції. [Контракт VIDA](../../../../docs/04-specifications/operation-finality-contract.md)

«Синхронізовано» і «Конфлікт» не виключають одне одного: інший Device може надійно зберегти обидві гілки, але бізнесове значення ще потребуватиме вирішення. **Погоджено:** у компактній позначці пріоритет має «Конфлікт», а технічний факт реплікації лишається в деталях.

## Перехресний висновок і сильний контраргумент

Сама база даних доводить локальний commit, Iroh — транспортний канал, Automerge — збіжність змін, а WebCrypto — доступні операції з ключами. **Лише їхня комбінація з власним policy/receipt шаром** може реалізувати гарантію VIDA; вибір однієї бібліотеки не закриває жодної сусідньої межі. [1][3][5][9][12]

Найсильніший контраргумент проти рекомендованого маршруту — вартість власного policy/receipt шару та direct-first WebRTC bridge: простіше було б прийняти довільний CRDT winner і relay-first. Це дало б швидший прототип, але порушило б уже погоджені рівність Devices, чесне «синхронізовано» та direct-first. Тому спрощуємо **інтерфейс**, не доказовий контракт. WebRTC/Iroh 1.2 direct і exact browser durability лишаються unverified до тестів. [3][5][7][8][10]

## 4. Відмови та перевірки до production gate

| Ін'єкція/кейс | Очікування |
|---|---|
| Crash до/після локальної транзакції | Або немає збереженої op, або вона рівно одна й відновлюється з outbox; false «збережено» немає. |
| Disk full / Web `QuotaExceededError` | Помилка збереження, текст лишається видимим лише поки реально існує; окремий durable draft тільки після підтвердженого запису. |
| Браузер user-clear/eviction | Не обіцяти повернення локальних bytes; restore лише із recovery kit + ciphertext replica. |
| QUIC ACK / ciphertext relay / pending deps | Не видавати `applied_at_frontier`, отже не показувати «Синхронізовано». |
| Duplicate direct+relay / replay | Один apply/Resource, стабільний receipt. |
| Старий або відкликаний Web grant, новий controller epoch | Не застосовувати op; автор бачить відмову та може врятувати текст як локальну копію без права публікації. |
| Три peer-и, різні порядки доставки, пізня гілка | Однакова безпечна projection або однаковий явний Conflict; немає device-rank/clock winner. |
| Два одночасні restores з тим самим старим bundle | Не видавати обом статус «єдина актуальна authority» без доказу reconciliation. |
| Path міняється direct → relay посеред payload | Дотримання direct-first/fallback gate доведене packet/path trace, не тільки UI написом. |
| Чужий origin або XSS у Web | Grant bound to origin/key/nonce; CSP, Trusted Types/output encoding, security review; non-extractable key не є захистом від same-origin malicious code. [12][13][15] |

**Етапність доказів:** (A) headless Rust fault-injection для envelope/receipt/merge; (B) фізичний Android local durability і recovery; (C) Android↔Web direct WebRTC/Iroh 1.2 + fallback trace; (D) три peers, revocation, browser clear, concurrent restore. Сторіс можна прототипувати раніше, але статус `Done/production-ready` лише після відповідних fixtures. Не встановлювати довільні timeout чи обіцяти production без вимірів.

## 5. Зафіксовані рішення та межа їх затвердження

1. **Єдиний маршрут Core:** atomic local operation/outbox commit → verified authority/causal acceptance → direct-first transfer (relay fallback) → independent durable apply → signed application receipt. Це не означає master Device чи серверний quorum.
2. **Простий UI:** три звичайні стани таблиці вище, плюс увага/конфлікт лише як виняток; «Синхронізовано» — receipt першої незалежної replica, не транспортний ACK.
3. **Web save UX, погоджено 2026-09-28:** просто зберігати Note й використовувати загальний стан «Збережено локально» після успішної транзакції; **не** додавати під кожним Web-записом пояснення про backup, спеціальну Web-позначку або діалог. Browser quota/eviction і persistence лишаються технічними conformance ризиками; onboarding-пояснення для єдиної browser-копії не повторюється при save.
4. **Conflict межа:** CRDT автоматично зводить лише безпечні незалежні зміни; непорівнювані несумісні гілки — явний Conflict; перша зміна виграє тільки коли порядок **доведено causal acceptance**, не за годинником/пакетом.
5. **Recovery freshness, погоджено 2026-09-28:** старий owner-held bundle без актуальної replica не доводить останній ControllerState, але відновлення Persona і локальна робота дозволені. Новий Device отримує локальний recovery grant; до reconciliation controller-гілка provisional. Точні правила зіткнення гілок, післякомпрометаційної ротації та прийняття її іншими peers лишаються OQ-0022/24.
6. **Hard feasibility gate, підтверджено 2026-09-28:** Android↔Web direct Iroh/WebRTC без N0 та з контрольованим relay fallback мусить мати виміряний доказ для Story 2.2; якщо стек не дозволяє enforce path, повернутися до архітектурного рішення, а не змінити обіцянку мовчки.

**Окреме UX-рішення, погоджене 2026-09-28:** коли Note вже репліковано, але текст має несумісний конфлікт, головна позначка — «Конфлікт», а «Синхронізовано» доступне в деталях. Рішення 1, 2 і 4 вище були погоджені раніше; поточні відповіді не замінюють їх.

**Наступний пакет продуктових рішень, погоджений 2026-09-28:** [десять правил для recovery, Web, relay, local-write failure, Note equivalence та mixed-version client](../../../../docs/00-governance/decision-batch-epics-1-2-2026-09-28.md). Це затверджує бажану поведінку, але не криптографічний механізм, сумісність бібліотек чи виконані fixtures. Recovery-частину виведено до [BMad spec](../../../specs/spec-vida-persona-recovery/SPEC.md) і candidate cases; інші чинні specs мають бути звірені перед production gate.

**Готовність після погодження:** цей документ закриє *семантичний дизайн* наскрізного шляху, але не OQ-0022/24/28/33/34/35/36/37/75 і не implementation-readiness verdict. Наступний документ — versioned transition/receipt contract + fixture index із конкретним encoding/crypto/storage/transport прототипними параметрами. У ньому відділити незмінні гарантії Core від замінних бібліотек.

## Карта актуальності

Для змінюваних Iroh/WebRTC/Automerge API наступна планова перевірка — **2026-10-26**; для browser storage — 2026-12-26; для стабільного SQLite pattern — 2028-09-26. Це обчислено `recon_kit.py staleness` із політикою 1/3/24 місяці. `pub_date=2026-09-26` у машинному вході — **proxy дати перевірки**, а не дата публікації джерел. Перед pin залежностей і production-рішенням перевірити API заново незалежно від цих дат. [2][3][5][8][9]

## Джерела

| № | Первинне джерело | Статус і дата перевірки |
|---|---|---|
| [1] | [SQLite transactional guarantee](https://www3.sqlite.org/transactional.html) | Офіційна документація, 2026-09-26 |
| [2] | [SQLite synchronous PRAGMA](https://www.sqlite.org/pragma.html#pragma_synchronous) | Офіційна документація, 2026-09-26 |
| [3] | [MDN IndexedDB transaction durability](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction) | Документація browser API, 2026-09-26 |
| [4] | [MDN Storage quotas and eviction](https://developer.mozilla.org/en-US/docs/Web/API/Storage_API/Storage_quotas_and_eviction_criteria) | Документація browser API, 2026-09-26 |
| [5] | [Iroh 1.2 Connection paths](https://docs.rs/iroh/1.2.0/iroh/endpoint/struct.Connection.html) | Офіційна API-документація, 2026-09-26 |
| [6] | [Iroh 1.2 Builder](https://docs.rs/iroh/1.2.0/iroh/endpoint/struct.Builder.html) | Офіційна API-документація, 2026-09-26 |
| [7] | [Iroh WebRTC discussion](https://github.com/n0-computer/iroh/discussions/4024) | Офіційний проект, 2026-09-26 |
| [8] | [WebRTC custom transport prototype](https://github.com/anchalshivank/iroh-webrtc-transport/blob/main/README.md) | Первинний репозиторій, не production evidence, 2026-09-26 |
| [9] | [Automerge Rust sync](https://automerge.org/automerge/automerge/sync/index.html) | Офіційна документація, 2026-09-26 |
| [10] | [Automerge conflicts](https://automerge.org/docs/reference/documents/conflicts/) | Офіційна документація, 2026-09-26 |
| [11] | [OWASP Key Management](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html) | OWASP, 2026-09-26 |
| [12] | [W3C Web Cryptography API](https://www.w3.org/TR/WebCryptoAPI/) | W3C, 2026-09-26 |
| [13] | [OWASP Cryptographic Storage](https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html) | OWASP, 2026-09-26 |
| [14] | [Signal Secure Backups](https://support.signal.org/hc/en-us/articles/9708267671322-Signal-Secure-Backups) | Офіційний продукт, приклад розмежування key/backup, 2026-09-26 |
| [15] | [OWASP DOM XSS Prevention](https://cheatsheetseries.owasp.org/cheatsheets/DOM_based_XSS_Prevention_Cheat_Sheet.html) | OWASP, 2026-09-26 |

[1]: https://www3.sqlite.org/transactional.html "SQLite Is Transactional"
[2]: https://www.sqlite.org/pragma.html#pragma_synchronous "SQLite synchronous PRAGMA"
[3]: https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction "MDN IDBDatabase transaction"
[4]: https://developer.mozilla.org/en-US/docs/Web/API/Storage_API/Storage_quotas_and_eviction_criteria "MDN Storage quotas and eviction"
[5]: https://docs.rs/iroh/1.2.0/iroh/endpoint/struct.Connection.html "Iroh Connection paths"
[6]: https://docs.rs/iroh/1.2.0/iroh/endpoint/struct.Builder.html "Iroh Builder"
[7]: https://github.com/n0-computer/iroh/discussions/4024 "Iroh WebRTC transport discussion"
[8]: https://github.com/anchalshivank/iroh-webrtc-transport/blob/main/README.md "WebRTC transport prototype"
[9]: https://automerge.org/automerge/automerge/sync/index.html "Automerge Rust sync"
[10]: https://automerge.org/docs/reference/documents/conflicts/ "Automerge conflicts"
[11]: https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html "OWASP Key Management"
[12]: https://www.w3.org/TR/WebCryptoAPI/ "W3C Web Crypto"
[13]: https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html "OWASP Cryptographic Storage"
[14]: https://support.signal.org/hc/en-us/articles/9708267671322-Signal-Secure-Backups "Signal Secure Backups"
[15]: https://cheatsheetseries.owasp.org/cheatsheets/DOM_based_XSS_Prevention_Cheat_Sheet.html "OWASP DOM based XSS prevention"

Змінювані API/project sources перевірити повторно безпосередньо перед вибором dependency та pin. Тутешня дата перевірки не є датою публікації. Stable OWASP/standards patterns придатні як принципи, не як сертифікація конкретної реалізації.
