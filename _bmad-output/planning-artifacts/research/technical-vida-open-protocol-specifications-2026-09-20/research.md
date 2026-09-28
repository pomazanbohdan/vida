# VIDA: відкриті специфікації чи Nostr NIP?

2026-09-20 · technical research · **доказова база, а не нормативний контракт**. Подальше рішення користувача про відкритість обох контурів зафіксоване в [ADR-0013](../../../../docs/03-architecture/decisions/ADR-0013-open-interoperability.md); формат, ліцензія й governance лишаються відкритими. Контекст: задача Codex «Дослідити стандарти Nostr» (`codex://threads/01a0b62d-c65a-7ab1-a1df-f20e96da28c2`), поточні [ADR-0005](../../../../docs/03-architecture/decisions/ADR-0005-iroh-transport-foundation.md) та `AD-2` в архітектурному spine. Першою лінією доказів є офіційні специфікації нижче; попередню відповідь Codex не вважаємо нормативним джерелом.

## Висновок для рішення

**Дослідницька рекомендація:** описувати відкриті VIDA-протоколи як власний набір версійних специфікацій поверх Iroh і дозволити незалежну реалізацію; використати модель NIP/MSC для процесу пропозицій, а машинно-читані схеми і перевірні вектори — для реалізацій. Це не означає винаходити новий транспорт чи заборонити сумісність із Nostr. Nostr-міст для окремих публічних сценаріїв досліджувати як факультативний адаптер, не як заміну authority, offline sync або приватного Space VIDA. Користувач згодом затвердив відкритість і незалежну реалізацію **і** клієнтів/вузлів, **і** Apps/плагінів, але не затвердив NIP/MSC-формат чи конкретний Nostr bridge.

## Що саме описують NIP і `kind`

NIP — нумерований документ *Nostr Implementation Possibilities*; список не є обов'язковим чеклістом, а сумісне ПЗ обирає підмножину. Наприклад, [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md) описує підписану JSON-подію і потік client↔relay; поле `kind` у події позначає її семантичний тип. Номер NIP і значення `kind` — різні простори імен. Репозиторій [NIPs](https://github.com/nostr-protocol/nips) пропонує для прийняття, де доречно, дві реалізації клієнта й одну relay, optional/backwards-compatible зміни й відсутність дубльованих способів тієї самої дії. Документи публічні й оголошені public domain; важливо окремо визначити ліцензію VIDA та права на внески, а не просто копіювати цей статус.

[NIP-29](https://github.com/nostr-protocol/nips/blob/master/29.md) (draft/optional) групи спираються на конкретний relay, що застосовує правила, і не визначає єдину семантику ролей між relay; [NIP-78](https://github.com/nostr-protocol/nips/blob/master/78.md) (draft/optional) дає app-specific **приватні** події з довільним content/tags, але прямо не призначений для публічного міжаплікаційного обміну. Це корисні референси для певних окремих сценаріїв, але самі не задають узгодження власника Space, key epochs, офлайн-операцій та authoritative receipts VIDA. Висновок про неповну відповідність — архітектурне зіставлення, не твердження про якість Nostr. У поточному VIDA [ADR-0005](../../../../docs/03-architecture/decisions/ADR-0005-iroh-transport-foundation.md) вже ухвалені власний ALPN `vida/<capability>/<major>` та envelope над Iroh. Зміна на Nostr wire/relay модель означала б нове архітектурне рішення та міграцію, не просто публікацію документації.

## Який «формат специфікації» обрати

| Підхід | Дає | Межа / вартість | Для VIDA |
|---|---|---|---|
| NIP-style Markdown з нумерацією | Публічні обговорення, короткий посилальний документ, вузькі optional розширення | Проза сама не забезпечує валідатор, точні байти чи узгоджений authority | Процес пропозицій, статуси draft/accepted/superseded |
| [Matrix MSC](https://spec.matrix.org/proposals/) + канонічна spec | Окремі proposals і інтегрована специфікація; потрібний доказ реалізації; unstable vendor prefixes до стабілізації | Більша дисципліна й підтримка редакції; ризик уповільнення маленьких змін | Добрий приклад життєвого циклу і тестів незалежних клієнтів |
| [AT Lexicon](https://atproto.com/specs/lexicon) | JSON-схеми для records, RPC та streams, NSID, правила еволюції | Прив'язка до atproto data model/XRPC; не готовий VIDA authority чи Iroh ALPN контракт | Запозичити принцип schema registry і namespace, не обов'язково Lexicon runtime |
| [CDDL](https://www.rfc-editor.org/rfc/rfc8610.html) + [CBOR](https://www.rfc-editor.org/rfc/rfc8949) | Формальні структурні правила компактних CBOR/JSON повідомлень | Не описує порядок, ACL, quorum, idempotency, шифрування чи retry | Кандидат для wire envelope/operation, якщо буде прийнято CBOR |
| [JSON Schema](https://json-schema.org/specification) + [OpenAPI 3.2.1](https://spec.openapis.org/oas/v3.2.1.html) | Валідовані JSON-дані та HTTP API, широкий tooling | JSON-канонізація для підпису потребує окремого правила (напр. [JCS](https://www.rfc-editor.org/rfc/rfc8785)); OpenAPI не описує P2P/QUIC автомати | Маніфести App/плагіна, каталоги, HTTP gateway; не є повною wire spec |
| [Protocol Buffers](https://protobuf.dev/programming-guides/proto3/) | Генерація типів і compact binary, відомі правила зміни полів | Field numbers стають довічними; ProtoJSON відрізняється; wire-safe не означає semantic-safe | Можливий кандидат transport messages, не приймати без binary/FFI test vectors |

Форма документа й кодування — незалежні рішення: один proposal може нормативно визначати семантику, мати CDDL/JSON Schema, приклади bytes і conformance-тести. Не потрібно вводити власну мову опису схем тільки заради відкритості.

## Що справді забезпечує відкритість

Публічний вихідний код сам по собі не гарантує, що сторонній клієнт може під'єднатися й коректно відтворити стан. Потрібні: (1) публічний незмінний реліз специфікації та історія змін; (2) явна вільна ліцензія на тексти/схеми/тести та чітка policy внесків/IPR; (3) описані wire bytes, canonical signing, ідентифікатори, authority/ACL, помилки, retry/idempotency, version negotiation; (4) machine-readable схеми, позитивні та негативні vectors, незалежні клієнт↔клієнт↔вузол тести; (5) сумісні правила розширень і міграцій; (6) публічний процес внесків, статуси draft/accepted/deprecated та незалежна перевірка сторонньої реалізації. Це **рекомендований набір критеріїв**, синтез [NIPs](https://github.com/nostr-protocol/nips), [Matrix](https://spec.matrix.org/proposals/), [Lexicon](https://atproto.com/specs/lexicon) і наявного `AD-2`, а не вимога одного зовнішнього стандарту. Така відкритість сумісна з платними hosting/marketplace/enterprise-функціями, але **не** з твердженням про повну незалежну сумісність, якщо базові протоколи, ключові capability negotiation або тести доступні лише під закритою ліцензією. Це продуктовий trade-off, не заборона монетизації.

Безпекова перевірка: [OWASP ASVS](https://owasp.org/projects/asvs) дає вимоги для перевірки контролів вебзастосунку, але не визначає інтервал читання offline-кешу чи VIDA quorum. Публічні протоколи мають тестувати unauthorized replay, old epoch, forged receipts, cross-Space reads, сторонній app package і конфлікт станів; не можна вважати публічність протоколу гарантією безпеки. Специфікація E2EE має відкривати формат/алгоритми, **не секрети** користувачів.

## Процес, який можна затвердити (приклад, не норма)

`VPS-0001` (VIDA Protocol Specification proposal) описує identity/envelope/ALPN; `VPS-0002` — authority receipt; `VPS-0003` — операції sync; окремий `VAP-0001` — App package/manifest. **Назви й номери лише приклади.** Proposal: motivation → use cases → protocol state machine/permissions/errors → schemas/wire examples/golden vectors → security/privacy → compatibility/migration → two незалежні реалізації або обґрунтований виняток → публічне прийняття → стабільний spec release. `experimental` розширення із namespace не можуть мовчки стати обов'язковими; versioned capability negotiation визначає, що може обробити вузол. Публічний Nostr bridge, якщо знадобиться, має чітко вказати lossiness, privacy boundary й окремий export/import, а не повторно використовувати підпис/ключі Persona без окремого рішення.

## Відкриті рішення

1. Що саме сторонній розробник повинен мати змогу зробити першою чергою: написати власний клієнт/вузол, власний schema-driven App/плагін чи обидва? Це визначає перший нормативний документ і conformance-набір.
2. Яке юридичне оформлення відкритості текстів, тестів, reference code і внесків; який процес передачі governance за межі однієї компанії? Юридичний висновок потрібен окремо.
3. Який wire codec і правило canonical signing після прототипу (CBOR+CDDL, protobuf чи інший)? Не змінювати затверджену архітектуру лише на підставі порівняння форматів.
4. Чи потрібна Nostr-сумісність для конкретного публічного сценарію, наприклад export публічного міського оголошення до relay, і що заборонено переносити із приватного Space?

## Межі дослідження

Офіційні джерела перевірено 2026-09-20; вони змінюються. Не проведено prototype interop чи незалежного юридичного аналізу. Nostr/Matrix/AT порівнюються як процеси й контракти, а не як кандидати замінити прийнятий Iroh без окремого ADR. Перший формат документа можна затвердити зараз, binary codec і governance — пізніше після proof.
