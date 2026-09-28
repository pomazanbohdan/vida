---
id: SPEC-SPACE-MEMBERSHIP-001
status: approved
implementation_status: unplanned
last_updated: 2026-09-20
requirement_refs:
  - ../02-requirements/access-control-requirements.md
  - ../02-requirements/identity-requirements.md
  - ../02-requirements/transport-sync-requirements.md
decision_refs:
  - ../03-architecture/decisions/ADR-0001-layered-access-control.md
  - ../03-architecture/decisions/ADR-0002-default-role-presets.md
  - ../03-architecture/decisions/ADR-0003-offline-revocation.md
  - ../03-architecture/decisions/ADR-0004-multi-axis-identity-model.md
---

# SPEC-SPACE-MEMBERSHIP-001: SpaceMembership contract

## Purpose

Відокремити Space authority від transport reachability та забезпечити однакову membership/revocation model для chats, documents, tasks і blobs.

## Contract

Membership state `MUST` derive from signed control operations. At minimum:

```text
InviteIssued
MemberJoined
RoleAssigned
RoleRevoked
CapabilityGranted
CapabilityRevoked
MemberRemoved
KeyEpochAdvanced
```

An endpoint connection, ticket possession or blob availability `MUST NOT` alone grant membership. `DeviceGrant` and Space membership are separate checks.

Початковий Owner нового Space з'являється лише через підписаний створювачем `WorkspaceGenesis` (product model), а не через запрошення або `RoleAssigned` від ще неіснуючого Owner. Правила призначення нижче діють після genesis.

У shared Space один чинний Owner `MAY` видалити будь-якого іншого Owner зі Space без co-owner approval або `M-of-N`, включно з critical policy. Authority `MUST` перевірити explicit built-in Owner-статус ініціатора в актуальному control state; custom role, навіть клон Owner, не дає цього права. Authority `MUST` зафіксувати й підписати `MemberRemoved` разом із відкликанням усіх Space grants та похідних делегацій як один логічний accepted transition, залишити щонайменше одного Owner та записати audit event. Future-access key epochs affected scopes `MUST` просунутися без вікна, коли старі grants дозволяють нові data operations. Будь-який шлях втрати Owner-статусу, зокрема demotion або replacement, `MUST` пройти ту саму перевірку та не залишати нижчої membership. Лише чинний Owner shared Space `MAY` призначити іншого Owner через `RoleAssigned`; Personal Space `MUST NOT` мати другого одночасного Owner. Admin зі Space-level `manage_members` `MAY` призначати Admin і нижчі ролі, але `MUST NOT` призначати/понижувати/видаляти Owner або підвищувати себе до Owner. Owner appointment у critical Space `MAY` додатково вимагати policy-defined quorum; виняток без quorum стосується лише Owner removal.

`InviteIssued` є наміром, а не набутим членством чи роллю. Коли запрошення активується, authority `MUST` знову перевірити актуальні повноваження ініціатора призначити саме цю роль та чинну policy/quorum; запрошення до Owner-ролі від ініціатора, який уже не Owner, `MUST` бути відхилене. Точний формат, TTL і replay-захист invitation token лишаються deferred.

Реєстрація node account, прийняття запрошення у Space та activation `AppInstance` є різними transitions. Саме посилання чи успішна реєстрація не надає Space grants. Після прийнятого вступу учасник отримує вже призначену й чинну роль із її default permissions та explicit overrides; якщо до вступу authority належно призначив Admin, ці права діють від моменту прийняття членства, без повторного ручного призначення. Вибір ролі, коли її не вказано в запрошенні/політиці, формат і replay protection залишаються відкритими; відсутність ролі не означає автоматичний Admin.

## Revocation

- removal/revoke affects future authorization after the defined causal cut;
- affected secrets/epochs `MUST` rotate according to policy;
- previously decrypted or exported content is outside cryptographic revocation;
- offline stale operations `MUST` pass execution-time authority checks against the accepted control sequence; a client timestamp or alleged pre-cut creation time cannot restore revoked rights (ADR-0003).
- після отримання effective `MemberRemoved` сумісний клієнт видаленого учасника `MUST` негайно заблокувати всі керовані UI/API поверхні цього Space, інвалідувати локальні grants/key envelopes і ініціювати очищення керованого кешу; офлайн-пристрій до отримання події та вже зроблені копії не підлягають гарантії дистанційного стирання;
- authority `MUST` просунути key epochs affected scopes і `MUST NOT` видавати видаленому учаснику нові keys, operations, snapshots чи blobs.

## Offline-read freshness

У shared Space всі schema-driven Apps `MUST` застосовувати одну періодичність узгодження прав `rightsReconciliationInterval = 7 діб` з authority-confirmed control state незалежно від `standard`/`protected`/`critical`. Лише actor, авторизований Space authority policy, може підтвердити оновлення control state; relay/federated node, локальна активність, перезапуск або повторний старий control head самі собою цього не роблять. Синхронізований вміст доступний офлайн до спливу інтервалу, включно з уже відкритим і новим `critical` документом після втрати зв’язку. Після спливу без успішного узгодження сумісний клієнт `MUST` блокувати всі керовані шляхи читання shared Space до нового підтвердження прав. Personal Space лишається доступним без мережі, доки локальний identity context активний.

Втрата мережі сама собою до спливу інтервалу не закриває вже видимий документ і не встановлює особливого online-open gate для `critical`. Якщо отримано effective `MemberRemoved` або сплив інтервал узгодження без нового підтвердження, усі Messenger, Notes, Projects та інші Apps, search, recents, client-managed notifications, export, runtime/API, локальні проєкції та plugin/automation reads `MUST` припинити читання shared Space; відкликання не дорівнює безповоротному стиранню відключеного пристрою. Інтервал узгодження прав не є окремим mutation lease; proof і anti-rollback визначить `OQ-0053`. Явні local lock/logout і policy заборони локального зберігання залишаються незалежними.

Read lock не знищує durable pending work. Клієнт `MUST` дозволяти створити нову локальну чернетку без доступу до заблокованого вмісту; вона не стає authority-accepted і не відкриває дані Space до нового rights proof та перевірки прав під час прийняття (`REQ-ACL-019`). Owner Personal Space керується власною локальною authority policy; Owner shared Space не має винятку із семиденного read lock, бо інший Owner може видалити його офлайн (`OQ-0063` закрито).

## Concurrency and recovery

Concurrent membership changes, зокрема одночасні спроби Owners видалити один одного зі Space, require deterministic ordering and a defined authority policy before implementation. Recovery `MUST NOT` allow a node operator to replace Persona controller authority implicitly.

Independent membership implementations are blocked until `OQ-0031` defines authority serialization, causal cut, concurrent grant/revoke precedence and quorum evaluation, with shared convergence fixtures.

## Acceptance criteria

- unauthorized connected endpoint cannot read/write Space resources;
- removed device cannot decrypt future-epoch content;
- operation already authority-accepted before the removal cut remains valid; a locally created but unaccepted offline operation is rejected after the cut regardless of client timestamp;
- concurrent grant/revoke and cross-removal of Owners fixtures converge deterministically, without ownerless Space;
- one current Owner can remove all other Owners without their approval, and authority records each effective change;
- removed Owner has no residual membership, role or grants in any AppInstance/Container of the Space; Admin removal/demotion of Owner is rejected;
- Owner clone/custom role cannot perform Owner removal; delegated grants rooted in removed membership no longer authorize future operations;
- Admin assignment of Owner, including self-promotion, is rejected; current Owner can assign Owner, Admin or lower roles;
- Owner appointment is rejected if the inviter has lost Owner status before acceptance; Personal Space never materializes two concurrent Owners; AppInstance/Container `manage_members` grants cannot appoint a Space Admin;
- Owner-like custom roles never gain reserved Owner-governance actions such as ownership transfer, key authority or Space deletion; accepted Owner appointment rechecks current critical quorum and policy;
- after removal reaches a compliant client, Messenger/Notes/Projects, search, recents, client-managed notifications and export no longer display managed data of that Space; already displayed OS banners and offline non-delivery are not reported as guaranteed remote wipe;
- offline-відкриття синхронізованих документів і вже відкрита сторінка поводяться однаково для `standard`, `protected` і `critical` при втраті зв’язку;
- уже відкритий документ не приховується лише через втрату мережі до спливу інтервалу; relay/node connectivity без authority-approved proof не видається за успішне узгодження прав; після семи діб без proof усі керовані shared reads заблоковані до нового узгодження;
- clock rollback, restart і old-head replay не видаються за нове authority-confirmed узгодження прав;
- після семиденного shared-read lock нова чернетка без читання старого Space content лишається durable pending, не стає authority-accepted до reconnect/rights check і не відкриває заблоковані дані;
- отримане effective revocation однаково забороняє UI та plugin/runtime/API reads навіть після crash/restart і old-head replay; окремі mutation windows не є read timeout;
- offline mutation candidate не втрачається лише через час; після reconnect чинний Owner/Member із правами може претендувати на прийняття після revalidation, а removed Member — ні, незалежно від локального часу створення;
- membership rebuild from signed control log matches materialized projection.

## Deferred

Controller-history format, quorum/threshold for operations other than Owner removal, invite token schema/replay policy, delegated-grant provenance and revocation fixtures, exact group key protocol (including pending-rekey behavior that cannot accept new writes under old grants), rights-reconciliation proof/time-source design (`OQ-0053`), and recovery cryptosystem.
