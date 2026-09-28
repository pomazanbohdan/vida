# Adversarial PRD review — статичний Web у Release 1

**Перевірено:** `prd.md`, `addendum.md`, `docs/02-requirements/browser-client-requirements.md`, `docs/02-requirements/platform-nfr.md`, ADR-0021; зріз 2026-09-25. Це аналіз продуктового контракту, не доказ працездатності прототипу.

**Вердикт:** рішення про Web у публічному Release 1 записано послідовно. PRD ще не дає однозначної поведінки для скомпрометованого статичного host, офлайн-перезапуску, закритої вкладки та платформозалежних можливостей. Вони не є підставою відкладати Web, але мають бути закриті до Web implementation fan-out і переведені у перевірні acceptance cases.

## 1. High — Обіцянка «static host не має доступу до відкритого вмісту» занадто абсолютна

**Де:** PRD FR-39, рядки 502–509, особливо 505; ADR-0021, «Рішення»; `REQ-BROWSER-005` і OWASP gate.

**Дві реалізації:** команда A розміщує mutable Flutter JS/Wasm на сторонньому Pages/CDN. Після компрометації host новий скрипт бачить plaintext і викликає Core з повноваженнями авторизованого Web Device. Команда B дозволяє доступ до ключів лише після перевіреної доставки/активації довіреного коду й жорсткої origin policy. Обидві статично розміщені, не передають ключі relay та можуть сказати, що host не є Space authority, але їхні гарантії конфіденційності різні.

**Пропозиція:** у PRD розрізнити відсутність *протокольних прав* статичного host і довіру до коду, який host віддає браузеру; не обіцяти захист від host, що підміняє виконуваний код, до відповідного integrity/profile proof. У Web gate додати сценарії підміни JS/Wasm/service worker, стороннього скрипту, неправильного origin та supply-chain update. Це відповідає [OWASP Third Party JavaScript Management](https://cheatsheetseries.owasp.org/cheatsheets/Third_Party_Javascript_Management_Cheat_Sheet.html) і [OWASP HTML5 Security](https://cheatsheetseries.owasp.org/cheatsheets/HTML5_Security_Cheat_Sheet.html).

## 2. High — «Працює offline» не визначає перезапуск Web без мережі

**Де:** PRD FR-39 рядки 502, 507; §6.2 рядки 536–540 вимагає restart/reopen, але окремий Web↔Android сценарій перевіряє local save + relay sync; `REQ-BROWSER-002/004`, `NFR-PLAT-016` не кажуть прямо, чи офлайн-перезапуск сторінки входить у pass.

**Дві реалізації:** команда A зберігає дані й pending operations у IndexedDB, але після закриття вкладки без мережі не може повторно завантажити app shell/Wasm; команда B кешує узгоджений застосунок і відкриває ті самі дані після offline reload/restart. Обидві мають локальний durable record, але тільки друга виконує очікування користувача «продовжити роботу після перезапуску».

**Пропозиція:** додати окремий Web acceptance flow: online install/load → local save → закрити всі вкладки → мережа offline → відкрити Web заново → прочитати й змінити дані → повернути мережу → sync; повторити з оновленням bundle та eviction. Якщо певний browser/profile не підтримує цей flow, це має бути явне обмеження підтримки, а не невидимий виняток. [MDN Service Worker API](https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API) описує кешування shell для офлайн-запуску; вибір механізму лишається технічним.

## 3. High — «Повний Web» не уточнює дзвінки й нагадування за закритої вкладки

**Де:** PRD FR-12 (дзвінки), FR-37 рядки 477–486 (нагадування), FR-39 рядки 502–509, §6.3 call/Web gate рядки 549, 554; Addendum §G забороняє external push для Autonomous anonymous Persona. `NFR-PLAT-016` відкидає успадковані native background guarantees, але продукт не пояснює користувачу, що стається після закриття вкладки.

**Дві реалізації:** команда A забезпечує дзвінок/нагадування лише доки Web відкритий; команда B реєструє Web Push або background wake для всіх Personas, щоб досягти «повного» Messenger/Calendar, порушуючи anonymous boundary. Для користувача це протилежні очікування щодо пропущеного дзвінка й нагадування.

**Пропозиція:** визначити user-facing platform promise: у відкритій активній вкладці дзвінки/нагадування проходять повну conformance; у закритій вкладці background delivery не гарантується; Autonomous anonymous Web не реєструє зовнішній push; на повторному відкритті відображаються пропущені події після sync. Якщо Public Persona матиме opt-in browser push, окремо описати згоду, ідентифікатор, unlinkability та режим без нього. Web call suite має містити browser↔native/Web↔Web і close/reopen cases, а не успадковувати G2 native-only доказ.

## 4. Medium — FR-4/онбординг не розрізняє ручні Web Contacts і системний імпорт

**Де:** PRD FR-4 рядки 145–155 і FR-33 рядки 441–449 пропонують добровільний системний імпорт; §6.3 рядок 545 перевіряє preview system import, а §6.3 рядок 554 і `REQ-BROWSER-008` кажуть, що Web проходить усі застосовні Core flows. Немає списку, які саме платформи повинні мати системний contact connector.

**Дві реалізації:** команда A робить у Web повні Contact Cards/пошук/share, але без доступу до адресної книги ОС; команда B активує browser contact picker у частині браузерів і заявляє Web FR-4 pass. Через невизначеність один QA зарахує першу реалізацію, інший — ні.

**Пропозиція:** у FR-4/feature matrix зазначити: Contact Cards обов'язкові на чотирьох клієнтах; системний імпорт у Release 1 обов'язковий лише на явно названих платформах. Для Web або зафіксувати виключення з поясненням UI, або вибрати supported-browser subset і вимагати його permission/preview fixtures. Не підміняти відсутній системний імпорт `.vcf`, який явно відкладено.

## 5. Medium — «Без єдиного зовнішнього сервера» потребує Web-винятку в основній обіцянці

**Де:** PRD §1 рядки 17–21 обіцяє базову роботу без одного зовнішнього сервера; FR-39 рядки 502–509 та §5 рядок 515 вимагають VIDA-operated relay для Web sync. `REQ-BROWSER-003/007` правильно кажуть «без paid Hosted Space», але це не означає «без інфраструктури VIDA».

**Дві інтерпретації:** користувач A очікує, що Web↔Android sync працюватиме повністю автономно між peers або через будь-який сумісний self-hosted relay. Команда B реалізує лише безплатний VIDA-operated relay; за його недоступності Web продовжує локальну роботу, але не синхронізується. Друга реалізація узгоджена з ADR-0021, перша — з нечіткою верхньою обіцянкою.

**Пропозиція:** уточнити в product-facing тексті: local Web work не потребує VIDA application server чи підписки, але Web↔peer sync у Release 1 потребує доступного Iroh relay; стан «relay недоступний» не є збоєм локального save. Окремо вирішити, чи підтримується сторонній/self-hosted протокольно сумісний relay у відкритому baseline; це не повинно непомітно перетворитися на paid-host залежність.

## Визнані gates, не нові суперечності

- OQ-19 уже прямо вимагає supported-browser matrix, Rust/Wasm bridge, key/recovery і Web media proof до fan-out. Його наявність — правильний blocker; `status: final` PRD не означає, що доказ пройдено.
- `REQ-BROWSER-002/004/005` коректно розрізняють «єдина локальна копія», local save, application sync і relay outage; реалізаційні fixtures потрібні, але продуктова термінологія вже існує.
- Відсутність durable mailbox у Release 1 означає, що relay не зберігає повідомлення для адресата, який недоступний. Це явно написано в ADR-0021/Addendum і не є прихованим дефектом, доки UX не заявляє гарантовану доставку при закритих клієнтах.
