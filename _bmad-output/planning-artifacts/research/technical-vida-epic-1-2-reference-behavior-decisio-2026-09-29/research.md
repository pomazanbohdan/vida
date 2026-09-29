---
title: 'VIDA Epics 1–2: reference behavior and approval package'
type: technical
topic: 'Persona, recovery, operation lifecycle, sync, Web and Tor'
status: approved-product-behavior
approved_by: product-owner
approved_on: '2026-09-29'
validation: source-checked; implementation-unproven
created: '2026-09-29'
updated: '2026-09-29'
---

# Референсна поведінка → правила VIDA для погодження

Пакет R1–R20 затверджено власником продукту 2026-09-29. Це затвердження поведінки, а не доказ реалізації. Поведінка референсів не автоматично є поведінкою VIDA; потрібні conformance fixtures, платформні прототипи й тести.

## Persona та recovery

1. **PersonaId і публічна адреса.** [W3C DID](https://www.w3.org/TR/did-core/) розділяє ідентифікатор, ключі та endpoints; [Nostr NIP-05](https://github.com/nostr-protocol/nips/blob/master/05.md) прив’язує доменну адресу до публічного ключа, але контакти слідують ключу, а не довіряють іншому власнику тієї ж адреси. VIDA: незмінний PersonaId не залежить від сервера; після публікації обраний вузол зберігає довготривалу прив’язку адреси до PersonaId і актуальний підписаний controller state. Recovery повертає те саме ім’я й акаунт лише після перевірки чинної прив’язки та доказу керування; самі PersonaId/ім’я не є секретом. Якщо вузол недоступний, Persona відновлюється локально, а публічне ім’я чекає підтвердження. Якщо вузол/домен зник, те саме ім’я на ньому не гарантоване.

2. **Всі старі пристрої втрачені.** [Element](https://docs.element.io/latest/element-support/device-verification/how-to-ensure-you-have-a-recovery-key/) приймає recovery key на новому пристрої після окремого входу до server account; [Delta Chat](https://delta.chat/en/help) переносить профіль QR/backup; [SimpleX](https://simplex.chat/docs/guide/chat-profiles.html) застерігає, що стара паралельна копія може зламати з’єднання. Жоден не доводить найновіше відкликання у повністю ізольованій старій копії. VIDA: kit відновлює Persona й наявні дані; новий Android може запросити Web/Android, але нові гранти provisional, доки незалежна replica або публічний вузол не звірить актуальний controller frontier. За наявності вузла акаунт може підтвердитися відразу; без будь-якого свідка локальна гілка не називається глобально перевіреною.

3. **Дві офлайн-ротації kit.** [Element](https://element.io/help) має актуальний recovery key для server-side encrypted storage; serverless winner із двох непорівнюваних ротацій не описаний. VIDA: причинно пізніша ротація чинна; непорівнювані гілки зберігаються до явної звірки, без «переможця» за неперевіреним часом.

4. **Викрадений kit.** [OWASP](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html) вимагає compromise-recovery/re-key plan; [Signal](https://signal.org/blog/backup-improvements/) показує, що власницький recovery key може й надалі розшифровувати старі локальні архіви. VIDA: відкликати право старого kit на нові пристрої, створити новий kit, окремо ротувати ключі майбутніх даних; не обіцяти видалення вже скопійованого вмісту.

## Операція та локальне збереження

5. **Позначка «Збережено».** [SQLite](https://sqlite.org/pragma.html) документує різницю `synchronous=FULL`/`NORMAL`; [MDN](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API/Basic_Terminology) — relaxed durability і можливе очищення IndexedDB. VIDA: Android — атомарний зашифрований commit operation+payload+outbox; Web — одна завершена IndexedDB/OPFS-транзакція. Показувати просто «Збережено», без попередження при кожному записі; локальний запис не називати backup.

6. **Один wire format.** [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html) описує deterministic CBOR. VIDA: версіонований canonical envelope для Android/Web/Rust з OperationId, автором, Space, причинними батьками, schema version, payload hash і підписом; невідомі mandatory поля зупиняють affected operation, optional поля зберігаються. Вибір encoding остаточно підтвердити golden vectors.

7. **Збій після commit.** [Stripe](https://docs.stripe.com/api/idempotent_requests) застосовує idempotency key, щоб повтор не дублював дію. VIDA: UI шукає той самий OperationId у journal і відновлює його статус, а не створює нову операцію.

8. **Статичний Web та секрети.** [OWASP](https://cheatsheetseries.owasp.org/cheatsheets/HTML5_Security_Cheat_Sheet.html) попереджає: XSS може прочитати/змінити IndexedDB, browser storage сам собою не забезпечує секретність. VIDA: trusted origin/build, CSP, без стороннього виконуваного коду, захищене key wrapping; Web — окремий Device із явним додаванням/відновленням.

9. **Перший прийнятий без сервера.** [Automerge](https://docs.rs/automerge/latest/automerge/struct.Automerge.html) та [Loro](https://docs.rs/loro/latest/loro/) мають причинні heads/frontiers, але deterministic display winner не доводить фізичний час. VIDA: першість визначає перевірний спільний порядок прийняття; незалежні офлайн-дії лишаються concurrent, не вирішуються мілісекундою пристрою.

10. **Редагування нотатки.** [Automerge](https://automerge.org/docs/reference/documents/conflicts/) зберігає конфліктні значення, [Loro](https://docs.rs/loro/latest/loro/) зливає сумісні текстові зміни. VIDA: різні фрагменти — automatic merge; несумісний фрагмент/смисл — явний вибір з обома варіантами.

11. **Однаковий результат двох дій.** CRDT-історія зберігає обидві зміни навіть за одного видимого стану ([Automerge](https://docs.rs/automerge/latest/automerge/struct.Automerge.html)). VIDA: один видимий результат, дві підписані операції в історії, якщо і канонічний стан, і бізнес-наслідок однакові.

12. **Очищення journal.** [Automerge sync](https://docs.rs/automerge/latest/automerge/sync/struct.State.html) обліковує shared heads, потрібні зміни й in-flight. VIDA: compact лише після перевірного snapshot та іншої доступної копії; не видаляти останню копію або невирішені гілки; довго офлайн-пристрою надати snapshot+tail або repair flow.

## Мережа, статус і права

13. **Android↔Web напряму.** [WebRTC](https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API/Connectivity) має direct і relay candidates. [Iroh-дискусія](https://github.com/n0-computer/iroh/discussions/4024) та [експериментальний транспорт](https://github.com/anchalshivank/iroh-webrtc-transport) не є production-гарантією для всіх мереж. VIDA: пряме з’єднання — вимірюваний gate Story 2.2; формат даних не залежить від транспорту.

14. **Relay.** [Iroh](https://docs.rs/iroh/latest/iroh/) підтримує direct addresses, custom relay та fallback; direct може не пройти NAT. VIDA: власний налаштований relay для discovery/першого контакту, encrypted payload через нього після виміряної невдачі direct; шлях показувати в діагностиці, без прихованого стороннього relay.

15. **«Синхронізовано».** [Automerge sync](https://docs.rs/automerge/latest/automerge/sync/struct.State.html) відділяє in-flight від shared heads. VIDA: потрібен підписаний application receipt незалежної авторизованої durable replica на конкретний OperationId/frontier. Один Device — «Збережено»; encrypted mailbox без застосування — «Збережено для доставки». Це не обіцянка backup на всіх пристроях.

16. **Пізня доставка після revoke.** [OWASP](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html) вимагає перевірки відкликання ключів. VIDA: раніше підтверджена до revoke-frontier операція може надійти пізніше як історія; непідтверджена до відкликання локальна дія не отримує право лише через пізню доставку.

17. **Два офлайн-перемикачі Tor.** [CockroachDB](https://www.cockroachlabs.com/blog/clock-management-cockroachdb/) застосовує HLC і межі clock skew: точність до мілісекунд не доводить правильність годинника. VIDA: «пізніша дія» за часом, коли часові докази порівнювані; інакше тимчасово утримати Tor й показати конфлікт, щоб не перейти звичайним маршрутом помилково.

18. **Tor і та сама Persona.** [Tor spec](https://spec.torproject.org/intro/) описує анонімізацію мережевого маршруту, а не стирання app-level ідентифікаторів. VIDA: окремі transport endpoint keys/addresses для Tor і normal; Persona для її контактів та сама. Не обіцяти незв’язуваність двох режимів одному співрозмовнику.

19. **Вмикання Tor для існуючого Space.** Tor працює лише коли дані реально йдуть Tor-маршрутом ([spec](https://spec.torproject.org/intro/)). VIDA: старим каналом може прийти тільки мінімальний автентифікований policy-only сигнал; після нього жодних payload/attachments normal-маршрутом. Пристрій прозоро активує Tor, а якщо не може — sync чекає. Policy-only контакт сам може розкрити метадані.

20. **QR/файл/текст запрошення.** [Delta Chat](https://delta.chat/en/help) використовує QR для другого пристрою. VIDA: усі форми містять той самий typed invite, прив’язаний до нового Device key, Persona/Space, наміру та nonce; не передавати recovery secret. Глобальну одноразовість під час partition не обіцяти до звірки.

## Межі погодження

Пакет 1–20 затверджено разом. Обмеження: (1) постійність публічної адреси обмежена життям вузла/домену; (2) нові гранти зі старого kit без актуального свідка provisional; (13) прямий Web↔Android — hard gate доказу; (17) непорівнюваний час не може автоматично вимкнути Tor. Погодження правил не переводить сторіс у «реалізовано».

Джерела переглянуто 2026-09-29. Версії Iroh, браузерні API та experimental WebRTC transport необхідно перевірити перед реалізацією.
