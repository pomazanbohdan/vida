---
id: VIDA-EPICS-1-2-READINESS-2026-09-29
status: approved-product-decisions-implementation-gated
approved_by: product-owner
approved_on: '2026-09-29'
date: '2026-09-29'
scope: Stories 1.1–1.4 and 2.1–2.15
method: five-pass BMad readiness audit and primary-source technical recon
verdict: FAIL-for-production-implementation; bounded-design-and-prototypes-allowed
---

# Epic 1–2: повний зріз готовності й нові рішення

## Правило статусу

Наступний інженерний handoff: [StorageProvider v1 — кандидат контракту](../specs/spec-vida-storage-provider/provider-contract-v1.md) та [послідовність прототипів P1–P6 / SPV-01–08](../specs/spec-vida-storage-provider/prototype-dispatch.md). Повторна перевірка 2026-09-29: Rust — 7 tests passed; recovery matrix — 14 passed / 16 unproven; Android/Web source scaffolds і APK не знайдено у перевіреному зрізі. Нові storage-fixtures заплановано, не виконано; цей handoff не змінює production verdict.

Перевірено всі **19 історій** з [epics.md](epics.md), попередній [readiness-аудит](implementation-readiness.md), [затверджені R1–R20](research/technical-vida-epic-1-2-reference-behavior-decisio-2026-09-29/research.md), draft-specs Persona Recovery, Operation Envelope, Signed Log, Storage Provider, Platform Bindings, Release Compatibility та `OQ-0022/24/28/33–37/75`. Власник продукту затвердив **D1–D15 без винятків 2026-09-29**; формулювання «Погодити» у п’яти ітераціях нижче збережені як історичні питання, а їхні рекомендації є прийнятими правилами VIDA. R1–R20 і D1–D15 — продуктова поведінка, **не** виконані conformance-тести. Статуси нижче оцінюють питання BMad: чи можна реалізувати історію без самовільного винаходу спільного контракту? Відповідь для production — ні. `FAIL` стосується всього комплекту Epic 1–2; це не відхилення продуктових результатів і не заборона ізольованих доказових прототипів.

| Story | Продуктовий результат | Чого бракує до production-ready | Наступний доказ |
|---|---|---|---|
| 1.1 | Погоджено | атомарний staged→active Persona/Space, key custody | crash/restart і дві Persona на host |
| 1.2 | Погоджено | точний bundle crypto/manifest/rotation commit | fresh-profile restore + wrong/stale bundle |
| 1.3 | Погоджено | encrypted durable Note+outbox/storage profile | power-loss/reopen без false «Збережено» |
| 1.4 | Погоджено | current controller proof, provisional/reconciliation | no-witness, stale kit, concurrent restore |
| 2.1 | Погоджено | Android/Web key custody, origin-bound grant, Rust/Wasm bridge | enrollment/replay/origin change |
| 2.2 | Погоджено | direct Web↔Android Iroh/WebRTC feasibility та wire parity | release-build direct trace без relay payload |
| 2.3 | Погоджено | own relay selection, signed receipt, retry/repair | forced direct failure + encrypted fallback |
| 2.4 | Погоджено | Note overlap/conflict profile і acceptance proof | offline incompatible-edit permutation |
| 2.5 | Погоджено | stable fragment identity/merge profile | independent fragment merge |
| 2.6 | Погоджено | comparability of first accepted vs concurrent | ordered/concurrent/stale triple fixture |
| 2.7 | Погоджено | signed resolution operation citing heads | preserve both histories + new edit |
| 2.8 | Погоджено | late branch and revoked-branch proof | delayed third branch fixture |
| 2.9 | Погоджено | canonical resolution-equivalence predicate | equivalent/incompatible/unknown permutations |
| 2.10 | Погоджено | snapshot+tail, anti-entropy, mixed-version | 3-peer partition/crash/rejoin convergence |
| 2.11 | Погоджено | application receipt fields, device presence/retry | receipt survives crash; no transport-ACK inflation |
| 2.12 | Погоджено | revocation frontier/key epoch, compromised origin response | stale Web operation/re-enroll rejection |
| 2.13 | Draft-поведінка, R17–R19 схвалені | exact Tor policy order, transport isolation, fail-closed | ordinary-network leak matrix |
| 2.14 | Draft-поведінка, R20 схвалена | typed-invite verification/use under partition | QR/file/text parity + replay/race |
| 2.15 | Draft-поведінка, Tor goal схвалена | Android Tor transport/reconnection/replay | Android↔Android Tor-only Note trace |

Stories 2.13–2.15 не мають мовчки набувати статусу `approved` лише тому, що погоджені R17–R20. Вони потребують окремого story-level acceptance review після рішень нижче. Web без Tor-мережевих дій у Release 1, а Android↔Web direct — hard gate; ці вже погоджені межі не ставляться на повторне голосування.

## П’ять ітерацій уточнення

Формат кожного пункту: **питання, поставлене до затвердження → поведінка референсу → затверджене правило VIDA**. D1–D15 затверджено одним рішенням власника продукту 2026-09-29. Слово «референс» означає аналогію, а не доказ готовності саме VIDA.

### Ітерація 1 — авторизація та відновлення (1.1, 1.2, 1.4, 2.1, 2.12, 2.14)

**D1. Чи може зашифрована копія бути свідком актуальних прав?** Приклад: усі телефони втрачені; в хмарі є вчорашній backup, але вчора ввечері ще відкликали Web-пристрій. [Matrix cross-signing](https://spec.matrix.org/latest/client-server-api/#cross-signing) відокремлює recovery ключів від перевірки підписів пристроїв; [Element recovery](https://docs.element.io/latest/element-support/device-verification/how-to-ensure-you-have-a-recovery-key/) працює поруч із серверним account state. **Рекомендація:** backup сам по собі повертає тільки засвідчений у ньому frontier; називати права актуальними можна лише після звірки з незалежною чинною підписаною controller history. Інакше новий Device лишається provisional, хоча локальна робота не блокується. **Погодити саме визначення незалежного свідка й статус без нього.**

**D2. Що робити з двома успішними застосуваннями одного запрошення під час partition?** Приклад: два ізольовані чинні пристрої отримали скопійований invite-файл, обидва підтвердили його для того самого нового Device key. [Matrix device verification](https://spec.matrix.org/latest/client-server-api/#key-verification-framework) прив’язує перевірку до конкретних ключів і транзакції; офлайн-глобальної одноразовості вона не обіцяє. **Рекомендація:** однаковий invite nonce + той самий target key = один логічний grant, підписи обох пристроїв лишаються в журналі; інший target key = replay/conflict, новий доступ не розширюється до з’ясування. **Погодити видиму поведінку race; exact bytes — робота протоколу.**

**D3. Чи можна новому пристрою, відновленому без свідка, запрошувати ще один новий пристрій?** [Element](https://docs.element.io/latest/element-support/device-verification/how-to-ensure-you-have-a-recovery-key/) верифікує новий пристрій через recovery key у системі з серверним account state; для повністю автономного VIDA це не доводить найновішого відкликання. **Рекомендація:** дозволити локальне підготування/обмін запрошенням, але всі похідні grants залишити provisional, доки незалежно не звіриться controller history; UI не каже «підтверджено для всіх». Це конкретизує R2 без заборони роботи. **Погодити.**

### Ітерація 2 — браузерний Device і локальна надійність (1.1–1.4, 2.1–2.3, 2.11)

**D4. Якщо Chromium не дає persistent storage, чи може Web бути єдиною копією?** [MDN storage quotas](https://developer.mozilla.org/en-US/docs/Web/API/Storage_API/Storage_quotas_and_eviction_criteria) розрізняє best-effort і persistent; [`persist()`](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persist) може повернути `false`. **Рекомендація:** Web лишається повноцінним Device і звичайне «Збережено» не супроводжується банером; під час додавання Web один раз просимо persistent storage, а за відмови показуємо в налаштуванні стан захисту копії й радимо незалежний backup/інший Device. Не називаємо таку копію відновлюваною після очищення браузера. **Погодити UX без блокування Web.**

**D5. Дві вкладки однієї Persona одночасно змінюють Note. Чи рахувати це двома Devices?** [W3C Web Locks](https://www.w3.org/TR/web-locks/) наводить саме editor і primary-sync-tab як мотивувальні кейси; [IndexedDB](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API/Basic_Terminology) координує транзакції, але не бізнесову семантику двох UI. **Рекомендація:** один браузерний профіль/origin = один logical Web Device; вкладки використовують один journal і ексклюзивний writer/sync leader, друга вкладка бачить оновлення. Не створюємо другий голос/receipt. **Погодити модель; технічний механізм — conformance gate.**

**D6. Браузер зупинився після commit, але до відповіді UI; новий запуск не знає результату.** [IndexedDB](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction) має commit/complete межу й не гарантує однаковий power-loss profile у всіх браузерах; [Automerge sync state](https://docs.rs/automerge/latest/automerge/sync/struct.State.html) окремо зберігає peer-state. **Рекомендація:** після перезапуску відновити дію за тим самим OperationId з journal, перевірити запис та outbox, не створювати новий Note/операцію; якщо транзакція не завершена — чесно показати незбережену дію. Це деталізація R7. **Погодити однакову поведінку Android/Web; точний durability profile перевірити тестами.**

### Ітерація 3 — спільний wire, сумісність і transport (2.1–2.3, 2.10–2.12)

**D7. Яка нижня межа сумісності двох версій клієнта?** [Iroh ALPN](https://docs.rs/iroh/latest/iroh/endpoint/struct.Builder.html) узгоджує підтримані application protocols під час з’єднання, а [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html) дає deterministic CBOR, але жоден не встановлює VIDA support window. **Рекомендація:** кожний реліз читає свою й попередню major-версію envelope, якщо mandatory capability сумісна; нові mandatory semantics не активуються, доки потрібні чинні Devices не оновлені. Не встановлювати календарний строк підтримки без release evidence. **Погодити мінімальне правило; golden vectors — до implementation-ready.**

**D8. Чи може peer оголосити старий/слабший security profile, щоб обійти нове правило?** [OWASP Cryptographic Storage](https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html) вимагає керованого життєвого циклу ключів; [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html) лише кодує байти. **Рекомендація:** supported profiles/mandatory capabilities і активна Space policy входять до підписаного handshake та envelope; downgrade, що суперечить поточній політиці, відхиляємо, а не непомітно відступаємо. **Погодити принцип; suite і signature domain — інженерне рішення з crypto review.**

**D9. Що саме доводить «direct» у Web↔Android?** [WebRTC ICE](https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API/Connectivity) розрізняє `host`/`srflx`/`prflx` від `relay`; [Iroh WebRTC discussion](https://github.com/n0-computer/iroh/discussions/4024) описує окремий custom transport, а [експериментальна реалізація](https://github.com/anchalshivank/iroh-webrtc-transport) не є достатнім production-доказом. **Рекомендація:** для Story 2.2 доказом є application payload між Android і статичним Web через non-relay selected candidate/route, з підписаним application receipt і повтором у release builds; signaling/discovery може користуватися сервером, але relay не переносить payload у цьому тесті. Якщо такого доказу немає, Story 2.2 не закривається. **Погодити критерій, не технічну бібліотеку.**

### Ітерація 4 — Note merge, порядок і пізній пристрій (2.4–2.10)

**D10. Яка одиниця незалежної правки в першій Note?** [Automerge text](https://automerge.org/docs/reference/documents/text/) автоматично зводить character edits, але [Automerge conflicts](https://automerge.org/docs/reference/documents/conflicts/) може мати deterministic display winner із доступними альтернативами; це не дорівнює бізнесовому вибору VIDA. **Рекомендація:** stable ID текстового блока/абзацу плюс range-aware текстові операції всередині нього; різні блоки й доведено незалежні діапазони одного блока merge, а справді перекритий смисловий фрагмент піднімає явний Conflict, якщо merge не може довести збереження обох намірів. Не робити split за поточними номерами рядків. **Погодити межу v1; editor/CRDT fixtures доведуть точність.**

**D11. Коли два офлайн-рішення вважати однаковими, а не новим конфліктом?** [Automerge conflicts](https://automerge.org/docs/reference/documents/conflicts/) розділяє видиме значення й збережені конкурентні записи. **Рекомендація:** equivalence лише коли canonical materialized content і всі заявлені domain effects збігаються; невідомий/недоступний ефект не вгадується — лишаємо Conflict; обидва OperationId в історії. R11 уже затверджує принцип, тут погоджується критерій `unknown`. **Погодити.**

**D12. Чи можна «прибрати стару історію», коли пристрій був офлайн необмежено довго?** [Loro snapshots](https://docs.rs/loro/latest/loro/enum.ExportMode.html) розрізняє повний і shallow snapshot; shallow snapshot уже не прийме дуже старі update. **Рекомендація:** в Epic 2 не вводити age-based обрізання. GC лише після перевірного snapshot, незалежної доступної копії та збереження unresolved/conflict/revocation proof; дуже старий peer отримує snapshot+tail з перевіркою lineage, інакше явний repair. Це конкретизує R12. **Погодити, що дата останнього онлайн сама не стирає історію.**

### Ітерація 5 — Tor, безпечне перемикання та запрошення (2.13–2.15)

**D13. Що робити з outbox, створеним до вмикання Tor, але ще не надісланим?** [Tor onion-service protocol](https://spec.torproject.org/rend-spec/protocol-overview.html) захищає маршрут лише для трафіку, який ним реально проходить; [iroh-tor-transport](https://docs.rs/crate/iroh-tor-transport/latest) наразі експериментальний референс. **Рекомендація:** після commit Tor-required policy усі ще не відправлені payload-и йдуть тільки Tor; normal transport не «досилає старе». Якщо Tor недоступний, outbox чекає, локальна робота триває. **Погодити явно для queued data.**

**D14. Що показувати, коли Tor увімкнено на одному Device, а другий ще не отримав policy?** [Tor spec](https://spec.torproject.org/intro/) не робить app-level peer автоматично анонімним; [Iroh Tor transport](https://docs.rs/crate/iroh-tor-transport/latest) лише транспорт. **Рекомендація:** мінімальний автентифікований policy-only сигнал може прийти старим каналом, потім peer прозоро активує Tor; до успіху він не отримує/не відправляє protected payload. Це деталізація R19; UI для автора показує «очікує Tor на іншому пристрої», не помилкове «синхронізовано». **Погодити статус, не повторюючи рішення про сам Tor toggle.**

**D15. Коли invite скопіювали в файл або текст, чи може ним скористатися інший пристрій?** [Matrix verification](https://spec.matrix.org/latest/client-server-api/#key-verification-framework) звіряє конкретні device keys, а [Tor restricted discovery](https://spec.torproject.org/rend-spec/restricted-discovery.html) окремо керує ключами клієнтів. **Рекомендація:** QR/файл/текст — лише різні носії одного invite; redeem доводить володіння *попередньо вказаним* target Device key і чинний trusted Device підтверджує fingerprint. Скопійований носій не є recovery secret і не дозволяє підмінити target key; спроба іншого key відхиляється, а не тихо створює новий grant. **Погодити видиму відмову; replay matrix — тест.**

## Не питати власника продукту повторно: інженерні докази

1. `OQ-0022/24`: versioned recovery-bundle bytes, KDF/AEAD/nonce, key-epoch binding, independent latest-frontier witness, crash-safe guarded rotation; OWASP review + fresh-profile restore + hostile vectors. [OWASP Key Management](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html), [RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html).
2. `OQ-0036/35`: candidate provider/bridge selection by fault injection, Android+Chromium one-writer, power-loss and key protection; no claim that IndexedDB completion is an off-device backup. [MDN IndexedDB](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API/Basic_Terminology), [W3C Web Locks](https://www.w3.org/TR/web-locks/).
3. `OQ-0028/37`: signed deterministic wire profile, version/capability manifest, positive/negative golden vectors; independent reader and Rust native/Wasm byte parity. [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html).
4. `OQ-0033/34`: comparable-acceptance certificate or explicit concurrent state; Note fragment/equivalence/late-branch fixtures; snapshot+tail proof. [Automerge conflicts](https://automerge.org/docs/reference/documents/conflicts/), [Loro snapshots](https://docs.rs/loro/latest/loro/enum.ExportMode.html).
5. `OQ-0075` + Tor: release-build Android↔Web direct trace; controlled relay fallback; Android↔Android Tor-only no-leak matrix, reconnect and platform background limits. [WebRTC ICE](https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API/Connectivity), [Tor spec](https://spec.torproject.org/rend-spec/protocol-overview.html). Не підмінювати невдалий direct proof relay-only реалізацією.

## Handoff після погодження

1. D1–D15 уже `approved` без винятків; перенести їх у версіоновані контракти та acceptance fixtures без повторного запиту продуктового рішення.
2. Закрити спільні `OQ` версіонованими контрактами й decision records; вибирати бібліотеки за функціоналом, розвитком, сумісністю та conformance, а не лише номером версії.
3. Створити design/proof tasks для 1.1–1.4 та 2.1–2.15; виконати fixtures на headless Rust, Android і статичному Chromium Web; Tor — Android release-build.
4. Переоцінити кожну Story: `ready-for-implementation` лише коли її контракт визначений і залежний ризиковий proof пройдено; `done` — лише після реалізації, тестів і демонстрації. До того `FAIL-for-production` лишається чесним статусом всього Epic 1–2.

Джерела перевірено 2026-09-29. Експериментальні транспорти, браузерна реалізація та залежності потребують повторної перевірки перед pinning.
