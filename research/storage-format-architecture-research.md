# ADR / Technical Design Note: формат даних платформи

- **Дата перевірки джерел:** 2026-09-18
- **Статус:** рекомендоване рішення для нового проєкту; остаточне wire contract потребує production benchmark-а та conformance tests
- **Тип документа:** ADR і технічний дизайн
- **Рішення:** **Platform Data Model + deterministic schema-aware CBOR** як канонічне binary representation; JSON як редаговане джерело; SQLite і серверні БД як проєкції моделі.

> Це архітектурна рекомендація, а не твердження про вже реалізовану систему. Позначення MUST нижче є запропонованою вимогою Platform CBOR Profile v1.

## 1. Резюме

Серіалізація не повинна бути логічною моделлю платформи. Спершу слід визначити єдину типізовану **Platform Data Model (PDM)** для сутностей, схем застосунків, сторінок, компонентів, permission rules, sync operations і зовнішніх посилань.

PDM має кілька незалежних адаптерів:

- **JSON** для редагування, code review, діагностики й імпорту/експорту.
- **Platform CBOR Profile** для канонічних переносних snapshots, sync payloads, CAS і міжмовного обміну.
- **SQLite та серверні SQL БД** для транзакцій, індексів і запитів.
- **Zstandard** для стискання блоків або пакетів, якщо вимірювання підтвердять break-even.
- **BLAKE3** для revision/content identity від точно визначених детермінованих байтів.
- **iroh / iroh-blobs** як кандидат на транспорт і verified transfer великих immutable blobs; це не schema registry та не conflict-resolution protocol.

CBOR рекомендований не через доведену перевагу швидкості. Попередні синтетичні експерименти вимірювали лише розміри. Вони показали, що **schema awareness, розмір batch і compression boundary впливають на результат не менше, ніж вибір CBOR проти MessagePack**. На великому batch Amazon Ion був дуже конкурентним і в окремих режимах трохи компактнішим. CBOR обрано як баланс стандартизованих deterministic encoding requirements, numeric map keys/tags, міжмовної доступності та меншої semantic surface, ніж повна Ion data model.

Це не означає, що всі дані треба зберігати в одному BLOB. Часті для пошуку й сортування поля мають бути індексованими колонками; CBOR payload доречний для dynamic remainder, snapshots та обміну.

## 2. Межі задачі

«Файли чи база», «текст чи binary», «стиснення» і «синхронізація» — різні рішення.

| Шар | Відповідає за | Пропозиція |
|---|---|---|
| Семантика | Значення й поведінка об’єктів | Platform Data Model |
| Схеми | Дозволені поля, ID, версії та сумісність | Platform Schema Registry |
| Серіалізація | Модель → байти | Platform CBOR Profile |
| Редагування | Людський source | JSON; YAML лише за окремої потреби |
| Стиснення | Розмір блоку/пакета | Zstandard після вимірювання |
| Локальне зберігання | Транзакції, індекси, відновлення | SQLite adapter |
| Серверне зберігання | Запити, індекси, транзакції | Server SQL adapter/projections |
| Великі файли | Attachments і великі payloads | Blob/CAS; PDM містить ContentRef |
| Content identity | Revision hash | BLAKE3 від profile-defined bytes |
| Транспорт | Доставка повідомлень і blobs | окремий transport adapter; оцінити iroh |
| Конфлікти | Merge, causal order, retry | окремий sync/change protocol |

CBOR не задає permission model, SQL schema, schema migration, replication semantics або CRDT. Ці контракти належать PDM та платформним протоколам.

## 3. Порівняння CBOR, Amazon Ion і MessagePack

| Властивість | CBOR | Amazon Ion | MessagePack |
|---|---|---|---|
| Основне призначення | Загальний IETF binary data model | Rich self-describing data model з текстовим і binary поданням | Компактний binary serializer |
| Типи | Int/float, bytes/text, arrays/maps, bool/null, tags | Arbitrary-size int, decimal, timestamp, symbol, blob/clob, list/sexp/struct, typed null, annotations | Int до signed 64/unsigned 64, float, string/bin, array/map, extensions, timestamp extension |
| Текст | Diagnostic notation для перегляду, не повноцінний парний source format | Ion Text — повноцінне семантично споріднене подання; text/binary транскодуються з майже повною fidelity | Вбудованого повноцінного text notation немає |
| Імена полів | Повторюються, якщо не введено numeric IDs | Binary symbols — integer SIDs з визначенням через symbol tables | Повторюються, якщо не введено numeric map keys |
| Схеми | Не нав’язує; потрібен окремий registry/profile | Ion Schema існує окремо; сама модель багата | Не нав’язує; потрібен власний profile |
| Determinism | RFC 8949 має core deterministic requirements; profile фіксує семантичні деталі | Треба визначити deterministic/hash rules та symbol-table context | Application profile має визначити map order, extensions і normalization |
| Sparse read | Streaming; для пропуску вкладеного container часто потрібні traversal або framing/index | Length-bearing values допомагають skip; random access усе одно потребує layout/index | Streaming; length для extension payload можливий, але це application design |
| Найсильніша сторона | Стандартна основа для переносного deterministic profile | Rich types, symbols, text/binary symmetry, великий batch/read-sparse | Прості та швидкі runtime serializers, RPC/cache |
| Основна ціна | PDM, schema evolution і semantic normalization визначає платформа | Symbol lifecycle та додаткова data semantics | Canonical storage вимагає власних conventions для types, extensions, schema і hashing |

### 3.1. CBOR

CBOR визначає maps, arrays, integers, floats, bytes/text, simple values і semantic tags. RFC 8949 описує core deterministic encoding requirements, але формат допускає профілі, які встановлюють точні правила. Тому Platform Profile все одно має визначити field IDs, types, duplicate-key policy, float/decimal policy, absence/null/default semantics і Unicode normalization.

Переваги для платформи:

- Numeric field IDs кодуються як unsigned map keys.
- Tags дозволяють застосовувати узгоджені semantic types.
- Можна обмежити дозволений піднабір CBOR без винайдення нового codec.
- Модель не прив’язує логіку до однієї бази чи мови.

Обмеження: CBOR сам не дає schema registry, міграцій або бізнес-семантики. Deterministic encoding сам по собі не гарантує однаковий hash для семантично еквівалентних значень, якщо profile не визначив decimal, floats, Unicode, defaults та timestamps. Schema metadata також може переважити виграш на одиничному маленькому записі.

### 3.2. Amazon Ion

Ion має rich data model та text і binary notations. Офіційна специфікація визначає їх як семантично ізоморфні; text whitespace/comments не переносяться у binary. У binary символи подаються SID, а таблиця символів надає відповідний текст. Shared symbol tables можуть винести повторювані назви полів і значення з окремих документів.

Переваги:

- arbitrary-size integer, decimal із значущими trailing zeros, timestamp precision/timezone semantics, symbol, typed null, annotations;
- вбудований читабельний text format;
- shared symbols добре підходять для однорідних великих streams;
- length-bearing encoded values допомагають пропускати непотрібні піддерева.

Витрати:

- Decoder має правильно отримати symbol-table context; imported symbols без таблиці можуть мати unknown text.
- Ion structs дозволяють duplicate field names, що не можна непомітно перетворити на звичайний map або SQL row.
- Повна Ion semantics збільшує підтримуваний surface для PDM та всіх DB/runtime adapters.
- Офіційний Amazon Ion .NET repository позначений archived, а бібліотека deprecated; це ризик підтримки .NET, а не доказ застарілості Ion як формату.
- Для окремих невеликих streams IVM і symbol-table overhead може нівелювати виграш symbols.

Ion варто підтримувати як optional import/export, інтеграцію з Ion-native системами або кандидат для повторного bulk/read-sparse benchmark-а. У PDM доцільно запозичити ідеї symbol registry, semantic types і text/binary symmetry, не переймаючи всю Ion data model.

### 3.3. MessagePack

MessagePack має компактні fixint/fixmap/fixarray та length-coded strings, binary values, arrays і maps; timestamp визначений як extension type. App-specific extension values потребують спільного registry між мовами. Специфікація допускає application profiles, включно з сортуванням map keys для hash use case, але це стає відповідальністю платформи.

Переваги: простий типний набір, компактні кодування типових значень і хороші serializer implementations. Це хороший вибір для RPC/cache/hot path, якщо вимірювання конкретного runtime доведе суттєвий виграш.

Для canonical storage доведеться формально визначити extensions, map order, numeric/float rules, schema evolution, unknown types і deterministic profile. Швидка реалізація однієї мови не доводить перевагу MessagePack як міжмовного canonical store.

## 4. Критерії та попередня оцінка

### 4.1. Критерії та ваги

| Критерій | Вага | Перевіряється |
|---|---:|---|
| Семантика й точність типів | 8 | decimal/bigint/time/binary/null/custom types |
| Малий object | 7 | headers, key IDs, scalar/container overhead |
| Великий batch | 6 | повторювані записи, schema amortization |
| Розмір після Zstd | 5 | compressed size з однаковими boundaries |
| Overhead незалежних objects | 5 | окремий файл, operation або payload |
| Runtime performance/ecosystem | 6 | production implementations у цільових мовах |
| Partial read/skip | 6 | вибірковий доступ без materialization |
| Streaming/append | 5 | incremental parse/encode і framing |
| Deterministic hashing | 8 | один normalized value → стабільні байти |
| Text ↔ binary | 7 | semantic round trip і debugging |
| Schema evolution | 8 | compatibility, unknown fields, migrations |
| DB projection | 6 | indexability, joins, backend adaptation |
| Extensibility | 5 | tags/extensions/custom types |
| Rust/.NET/Web ecosystem | 9 | реалізації й однакова conformance |
| Standard/governance | 5 | стабільність формату та специфікації |
| Tooling/security maturity | 4 | conformance, fuzzing, limits, updates |
| **Разом** | **100** | |

### 4.2. Експертна scorecard

Шкала 1–5; підсумок зважено до 100. Це архітектурна оцінка, не вимірювання швидкості. Переоцінити після production corpus і перевірки library versions.

| Критерій | Вага | CBOR Profile | Ion | MessagePack Profile |
|---|---:|---:|---:|---:|
| Semantic types | 8 | 4.6 | 5.0 | 3.5 |
| Small object | 7 | 4.95 | 4.5 | 5.0 |
| Bulk compactness | 6 | 4.8 | 5.0 | 4.7 |
| Compression | 5 | 4.7 | 4.85 | 4.8 |
| Independent objects | 5 | 5.0 | 4.2 | 4.9 |
| Runtime | 6 | 4.7 | 4.0 | 5.0 |
| Sparse read | 6 | 3.2 | 5.0 | 3.1 |
| Streaming | 5 | 5.0 | 4.2 | 4.2 |
| Determinism/hash | 8 | 5.0 | 4.6 | 2.5 |
| Text ↔ binary | 7 | 3.0 | 5.0 | 2.0 |
| Schema/evolution | 8 | 4.5 | 4.8 | 2.8 |
| DB mapping | 6 | 4.6 | 3.8 | 5.0 |
| Extensibility | 5 | 5.0 | 5.0 | 3.5 |
| Rust/.NET/Web | 9 | 5.0 | 3.5 | 5.0 |
| Standards | 5 | 5.0 | 3.8 | 4.0 |
| Tooling/security | 4 | 4.6 | 4.1 | 4.8 |
| **Weighted total** | **100** | **91.8** | **89.3** | **79.5** |

CBOR і Ion близькі. Результат залежить від того, наскільки важливі cross-language canonical bytes і наскільки платформа готова нести багатшу Ion semantics та table lifecycle.

## 5. Попередні synthetic benchmark-и

### 5.1. Надійність і межі

Таблиці нижче збережені з попереднього дослідження. У поточному project mirror не знайдені harness, seed-и, synthetic input corpus або raw output files. Тому їх **незалежно не відтворювали** під час підготовки цього документа.

- Перший раунд описаний як size test, не performance test; Zstandard 1.5.7, level 3, без dictionary. Для CBOR використовувався мінімальний код, що реалізував лише потрібну тесту підмножину.
- Другий раунд включав representative objects, PageSchema, batch 100 000 Tasks та 1 000 окремих streams. Для compression зазначено Zstd-3, але решта harness/config деталей не збережена.
- Невідомі повні encoder/decoder версії, hardware, точні compression frame settings, metadata/framing inclusion, schema registry transfer, signing/encryption, error bars і timings.
- Одиниці MB залишено як у попередньому звіті; decimal чи binary convention невідомий.
- Ці числа формують гіпотези, але не прогнозують production capacity або latency.

### 5.2. Раунд 1: малий object та 1 000 Tasks

Synthetic Task містив UUID, українські назви, дати, priority і tags. 1 000 Tasks кодувалися одним масивом і стискалися одним блоком.

| Подання | Один object raw → Zstd | 1 000 Tasks raw | 1 000 Tasks + Zstd |
|---|---:|---:|---:|
| JSON | 51 → 60 B | 187 034 B | **42 393 B** |
| MessagePack, text keys | 32 → 41 B | 156 329 B | 42 524 B |
| CBOR, text keys | 33 → 42 B | 156 504 B | 42 664 B |
| CBOR, numeric field IDs | **14 → 23 B** | **128 504 B** | **37 062 B** |

Висновки саме цього тесту:

1. Compression одного малого object збільшив розмір у всіх випадках; стискати кожне маленьке operation окремо невигідно.
2. Для 1 000 Tasks compressed JSON був на 131 B меншим за MessagePack і на 271 B меншим за generic CBOR.
3. Numeric field IDs дали близько 12.6% виграшу проти JSON+Zstd у цьому наборі.
4. Схема не була включена в ці payload sizes; демонстраційна JSON mapping таблиця займала 59 B. Для одиночного автономного object overhead контексту може перекрити виграш.
5. Round-trip тести проходили, але це не production library performance result.

### 5.3. Раунд 2: десять representative structures

Сумарний розмір десяти окремих representative objects: Task, Note, Document, Workspace, ApplicationSchema, PageSchema, PermissionRule, ChatMessage, Address і SyncOperation.

| Формат | Raw | Zstd-3 |
|---|---:|---:|
| JSON | 26 037 B | 4 119 B |
| Ion Text | 22 691 B | 3 958 B |
| CBOR generic | 18 394 B | 3 584 B |
| MessagePack generic | **18 370 B** | 3 589 B |
| Ion + local symbols | 11 770 B | 4 010 B |
| **CBOR + schema IDs** | 10 054 B | **2 786 B** |
| **MessagePack + schema IDs** | **10 030 B** | 2 818 B |
| Ion + shared symbol table | 11 024 B | 3 297 B |

CBOR schema-aware та MessagePack schema-aware відрізнялися на 24 B raw, приблизно 0.24% від CBOR total. За розміром raw цієї таблиці вибір між ними нічим не обґрунтований. CBOR schema-aware був на 32 B меншим після Zstd.

**ApplicationSchema raw sizes:**

| Подання | Raw |
|---|---:|
| JSON | 10 852 B |
| CBOR generic | 7 387 B |
| MessagePack generic | 7 387 B |
| Ion local symbols | **3 782 B** |
| CBOR schema-aware | 3 436 B |
| MessagePack schema-aware | 3 436 B |
| Ion shared symbols | 3 673 B |

**PageSchema із 60 UI components:**

| Подання | Raw | Zstd |
|---|---:|---:|
| CBOR generic | 7 392 B | 928 B |
| MessagePack generic | 7 356 B | 885 B |
| Ion local symbols | 4 173 B | 986 B |
| CBOR schema-aware | 3 645 B | **781 B** |
| MessagePack schema-aware | **3 609 B** | 792 B |
| Ion shared table | 4 052 B | 856 B |

Ці приклади показують, що numeric/symbol field IDs скорочують raw data, але повторювані text keys можуть добре стискатися. Після Zstd потрібне окреме вимірювання.

### 5.4. 100 000 Tasks одним batch

| Подання | Raw | Zstd-3 |
|---|---:|---:|
| JSON | 34.56 MB | 4.871 MB |
| CBOR generic | 23.37 MB | 3.544 MB |
| MessagePack generic | 23.57 MB | 3.663 MB |
| Ion shared symbols | **16.52 MB** | **3.522 MB** |
| CBOR schema IDs | 16.67 MB | 3.871 MB |
| MessagePack schema IDs | 16.87 MB | 3.581 MB |

Ion shared symbols був найменшим raw encoding-ом і був приблизно на 0.9% менший за schema-aware CBOR. Після compression Ion був на 22 kB меншим за generic CBOR у цьому corpus; schema-aware CBOR мав більший compressed size за generic CBOR. Це не універсальна властивість форматів: ключі, table representation, batch shape і compression boundaries впливають на результат.

### 5.5. 1 000 Tasks як 1 000 незалежних streams

| Подання | Raw total |
|---|---:|
| CBOR schema-aware | **165 059 B** |
| MessagePack schema-aware | 167 024 B |
| Ion shared-table import | 194 367 B |
| Ion local symbol table | 255 367 B |

Попередня інтерпретація: IVM та symbol-table context помітні, якщо кожний маленький object — незалежний stream. Точний encoder/framing setup не збережений, тому результат має нижчу відтворюваність і потребує повторного тесту.

### 5.6. Загальні висновки з вимірювань

- Schema-aware field IDs мали найбільший raw-size ефект на звичайних записах.
- Ion shared tables конкурентні на довгих однорідних streams.
- Text keys іноді краще стискаються, ніж numeric IDs; raw і compressed size не можна вважати одним metric.
- Треба окремо вимірювати одиничний object, окрему sync operation, block, homogeneous batch і повний package.
- Жодна таблиця не порівнює throughput, allocations, latency, random access, security або migration cost.

## 6. Platform CBOR Profile v1

### 6.1. Envelope і schema binding

Standalone object чи sync operation мусить мати decode context. У homogeneous block/package спільні атрибути можна записати один раз, щоб не дублювати schema metadata в кожному record.

~~~text
PlatformEnvelope {
  formatVersion
  objectKind
  schemaId
  schemaVersion
  payload
}
~~~

- **formatVersion** — версія wire profile, незалежна від app build.
- **objectKind** — тип root object або operation.
- **schemaId** — стабільна namespaced schema identity.
- **schemaVersion** — точні semantics payload-а.
- **payload** — schema-aware CBOR map з numeric field IDs.
- Entity ID може бути в payload чи operation envelope згідно з PDM.
- Revision hash зберігається поза preimage bytes, щоб уникнути самопосилання.
- Фактичну envelope wire form треба зафіксувати golden vectors перед реалізацією.
- Для hashing schema context має бути включений у deterministic bytes. Homogeneous block може містити context у hashed block header.

### 6.2. Schema і ID lifecycle

- Schema ID має бути стабільним namespaced opaque ID, незалежним від display name чи шляху source file.
- Schema definitions — versioned artifacts, доступні локально/offline; decoder не повинен мовчки завантажувати довільні schemas із мережі при читанні untrusted data.
- Schema version означає конкретні schema semantics, не package build.
- Compatibility, defaults, aliases і migrations належать registry, не serializer.
- Field IDs — positive unsigned integer map keys, стабільні в межах schema identity.
- Rename змінює registry label, але не wire ID.
- Видалений field/enum/union ID залишається tombstoned; IDs не перевикористовуються.
- Нове optional поле отримує новий ID. Required field потребує явної migration/default policy.
- Треба визначити діапазони IDs, reserved values, enum IDs і global vs schema-local allocation.
- Multi-tenant app schemas мають бути namespaced, без вимоги однієї глобальної таблиці всіх користувачів.

### 6.3. Unknown fields, absent і null

Profile має чітко розрізняти missing, null, false, 0, порожній string, порожній list/map.

Unknown-field modes:

1. **Strict** — відхилити unknown keys для закритих або security-critical schemas.
2. **Preserve** — зберегти unknown ID/value як opaque subtree для forward-compatible round-trip.
3. **Ignore** — лише для read-only client, якщо втрата даних дозволена контрактом.

Default для editor/import/export і replication — preserve, якщо runtime може безпечно зробити round-trip. Unknown values не виконуються як code, expression чи permission rule.

### 6.4. Semantic types

| Тип PDM | Profile contract |
|---|---|
| Bool/String/Bytes | Окремі типи; bytes у binary не кодувати мовчки як Base64 string |
| Int/UInt/BigInt | Допустимий діапазон і exact JSON round-trip; JS binary64 не використовувати для arbitrary 64-bit integer |
| Decimal/Money | Не кодувати як float; закріпити scale/significance, minor-unit integer або allowlisted exact decimal tag |
| Float | Визначити width, NaN, infinities, negative zero; non-finite values бажано відхиляти |
| Date | Calendar date без автоматичного UTC conversion |
| Instant/DateTime | Визначити UTC basis, precision, leap-second і fractional-second policy |
| LocalDateTime/ZonedDateTime | Не перетворювати на instant без timezone та migration rule |
| Duration | Визначити одиниці й точність |
| EntityRef/SchemaRef/ContentRef | Явні типи з namespace та hash algorithm |
| UUID | Зафіксувати однаковий byte order у Rust/.NET/JS |
| Enum/Union | Stable numeric discriminants і unknown-variant policy |
| List/Map/Object | Зафіксувати key type, ordering, duplicates та nested constraints |

CBOR tags використовувати для усталеної стандартизованої семантики після перевірки IANA registry. Custom/platform tag namespace не затверджувати до окремої policy. Де тип відомий зі schema, tag може бути зайвим overhead; schema semantics можуть задавати точний тип.

### 6.5. Deterministic encoding

Profile v1 пропонується:

1. RFC 8949 core deterministic encoding: preferred shortest encodings, definite lengths, deterministic map key order.
2. Відхиляти duplicate map keys і trailing top-level bytes у single-object file.
3. Визначити allowlist semantic tags та unknown-tag handling.
4. В schema-aware maps сортувати unsigned field IDs чисельно згідно з profile.
5. Зафіксувати canonical bytes кожного semantic type.
6. Заборонити неявну coercion Int/Decimal/Float/String/Date/Instant/Bytes.
7. Визначити, чи optional defaults omitted чи encoded; одна policy для всіх encoder-ів.
8. Unicode normalization визначати для окремих semantic types лише за потреби; не нормалізувати всі user strings без явної вимоги.
9. Визначити NaN/infinity/negative-zero behavior.
10. Hash/signature-sensitive decoder має відхиляти недетермінований або malformed input.

Deterministic byte encoding не робить семантично еквівалентними різні timezone spellings, decimal scales, omitted defaults і Unicode normalization forms. Цю нормалізацію визначає PDM profile.

### 6.6. Map і tuple profiles

- **Schema-aware numeric map** — default v1; дозволяє optional fields і додавання IDs без позиційного зсуву.
- **Text-key map** — diagnostic/import/export profile, не hashed canonical representation.
- **Tuple/array** — майбутня optimization для compiled fixed schemas; не вмикати у v1, доки не узгоджені reserved positions, absent/null, trailing omissions, evolution та sparse read.
- Tuple mode, якщо його затвердять, має бути versioned явно; decoder не виводить його з випадкової форми root value.

## 7. Hashing, compression і CAS

### 7.1. BLAKE3 identity

~~~text
PDM value
  -> schema-defined semantic normalization
  -> deterministic PlatformEnvelope CBOR
  -> BLAKE3-256
  -> content revision ID
~~~

BLAKE3 default output — 256 bits. Специфікація має закріпити algorithm ID/version і exact bytes, що hash-яться. Hash не є signature, ACL, actor identity чи provenance proof. Для authenticity потрібні окремі signatures/MAC/key-management decisions.

Не змішувати:

1. **Entity ID** — стабільна логічна сутність.
2. **Schema ID/version** — semantics для decode.
3. **Revision/content ID** — digest exact profile-defined bytes.

### 7.2. iroh relationship

iroh-blobs описує blob як arbitrary bytes, а link як 32-byte BLAKE3 hash цього blob-а; protocol підтримує hash-verified streaming і range requests.

- Якщо iroh blob — raw deterministic CBOR envelope і hash input однаковий, link може бути platform content ID.
- Якщо blob — Zstd wrapper/package, iroh link ідентифікує compressed bytes; logical revision ID лишається окремим.
- Треба вирішити, чи blob є object, block, package чи attachment.
- Blob address не замінює entity ID, schema ID, signature або access control.
- Перш ніж фіксувати dependency, перевірити pin-нутий release, API та production-readiness; iroh API й release status можуть змінюватися.

### 7.3. Zstandard policy

Zstd є lossless compression, але compressed bytes не є canonical platform representation.

- Не стискати кожне коротке sync operation окремо, доки немає виміряного виграшу.
- Стискати block/package після вибору break-even size.
- Wrapper має містити codec, uncompressed length, profile version і independent decode boundaries.
- Hash-ити canonical payload/package envelope до compression для identity, незалежної від compressor version.
- Перевіряти decompressed size limits та повторний hash після розпакування.
- Якщо Zstd використовується як HTTP content coding, виконувати RFC 9659: encoder/decoder window size не більше 8 MB згідно з RFC для interoperability та обмеження пам’яті.
- Compression settings належать storage/transport profile, не schema semantics.
- Dictionary compression тестувати окремо з versioned dictionary ID і offline availability rules.

## 8. Файли та database projections

### 8.1. Source package

~~~text
application/
  manifest.json
  schemas/
    task.json
    page.json
  data/                 # optional readable fixtures
  blobs/                # binary attachments outside structured payloads
~~~

Portable/runtime package може містити versioned manifest, schema artifacts/refs, deterministic CBOR blocks і blobs. JSON зручний для diff/review; CBOR — для canonical interchange/storage. YAML не додавати без потреби й визначеного conversion contract.

Standalone CBOR item має містити schema binding або manifest reference, що доступний offline. Homogeneous block може задати schema context один раз.

### 8.2. SQLite

Приклад змішаного relational/payload layout:

~~~text
tasks(
  id,
  workspace_id,
  status,
  priority,
  due_at,
  assignee_id,
  revision_id,
  schema_version,
  dynamic_payload BLOB
)
~~~

- Часто query-вані fields — indexed typed columns.
- CBOR BLOB доречний для dynamic remainder, snapshots, versioned payload чи history.
- Не ховати всі query-вані дані в одному opaque BLOB.
- SQLite JSONB є внутрішнім SQLite JSON representation і не є portable interchange format чи Platform CBOR.
- SQLite physical schema migration незалежна від PDM schema migration.

### 8.3. Серверна SQL база

Server adapter проєктує PDM у relational columns для filters, joins, search, permissions і transactions. Dynamic fields можуть лишатися CBOR payload-ом. JSON/JSONB projection допустима як похідний query view, але не як canonical revision bytes.

~~~text
SQL rows <-> server adapter <-> Platform Data Model <-> CBOR / JSON
~~~

API/export мають зберігати PDM semantics, навіть якщо storage schema SQLite/PostgreSQL відрізняється.

## 9. Sync та транспорт

CBOR визначає serialization повідомлення, але не sync semantics. Окремо спроєктувати:

- operation IDs, idempotency і entity/workspace scope;
- base revision, actor/device identity, authorization;
- causal order/clock model; timestamp сам по собі не доводить causal order;
- deduplication, retry, tombstones і apply ordering;
- conflict detection/resolution та merge semantics;
- snapshot/checkpoint cadence, retention, ack/compaction;
- schema negotiation/migrations;
- authentication/signature and key lifecycle.

CRDT потрібен лише для типів, де потрібне concurrent merge із формально визначеною поведінкою. iroh можна оцінити як peer-to-peer network/blob transport; identity, authorization, conflict policy й key management залишаються платформними.

## 10. Роль технологій після рішення

| Компонент | Пропонована роль |
|---|---|
| Platform Data Model | Канонічна семантика application objects |
| Platform Schema Registry | Stable schema/field IDs, versions, defaults, compatibility, migrations |
| Platform CBOR Profile | Deterministic canonical binary representation |
| JSON | Source editing, code review, diagnostics, import/export |
| Amazon Ion | Optional interoperability; повторний bulk/read-sparse benchmark |
| MessagePack | Optional RPC/cache/hot path після доведеного performance win |
| Zstandard | Block/package compression після break-even test |
| BLAKE3 | Content/revision hash від profile-defined bytes |
| SQLite | Локальна БД з індексованими columns і вибірковими CBOR payloads |
| Server SQL | Relational projections, transactions, indexes |
| Blob/CAS | Великі файли й immutable package data |
| iroh-blobs | Candidate verified content-addressed transfer |
| CRDT/event log | Окремий change/conflict layer, якщо цього вимагає продукт |

## 11. Ризики й відкриті питання

### Ризики

1. Canonicalization ще не визначена для decimal, Unicode, defaults, missing/null, float і timestamps.
2. Offline decoding залежить від доступності точної schema version.
3. Перевикористання field/enum ID може непомітно змінити старі дані.
4. Schema metadata overhead може перекрити економію для коротких payload-ів.
5. Profile/schema changes змінюють content hashes.
6. Hash compressed package відрізняється від hash canonical payload.
7. SQLite та серверні DB мають різні decimal/date/bigint/JSON semantics.
8. Blind ignore unknown fields руйнує round-trip; безмежний preserve режим може спричинити resource exhaustion.
9. Huge lengths, deep nesting, malformed tags і decompression bombs потребують parser limits.
10. Ion .NET library archived/deprecated; використання потребує maintenance strategy.
11. iroh APIs й production-readiness можуть дрейфувати; pin-нути release і protocol version.
12. Форматні labels однакові не гарантують однакові defaults у різних encoder-ах.

### До затвердження v1 вирішити

- Schema ID namespace/width та registry authority.
- Field ID allocation ranges, reserved IDs, enum IDs, tombstones.
- Envelope wire form і schema-context amortization.
- CBOR tag allowlist та custom tag policy.
- Decimal scale, Money/currency, BigInt JSON form і float constraints.
- Date, Instant, LocalDateTime, ZonedDateTime й Duration representations.
- UUID byte order, Bytes JSON form, Unicode normalization.
- Absent/null/default semantics і unknown field preserve/strict policy.
- Zstd block size, threshold, dictionary strategy, random/range read needs.
- Hash algorithm agility й те, чи schemaVersion включено до exact preimage.
- Raw/compressed bytes для iroh, server DB projection ownership.
- Sync operation model, conflict resolution, retention та security model.
- Мінімальні Rust/.NET/Web targets і version support policy.

## 12. План наступної перевірки

### A. Відтворюваний benchmark harness

У новому проєкті зберегти generator, fixed seeds, dataset, lockfiles, commands, hardware/runtime facts, raw results і licenses. Включити realistic + edge-case data: українські/ASCII строки, UUID, tags, large schema, 1k/100k records, independent operations, Decimal/Money, BigInt, timestamps, null/absent/default, unknown fields і blobs.

Порівняти для однакових PDM values:

- minified JSON, JSON+Zstd;
- generic CBOR text keys, CBOR numeric field IDs;
- tuple CBOR як окремий future profile;
- MessagePack text/numeric keys + extensions;
- Ion Text, Binary local symbols, Binary shared symbols;
- schema/context bytes amortized per object, block і package;
- isolated payload, homogeneous batch і package compression.

### B. Виміряти не тільки bytes

- raw/compressed bytes;
- encode/decode throughput, median і tail latency;
- allocations, peak memory, CPU, binary size для Rust/.NET/JS/WASM;
- cold-start, append/stream parse, skip/partial read, random access;
- SQLite/server query, index і transaction behavior;
- BLAKE3 throughput, dedup rate;
- фактичні network bytes з framing, encryption, retries та iroh;
- differential output across library versions.

### C. Conformance/security

- Golden vectors: PDM input + schema version → exact CBOR bytes + exact BLAKE3 digest.
- Вивести ті самі bytes з Rust/.NET/TypeScript і взаємно decode.
- Negative vectors: duplicate keys, non-shortest ints, indefinite lengths, invalid UTF-8, unknown tags, overflow, trailing bytes, truncated data, disallowed NaN.
- JSON↔PDM↔CBOR semantic round-trip, unknown-field preserve, schema migrations.
- Fuzz decoders; caps на bytes, nesting depth, item count, text length, decompressed size і processing time.
- Перевірити, що схема не виконує code і не завантажується з неавторизованого URL.

### D. Критерії затвердження ADR

1. Типи, IDs, envelope, defaults/null/unknown і deterministic rules закріплені у versioned profile.
2. Є міжмовні golden vectors.
3. Reproducible benchmark є на representative corpus і дає size/runtime evidence.
4. SQL/local/server projections та migrations визначені.
5. Sync semantics відокремлені від serializer-у.
6. Hash/compression/iroh точно прив’язані до stored bytes.
7. Security limits і library versions визначені.

## 13. Рекомендована формула рішення

~~~text
Canonical logical model: Platform Data Model
Schema system: versioned Platform Schema Registry
Canonical binary: Platform CBOR Profile v1, numeric field IDs
Human-edited form: JSON source + explicit normalization
Content revision: BLAKE3-256 over profile-defined deterministic bytes
Compression: optional Zstandard at block/package level after measurement
Local storage: SQLite columns + selective CBOR BLOB payloads
Server storage: relational projections + optional dynamic CBOR payload
Large binary data: separate blob/CAS store referenced by ContentRef
Transport: CBOR messages; evaluate iroh-blobs for immutable blob transfer
Amazon Ion: optional interoperability and bulk/read-sparse candidate
MessagePack: optional runtime/RPC codec if benchmark proves material win
~~~

**Висновок:** schema-aware design, record granularity і compression boundaries важливіші за вибір CBOR проти MessagePack. Ion є сильною альтернативою для rich, symbol-heavy, batch-oriented data. Для ядра платформи наразі рекомендовано PDM поверх вузького deterministic CBOR profile, а не прив’язування моделі до одного serializer-а.

## 14. Первинні референси

Ключові специфікації та runtime/project references перевірені 2026-09-18. Перед pin-ning package versions перевірити releases і документацію повторно.

### CBOR

- [RFC 8949 — CBOR](https://www.rfc-editor.org/rfc/rfc8949.html) — data model, tags і deterministic encoding requirements.
- [IANA CBOR Tags registry](https://www.iana.org/assignments/cbor-tags) — зареєстровані semantic tags.
- [Microsoft System.Formats.Cbor API](https://learn.microsoft.com/en-us/dotnet/api/system.formats.cbor.cborreader?view=net-10.0) — .NET reader; перевірити target runtime/package та conformance mode.

### Amazon Ion

- [Amazon Ion specification](https://amazon-ion.github.io/ion-docs/docs/spec.html) — data model і text/binary semantics.
- [Ion Binary Encoding](https://amazon-ion.github.io/ion-docs/docs/binary.html) — value encodings, lengths і symbols.
- [Ion Symbols and Symbol Tables](https://amazon-ion.github.io/ion-docs/docs/symbols.html) — local/shared tables, imports, SIDs та IVM.
- [Ion Schema 2.0 Specification](https://amazon-ion.github.io/ion-schema/docs/isl-2-0/spec) — окремий schema language для Ion values.
- [Amazon Ion documentation index](https://amazon-ion.github.io/ion-docs/docs.html) — specification, guides, decimal, float і schemas.
- [Official ion-dotnet archive/security advisory](https://github.com/amazon-ion/ion-dotnet/security/advisories/GHSA-q5r6-9qwq-g2wj) — .NET deprecation/archive status та security maintenance details.

### MessagePack

- [MessagePack specification](https://github.com/msgpack/msgpack/blob/master/spec.md) — base types, extension types, timestamps і application profiles.

### Hash та compression

- [BLAKE3 official specification](https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.tex) — hash modes, tree definition, default output.
- [BLAKE3 official implementation and test vectors](https://github.com/BLAKE3-team/BLAKE3) — implementations, test vectors і API modes.
- [RFC 8878 — Zstandard](https://www.rfc-editor.org/rfc/rfc8878.html) — zstd framing та media type.
- [RFC 9659 — Window Sizing for Zstandard Content Encoding](https://www.rfc-editor.org/rfc/rfc9659.html) — оновлює HTTP window-size рекомендацію з RFC 8878 до вимоги 8 MB.

### SQLite

- [SQLite Datatypes](https://www.sqlite.org/datatype3.html) — storage classes, BLOB, flexible typing і STRICT tables.
- [SQLite JSON functions and JSONB](https://www.sqlite.org/json1.html) — JSON support і приватний SQLite JSONB; це не portable interchange format.

### iroh

- [iroh-blobs official repository](https://github.com/n0-computer/iroh-blobs) — BLAKE3 links, blobs і verified streaming; сторінка містить актуальне release/readiness note.
- [iroh official repository](https://github.com/n0-computer/iroh) — networking library та related components.
- [Blob store design challenges](https://www.iroh.computer/blog/blob-store-design-challenges) — design context для BLAKE3 blobs і verified streaming.

## 15. Походження й межі доказів

Synthetic benchmark results і початкові рекомендації перенесено з попередньої розмови **«Дослідження форматів зберігання»**. Harness і raw synthetic input у поточному project mirror відсутні, тому результати позначені як раніше повідомлені, а не як незалежно відтворені в цій роботі.

Першоджерела в розділі 14 перевірено онлайн 2026-09-18. Правила schema lifecycle, profile, DB layout, hashing і test plan — архітектурні пропозиції на основі джерел і вимог платформи; вони не є вимогами, які вже нав’язують CBOR, Ion, MessagePack, SQLite, BLAKE3, Zstandard чи iroh.
