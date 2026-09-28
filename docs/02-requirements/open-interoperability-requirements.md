---
id: REQ-OPEN-001
status: approved
last_updated: 2026-09-20
source_refs:
  - ../03-architecture/decisions/ADR-0013-open-interoperability.md
  - ../../_bmad-output/planning-artifacts/research/technical-vida-open-protocol-specifications-2026-09-20/research.md
decision_refs:
  - ADR-0005
  - ADR-0013
---

# Вимоги до відкритої міжоперабельності VIDA

Прийнята ціль — незалежні реалізації **обох** контурів: клієнтів/вузлів і schema-driven застосунків/керованих плагінів. «Плагін» тут — кероване розширення ADR-0007–0009, не довільний код із доступом до ОС. «Прозорість» стосується нормативних контрактів, сумісності й доказів, а не публікації приватних ключів, користувацьких даних або автоматичного відкриття всіх платних реалізацій.

| ID | Вимога | Перевірка |
|---|---|---|
| REQ-OPEN-001 | Кожен заявлений interoperable VIDA protocol profile `MUST` мати публічну версійну специфікацію з повним списком обов'язкових capabilities і залежностей, достатню для незалежного клієнта і вузла. Відкритий baseline `MUST NOT` мати прихованої чи платної обов'язкової залежності. | Два незалежні розробники реалізують клієнт і вузол й успішно взаємодіють між собою та з офіційними реалізаціями без приватних контрактів. |
| REQ-OPEN-002 | Публічний контракт `MUST` описувати wire bytes/canonical signing, capability/version negotiation, identity/authority, операції й стани, errors, retry/idempotency і security/privacy boundaries, якщо вони належать цьому profile. | Golden і negative-security fixtures відтворюються стороннім інструментом; прогалини оформлені як явні implementation gates, не замасковані твердженням про готовність. |
| REQ-OPEN-003 | App/managed-extension package format, schema registry, repository/update metadata, hook/effect/entitlement semantics та дозволені runtime/host APIs `MUST` бути публічно специфіковані й однакові для bundled і зовнішніх пакетів у межах заявленого profile. | Сторонній пакет з іншого репозиторію проходить той самий набір conformance-тестів і запускається без приватного SDK; Space grants, publisher trust і network egress не виникають від самого встановлення або успішного conformance. |
| REQ-OPEN-004 | Машиночитані схеми, byte-exact і behavior vectors, negative-security tests, suite versions та результати офіційних релізів `MUST` бути публічними. Жодне приховане обов'язкове розширення `MUST NOT` визначати базову заявлену сумісність. Claim сумісності `MUST` називати конкретний versioned profile, а не всю VIDA безмежно. | Стороння команда без платних VIDA credentials запускає опублікований suite; release manifest указує immutable digest специфікації, profile й suite; cross-vendor pairwise tests проходять. |
| REQ-OPEN-005 | Для змін відкритих контрактів `MUST` існувати публічна історія proposal/review/status, migration і deprecation. `MUST` бути задокументований спосіб запропонувати зміну ззовні. | За номером/версією контракту знаходяться rationale, статус, сумісність, тести та попередні редакції. |
| REQ-OPEN-006 | vida-core та інший заявлений open-source code `MUST` публікуватися під MIT License, що дозволяє незалежне й комерційне використання/fork/hosting із дотриманням її notice conditions. Нормативні тексти, schemas/fixtures, conformance suite, reference assets, trademarks, patent/IPR і contribution governance `MUST` отримати окремі сумісні правила через `OQ-0055`. | Release містить MIT LICENSE і notices для code; non-code artifacts мають явні licenses; юридична перевірка завершується до production claim відкритої сумісності. |
| REQ-OPEN-007 | Відкритий baseline `MUST` дозволяти fork клієнта/App і продовження роботи з уже створеними VIDA data без активного VIDA commercial service. Portable encrypted package `MUST` включати versioned schemas, resources, relations, files/blobs, identity/key-recovery metadata, необхідні для незалежного сумісного reader/importer, у межах прав експортера. Public schemas, export і незалежний reference reader `MUST` бути release gate v1. | Чисте середовище без VIDA account відкриває test export незалежним reference client; encrypted data не обходить key ownership/recovery. |
| REQ-OPEN-008 | Публічна специфікація `MUST` чітко відрізняти open baseline від optional hosted/premium services. Платний relay, mailbox, backup, blob storage або hosted replica `MUST NOT` бути прихованою залежністю baseline protocol чи формату даних. | Conformance suite запускається без paid credentials; manifest явно називає optional proprietary extensions. |

Формат пропозицій (NIP-подібний Markdown, MSC-подібний процес), schema language (CDDL/JSON Schema/Protobuf), Nostr bridge та порядок публікації ще не затверджені. Існуючі `OQ-0028` (wire envelope) та `OQ-0055` (open governance/IPR) залишаються воротами реалізації; це не змінює ADR-0005 про Iroh.
