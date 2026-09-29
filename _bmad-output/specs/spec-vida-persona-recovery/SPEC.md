---
id: SPEC-VIDA-PERSONA-RECOVERY
status: draft-decision-gated
companions:
  - recovery-cases.md
  - recovery-bundle-contract.md
  - ../../../docs/02-requirements/identity-requirements.md
  - ../../../docs/04-specifications/identity-domain-contract.md
  - ../../planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md
sources:
  - ../../planning-artifacts/research/technical-vida-persona-recovery-crypto-and-bundle-2026-09-26/research.md
---

# Persona recovery and new-Device enrollment

## Why

Власник автономної Persona має зберігати контроль після втрати пристрою, не передаючи приватну ідентичність федеративному вузлу. Відновлення права керувати Persona та відновлення історії — різні результати: для другого потрібні ще доступні зашифровані дані.

## Capabilities

- **CAP-1**
  - **intent:** Власник створює і зберігає recovery material окремо від пристрою під час створення автономної Persona.
  - **success:** Setup показує вже створений секрет і надає зашифрований bundle для окремого збереження, відновлює незавершений крок після restart, вимагає підтвердження збереження обох частин і не відправляє секрет федеративному вузлу; лише після підтвердження активує ту саму Persona. UI попереджає, що втрата всіх Devices разом із комплектом може бути незворотною, а підтвердження не доводить існування копії.
- **CAP-2**
  - **intent:** Власник додає новий Device через чинний довірений Device або власний recovery authority, не змінюючи Persona.
  - **success:** Новий Device має окремі ID і ключі для кожної Persona; trusted enrollment перевіряє чинний `ControllerState` і видає підписаний `DeviceGrant` у відповідному scope. Незалежні чинні дозволи Personal і Work можуть співіснувати на одному фізичному пристрої без автоматичного розширення прав між Personas або Spaces. Коли після втрати всіх Devices свіжий frontier недоступний, recovery authority може видати лише явно provisional локальний grant до подальшої звірки.
- **CAP-3**
  - **intent:** Власник відновлює контроль і лише ту історію, для якої залишилися доступні зашифровані дані.
  - **success:** Restore окремо показує результат відновлення authority, data keys і content; секрет без bundle не видається за відновлення authority, а комплект без доступної зашифрованої копії ресурсів не видається за відновлення історії. Стан останньої перевіреної копії не приписує їй пізніші локальні зміни. Якщо всі старі Devices недоступні й актуальність controller history неможливо перевірити, новий Device може отримати необхідний локальний recovery grant і працювати в явно provisional гілці до звірки, не видаючи її за перевірену спільну authority.
- **CAP-4**
  - **intent:** Власник відкликає втрачений або скомпрометований Device без втрати своєї Persona.
  - **success:** Старий `DeviceGrant` перестає діяти, affected envelopes/epochs змінюються, а stale/replayed transition не повертає доступ. При підозрі на витік recovery secret і bundle власник запускає окрему термінову ротацію комплекту й переузгодження Devices; вже скопійований стороннім plaintext не вважається відкликаним.
- **CAP-5**
  - **intent:** Власник за бажанням додає зашифровані зовнішні копії recovery bundle і ресурсів, не плутаючи їх із локальним збереженням.
  - **success:** Залишається документований незалежний шлях збереження; відсутня, застаріла чи недоступна копія показує явний результат, а не хибну гарантію відновлення. «Перевірено» стосується лише копії, яку після запису можна було повторно прочитати, автентифіковано розшифрувати й перевірити щодо заявленого покриття; це не доводить наявність незалежної доступної копії після втрати Device. Сам export і підтвердження користувача показуються окремо.

## Constraints

- `PersonaId`, `DeviceId`, `EndpointId` і `ServiceAccountId` різні; кожний Device має окремі ключі та відкличний grant.
- Для спільно визнаних `DeviceGrant`/revoke потрібна перевірна controller history; recovery authority може видати необхідний локальний provisional grant без заяви про актуальність для інших peers. Вузол не стає controller приватної Persona через attestation чи відновлення власного акаунта.
- **[APPROVED — product owner, 2026-09-26]** Власник окремо зберігає випадковий recovery secret і версійний зашифрований recovery bundle; для історії потрібна ще доступна копія encrypted resources. Bundle не містить звичайних Device private signing keys; новий Device створює власні ключі. Деталі у [recovery-bundle-contract.md](recovery-bundle-contract.md).
- Корпоративне відновлення не відкриває анонімну чи приватну Persona; секрети й між-Persona linkage не потрапляють у logs, telemetry або публічні проєкції.
- Key lifecycle і захист exported material перевіряються за [OWASP Key Management](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html); локальне зберігання чутливих даних у мобільному клієнті — за [MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/). UX і фактичне відновлення перевіряють власні fixtures VIDA; жодне з цих джерел не затверджує її конкретний формат.
- **[APPROVED — product owner, 2026-09-26]** Незавершений локальний bootstrap не дорівнює `PersonaCreated`: staged Persona/Space/Owner authority не відкривають protected Space work до durable confirmation; після restart відновлюється той самий pending стан. Це правило VIDA, а не вимога Delta Chat чи OWASP; перевірки наведені в [recovery-cases.md](recovery-cases.md).
- **[APPROVED — product owner, 2026-09-28]** Два незалежні відновлення тим самим комплектом можуть зберігати локальні дані. Після зустрічі сумісні гілки об'єднуються, а несумісні controller/grant transitions не мають переможця за часом чи Device; спірні authority-дії чекають перевірного reconciliation. Доведено відкликаний комплект не публікує локальних операцій; автор може врятувати текст після нової авторизації.
- **[APPROVED — product owner, 2026-09-28]** Canonical controller history — підписані append-only послідовності подій окремих Devices із причинними посиланнями на відомий frontier; `ControllerState` є похідною проєкцією, а не єдиним змінюваним документом. Сумісні `DeviceGrant`, revoke, recovery і rotation переходи зводяться без пріоритетного пристрою. При одночасних несумісних grant/revoke чи recovery зберігаються обидві гілки; спірний новий доступ не визнається спільно до підписаного рішення з посиланням на обидві гілки. DID-документ, якщо буде, лише проєкція; вибір DID-методу лишається відкритим. Точний підписувач reconciliation та доказ актуальності — ще `OQ-0022/0024`.
- **[APPROVED — product owner, 2026-09-28]** Незалежні чинні підписи додавання різних Devices об'єднуються автоматично. Якщо для вже авторизованого Device виявлено одночасні несумісні renewal/revoke, peers, які побачили конфлікт, зупиняють його нові захищені читання/записи, видачу ключів і прийняття операцій до явного рішення власника; це не видаляє раніше отриманий plaintext. Власник обирає результат на будь-якому чинному перевіреному Device, доступ якого не є спірним; підписане рішення посилається на обидві гілки. Після подвійного відновлення без перевірного актуального frontier обидві гілки працюють локально як provisional до порівняння й явного вибору власника; спільна authority потребує окремого доказу. Запрошення нового Device через QR, файл чи текст короткоживуче й одноразове, прив'язане до нового ключа і потребує явного підтвердження на чинному trusted Device; recovery після втрати всіх Devices — окремий flow. Точні правила proof та enforcement у розділеній мережі лишаються `OQ-0022/0024`.
- **[APPROVED — product owner, 2026-09-28]** Звичайна ротація вимагає нового комплекту й наполегливого запиту зберегти його без блокування щоденних редагувань. Підозрюваний витік обох частин вимагає негайного відкликання старого комплекту; жодна ротація не стирає вже викрадених копій.
- **[APPROVED — product owner, 2026-09-29]** Фізичний пристрій може містити кілька Personas з окремими логічними Device IDs/ключами. Чинні Devices можуть окремо дозволити Personal і Work на цій інсталяції; кожен дозвіл діє лише у власному Persona/Space scope. Різні джерела дозволів не створюють автоматичного доступу до інших Personas або Spaces.
- **[APPROVED — product owner, 2026-09-29]** Локальна робота триває, якщо зовнішня копія недоступна; «збережено локально» не означає «можна відновити після втрати пристрою». UI показує останній перевірений backup frontier/покриття, не обіцяє відновлення пізніших змін і не робить з відсутності змін у старому snapshot висновку, що їх ніколи не було.
- **[APPROVED — product owner, 2026-09-29]** Нову копію називаємо перевіреною лише після запису, повторного читання, автентифікованого розшифрування/розбору та перевірки manifest/покриття; до цього попередня справна копія того самого класу зберігання зберігається. Перевірка файла не доводить, що незалежна off-Device копія доступна. Якщо destination не дає повторно відкрити файл, повідомляємо лише про створений export. Автоматизоване пробне відновлення у свіжому профілі після першої копії з даними та ротації комплекту є release gate, не обов'язковою дією кожного користувача й не тестом після кожного редагування; особистий guided test добровільний. Gate Story 1.2 лишається підтвердженням окремого зберігання, а не доказом наявності копії.
- **[APPROVED — product owner, 2026-09-29]** Звичайна ротація двофазна: спершу створити новий secret і bundle, перевірити export там, де destination дозволяє readback, отримати підтвердження власника про окреме зберігання; лише потім зафіксувати підписаний controller transition, який відкликає старий комплект. Crash до transition відновлює незавершений процес зі ще чинним старим комплектом; звичайні редагування не блокуються. Підозрюваний витік лишається окремим терміновим flow. Атомарність commit і його поширення між Devices ще `OQ-0022/0024`.

## Non-goals

- Ця чернетка не затверджує байтовий encoding, конкретні KDF/AEAD/nonce, конкретне сховище, криптографічний механізм ротації чи production enrollment proof; обов'язкового server escrow немає.
- Корпоративну controller/recovery policy не переносимо на приватну Persona і не закриваємо тут.

## Success signal

**[APPROVED — product owner, 2026-09-29]** Під час звичайної ротації зміна controller frontier або data-key epoch між підготовкою bundle і signed commit зупиняє саме commit: новий комплект перебудовується для актуального стану й окреме зберігання підтверджується повторно. Звичайні редагування не зупиняються. Невдалий fresh-profile test після commit показує критичний ризик відновлення; чинний trusted Device може створити виправлений комплект, але старий відкликаний комплект не активується автоматично. Без чинного Device чесно показується межа відновлення. Точний guard/repair proof лишається `OQ-0022/0024`.

**[APPROVED — product owner, 2026-09-29]** Зашифрований backup можна запускати вручну або за добровільним розкладом для Persona на налаштованому Device та обраному destination; secret не завантажується автоматично. Помилка backup не блокує локальну роботу. У налаштуваннях Persona постійно, але ненав'язливо показується ризик, коли немає перевіреної незалежної off-Device копії або backup відстає від локального frontier. Попередній перевірений snapshot ресурсів не видаляється автоматично після перевірки нового: очищення є окремою явною дією. Це не продовжує authority відкликаного recovery kit.

**[APPROVED — product owner, 2026-09-29]** Непідтримувана обов'язкова версія bundle зберігається незмінною, частковий import заборонено, потрібне оновлення клієнта; інші функції VIDA продовжують працювати. Автоматизований fresh-profile restore після першого backup з даними та ротації — release gate. Особистий guided restore для користувача добровільний; неперевірену ним копію не називаємо «відновлення перевірено».

Сценарії [REC-F01–F29 та REC-F09a](recovery-cases.md) мають демонструвати staged/restart/finalize lifecycle, окреме збереження двох частин, provisional restore, concurrent restore, scoped grants одного Device, одноразове enrollment-запрошення, guarded rotation і repair, нові ключі Device, відмову replay, перевірку backup, release-gate restore та чесну межу відновленої історії. Це план випробувань, а не production-ready протокол.

## Open Questions

- **Product behavior approved 2026-09-29:** [D1–D3, D15](../../planning-artifacts/implementation-readiness-epics-1-2-2026-09-29.md): backup frontier alone is not latest-rights proof; duplicated same-key invitation is one logical scoped grant, another key is rejected/conflicted; grants derived from no-witness recovery stay provisional; copied invite cannot silently substitute target key. The following questions are implementation/proof gates, not renewed product votes.

- `OQ-0022/0024`: сумісні signed grants різних Space scopes для того самого логічного Device/key однієї Persona складаються лише у своїх дозволених межах; відкриті їх wire representation та merge proof. Також відкриті: proof чинності підписувача reconciliation і frontier без привілейованого Device/сервера; одноразовість запрошення при partition; proof відкликання старого комплекту; byte format, KDF/AEAD/nonce, causal-cut/closure backup, bundle-to-controller/key-epoch binding, commit-time guard і crash-safe repair/rotation, integrity/version/ownership validation, manifest visibility та cross-platform restore fixtures.
- `OQ-0049`: хто контролює окрему корпоративну Persona і які саме grants може відновити або припинити компанія?
