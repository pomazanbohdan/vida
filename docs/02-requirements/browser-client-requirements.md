---
id: REQ-BROWSER-001
status: approved
last_updated: 2026-09-28
decision_refs:
  - ADR-0018
  - ADR-0021
source_refs:
  - https://cheatsheetseries.owasp.org/cheatsheets/HTML5_Security_Cheat_Sheet.html
---

# Release-1 static browser client

## Прийнята продуктова межа

| ID | Вимога |
|---|---|
| `REQ-BROWSER-001` | Public Release 1 `MUST` постачати повноцінний статично розміщуваний Flutter Web client зі спільною Rust Core-семантикою через конформний Rust/Wasm boundary. Browser profile `MUST` локально зберігати дозволені дані й після авторизованого Device enrollment синхронізувати їх з іншими VIDA peers без paid Hosted Space. До підключення peer браузер може мати лише одну локальну копію; це не окремий несинхронізований продуктовий tier. |
| `REQ-BROWSER-002` | UI `MUST` пояснювати під час підключення/в деталях, що очищення browser/site data, втрата profile або browser eviction можуть зробити останню локальну копію недоступною. UI `MUST` позначати стан «єдина локальна копія», запитувати persistent storage і надавати перевірений encrypted export/restore без блокування кожного редагування або окремого browser-warning після кожного успішного save. |
| `REQ-BROWSER-003` | Опційний paid Hosted Space/browser mode `MAY` з'явитися окремим наступним increment. Release-1 Web↔Device sync `MUST NOT` залежати від hosted subscription, server-side App runtime або mailbox. Тарифікація, SLA та hosted trust/key profile не входять у Release 1. |
| `REQ-BROWSER-004` | Web UI `MUST` розрізняти «збережено локально», «очікує синхронізації», «синхронізовано» після application receipt та втрату доступного transport path. Діагностика `MAY` уточнювати direct/relay/fallback state, але сам маршрут не є receipt. Локальна копія або transport ACK `MUST NOT` подаватися як remote backup, доставка чи business acceptance. |
| `REQ-BROWSER-005` | Browser storage `MUST NOT` зберігати plaintext secrets у `localStorage`. Підключений клієнт у тому самому profile за замовчуванням відкривається без повторного recovery secret; додаткове локальне блокування є опцією, а не гарантією захисту скомпрометованого origin/profile. Невдалий write `MUST` показати «Не збережено», залишити видимий текст для retry/копіювання й не обіцяти його після закриття вкладки. Canonical data store, encryption/key wrapping, persistence request, quota handling, export/restore та XSS/service-worker protections потребують окремого security contract і ASVS verification. |
| `REQ-BROWSER-006` | Android/iOS/Windows Flutter installed clients і статичний Flutter Web client разом утворюють public Release 1; відсутність повного Web conformance `MUST` блокувати Release 1. Paid hosted browser лишається пізнішим increment. |
| `REQ-BROWSER-007` | Статичний Web build `MUST` працювати без VIDA application server. Web↔peer data sync `MUST` спершу виконати обмежену за prototype policy спробу прямого автентифікованого Iroh-шляху; relay `MAY` переносити lookup/SDP/ICE, але не application payload до недоступності/таймауту direct. Після цього дозволений явно налаштований VIDA-operated protocol-compatible Iroh relay як E2EE fallback, якщо власник Persona не вимкнув relay-передачу; неочікуваного public/N0 default немає. Stock Iroh/Wasm не є доказом прямого browser P2P: WebRTC custom transport **усередині Iroh/VidaNodeHost**, сумісність з exact-pinned Iroh 1.2, Android↔Web/Web↔Web direct tests (без TURN/relay candidate pair), path trace, receipt, fallback і міграція без дублювання є обов'язковим Release-1 gate. Якщо жоден дозволений шлях недоступний, remote зміни лишаються pending. Relay `MUST NOT` читати E2EE payload чи ставати mailbox/authority. |
| `REQ-BROWSER-008` | Web як рівнозначний Device `MUST` проходити той самий operation/ACL/merge/receipt contract і окремі browser-specific tests для Chat, Notes, Project, Files, Calendar, AppPackages та E2EE calls у звичайному network profile. Release-1 підтримка Web обмежується актуальними Chromium-based браузерами; Firefox/Safari не є release gates. Точні мінімальні версії, Rust/Wasm bridge і browser media/key profile `MUST` бути доведені до implementation fan-out. Статичний Web Release 1 не виконує мережеві дії будь-якої Persona з увімкненим Tor або розмови зі strict mutual-Tor вимогою: вони блокуються/pending без переходу на звичайний маршрут (`REQ-TOR-008–009`). Після явного вимкнення Tor Web може працювати у звичайному режимі, якщо немає окремої strict policy. |
| `REQ-BROWSER-009` | Статичний host не є Persona/Space authority, але доставляє виконуваний JS/Wasm і за компрометації може прочитати доступний клієнту plaintext. Web `MUST NOT` заявляти zero-knowledge відносно host-а; UI `MUST` показати повний current HTTPS origin і пояснити довіру до його оператора перед Device enrollment. Кожний інший origin, навіть з ідентичним кодом, є окремим Web Device з власними ключами й grant. Origin/code-delivery integrity, version pinning, XSS isolation і threat model `MUST` пройти Release-1 gate. |
| `REQ-BROWSER-010` | Web `MUST` дозволяти ручне створення Contact Cards і sync дозволених карток, але Release 1 `MUST NOT` обіцяти браузерний імпорт системної адресної книги чи `.vcf`. Після закриття або призупинення вкладки Web `MUST NOT` обіцяти постійний sync, вхідний дзвінок або точне локальне нагадування; при повторному відкритті pending state `MUST` узгоджуватися з Core. |

## OWASP gate

[OWASP HTML5 Security](https://cheatsheetseries.owasp.org/cheatsheets/HTML5_Security_Cheat_Sheet.html) вказує, що XSS може читати/змінювати browser storage, а client-side storage не слід вважати конфіденційним без окремого захисту. Browser profile `MUST` пройти ASVS, XSS, storage/key, origin isolation, code-delivery integrity, Clear-Site-Data/logout та backup/restore tests.
