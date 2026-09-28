---
id: SPEC-vida-tor-network-mode
companions:
  - web-feasibility-and-story.md
  - ../../planning-artifacts/research/technical-vida-tor-anonymous-transport-2026-09-28/research.md
sources: []
status: product-scope-approved-implementation-open
---

> Canonical contract for optional Tor routing and separately enforced mutual-Tor communication. A product decision does not prove a platform implementation.

# VIDA Tor routing and mutual-Tor communication

## Why

Власник будь-якої Persona може за бажанням спрямувати її мережеві дії через Tor. Окремо учасники розмови можуть вимагати Tor-маршрут з обох сторін. Звичайний direct-first режим залишається доступним після явного вимкнення Tor, але таке вимкнення не приховає попередні мережеві зв'язки.

## Capabilities

- **CAP-1**
  - **intent:** Користувач може увімкнути або вимкнути Tor-маршрутизацію для будь-якої Persona в будь-який момент.
  - **success:** При ввімкненому Tor мережеві дії цієї Persona або проходять перевірений Tor path, або чекають локально; вимкнення є явною дією з попередженням про можливу кореляцію, не змінює налаштування інших Personas і не знижує окрему вимогу взаємного Tor.
- **CAP-2**
  - **intent:** Persona може передавати й синхронізувати дозволені ресурси через Tor із чинним E2EE та application receipts; окреме налаштування розмови може вимагати Tor від обох учасників.
  - **success:** Авторизовані клієнти отримують ту саму підписану операцію й підтверджують її застосування; у взаємному Tor-режимі з'єднання не встановлюється, доки обидві сторони не пройдуть перевірку маршруту, без тихого downgrade.
- **CAP-3**
  - **intent:** Користувач може безпечно продовжити локальну роботу, коли Tor недоступний.
  - **success:** Запис durable-збережено як pending; жоден network retry не переходить на звичайний direct, relay, DNS, WebRTC чи push; після відновлення Tor він синхронізується один раз.
- **CAP-4**
  - **intent:** Користувач розуміє межі Tor-режиму та стан його доставки.
  - **success:** UI відрізняє локальний save, очікування Tor, receipt і помилку; не обіцяє абсолютної анонімності чи синхронізації без receipt.

## Constraints

- Увімкнений Tor застосовується до всього network context Persona, не лише Messenger: discovery, чат/групи, файли, нотатки, інші ресурси, receipts, presence, previews, diagnostics і запрошення проходять Tor-specific egress policy або чекають/блокуються. Вимога взаємного Tor є окремою від локального перемикача; її точний рівень налаштування ще відкритий.
- Вимкнення Tor дозволене будь-коли, але UI попереджає, що звичайний маршрут може розкрити IP та пов'язати сесії; черги й активні з'єднання переоцінюються за новою політикою без повторного використання старого route/session context. Строга вимога взаємного Tor ніколи не послаблюється таким перемиканням.
- Контактне запрошення у формі QR, QR-зображення/файла або текстового URI саме по собі не надає доступу до Persona; додавання власного Device вимагає окремого явного підтвердження власника та підписаного DeviceGrant. Секрети відновлення не вкладаються в контактний код.
- Epic 2 може чекати появи іншого пристрою без Tor mailbox. Дзвінки в строгому взаємному Tor-режимі недоступні до окремого доказу маршруту та E2EE у відповідному епіку.
- Tor не замінює E2EE, DeviceGrant, ACL, authority acceptance або чинну модель рівнозначних пристроїв.
- Криптографічні та circuit identifiers різних Personas не можна неявно поєднувати; реалізація перевіряється проти [Tor stream isolation](https://spec.torproject.org/path-spec/stream-isolation.html) і [OWASP MASVS-PRIVACY](https://mas.owasp.org/MASVS/12-MASVS-PRIVACY/).
- Вибір Rust Tor runtime та Iroh adapter є відкритим до release-build, mobile lifecycle і leak-proof tests; номер версії `<1.0` не є автоматичною відмовою.
- Release-1 статичний Chromium Web є повноцінним клієнтом звичайного режиму, але не підтримує мережевих дій будь-якої Persona, коли для неї ввімкнено Tor, або дій розмови зі строгою взаємною вимогою Tor. Без окремого browser proof Web не обходить ці правила звичайним direct/relay.
- Погоджений Release-1 набір встановлюваних клієнтів — Android, iOS і Flutter Windows; Tor-only release gate має доказ для кожного, хоча перша історія може перевірити механізм на Android раніше за platform fan-out.

## Non-goals

- Обіцянка абсолютної анонімності або захисту від саморозкриття, кореляції трафіку та ідентифікуючого вмісту вкладень.
- Сумісність із Delta Chat/Chatmail relay або поштовим протоколом.
- Підміна звичайного direct-first синхронізування всіх Personas Tor-режимом.

## Success signal

Два пристрої однієї Persona з увімкненим Tor обмінюються тестовою нотаткою й повідомленням, отримують application receipts і переживають розрив мережі без втрати локальних змін; захоплення трафіку не знаходять звичайного egress цієї Persona. Окремі тести підтверджують явне перемикання, збереження strict mutual-Tor policy, відсутність тихого fallback і межі Web. Це не закриває дзвінки та всі App-specific перевірки автоматично.

## Open Questions

- Який Rust Tor runtime та Iroh adapter проходять release-build security, onion, mobile-lifecycle і leak-proof tests?
- Чи вимога взаємного Tor налаштовується на розмову, Space або обох рівнях? Чи локальний перемикач Tor синхронізується між Devices Persona, чи кожен Device налаштовується окремо?
