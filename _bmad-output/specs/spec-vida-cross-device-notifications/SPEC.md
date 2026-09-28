---
id: SPEC-vida-cross-device-notifications
companions:
  - ../../../docs/02-requirements/transport-sync-requirements.md
  - ../../../docs/02-requirements/access-control-requirements.md
  - ../../../docs/02-requirements/native-client-requirements.md
  - ../../../docs/02-requirements/effect-execution-constraints.md
  - ../../../docs/03-architecture/decisions/ADR-0016-equal-device-peers.md
  - ../../../docs/03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md
  - ../../../docs/04-specifications/sync-presence-status-model.md
  - ../../../docs/04-specifications/schema-evolution-contract.md
sources: []
---

> **Canonical contract.** Ця специфікація фіксує підтверджену поведінку сповіщень між пристроями. Відкриті питання не є затвердженими вимогами.

# Сповіщення та наслідки дій між пристроями VIDA

## Why

Користувач VIDA працює з одним Space на кількох пристроях. Сповіщення про подію не повинне губитися через те, що конкретний пристрій був offline, а дія, виконана на одному пристрої, не повинна лишати інші пристрої в суперечливому стані.

## Capabilities

- **CAP-1**
  - **intent:** Адресат може отримати одне логічне сповіщення на кожному своєму авторизованому пристрої після синхронізації цього пристрою.
  - **success:** Телефон і ноутбук адресата показують сповіщення про одну подію; offline-пристрій отримує його після повернення; повторна доставка не створює дубль на тому самому пристрої.
- **CAP-2**
  - **intent:** Адресат може виконати дозволену дію на будь-якому пристрої та побачити її підтверджений наслідок на всіх інших авторизованих пристроях після синхронізації.
  - **success:** Завершення задачі на телефоні змінює її стан на ноутбуці після синхронізації; replay operation не виконує початкову бізнес-команду чи зовнішній ефект вдруге.
- **CAP-3**
  - **intent:** Адресат може відкрити сповіщення на будь-якому пристрої, щоб воно стало прочитаним на всіх його авторизованих пристроях.
  - **success:** Після відкриття сповіщення на телефоні ноутбук показує це саме логічне сповіщення прочитаним після синхронізації, навіть якщо був offline під час відкриття.
- **CAP-4**
  - **intent:** Адресат може відкрити сповіщення на одному пристрої, щоб відповідні системні банери зникли з інших пристроїв, де це підтримує ОС.
  - **success:** Після відкриття на телефоні та синхронізації ноутбук із підтримуваним OS API прибирає відповідний банер; без такого API спільний статус «прочитано» у VIDA однаково зберігається.
- **CAP-5**
  - **intent:** Автор несумісної офлайн-зміни статусу задачі може після синхронізації побачити вже застосований спільний стан і власний намір та явно обрати подальшу дію.
  - **success:** За спільного перевірного порядку перше прийняття встановлює статус; за непорівнюваних несумісних прийнять усі peers показують явний конфлікт без winner. Два несумісні авторизовані офлайн-рішення без спільного порядку знову конфліктують; доведено еквівалентні рішення дають один видимий результат із двома audit records. Пізня прийнята несумісна гілка повторно відкриває конфлікт, лише якщо її authority acceptance непорівнюване з рішенням. Вирішити його може лише актор із чинними правами на дію та читання всіх потрібних variants.
- **CAP-6**
  - **intent:** Автор пізнішого конфліктного variant ресурсу може зберегти свою роботу й прийняти бізнес-рішення, не втрачаючи вже прийнятого стану.
  - **success:** За непорівнюваних несумісних замін файла peers показують обидві версії та конфлікт без автоматично поточного файла. Два несумісні авторизовані офлайн-рішення знову конфліктують; пізня прийнята несумісна гілка з непорівнюваним щодо рішення acceptance відновлює конфлікт. Альтернативу можна явно зберегти як ревізію того самого ресурсу або як окрему копію з новим ID і перевіркою прав. Жоден payload не зникає від локального вирішення; незалежні фрагменти тексту поєднуються автоматично, перекриття лишається для явного рішення.

## Constraints

- Identity/relay-only вузол приватного federated Space пересилає лише непрозорий сигнал/шифротекст; приватні деталі сповіщення отримує авторизований пристрій адресата через sync/decrypt. Окремий довірений AppCenter не розглядається в цій device-core специфікації.
- Спільний наслідок дії є доменним станом або зафіксованою operation, а не висновком із per-device delivery ACK.
- Усі авторизовані пристрої однієї Persona — рівноправні sync peers; Owner/Admin є роллю учасника Space, не телефона/ноутбука. Тип пристрою, replica ID або перший ACK не створюють пріоритету.
- Спільний статус «прочитано» синхронізується як стан застосунку; `delivery.read` окремого пристрою сам по собі не визначає його.
- Прибирання системного банера є best-effort і залежить від можливостей платформи; його невдача не змінює спільного статусу «прочитано».
- `sync.apply` застосовує вже зафіксовану operation і не запускає нову бізнес-команду або post-commit зовнішню дію.
- Очікування offline-пристрою обмежене чинною retention policy; ця специфікація не встановлює її строк.
- Часова позначка до мілісекунд не є доказом точності годинника незалежного офлайн-пристрою; для конфліктного статусу порівнюваний acceptance важливіший за client timestamp, а для двох роз'єднаних peer-груп глобального «першого sync» немає.
- Для тексту й коду незалежні частини поєднуються автоматично, а перекриття одного логічного фрагмента потребує явного рішення; правило статусу задачі не замінює їхню окрему merge-модель.
- Прямий peer-to-peer transport та history ACK не створюють глобальний порядок acceptance. Для несумісних непорівнюваних status/file branches затверджено явний multi-value conflict; proof acceptance, wire format та інші operation families ще відкриті в `OQ-0033`/`OQ-0034`. [Контур сесії](../../../docs/04-specifications/device-sync-session.md) має статус `review`.
- Лише authority-accepted variants входять у спільний conflict; відкликаний/невалідний pending candidate лишається recoverable за правами, але не проектується як прийнята гілка. Для конфліктного файла payloads/pins зберігаються до безпечної retention frontier, а не лише до локального рішення; бібліотечний tie-break або звичайне assignment не є тихим бізнес-рішенням.
- Конкурентне створення operations саме по собі не викликає multi-value conflict: якщо їхні authority acceptances мають спільний перевірний порядок, діє правило першого прийняття. Явний unresolved conflict виникає лише там, де такого порядку для прийнятих несумісних гілок немає.
- Незворотний зовнішній наслідок очікує підтвердження source transition за Space/object policy і відсутності відомого невирішеного conflict; локальне збереження та доставка не є цим підтвердженням. Пізня невідома гілка все ще можлива; exact proof та компенсація відкриті.
- Після вже виконаного незворотного наслідку пізній конфлікт не стирає факт ефекту й не створює фіктивне «скасування»: користувачу пропонується окреме виправлення. Невідомий результат стороннього запиту не можна вирішити самим Iroh/Automerge sync; safe retry залежить від контракту зовнішнього сервісу.
- Conflict detection/merge є глобальною core/runtime функцією без окремого права `resolve_conflict`. Звичайна нова update поточного state проходить нормальне action permission; resolution, що причинно покриває conflicting heads, додатково потребує читання variants. Agent/bot може auto-submit ordinary update за її general automation opt-in; conflict resolution потребує explicit conflict-context opt-in, але не нового ACL-права. Source fact для похідного ресурсу прив'язує ефективну версію правила, тому mixed-version peers сходяться на одному logical ID; exact binding/protocol лишається відкритим.
- Агент використовує загальну модель ACL/ServicePrincipal без спеціальної обов'язкової Owner/Admin схеми. Пристрій, що лише отримує уже створений похідний ресурс, синхронізує його дані, не перевиконує стару логіку AppInstance.
- Iroh `Endpoint::online`, address і transport ACK не визначають user-facing presence або operation processing. «Онлайн» потребує свіжого VIDA application handshake; «синхронізовано» — application receipt про durable validate/apply у визначеному replication scope. Другий пристрій тієї самої Persona є другою durable-копією, не другим authority vote. Count включає current user-controlled device; bots/ServicePrincipals показуються окремо. One-device Persona може бути profile-online, коли її нова operation ще лише `saved locally`. Анонімний profile count не агрегується з іншими Personas.
- VIDA-to-VIDA request outcome є `RequestId`-bound signed durable resource state з operation-family authority/resolver та доставляється normal sync; відсутність requester-а online не втрачає відповідь. Якщо branch поза causal confirmation frontier незворотного effect відкриває conflict, correction workflow отримує її актор; arrival order/clock не призначають «винного», а кілька непорівнюваних outside-frontier branches чекають explicit resolution. Попередній effect лишається в audit. Сторонній API без durable status/idempotency залишається окремим unknown-outcome випадком.
- Offline chat message отримує primary timestamp за publication після sync; shared order використовує causal/topological publication order і canonical operation-ID tie-break для непорівнюваних simultaneous publications. Millisecond publication time є display metadata, не єдиним order key; written-at лишається audit/detail.
- Failed blob write/download дає локальний стан «не завантажено», error і bounded retry, не змінює shared manifest або remote replicas. Existing resource schemas проектуються/мігруються до current AppInstance schema; edit legacy record пише current schema, materialize-ить safe defaults і вимагає введення required-without-default; historical operation versions не переписуються.
- Installed clients are continuous-sync/online-first while the OS permits execution. Android/iOS use only policy-compliant foreground/background/push mechanisms and durable reconciliation; suspended/force-stopped/network-unreachable is an explicit degraded exception, not hidden as online. A 100% mobile background-availability guarantee and mandatory floating overlay are excluded.
- Інший authorized actor або bot може створити звичайну нову domain update за своїми правами; це не transfer чужого correction workflow. Bot update поточного state відрізняється від resolution operation, що причинно покриває conflicting heads.

## Non-goals

- Обрати push provider, механізм OS-банерів, lease/executor, точний формат notification operation або політику зберігання.
- Гарантувати прибирання системного банера на ОС, яка не надає відповідного API, або визначати точний platform API.
- Проєктувати окремий AppCenter/сервер чи вебінтерфейс у цій хвилі; користувач відклав ці контури на користь core і встановлюваних застосунків пристроїв.

## Success signal

У двох пристроїв адресата сходяться стан задачі та статус «прочитано» після дії або відкриття сповіщення на одному з них; підтримувана ОС прибирає застарілий банер, а повторна доставка й тимчасовий offline не створюють дубль сповіщення або повторний зовнішній ефект. Несумісні офлайн-рішення залишають явний conflict, еквівалентні — один видимий результат із двома audit records, пізня прийнята несумісна гілка з непорівнюваним щодо рішення acceptance повторно відкриває conflict. Копія й ревізія файла є окремими діями; правило однієї похідної нотатки працює попри різні версії клієнтів.

## Open Questions

- Який proof acceptance, equivalence predicate, wire/capability contract і provisional UX реалізують затверджені правила status/file; які правила потрібні для інших operation families?
- Яка точна гранулярність фрагмента, metadata/ACL inheritance для copy/revision та безпечна retention frontier зберігають альтернативні file variants?
- Як представити automation opt-in для звичайної domain action у загальних ACL/ServicePrincipal, включно з principal, делегуванням, audit, revocation і budgets; як підтвердити, перевірити невідомий результат і безпечно повторити незворотний зовнішній наслідок лише там, де це гарантує стороння інтеграція?
- Як прив'язати rule version до source fact, зберегти стару версію правила та вивести один logical ID при змішаних версіях клієнтів?
- Які freshness/expiry та receipt fields розрізняють online peers, «синхронізовано», domain acceptance і conflict без cross-Persona витоку; як suspension/force-stop впливають на public count?
- Які exact application publication receipt fields та epoch/frontier encoding реалізують уже погоджені causal order + operation-ID tie-break?
- Які current-schema activation authority, converter ABI, migration retry, mixed-version window і rollback barrier застосувати до AppInstance data?
