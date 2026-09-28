---
id: ADR-0012
status: accepted
last_updated: 2026-09-19
source_refs:
  - ../../../_bmad-output/planning-artifacts/research/technical-vida-application-logic-triggers-and-exec-2026-09-19/research.md
  - ./ADR-0006-durable-delivery-and-operation-envelope.md
  - ./ADR-0011-instance-trigger-precedence.md
decision_refs:
  - ADR-0006
  - ADR-0011
supersedes: []
superseded_by: []
---

# ADR-0012: Межа між бізнес-командою, фактом і синхронізацією

## Context

Один `onSave` не відрізняє новий намір користувача від отримання тієї самої операції на іншому пристрої, повторної доставки, replay або оновлення UI. Без цієї межі Messenger, Notes, Projects і майбутні інтеграції можуть повторно виконати бізнес-дію під час синхронізації.

## Decision

VIDA `MUST` розділяти точки входу за семантикою:

- **Named business command** — новий намір з actor, input і результатом; приклад: `notes.update`, `message.send`, `booking.request`. Обробник `AppInstance` може ухвалювати прикладне рішення, але trusted core повторно перевіряє доступ та інваріанти перед commit.
- **Committed domain fact** — наслідок операції, прийнятої за policy відповідного Space/об'єкта; локально збережений pending intent або `delivery.accepted` сам по собі не є таким фактом. Реакції не змінюють минулий факт. Потенційний зовнішній ефект виконується окремим контрольованим шляхом, а не кожним одержувачем операції.
- **Timer / external signal** — окремі входи, які можуть ініціювати команду або продовжити процес тільки за визначеним контрактом повноважень і повторів.
- **UI projection / observer** — відображає локальні та віддалені зміни, не створюючи бізнес-намір самостійно.

`sync.apply` тут є пояснювальною назвою для idempotent receive/apply, **не затвердженим API**. Після перевірки операції, dedupe і causal dependencies він застосовує вже створену операцію до локального стану та оновлює derived projections. Отримання, reconnect, replay або UI rerender `MUST NOT` повторно викликати початкову бізнес-команду чи автоматично створювати нову. `delivery.applied` не дорівнює `authority.accepted`.

## Consequences

- Пріоритет handler-ів з `ADR-0011` застосовується до визначеної бізнес-точки входу, а не до кожного sync replay.
- Якщо за committed fact потрібен зовнішній ефект, його виконавець, durable handoff, retries та idempotency потребують окремого контракту; це рішення не призначає authority.
- Точні назви API/trigger-ів, набір v1, порядок фаз, base-handler continuation та error semantics лишаються відкритими (`OQ-0045`).
- Domain authority і deterministic transitions лишаються відкритими (`OQ-0033`, `OQ-0034`).

## Acceptance evidence

1. `message.send` на пристрої A дає один логічний факт; дубль доставки й reconnect пристрою B оновлюють його локальне відображення без нового `message.send`.
2. Повторний replay того самого `notes.update` не викликає повторно instance command handler, але rebuild відтворює ту саму projection.
3. Відхилена або causally pending операція не відображається як повністю applied і не запускає зовнішній бізнес-ефект.

## Open questions

- Хто саме виконує post-commit reactions у автономному personal і federated Space та як переживає дублікати й offline?
- Які конкретні trigger/phase API та гарантії v1 потрібні для команд, фактів, таймерів і зовнішніх сигналів?
