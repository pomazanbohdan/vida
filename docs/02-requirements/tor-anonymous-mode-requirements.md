---
id: REQ-TOR-001
status: approved-product-goal-technical-proof-open
last_updated: 2026-09-28
source_refs:
  - ../../_bmad-output/planning-artifacts/research/technical-vida-tor-anonymous-transport-2026-09-28/research.md
decision_refs:
  - ../00-governance/decision-batch-epics-1-2-2026-09-28.md
---

# Опціональна Tor-маршрутизація Persona — Release 1

Це продуктова вимога Release 1, **не** затвердження конкретної Rust-бібліотеки чи твердження, що «абсолютна анонімність» технічно доведена.

| ID | Вимога |
|---|---|
| `REQ-TOR-001` | Будь-яка Persona `MUST` мати опціональний перемикач Tor-маршрутизації, який власник може ввімкнути чи вимкнути в будь-який момент. За вимкненого Tor діє звичайний direct-first Iroh із дозволеним VIDA relay fallback. Перемикання однієї Persona не змінює інші. |
| `REQ-TOR-002` | У Tor-only профілі повідомлення й групи, вкладення/файли, sync нотаток та інших ресурсів, lookup/invites, receipts, presence, дзвінки, metadata requests, preview URLs і diagnostics `MUST` або проходити перевірений Tor path, або бути вимкнені/відкладені. AppPackage `MUST NOT` обходити network policy Core. |
| `REQ-TOR-003` | За ввімкненого Tor, при його недоступності `MUST` бути fail-closed: локальні записи можуть durable-зберігатися, але мережеві дії чекають; немає автоматичного fallback на direct, звичайний relay, STUN/TURN, proxy-bypassing HTTP/DNS чи push. Власник `MAY` явно вимкнути Tor з попередженням, що попередні з'єднання не можна приховати заднім числом, а новий звичайний маршрут може пов'язати сесії. Черги й з'єднання `MUST` переоцінюватися за поточною політикою; строгий взаємний Tor не послаблюється. |
| `REQ-TOR-004` | Tor relay/onion service `MAY` забезпечувати discovery і передачу даних, але E2EE payload/ACL/receipts лишаються VIDA Core contracts. Якщо власний relay використовується в Tor-only контексті, до нього звертаються через Tor; звичайний relay fallback не є Tor path. |
| `REQ-TOR-005` | Різні Personas `MUST` мати ізольовані Tor streams/circuits і незв'язувані network identifiers відповідно до threat model. Identity key однієї Persona `MUST NOT` автоматично ставати onion/Iroh endpoint key іншої, незалежно від рівня її публічності. |
| `REQ-TOR-006` | UI `MUST` описувати режим як «Tor-захищений» і чесно показувати обмеження: він приховує мережеву адресу в заявленому scope, але не анонімізує добровільно розкриту особу, вміст файлів, поведінкові зв'язки чи весь системний трафік. |
| `REQ-TOR-007` | Вибір Rust Tor runtime (Arti, контрольований Tor daemon або інший), Iroh custom-transport adapter та target-platform support `MUST` пройти окремий dependency/security review і release-build conformance. Версія `<1.0` сама по собі не дискваліфікує бібліотеку; безпека, функціонал, розвиток, issues та перевірні тести визначають вибір. |
| `REQ-TOR-008` | Наявність Tor-режиму на Android, iOS чи Flutter Windows `MUST NOT` заявлятися до packet-capture і leak tests для DNS, WebRTC/ICE, Iroh path, HTTP, push, notifications, calls, previews і background lifecycle. Статичний Chromium Web у Release 1 підтримує повний звичайний режим, але `MUST NOT` виконувати мережеві дії Persona з увімкненим Tor або strict mutual-Tor розмови через звичайний transport; дозволена локальна робота лишається pending. Browser Tor mode — окремий пізніший increment після нового proof. |
| `REQ-TOR-009` | Строга взаємна вимога Tor `MUST` бути окремою опцією комунікації. Локальне ввімкнення Tor не вимагає Tor від співрозмовника і не повідомляє йому про маршрут за замовчуванням. Якщо strict policy діє, обидва peer-и `MUST` використовувати підтверджений VIDA Tor path для цієї комунікації; недоступність peer-а/Tor залишає дію pending, без downgrade. Це не є заявою про весь трафік чужого пристрою. |
| `REQ-TOR-010` | Контактний locator/invite `MAY` передаватися як QR, QR-зображення/файл або еквівалентний текстовий URI. Його обробка `MUST` перевіряти тип, версію та підпис/зв'язок із Persona й `MUST NOT` сама надавати DeviceGrant. Авторизація другого власного Device `MUST` мати окремий owner-approved, одноразовий і пов'язаний із ключем нового Device flow; recovery secret не входить у контактний payload. У Epic 2 офлайн-пристрій `MAY` лишатися pending без Tor mailbox. |

Референсна оцінка та межі доказу: [технічний звіт](../../_bmad-output/planning-artifacts/research/technical-vida-tor-anonymous-transport-2026-09-28/research.md). OWASP security verification застосовується до ключів, мережевих викликів, Web-origin і діагностики; Tor-specific anonymity перевіряється окремими leak/correlation tests, а не лише OWASP checklist.
