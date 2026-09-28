---
id: REQ-CONTACT-001
status: approved
last_updated: 2026-09-24
source_refs:
  - ../../_bmad-output/planning-artifacts/research/technical-vida-contact-cards-and-mobile-store-reso-2026-09-22/research.md
decision_refs:
  - OQ-0074
---

# Контактна картка та адресні книги

## Прийнята межа

Загальний імпорт нотаток, задач, чатів і файлів не входить у поточний scope. Контакти є обов'язковим Core service: користувач працює з контактною карткою VIDA, створеною вручну або пов'язаною із системною/хмарною адресною книгою. First-party Contacts view може відображати Core service, але Contacts не є optional AppPackage, який можна прибрати з Core.

У Release 1 імпорт системних контактів спрямований до Personal Space. ContactCard, створена всередині робочого Space, лишається окремим ресурсом цього Space; Space member і ContactCard — різні поняття. Явне копіювання між Spaces показує поля перед дією, створює нову картку з іншим VIDA ID і не вмикає автоматичне оновлення копії. У Shared Space картка видима учасникам із доступом до відповідного App/Project scope; приватної картки автора за замовчуванням немає. Export, двостороння provider-синхронізація та `.vcf` file interchange нижче описані як майбутні connector/interchange contracts, а не як обов'язкові Release-1 можливості.

| ID | Вимога |
|---|---|
| `REQ-CONTACT-001` | VIDA `MUST` мати власний canonical `ContactCard` зі stable VIDA ID. Android/iOS/Windows/Google record `MUST NOT` бути canonical identity або єдиним source of truth. |
| `REQ-CONTACT-002` | Картка `MUST` підтримувати ім'я/відображуване ім'я, організацію/посаду, телефони, email, адреси, фото, нотатки/мітки та зв'язки; поля `MUST` мати provenance і visibility. |
| `REQ-CONTACT-003` | `identity_bindings[]` `MUST` дозволяти прив'язати до картки кілька VIDA-ідентифікаторів різних режимів: anonymous, private/personal, public і federated. Binding `MUST` містити type/scheme, identifier, Persona/context, verification state, source і visibility. |
| `REQ-CONTACT-004` | VIDA Core `MUST NOT` дедублікувати, автоматично зливати або пропонувати merge контактних карток на основі телефона, email, імені, profile URL чи platform record. Кожна отримана/створена картка зберігає власний ID; явне ручне редагування користувача не є окремою deduplication subsystem. |
| `REQ-CONTACT-005` | У Release 1 системний connector `MUST` підтримувати явний `read/import to VIDA` у Personal Space з мінімальними permissions і можливістю відключення. Майбутні `export from VIDA` і `two-way sync` потребують окремого явного вибору та preview перед записом назовні. Permission denial не блокує локальні VIDA Contacts. |
| `REQ-CONTACT-006` | Connector metadata `MUST` зберігати provider/account, external record ID, version/ETag/change token, sync cursor, last sync і field mapping окремо від canonical card. Provider token expiry `MUST` запускати безпечний full reconcile, а не втрату картки. |
| `REQ-CONTACT-007` | Android adapter `MAY` публікувати VIDA-owned raw contacts/custom MIME data через власний account/sync adapter. Він `MUST NOT` записувати VIDA-only поля у raw contact іншого provider account, якщо той provider не гарантує round-trip. |
| `REQ-CONTACT-008` | Apple adapter `MUST` використовувати лише підтримувані Contacts fields і limited/full authorization, не припускаючи round-trip довільних VIDA fields. Windows adapter `MUST` використовувати підтримувані ContactStore/annotation capabilities з тим самим обмеженням. |
| `REQ-CONTACT-009` | Якщо в майбутньому додається Google Contacts sync, він `MUST` використовувати офіційний consented People API/CardDAV profile, ETag/version semantics і documented incremental/full reconciliation; запис у локальний Android Contacts Provider сам по собі не доводить Google-cloud sync. Google-cloud sync не є вимогою Release 1. |
| `REQ-CONTACT-010` | vCard 4.0 `MUST` бути baseline field-mapping format для майбутнього interchange сумісних полів; `.vcf` import/export `MUST NOT` вважатися Release-1 feature. Anonymous/private VIDA bindings `MUST` лишатися локальними й `MUST NOT` потрапляти у system contact, vCard чи platform card. VIDA `MUST NOT` створювати або доповнювати device contact нестандартними полями, яких не підтримує цільовий standard/provider. CardDAV `MAY` бути transport connector, але не canonical VIDA sync protocol. |
| `REQ-CONTACT-016` | ContactCard у Shared Space `MUST` бути доступною чинним учасникам із правом читання відповідного App/Project scope; створення картки учасником `MUST NOT` робити її приватною лише для автора. Особистий коментар або per-user видимість `MAY` бути властивістю окремого App/resource schema з перевірним ACL, але `MUST NOT` мовчки змінювати базову видимість ContactCard/Notes. |
| `REQ-CONTACT-011` | Sharing картки `MUST` бути field- і binding-scoped. Поширення телефонної картки не відкриває автоматично anonymous/private/federated identifiers; UI `MUST` показати preview фактичних полів і bindings. |
| `REQ-CONTACT-012` | Видалення connector link `MUST` відокремлюватися від видалення canonical card і від видалення provider record. Кожна дія `MUST` явно показувати, де саме буде зміна. |
| `REQ-CONTACT-013` | Sharing `MUST` пропонувати два явно названі режими: immutable snapshot і live share. Snapshot `MUST` бути default; зміни owner card після надсилання snapshot `MUST NOT` змінювати отриману копію. |
| `REQ-CONTACT-014` | Live share `MUST` синхронізувати лише дозволені поля/bindings до revoke. Одержувач `MUST NOT` редагувати canonical owner card; власні локальні notes/tags одержувача `MUST` зберігатися окремо й `MUST NOT` повертатися власнику як зміни картки. |
| `REQ-CONTACT-015` | Перше підключення connector `MUST` за замовчуванням бути `read/import to VIDA`. Майбутні `export from VIDA` і `two-way sync` `MUST` вимагати окремого явного вибору користувача та preview перед зовнішнім записом; їх не слід вважати вже включеними в Release 1. |

## OWASP gate

- Контакти є персональними даними; permission і collection flows `MUST` перевірятися за [OWASP MASVS-PRIVACY-1](https://mas.owasp.org/MASVS/controls/MASVS-PRIVACY-1/) та [MASWE-0066](https://mas.owasp.org/MASWE/MASVS-PRIVACY/MASWE-0066/).
- Tests `MUST` покривати denial/revocation, partial contact access, team-visible card versus denied App/Project scope, accidental export приватного binding, cross-Persona correlation, provider token expiry, concurrent provider edit і відсутність автоматичної дедублікації.

## Implementation gate

- Точний UI, platform permission mapping і conflict behavior для `read`, `export` та `two-way` режимів кожного connector мають бути доведені platform-specific prototypes і conformance tests.
