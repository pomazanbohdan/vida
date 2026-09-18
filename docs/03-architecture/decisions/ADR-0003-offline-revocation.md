---
id: ADR-0003
status: accepted
last_updated: 2026-09-18
source_refs:
  - ./ADR-0001-layered-access-control.md
  - ../../../research/vida-current-architecture.md
  - ../../../research/Новий Text Document (10).txt
  - https://www.rfc-editor.org/rfc/rfc9420.html
  - https://spec.matrix.org/v1.18/client-server-api/
  - https://book.keybase.io/docs/teams/clkr
  - https://automerge.org/docs/keyhive/ark-api-guide/
  - https://github.com/ucan-wg/spec
  - https://github.com/n0-computer/iroh/discussions/3168
supersedes: []
superseded_by: []
---

# ADR-0003: Відкликання доступу в offline/local-first системі

## Context

Vida дозволяє локальну роботу, P2P-синхронізацію та encrypted scopes. Після revocation частина пристроїв може бути офлайн, мати старі ключі, локальні копії та непередані operations. Система має чітко розділяти припинення майбутнього доступу, прийняття offline operations і неможливе дистанційне стирання вже отриманих даних.

## Findings from similar architectures

### MLS

MLS видаляє учасника через Remove proposal + Commit. Commit створює новий group epoch і fresh entropy, недоступну видаленому учаснику. Захист починає діяти після commit/state transition, а не в момент локального наміру видалити.

### Matrix

Після leave/ban користувач не отримує нові room events, але може бачити історію, яку мав право бачити до виходу. Це прямо відділяє future access від already-visible history.

### Keybase Teams

Видалення учасника супроводжується rotation per-team keys. Нові ключі роздаються чинним учасникам, але не видаленому. Якщо rotation не може виконати ініціатор, сервер оркеструє lazy rotation через доступних admins.

### Automerge Repo Keyhive

Keyhive додає E2EE, access levels, delegation і member revocation до local-first Automerge. Membership changes оновлюють share configuration; protected documents мають окреме керування members/capabilities та keys.

### UCAN

Capability має validity interval і delegation chain. Executor перевіряє capability під час execution, а revocation ламає delegation chain. Повністю автономний offline actor не може миттєво знати про нове revocation, тому короткий lifetime обмежує вікно ризику.

### Iroh

Iroh автентифікує endpoint і транспортує протоколи, але application authorization та revocation залишаються відповідальністю Vida protocol. Ticket або знання EndpointId `MUST NOT` вважатися достатнім правом доступу.

## Decision drivers

- revocation має припиняти майбутні read/write/sync;
- offline device не може бути примусово очищений;
- client timestamp не є довіреним доказом, що operation створено до revocation;
- control-plane state має синхронізуватися раніше за domain data;
- key rotation має бути scope-bound;
- відхилена offline-робота не повинна мовчки зникати.

## Decision

### 1. Authority-accepted revocation

Revocation набуває сили лише після прийняття authority policy відповідного Space/scope. UI `MUST` розрізняти `pending revocation` і `effective revocation`.

Authority записує підписану подію:

```text
GrantRevoked {
  revocationId
  spaceId
  scopeId
  subjectId
  grantId
  acceptedControlSequence
  previousKeyEpoch
  nextKeyEpoch
  authoritySignature
}
```

`acceptedControlSequence`, а не client clock, визначає ordering.

### 2. Control plane before data plane

Під час reconnect peer `MUST` спочатку синхронізувати membership, grants, revocations і current key epochs. Лише після цього дозволяється domain-data sync.

Peer, який не підтвердив актуальний control head, `MUST NOT` отримувати нові encrypted scope data або відправляти authority-accepted mutations.

### 3. Execution-time authorization

Кожна operation `MUST` містити `grantId`, `scopeId`, `policyVersion` і `keyEpoch`. Authority/executor `MUST` перевіряти grant під час прийняття operation.

Operation, не прийнята до effective revocation, `MUST` бути відхилена навіть тоді, коли offline client стверджує, що створив її раніше. Client timestamp не відновлює відкликане право.

### 4. Quarantine замість втрати

Відхилена offline operation `MUST` перейти до local `RevokedOperationQuarantine` із reason, affected resource і recovery actions. Користувач `MAY` експортувати зміни, скопіювати їх у дозволений особистий draft або запросити Manager повторно застосувати їх. Вони `MUST NOT` автоматично merge-итися назад.

### 5. Scope key epoch rotation

Effective revocation `MUST` збільшувати `keyEpoch` лише для affected scopes. Нові operations, snapshots і attachments `MUST` шифруватися новим epoch key, який видається тільки чинним members/devices.

Старий ключ `MUST NOT` використовуватися для нових записів після effective revocation. Rotation у батьківському scope `MAY` каскадуватися до children відповідно до policy.

### 6. Межа гарантії

Vida `MUST` чесно заявляти: revocation припиняє майбутній доступ, але не може стерти plaintext, screenshots, exports або ciphertext+keys, які вже отримав пристрій.

Re-encryption старої історії не скасовує знання видаленого учасника й не є основною revocation-гарантією.

### 7. Offline capability leases

Дані, повністю синхронізовані на пристрій, `MUST` бути доступні локально без мережі за замовчуванням. Це правило охоплює Messenger, Knowledge/Notes, Projects/Tasks та інші schema-driven Apps.

Offline capability lease `MUST` обмежувати не локальне читання вже отриманих даних, а строк, протягом якого створена offline mutation може претендувати на authority acceptance після reconnect. Offline grants `MUST` мати expiration і maximum control-head age. Після expiry пристрій `MUST` оновити grant/control head перед authority-requiring operations.

Platform policy задає глобальну верхню межу. Space або protected scope `MAY` скоротити її чи заборонити локальне зберігання, але `MUST NOT` розширювати понад platform maximum.

V1 використовує такі maximum offline mutation acceptance windows:

- `standard` — 30 днів;
- `protected` — 7 днів;
- `critical` — 24 години.

Membership, role, permission, policy та key-management mutations `MUST` проходити online authority validation і `MUST NOT` покладатися на offline lease. Mutation після завершення window `MUST` перейти до quarantine, а не бути втраченою чи автоматично прийнятою.

Точний maximum lifetime визначається risk class scope; довгоживучі безстрокові write grants `MUST NOT` бути default.

### 8. Re-grant

Повторний доступ після revocation `MUST` створювати новий `grantId`, нові key envelopes та прив'язку до current epoch. Старий grant `MUST NOT` реактивуватися.

### 9. Authority та quorum

У Personal Space зміну доступу `MUST` ініціювати Owner. У shared Space її `MAY` ініціювати Owner або Admin із чинним `manage_members` у відповідному scope.

Authority service відповідного Space `MUST` перевірити поточні права ініціатора, прийняти команду в control log та підписати результат. Локальна клієнтська команда без authority acceptance `MUST NOT` вважатися effective.

Звичайні membership і revocation operations `MUST NOT` вимагати голосування кількох Owners за замовчуванням. Policy critical Space або scope `MAY` вимагати quorum, зокрема `M-of-N`; required quorum `MUST` бути виконаний до authority acceptance.

## Consequences

- local-first UX зберігається, але offline work може бути quarantined після revocation;
- scope isolation зменшує blast radius rotation;
- потрібен durable signed control log і authority receipt;
- clients повинні вміти оновлювати keys/epochs до data sync;
- revoked peer може читати вже локально збережене — це documented limitation.

## Risks і mitigations

- Authority недоступний → revocation лишається pending і явно показується.
- Старий peer роздає старі дані → current peers не приймають old-epoch writes; transport authorization блокує новий sync.
- Втрата offline work → quarantine/export/reapply flow.
- Масова rotation → scope keys, batching і asynchronous envelopes для чинних members.
- Replay → unique operation IDs, grant ID, epoch, control sequence та accepted-operation ledger.

## Acceptance evidence

- revoked offline peer не отримує new-epoch data після reconnect;
- old-epoch operation відхиляється незалежно від client timestamp;
- control plane синхронізується раніше за domain data;
- unaffected scope не змінює key epoch;
- rejected operation зберігається в quarantine;
- re-grant використовує нові IDs/keys;
- UI пояснює неможливість дистанційно стерти вже отримані дані.

## Open questions

Немає для поточного рішення.
