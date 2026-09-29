---
title: 'Technical research: Tor-isolated transport for VIDA Release 1'
type: technical
topic: 'Anonymous Persona network transport, Iroh, Delta Chat and Chromium Web'
date: 2026-09-28
status: researched-not-library-selected
---

# Tor-захищений транспорт VIDA: рішення для погодження

## Висновок

У Release 1 потрібен окремий **Tor-only мережевий профіль** для анонімної Persona. Він не замінює звичайний VIDA-профіль direct-first із дозволеним Iroh relay fallback. Профіль має приховувати IP учасників від одне одного і від звичайного VIDA relay, але не може гарантувати «повної анонімності»: саморозкриття, кореляція часу/обсягу, вміст вкладень та ідентифікатори інших сервісів лишаються ризиками. Tor Project прямо застерігає, що досконала анонімність неможлива і Tor захищає лише коректно сконфігурований трафік застосунку. [Tor safety](https://support.torproject.org/tor-browser/security/using-tb-safely/), [Tor stream isolation](https://spec.torproject.org/path-spec/stream-isolation.html).

## Що насправді показує Delta Chat

- Delta Chat V2 передає звичайні повідомлення через **chatmail relays**, які тимчасово тримають шифротекст; це не Iroh relay і не модель VIDA. [Delta Chat FAQ](https://delta.chat/en/help).
- У Delta Chat є історичний сценарій SOCKS5/Tor для частини мережевого трафіку. Публічне повідомлення розробника застерігало, що Iroh realtime не успадковував proxy settings навіть при relay; issue desktop фіксував окремі непрохідні через proxy WebRTC calls та Iroh realtime. Це докази **ризику**, а не твердження, що всі поточні версії Delta Chat досі мають ті самі дефекти. [Обговорення Iroh/proxy](https://support.delta.chat/t/alternative-p2p-realtime-chat-method-without-revealing-your-lan-wan-ip/3554), [Delta Chat desktop issue](https://github.com/deltachat/deltachat-desktop/issues/3093).
- У червні 2026 розробник Delta Chat писав, що calls використовують WebRTC, а перехід Iroh з 0.35 на 1.0 ще планувався; Tor/Nym для P2P згадували як надію після переходу. Це **не доказ** готового анонімного Iroh+Tor у Delta Chat. [Delta Chat forum](https://support.delta.chat/t/wider-usage-of-iroh/5353).

## Кандидати та межі доказу

1. Iroh показав Tor onion-service custom transport. Концептуально це найкращий референс для VIDA Iroh protocol over Tor, але автор називає custom transport експериментальним, а приклад запускає окремий C Tor daemon із вимкненою cookie authentication. Таку конфігурацію не можна переносити в production. Автор прямо вимагає вимкнути звичайні IP/relay transports і discovery для приватного профілю. [Iroh Tor transport write-up](https://www.iroh.computer/blog/tor-custom-transport), [репозиторій](https://github.com/n0-computer/iroh-tor-transport).
2. `arti-client` — офіційний вбудовуваний Rust-кандидат для Tor streams. [Головна сторінка Arti](https://arti.torproject.org/) називає proxy та onion-service support готовими до використання, але [сторінка обмежень](https://arti.torproject.org/guides/capability-limitations/) одночасно каже «не рекомендований для production» і попереджає, що її деталі частково застарілі. Це суперечливий стан документації, а не доказ непридатності. Версія `<1.0` сама по собі не дискваліфікує Arti; потрібні перевірка актуального release/feature profile, мобільних збірок, onion hosting, security review і conformance.
3. Альтернатива для першого prototype — ізольований Tor daemon/sidecar із автентифікованим control channel, але Android/iOS background і distribution профіль потребують окремих перевірок. Публічний Iroh example із `CookieAuthentication 0` є лише демонстрацією, не безпечним шаблоном.

## Рекомендований контракт VIDA

- Режим обирається на рівні **Persona/network context**, щоб чат, файли, запрошення, контакти/lookup, sync, receipts, presence, дзвінки, push, діагностика та відкриття зовнішніх URL не залишили непоміченого каналу повз Tor. Для операції в анонімному контексті усі мережеві підзапити йдуть через перевірений Tor path або fail-closed. Окремі Apps не можуть самі вимкнути це правило.
- Якщо Tor недоступний, локальний durable save дозволений, мережеві операції лишаються pending; **ніякого** fallback на звичайний direct, N0/public relay чи VIDA relay. Власний relay може бути доступним як `.onion` endpoint, але це все ще Tor path; звичайний relay fallback лишається тільки для звичайного профілю.
- Вхідний peer може бути onion service, а не відкритий IP endpoint. Ідентифікатори/ключі для Tor-контексту треба відокремити від публічної/робочої Persona; не копіювати референсне використання одного Iroh private key для onion-адреси без privacy review. Tor circuits між Personas ізолювати application-provided tokens. [Tor onion services](https://community.torproject.org/onion-services/overview/), [Tor stream isolation](https://spec.torproject.org/path-spec/stream-isolation.html).
- E2EE ресурсу лишається незалежним від Tor: Tor приховує маршрут, не замінює шифрування вмісту, права або acceptance receipts. UI називає режим «Tor-захищений», а не гарантує абсолютну анонімність.
- Для **звичайних** нотаток, задач, файлів і чатів зберігається direct-first + налаштований relay fallback. Але будь-який ресурс/чат, що належить анонімній Persona, включно з його вкладеннями та синхронізацією, не може непомітно вийти звичайним шляхом.
- Chromium-based static Web є окремим ризиком: [Chromium documentation](https://developer.chrome.com/docs/iwa/direct-sockets) прямо зазначає, що стандартна сторінка не має raw TCP/UDP; Direct Sockets доступні Isolated Web Apps, а не звичайній GitHub Pages/HTTPS-сторінці. Варіант із віддаленим WebSocket gateway технічно переносить транспорт, але цей gateway бачить IP клієнта до входу в Tor; він не виконує обіцянки «лише Tor». Локальний native helper або IWA змінюють модель самостійного статичного Web. Додатково Tor Project не рекомендує Tor в іншому браузері через WebRTC/DNS/fingerprint/cookies. Отже звичайний Web Release 1 підтримується, але анонімний Web-профіль потребує окремого продуктового рішення й proof; його не можна позначити готовим лише через Rust/Wasm. [Tor and other browsers](https://support.torproject.org/tor-browser/security/using-tor-with-other-browsers/).

## Release-1 proof gate, не виконаний дослідженням

Threat model визначає, від кого приховуємо IP та metadata. Integration prototype має довести Tor-only transport на кожній заявленій платформі, onion discovery/connection, E2EE Iroh payload і receipt, network-capture tests без DNS/STUN/WebRTC/direct/relay витоків, fail-closed після втрати Tor, cross-Persona isolation, recovery та revoked-device поведінку, медіа/вкладення, зовнішні URL/preview, push і diagnostics. Окремо перевіряються battery/background policy і latency. Ні бібліотеку, ні Web анонімний режим цим документом не затверджено.

## Доповнення 2026-09-28: перехід на Tor у вже наявному Space

Це **архітектурна пропозиція для перевірки**, а не доказ реалізації. Погоджене продуктове правило: суворий Tor можна ввімкнути в тому самому Space; інший Device після отримання налаштування прозоро застосовує його і без Tor не продовжує синхронізацію даних. Перемикач Persona можна вмикати й вимикати, але він не послаблює суворішу політику Space/чату. Для двох офлайн-перемикань Persona задуманий пріоритет **фактичного часу дії**, не часу sync.

### Поведінка референсів і переносимість

| Референс | Перевірена поведінка | Межа для VIDA |
|---|---|---|
| [Briar](https://briarproject.org/how-it-works/) | Через інтернет синхронізує пристрої через Tor; без інтернету використовує Bluetooth/Wi-Fi/носії. | Доводить, що транспорт можна відділити від моделі даних; **не** доводить безпечний атомарний перехід існуючого VIDA Space. |
| [SimpleX settings](https://simplex.chat/docs/guide/app-settings.html) | Має `when available` та `required` для onion-адрес. У `required` за відсутності onion-шляху з'єднання завершується помилкою. | Для суворого Space потрібна семантика **required/fail-closed**, не «віддавати перевагу Tor». |
| [Iroh Tor custom transport](https://www.iroh.computer/blog/tor-custom-transport) і [Iroh 1.2 Builder](https://docs.rs/iroh/latest/iroh/endpoint/struct.Builder.html) | Iroh показав Tor custom transport, але автор називає його експериментальним; Builder має `clear_ip_transports`, `clear_relay_transports`, `clear_address_lookup`, а custom transport/path selector позначені unstable. | Для Tor-контексту будувати **окремий Tor-only endpoint** без звичайних IP/relay/discovery, а не покладатися на пріоритет шляху в змішаному endpoint. Приклад із неавтентифікованим Tor control port не переносити в production. |
| [Tor stream isolation](https://spec.torproject.org/path-spec/stream-isolation.html) і [Arti client](https://docs.rs/arti-client/latest/arti_client/struct.TorClient.html) | Різні акаунти/сесії не слід неявно вести одним circuit; Arti підтримує ізольовані клієнтські контексти. | Не змішувати мережеві контексти Personas; Rust runtime/мобільний lifecycle ще потребують proof. |
| [Tor SOCKS specification](https://spec.torproject.org/socks-extensions) | SOCKS є TCP proxy; `UDP ASSOCIATE` не підтримується, локальний DNS може розкрити адресу. | Просто поставити SOCKS-проксі перед QUIC/WebRTC недостатньо; Tor-адаптер мусить мати окремий перевірений транспорт та leak tests. |
| [OWASP MASVS-PRIVACY-1](https://mas.owasp.org/MASVS/controls/MASVS-PRIVACY-1/) | Принцип мінімізації доступу/передачі чутливих даних і контролю сторонніх компонентів. | Policy-only bootstrap перед Tor не повинен переносити payload, назви ресурсів, ключі або telemetry сторонніх SDK; сам факт контакту все одно може бути видимим. |

### Оптимальний маршрут переходу (пропозиція)

1. Уповноважена дія створює підписаний `RoutePolicyChange(space_id, previous_policy_hash, control_epoch, required=tor, operation_id)`. Після **атомарного локального commit** Core зупиняє outbox цього Space на звичайному маршруті, закриває його старі сесії та готує окремий Tor-only endpoint. Політика маршруту не є новою Space-копією й не змінює історію даних.
2. Оновлений peer не приймає від застарілого Device жодної data-operation на старому шляху. Якщо той уже звернувся звичайним каналом, дозволений лише мінімальний автентифікований **policy-only** обмін: ідентифікатор/epoch, digest підписаної політики, потрібні для переходу Tor-координати та ack — без вмісту Space. Сам факт такого контакту може розкрити мережеві метадані; його не називати Tor-захищеним.
3. Застарілий Device перевіряє підпис, повноваження, `previous_policy_hash` і monotonic `control_epoch`; надійно зберігає політику **до** будь-якого data apply/receipt. Він припиняє старий канал, перебудовує route context як Tor-only і лише потім запитує відсутні операції. Якщо Tor недоступний, лишає локальні зміни pending. Старі локальні наміри не губляться; їхню чинність перевіряють за ACL/causal rules після Tor-reconnect.
4. Наявні peer-и ведуть application receipts окремо від policy ack. Ack, що Device прийняв policy, **не означає**, що нотатку синхронізовано. Під час повторів `operation_id`/epoch роблять перехід ідемпотентним. Вихід із Tor за налаштуванням Persona переоцінює черги та створює новий route context; якщо Space вимагає Tor, звичайний маршрут усе одно заборонений.
5. Два старі ізольовані Devices можуть не знати про перехід і обмінятися між собою старим шляхом; мережевий протокол не може це заборонити заднім числом. Строгий доказ починається **для оновлених peer-ів з моменту local policy commit**. Якщо продукт вимагатиме блокувати також повністю ізольовані Devices, знадобиться змінити offline-модель (наприклад, короткі online leases); це не входить до вже погодженої поведінки.

### Окремий невирішений доказ: «пізніша за фактичним часом дія»

[Лампорт](https://lamport.azurewebsites.net/pubs/time-clocks.pdf) формалізує причинний **частковий** порядок; [Spanner TrueTime](https://docs.cloud.google.com/spanner/docs/true-time-external-consistency) отримує зовнішній порядок лише завдяки часовій невизначеності з відомою межею та додатковому протоколу. Це **висновок для VIDA**, а не готовий алгоритм з цих джерел: дві довго офлайн дії на незалежних годинниках можуть мати мілісекундні timestamps, які не доводять, котра реально сталася пізніше. Підпис захищає записане значення, але не правильність локального годинника.

Пропозиція для proof: до дії зберігати підписаний wall-time, локальний monotonic sequence та оцінку похибки від останньої перевірки часу; при неперетинних інтервалах порівнювати фактичний час, при причинній залежності — причинний порядок. **Якщо інтервали перетинаються і причинного порядку немає, правильного автоматичного вибору довести неможливо:** показати конфлікт двох намірів і дати власнику створити нову дію. Це не підміняє продуктове правило `Tor-on wins` або `first sync wins`; питання автоматичного результату в такому невизначеному випадку потребує окремого рішення.

### Мінімальні negative fixtures перед статусом Done

- Перемкнути Space під час відкритої direct/relay сесії та при queued large file: після barrier — нуль data-пакетів старим маршрутом.
- Старий Device запитує Space через звичайний канал: отримує тільки policy-only control, durable-commit policy перед Tor data; при Tor outage — pending без direct fallback.
- Повторити policy update/ack і receipt після crash: один policy epoch, жодної подвійної data-operation, status sync тільки після application receipt.
- Два старі Devices окремо офлайн: тест явно показує межу гарантії, а не зелений `all devices Tor protected`.
- Вимкнути Tor у Persona при strict Space/chat: Space/chat лишаються Tor-only; capture включає DNS, STUN/WebRTC, push, preview, diagnostics та cross-Persona circuits.
- Два офлайн toggle з переставленим/неточним годинником: не маскувати невизначений фактичний порядок сортуванням за мілісекундами.
