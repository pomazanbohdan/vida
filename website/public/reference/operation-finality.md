---
id: SPEC-OPERATION-FINALITY-001
status: approved
implementation_status: unplanned
last_updated: 2026-09-22
requirement_refs:
  - ../02-requirements/transport-sync-requirements.md
  - ../02-requirements/effect-execution-constraints.md
  - ../02-requirements/platform-nfr.md
decision_refs:
  - ../03-architecture/decisions/ADR-0005-iroh-transport-foundation.md
  - ../03-architecture/decisions/ADR-0006-durable-delivery-and-operation-envelope.md
  - ../03-architecture/decisions/ADR-0012-command-event-sync-boundary.md
  - ../03-architecture/decisions/ADR-0016-equal-device-peers.md
  - ../03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md
source_refs:
  - sync-log-contract.md
  - durable-delivery-contract.md
  - device-sync-session.md
  - operation-semantics-catalog.md
  - sync-presence-status-model.md
---

# SPEC-OPERATION-FINALITY-001: Operation evidence, receipts and bounded finality

## Purpose

Визначити один канонічний контракт, за яким VIDA відрізняє локальне збереження, реплікацію, доставку адресату, бізнесове рішення та виконання зовнішнього ефекту. Контракт закриває неоднозначність слова «готово»: Iroh/QUIC ACK, durable replica, прочитання повідомлення й підтверджене бронювання є різними доказами.

## Scope і non-goals

У scope входять operation/request identity, evidence axes, signed receipts, user-visible states, bounded finality, direct/mailbox paths і conformance fixtures.

Контракт не:

- робить Iroh transport source of truth;
- призначає один фізичний Device master/arbiter;
- використовує client clock або packet arrival як authority order;
- обіцяє глобальну незмінність Resource назавжди;
- визначає конкретний CRDT, database або serialization library.

## Terminology

- **Operation** — підписана ідемпотентна зміна зі stable `OperationId`.
- **Request** — Operation, для якої окремий named process/authority видає outcome; має stable `RequestId` і revision.
- **Frontier** — причинно замкнений набір прийнятих operations, представлений канонічним digest.
- **Replication Policy** — оголошений scope і кількість незалежних authorized durable replicas, потрібних для напису `Synchronized`.
- **Application Receipt** — підписаний VIDA-level доказ конкретного факту; transport ACK ним не є.
- **Resource Authority** — логічний actor/process, названий schema/Space policy для outcome конкретної operation family; co-location із server/mailbox не надає authority.
- **Bounded finality** — outcome незмінний для конкретного `RequestId + revision + authority frontier`; нова operation/revision може змінити Resource пізніше.

## Requirements

| ID | Requirement |
|---|---|
| REQ-FINALITY-001 | VIDA `MUST` зберігати п'ять evidence axes незалежно: local durability, replication, recipient delivery, domain authority і external effect. Один axis `MUST NOT` автоматично підвищувати інший. |
| REQ-FINALITY-002 | `Saved locally` `MUST` означати atomic commit `OperationEnvelope + SyncLog + outbox + required local bytes`; crash до commit повертає failure, після commit — відновлюваний pending intent. |
| REQ-FINALITY-003 | `Synchronized(scope)` `MUST` вимагати signed `ReplicationReceipt(state=applied_at_frontier)`, прив'язаний до `OperationId`/Frontier, від replica, яка durable-зберегла operation та застосувала всі causal dependencies до цього Frontier. Default policy — перша незалежна authorized durable application replica. Persona з одним Device без такої replica лишається `Saved locally`. `validated_persisted`, Iroh/QUIC ACK або mailbox ciphertext storage недостатні. |
| REQ-FINALITY-004 | `Delivered(recipient)` `MUST` означати, що хоча б один чинний recipient-controlled Device розшифрував, перевірив і durable-зберіг точну Operation та видав signed receipt. Sender outbox, push/QUIC ACK, relay або mailbox storage недостатні. |
| REQ-FINALITY-005 | `Read` `MUST` бути окремою Persona-level синхронізованою domain operation, створеною після фактичного показу/відкриття; per-device receive ACK не є shared read state. Sender-visible Read receipt `MUST` поважати privacy policy/opt-in. |
| REQ-FINALITY-006 | Operation family без окремого approval `MAY` отримати deterministic domain acceptance лише після перевірки signature, grant/control frontier, schema/policy version, preconditions і semantic class. |
| REQ-FINALITY-007 | Exclusive claim/approval `MUST` лишатися `domain.pending`, доки named Resource Authority не видасть signed `accepted|rejected|expired` Outcome для точного `RequestId + revision + frontier`. |
| REQ-FINALITY-008 | Serverless exclusive operation без досяжної serialized logical authority `MUST` лишатися pending; Device clock, arrival order або replica ID `MUST NOT` створювати winner. |
| REQ-FINALITY-009 | Порівнювані authority acceptances використовують causal order; несумісні непорівнювані authority-accepted status/file branches `MUST` створювати explicit Conflict без winner. |
| REQ-FINALITY-010 | External effect `MUST` мати stable idempotency key і окремий receipt/status. Якщо API не підтримує lookup або безпечний retry, невідомий результат `MUST` лишатися `effect.unknown`. |
| REQ-FINALITY-011 | Evidence receipt `MUST` бути ідемпотентним, перевіряти чинний protocol/version/signature domain і fail closed для невідомих mandatory fields або invalid bindings. |
| REQ-FINALITY-012 | Поява пізньої authority-accepted incomparable branch `MAY` повторно відкрити Conflict; UI `MUST NOT` обіцяти «остаточно для всіх назавжди». |

## Data and state model

### Незалежні evidence axes

| Axis | Canonical states | Monotonic evidence | User meaning |
|---|---|---|---|
| Local durability | `uncommitted`, `origin.committed` | atomic local commit | «Збережено локально» |
| Replication per replica | `unknown`, `validated_persisted`, `applied_at_frontier` | signed receipt від конкретної replica | «Синхронізовано» лише після виконання Replication Policy |
| Delivery per recipient | `none`, `stored_for_delivery`, `delivered`, `read` | mailbox receipt; recipient replica receipt; read operation | «Збережено для доставки», «Доставлено», «Прочитано» |
| Domain authority | `not_required`, `pending`, `accepted`, `rejected`, `expired`, `conflicted` | deterministic validation або signed Outcome | «Очікує підтвердження», «Прийнято», «Відхилено», «Строк минув», «Конфлікт» |
| External effect | `not_required`, `pending`, `running`, `succeeded`, `failed`, `unknown`, `correction_needed` | executor/API receipt або explicit uncertainty | окремий status наслідку |

`syncing`, `online`, `reconnecting`, «усі відомі peers наздогнані» та поточна projection не є monotonic evidence. Вони обчислюються з поточного connection/frontier view і можуть змінитися.

### Receipt envelope

Кожний Application Receipt `MUST` мати:

- `receipt_id`, `receipt_version`, `receipt_kind`;
- `operation_id` та, за наявності, `request_id` + `request_revision`;
- issuer Persona/Device/Service binding і signature domain;
- `space_id`, `app_instance_id`, optional `resource_id`/`process_id`;
- context-scoped `recipient_principal_id`, recipient Device/replica binding і immutable `audience_revision`, коли receipt стосується доставки;
- `control_epoch`, schema/semantic-class/policy version;
- causal frontier digest;
- evidence state/decision;
- operation payload digest або outcome/result digest;
- `issued_at` лише як audit metadata, не authority ordering;
- cryptographic signature.

Receipt payload `MUST` мінімізувати routing metadata й не розкривати Resource content сторонній mailbox/relay infrastructure.

## Interfaces and protocols

### `ReplicationReceipt`

Видається authorized replica після durable validation і persistence. State `validated_persisted` ще не виконує default Replication Policy. `Synchronized` вимагає state `applied_at_frontier`, який видається лише після apply усіх causal dependencies до названого Frontier.

### `DeliveryReceipt`

- `stored_for_delivery` може видати trusted mailbox після durable ciphertext storage;
- `delivered` видає чинна recipient-controlled replica після decrypt, validation і durable persistence;
- 1:1 sender projection стає `Delivered` після першого такого receipt;
- group projection агрегується per Persona в immutable audience revision як `Delivered N/M`; `Delivered to all recipients` означає щонайменше один qualifying receipt від кожного intended recipient, а не кожного Device;
- точна Device topology та `1/N Devices` не показуються sender-у; coverage усіх власних Devices є owner-only sync diagnostics;
- `read` є Persona-level domain operation адресата, а не transport receipt.

### `AuthorityOutcome`

Видається Resource Authority для `RequestId + revision`; decision: `accepted|rejected|expired`. Conflict є результатом reconciliation кількох належно accepted incomparable branches, а не довільним authority response.

### `EffectReceipt`

Прив'язується до stable effect/idempotency key та causal confirmation frontier. `unknown` забороняє сліпий automatic retry, якщо зовнішній контракт не доводить його безпечність.

## Lifecycle and failure behavior

### Звичайна offline зміна

1. Atomic commit → `origin.committed` / «Збережено локально».
2. Встановлення Iroh connection → `syncing`; QUIC ACK не змінює evidence axes.
3. Replica durable-зберігає operation, застосовує всі causal dependencies до Frontier → `ReplicationReceipt(state=applied_at_frontier)`.
4. Виконання Replication Policy → «Синхронізовано».
5. Якщо є адресат, його receipt окремо дає «Доставлено»; read operation — «Прочитано».

Історичний `Delivered(recipient)` не відкочується після revocation або видалення Device: доказ фактичної доставки лишається audit evidence. Новий Device не змінює старий message audience revision і не повертає message до pending; його backfill є окремим sync coverage.

### Request з authority

1. Candidate atomic commit → «Збережено локально» + «Очікує підтвердження».
2. Реплікація candidate може дати «Синхронізовано», але не `accepted`.
3. Authority перевіряє права, preconditions, semantic class і current frontier.
4. Signed Outcome → `accepted|rejected|expired` для точного Request revision.
5. Requester отримує Outcome як окремий обов'язковий sync fact.

### Direct і mailbox paths

Direct path може перейти з `origin.committed` прямо до recipient `validated_persisted/applied_at_frontier`; `stored_for_delivery` не є обов'язковим проміжним станом. Mailbox path додає незалежний `stored_for_delivery`, але не підвищує operation до `Synchronized` або `Delivered` без потрібного Application Receipt.

### Failure table

| Failure | Required state/result |
|---|---|
| Crash до origin commit | failure; committed Operation відсутня |
| Crash після origin commit | intent відновлюється з outbox |
| Missing causal dependency | replica зберігає pending; не видає `applied_at_frontier` |
| Stale/revoked grant або epoch | candidate rejected; delivery не перетворюється на acceptance |
| Stale precondition | authority видає `rejected`/`expired` за policy |
| Authority unavailable | `domain.pending`; terminal winner відсутній |
| Invalid/unknown mandatory receipt field | fail closed; evidence axis не підвищується |
| Duplicate direct/mailbox delivery | один idempotent apply і стабільний receipt |
| Incomparable accepted status/file branches | explicit Conflict без winner |
| External API response lost without lookup/idempotency | `effect.unknown`; manual resolution policy |

## Security and privacy

- Кожен receipt перевіряє issuer binding, current authorization/control proof, protocol version і signature.
- Receipt issuer не отримує authority лише через network/storage role.
- Receipt aggregation не рахує кілька Devices однієї Persona як кілька approval votes.
- Sender-facing delivery aggregation відбувається за logical recipient Persona й audience revision, без розкриття Device IDs/count; exact coverage бачить лише власник Persona або окремо уповноважений diagnostics actor.
- Public presence/online count не розкриває raw Device identities без окремого права.
- Receipt metadata не використовується для прихованої cross-Persona correlation.

## Compatibility and migration

- Receipt/envelope semantics versionуються разом із VIDA ALPN/application contract.
- Невідомий optional field ігнорується лише коли його omission не змінює proof semantics.
- Невідомий mandatory feature/version fail closed і не підвищує evidence state.
- Breaking change потребує нового protocol major або explicit migration.
- Старі лінійні `delivery.*` записи мігруються в незалежні axes без вигаданих proofs; відсутній proof стає `unknown`, а не accepted/delivered.

## Observability and operations

Локальні privacy-preserving metrics `MAY` рахувати latency і failure class для origin commit, replication receipt, recipient delivery, authority outcome й effect. Логи `MUST` містити correlation IDs/digests, але не plaintext Resource content або key material.

## Acceptance criteria

- Iroh/QUIC completion без Application Receipt не дає «Синхронізовано».
- Mailbox ciphertext receipt дає лише «Збережено для доставки».
- Persona з одним Device і без незалежної authorized durable replica лишається «Збережено локально».
- Replication Receipt першої незалежної authorized durable application replica виконує default Replication Policy, але не створює другого approval vote.
- Recipient-controlled durable receipt дає Delivered тільки відповідному recipient; sender не бачить exact Device count.
- У групі `N/M` рахує Personas з qualifying receipt у bound audience revision; plain «Доставлено всім» з'являється лише за `N=M`.
- Додання/видалення Device не скасовує історичний Delivered і не змінює audience revision старого Message.
- Request може бути Synchronized і водночас domain.pending.
- Signed AuthorityOutcome повторно застосовується ідемпотентно.
- Late incomparable accepted branch повторно відкриває Conflict.
- Terminal outcome завжди називає Request revision/frontier; нова revision є новим request lifecycle.

## Test and conformance plan

Машиночитані golden vectors `MUST` покривати:

1. crash до/після origin commit;
2. QUIC ACK без VIDA receipt;
3. mailbox stored без recipient apply;
4. direct + mailbox duplicate/replay;
5. single-Device Persona лишається locally saved;
6. first independent replica → synchronized без business quorum;
7. 1:1 first-recipient-Device delivery без Device-count disclosure;
8. group per-Persona `N/M`, all-recipients та read opt-in;
9. Device add/remove/revoke не відкочує historical delivery;
10. causal first acceptance та incomparable Conflict permutations;
11. stale/revoked grant і epoch;
12. outcome replay, expiry та authority unavailable;
13. restart/rebuild із receipt log;
14. invalid signature/version/mandatory field;
15. late unknown branch і correction-needed effect;
16. external API `effect.unknown` без lookup/idempotency.

Rust Core, Flutter adapters та незалежна reference implementation `MUST` отримувати однакові canonical outcomes для тих самих vectors.

## Approved product decisions

1. **Replication Policy default:** «Синхронізовано» з'являється після першої незалежної authorized durable application replica. Single-Device Persona без replica лишається «Збережено локально».
2. **Delivered aggregation:** 1:1 `Delivered` означає перший qualifying recipient Device receipt. Group UI показує `N/M logical recipients`; sender не бачить exact Device topology. «На всіх моїх пристроях» є owner-only diagnostics, не delivery semantics.
3. **Mailbox wording:** trusted mailbox receipt означає лише «Збережено для доставки»; він не є `Synchronized` або `Delivered`. Managed application replica може виконати Replication Policy лише після decrypt/validate/durable-apply та власного Application Receipt.
4. **Read:** перший фактичний показ на будь-якому Device створює Persona-level read operation; sender-visible receipt залежить від privacy opt-in.

Ці рішення узгоджені з [Signal delivery/read semantics](https://support.signal.org/hc/en-us/articles/360007320751-How-do-I-know-if-my-message-was-delivered-or-read), [WhatsApp multi-device delivery](https://faq.whatsapp.com/665923838265756/), [Matrix user-scoped receipts](https://spec.matrix.org/latest/client-server-api/#mreceipt), [MLS Delivery Service boundary](https://www.rfc-editor.org/rfc/rfc9750.html#section-5.2) та [OWASP MASVS-PRIVACY-1](https://mas.owasp.org/MASVS/controls/MASVS-PRIVACY-1/). Вони є референсами межі доказів, а не протоколами VIDA.

## Remaining implementation work

- Зафіксувати canonical binary serialization/signature suite у protocol-major architecture contour.
- Підтримувати й публікувати затверджені machine-readable semantic fixtures `fixtures/operation-finality-v1.yaml` разом зі змінами protocol-major.
- Прогнати однакові vectors через Rust Core, Flutter adapters та незалежну reference implementation.
