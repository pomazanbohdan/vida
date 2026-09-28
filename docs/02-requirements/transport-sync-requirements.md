---
id: REQ-TRANSPORT-SYNC-001
status: approved
last_updated: 2026-09-20
source_refs:
  - ../../_bmad-output/planning-artifacts/research/technical-iroh-ecosystem-and-project-patterns-for-2026-09-18/research.md
decision_refs:
  - ../03-architecture/decisions/ADR-0005-iroh-transport-foundation.md
  - ../03-architecture/decisions/ADR-0006-durable-delivery-and-operation-envelope.md
  - ../03-architecture/decisions/ADR-0016-equal-device-peers.md
---

# Вимоги до transport, delivery та synchronization

## Transport

- `REQ-TR-001`: VIDA `MUST` використовувати Iroh core 1.2 як primary transport foundation.
- `REQ-TR-002`: `VidaNodeHost` `MUST` бути єдиним owner Iroh Router, Address Lookup, relay policy та endpoint lifecycle в application process.
- `REQ-TR-003`: domain modules `MUST NOT` напряму залежати від Iroh crates.
- `REQ-TR-004`: кожен VIDA protocol `MUST` мати versioned ALPN і VIDA-owned envelope.
- `REQ-TR-005`: `EndpointId` `MUST NOT` бути Persona, role або authorization grant.
- `REQ-TR-006`: anonymous/session contexts `MUST` використовувати isolated ephemeral endpoints у межах policy-defined unlinkability scope; persistent endpoint у такому context заборонений.

## Durable delivery

- `REQ-DD-001`: operation, що отримала `delivery.accepted`, `MUST` бути записана до local log і outbox атомарно.
- `REQ-DD-002`: direct та mailbox paths `MUST` переносити той самий operation ID/envelope.
- `REQ-DD-003`: receiver `MUST` deduplicate та idempotently apply повторну delivery.
- `REQ-DD-004`: delivery ACK states `MUST` розрізняти `delivery.accepted`, `delivery.stored`, `delivery.applied`, `delivery.read`; `delivery.accepted` `MUST NOT` означати `authority.accepted`.
- `REQ-DD-005`: durable node `MUST` зберігати E2E ciphertext і `MUST NOT` отримувати domain authority з факту зберігання.
- `REQ-DD-006`: reconnect `MUST` запускати pull/reconcile, а не покладатися на replay transient events.

## Membership and revocation

- `REQ-MEM-001`: Space policy `MUST` бути джерелом roles/capabilities; transport connection `MUST NOT` grant access.
- `REQ-MEM-002`: device/member removal `MUST` advance affected future-access epoch.
- `REQ-MEM-003`: product `MUST NOT` обіцяти erasure plaintext, already replicated before revocation.
- `REQ-MEM-004`: concurrent grants/revokes `MUST` мати deterministic ordering/resolution contract before implementation lock.

## Sync and blobs

- `REQ-SYNC-001`: durable state `MUST` converge from signed operations and causal repair.
- `REQ-SYNC-002`: projections/indexes `MUST` be rebuildable or verifiably derived from authoritative state.
- `REQ-SYNC-003`: large immutable payloads `MUST` be transferred separately from operation log through verified resumable chunks.

У `REQ-SYNC-004/007` «непорівнювані» означає, що **authority acceptances** несумісних гілок не мають спільного перевірного порядку. Конкурентне створення самих operations не викликає multi-value conflict, якщо їхнє подальше authority acceptance впорядковане; тоді діє правило першого прийняття. Невалідні/revoked pending candidates не стають прийнятими variants лише через delivery.
- `REQ-SYNC-004`: для двох несумісних змін статусу однієї задачі з того самого base, якщо їх прийняття **мають спільний перевірний порядок**, перша успішно прийнята зміна стає спільною; client timestamp, отримання пакета, `delivery.accepted` і локальний durable commit не встановлюють цей порядок. Інший несумісний намір `MUST NOT` непомітно перезаписувати поточний стан: його автор бачить обидва варіанти та за чинного права може прийняти спільне або подати своє як **нову** операцію від нього після перевірки прав/preconditions. Його локальний намір лишається recoverable без доступу до даних для відкликаного актора. Якщо дві несумісні зміни непорівнювані, peers після reconciliation `MUST` показати явний невирішений multi-value conflict з обома variants, без поточного winner; жоден пристрій `MUST NOT` заявляти глобальне «перший sync». Нова операція вирішення `MUST` причинно врахувати обидві гілки та пройти чинну перевірку прав. Proof acceptance, wire format і race двох рішень — `OQ-0033`/`OQ-0034`. Це не є merge-моделлю тексту, коду, лічильників, повідомлень чи бронювань.
- `REQ-SYNC-005`: контракт кожного типу ресурсу та сімейства операцій `MUST` визначати, як знайти його authority для остаточного прийняття зміни: це може бути локальний Owner Personal Space, authority спільного Space або власник зовнішнього ресурсу/сервісу. Фізичне зберігання, transport delivery і federated identity binding самі по собі `MUST NOT` призначати authority. Формат mapping, receipt і поведінка недоступного authority лишаються `OQ-0033`.
- `REQ-SYNC-006`: конфлікти структурованих даних, тексту й коду `MUST` визначатися на рівні застосовного типу ресурсу та незалежних змін, а не автоматично як заміна всього файла або застосування `REQ-SYNC-004` до кожного символу. Незалежні редагування, зокрема різних абзаців, `MUST` поєднуватися автоматично без втрати жодного. Для несумісних змін того самого логічного фрагмента, зокрема речення нотатки, діє погоджене правило: за спільного перевірного порядку **authority acceptance** перша прийнята зміна є поточною; пізніший несумісний намір зберігається для порівняння, а його автор може прийняти поточне або, за чинного права, подати своє як нову зміну від поточного стану. Якщо належно прийняті гілки не мають спільного перевірного порядку, жодна не стає майстром: обидва варіанти утворюють явний Conflict до нового авторизованого рішення, що причинно враховує обидві гілки. Client timestamp, Device ID і порядок доставки не доводять прийняття. Точна гранулярність, бібліотека merge, proof/wire та UX — `OQ-0033`/`OQ-0034`.
- `REQ-SYNC-007`: для несумісних змін одного значення/версії, де schema operation family не допускає безпечного автоматичного merge, перша прийнята версія лишається поточною **лише за перевірного спільного порядку**. Непорівнювані конкурентні версії `MUST` зберігатися без втрат як conflict variants. Для заміни файла після reconciliation `MUST` відображатися невирішений multi-value conflict з обома версіями, без автоматично поточної; уповноважена людина може вирішити його новою операцією, що причинно враховує обидві гілки, або зберегти підтримувану копію/ревізію. Для інших operation families відображення ще відкрите. Це `MUST NOT` підміняти operation-specific merge, безповоротну зовнішню дію чи ownership. Відповідність інших класів схем, wire model, resolution races і семантика копії/ревізії — `OQ-0034`.
- `REQ-SYNC-008`: всі авторизовані пристрої однієї Persona `MUST` бути рівноправними peers для sync/merge. Owner/Admin — ролі учасника Space, не пристрою. Пристрій, replica ID, Iroh endpoint або порядок connect/ACK `MUST NOT` давати постійний пріоритет, право арбітражу чи інші бізнес-права. При однакових валідних data operations **і однаковому перевіреному control/acceptance state** різні платформи `MUST` отримувати однаковий результат і збережені альтернативи незалежно від порядку доставки. Офлайн-пристрій може локально підготувати candidate, але не вважати неперевірений grant чинним proof; default winner за replica ID не є затвердженим бізнес-правилом (`ADR-0016`).
- `REQ-SYNC-009`: якщо два авторизовані вирішення одного status/file conflict з несумісними результатами були належно прийняті офлайн, але їхні authority acceptances непорівнювані, після reconciliation `MUST` з'явитися новий явний multi-value conflict між цими рішеннями без автоматичного winner. Обидва рішення та потрібні payloads `MUST` залишатися recoverable у межах чинних прав/retention; наступна resolution operation `MUST` врахувати обидва актуальні heads і пройти повторну перевірку прав/preconditions. Порівнювані прийняття підпадають під `REQ-SYNC-004/007`. Точні proof/wire та provisional UX лишаються `OQ-0033`/`OQ-0034`.
- `REQ-SYNC-010`: два належно прийняті рішення того самого conflict з доведено однаковим канонічним бізнес-результатом і наслідками `MUST` давати один видимий результат після reconciliation, зберігаючи обидві operations, авторство та causal/audit history. Дубль зовнішнього наслідку відтворення історії заборонений. Якщо еквівалентність не доведена, `MUST NOT` тихо coalesce їх; точний equivalence predicate — `OQ-0034`.
- `REQ-SYNC-011`: належно прийнята несумісна гілка, яка з'явилася після рішення, не входила до його causal base **і має непорівнюване щодо рішення authority acceptance**, `MUST` повторно відкрити явний conflict без автоматичного winner. За порівнюваного acceptance діє правило першого прийняття. Старе рішення та пізня гілка залишаються доступними за чинними правами; invalid/revoked candidate не стає accepted branch лише через delivery.
- `REQ-SYNC-012`: для рішення conflict actor `MUST` мати чинне право на відповідну дію **і** читання всіх variants, потрібних для осмисленого вибору. За відсутності права читання хоча б одного variant клієнт `MUST NOT` розкривати його вміст чи приймати рішення від цього actor; може показати безпечний сигнал про потребу уповноваженого reviewer. Перевірка повторюється при authority acceptance.
- `REQ-SYNC-013`: для альтернативної версії файла клієнт `MUST` пропонувати окремі явно названі дії: ревізія того самого resource та копія як новий resource із власним ID. Обидві дії `MUST` перевіряти чинні права й не переписувати основний файл мовчки; precise metadata/ACL inheritance і lifecycle — `OQ-0034`.
- `REQ-BLOB-001`: blob manifest `MUST` bind digest, encryption/key reference, size/chunks and authorization context.
- `REQ-BLOB-002`: pin/retention/GC ownership `MUST` be explicit; reference deletion alone `MUST NOT` silently promise physical erasure.
- `REQ-BLOB-003`: локальне вирішення conflict на одному peer `MUST NOT` саме по собі дозволяти unpin/GC альтернативного file payload, який може знадобитися для пізньої правомірної гілки. Потрібні безпечний retention frontier і помітне повідомлення про використання місця; точний proof, строки та межі — `OQ-0034`.

## Deferred requirements

Mailbox replication factor/SLO, operation wire schema, blob hash/encryption scheme, Address Lookup provider, persistence engine and performance thresholds require prototype evidence.

Незалежна реалізація відповідного seam `MUST NOT` заявляти conformance, доки її deferred wire/security decision не закрито в `OQ-0028`–`OQ-0032` та не опубліковано golden/conformance fixtures.
