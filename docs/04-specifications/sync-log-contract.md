---
id: SPEC-SYNC-LOG-001
status: approved
implementation_status: unplanned
last_updated: 2026-09-22
requirement_refs:
  - ../02-requirements/transport-sync-requirements.md
  - ../02-requirements/platform-nfr.md
  - ../02-requirements/access-control-requirements.md
decision_refs:
  - ../03-architecture/decisions/ADR-0006-durable-delivery-and-operation-envelope.md
  - ../03-architecture/decisions/ADR-0012-command-event-sync-boundary.md
  - ../03-architecture/decisions/ADR-0003-offline-revocation.md
  - ../03-architecture/decisions/ADR-0016-equal-device-peers.md
---

# SPEC-SYNC-LOG-001: SyncLog contract

## Purpose

Визначити transport-independent validated history та repair boundary для durable mutations.

## Operation invariants

Every durable operation `MUST` contain stable operation ID, author/device binding reference, Space/context reference, causal metadata, schema/protocol feature set, encrypted or signed payload binding and signature/authentication evidence.

Encoding та algorithms are deferred; semantic fields are mandatory.

## Interfaces

```text
Validate(operation, context) -> Valid | Rejected
Accept(operation) -> AlreadyPresent | Accepted
Missing(summary) -> OperationRef[]
Fetch(refs) -> Operation[]
Snapshot(scope, frontier) -> SnapshotRef
Rebuild(scope) -> Projection
```

`Accept` `MUST` be idempotent. Operations with missing causal dependencies remain pending and `MUST NOT` be projected as fully applied.

`Accept` у цьому абстрактному інтерфейсі означає idempotent прийняття операції в локальну validated/pending history, **не** автоматичне `authority.accepted`. Origin-durable, replica-applied і authority-accepted frontiers `MUST` розрізнятися за [SPEC-OPERATION-FINALITY-001](operation-finality-contract.md). Tentative local projections `MUST NOT` видаватися за прийнятий domain state.

Старий offline candidate `MUST NOT` видалятися або відхилятися лише за віком. При authority acceptance чинні права для Space/об'єкта/дії, grant/epoch і domain/causal preconditions перевіряються заново; invalid або revoked candidate зберігається recoverable і не project-иться як accepted. Personal Space Owner може приймати за власною authority policy без зовнішнього погодження. Коли старий envelope не проходить поточну перевірку, спосіб re-sign/rebase без дублювання operation ID лишається `OQ-0033`/`OQ-0034`, а не автоматичне прийняття.

Receiving or replaying an already-created operation `MUST NOT` reissue its originating business command. Validated receive/apply may advance local derived state and notify UI observers; any reaction with external effects requires a separately specified executor and idempotency contract. The shorthand `sync.apply` does not name an approved public API.

## Authority and projections

Signed operations validated under the selected Space authority policy and explicitly defined snapshots are authoritative. Search indexes, unread counts, timelines and SQL/server projections are derived and MUST be rebuildable or verifiably reconciled.

Для кожного resource type / operation family schema contract `MUST` явно ідентифікувати resolver застосовної authority policy. Власна нотатка у Personal Space може отримати локальну перевірку Owner-Persona; будь-який її авторизований пристрій рівноправний як репліка. Shared задача перевіряється за Space/object policy; запис до зовнішнього календаря — власником відповідного ресурсу/сервісу. Ці приклади не визначають універсальний сервер або головний пристрій. Саме co-location із федеративним вузлом, storage або transport не надає authority. Receipt і bounded-finality proof визначає `SPEC-OPERATION-FINALITY-001`; serverless authority topology/equivalence лишаються `OQ-0033`/`OQ-0034`.

## Compatibility and migration

Breaking semantic changes require new ALPN major or explicit migration protocol. Unknown mandatory feature fails. Persisted-state migrations and backup restore are part of release conformance.

## Acceptance criteria

- duplicate/order/partition property tests converge;
- missing dependency repair succeeds after reconnect;
- projection rebuild produces the same verified state;
- mixed-version test matrix detects incompatible mandatory features;
- corrupted operation or snapshot fails closed with diagnostics.
- duplicate receive/replay updates the recipient projection without a second originating command or external business effect.
- pending або відхилена authority операція не запускає похідне створення accepted shared resource, навіть якщо вона origin-durable чи вже доставлена.
- тестовий offline candidate після довгої відсутності мережі не відхиляється лише за віком; прийняття після reconnect залежить від чинного права та інших preconditions, а revoked candidate не стає accepted навіть із client timestamp до revocation.
- два offline task-status candidates від того самого base не можуть обидва непомітно стати остаточними: за порівнюваного acceptance перший задає спільний стан; за непорівнюваної несумісної concurrent history після reconciliation два **authority-accepted** variants зберігаються як явний невирішений multi-value conflict без winner. Лише eligible, але ще не accepted candidate не є спільним variant. Відкликаний/невалідний pending candidate не стає спільною гілкою лише через доставку. Уповноважений актор може подати рішення як нову операцію, що причинно враховує обидва variants. Fixture перевіряє перестановки доставки, однаковий conflict на peers, відмінність `delivery.accepted` від domain acceptance, збереження variants і повторну перевірку прав.
- для двох офлайн-редагувань незалежних частин тексту або коду implementation не замінює весь файл лише через різні версії; для однієї несумісно зміненої частини зберігає обидва варіанти для авторизованого порівняння. Для фрагмента нотатки перше порівнювано прийняте редагування є поточним, а пізніший несумісний намір лишається доступним автору для прийняття поточного або нової зміни від нього; непорівнювані authority-accepted редагування дають явний Conflict без winner. Гранулярність, proof/wire і merge algorithm — `OQ-0033`/`OQ-0034`.
- дві непорівнювані несумісні заміни файла дають той самий явний unresolved conflict з обома збереженими blob variants без автоматично поточного файла; після рішення нова операція спирається на обидві гілки. Тест відкликає право актора перед рішенням і перевіряє відмову; для двох несумісних правомірно прийнятих непорівнюваних рішень діє новий явний conflict за `REQ-SYNC-009`, а для доведено еквівалентних — один видимий результат з обома audit operations за `REQ-SYNC-010`. Точний proof/equivalence predicate лишається `OQ-0033`/`OQ-0034`.
- два авторизовані рішення status/file conflict з несумісними результатами, прийняті офлайн без спільного перевірного порядку, після reconciliation утворюють новий явний conflict без winner; permutation tests перевіряють збіжність peers, збереження двох resolution heads/payloads та нову операцію, що причинно враховує обидва heads. Порівнювані прийняття й невалідні candidates перевіряються окремо.
- два доведено еквівалентні рішення status/file conflict дають один видимий результат, але обидві signed operations зберігаються для аудиту та не повторюють зовнішній наслідок; якщо канонічний результат або наслідки різні чи невідомі, coalescing не застосовується.
- третя належно прийнята несумісна гілка, відсутня в causal base ранішого рішення **і з непорівнюваним щодо нього authority acceptance**, повторно відкриває conflict без winner. За порівнюваного прийняття діє правило першого acceptance. Тест перевіряє пізній порядок доставки, crash/restart і збереження payloads; invalid/revoked candidate не відкриває прийнятий conflict.
- resolver без чинного read-доступу хоча б до одного потрібного variant не бачить закритого payload і не може подати accepted resolution, навіть якщо має право update на ресурс; той самий control gate застосовується до agent opt-in.
- перша порівнювано прийнята заміна одного неавтомержного значення/версії лишається поточною; непорівнювані concurrent variants зберігаються до однакового merge/вибору. Для task status і file replacement діє явний conflict без winner з [ADR-0017](../03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md); для несумісних правок одного фрагмента нотатки діє `REQ-SYNC-006`; для решти operation families правило ще відкрите. Уповноважений актор може прийняти чинне за порівнюваного порядку, подати нове рішення від обох конфліктних гілок або зберегти окрему копію/ревізію, якщо це підтримує ресурс. Дубль ACK/transfer не змінює outcome.
- жодна replica тієї самої Persona не має пріоритету від типу пристрою, endpoint або порядку з'єднання; property tests переставляють delivery order та місцями платформи й перевіряють однаковий outcome для того самого набору валідних operations.
- пряма сесія проходить контрольне узгодження й перевірку доступу перед передачею захищених даних; історія causal dependencies, локальний ACK та доменне прийняття відрізняються згідно з [device-sync-session.md](device-sync-session.md). Це не призначає serverless device-арбітра; status/file conflict outcome прийнятий, а proof та exact concurrent acceptance mechanics лишаються `OQ-0033`/`OQ-0034`.

## Deferred

Canonical encoding, hash/signature algorithms, snapshot format, pruning/checkpoint policy and workload-specific CRDT use.

Independent `SyncLog` implementations use `SPEC-OPERATION-FINALITY-001` receipt fixtures, але лишаються blocked до визначення в `OQ-0033`/`OQ-0034` serverless authority topology, equivalence predicate, deterministic operation-family transitions, frontier/snapshot semantics і pruning rules зі shared convergence fixtures.
