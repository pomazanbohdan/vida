---
id: SPEC-SCHEMA-EVOLUTION-DRAFT
status: draft
implementation_status: research-backed-proposal
last_updated: 2026-09-23
requirement_refs:
  - ../02-requirements/app-package-requirements.md
decision_refs:
  - ../03-architecture/decisions/ADR-0007-declarative-app-packages.md
  - ../03-architecture/decisions/ADR-0012-command-event-sync-boundary.md
open_question_refs:
  - ../00-governance/open-questions.md#oq-0040
---

# Чернетка контракту еволюції схем VIDA

## Дві різні міграції

**Resource-schema migration** змінює чинний контракт синхронізованих Note, Task та інших Resource у Space. Її активація потребує звичайного Space authority/compatibility процесу; окремий Device або локальна БД не визначає current schema для всіх. **Local-store migration** змінює фізичні SQLite таблиці, індекси або кеш одного Device. Локальна транзакція може зробити цей крок атомарним, але не приймає shared-Space schema і не вирішує записи старого offline клієнта ([SQLite transactions](https://www.sqlite.org/lang_transaction.html), [`user_version`](https://www.sqlite.org/pragma.html#pragma_user_version)). Це розділення не обирає конкретний database provider.

## Зафіксований напрям

- Кожен durable resource і operation `MUST` містити immutable `ContractId/SchemaId` та version, за якою створено payload.
- Один `AppInstance` має одну **активну current schema** для нових writes. Оновлення пакета саме по собі не переписує дані й не активує нову schema до compatibility/migration gate.
- Старі записи `MUST` бути читабельними через детермінований versioned converter/upcaster. Якщо current schema додала required field без безпечного default, edit projection `MAY` містити явний non-persistable `missing-required` validation state; це не видається за валідний current resource. Історичні operations не переписуються; їхній original schema/version і audit provenance зберігаються.
- Сумісне додавання optional/defaulted field `SHOULD` використовувати lazy read projection і нові writes у current schema; масовий eager rewrite не потрібний лише заради доданого поля. Під час **будь-якого редагування** legacy record runtime upcast-ить його до current schema: безпечні defaults матеріалізуються, а required field без безпечного default блокує save, доки користувач або authorized automation не надасть значення. Read старого запису не зобов'язує одразу переписувати історію.
- Breaking change `MUST` мати підписаний deterministic/idempotent forward migration path `vN -> vN+1`, negative fixtures і crash-resume semantics. Старий client `MUST NOT` виконувати lossy write, що видаляє невідомі поля; affected AppInstance стає read-only/update-required, тоді як інші Apps продовжують працювати. Post-activation schema downgrade/rollback не входить у baseline.
- Числовий field/enum identity `MUST NOT` перевикористовуватися з новим значенням; видалені numbers резервуються. Видалені names також резервуються, коли підтримується JSON/TextProto або інша name-based representation. Unknown fields/values `MUST` або зберігатися round-trip, або викликати явну compatibility failure — ніколи тиху втрату.
- Rename `MUST` зберігати stable field identity або явний alias/converter `oldName -> newName`. «Видалення» означає deprecation/tombstone у current presentation і резервування identity; значення не видаляється з retained operations/snapshots. Фізичне очищення можливе лише окремою затвердженою data-retention operation після safe migration frontier, не як побічний ефект зміни schema.
- Схеми й converters, потрібні для даних у retained history/snapshots, лишаються доступними за immutable digest. Після доведеного safe migration frontier старі **served/runtime** версії можна вимкнути, але audit schema descriptor для retained історії не видаляється.

## Рекомендована модель

VIDA використовує hub-and-spoke підхід: current canonical domain model є hub; кожна підтримувана stored/wire schema має converter до/з hub там, де round-trip безпечний. Це не означає Kubernetes або Avro як runtime dependency; це запозичує перевірені принципи: одна current storage version, паралельне serving під час rollout, явна conversion/migration, writer schema для старих даних, defaults лише для сумісних additions і заборона повторного використання field identity.

Міграція є окремою signed system operation/batch із `migrationId`, source/target schema digests, input frontier, output digest, progress checkpoint і failure record. Вона може детерміновано заповнити нове required field погодженим значенням/default. Якщо безпечного значення немає, кожен legacy record вимагає його при наступному edit. Окремої generic bulk-update дії лише для цього baseline не вводить. Міграція не запускає command handlers або зовнішні effects. Результати синхронізуються як звичайні operations; два peers не повинні незалежно породжувати різні logical migrations для того самого `migrationId`.

**Кандидат протоколу, не затверджена authority topology:** перевірений пакет фіксує immutable schema/converter digests; кожен Device виконує preflight на staged copy від названого input frontier; пропозиція міграції зв'язує `migrationId`, source/target digests, input frontier та очікуваний output digest; Space приймає пропозицію за своєю чинною policy; після acceptance кожен Device ідемпотентно застосовує її й атомарно перемикає локальний active schema pointer. Старий offline Device зберігає власні записи до reconnect; якщо безпечний round-trip/conversion неможливий, його write залишається видимим pending/incompatible, а не стирає нові поля. Точне правило authority, mixed-version window і repair залишається `OQ-0039`/`OQ-0040`/`OQ-0037`. Підписаний package і TUF-style update захищають походження/freshness артефактів, але не доводять правильність converter.

## Candidate conformance fixtures — не виконані

| ID | Перевірний результат |
|---|---|
| SCHEMA-F01 | Optional/defaulted field читається у legacy записі; наступний edit створює current-schema write, не переписуючи immutable history. |
| SCHEMA-F02 | Required field без default показує `missing-required`; save не проходить, доки actor не надасть значення. |
| SCHEMA-F03 | Два рівнозначні Devices застосовують той самий accepted `migrationId`/input frontier та отримують однаковий output digest; replay не дублює logical migration. |
| SCHEMA-F04 | Crash у кожній точці multi-record local migration відновлюється з checkpoint; active version не стає частково новою. |
| SCHEMA-F05 | Відсутній blob, converter або malformed record зупиняє preflight з явною причиною; стара активна версія залишається чинною. Для першої активації instance лишається неактивним. |
| SCHEMA-F06 | Старий offline клієнт після активації v2 не губить невідоме поле з нового запису: safe round-trip або явний incompatible/pending write. |
| SCHEMA-F07 | Stale/expired/downgraded package metadata відхиляється **до** міграції; це retrieval/trust fixture, не доказ semantic correctness. |
| SCHEMA-F08 | Local SQLite migration успішно відновлює projection/cache, але не може сама змінити shared-Space current schema чи authority frontier. |

## Відкриті рішення

1. Погоджено: optional/defaulted additions читаються lazy, можуть materialize background і обов'язково записуються у current schema при наступному write. Відкрите лише правило вибору/планування background materialization.
2. Який minimum supported mixed-version window і коли старий client переходить у read-only/update-required?
3. Хто має право активувати package schema migration в shared Space і який authority receipt робить її чинною?
4. Які mandatory pre-activation fixtures доводять, що migration, UI і handler працюють до atomic activation? Post-activation defect recovery/rollback зараз не проєктується.
5. Як міграція поводиться з partially available blobs, unknown extensions і даними plugin, якого більше немає локально?
6. Після прийняття нової schema старий offline клієнт надсилає зміну старого формату: які conversion і mixed-version умови дозволяють її прийняти автоматично, а коли лишити pending та попросити оновити лише affected AppInstance?

## Референси

- [Kubernetes CRD versioning](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definition-versioning/) підтримує паралельні served versions, одну storage version, conversion і явну storage migration перед видаленням старої версії.
- [Protocol Buffers proto3](https://protobuf.dev/programming-guides/proto3/) зберігає unknown binary fields, забороняє перевикористовувати field/enum numbers і радить резервувати names для JSON/TextProto compatibility; JSON/field-by-field conversion може втратити unknown fields.
- [Apache Avro 1.12 schema resolution](https://avro.apache.org/docs/1.12.0/specification/) читає дані з writer schema та reader schema; added reader fields потребують reader default, інакше resolution завершується помилкою. Default не робить поле optional під час encoding.
- [The Update Framework](https://theupdateframework.github.io/specification/) дає signed version/expiry metadata та rollback/freeze/mix-and-match protection для отримання package і migration artifacts; TUF захищає retrieval, але не визначає correctness самої міграції.
- [SQLite transactions](https://www.sqlite.org/lang_transaction.html) стосуються атомарності **локальної** БД; [`user_version`](https://www.sqlite.org/pragma.html#pragma_user_version) задається застосунком і не є distributed authority. [Android Room migrations](https://developer.android.com/training/data-storage/room/migrating-db-versions) показують, навіщо зберігати versioned schemas та тестувати всі migration paths; destructive fallback не є прийнятним механізмом збереження VIDA-даних.

Ці референси визначають безпечні патерни, але не обирають wire codec VIDA й не наказують саме hub-and-spoke, signed migrations або retention policy. Exact manifest, converter ABI, migration authority, background materialization schedule та mandatory activation fixtures лишаються `OQ-0040`/`OQ-0037`.
