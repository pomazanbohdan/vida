---
title: 'Technical research: VIDA multi-device, federation, and authority confirmation'
type: technical
topic: 'VIDA multi-device federation ownership confirmation'
decision: 'Choose operation derivation and conflict semantics, federated-node roles, and authority/receipt protocol for shared and booking resources'
source: 'official protocol documentation and original papers'
status: complete
preset: standard
validation: normal
created: 2026-09-19
updated: 2026-09-23
---

# VIDA: багатопристроєві зміни, федерація та підтвердження операцій

**Рішення, якому служить дослідження:** відокремити синхронізацію, виконання логіки, право прийняти зміну та доказ її прийняття для персональних і спільних ресурсів.

## Висновок для рішення

1. **Усі пристрої можуть перераховувати однаковий відтворюваний стан, але це не дорівнює багатократному створенню нових спільних об'єктів.** Для нового domain resource потрібні сталий зв'язок із вихідною подією, dedupe й політика прийняття. Дві правки тієї самої людини на несинхронізованих пристроях є concurrent, навіть якщо друга зроблена пізніше за годинником [1][2][5][6].
2. **«Федеративний вузол» краще описувати набором явних сервісних ролей.** Адреса/акаунт, relay, durable mailbox, сховище, app host і authority можуть співіснувати в одному deployment, але не випливають один з одного. Федерація сама не означає виконання приватних автоматизацій на вузлі [9][11][13][14][15].
3. **Підпис власника, M-of-N схвалення, кворум реплік і квитанція бронювання — різні докази.** Для звичайного server-owned запису достатньо визначеної authority acceptance; додаткові owner approvals потрібні за політикою для критичних дій, а replica majority — коли потрібен кластерний журнал. Timeout зовнішнього booking-запиту означає `unknown`, не «не створено» [16][17][20][21][23].

**Рішення користувача від 2026-09-19:** якщо логіка на кількох пристроях створює нотатку за однією подією, результатом є **одна спільна нотатка у Space**, не по одній на пристрій. Це фіксує продуктовий інваріант; єдиний допустимий механізм створення канонічної операції ще не обрано. Також корпоративна реєстрація створює окремий акаунт/робочий контекст, не використовує приватний ідентифікатор людини; блокування цього акаунта прибирає доступ до відповідних робочих Spaces, не блокує її інші анонімні чи публічні акаунти. Це рішення користувача, не зовнішній research claim.

## 1. Кілька пристроїв: одна подія, похідні дані, пізніше редагування

Automerge дозволяє незалежне offline-редагування, а Yjs застосовує однакові updates повторно й у різному порядку без зміни результату документа [1][2]. **Висновок для VIDA:** це гарантія збіжності певного типу даних, не гарантія, що два пристрої не створять два різні `TaskId` за однією вхідною подією. Якщо результат лише відтворювана projection, її може обчислити кожен пристрій; якщо це новий спільний ресурс, потрібні стабільний ключ походження/ідентифікатор результату та правило прийняття операції. Це проєктний висновок, а не властивість Iroh чи CRDT [1][2][3][4].

У прикладних журналах дедуплікація append за `EventId` і дедуплікація повторного виклику handler — різні межі [3][4]. Для VIDA кандидат: `sourceOperationId + handlerId/version + outputKind` як стабільний ключ виводу, але точну схему та сумісність різних версій застосунку ще треба визначити. Підпис входу доводить автора/цілісність, але не факт одноразового виконання логіки [3][4].

Якщо телефон змінив статус offline, а ноутбук пізніше відносно годинника змінив той самий статус, не побачивши телефонної операції, зміни **каузально паралельні**, навіть коли Persona одна [5][6][8]. Automerge зберігає обидва значення конфлікту, хоча показує детермінованого переможця; його LWW не спирається на wall-clock [6]. Тому «пізніший мій пристрій стає master» є можливою *продуктовою політикою*, але не фактом про причинність. Для критичних переходів/бронювань приховане LWW може втратити людський намір: потрібні базова ревізія, explicit supersede або authority validation [5][6][8].

Replay для відновлення projection не повинен повторювати зовнішній виклик; event-sourcing та transactional-outbox джерела окремо вимагають ізолювати side effects і дедуплікувати повторне споживання [7][25]. **Сумісність із чинним ADR-0012:** локальна детермінована projection уже дозволена; автоматичне створення нової доменної операції під час `sync.apply` зараз заборонене. Якщо користувач хоче саме нові спільні ресурси від обробки вхідної події на кожному пристрої, ADR потребує окремого уточнення або заміни після рішення, не мовчазного обходу.

**Контраргумент:** для суто комутативного CRDT-стану без нових ID чи зовнішніх ефектів окремий онлайн-лідер лише погіршить offline-доступність [1][2][8].

## 2. Федеративний вузол — ролі, а не один обов'язковий «суперсервер»

У Matrix homeserver зберігає акаунти й історію своїх клієнтів та федеративно обмінюється room events; окремий identity service зіставляє сторонні ідентифікатори, а application service є окремою інтеграцією для спеціальної серверної поведінки [13][14]. В AT Protocol PDS розміщує акаунт і канонічний репозиторій, Relay агрегує потоки, а AppView будує прикладне представлення; handle/DID і адреса PDS пов'язані, але це різні ролі [10][11][12]. ActivityPub прямо розрізняє client-to-server і server-to-server шари [9]. **Висновок:** федерація визначає взаємодію незалежних операторів, а не автоматичний обов'язок кожного вузла виконувати довільну бізнес-логіку користувача [9][11][13][14].

Iroh relay передає зашифровані пакети до підключеного endpoint і не є durable offline mailbox сам по собі [15]. Отже, для VIDA потрібно окремо називати capabilities вузла: `Address/Account`, `Relay`, `DurableMailbox`, `Replica/Sync`, `Catalog/AppHost` і, **лише якщо явно надано**, `SpaceAuthority` чи `AutomationExecutor`. Це запропонований поділ VIDA, а не готовий стек Iroh/Matrix/AT. Кілька capabilities можуть працювати на одному сервері, але факт розміщення не надає права розшифровувати приватний Space або підтверджувати його бізнес-зміни [9][11][13][15].

**Сумісність із чинним ADR-0004:** `богдан@company.com` може бути доменною адресою/атестацією через `ServiceBinding`, а переносна `Persona` залишається кореневою ідентичністю. Якщо бажано, щоб компанія могла без згоди користувача замінювати root identity або контролювати всі його приватні Spaces, це вже зміна прийнятого рішення, а не просто додавання вузла. **Сумісність із ADR-0006:** opaque delivery може бути capability того самого deployment, але не тотожна authority для бронювання. Користувацьке уточнення «автоматизація не проходить на федеративному вузлі за замовчуванням» узгоджується з розділенням ролей; node-hosted AppService як окрема opt-in capability потребує окремих grants.

**Контраргумент:** Matrix homeserver і AT PDS справді поєднують у одному deployment управління акаунтом і зберігання даних [11][12][13]. Тому заборона будь-якого їх суміщення була б штучною; потрібна заборона **неявного переходу повноважень**, а не обов'язково окремі машини.

## 3. Підпис власника, кворум реплік, прийняття сервером і бронювання

Тут чотири різні докази. **Підпис/дозвіл** доводить, хто ініціював або схвалив команду. **Поріг співвласників** (напр. 2-of-3) може вимагати кількох уповноважених ключів [17]. **Кворум реплік** у Raft означає стійке прийняття позиції журналу більшістю серверів у його crash-failure моделі, а не згоду двох людей [16]. **Authority receipt** доводить, що відповідний сервіс прийняв конкретну версію зміни; зовнішній `reservationId`/стан провайдера доводить окремий результат бронювання [18][19][22][23]. Змішування цих шарів у формулу «1+1» чи «2+1» створює хибне підтвердження.

Для звичайної зміни одного ресурсного власника розумний кандидат — підписаний намір + перевірка чинних прав/ревізії + стійкий запис уповноваженим сервером і квитанція з `operationId`, версією та видавцем. AT PDS показує модель канонічного signed repository без універсального кворуму власників [19]; HTTP `If-Match` показує conditional update як захист від втраченої одночасної правки [20]. Для **критичних дій спільного володіння** (передача ownership, видалення спільного Space тощо) політика може додатково вимагати M-of-N дозволів [17]. Якщо один авторитетний сервер має власний кластер, його внутрішні репліки *можуть* потребувати Raft-подібну більшість для crash tolerance — це інша вісь [16]. Це пропозиція профілів, а не затверджена VIDA-політика.

Після timeout запиту `booking.request` результат є **невідомим**, не «не відбулося»: HTTP дозволяє ситуацію, коли запис успішний, але відповідь втрачена [20]. Stripe показує same-key retry з обмеженим строком зберігання ключа; повтор із новим ключем після втрати відповіді може створити іншу дію [21]. Booking.com окремо документує request ID, reservation ID та acknowledgement повідомлень; факт отримання/схвалення запиту не завжди дорівнює створеній броні [22][23]. Для VIDA кандидат: durable `bookingIntentId` → `pending/unknown` → запит статусу в уповноваженого власника слота/провайдера → повтор **лише** за перевіреним idempotency-контрактом того API → `confirmed` лише після його квитанції/ID. Якщо API не дозволяє безпечно з'ясувати результат, потрібна ручна звірка, а не blind retry [20][21][22][23].

**Контраргумент:** один сервер-приймач простіший, але його відмова зупиняє підтвердження; кластерний консенсус дає іншу availability/durability ціну [16]. Навіть підписана квитанція сервера доводить його твердження про commit, не чесність сервера чи реальний запис у зовнішньому City Portal. Це межа довіри, яку треба задавати окремо.

**Межі доказу:** FROST [17] реалізує пороговий криптографічний підпис, але сам не задає бізнес-ролі співвласників. Stripe [21] та Booking.com [22][23][24] — приклади контрактів різних провайдерів, не обіцянка, що майбутній City Portal матиме такі самі статуси, строки dedupe чи API звірки. До архітектурного рішення потрібно перевірити контракт саме City Portal.

## Перехресний висновок та сумісність із VIDA

У наших [старих матеріалах про merge policy](<../../../../research/Новий Text Document (10).txt>) є per-field матриця, а в [матеріалах про масштабний messaging](<../../../../research/Новий Text Document (8).txt>) — «2 із 3 реплік». Перша відповідає на *яке значення показати після concurrent edits*; друга — *скільки серверів записали журнал*. Жодна не доводить право людини діяти або існування броні. Ці локальні файли — контекст VIDA, не зовнішнє підтвердження.

| Чинний документ | Наслідок нового обговорення | Статус |
|---|---|---|
| [ADR-0012](../../../../docs/03-architecture/decisions/ADR-0012-command-event-sync-boundary.md) | Дозволяє rebuild projection на кожному пристрої, але забороняє автоматичну нову бізнес-команду під час `sync.apply`. Один спільний результат прийнято як продуктову вимогу, а його canonical creation path ще не визначено. | ADR збережено; `OQ-0048` відкрито. |
| [ADR-0004](../../../../docs/03-architecture/decisions/ADR-0004-multi-axis-identity-model.md) | Корпоративна реєстрація може створити окрему Persona й ServiceAccount/Binding, не використовуючи приватну Persona; блокування вузлового акаунта стосується лише залежного робочого доступу. | Ізоляцію затверджено; controller корпоративної Persona лишається `OQ-0049`. |
| [ADR-0006](../../../../docs/03-architecture/decisions/ADR-0006-durable-delivery-and-operation-envelope.md) | Node може спільно розміщувати різні capabilities, але ciphertext mailbox не стає domain authority лише через hosting. | Чинна межа зберігається. |
| [ADR-0003](../../../../docs/03-architecture/decisions/ADR-0003-offline-revocation.md) | Звичайна revocation не вимагає кількох Owners; critical scope може вимагати M-of-N. Розширення на всі типи ресурсів потребує нового правила. | Не узагальнювати без рішення. |

### Generic policy прийняття операції — кандидат, не рішення

Замість фіксованого правила для всіх даних схема або operation family могла б задавати `AuthorityPolicy`: **хто має право запропонувати дію; скільки й чиїх схвалень вона потребує; хто виконує остаточний commit; які базова версія/causal frontier перевіряються; яка квитанція доводить саме commit; що робити з невідомим результатом після timeout**. Це не одна формула «1+1» для кожного об'єкта. M-of-N співвласників вирішує людське схвалення, а quorum реплік — стійкість одного authority; обидва можуть існувати разом або окремо [16][17][20].

Кандидатний алгоритм: `opId + objectId + schema/operation type + actor + base frontier` → перевірка grants і policy → збір потрібних owner approvals (якщо policy вимагає) → conditional commit у визначеного logical authority → durable receipt для `opId`/нової версії → синхронізація того самого результату на пристрої. Повтор із тим самим `opId` повертає той самий outcome; timeout переводить клієнт у `unknown` до query/reconcile, не до blind retry. Якщо authority — кластер, його replica quorum є внутрішнім етапом durable commit, не додатковим голосом власника [16][20][21]. **Усе це потребує user approval і conformance fixtures; з джерел не випливає автоматично, що будь-який VIDA-вузол має ці повноваження.**

### Кандидатні профілі прийняття операції

| Ресурс/дія | Хто дозволяє | Хто приймає | Що є підтвердженням | Чого це **не** доводить |
|---|---|---|---|---|
| Personal Space offline | Власник/чинний device grant | Локальна Space policy, пізніше reconcile | Signed local commit; окремий статус sync | Що інші пристрої вже одержали операцію |
| Приватний shared Space | Actor із Space grant; для critical дій — policy M-of-N | Визначена Space authority, не довільний relay | Authority receipt для `operationId`/revision | Що сервер delivery прочитав приватний payload |
| City Portal booking | Запитувач має право подати заявку; бізнесова policy керує слотом | Власник календаря/уповноважений City Portal backend | Provider booking status + reservation ID | Що один лише client ACK або approval створив бронь |
| Кластер одного authority server | Як у відповідному рядку вище | Server leader із replica quorum | Commit index/term + server receipt | Що репліки є співвласниками чи дали людську згоду |

Ця таблиця — **пропозиція для обговорення**, не затверджена policy. Зокрема, хто саме є authority для `Task`, `Note`, `Message` та `Booking`, лишається `OQ-0033`. Якщо бізнес-сервер «має пріоритет», потрібно вказати, для **якого ресурсу й операції** він є source of truth; інакше статус «100% підтверджено» вводитиме користувача в оману.

## Рекомендації для наступних рішень

### Поглиблення: що роблять референси з паралельними офлайн-правками

| Референс | Фактична поведінка | Межа для VIDA |
|---|---|---|
| Automerge [6] | Для одночасної заміни одного scalar поля показує детермінованого «переможця», але інші значення зберігає як conflicts. «Last» визначається ID операції, не часом годинника. | Збіжність копій не доводить, що видиме значення — останній людський намір. |
| Linear [26] | Зберігає офлайн-правки і повторює їх після відновлення мережі; прямо попереджає, що не впорядковує їх за часом створення, тому опис або статус можуть бути перезаписані. | Це документоване обмеження референса, не правило, яке варто копіювати для критичних полів. |
| CloudKit [27][29] | За політики `ifServerRecordUnchanged` (типової) сервер порівнює `recordChangeTag`; при застарілій версії повертає `serverRecordChanged` і три копії — client, server, ancestor — для злиття й повторної спроби. Інші save policies можуть не порівнювати тег. | Conditional acceptance дозволяє відрізнити нову правку від overwrite за старою базою, якщо відповідна policy увімкнена. |
| CouchDB [28] | Репліки показують того самого детермінованого winner, але зберігають conflicting revisions; застосунок може надати їх людині або зробити domain-specific merge. | Не слід плутати «є один видимий winner» із «конфлікт розв'язаний за наміром людини». |

**Висновок для VIDA — проєктна рекомендація, ще не рішення:** не вводити глобальне «пізніший timestamp перемагає». Для операції над статусом зберігати `baseRevision`/causal frontier. Якщо друга дія справді бачила першу, це послідовний перехід. Якщо ні — **у пропонованій VIDA** обидві операції лишаються в історії, а схема/operation family визначає: auto-merge лише за доведеного доменного правила, інакше pending conflict з явним підтвердженим новим наміром. Для неважливого поля схема може окремо дозволити deterministic LWW; для статусу задачі, бронювання або зовнішнього ефекту тихе LWW небезпечне [6][26][27][28][29]. Це пояснює вашу інтуїцію про «пізнішу мою дію»: користувач може зробити її остаточною **явною новою командою після виявлення конфлікту**, не шляхом вгадування часу двох ізольованих пристроїв.

1. Зберегти в `sync.apply` чисте, детерміноване оновлення локальних projections на всіх пристроях. Для нового спільного об'єкта розглянути окремий `DerivedOperation` зі сталими `sourceOperationId`, `ruleId/version`, `outputId`, pinned input snapshot і authority/dedupe. Це **кандидат на розширення ADR-0012**, а не прийнятий контракт; перед вибором потрібен прототип mixed-version/replay [1][2][3][4][7][25].
2. Для «пізніша моя правка — головна» дозволяти automatic supersede лише коли наступна команда посилається на відому базову ревізію або правило конкретного поля допускає LWW. Інакше показувати concurrent conflict чи вимагати authority validation для статусу/бронювання [5][6][8][20].
3. Описати federation через незалежні capabilities і grants. Доменний handle може бути підтверджений вузлом, а root Persona — переносною; node-hosted app чи service principal отримує доступ/право виконання лише явно [9][10][11][12][13][14][15].
4. Для звичайної server-owned дії розділити owner authorization, conditional commit, signed authority receipt та provider outcome; не нав'язувати всім операціям фіксований «1+1»/«2+1». M-of-N — лише на визначених критичних діях, replica quorum — на кластерному authority [16][17][19][20].

5. Для booking не повторювати `create` з новим ID після timeout. Зберігати stable intent/key, запитувати статус, повторювати лише за доведеним контрактом конкретного API; без нього — manual reconcile [20][21][22][23][24].

**Пізніше продуктове рішення, не висновок референсів (2026-09-19):** чинний Owner shared Space може повністю видалити будь-якого іншого Owner зі Space без co-owner approval/M-of-N, навіть у critical policy; Admin цього робити не може. Це уточнює п. 4: можливий quorum стосується лише інших, окремо визначених операцій. Авторитетна перевірка прав, послідовний запис і інваріант щонайменше одного Owner залишаються обов'язковими; після отримання revocation сумісний клієнт приховує керовані дані Space, але вже скопійовані дані та офлайн-пристрій неможливо дистанційно стерти. Див. [ADR-0002](../../../../docs/03-architecture/decisions/ADR-0002-default-role-presets.md) і [ADR-0003](../../../../docs/03-architecture/decisions/ADR-0003-offline-revocation.md).

## Питання, які джерела не можуть вирішити за користувача

1. Один спільний результат похідної логіки підтверджено користувачем. Хто має право подати й прийняти його єдину канонічну операцію, якщо правило запустили кілька пристроїв, і як узгодити це з ADR-0012 без повтору бізнес-команди під час `sync.apply`?
2. Для двох offline-правок однієї Persona без причинного зв'язку: чи «пізніша за людським наміром» правка повинна автоматично перекривати старішу для всіх полів, чи лише для явно налаштованих типів/полів після показу конфлікту?
3. Окремий корпоративний акаунт і невплив блокування на інші акаунти підтверджено. Чи є корпоративний акаунт окремою керованою Persona з власним controller, чи окрема Persona користувача + node-owned ServiceAccount/Binding? Які node capabilities (mailbox, sync, storage, catalog, app host) обов'язкові, а які opt-in?
4. Для яких інших операцій (крім уже вирішеного видалення іншого Owner зі Space) потрібні owner approval, серверна authority acceptance, M-of-N співвласників або реплікаційний quorum? Хто є остаточним owner/authority для City Portal booking і які proof/status API він реально надає?

## Поглиблення 23.09: authority без постійного сервера (`OQ-0033`)

Це **варіанти для рішення, не вибір механізму**. Пізніші затверджені [SyncLog](../../../../docs/04-specifications/sync-log-contract.md), [operation finality](../../../../docs/04-specifications/operation-finality-contract.md) та [реєстр OQ-0033/0034](../../../../docs/00-governance/open-questions.md) мають пріоритет над попередніми кандидатами цього звіту: рівнозначні пристрої не отримують прихованого статусу головного; `SyncLog.Accept` ще не є `authority.accepted`; локальна офлайн-зміна може бути збережена як кандидат без остаточного результату. Для несумісних непорівнюваних правомірно прийнятих status/file гілок прийнято явний conflict без winner. Це чинні рішення VIDA, не властивості Iroh.

| Механізм логічного прийняття | Що стається під час розриву мережі | Сумісність із VIDA / ціна |
|---|---|---|
| Постійно доступний уповноважений сервіс | Відокремлений кандидат зберігається локально; остаточне exclusive-claim рішення чекає сервісу | Простий спільний порядок, але сервіс не є обов'язковим для автономної Persona; його відмова затримує підтвердження |
| Єдиний призначений Device-арбітр | Інші пристрої чекають його | Не підходить як загальний принцип: порушує рівність Devices і створює неявний master |
| Логічний quorum реплік Space | Лише група з перетином quorum може видати доказ прийняття; ізольована меншість лишає exclusive кандидат pending | Можливий без сервера за визначеного членства, ротації й quorum certificate; ці протоколи/витрати ще не доведені; реплікаційний quorum не є M-of-N голосами власників |
| Оптимістичний local-first merge | Кожна частина працює локально, після контакту зводить дані | Добре для mergeable Resource, але не створює «першого глобально прийнятого» для взаємовиключної дії |
| Кілька видимих гілок + рішення людини | Конфлікт зберігає обидва варіанти до нового причинно обґрунтованого рішення | Відповідає вже прийнятому status/file правилу; не можна мовчки оголосити дві броні одного слота підтвердженими |

Межа не є недоліком Iroh: доказ Gilbert–Lynch показує, що за мережевого partition не можна одночасно гарантувати лінеаризований єдиний результат і завершення кожної операції на кожній ізольованій стороні [30]. Це **висновок із теореми для exclusive/strong-consistency операцій VIDA**, а не твердження, що весь VIDA має бути недоступним офлайн. Iroh та бібліотеки на ньому дають транспорт/реплікацію й частину control history, але не обирають за VIDA доменний authority: [Irokle](https://github.com/arunaengine/irokle) документує підписаний causal DAG, membership і bounded sync [31]; [iroh-db](https://github.com/holon-technologies/iroh-db) документує MultiWriter/SingleWriter/AppendOnly domain modes [32]. Жоден README не є proof, що його режим задовольняє `OQ-0033` або VIDA fixtures.

**Наступний decision gate:** визначити для кожної *exclusive operation family* носія логічного права прийняття (Persona/Space/зовнішній owner), механізм спільного serial order і frontier у безсерверному Space, поведінку меншості в partition, membership/revocation при зміні quorum та proof у квитанції. До цього можна прототипувати порівнювані кандидати, але не називати origin-durable чи sync receipt остаточним підтвердженням. `OQ-0034` окремо залишає equivalence predicate, інші operation families, merge overlap, copy/revision ACL і safe pruning відкритими.

**Тест на відмінність варіантів:** дві частини Space під час partition окремо створюють несумісні exclusive-заявки з одного frontier; жодна не отримує `authority.accepted` лише через локальний запис. Після відновлення зв'язку перевіряємо однаковий outcome і proof на кожній репліці. Для quorum-кандидата повторюємо split `2+1`, зміну складу та revocation: меншість чекає, stale epoch не підтверджує заявку. Для optimistic-кандидата перевіряємо, що локальна робота збережена, але UI не називає заявку остаточною. Дублі transport-пакетів мають повертати той самий результат за `OperationId`; два пристрої однієї Persona не стають двома незалежними голосами. Детальніші decision-gated fixtures — у [BMad signed-log conformance plan](../../../specs/spec-vida-signed-log-engine/candidate-evaluation-and-conformance.md).

**Паралельний `OQ-0034`:** кандидатна еквівалентність двох resolution operations потребує однакових conflict heads, canonical state, effects і schema/policy version, але зберігає обидва Operation IDs та авторство. Це не затверджений predicate. [Automerge](https://automerge.org/docs/reference/documents/conflicts/) детерміновано показує одного winner, хоча зберігає інші values у conflict map; цей автоматичний winner не є бізнес-рішенням VIDA. [Loro](https://loro.dev/docs/tutorial/encoding) підтримує shallow snapshot, але не може імпортувати updates, конкурентні до його start frontier. Отже, без протоколу stale-peer rebootstrap не можна безпечно переносити pruning з бібліотеки до VIDA. Snapshot прискорює читання, але не створює authority.

## Актуальність і перевірка

Звіт перевірено 2026-09-19: 29 джерел у додатку; усі маркери посилань зіставлено з рядками додатка. Журнал окремо розрізняє перевірені факти референсів і неперевірені/проєктні рекомендації для VIDA. Унікальні похідні операції та поведінка майбутнього City Portal потребують прототипу/контракту.

| Твердження, що може застаріти | Повторна перевірка |
|---|---|
| Ролі AT Protocol і Matrix, Iroh relay | 2027-09-19 або раніше перед фіксацією протоколу |
| Stripe, Booking.com API та поведінка Linear offline sync | 2026-10-19 або безпосередньо перед інтеграцією |
| CloudKit і CouchDB conflict patterns | 2028-09-19 або перед вибором конкретного механізму |

Для недатованих живих специфікацій початкова дата цього графіка — **дата перевірки**, а не дата публікації. Поточне твердження про Iroh relay додатково звірено зі статтею авторів від 2026-09-08 [15].

## Джерела

| № | Підтверджуваний фрагмент | Першоджерело | Публікація | Доступ | Впевненість |
|---:|---|---|---|---|---|
| [1] | Independent offline edits | [Automerge Welcome](https://automerge.org/docs/hello/) | не датовано | 2026-09-19 | висока |
| [2] | Commutative/idempotent document updates | [Yjs Document Updates](https://docs.yjs.dev/api/document-updates) | не датовано | 2026-09-19 | висока |
| [3] | Duplicate append/expected version | [Kurrent Appending Events](https://docs.kurrent.io/clients/tcp/dotnet/21.2/appending) | версія 21.2; дата не вказана | 2026-09-19 | середня для актуальних API |
| [4] | Idempotent event handlers | [EventSourcingDB Best Practices](https://docs.eventsourcingdb.io/best-practices/building-event-handlers/) | не датовано | 2026-09-19 | висока |
| [5] | Concurrent Automerge merge rules | [Automerge Merge Rules](https://automerge.org/docs/reference/under-the-hood/merge-rules/) | не датовано | 2026-09-19 | висока |
| [6] | Conflict alternatives and non-wall-clock winner | [Automerge Conflicts](https://automerge.org/docs/reference/documents/conflicts/) | не датовано | 2026-09-19 | висока |
| [7] | Replay and external side effects | [Martin Fowler, Event Sourcing](https://www.martinfowler.com/eaaDev/EventSourcing.html) | 2005-12-12 | 2026-09-19 | середня, draft pattern |
| [8] | CRDT causal/LWW model | [Shapiro et al., CRDTs](https://pages.lip6.fr/Marc.Shapiro/papers/CRDTs-beatcs-2011-06.pdf) | 2011-06 | 2026-09-19 | висока, original paper |
| [9] | C2S/S2S federation layers | [W3C ActivityPub](https://www.w3.org/TR/activitypub/) | 2018-01-23 | 2026-09-19 | висока |
| [10] | DID/handle/PDS identity | [AT Protocol Identity](https://atproto.com/guides/identity) | не датовано | 2026-09-19 | середня |
| [11] | PDS/Relay/AppView glossary | [AT Protocol Glossary](https://atproto.com/guides/glossary) | не датовано | 2026-09-19 | середня |
| [12] | PDS account and proxy roles | [AT Protocol Stack](https://atproto.com/guides/the-at-stack) | не датовано | 2026-09-19 | середня |
| [13] | Homeserver/accounts/history/identity | [Matrix Specification](https://spec.matrix.org/latest/) | living v1.19; сторінка не датована | 2026-09-19 | висока |
| [14] | Separate application service | [Matrix Application Service API](https://spec.matrix.org/v1.19/application-service-api/) | v1.19; сторінка не датована | 2026-09-19 | висока |
| [15] | Iroh relay forwards without storage | [Iroh shared relays, current engineering article](https://www.iroh.computer/blog/shared-relays) | 2026-09-08 | 2026-09-19 | висока для relay, не для VIDA mailbox |
| [16] | Majority replicated log | [Ongaro/Ousterhout, Raft](https://raft.github.io/raft.pdf) | 2014-05-20 | 2026-09-19 | висока |
| [17] | Threshold signing | [RFC 9591, FROST](https://www.rfc-editor.org/rfc/rfc9591.html) | 2024-06 | 2026-09-19 | висока |
| [18] | Federated event acceptance | [Matrix Server-Server API](https://spec.matrix.org/latest/server-server-api/) | living v1.19; сторінка не датована | 2026-09-19 | висока |
| [19] | PDS authoritative repo | [AT Protocol Repository](https://atproto.com/specs/repository) | living spec; сторінка не датована | 2026-09-19 | висока |
| [20] | HTTP preconditions/unknown outcome | [RFC 9110](https://www.rfc-editor.org/rfc/rfc9110.html) | 2022-06 | 2026-09-19 | висока |
| [21] | Bounded same-key retry | [Stripe Idempotent Requests](https://docs.stripe.com/api/idempotent_requests) | living API; сторінка не датована | 2026-09-19 | висока для Stripe, не для City Portal |
| [22] | Reservation delivery/ACK/dedupe | [Booking.com Reservations API](https://developers.booking.com/connectivity/docs/reservations-api/reservations-overview) | living API; сторінка не датована | 2026-09-19 | висока для inbound API |
| [23] | Inquiry/RTB/request vs reservation | [Booking.com Request to Book](https://developers.booking.com/connectivity/docs/request-to-book/overview) | living API; сторінка не датована | 2026-09-19 | висока для documented modes |
| [24] | RTB 2.0 approval path | [Booking.com RtB 2.0](https://developers.booking.com/connectivity/docs/request-to-book/rtb-2.0/overview) | living API; сторінка не датована | 2026-09-19 | висока для documented mode |
| [25] | Outbox duplicates/idempotent consumer | [AWS Transactional Outbox](https://docs.aws.amazon.com/en_en/prescriptive-guidance/latest/cloud-design-patterns/transactional-outbox.html) | не датовано | 2026-09-19 | висока |
| [26] | Offline retry, no created-date ordering and overwrite risk | [Linear: Real-time sync and offline](https://linear.app/docs/get-the-app) | living docs; сторінка не датована | 2026-09-19 | висока для документованої поведінки Linear |
| [27] | Record change tag, client/server/ancestor conflict | [Apple CloudKit serverRecordChanged](https://developer.apple.com/documentation/cloudkit/ckerror/serverrecordchanged) | living docs; сторінка не датована | 2026-09-19 | висока для CloudKit; механізм VIDA окремий |
| [28] | Deterministic winner plus retained conflicting revisions | [Apache CouchDB replication and conflicts](https://docs.couchdb.org/en/stable/replication/conflicts.html) | living 3.5 docs; сторінка не датована | 2026-09-19 | висока для CouchDB |
| [29] | Save-policy-dependent change-tag comparison | [Apple CloudKit savePolicy](https://developer.apple.com/documentation/cloudkit/ckmodifyrecordsoperation/savepolicy) | living docs; сторінка не датована | 2026-09-19 | висока для CloudKit |
| [30] | Неможливість одночасної лінеаризованості й повної доступності під partition | [Gilbert і Lynch, Brewer's Conjecture](https://www.cs.princeton.edu/courses/archive/spr22/cos418/papers/cap.pdf) | 2002, original paper | 2026-09-23 | висока для формальної межі; проєктне застосування зазначено як висновок |
| [31] | Signed causal DAG, membership і bounded sync; не generic domain authority | [Irokle current README](https://github.com/arunaengine/irokle) | current main, GitHub release не опублікований на дату перевірки | 2026-09-23 | висока для документованого surface, не для VIDA conformance |
| [32] | MultiWriter/SingleWriter/AppendOnly modes, capabilities і sync | [iroh-db current README](https://github.com/holon-technologies/iroh-db) | current main; останній default-branch commit 2026-07-21 за окремим activity digest | 2026-09-23 | висока для документованого surface, не для VIDA conformance |
