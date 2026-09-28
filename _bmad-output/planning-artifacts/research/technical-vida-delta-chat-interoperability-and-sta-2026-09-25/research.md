---
title: 'Technical research: Iroh-only VIDA messaging and Delta Chat boundary'
type: technical
topic: 'VIDA-owned Iroh Messenger, Delta Chat patterns, and static Web feasibility'
decision: 'What is the independent VIDA messaging path after rejecting stock Delta Chat interoperability and Chatmail transport?'
source: 'User-approved research plan, 2026-09-25'
status: complete
preset: standard
validation: normal
verified_claims: 16
unverified_claims: 0
verification_basis: '16 claims checked against current primary sources; no stock-client conformance or application build run'
interop_test_status: not-run
decision_outcome: 'VIDA-owned Iroh application protocols; Delta Chat and other projects are pattern references only, not client or relay dependencies'
created: '2026-09-25'
updated: '2026-09-25'
---

# Technical research: VIDA-owned Iroh messaging and Delta Chat boundary

**Decision this research serves:** Define an independent VIDA implementation for account creation, 1:1/group messaging, and note sync across Android and static Web, retaining the accepted VIDA/Iroh architecture and using other projects only as references.

## Короткий висновок для рішення

**Підсумкове рішення користувача (2026-09-25):** VIDA робить **власну незалежну реалізацію** повідомлень, груп і синхронізації на Iroh; відмовляється від stock Delta Chat interoperability, Chatmail-акаунтів, SMTP/IMAP, Delta Core та поштового шлюзу. Delta Chat і інші проєкти — лише джерела патернів та ризиків, без runtime- чи wire-залежності. Це збігається з ADR-0005 і попереднім Iroh research. Попередню рекомендацію про Chatmail-адаптер відкликано. «Власна реалізація» означає VIDA-owned identity, schemas, envelopes, ALPN, authorization та sync semantics, **не** написання власних криптопримітивів і **не** відмову від бібліотеки Iroh 1.2. [3][4][36][L3][L4]

**Статичний Web-клієнт із VIDA↔VIDA чатом і нотатками** залишається ціллю, але Iroh у звичайному браузері сьогодні потребує *Iroh transport relay* для кожного з'єднання; прямий браузерний P2P шлях не підтримується. Це не Chatmail relay і не прив'язка до Delta Chat, проте фраза користувача «якщо потрібен релей — не робимо» може охоплювати й цей relay. Якщо заборона стосується **будь-якого** relay, ціль статичного Web ↔ Android sync не підтверджена й потребує окремого технічного рішення або зміни платформи. До того ж ADR-0014 виключає браузерний Release 1 клієнт; вимогу Web треба явно узгодити з ADR/PRD. [6][L3][L5]

**Причина відмови від stock Delta:** його Iroh peer channel починається з регулярного Chatmail-повідомлення з topic/NodeAddr і призначений для webxdc realtime, не штатного чату. Щоб говорити зі stock Delta, довелося б підтримати окремий поштовий шлях або змінити сам Delta Chat; користувач обрав не робити цього. Аналіз поштового шлюзу нижче збережений лише як відхилена альтернатива, не беклог VIDA. [3][36][L3]

**Уточнення щодо «анонімного Iroh-акаунта».** За опублікованою Delta Chat privacy policy (вересень 2026), анонімний профіль не просить телефон або вашу адресу, але автоматично отримує випадкову адресу на chatmail relay. Окрема ефемерна Iroh-ідентичність створюється для P2P webxdc-сесії; це не документований постійний чат-акаунт, який замінює адресу relay. Офіційний опис прямо каже, що початковий Iroh ticket передається encrypted system message через federated email. Якщо у користувача є новіша збірка з окремим Iroh-only account mode, потрібні точна версія, скріншот або посилання, бо перевірені публічні джерела такого режиму не показали. [1][36][37]

Нижче залишено протокольний аналіз Chatmail як **доказ, чому відхилений шлях відрізняється від Iroh-only**; попередні mail-crate кандидати, плани адаптера й conformance-тести **не є** актуальним планом реалізації VIDA.

## 1. Акаунт і анонімність

Delta Chat дозволяє створити профіль без телефону чи вже наявної пошти; Chatmail видає випадкову адресу при першому вході. Core надає JSON-RPC операції для акаунтів і транспортів. Профіль переноситься QR на другий пристрій або резервною копією. Це добрий механізм **анонімної реєстрації для чату**, але оператор relay бачить ваш IP, випадкову адресу й технічні метадані доставки; відсутність signup-PII не дорівнює повній мережевій анонімності. З цього не випливає видимість plaintext назви чи повного складу групи. [1][2][9]

Не варто автоматично робити Chatmail-адресу чи ключ Delta Chat кореневою VIDA Persona. Поточний VIDA контракт вимагає незалежності Persona від email, сервера, пристрою й endpoint; Delta профіль краще моделювати `ServiceBinding`/messaging identity, із явним локальним зв'язком до Persona. Чи взагалі використовувати один криптографічний ключ для обох систем — окремий security/design gate, не припущення. [L2][1]

Публічний `nine.testrun.org` заявляє ліміти 60 повідомлень/хв і 700 MB та коротке зберігання. Це підтверджує придатність для тесту, але не контракт на масовий VIDA production. Оператор може обмежувати реєстрацію. [10][2]

## 2. Нативні повідомлення, групи й нотатки

Core — Rust-реалізація, яку використовують Delta Chat Android/iOS/Desktop; для нової інтеграції автори рекомендують JSON-RPC. Підтримуються 1:1 і групові повідомлення, QR/SecureJoin, події та явні групові add/remove. Це дозволяє сформулювати реальний interoperability test із stock Delta Chat, а не лише VIDA↔VIDA. [3][4][11]

Група Delta Chat — email-адресати, group ID і зміни складу через повідомлення, не тотожна VIDA Space з його ACL, keys, authority та AppInstances. Відображення `Delta group ↔ VIDA conversation` можливе лише з окремими правилами membership/permission; автоматична тотожність групи й Space була б хибною. [4][L2]

Структуровані Notes можна передати як файли/повідомлення або webxdc app-state updates, але stock Delta Chat не почне сам інтерпретувати VIDA schema. webxdc не надає повного CRDT чи гарантованого глобального порядку; merge/conflict, receipt і відновлення лишаються обов'язком VIDA. Якщо `.xdc` є окремим редактором у Delta Chat, це **сумісна доставка/хостинг застосунку**, не нативна функція Notes стандартного клієнта. [12][13]

Core має MPL-2.0 (не блокувальник для окремих MIT-файлів VIDA, але потребує виконання MPL щодо Core-файлів/змін і дистрибуції). Поточний Cargo manifest Core безумовно залежить від `iroh`/`iroh-gossip` 0.35, тоді як VIDA ADR використовує Iroh 1.2. Cargo може розв'язати різні версії як різні crates, але це не означає спільних типів чи transport compatibility; збірку, розмір, native-link і межу JSON-RPC потрібно виміряти. [14][15][L3]

## 3. Відхилений варіант: власний Chatmail-адаптер

Цей розділ описує технічну ціну **необраного** stock Delta interoperability. Слова «рекомендовано» й «proof» нижче стосуються лише того, що довелося б зробити **за умови скасування Iroh-only**; вони не є поточним roadmap VIDA.

**Що саме означає сумісність.** Stock Delta Chat має змогти розпізнати Chatmail-акаунт VIDA-користувача як адресата, пройти контактне знайомство, отримати/надіслати зашифрований 1:1 текст, взяти участь у сумісній групі та побачити зміну її складу. VIDA-специфічні Persona, Space ACL, Notes, Projects, синхронізація Iroh і форуми не стають функціями stock Delta Chat автоматично. Група Delta — email-протокол і список адресатів, не authority VIDA Space. [3][4][21][22][L2][L3]

**Протокольний мінімум — не лише заголовок.** Потрібні реєстрація/вхід Chatmail (URI `DCACCOUNT` описує налаштування нового випадкового акаунта, `DCLOGIN` — наявного; власне створення відбувається через механізм relay/перший IMAP login), IMAPS/SMTP submission, черга вихідних і дедуплікація `Message-ID`, RFC 5322/MIME, `Chat-Version: 1.0`, Autocrypt 1.1/OpenPGP із PGP/MIME, актуальний SecureJoin, групові `Chat-Group-ID`/`Chat-Group-Name` і явні add/remove. `Chat-Version` є MUST у поточній специфікації; Autocrypt-шифрування й захищені заголовки RFC 9788 описані як SHOULD, але ми пропонуємо перевіряти їх як ціль сумісності/безпеки VIDA. Chatmail relay може відхиляти незашифрований вихідний лист; отже простий MIME-«чат» без криптографії не проходить навіть як надійний перший етап на публічному relay. Відправка в SMTP не є ще application-level доставкою адресату. Специфікація Chatmail позначена `0.37.0 / in-progress`, тому фактичну сумісність визначають black-box тести зі stock-клієнтом, а не лише буквальне читання документа. [4][8][21][22][23][24]

**Групи потребують власної машини станів.** ID містить 11–32 дозволених символів; усі учасники зазначені в `From`/`To`, а `Message-ID` групового листа має включати `Gr.<group-id>.<unique>`. Початковий склад можна вивести з першого листа, але наступне вилучення учасника не можна вгадувати з відсутності в `To`: потрібна окрема дія видалення. Для duplicate/reorder/offline необхідні стійкий журнал подій, causal/order правила і повторювані golden fixtures. Зміна членства Delta-групи не повинна непомітно змінювати ACL чи криптоключі VIDA Space. [4][L2]

**Ключі та контактне знайомство.** Autocrypt Level 1 задає обмін ключами у заголовку й профіль Ed25519/Curve25519; поточний Core показує конкретні legacy-key і symmetric-algorithm параметри, які слід відтворити тестовими пакетами, а не здогадуватися за оголошеними preferences. Публічна текстова документація SecureJoin попереджає, що застаріла для key-contact v2 після 2025 року; поточний Core уже містить v3 `vc-request-pubkey`/`vc-pubkey` та legacy-шляхи. Отже QR, fingerprint, автентифікацію, replay і key-continuity тестуємо обома напрямами зі stock Delta, не копіюємо старі кроки. Ключ Delta binding не є автоматично кореневим ключем VIDA Persona. [9][22][25][35][L2]

**Рекомендована межа модулів (проєктна пропозиція).** `vida-core` володіє Persona/Space/ресурсами; `vida-transport-iroh` реалізує VIDA-синхронізацію на Iroh 1.2; `vida-interop-delta` володіє поштовою сесією, криптографічним binding, MIME, SecureJoin, group state та outbox/inbox; він відображає перевірені зовнішні факти в Core-команди, але не обходить VIDA ACL. `vida-sdk` подає той самий контекст Flutter-клієнтам. Адаптер не має ставати другим master-сховищем Notes/Projects. Власна реалізація не означає власну реалізацію TLS/OpenPGP-примітивів: використовуємо перевірені crates, а VIDA пише protocol glue і політику. Це архітектурний висновок із джерел, не твердження, що модулі вже існують. [4][22][29][30][31][32][33][L2][L3]

| Шар | Актуальні кандидати на 2026-09-25 | Межа рішення |
| --- | --- | --- |
| VIDA P2P | `iroh` 1.2.0 | Уже прийнятий primary transport; не підміняє SMTP/IMAP Delta. [L3][33] |
| IMAP / SMTP | `async-imap` 0.11.3 / `lettre` 0.11.23 | Обидва лише mail primitives; потрібні прототип авторизації Chatmail і retry-семантики. [29][30] |
| MIME | `mail-parser` 0.11.9 / `mail-builder` 1.0.0 | Парсер і builder не реалізують Chatmail group/SecureJoin semantics. [31][32] |
| OpenPGP | `pgp` (rPGP) 0.20.0; `sequoia-openpgp` 2.4.1 — альтернатива | rPGP ближче до поточного Core і MIT/Apache; Sequoia має LGPL/backend/Wasm оцінку. Фінальний вибір — після packet conformance, license і platform build. [27][28] |

Вік версії сам собою не є критерієм відсіву. За вибіркою **останніх 20 default-branch commits** у GitHub Atom на 2026-09-25: Iroh має щонайменше 20 за попередні 30 днів (feed обрізаний), rPGP — 5, `async-imap` — 1, `lettre` — 1, `mail-parser` — 4, `mail-builder` — 3. Це не повна статистика активності і не оцінка якості; issues/PRs, функціонал, conformance і ліцензію перевіряємо окремо. Межі вибірки та commit IDs збережено в [activity snapshot](digests/own-interop-library-activity-r1.md). [34]

**Вебмежа не зникає від власного коду.** Для звичайного GitHub Pages-клієнта браузерний Iroh/Wasm і локальне сховище дають напрям прототипу VIDA Notes sync, але працездатність саме нашого Flutter/Rust/Web клієнта ще не доведена. Сторінка не може просто відкрити TCP до Chatmail SMTP/IMAP. Варіанти: (а) Web у першому зрізі доводить VIDA Notes/sync, а native Android — Delta Messenger; (б) статичний frontend звертається до окремого HTTPS/WebSocket mail bridge із чітко визначеною довірою/оператором — він не стає не-статичним, але має backend-залежність; (в) окремий браузерний розподіл, не звичайний static Pages. Жоден варіант не слід називати підтвердженим web-parity до прототипу. [5][6][8][26]

**Порядок proof, від меншого до більшого:** (0) зафіксувати підтримувані версії stock Delta, мінімальний wire-profile і golden fixtures із тестових акаунтів; (1) headless Rust Chatmail signup/login + надійне локальне збереження credentials/outbox; (2) Autocrypt key advertisement, ключовий bootstrap із stock Delta та OpenPGP/PGP-MIME 1:1 текст в обидва боки, з негативними tamper/replay тестами; (3) перевірений SecureJoin/QR поточного stock-клієнта та група/create/add/remove/offline/reorder; (4) Android Flutter інтегрує VIDA Persona + Delta binding, паралельно 2–3 Android клієнти синхронізують ту саму VIDA Note через Iroh 1.2; (5) статичний Flutter Web доводить VIDA Note sync/storage/recovery; окремий gate вирішує, чи потрібен веб-Delta bridge. `Chat-Version`, SMTP accepted, mailbox received, decrypted/applied і read — різні receipts. Не копіювати MPL-реалізацію Core у MIT-ядро без окремого license review; використовувати відкриту специфікацію й black-box конформанс. [4][14][21][22][26][L2][L3]

| Conformance gate | Мінімальний доказ, який зберігаємо як fixture/test |
| --- | --- |
| Акаунт | VIDA створює Chatmail-акаунт без телефону; після restart входить у нього; неправильні дані входу не перетворюються на новий акаунт. |
| 1:1 | VIDA→stock Delta і stock Delta→VIDA: зашифрований текст, той самий зміст після decrypt, стійкий `Message-ID`, немає дублів після retry. |
| Контакт | Актуальний SecureJoin/QR в обох напрямках; fingerprint і key-continuity перевіряються; tamper/replay відхиляються. |
| Група | Створення, вступ, add/remove, текст у двох напрямках; запізнілий лист і повтор не повертають видаленого учасника. |
| VIDA Core | Одна `NoteId` на фізичному Android, емуляторі та статичному Web; offline/restart/duplicate/reorder не породжують втрату або другу нотатку. Це **окремий** proof Iroh, не proof Delta. |

## 4. Статичний Web і relays

GitHub Pages може віддати скомпільовані статичні активи. Flutter `--wasm` не компілює Rust і не відкриває native `dart:ffi` у браузері: потрібен окремий Rust/Wasm artifact і JS/web binding. Iroh має браузерні Rust/Wasm приклади, але у browser sandbox немає UDP hole punching, тому кожне з'єднання йде через Iroh relay. [5][6][16]

**Chatmail relay ≠ Iroh relay.** Chatmail забезпечує SMTP/IMAP-обмін та тимчасову поштову доставку. Інсталяція Chatmail може також піднімати окремий Iroh relay, але це не робить поштовий endpoint сумісним з VIDA/Iroh і не означає дозволу довільного VIDA traffic на публічному endpoint. Iroh default public relays документовані для development/testing; production може взяти managed shared/dedicated relay без власного сервера, але вже з окремою залежністю/умовами. [7][8][17]

IndexedDB/OPFS можуть зберігати локальні дані, однак browser storage може бути очищений/витіснений; `persist()` знижує ризик, не скасовуючи його. Для vault із ключами потрібні encrypted export/recovery, threat model, CSP/XSS захист, окремий origin. GitHub project Pages одного `owner.github.io` поділяють origin між шляхами; для production-клієнта з ключами доцільний окремий домен. OWASP застерігає не тримати чутливі secrets у `localStorage` й не покладатися на browser storage як конфіденційне сховище саме по собі. [18][19][20]

## Перехресні висновки та варіанти

| Варіант | Що дає | Ключовий недолік | Вердикт дослідження |
| --- | --- | --- | --- |
| A. VIDA-owned Messenger на Iroh 1.2 | Єдиний VIDA-протокол для Android/Web, незалежна Persona, 1:1/групи/нотатки | Не обмінюється чатами зі stock Delta Chat | **Актуальний Iroh-only напрям**; Delta — референс патернів [L3][L4] |
| B. Власний Chatmail adapter або Delta Core | Stock Delta Chat interoperability можлива після conformance | Потребує email transport, який користувач і ADR-0005 відхилили | **Відхилено для VIDA**, наведено лише як доказ межі [3][L3] |
| C. Спільний Iroh chat protocol, який додасть upstream Delta Chat | Теоретично зможе дати stock-сумісність без пошти | Потребує змін Delta Chat, згоди/випуску upstream і спільних wire/identity правил; підтримки зараз не підтверджено | **Не планується**: VIDA не залежить від roadmap іншого проєкту |
| D. Окремий Iroh↔Chatmail gateway | VIDA Android/Web мають лише Iroh-з'єднання; stock Delta лишається незміненим | Gateway використовує пошту, бачить метадані й потребує identity/crypto/receipt конформансу | **Відхилено** разом зі stock-сумісністю та поштовим шлюзом [3][36] |

Для першого лабораторного епіка можна перевірити Iroh через дозволений публічний тестовий relay без власного поштового сервісу. Браузер усе одно проходить через Iroh relay; це не Chatmail relay і не гарантія production SLA. [6][7][8]

## Рекомендації й acceptance gates

1. Epic 1: одна автономна VIDA Persona, два Android Devices і статичний Web-клієнт обмінюються **VIDA↔VIDA** зашифрованими 1:1 повідомленнями й тією самою Note через versioned VIDA ALPN на Iroh 1.2. Уточнення Web змінює ADR-0014/PRD; оновити їх явно до реалізації. Впевненість: висока щодо архітектурної межі, середня щодо браузерної збірки до proof. [6][L2][L3][L5]
2. У нашому wire protocol розробити власні account/Persona bootstrap, контактний QR, group membership, durable outbox, receipts, encrypted local storage й sync/merge. Ідеї Delta Chat — референси, але не нативна сумісність зі stock-клієнтом. Впевненість: висока щодо розмежування, деталі контрактів вимагають окремої архітектури. [3][36][L3][L4]
3. Для static Web перевірити Flutter/Rust/Wasm/Iroh relay path, browser storage recovery і OWASP browser-security gates; не обіцяти offline persistence або доступність без тестів. [6][18][19][20]
4. Не планувати Chatmail account, SMTP/IMAP, MIME, Autocrypt, SecureJoin wire, browser mail gateway і Delta Core dependency в Epic 1. Якщо колись знадобиться stock Delta interop, це буде **нове окреме рішення**, що змінює Iroh-only constraint або потребує upstream Iroh chat support. [3][4][36][L3]

## Відкриті питання для власника продукту

1. Чи «без relay» означає без **Chatmail/Delta-сервісу**, але з допустимим транспортним Iroh relay для Web і NAT, чи заборону **будь-якого** relay? У браузері Iroh наразі працює через relay; за повної заборони статичний Web↔Android sync не можна обіцяти без окремого транспортного рішення. [6]
2. Static Web для Epic 1 уже явно запитаний користувачем, але ADR-0014 і PRD його виключають. Після уточнення транспортного профілю синхронізувати ці релізні документи з чинним рішенням.
3. Якщо транспортний Iroh relay дозволений, який тимчасовий оператор допустимий для лабораторного proof, а який production-профіль забезпечить браузерну доступність? Це не Chatmail, не поштовий шлюз і не власник даних.

## Джерела

| № | Що підтримує | Видавець/джерело | Опубліковано | Перевірено | Впевненість |
| --- | --- | --- | --- | --- | --- |
| [1] | анонімний профіль, ключі, метадані | [Delta Chat privacy](https://delta.chat/en/privacy) | 2026-09 | 2026-09-25 | high |
| [2] | Chatmail signup, транспорти | [Chatmail relay setup](https://chatmail.at/doc/relay/getting_started.html) | undated/current | 2026-09-25 | high |
| [3] | Core platforms, JSON-RPC, protocol stack | [Chatmail Core README](https://github.com/chatmail/core) | current main | 2026-09-25 | high |
| [4] | Delta groups, member actions | [Core spec](https://github.com/chatmail/core/blob/main/spec.md) | current main | 2026-09-25 | high |
| [5] | static hosting; Flutter web boundary | [GitHub Pages](https://docs.github.com/en/pages/getting-started-with-github-pages/what-is-github-pages), [Flutter architecture](https://docs.flutter.dev/resources/architectural-overview) | undated/current | 2026-09-25 | high |
| [6] | browser Iroh relay-only | [Iroh browser guide](https://docs.iroh.computer/languages/wasm-browser) | undated/current | 2026-09-25 | high |
| [7] | public relays are dev/test | [Iroh relay guidance](https://docs.iroh.computer/add-a-relay) | undated/current | 2026-09-25 | high |
| [8] | Chatmail vs Iroh relay | [Chatmail architecture](https://chatmail.at/doc/relay/overview.html) | undated/current | 2026-09-25 | high |
| [9] | profile transfer/backup, key UI caveat | [Delta Chat FAQ](https://delta.chat/en/help) | undated/current | 2026-09-25 | high |
| [10] | sample public relay quotas/retention | [nine.testrun.org info](https://nine.testrun.org/info.html) | undated/current | 2026-09-25 | medium |
| [11] | JSON-RPC account/events/SecureJoin API | [Core JSON-RPC API](https://github.com/chatmail/core/blob/main/deltachat-jsonrpc/src/api.rs) | current main | 2026-09-25 | high |
| [12] | webxdc updates | [webxdc sendUpdate](https://webxdc.org/docs/spec/sendUpdate.html) | undated/current | 2026-09-25 | high |
| [13] | shared-state conflict order | [webxdc conflicts](https://webxdc.org/docs/shared_state/conflicts.html) | undated/current | 2026-09-25 | high |
| [14] | MPL obligations | [Chatmail Core LICENSE](https://github.com/chatmail/core/blob/main/LICENSE) | current main | 2026-09-25 | high |
| [15] | Core version/deps | [Core Cargo.toml](https://github.com/chatmail/core/blob/main/Cargo.toml) | current main | 2026-09-25 | high |
| [16] | Rust/Web bridge | [flutter_rust_bridge Wasm limits](https://cjycode.com/flutter_rust_bridge/manual/miscellaneous/wasm-limitations) | undated/current | 2026-09-25 | high |
| [17] | separate Iroh relay in Chatmail deployment | [Chatmail relay config](https://github.com/chatmail/relay/blob/main/chatmaild/src/chatmaild/ini/chatmail.ini.f) | current main | 2026-09-25 | medium |
| [18] | eviction/persistence | [MDN storage quotas](https://developer.mozilla.org/en-US/docs/Web/API/Storage_API/Storage_quotas_and_eviction_criteria) | undated/current | 2026-09-25 | high |
| [19] | key/origin and XSS boundaries | [MDN same-origin](https://developer.mozilla.org/en-US/docs/Web/Security/Defenses/Same-origin_policy), [MDN WebCrypto](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto) | undated/current | 2026-09-25 | high |
| [20] | browser storage security | [OWASP HTML5 Security](https://cheatsheetseries.owasp.org/cheatsheets/HTML5_Security_Cheat_Sheet.html) | undated/current | 2026-09-25 | high |
| [21] | `DCACCOUNT`/`DCLOGIN`, QR/URI forms | [Delta Chat URI schemes](https://github.com/deltachat/interface/blob/main/uri-schemes.md) | current main | 2026-09-25 | high |
| [22] | Autocrypt Level 1 key exchange/profile | [Autocrypt 1.1 specification](https://docs.autocrypt.org/level1.html) | 2019/version 1.1 | 2026-09-25 | high |
| [23] | e-mail/MIME and protected headers baseline | [RFC 5322](https://www.rfc-editor.org/rfc/rfc5322), [RFC 2045](https://www.rfc-editor.org/rfc/rfc2045), [RFC 9788](https://www.rfc-editor.org/rfc/rfc9788) | standards | 2026-09-25 | high |
| [24] | Chatmail transport, encryption filter, signup behavior | [Chatmail relay overview](https://chatmail.at/doc/relay/overview.html) | undated/current | 2026-09-25 | high |
| [25] | current Core key/algorithm choices | [Core `pgp.rs`](https://github.com/chatmail/core/blob/main/src/pgp.rs) | current main | 2026-09-25 | high for observed source |
| [26] | raw TCP only for packaged IWA, not ordinary static page | [Chrome Direct Sockets](https://developer.chrome.com/docs/iwa/direct-sockets) | undated/current | 2026-09-25 | high |
| [27] | rPGP 0.20 API/license/features | [rPGP docs](https://docs.rs/pgp/0.20.0/pgp/), [crate](https://crates.io/crates/pgp) | 2026-06-23 release | 2026-09-25 | high for published crate |
| [28] | Sequoia 2.4.1 license/backend constraints | [Sequoia crate manifest](https://docs.rs/crate/sequoia-openpgp/latest/source/Cargo.toml), [crate](https://crates.io/crates/sequoia-openpgp) | 2026-07-09 release | 2026-09-25 | high for published crate |
| [29] | IMAP client candidate/version | [`async-imap` crate](https://crates.io/crates/async-imap) | 2026-07-17 release | 2026-09-25 | high |
| [30] | SMTP/MIME candidate/version | [`lettre` crate](https://crates.io/crates/lettre) | 2026-08-03 release | 2026-09-25 | high |
| [31] | MIME parser candidate/version | [`mail-parser` crate](https://crates.io/crates/mail-parser) | 2026-09-09 release | 2026-09-25 | high |
| [32] | MIME builder candidate/version | [`mail-builder` crate](https://crates.io/crates/mail-builder) | 2026-09-12 release | 2026-09-25 | high |
| [33] | Iroh current version | [`iroh` crate](https://crates.io/crates/iroh), [Iroh docs 1.2](https://docs.rs/iroh/1.2.0/iroh/) | 2026-09-09 release | 2026-09-25 | high |
| [34] | bounded default-branch commit activity sample | [Iroh commits](https://github.com/n0-computer/iroh/commits.atom), [rPGP](https://github.com/rpgp/rpgp/commits.atom), [async-imap](https://github.com/chatmail/async-imap/commits.atom), [lettre](https://github.com/lettre/lettre/commits.atom), [mail-parser](https://github.com/stalwartlabs/mail-parser/commits.atom), [mail-builder](https://github.com/stalwartlabs/mail-builder/commits.atom) | rolling feed as of 2026-09-25 | 2026-09-25 | medium; Iroh feed capped at 20 |
| [35] | current SecureJoin v3 and old-doc warning | [Core `securejoin.rs`](https://github.com/chatmail/core/blob/main/src/securejoin.rs), [SecureJoin guide](https://securejoin.delta.chat/) | main/current + older guide | 2026-09-25 | high for observed source, medium for undocumented protocol interpretation |
| [36] | Iroh in stock Core is for device setup/webxdc realtime, chat uses email stack | [Chatmail Core README](https://github.com/chatmail/core), [Delta Chat FAQ](https://delta.chat/en/help), [Core peer channels](https://github.com/chatmail/core/blob/main/src/peer_channels.rs) | current main/FAQ | 2026-09-25 | high for published architecture |
| [37] | Iroh ticket bootstrap, ephemeral identity and real-time webxdc scope | [Delta Chat P2P integration explanation](https://delta.chat/en/2024-11-20-webxdc-realtime) | 2024-11-20 | 2026-09-25 | high for described architecture; check current implementation in [36] |

Локальні нормативні джерела: [L1] [PRD Release 1](../../prds/prd-vida-2026-09-22/prd.md); [L2] [ADR-0004](../../../../docs/03-architecture/decisions/ADR-0004-multi-axis-identity-model.md); [L3] [ADR-0005](../../../../docs/03-architecture/decisions/ADR-0005-iroh-transport-foundation.md); [L4] [попередній Iroh ecosystem research](../technical-iroh-ecosystem-and-project-patterns-for-2026-09-18/research.md); [L5] [ADR-0014](../../../../docs/03-architecture/decisions/ADR-0014-native-only-vida-clients.md).

## Staleness map

`recon_kit.py staleness` за [claims.json](claims.json) і вікнами technical pack (compatibility/service policy: 1 місяць; privacy: 6; architecture pattern: 24) дав найранішу перевірку **2026-10-25** для Core API/deps, браузерного Iroh, relay policy і Flutter/Rust bridge. Delta privacy — **2027-03-01**, identity/adapter pattern — **2028-09-25**. Для undated/current сторінок дата 2026-09-25 у розрахунку означає **день перевіреного зрізу**, а не доведену дату публікації. Перед production-вибором public relay умови та конкретні pinned crate версії перевіряються знову незалежно від цих формальних дат.
