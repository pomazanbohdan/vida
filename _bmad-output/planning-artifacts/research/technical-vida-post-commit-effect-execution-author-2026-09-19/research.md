---
title: 'Technical research: VIDA post-commit effect execution authority'
type: technical
topic: 'VIDA post-commit effect execution authority across personal and federated Spaces'
decision: 'Compare effect executor models and decide separately for autonomous personal and federated Spaces'
source: 'primary official documentation and papers retrieved 2026-09-19'
status: complete
preset: standard
validation: normal
verified_claims: 5
unverified_claims: 1
created: '2026-09-19'
updated: '2026-09-19'
---

# Technical research: VIDA post-commit effect execution authority

**Decision this research serves:** determine which actor executes one post-commit external effect in each Space mode, and how retries, privacy and offline behavior affect that choice.

## Висновок для обговорення

Рекомендую **не призначати одного універсального виконавця всіх реакцій**. Для повністю автономного персонального Space — durable intent на пристрої-ініціаторі, виконання після його повернення онлайн; інший пристрій може перебрати роботу лише через окремий підтверджений механізм передачі. Для приватного федеративного Space — вузол доставляє шифротекст і, якщо дозволено маршрутизаційною політикою, короткий непрозорий сигнал; пристрій адресата розшифровує подію та формує видиме сповіщення. Автоматизації, що звертаються до зовнішнього API або читають приватний вміст, потребують окремо призначеного й уповноваженого виконавця. Це архітектурна рекомендація, **не затверджене рішення VIDA**. [2][3][4][6][8]

Погоджені користувачем обмеження цього обговорення: автономна зовнішня дія може чекати онлайн-пристрою; у приватному спільному проєкті федеративний вузол бачить лише сигнал, а деталі обробляє пристрій. Ці вимоги походять від користувача, не від зовнішніх джерел; для public/business Spaces правило ще не узагальнено.

Головне застереження: **«один логічний намір» не тотожний «рівно один фізичний виклик API/показ push»**. Outbox, queue і workflow можуть повторити доставку чи роботу після збою; зовнішній результат залежить від idempotency контракту адресата або подальшого звіряння невідомого результату. [5][6][7][8][9]

## 1. Що саме треба виконати один раз

Для `task.assign` варто розрізняти: (a) один прийнятий доменний факт `task.assigned`; (b) один логічний намір повідомити конкретного адресата; (c) спроби транспорту до його пристроїв; (d) локальний показ тексту на кожному пристрої. Це пропонована декомпозиція VIDA, не готовий протокол. Matrix уже відділяє homeserver, Push Gateway і провайдера push, а також вимагає приглушувати дублікати за event ID. [4] Automerge демонструє offline-створення й пізнішу синхронізацію стану, але не призначає виконавця зовнішніх бізнес-ефектів. [1]

У таблиці нижче **виконавець** — той, хто робить дію поза журналом VIDA; **намір ефекту** — збережене завдання на цю дію; **fencing** — захист від старого виконавця, який повернувся після втрати права діяти. Це не назви затверджених VIDA API.

Для приватного Space вміст задачі може залишатися E2EE: сервер-посередник не має ключів для читання приватної історії в моделі на кшталт Signal. [3] За аналогією Matrix `event_id_only` дозволяє надіслати wake-up без тексту події; це **референс маршрутизації, не доказ**, що VIDA уже має безпечний recipient token чи точну push-схему. [4] Щоб вузол сповістив саме Олену, йому все одно потрібен мінімальний маршрутизаційний ідентифікатор або fan-out політика; обсяг видимих метаданих відкритий.

## 2. Варіанти виконавця

| Варіант | Personal autonomous | Private federated | Головний компроміс |
|---|---|---|---|
| A. Пристрій-ініціатор | Відповідає погодженому очікуванню: intent чекає його повернення онлайн | Може сформувати шифрований intent і відправити вузлу | Простий ownership, але зламаний/втрачений пристрій потребує явного recovery/handoff; mobile background не гарантує негайний запуск. [2] |
| B. Будь-який peer після election/lease | Доступніший за A, коли є другий пристрій | Можливий серед клієнтів | Складна координація; сам вибір лідера не забезпечує fencing і не виключає двох активних виконавців. [10][11] |
| C. Федеративний вузол як повний worker | Порушує вимогу «без постійного вузла» для повної автономності | Дає always-on retries/timers, **лише якщо** має потрібні дані, повноваження та ключі | Несумісний із ciphertext-only правилом приватного проєкту для правил, які читають задачу; хостинг також не створює exactly-once зовнішній результат. [3][5][8] |
| D. Розділення за типом ефекту | Локальний worker; дія може чекати | Вузол — opaque delivery/wake-up; авторизований клієнт — зміст і локальний показ; окремо делегований сервіс — лише для дозволених інтеграцій | Найкраще відповідає погодженим обмеженням, але потребує чітких типів effect, grants, routing metadata і recovery. [2][4][6] |

**Рекомендація для прототипу:** D як політика, A як перший executor для автономних зовнішніх дій; не впроваджувати B як «легкий failover» без authority/fencing. Для приватної задачі вузол виконує **доставку сигналу**, а не `task.assign` чи content-dependent automation. `task.assigned` і notification intent не генеруються повторно на кожній репліці під час sync.

## 3. Надійність: межа гарантій

AWS описує transactional outbox: commit даних і pending-події разом, але downstream може отримати дубль; потрібна idempotency. [6] Temporal після timeout або втрати worker повторює Activity навіть якщо функція вже запускалася. [7] Google Spanner прямо вказує, що виклик зовнішнього API не можна атомарно зв'язати з ACK черги; після збою потрібні dedupe або звіряння. [8] Cloudflare alarms — приклад hosted wake-up із at-least-once retries, не гарантія одноразового зовнішнього ефекту. [5]

Пропонований VIDA-контракт для подальшого обговорення: сталий `EffectIntentId`, прив'язка до прийнятого факту/одержувача/каналу; durable pending state до спроби; один призначений виконавець; журнал спроб і результату; той самий idempotency key для повторів; стан `unknown` і reconcile, коли виклик міг пройти, але ACK втрачено. Це **проєктна пропозиція**, а не висновок, що її автоматично надає Iroh або workflow engine. Stripe показує робочий idempotency key, але документує можливе видалення ключа після 24 годин; для бронювання/платежу власна гарантія має враховувати строк адресата. [9]

На iOS silent/background push є best-effort і може бути throttled; він придатний як сигнал швидкого оновлення, не як єдина гарантія виконання. [2] Якщо всі персональні пристрої вимкнені, погоджена політика дозволяє чекати. Якщо зовнішній API не підтримує ідентифікатор повтору й не дозволяє перевірити попередній результат, чесний статус — «можливий дубль або ручне звіряння», не «exactly once». [8][9]

## Перехресний висновок і заперечення

Local-first синхронізація, E2EE та фонові обмеження клієнтів разом пояснюють, чому «вузол завжди виконує все» і «кожен peer виконує те, що побачив» обидва не підходять приватному Space. [1][2][3][4] Контраргумент до рекомендованого D: користувач може отримати видиме сповіщення пізніше, якщо його пристрій не прокинувся; це реальне UX-обмеження, а не баг, який усуває обіцянка негайного push. [2] Для нечутливих business/public workflows призначений server worker може бути доречнішим, але його доступ і scope потребують окремого рішення.

## Питання перед ADR

1. Для людини з кількома пристроями: один логічний notification intent може показати alert **на кожному** пристрої, чи лише на одному активному?
2. Як адресувати opaque wake-up без зайвого розкриття assignee/Space метаданих федеративному вузлу? Це пов'язано з `OQ-0026`.
3. Якщо пристрій-ініціатор назавжди втрачено, хто й за якою authority може перебрати pending external effect? Це пов'язано з `OQ-0033` та `OQ-0045`.
4. Які інтеграції (City Portal booking, webhook, email, payment) підтримують idempotency key/status lookup, а для яких потрібне ручне reconcile?

## Джерела

| № | Підтримує | Першоджерело | Опубліковано / оновлено | Переглянуто | Впевненість |
|---|---|---|---|---|---|
| [1] | offline sync, не execution ownership | [Automerge Network Sync](https://automerge.org/docs/tutorial/network-sync/) | н/в | 2026-09-19 | висока для sync; відсутність executor — висновок |
| [2] | background push best-effort | [Apple Developer](https://developer.apple.com/documentation/usernotifications/pushing-background-updates-to-your-app) | н/в | 2026-09-19 | середня; пряме джерело, без незалежної перевірки |
| [3] | server blind to E2EE content | [Signal linked devices](https://signal.org/blog/a-synchronized-start-for-linked-devices/) | 2025-01-27 | 2026-09-19 | середня |
| [4] | per-device push, `event_id_only`, dedupe | [Matrix Specification v1.19](https://spec.matrix.org/v1.19/push-gateway-api/) | 2026-07-08 (release) | 2026-09-19 | висока |
| [5] | hosted alarm retries | [Cloudflare Durable Objects](https://developers.cloudflare.com/durable-objects/api/alarms/) | 2026-04-21 | 2026-09-19 | висока |
| [6] | outbox і дублікати | [AWS Prescriptive Guidance](https://docs.aws.amazon.com/prescriptive-guidance/latest/cloud-design-patterns/transactional-outbox.html) | н/в | 2026-09-19 | висока разом із [8] |
| [7] | Activity retries після втрати worker | [Temporal documentation source](https://github.com/temporalio/documentation/blob/main/docs/encyclopedia/activities/activity-execution.mdx) | поточний main, дата н/в | 2026-09-19 | висока разом із [5] |
| [8] | non-atomic external API і queue ACK | [Google Spanner docs](https://docs.cloud.google.com/spanner/docs/queues/queues-at-most-once) | н/в | 2026-09-19 | висока разом із [6] |
| [9] | idempotency key і строк зберігання | [Stripe API](https://docs.stripe.com/api/idempotent_requests) | н/в | 2026-09-19 | висока для Stripe; не узагальнювати на інші API |
| [10] | election без fencing | [Kubernetes client-go](https://pkg.go.dev/k8s.io/client-go/tools/leaderelection) | н/в | 2026-09-19 | висока разом із [11] |
| [11] | застарілий lease і fencing token | [Martin Kleppmann, original analysis](https://martin.kleppmann.com/2016/02/08/how-to-do-distributed-locking.html) | 2016-02-08; історичне пояснення | 2026-09-19 | середня, підтверджено [10] |

## Мапа актуальності

Механічний `recon_kit.py staleness` для шести ключових claims: найраніша повторна перевірка — **2026-10-19** для обмежень platform background execution; усі інші ще в заданих вікнах. Для джерел без дати публікації використано 2026-09-19 як **дату доступу-проксі**, не вигадану дату публікації. Вікна: platform 1 місяць, privacy/coordination 12, architecture patterns 24. Перед рішенням про конкретні iOS/Android background SLA, push provider та third-party API потрібна нова перевірка відповідної актуальної документації.
