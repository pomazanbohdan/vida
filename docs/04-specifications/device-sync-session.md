---
id: SPEC-DEVICE-SYNC-SESSION-001
status: review
implementation_status: unplanned
last_updated: 2026-09-22
requirement_refs:
  - ../02-requirements/transport-sync-requirements.md
  - ../02-requirements/access-control-requirements.md
decision_refs:
  - ../03-architecture/decisions/ADR-0003-offline-revocation.md
  - ../03-architecture/decisions/ADR-0005-iroh-transport-foundation.md
  - ../03-architecture/decisions/ADR-0006-durable-delivery-and-operation-envelope.md
  - ../03-architecture/decisions/ADR-0016-equal-device-peers.md
---

# Пряма сесія синхронізації пристроїв: контур для рішення

Це **не затверджений wire protocol**. Нижче поєднано вже погоджені інваріанти VIDA з досліджуваною послідовністю прямої сесії. Пакет Iroh, запис у віддалене сховище та ACK реплікації не є самі собою прийняттям доменної зміни.

## Послідовність, яку має покрити протокол

1. **Локальний намір.** Core перевіряє доступний контрольний стан, schema та команду; формує підписану операцію з незмінним ID, causal/base frontier і контекстом Space. Після атомарного локального запису operation + outbox UI може показати «збережено локально», але не «прийнято всіма».
2. **З'єднання.** `VidaNodeHost` встановлює Iroh-сесію з versioned VIDA ALPN; обидві сторони пов'язують транспортний ключ із device/Persona evidence. Наявність каналу не видає членство чи право читати Space.
3. **Контроль перед даними.** Сторони звіряють підписані head/epoch, grants і revocations; доводять чинне членство й scope, застосовують новий key epoch. Коли доказ недостатній, захищений payload не видається, сесія відмовляє або обмежується дозволеним bootstrap. Семиденне правило читання shared cache лишається окремим інваріантом.
4. **Summary і plan.** Лише для дозволених scope обмінюються causal summaries/frontiers, обчислюють відсутні operation та залежності, узгоджують версію протоколу й обмежені сторінки передачі. План не є канонічним порядком бізнес-змін.
5. **Передача й перевірка.** Підписані операції та encrypted blob manifests/chunks передаються незалежно; перевіряються підписи, схема, цілісність, causal dependencies і права для отримання. Недостатні залежності залишаються pending. Дубль того самого operation ID застосовується ідемпотентно; replay `sync.apply` не запускає початкову команду.
6. **Доменна перевірка.** Кожний peer за тим самим schema/Space contract перевіряє підпис, право актора, epoch, causal base і доменні передумови проти перевіреної control history. За браку доказу чинних прав candidate лишається pending, а не accepted. Роль Owner визначається membership, не пристроєм. Результат перевірки не залежить від того, телефон чи ноутбук першим прийняв пакет. Evidence axes, `ReplicationReceipt`, `DeliveryReceipt` і `AuthorityOutcome` визначає [SPEC-OPERATION-FINALITY-001](operation-finality-contract.md); authority topology/equivalence для окремих operation families лишаються `OQ-0033`/`OQ-0034`.
7. **Merge і рішення людини.** Валідні causal histories поєднуються; незалежні текстові/структурні зміни сходяться автоматично. Для перекриття фрагмента або непорівнюваних несумісних значень зберігаються обидва variants і застосовується однакове на peers schema-level правило. Автор може переглянути обидва, за чинного права повторно подати свій вибір як нову операцію або зберегти копію/ревізію, де це підтримує ресурс. Фоновий sync не повинен вимагати негайної modal-відповіді; UX тимчасового/остаточного стану є `OQ-0034`.
8. **Checkpoint і ACK.** Кожний peer надійно фіксує нові frontiers і підтверджує окремо отримання, локальний запис та застосування. Перша незалежна authorized durable application replica може виконати default Replication Policy; receipt тієї самої Persona не є другим approval vote. Delivery/history ACK не підвищується до domain acceptance. Перервана сесія відновлюється з summaries без втрати pending намірів.

## Інваріант порядку

Усі авторизовані пристрої однієї Persona є рівноправними; жоден не є Owner/master/арбітром інших. «Перший синхронізувався» означає виконання Replication Policy, а не domain winner. Для двох незалежних peer-груп під час partition різний порядок прибуття пакетів не дає глобального порядку: потрібні причинність + однакове schema-level правило merge/conflict. Finality є bounded до конкретного Request revision/frontier; пізня accepted incomparable branch може повторно відкрити Conflict. Це не скасовує Owner-повноважень **людини/Persona** й не дає Device binding бізнес-пріоритету.

Порівнюється доказане **authority acceptance**, а не лише момент створення operations: навіть конкурентно створені зміни підпадають під правило першого прийняття, якщо прийняття згодом впорядковані. Multi-value status/file conflict виникає тільки для двох належно прийнятих, несумісних гілок без такого спільного порядку; невалідний pending candidate туди не входить.

## Рішення для непорівнюваних status/file змін

| Кандидат | Що бачать peers після повного обміну | Ціна |
|---|---|---|
| **Multi-value conflict (затверджено для status/file)** | Обидва варіанти та позначку «потребує рішення» без автоматичного winner; уповноважена людина створює нову операцію, яка причинно враховує обидві гілки. | Тимчасово немає одного підтвердженого статусу/файла; два конкурентні людські рішення знову можуть вимагати merge. |
| **Детермінований видимий winner + альтернативи (відхилено для status/file)** | Усі peers показують те саме значення за окремо погодженим правилом, але інше лишається доступним як conflict. | Видиме значення може змінитися після надходження раніше невідомої гілки; ID-based tie-break може створити фактичну перевагу одного replica ID, а winner не означає «синхронізувався першим» і може бути бізнесово хибним. |

Для незалежних редагувань тексту/структурних полів лишається погоджений автоматичний merge. Для несумісних непорівнюваних змін статусу задачі та заміни файла рішення прийнято в [ADR-0017](../03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md); жоден пристрій не отримує вищих прав. Два несумісні авторизовані офлайн-рішення цього конфлікту, чиї authority acceptances залишилися непорівнюваними, після reconciliation дають новий явний conflict між рішеннями; наступне рішення враховує обидва актуальні heads. Доведено еквівалентні рішення дають один видимий результат зі збереженням обох operations, а пізня правомірно прийнята несумісна гілка з непорівнюваним щодо рішення acceptance знову відкриває conflict. Точний proof, equivalence predicate і формат операції лишаються відкритими.

## Референси й межі запозичення

- [Irokle](https://github.com/arunaengine/irokle): signed Merkle-DAG, control operations у DAG, summary → missing causal data → ACK, перевірка членства. Це кандидат для history/repair; його ACK не є VIDA domain receipt.
- [Automerge sync/concepts](https://automerge.org/docs/reference/concepts/) і [conflicts](https://automerge.org/docs/reference/documents/conflicts/): незалежний від транспорту sync та збереження альтернатив scalar-значення. Його внутрішній deterministic winner не реалізує VIDA first authority acceptance.
- [Loro sync](https://loro.dev/docs/tutorial/sync): version-vector обмін і докачування відсутніх змін; приклад окремої CRDT-merge площини, не authority policy.
- [Loro API peer-ID guidance](https://www.loro.dev/docs/api/js): конкурентні сеанси не повинні ділити один replica/PeerID, навіть якщо належать тій самій людині. Різні causal actor IDs не означають різні права чи пріоритет пристроїв.
- [CouchDB replication conflicts](https://docs.couchdb.org/en/stable/replication/conflicts.html): приклад детермінованого видимого winner зі збереженням інших ревізій; це не доказ, що winner фізично синхронізувався першим.
- [OWASP Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html): default deny і перевірка дозволів для кожної нової дії; це обґрунтовує повторну перевірку перед re-submit та захист payload після revocation, але не визначає алгоритм розподіленого ordering.
- [OWASP MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/): альтернативні офлайн-варіанти, чернетки й ключі на Android/iOS мають захищатися як sensitive local data, а не просто зберігатися «про всяк випадок». Перевірено 2026-09-20; тест: після конфлікту й restart перевірити recoverability для чинного учасника та відсутність відкритого plaintext/ключів у незахищеному сховищі. Конкретну модель шифрування визначає `OQ-0029`/`OQ-0036`.

## Відкриті decision gates

- `OQ-0033`: конкретна serialized logical authority topology/current-frontier mechanism для serverless exclusive operation families; receipt/replication semantics закриті `SPEC-OPERATION-FINALITY-001`.
- `OQ-0034`: реалізація прийнятого multi-value status/file правила (wire/capability, точний equivalence predicate і safe retention frontier), правила інших operation families, деталізація overlap, metadata/ACL для revision/copy та snapshot/pruning.
- `OQ-0028`/`OQ-0037`: canonical wire envelope і сумісність різних версій core.

## Conformance scenarios до protocol lock

- Два пристрої однієї Persona мають несумісні непорівнювані offline status candidates: transport receipt не завершує конфлікт; після повного обміну обидва variants recoverable, показаний явний unresolved conflict без winner незалежно від того, котрий пристрій був першим peer. Нове рішення посилається на обидві гілки й перевіряє чинне право.
- Два учасники змінюють різні абзаци → обидва збережені; те саме речення → обидва варіанти доступні для явного рішення.
- Два PDF-замінники → обидва зберігаються; якщо прийняття мають спільний порядок, перший лишається поточним. За непорівнюваної несумісної конкуренції після reconciliation жоден не є поточним автоматично: усі peers показують unresolved conflict з обома версіями до нового авторизованого рішення.
- Пристрій відкликаного учасника не отримує захищені дані та не може провести нову зміну лише через успішний Iroh connect або старий client timestamp.
- Повторна доставка/відновлення перерваної сесії не дублює операцію, бізнес-команду чи зовнішній ефект.
- Дві конкурентні прямі сесії з різними peer-групами після взаємного обміну сходяться до однакових стану й набору конфліктів без designated peer; жоден локальний ACK не подається як доказ глобального «першого».
- Богдан і Олена офлайн по-різному вирішили той самий status/file conflict; обидва рішення пройшли authority policy без спільного порядку. Після reconciliation усі peers показують новий conflict з обома рішеннями без winner; payloads збережені, а подальше рішення спирається на обидва resolution heads і чинні права.
- Два рішення мають доведено однаковий результат і наслідки: peers показують один результат, але обидва автори лишаються в audit history. Третя прийнята несумісна гілка, яка не входила до causal base рішення і має непорівнюване acceptance, знову відкриває conflict; порівнюване acceptance підпадає під правило першого прийняття.
- Учасник із правом update, але без read до одного variant, не бачить закритий payload і не може вирішити conflict; уповноважений reviewer може. Revision лишає resource ID, copy створює новий ID після окремої перевірки прав.
