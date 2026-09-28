# Chromium Web feasibility and proposed story boundary

## Чотири варіанти

| Варіант | Що треба змінити | Чи доводить анонімність звичайного статичного Web? |
|---|---|---|
| Звичайний статичний Flutter Web + Rust/Wasm Tor | Потрібен мережевий socket path, якого стандартна сторінка не має. [Chrome Direct Sockets](https://developer.chrome.com/docs/iwa/direct-sockets) доступний IWA, не звичайній HTTPS-сторінці. | Ні, сам Wasm не дає raw TCP/UDP. |
| WebSocket-to-Tor gateway | Оператор gateway бачить IP браузера; code host також бачить IP при звичайному завантаженні сторінки. | Ні, навіть коли gateway далі користується Tor. |
| Local Tor/native helper або системний proxy + Chromium | Потрібна інсталяція/налаштування поза статичним Web; браузерні DNS/WebRTC/fingerprint/cookie витоки лишаються. | Ні без окремого hardened browser profile і перевірки; [Tor Project застерігає](https://support.torproject.org/tor-browser/security/using-tor-with-other-browsers/). |
| Chromium Isolated Web App + Direct Sockets | Це новий формат постачання, не GitHub Pages-сторінка; потрібні adapter, security review і platform support. | Теоретичний кандидат для prototype, не доведений Release-1 клієнт. |

**Погоджено:** Release-1 статичний Chromium Web має всі звичайні VIDA функції та direct-first/relay fallback. Для будь-якої Persona з увімкненим Tor або strict mutual-Tor розмови її мережеві дії недоступні/pending; звичайне з'єднання не є прихованим fallback. Якщо власник явно вимкне Tor, Web знову може працювати звичайним маршрутом, але окрема strict policy лишається чинною. Встановлювані Android, iOS і Flutter Windows мають Tor release gate; Web Tor increment вимагає окремого доказу.

## Межа Story 2.2 і наступна історія

Story 2.2 уже визначає **звичайну** пряму Android↔Web синхронізацію нотатки. Її завершення доводиться direct-path trace, canonical operation parity, local durability, authorization і application receipt. Tor не можна додати до 2.2 як непомітний критерій: це окремий мережевий профіль з іншим threat model.

**Переглянута послідовність у `epics.md`:** Story 2.13 — перемикач Tor для будь-якої Persona на Android; Story 2.14 — окрема авторизація другого Android Device через QR scan, QR image/file або текстове представлення; Story 2.15 — синхронізація простої нотатки через Tor між ними. Вимоги власника до трьох результатів погоджені; деталізовані acceptance criteria залишаються на story-review перед статусом implementation-ready. Story 2.15 — перший Android-доказ, не повна перевірка Release 1:

- Spec checkpoint: `SPEC-vida-tor-network-mode` CAP-1–CAP-3 і `REQ-TOR-001–008`; Story 2.2/2.3 зберігають звичайний прямий і relay шляхи для іншого профілю.

- Given двоє авторизованих Devices однієї Persona з увімкненим Tor, When вони синхронізують просту нотатку, Then обидва застосовують той самий signed operation і видають application receipt; жодний network trace не показує прямого IP-шляху до peer або звичайного VIDA relay.
- Given Tor недоступний, When користувач змінює нотатку, Then durable local save доступний, network action pending і жоден звичайний fallback не запускається.
- Given інша Persona на тому самому Device використовує звичайний профіль, When обидві активні, Then їхні ключі, Tor circuit tokens, presence, lookup і маршрути не змішуються. Вимкнення Tor не знімає окрему strict mutual-Tor вимогу розмови.
- Done checkpoint для цієї першої історії: exact-pinned Rust Tor runtime/Iroh adapter, Android native proof, loss/reconnect replay, packet-capture leak matrix, OWASP MASVS-NETWORK/PRIVACY review та accessibility/wording. iOS і Flutter Windows мають повторити conformance у наступних platform stories до Release 1; Android proof не є дозволом відкласти ці платформи. Незалежний security review privacy-critical path залишається окремим release evidence gate, рішення про його форму відкрите в OQ-0081.

Після 2.15 App-specific acceptance не успадковується автоматично: Messenger/attachments, Files, presence, calls, previews і diagnostics мають власні негативні leak fixtures у відповідних епіках. Дзвінки в strict mutual-Tor режимі недоступні до доказу в Epic 8; звичайний fallback неприпустимий. Web/platform scope вирішено; story не слід позначати `done` до виконання fixtures, а release-ready для Tor вимагає додаткових iOS/Windows та App-specific proofs.

**Аудит:** Stories 2.13–2.14 закривають раніше пропущені передумови активного Tor й авторизованого другого Device. Контактний locator не є DeviceGrant: той самий payload можна сканувати, імпортувати як QR-зображення або вставити як текст, але його тип і контекст визначають лише намір. Наявність коду не дає доступу; enrollment має окреме owner confirmation. Якщо peer офлайн, стан pending достатній у Epic 2, mailbox не потрібен. Точний URI/crypto/expiry contract і платформні fixtures ще є implementation gates.

## Вхід у BMad Step 1 для актуалізації епіків

`epics.md` містить 40 FR, 17 NFR, 12 UX-DR, AR-16 і переглянуту послідовність 2.13–2.15. Продуктові рішення погоджені; story-level review, точний контракт запрошення, iOS/Windows/Web трасованість і conformance proof лишаються відкритими.
