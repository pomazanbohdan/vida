---
id: ADR-0021
status: accepted
last_updated: 2026-09-25
source_refs:
  - ../../../_bmad-output/planning-artifacts/research/technical-iroh-direct-connectivity-uniclipboard-re-2026-09-25/research.md
  - ../../../_bmad-output/planning-artifacts/prds/prd-vida-2026-09-22/prd.md
  - https://www.iroh.computer/blog/iroh-and-the-web
  - https://github.com/n0-computer/iroh/discussions/4024
  - https://github.com/anchalshivank/iroh-webrtc-transport
decision_refs:
  - ADR-0005-iroh-transport-foundation.md
  - ADR-0018-browser-local-and-hosted-modes.md
  - ADR-0020-flutter-windows-in-release-1.md
supersedes:
  - ADR-0014-browser-exclusion
  - ADR-0018-free-browser-local-only-and-post-v1-scope
  - ADR-0020-browser-post-v1-consequence
superseded_by: []
---

# ADR-0021: статичний Web-клієнт VIDA входить до Release 1

## Рішення

Public Release 1 `MUST` включати повноцінний статично розміщуваний Flutter Web-клієнт поряд із встановлюваними Flutter Android, iOS і Windows. Browser profile `MUST` використовувати спільну нормативну VIDA Core-семантику через перевірений Rust/Wasm boundary, а не окрему реалізацію прав, операцій чи merge. Web-клієнт `MUST` зберігати локальні дані у browser storage й, після явної авторизації як рівнозначного Device, синхронізувати дозволені Spaces з іншими VIDA Devices без платного Hosted Space. Статичний host не є бізнесовим власником Persona/Space, проте оператор або компрометація origin може підмінити JS/Wasm та прочитати plaintext запущеного клієнта; безпека Web залежить від довіри до доставки коду і не заявляється як zero-knowledge відносно host-а.

Web↔peer синхронізація `MUST` бути direct-first: після пошуку peer клієнт робить обмежену за виміряною prototype policy спробу встановити автентифікований прямий Iroh transport path. Пошук/SDP/ICE-сигналізація `MAY` проходити через relay, але application payload `MUST NOT` пересилатися ним, доки прямий шлях не визнано недоступним або не вичерпано цю спробу. Тоді VIDA-operated protocol-compatible Iroh relay переносить той самий E2EE application envelope як fallback. Для прямих Web-з'єднань нормативна межа — browser-compatible WebRTC **custom transport усередині Iroh/VidaNodeHost**, не паралельний WebRTC-протокол синхронізації; якщо цей механізм не проходить gate, заміна межі потребує окремого архітектурного рішення. Пошук/сигналізація, прямий transport, relay forwarding, durable mailbox та business authority є різними функціями. Relay `MUST NOT` ставати durable mailbox, business authority чи доказом application sync. Після появи прямого шляху клієнт `SHOULD` переходити на нього без повторного domain apply; жодна зміна маршруту не змінює operation ID чи права. Native peers зберігають direct-first і relay-disabled LAN profile за ADR-0005.

Поточний stock Iroh/Wasm browser transport не надає прямого browser↔native шляху. Тому Release-1 Web direct path — окремий implementation/conformance gate: exact-pinned Iroh 1.2 + browser-compatible WebRTC custom transport має пройти збірку, Android↔Web та Web↔Web, supported-browser/security/licensing/maintenance перевірку і application receipt. Два наявні community WebRTC-мости орієнтовані на старіші Iroh 0.x та unstable custom-transport API: це лише референси, не затверджені залежності й не доказ сумісності з 1.2. Для direct-доказу negotiated WebRTC pair `MUST NOT` бути TURN/relay; тест фіксує клас маршруту та окремо облік lookup/signaling bytes і encrypted application-forwarding bytes без plaintext. Конфігурація VIDA relay `MUST` явно виключати неочікуваний public/N0 default. Якщо direct-кандидат не проходить gate, це відкрите блокувальне рішення для Web Release 1, а не дозвіл мовчки зробити relay-first клієнт. Неможливість і прямого, і relay-шляху залишає remote operations pending, не скасовуючи durable local save.

Browser profile `MUST` мати окремі conformance gates для persistent storage/quota/eviction, key custody/recovery, origin/XSS і довіри до статичного коду, offline cold reopen з повним кешованим shell/JS/Wasm, lifecycle/reconnect, Rust/Wasm bridge, accessibility, усіх обов'язкових Core Apps та E2EE calls. Закрита або призупинена вкладка не гарантує фоновий дзвінок, sync чи точне спрацювання нагадування; після відкриття стан узгоджується з Core. Статичні Flutter/JS/Wasm assets і protocol/schema compatibility `MUST` оновлюватися як цілісний версійований release; змішані кешовані версії не можуть мовчки виконувати несумісні дії. Втрата browser keys `MUST NOT` переносити старий DeviceGrant на нові ключі: потрібне нове enrollment, а старий grant має бути доступний для відкликання через чинний Device/recovery. Нестача відповідної browser capability не дає права мовчки замінити його урізаним companion; вона блокує відповідний Release-1 gate до доведеного рішення. Paid Hosted Space, durable mailbox і server-rendered private web залишаються окремими пізнішими можливостями.

## Межа з попередніми рішеннями

- ADR-0014 лишається чинним для offline-поведінки встановлюваних клієнтів, але його browser exclusion скасовано.
- ADR-0018 лишається історією browser storage/hosted-моделі; його правило «без peer/device sync у free browser» та post-v1 відкладення браузера скасовано. Browser може бути єдиною локальною копією до підключення peer, але після авторизованого підключення має повну sync-модель.
- ADR-0020 лишається чинним для Flutter Windows/WinUI 3; лише його фраза про browser після Release 1 скасована.

## Доказ для випуску

- статичний build відкривається без VIDA application server і створює/відновлює Persona та durable Resource;
- Web↔Android і Web↔Web авторизують рівнозначні Devices; окремі тести доводять direct Note sync з автентифікованим peer, application receipt, без relay application forwarding та без TURN-path під виглядом direct; bounded direct-attempt → зашифрований relay fallback; перемикання маршруту зі сталим operation ID без дублювання; pending при втраті обох шляхів. Exact-pinned Iroh 1.2 build, supported-browser matrix, ліцензійний і maintenance review custom transport мають окремий доказ до затвердження залежності. Пізніші Message/Task/File проходять ті самі транспортні інваріанти;
- browser storage/quota/eviction і ключові негативні сценарії не спричиняють хибного «збережено/синхронізовано»; якщо browser profile є єдиною копією, UI до втрати попереджає про ризик очищення/eviction і надає перевірений export/restore, не обіцяючи фізично запобігти видаленню browser data;
- offline cold reopen, цілісне оновлення Flutter/JS/Wasm assets, втрата browser profile/keys і re-enrollment/revocation проходять негативні та recovery fixtures;
- supported-browser matrix та всі застосовні Release-1 функції, E2EE calls, accessibility і OWASP ASVS/browser-security fixtures проходять окремий Web gate.

## Відкладені механізми

Точний Rust/Wasm↔Flutter bridge, прямий browser transport/WebRTC adapter, browser storage/key/recovery profile, offline app-shell/cache strategy, supported-browser matrix, AppPackage execution profile, media adapter і relay/lookup deployment profile обираються за прототипними доказами до implementation fan-out. Включення Web до Release 1 не означає, що ці механізми вже доведені.
