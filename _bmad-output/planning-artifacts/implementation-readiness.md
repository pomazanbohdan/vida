---
id: VIDA-EPICS-1-2-IMPLEMENTATION-READINESS
status: review
date: 2026-09-26
scope: approved Stories 1.1-1.4 and 2.1-2.12 only
method: BMad Review plus scoped Sprint Planning readiness gate
verdict: FAIL-for-production-implementation; bounded-prototypes-allowed
---

# Готовність епіків 1–2 до реалізації

## Висновок і межа

У [epics.md](epics.md) затверджено **16 сторіс** двох перших епіків. Вони достатньо конкретні як погоджений *продуктовий backlog* і задають ранню Android↔Web демонстрацію, але **не є готовим до production-реалізації технічним контрактом**: кілька сторіс потребують ще не ухвалених байтових, криптографічних, транзакційних і authority-правил. За BMad readiness gate результат для реалізації всього обсягу 1–2 — **FAIL**: розробнику довелося б самостійно вигадувати поведінку, яка має бути спільною для всіх пристроїв. Це не скасовує схвалені сторіс і не забороняє обмежені доказові прототипи. Жоден запланований fixture тут не позначено як виконаний.

Перевірено [PRD](prds/prd-vida-2026-09-22/prd.md), [архітектурний spine](architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md), UX [EXPERIENCE.md](ux-designs/ux-vida-2026-09-22/EXPERIENCE.md), [DESIGN.md](ux-designs/ux-vida-2026-09-22/DESIGN.md), [контурну мапу](../../docs/00-governance/contour-status.md) і відповідні BMad specs. PRD і spine мають статус `final` / `accepted`; UX лишається концептним `discovery`; усі перевірені технічні spec kernels лишаються `draft` або `draft-decision-gated`. Статус документа не підміняє виконаний тест.

Епіки 3–10 тут **не деталізуємо**. Їхні затверджені результати лише обмежують повторне використання контрактів: Note-прототип не доводить Messenger, Project, Files, calls чи повну Web parity. FR-39 у цьому зрізі означає Android↔Web основу, а не закриття всієї вимоги Release 1.

**Зміна scope після цього зрізу (2026-09-28):** опціональний Tor-маршрут для будь-якої Persona й окрему вимогу взаємного Tor погоджено як Release-1 цілі на Android/iOS/Flutter Windows; статичний Chromium Web залишається повним звичайним клієнтом без Tor-мережевих дій. Після цього readiness-аудиту Tor-послідовність у [epics.md](epics.md) переглянуто: draft Stories 2.13–2.15 охоплюють перемикання маршруту, авторизоване додавання Android Device та синхронізацію нотатки. Оцінка вище охоплює тільки початкові 16 сторіс і не доводить готовність нових історій або platform fan-out; Story 2.2/2.3 не розширюються заднім числом.

## Перший масовий контур

**Наскрізний контракт Core-операції та її доказів** перетинає обидва епіки: `команда/створення Persona або Note → атомарний локальний запис → підписана операція/контрольний перехід → перевірка прав і authority → передача → застосування іншою replica → підписаний application receipt → однакова проєкція або явний Conflict`. Він встановлює окремі значення для `staged`, `збережено локально`, `accepted за політикою Personal Space`, `синхронізовано` та `відхилено/невідомо`. Ні Iroh ACK, ні перший отриманий пакет, ні годинник, ні тип пристрою не є business authority. Це семантичний контракт, не вибір бібліотеки чи сервера.

Його не можна замкнути одним файлом: він пов'язує [recovery](../specs/spec-vida-persona-recovery/SPEC.md), [operation envelope](../specs/spec-vida-operation-envelope/SPEC.md), [signed log](../specs/spec-vida-signed-log-engine/SPEC.md), [storage](../specs/spec-vida-storage-provider/SPEC.md) і [platform bindings](../specs/spec-vida-platform-bindings/SPEC.md). Першим спільним deliverable має стати **версіонований контракт переходів/receipt і fixture index**: хто підписує, який frontier доводиться, що саме commit-иться атомарно, який стан бачить UI та яка негативна перевірка його забороняє. Суміжні байтові рішення й runtime evidence ідуть нижче.

| Етап спільного контракту | Мінімальний доказ | Що дозволено показати | Чого доказ **не** означає |
|---|---|---|---|
| Pending Persona/Space bootstrap | Повний локальний staging commit; після restart ті самі IDs | «Налаштування не завершено» | Активну Persona, захищену Note або перевірений recovery backup |
| Активована Persona і локальна Note | Durable recovery confirmation; потім окремий durable Note/operation commit | «Збережено локально» | Наявність другої копії або доступність іншому Device |
| Authority acceptance | Перевірений controller/grant, causal base і політика Personal Space | Прийнятий локальний стан за визначеним frontier | Peer replication, recipient delivery або незворотний зовнішній effect |
| Peer apply і application receipt | Незалежна authorized replica перевірила й durably applied точний operation/frontier та підписала receipt | «Синхронізовано» в оголошеному scope | Поточну наявність backup назавжди або прийняття будь-якою третьою стороною |
| Непорівнювані несумісні acceptances | Обидва докази чинні; безпечного merge немає | Явний Conflict із дозволеними варіантами | Автоматичного winner за часом, Device ID чи порядком доставки |

Ця таблиця фіксує вже погоджені *значення* станів; алгоритм доказу authority, байти receipt і crash-атомарність залишаються окремими рішеннями 1–4 нижче.

## Матриця всіх наявних сторіс

`P` — можна прототипувати з явно тимчасовим адаптером; `D` — бракує нормативного рішення; `E` — потрібен виконаний conformance/feasibility доказ перед закриттям сторіс. Позначки не означають, що сторіс треба переписати з нуля.

| Story | Вимога / продуктова обіцянка | Основний незакритий контракт або доказ | Gate |
|---|---|---|---|
| 1.1 | FR-1: локальна Persona/Space, pending setup, ті самі IDs після restart | OQ-0024 controller/bundle lifecycle; OQ-0036 атомарний bootstrap, захист ключів | P, D, E |
| 1.2 | FR-3: окремо збережені secret і bundle; активація після durable confirmation | OQ-0024 формат, integrity, ротація/re-export; відрізнити підтвердження користувача від перевіреної копії | P, D, E |
| 1.3 | FR-1 + ранній FR-22: видима проста Note, durable reopen, Persona isolation | OQ-0036 transaction/failed-write; стабільний мінімальний Resource/Note profile без demo-only моделі | P, D, E |
| 1.4 | FR-3: повернути authority без старого Device; контент лише з доступного ciphertext | OQ-0022/24 поточний ControllerState, stale bundle, два concurrent restores, новий DeviceGrant | D, E |
| 2.1 | FR-2 + FR-39: схвалити Web як рівнозначний Device | OQ-0022/24 grant/epoch; OQ-0035 native/Wasm boundary; origin/key binding, replay | P, D, E |
| 2.2 | FR-23 + FR-39: та сама Note напряму Android↔Web | OQ-0028 canonical operation/receipt; OQ-0035 bridge; OQ-0036 Web durability; OQ-0075 Iroh 1.2/WebRTC direct-path feasibility | P, D, E |
| 2.3 | FR-23 + FR-39: relay лише після обмеженої невдалої прямої спроби | виміряний direct-attempt budget; explicit VIDA relay allowlist; same-operation dedupe та application receipt | P, D, E |
| 2.4 | FR-25: дві несумісні Note-гілки без вигаданого winner | OQ-0033/34 acceptance proof та Note Conflict profile, права на варіанти | D, E |
| 2.5 | FR-24: незалежні зміни абзаців зливаються | OQ-0034 fragment/merge policy; OQ-0073 editor/CRDT conformance | P, D, E |
| 2.6 | FR-24/25: перша *доведено* прийнята зміна лишається поточною | OQ-0033/34 порівнюваний authority order; stale intent і повторне редагування | D, E |
| 2.7 | FR-25: resolution є новою операцією з усіма heads | OQ-0033/34 causal base, поточні права, збереження історії | D, E |
| 2.8 | FR-25: невідома пізня гілка знову відкриває Conflict | OQ-0033/34 comparable/incomparable proof, revoked branch, replay | D, E |
| 2.9 | FR-25: одночасні resolution: еквівалентність або новий Conflict | OQ-0034 canonical equivalence predicate, невідома еквівалентність | D, E |
| 2.10 | FR-2/23/24: ремонт історії та збіжність трьох рівних Devices | OQ-0028, OQ-0033/34, OQ-0036/37 dependency repair, compatibility, replay | D, E |
| 2.11 | FR-23: чесний receipt і Persona presence | OQ-0028 receipt fields/frontier; виміряний platform presence/retry profile | P, D, E |
| 2.12 | FR-2/39: revoke скомпрометованого Web grant і новий enrollment | OQ-0022/24 effective controller frontier, trusted recovery без старого Android, epoch/key rotation | D, E |

### Що вже відповідає картам

- FR-1/3 мають Epic 1; FR-2/22–25 і основа FR-39 — Epic 2. Story 1.3 свідомо дає ранній доказ локальної Note, не повний Notes App.
- PRD, ADR-0021, architecture spine й епіки узгоджені щодо статичного Web, direct-first Iroh та VIDA-operated encrypted relay fallback. Вони однаково називають browser direct **недоведеним** gate; stock Iroh/Wasm не можна оголошувати готовим direct transport.
- Сторіс 2.4–2.9 узгоджені з рівністю Devices і явним Conflict: непорівнювані прийняття не дають «першого за часом» переможця. Це коректна продуктова семантика, але не реалізований алгоритм.

### Редакційні уточнення й відкриті сценарії

1. У [epics.md](epics.md) UX-DR-6 та UX [EXPERIENCE.md](ux-designs/ux-vida-2026-09-22/EXPERIENCE.md) **уточнено**: відмова від optional контактних/high-availability дозволів не блокує Personal Space; непідтверджене збереження recovery material лишає захищену роботу pending за політикою VIDA. Підтвердження користувача не доводить наявність копії чи успішний restore.
2. У Story 1.2 **прибрано** дубль про неперевірені зовнішні копії; у 2.3 уточнено time limit direct-спроби; у 2.4 зафіксовано, що явний Conflict потребує одночасно непорівнюваних acceptances і відсутності безпечного text merge. Це редакційне вирівнювання вже погодженої поведінки.
3. Story 1.4 має явний сценарій, коли **всі** старі Devices втрачено: визначити, звідки новий Device бере актуальний ControllerState і що робить, коли його freshness неможливо довести. Файл із попереднім checkpoint сам по собі не доводить поточність.
4. Story 2.12 описує trusted Android для revoke, але в [UX UJ-2W](ux-designs/ux-vida-2026-09-22/EXPERIENCE.md) допускається recovery path. Додати окремий тест, коли Android теж втрачено; не удавати, що скомпрометована Web-сторінка є довіреним каналом.
5. Для 2.2/2.3 і 2.11 зафіксувати **scope receipt**: які operation/frontier/replica він покриває, чи історичний `синхронізовано` не обіцяє наявну резервну копію після втрати replica, як відображається відхилена офлайн-зміна.

### Обов'язкові негативні й граничні fixtures цього зрізу

| Stories | Граничний випадок | Перевірюваний результат / відкрите рішення |
|---|---|---|
| 1.1–1.2 | Crash до/після staging commit; диск заповнився до confirmation | Немає частково активної Persona; restart відкриває той самий pending стан або явну помилку |
| 1.2–1.4 | Після ротації користувач тримає старий bundle; всі попередні Devices втрачено | Не обіцяти restore з застарілим комплектом; правила re-export і доказу поточного ControllerState закрити в OQ-0022/24 |
| 1.4 | Два replacement Devices одночасно використовують один recovery kit | Не видавати дві несумісні «поточні» controller-гілки без явного reconciliation proof |
| 2.1 | Web enrollment request підмінено між показом origin та Android approval | Approval криптографічно зв'язаний із origin, Web Device key, nonce, Persona та scope |
| 2.2–2.3 | Web-операцію локально збережено, але peer відхилив її після ефективного revoke | Не показувати нескінченну «Синхронізація»; зберегти текст як дозволений recoverable intent і показати відмову без apply |
| 2.4–2.9 | Операцію authority прийнято до revoke, але вона доставлена після нього; еквівалентність двох resolution не доведена | Перевіряти права на доведеному acceptance frontier; невідому еквівалентність не стискати в один результат |
| 2.10–2.11 | Replica видала receipt, а потім втратила дані або grant | Історичний receipt не стає гарантією наявного backup; показати дійсний scope і стан відновлюваної копії окремо |
| 2.12 | Web origin скомпрометовано і старий Android недоступний | Довірений recovery authority має окремий перевірюваний revoke/re-enrollment шлях; заражена сторінка не підтверджує власну безпечність |

Це **fixture backlog**, а не твердження, що поведінку вже реалізовано. Там, де вказано `відкрите рішення`, тестовий oracle затверджується разом з відповідним OQ/ADR.

## Пакет рішень і доказів до production-розробки

| Порядок | Контур / рішення, яке треба ухвалити | Доказ для прийняття | Зачіпає |
|---|---|---|---|
| 0 | Ухвалити семантичний Core transition/receipt contract: `staged`, local commit, candidate, authority acceptance, peer apply, rejection, Conflict; Persona/Device/Space/Resource IDs та controller frontier. Не змішувати proof axes. | Таблиця станів + позитивні/негативні headless vectors для create, save, revoke, sync; спільний `ContractId` | 1.1–1.4, 2.1–2.12 |
| 1 | OQ-0022/24: ControllerState history/freshness, concurrent restore, двокомпонентний bundle, конкретний crypto/key-epoch/rotation/export contract. Модель secret + encrypted bundle + окремий ciphertext **уже погоджена**; повторно її не обирати. | REC fixtures: restart, wrong/stale bundle, old epoch, два restores, no data copy, lost old Device, no secret in logs; криптографічний review за OWASP key management | 1.1, 1.2, 1.4, 2.1, 2.12 |
| 2 | OQ-0036: atomic bootstrap та `operation+outbox+frontier` commit, provider tuple Android/Web, ключі, quota/crash/restart, snapshot/restore. SQLite є лише першим кандидатом; browser storage — окремий профіль. | Той самий fault-injection oracle на headless, Android, cached Web; жодного false saved/pending loss | 1.1–1.3, 2.2–2.3, 2.10–2.11 |
| 3 | OQ-0028/37: byte-exact signed envelope, signature domain/feature negotiation, canonical IDs, operation/application receipt і mixed-version failure. | Golden + hostile vectors; native Rust/Web Wasm byte parity; незалежний reader для заявленого відкритого профілю | 2.2–2.12 |
| 4 | OQ-0033/34: serverless Personal Space authority proof, порівнювані/incomparable acceptances, Note merge/conflict/equivalence, late branch, retention/rebase. CRDT materialization не є рішенням про business winner. | 2–3 peer permutation, revoke-before/after-acceptance, duplicate/replay, crash/rebuild, equivalent/unknown resolution fixtures | 2.4–2.10, 2.12 |
| 5 | OQ-0035/75: одна Rust Core семантика через Android binding та Rust/Wasm, Web key/storage/origin profile; **експериментально довести** direct WebRTC transport усередині version-pinned Iroh 1.2. | Path trace без TURN/relay payload, signed receipt, cached offline reopen, quota/key loss, origin compromise; якщо direct не працює — явний ADR/рішення, не тихе relay-first | 2.1–2.3, 2.10–2.12 |
| 6 | OQ-0073 для простого Note: вибрати fragment/merge adapter тільки після однакових fixtures. Розширене редагування Epic 5 тут не обирати. | незалежні абзаци зливаються; той самий фрагмент не губиться; cursor/presence не видається за durable edit | 2.4–2.9 |

У кожного контуру має бути: рішення в ADR/spec з версією, один власник нормативного контракту, мінімальний профіль сумісності, named positive/negative fixtures та зафіксований результат виконання. Дослідницька таблиця кандидатів — evidence, а не рішення. З [інвентаря бібліотек](research/technical-iroh-project-library-inventory-for-vida-2026-09-23/research.md) брати актуальні кандидати за реальною функціональністю, частотою змістовних commit/PR, handling issues, ліцензією й однаковим fixture suite; версія `<1.0` сама по собі не дискваліфікує. Спершу контракт/тест, потім фізична бібліотека.

## Рекомендована послідовність робіт і BMad handoff

1. **Зараз:** узгодити контур 0 і пакет рішень 1–6 як межу *цього* зрізу; редакційні неточності вище виправити без зміни погодженої політики. Для невизначених технічних варіантів оформити короткий ADR/spec decision, не підмінювати рішення припущенням у story.
2. **Доказові прототипи:** headless Persona/Note durability → restore/current-controller → canonical signed operation → Android/Web binding parity → authenticated direct Iroh path → relay fallback → 2/3 peer merge/Conflict. Порядок зберігає діагностованість; прототип не є публічним alpha/beta і не закриває сторі без acceptance evidence.
3. **Story-level gate:** для кожної з 16 сторіс пов'язати її AC із версією Core контракту, позитивним/негативним fixture, Android/Web UX-станом та записом фактичного результату. Де рішення змінило поведінку — оновити story + PRD/UX/architecture impact до коду.
4. **Повторити BMad readiness** лише для епіків 1–2. Якщо немає незадокументованих рішень, а блокувальні прототипи/контракти мають результат, перейти до **BMad Sprint Planning** і створити `sprint-status` для першого інкременту; далі **BMad Build** по сторіс у залежному порядку. Зараз створення green sprint-status означало б помилкову готовність.

## Відкладене, але не скасоване

Функціонал епіків 3–10, повна FR-39 Web parity, повний редактор Notes, iOS/Windows equal-device докази та загальні operation families перевіряються під час їхнього story elaboration і Release-1 gate. Вони **не** повинні бути оголошені виконаними за Note-зрізом. Якщо прототип провалить direct Web gate або криптографічне відновлення, змінюється архітектурний/продуктовий план через явне рішення; scope не скорочується мовчки.
