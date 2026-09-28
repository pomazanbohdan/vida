---
title: 'Technical research: VIDA application logic triggers and execution phases'
type: technical
topic: 'VIDA application logic triggers and execution phases'
decision: 'Candidate trigger and execution-phase contract for VIDA AppInstance logic, pending user discussion'
source: 'official primary documentation and project decision context'
status: complete
preset: focused
validation: normal
created: '2026-09-19'
updated: '2026-09-19'
verified_claims: 3
unverified_claims: 12
---

# VIDA: що запускає прикладну логіку

**Мета:** визначити можливі точки входу, фази й композицію логіки в `AppInstance` перед рішенням щодо `OQ-0045`. Це дослідження для обговорення, не затверджена специфікація або вибір Rhai/Wasmtime.

## Короткий висновок

Один універсальний `onSave` приховує різні явища: користувацьку команду, зміну запису, перехід стану, таймер і зовнішній сигнал. Airtable розділяє ці тригери; «оновлено» може спрацювати ще до завершення редагування, а перехід до умови повторюється після виходу й нового входу в стан. [1][2] Appwrite відділяє синхронний прямий виклик від асинхронної події/розкладу й попереджає про самозапуск циклу. [3]

**Пропозиція VIDA:** named business command → коротке прикладне рішення/валідація → trusted core commit → зафіксований доменний факт → окрема реакція чи durable workflow. Синхронізація доставляє стан/операції, але сама по собі не створює нового бізнес-наміру. Це висновок з local-first/replay і retry-джерел, не готова гарантія жодної бібліотеки. [10][11][12]

## 1. Які події можуть викликати рушій

| Кандидат точки входу | Приклад VIDA | Семантика |
|---|---|---|
| Названа команда | `notes.update`, `task.assign`, `message.send`, `booking.request` з UI, бота, API | Намір з actor/input/результатом; може бути відхилений до commit. |
| Зафіксований факт | `note.updated`, `task.assigned`, `booking.confirmed` | Після прийнятої операції; реакція не може змінити минулий факт. |
| Перехід стану | задача стала `ready` | Важливий саме перехід, не кожне збереження; можливий повторний вхід у стан. |
| Таймер | нагадування, прострочення, timeout | Потрібний відповідальний виконавець; це не запис користувача. |
| Зовнішній сигнал | відповідь City Portal, webhook | Автентичність, dedupe, перетворення на дозволену команду/сигнал. |
| Lifecycle пакета | активація, міграція, деактивація | Адміністративний протокол, не масове `created` для старих ресурсів. |
| UI projection/observer | оновлення форми, локального preview | Повторюваний наслідок local/remote змін, не право надсилати платіж. |

У Airtable є create/update/condition/form/schedule/webhook/button; «record updated» не тотожний «created». [1][2] У local-first Automerge зміни синхронізуються після reconnect і observer може бачити віддалені зміни. [10] Replicache показує локальне optimistic виконання, серверне застосування та повторний rebase; subscription реагує на локальні й sync-зміни. Це історичний pattern, не рекомендація залежності Replicache. [11] Отже, `sync.apply`, rerender та повторний показ стану **не повинні автоматично** запускати нове `booking.request` чи повторну відправку повідомлення (архітектурний висновок).

## 2. Де виконувати рішення та ефекти

| Фаза-кандидат | Роль | Помилка/ефект |
|---|---|---|
| `command.resolve` | вибрати названу base/instance реалізацію | typed unsupported/version error; без обходу core |
| `command.decide` | коротке обчислення запропонованих змін без прямого I/O | reject або typed proposed operations |
| `core.commit` | trusted grants, constraints, запис, журнал, подія | єдине authoritative прийняття/відмова |
| `event.react` | індексація, повідомлення, зовнішній виклик, старт процесу | durable handoff, retry/dedupe, окремий failure |
| `workflow.resume` | timer, відповідь людини чи сервісу | версія процесу та authority/lease |

Це **пропозиція VIDA**, а не копія DB trigger. PostgreSQL `AFTER` виконується все ще у тій самій транзакції, тому не є автоматично «після commit». [4] Для зовнішньої реакції AWS описує outbox: зміна даних і pending event записуються атомарно; доставка після commit може повторитися, отже потрібна idempotency. [5] Supabase демонструє асинхронні webhooks після INSERT/UPDATE/DELETE. [15]

Temporal документує durable event history і повтор Activity після втрати роботи/timeout; журнал сам не гарантує, що платіж у зовнішній системі відбудеться лише раз. [12] Тому кандидати поля envelope — `eventId`, `AppInstanceId`, версія package/rule, origin/actor, causal ID, idempotency key; точну схему та виконавця ще не обрано.

Rhai має ліміт операцій, але один виклик host-функції може бути дорогим; Wasmtime обмежує guest compute через fuel/epoch. Обидва факти не замінюють host permissions, budget I/O й контрольованого commit. [8][9]

## 3. Чому «пріоритет» не визначає долю базової дії

| Референс | Модель | Урок |
|---|---|---|
| WordPress | filter повертає змінений результат; action виконує побічну дію; порядок callbacks має priority. [6] | Рішення й реакція — різні typed slots. |
| Lexical | вищий handler отримує command першим, `handled` зупиняє подальшу передачу. [13] | Пріоритет + явна зупинка, а не завжди обов'язковий base. |
| Tiptap | override може явно викликати `parent()`, щоб зберегти частину базової поведінки. [14] | Явна композиція base можлива. |
| Shopify Functions | outputs незалежних functions комбінуються host-правилами; наведена API-сторінка `unstable`. [7] | Злиття доречне лише для typed результатів з формальними правилами. |

`ADR-0011` уже встановив пріоритет процесу `AppInstance`, але не правило продовження. Референси [6][13][14] показують різні контракти. Пропоную розрізнити в manifest: `replace named command`, `compose base command` і `subscribe to committed event`. Автор складає власні handler-и в instance; платформний Owner-level арбітр бізнес-конфліктів не потрібен. Це ще не затвердження default continuation.

## Сценарії, на яких перевірити контракт

- **Notes:** `notes.update` → trusted запис → `note.updated` → індекс/notification. Autosave поля не тотожний «документ готовий»; останнє потребує окремої команди/переходу. [1][2][5]
- **Messenger:** `message.send` записує повідомлення й pending delivery; повторне отримання через sync оновлює локальний стан, не надсилає ще раз. [5][10][11]
- **Projects:** `task.assign` породжує факт для оповіщення; due-date timer і approval — інші точки входу, останній може бути довгим процесом. [1][3][12]
- **City Portal booking:** offline request може бути pending, але підтвердження слота належить authority; дубль доставки не повинен означати друге бронювання. Це **припущення для перевірки інтеграційного контракту**, не зовнішньо доведена властивість VIDA. [5][11][12]

## Рекомендації до обговорення, не ADR

1. Узгодити три площини: named command/decision, committed-event reaction, durable timer/external signal; UI projection без бізнес-ефектів окремо. Висока впевненість у потребі розділення з незалежних референсів; назви VIDA — лише кандидати. [1][3][4][5]
2. Не трактувати reconnect, `sync.apply`, optimistic rebase і UI rerender як новий business intent. Якщо реакція на віддалений факт потрібна, обрати одного виконавця й idempotency; це висновок середньої впевненості. [10][11][12]
3. Уточнити override named command окремо від additive post-commit reaction; explicit base continuation, error behavior і nested-call guard лишити відкритими до вашого рішення. [6][13][14]
4. Прототип: `Notes.update` + реакція + два офлайн-пристрої/reconnect + заборонений effect + package upgrade. Вибір Rhai/Wasmtime — після перевірки contract і budget. [5][8][9][11]

## Заперечення та невизначеність

Універсальна підписка на зміни простіша для автора, але надто широкі update/condition triggers спричиняють ранні спрацювання, повторні входи й self-trigger loops. [1][2][3] Явний `base()` додає роботу автору; альтернатива Lexical — `handled/continue`, Tiptap — `parent()`. [13][14] Жоден референс не ухвалює рішення за VIDA. Shopify API [7] нестабільний і наведений лише як приклад іншої композиції. WordPress handbook [6] старший за дворічне вікно pattern pack; поточний висновок перевірено свіжими Lexical/Tiptap docs.

## Питання для затвердження

1. Чи є named command, committed event, timer/external signal потрібними у першому package contract? Чи має `sync.apply` коли-небудь створювати новий бізнес-наміру на *кожному* вузлі?
2. Хто authority для ефектів у personal autonomous та federated Space і як dedupe переживає offline/reconnect?
3. При override базовий handler пропускається, викликається явно, чи продовжується за результатом `continue`? Які правила failure й recursion?
4. Де межа локальних UI reactions і переносної logic package; який versioned ABI/budget?

## Джерела

| № | Що підтримує | Видавець і посилання | Дата публікації | Доступ | Впевненість |
|---|---|---|---|---|---|
| [1] | типи triggers, ранні edits | [Airtable Help](https://support.airtable.com/articles/3669392397-getting-started-with-airtable-automations) | 2026-08-12 | 2026-09-19 | medium |
| [2] | state transition/re-entry | [Airtable Help](https://support.airtable.com/articles/7369519794-airtable-automation-trigger-when-record-matches-conditions) | 2026-09-18 | 2026-09-19 | medium |
| [3] | direct sync/event async, recursion | [Appwrite Docs](https://appwrite.io/docs/products/functions/execute) | не вказано | 2026-09-19 | medium |
| [4] | DB trigger timing, transaction | [PostgreSQL Docs](https://www.postgresql.org/docs/18/trigger-definition.html) | не вказано | 2026-09-19 | medium |
| [5] | outbox, duplicates, rollback | [AWS Prescriptive Guidance](https://docs.aws.amazon.com/prescriptive-guidance/latest/cloud-design-patterns/transactional-outbox.html) | не вказано | 2026-09-19 | high; cross-check [12] |
| [6] | filter/action semantics, priority | [WordPress Hooks](https://developer.wordpress.org/plugins/hooks/) / [priority API](https://developer.wordpress.org/reference/functions/add_filter/) | handbook 2024-01-29; API не вказано | 2026-09-19 | medium; historical |
| [7] | concurrent outputs/combination | [Shopify Dev](https://shopify.dev/docs/api/functions/unstable/discount) | не вказано | 2026-09-19 | low; unstable |
| [8] | guest interruption | [Wasmtime Docs](https://docs.wasmtime.dev/examples-interrupting-wasm.html) | не вказано | 2026-09-19 | medium |
| [9] | Rhai operations vs host work | [Rhai Book](https://rhai.rs/book/safety/max-operations.html) | не вказано | 2026-09-19 | medium |
| [10] | offline/remote sync, observer | [Automerge network](https://automerge.org/docs/tutorial/network-sync/) / [local observer](https://automerge.org/docs/tutorial/local-sync/) | не вказано | 2026-09-19 | high; cross-check [11] |
| [11] | optimistic rebase/subscriptions | [Replicache Docs](https://doc.replicache.dev/concepts/how-it-works) | не вказано | 2026-09-19 | medium; historical |
| [12] | durable history/Activity retries | [Temporal Docs source](https://github.com/temporalio/documentation/blob/main/docs/encyclopedia/workflow/workflow-execution/event.mdx) | не вказано | 2026-09-19 | high; cross-check [5] |
| [13] | priority + handled/continue | [Lexical Docs](https://lexical.dev/docs/concepts/commands) | не вказано | 2026-09-19 | high; compare [14] |
| [14] | override with optional parent | [Tiptap Docs](https://tiptap.dev/docs/editor/extensions/custom-extensions/extend-existing) | не вказано | 2026-09-19 | medium |
| [15] | async DB webhook after row change | [Supabase Docs](https://supabase.com/docs/guides/database/webhooks) | не вказано | 2026-09-19 | medium |

## Актуальність

Для датованих джерел використовується дата публікації; для недатованих — дата перевірки **лише як proxy планового перегляду, не як вигадана дата публікації**. Механічний розрахунок із `claims.json`: WordPress handbook [6] має recheck 2026-01-29 і є за межами дворічного вікна; він лишається історичним прикладом. `Unstable` Shopify [7], executor API [8][9] мають наступний перегляд 2026-10-19. Решта pattern/local-first/workflow джерел — 2028-08-12 або пізніше за 24-місячним вікном, але version/compatibility факти вимагають свіжішої перевірки перед реалізацією. Із 15 source-bound claims три критичні узагальнення перевірено незалежними видавцями; 12 vendor-specific фактів мають по одному першоджерелу та позначені `unverified` у ledger, не як спростовані.
