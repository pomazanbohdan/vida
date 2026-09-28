---
id: ADR-0003
status: accepted
last_updated: 2026-09-20
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
  - https://mas.owasp.org/MASWE/MASVS-AUTH/MASWE-0024/
  - https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/
  - https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html
  - https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html
  - https://api-security.owasp.org/editions/2023/en/0xa1-broken-object-level-authorization/
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

Відхилена offline operation `MUST` перейти до local `RevokedOperationQuarantine` із reason, affected resource і recovery actions. Користувач `MAY` експортувати зміни, скопіювати їх у дозволений особистий draft або запросити Manager повторно застосувати їх, лише якщо це не суперечить чинному праву доступу. Після повного видалення membership клієнт `MUST NOT` показувати колишньому учаснику захищений вміст Space через quarantine/recovery UI; операції не merge-яться назад автоматично.

### 5. Scope key epoch rotation

Effective revocation `MUST` збільшувати `keyEpoch` лише для affected scopes. Нові operations, snapshots і attachments `MUST` шифруватися новим epoch key, який видається тільки чинним members/devices.

Старий ключ `MUST NOT` використовуватися для нових записів після effective revocation. Rotation у батьківському scope `MAY` каскадуватися до children відповідно до policy.

### 6. Межа гарантії

Vida `MUST` чесно заявляти: revocation припиняє майбутній доступ, але не може стерти plaintext, screenshots, exports або ciphertext+keys, які вже отримав пристрій.

Коли сумісний клієнт отримав підтверджене повне видалення свого membership, він `MUST` негайно заблокувати показ усіх керованих даних цього Space в Messenger, Notes, Projects, пошуку, недавніх елементах, клієнтських списках сповіщень, експорті та інших керованих поверхнях; `MUST` інвалідувати локальні grants/key envelopes і ініціювати очищення керованого кешу. Уже показані OS notification banners, резервні копії та раніше скопійовані дані не піддаються гарантованому відкликанню; прибрання OS banners є best effort, якщо платформа це дозволяє. Це UX/local-state правило **після отримання revocation**, а не обіцянка дистанційно стерти офлайн-пристрій.

Re-encryption старої історії не скасовує знання видаленого учасника й не є основною revocation-гарантією.

### 7. Офлайн-читання, узгодження прав і mutation candidates

Повністю синхронізовані дані `MUST` бути доступні локально без мережі за замовчуванням у Personal Space та, до спливу інтервалу узгодження прав, у shared Space, доки локальний identity context активний і клієнт не отримав effective revocation. Це стосується Messenger, Knowledge/Notes, Projects/Tasks та інших schema-driven Apps; сама втрата з'єднання до спливу інтервалу `MUST NOT` ховати вже відкритий документ, зокрема позначений `critical`. Logout, local lock і explicit policy, що забороняє local persistence, залишаються окремими підставами для обмеження.

Один сталий platform-level інтервал `rightsReconciliationInterval = 7 діб` `MUST` застосовуватися до перевірки й узгодження актуального Space control state в усіх Apps для **кожного учасника shared Space, включно з Owner**. Він **не поділяється за risk class або роллю**: інший Owner може відкликати membership Owner, який перебуває офлайн. Успішне узгодження має походити від actor, авторизованого Space authority policy; саме з'єднання з peer, relay або федеративним вузлом не доводить актуальності прав. Якщо клієнт отримує effective revocation, негайно діє повне блокування з п. 6 незалежно від інтервалу. Якщо після семи діб нового authority-confirmed стану немає, сумісний клієнт `MUST` заблокувати всі керовані шляхи читання кешу shared Space, включно з уже відкритим вмістом, до успішного узгодження. Owner власного Personal Space не успадковує цю межу. Це не TTL для offline mutation candidate і не обіцянка дистанційного стирання вже скопійованого plaintext. Точний proof свіжості та anti-rollback механізм лишаються `OQ-0053`.

Перевірені 2026-09-20 [OWASP MASWE-0024](https://mas.owasp.org/MASWE/MASVS-AUTH/MASWE-0024/), [MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) і [Session Management Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html) стосуються доступу після завершення сесії, захисту локальних даних і session timeout; вони не задають ні класів, ні універсального числа для офлайн-читання синхронізованого Space. Дистанційне стирання даних із відключеного чи зміненого пристрою не гарантується.

Час очікування локально збереженого **offline mutation candidate** не обмежений класом ризику або фіксованим TTL: `standard` 30 днів, `protected` 7 днів і `critical` 24 години скасовані. Сам вік candidate `MUST NOT` бути підставою його відхилення, приховування або автоматичного карантину. Це не є безстроковим правом на прийняття: при повторному підключенні authority/executor `MUST` перевірити актуальні права до конкретного Space/об'єкта/дії, чинний control state, grant/epoch, signature, schema, причинні та бізнесові preconditions. Якщо ці умови чинні, candidate `MAY` бути прийнятий незалежно від віку; якщо ні — відхилений із recoverable quarantine. Створення candidate саме собою не є `authority.accepted` і не обходить конфлікт чи вже зафіксовані зміни.

У Space, де актор є чинним Owner — Personal або shared — локальна робота без мережі `MAY` тривати без обмеження віком candidate у межах чинного identity context. У Personal Space власна authority policy визначає локальне прийняття без чужого member approval; у shared Space навіть кандидат Owner лишається pending до перевірки застосовної Space/object authority policy. Окрема короткоживуча capability чи grant `MAY` спливати, але її expiry не знищує candidate і не замінює перевірку чинних прав при acceptance; спосіб оновлення grant/rebase/re-sign старого envelope без replay лишається `OQ-0033`/`OQ-0034`. Усі mutation classes можуть бути staged без обмеження віком, але membership, role, permission, policy і key-management `MUST` проходити authority validation перед набуттям чинності; offline stage не змінює доступ сам собою.

Це продуктове рішення VIDA, а не числова вимога OWASP. [OWASP Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) і [API1 Broken Object Level Authorization](https://api-security.owasp.org/editions/2023/en/0xa1-broken-object-level-authorization/) вимагають перевіряти права для кожного об'єкта/дії під час виконання; вони не задають максимального віку неприйнятої офлайн-операції.

Можливий строк дії окремої write capability/grant — властивість облікового доступу, а не maximum lifetime збереженого candidate; його значення й policy ще не затверджені. Безстроковий write grant `MUST NOT` випливати з безстрокового очікування candidate.

### 8. Re-grant

Повторний доступ після revocation `MUST` створювати новий `grantId`, нові key envelopes та прив'язку до current epoch. Старий grant `MUST NOT` реактивуватися.

### 9. Authority та quorum

У Personal Space зміну доступу `MUST` ініціювати Owner. У shared Space звичайну зміну non-Owner membership її `MAY` ініціювати Owner або Admin із чинним `manage_members` у відповідному scope. Admin `MAY` призначати Admin або нижчі ролі, але `MUST NOT` ініціювати видалення/пониження Owner чи призначення нового Owner. Останнє може зробити лише чинний Owner.

Authority service відповідного Space `MUST` перевірити поточні права ініціатора, прийняти команду в control log та підписати результат. Локальна клієнтська команда без authority acceptance `MUST NOT` вважатися effective.

Звичайні membership і revocation operations `MUST NOT` вимагати голосування кількох Owners за замовчуванням. Policy critical Space або scope `MAY` вимагати quorum, зокрема `M-of-N`; required quorum `MUST` бути виконаний до authority acceptance.

Виняток із optional quorum: чинний Owner shared Space `MAY` одноосібно видалити іншого Owner зі Space. Для будь-якого шляху втрати Owner-статусу `MUST NOT` вимагатися co-owner approval або `M-of-N` навіть у critical Space, але він `MUST` закінчуватися повним видаленням membership, а не залишковою нижчою роллю. Authority `MUST` перевірити explicit built-in Owner-статус ініціатора в чинному control state, серіалізувати зміну в control log, зберегти щонайменше одного Owner, відкликати всі Space grants видаленої особи та делегації, похідні від цього membership, просунути key epochs усіх affected scopes і підписати результат. Membership removal та припинення всіх grants `MUST` бути одним логічним authority-accepted transition; відкладена фізична rotation `MUST NOT` залишати старі grants чинними для нових data operations. Admin `MUST NOT` бути ініціатором цієї операції.

## Consequences

- local-first UX зберігається, але offline work може бути quarantined після revocation;
- scope isolation зменшує blast radius rotation;
- потрібен durable signed control log і authority receipt;
- clients повинні вміти оновлювати keys/epochs до data sync;
- відкликаний, але ще не поінформований офлайн-пристрій сумісного клієнта може читати вже локально збережене не довше ніж до спливу семиденного інтервалу від останнього підтвердженого узгодження; змінений клієнт або раніше скопійовані дані залишаються поза гарантією.

## Risks і mitigations

- Authority недоступний → revocation лишається pending і явно показується.
- Старий peer роздає старі дані → current peers не приймають old-epoch writes; transport authorization блокує новий sync.
- Втрата offline work → quarantine/export/reapply flow.
- Масова rotation → scope keys, batching і asynchronous envelopes для чинних members.
- Replay → unique operation IDs, grant ID, epoch, control sequence та accepted-operation ledger.
- Optional `M-of-N` для інших critical operations не захищає від одноосібного усунення співвласників; це прийнятий trade-off простої ownership-моделі. Точна поведінка threshold після зміни складу Owners лишається в `OQ-0033`.

## Acceptance evidence

- revoked offline peer не отримує new-epoch data після reconnect;
- old-epoch operation відхиляється незалежно від client timestamp;
- 6-місячний offline candidate з чинними правами, незмінним grant/epoch і виконаними preconditions не відхиляється за віком; той самий candidate від removed member не приймається незалежно від часу створення;
- Owner Personal Space продовжує локальну роботу без мережі після такого самого інтервалу без зовнішнього approval; pending candidate переживає restart/restore;
- control plane синхронізується раніше за domain data;
- unaffected scope не змінює key epoch;
- rejected operation зберігається в quarantine;
- re-grant використовує нові IDs/keys;
- видалення іншого Owner приймається з авторизацією одного чинного Owner без додаткових голосів, забирає весь його Space access і не може залишити Space без Owner; Admin із `manage_members` отримує відмову;
- клієнт після прийняття й отримання membership removal перестає показувати весь керований Space content і інвалідує локальні ключі/кеш; тест офлайн-пристрою окремо підтверджує, що до отримання revocation негайне стирання не гарантується;
- зникнення мережі саме собою до спливу семи діб не приховує вже відкритий документ і не встановлює окремого read timeout для `critical`; received effective revocation блокує керовані поверхні негайно, а сплив інтервалу без нового підтвердження блокує cached shared read;
- всі Apps використовують один інтервал узгодження прав без `standard/protected/critical` таймерів читання; сам факт підключення до peer/relay/federated node не підтверджує rights freshness;
- UI пояснює неможливість дистанційно стерти вже отримані дані.

## Open questions

- `OQ-0051` закрито: 7 діб, після спливу без підтвердження shared read блокується до узгодження.
- `OQ-0063` закрито: Owner shared Space підпадає під те саме семиденне блокування читання; Owner Personal Space — ні.
- `OQ-0053`: формат доказу свіжості, доставка revocation та anti-rollback перевірка семиденного інтервалу.
- `OQ-0033`/`OQ-0034`: як revalidate/rebase/re-sign старий candidate після зміни grant/epoch та як розв'язати конфлікт із новішим accepted state без залежності від client timestamp.
