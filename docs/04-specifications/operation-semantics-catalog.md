---
id: SPEC-OPERATION-SEMANTICS-DRAFT
status: draft
implementation_status: partially-approved
last_updated: 2026-09-22
open_question_refs:
  - ../00-governance/open-questions.md#oq-0069
---

# Чернетка: загальна система семантик операцій VIDA

## Принцип

Resolver є алгоритмом VIDA Core, а не кодом AppPackage і не довільним налаштуванням Owner/Admin. Package оголошує semantic class із закритого versioned registry та бізнесові параметри, дозволені цією class. Core перевіряє preconditions, права, причинність і conformance fixtures. Змінити class для наявних даних можна лише через версійну migration, не перемикачем адміністратора.

## Погоджений поперечний контракт підтвердження

Canonical evidence axes, receipt envelope, bounded finality й sender-facing delivery semantics визначає [SPEC-OPERATION-FINALITY-001](operation-finality-contract.md). Цей каталог визначає domain semantic classes і не перевизначає transport/replication evidence.

`ConfirmationPolicy` є системною конфігурацією для конкретного названого процесу/operation family всередині `AppInstance`. AppPackage/schema постачає **рекомендований оптимальний default**, але він не є незмінним мінімальним рівнем approval. Space Admin обирає ефективний Core-supported режим окремо для кожного процесу. Це не довільний resolver із пакета і не ознака транспортної доставки.

1. Користувацька дія спочатку створює durable candidate зі stable operation ID. У режимі offline UI може чесно показати «збережено локально», але не «підтверджено».
2. Після sync Core перевіряє чинні права, preconditions, semantic class та названу resource authority.
3. Space Admin може залишити рекомендований default або вибрати `auto-approve`, `single-approver`, `sequential-stages` чи `threshold M-of-N` у межах Core catalog. `all-of` є `M=N`. Auto-approve означає автоматичне рішення після всіх Core-перевірок, а не обхід перевірок.
4. Результат фіксується окремою підписаною outcome operation `accepted|rejected|expired`, причинно пов'язаною з candidate.
5. Отримання outcome заявником є окремим обов'язковим sync-фактом. Відсутність receipt означає `confirmation pending`, навіть якщо transport packet був доставлений; повторна доставка outcome ідемпотентна.

`exclusive-claim` використовує цей контракт і додає умову дефіцитності: authority може підтвердити лише один сумісний claim на той самий ресурс/version frontier. Вимкнення ручного approval для бронювання означає автоматичне authority-рішення, але не вимикає серіалізацію слота. Це Core-інваріант semantic class, а не «мінімальний approval». Звичайне `approval` використовує той самий контракт без обов'язкової конкуренції за єдиний ресурс.

## Запропонований закритий каталог

| Class | Бізнес-приклад | Concurrent behavior | Що конфігурується |
|---|---|---|---|
| `append-only` | повідомлення, коментар, audit event | усі валідні записи зберігаються; причинний порядок + canonical tie-break | retention, visibility; не winner algorithm |
| `mergeable` | документ, нотатка, список | незалежні зміни auto-merge; overlap зберігає variants і відкриває conflict | merge granularity із Core-supported набору |
| `mutable-state` | статус задачі, assignee, title | causal later update застосовується; несумісні incomparable heads дають explicit conflict | state machine transitions |
| `exclusive-claim` | слот 14:00, username, остання одиниця товару | лише названий resource authority може зробити terminal accept; stale precondition відхиляється | authority identity, expiry, allowed transitions |
| `approval` | заявка, яку мають схвалити 2 з 3 менеджерів | кожен vote append-only; Core обчислює threshold/sequential/all-of outcome | approver set, threshold/stages у межах class |
| `governance` | додати/видалити Owner, revoke membership, key epoch | фіксовані security invariants і control-log ordering; package не перевизначає | лише дозволені policy parameters |

## Бізнес-пояснення exclusive claim

Клієнт офлайн може створити **запит** на слот 14:00, але не може чесно показати остаточне бронювання, бо не знає, чи слот уже зайняв інший запит. Запит має стани `saved locally` → `syncing` → `pending authority` → `accepted|rejected|expired`. Остаточний стан видає власник ресурсу: наприклад, authoritative calendar service або визначена authority Persona/Space process.

Якщо два менеджери працювали на старих офлайн-копіях, їхні `accept` не стають двома істинами. Authority приймає перший outcome, precondition якого відповідає current resource version; другий отримує `stale/rejected` і може створити нову звичайну operation. Якщо ж немає жодної досяжної **серіалізованої** authority, негайна гарантія «один слот — один запис» неможлива: VIDA тримає запит provisional, а не вигадує winner із device clock чи network arrival. Як resource authority доводить єдиний current version у serverless Space, лишається механічною частиною `OQ-0033`; каталог не маскує цю проблему.

Це не робить пристрої користувача master/secondary. Рівність peers зберігається для replication; authority належить operation/resource contract, а не фізичному пристрою.

## Бізнес-пояснення approval

`approval` потрібен не для кожної зміни даних, а коли бізнес-процес вимагає окремого рішення уповноваженої сторони після подання запиту.

- **Відпустка:** працівник зберігає заяву; один керівник підтверджує або відхиляє її. Це `single-authority`.
- **Закупівля:** заявка до визначеної суми підтверджується керівником; понад ліміт додатково потрібне підтвердження фінансів. Це `sequential stages`, наприклад `manager → finance`.
- **Публікація документа:** редактор і юрист обидва мають підтвердити одну незмінну ревізію. Це `all-of`; зміна документа після голосування робить попередні підтвердження непридатними до нової ревізії.
- **Комісія:** із трьох визначених членів достатньо двох. Це `threshold M-of-N`; кожна Persona голосує один раз для конкретної ревізії, а Core рахує результат.
- **Бронювання з ручною модерацією:** календар спочатку перевіряє, що слот ще вільний, а менеджер потім підтверджує клієнта. Тут `exclusive-claim` і `approval` є двома послідовними умовами одного workflow.

Owner-removal, key revocation та інші захисні правила Space не перетворюються на довільний approval workflow: це `governance`, і AppPackage не може їх послабити або перевизначити.

## Референси

- [RFC 9110 `If-Match`](https://www.rfc-editor.org/rfc/rfc9110.html#name-if-match) запобігає lost update через precondition і відхиляє stale write.
- [PostgreSQL Serializable](https://www.postgresql.org/docs/17/transaction-iso.html#XACT-SERIALIZABLE) допускає abort/retry однієї concurrent transaction замість двох суперечливих commits.
- [Automerge conflicts](https://automerge.org/docs/reference/documents/conflicts/) зберігає concurrent values, але його deterministic visible winner не є доказом бізнесового права або exclusive reservation; VIDA не використовує library winner як domain decision.

## Approved baseline і remaining mechanics

Generic confirmation lifecycle, окремий synchronized outcome receipt, per-process Admin policy та незмінність Core-governance погоджені. V1 підтримує `single-approver`, `sequential-stages` і `threshold M-of-N`; `all-of` кодується як `M=N`. Відкритими лишаються authority topology/current frontier для serverless exclusive claims і точні resolvers інших operation families, не OQ-1 receipt semantics.
