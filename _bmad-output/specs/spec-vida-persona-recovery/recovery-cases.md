# Recovery contract and candidate fixtures

Це похідний від [SPEC](SPEC.md) план conformance, **не виконані тести** і не вибір алгоритму. Нормативна межа — `REQ-ID-003/010/011/015/016`, `SPEC-ID-011` та `AD-37` з adopted companions.

## Межа authority та даних

1. За доступної свіжої controller history restore звіряється з нею. Якщо всі старі Devices недоступні й актуальність довести неможливо, новий Device може отримати необхідний локальний recovery grant і працювати в provisional гілці. Це не доводить спільну authority або відсутність невідомого revoke. Вузол підтверджує лише свої окремі account/service claims, якщо інша явно погоджена policy не визначить більше.
2. Погоджений комплект складається з окремо збережених recovery secret і encrypted recovery bundle. Відновлення історії додатково залежить від доступної encrypted resource copy. Наявність secret не доводить, що bundle чи дані існують; локальна OS/cloud копія не доводить, що її можна відкрити після втрати цього пристрою.
3. Платформне secure storage захищає локальні ключі, але device-bound non-exportable ключ не можна без доказу називати міжпристроєвим способом відновлення. Bundle не копіює приватні Device signing keys; точний формат і manifest privacy ще треба визначити до user-facing обіцянки.

## Bootstrap lifecycle — погоджене правило VIDA

| Стан | Видимий результат | Дозволений перехід |
|---|---|---|
| `setup_pending` | Ті самі локально зафіксовані Persona/Space IDs, ключі, recovery material і staged Owner authority переживають restart; protected Space work заблокована. | Показати матеріал і отримати явне підтвердження окремого збереження. |
| `active` | Підтвердження надійно записане; `PersonaCreated` і Owner authority стосуються тих самих IDs; дані Space стають доступними. | Звичайні дозволені Core-команди. |

Якщо staging або фіналізація перервалися, reopen визначає один із цих станів із durable даних; UI не показує частковий успіх і не генерує нову Persona мовчки. Product owner підтвердив суворий gate 2026-09-26; це власне правило VIDA, а не поведінка, доведена для Delta Chat чи вимога OWASP. Воно уточнює Stories 1.1–1.2, але не обирає формат ключів, recovery bundle або фізичний storage provider (`OQ-0024/0036`).

## Candidate fixtures — not executed

| ID | Кейс | Очікуване спостереження |
|---|---|---|
| REC-F01 | Автономна Persona створюється без вузла; kill/restart перед і після staging та confirmation | До confirmation відновлюється той самий `setup_pending` без `PersonaCreated`; після durable confirmation активуються ті самі IDs і Owner authority рівно один раз. Secret показаний власнику, bundle доступний для окремого export; setup вимагає підтвердження збереження обох і попереджає про можливу незворотну втрату; сервер не отримує secret. |
| REC-F02 | Новий Device додає чинний trusted Device | Незмінний `PersonaId`, новий `DeviceId`/keys/grant; версіонований control transition перед protected read. |
| REC-F03 | Усі Devices втрачені; secret, bundle і encrypted resource copy доступні, свіжий controller frontier недоступний | Локальний recovery grant і робота дозволені як provisional; restore окремо показує authority, key і фактично відновлені content classes/omissions, не заявляючи перевірену спільну authority. Точна перевірка й reconciliation — `OQ-0022/0024`. |
| REC-F04 | Є secret, але немає bundle або resource copy | Без bundle authority не обіцяється; за наявності bundle, але без ресурсів, не обіцяється повна історія. |
| REC-F05 | Є bundle, але немає потрібного secret | Recovery credential і вміст не розкриваються й новий grant не видається. |
| REC-F06 | Bundle чужої Persona, tampered, truncated або невідомої версії | Кандидатний safety oracle: відмова без часткової активації чи підміни `PersonaId`; точний валідатор, version policy та failure UX визначити в `OQ-0024`. |
| REC-F07 | Stale/replayed `ControllerState`, forged grant або дві concurrent спроби restore | Forged/replayed grant не приймається; кожна provisional гілка може зберігати локальні зміни, але дві несумісні controller-гілки не оголошуються єдиною authority. Перевірний reconciliation лишається `OQ-0022/0024`. |
| REC-F08 | Crash під час видачі grant або restore; повтор тієї самої команди | Немає активного напівперенесеного стану чи дубльованого Device; retry за command ID і контрольоване відновлення. |
| REC-F09 | Втрата або компрометація старого Device | Його `DeviceGrant` відкликаний, affected envelopes/epochs змінені; майбутні protected reads старим Device відхилені. |
| REC-F09a | Підозрюваний витік recovery secret і bundle | Власник може запустити термінове відкликання старого комплекту, новий комплект і переузгодження Devices; точний протокол і доказ ефективності ще `OQ-0024`. Уже скопійований plaintext не обіцяється стерти. |
| REC-F10 | OS/cloud backup вимкнено, застарів, недоступний, або інша платформа/перевстановлена ОС | Явна помилка та межа доступних даних; manual owner-held route не підмінюється неіснуючим backup. |
| REC-F11 | Компанія блокує чи відновлює виданий акаунт | Лише залежні робочі grants змінюються; незалежні приватна/анонімна Persona та їхні Spaces не відкриваються. Controller/recovery окремої корпоративної Persona — `OQ-0049`. |
| REC-F12 | Перевірка logging/export/telemetry на кожному шляху | Немає recovery material, ключів або прихованих cross-Persona linkage у logs/public projections/telemetry. Видимість bundle manifest — окремий кандидатний тест після рішення `OQ-0024`. |
| REC-F13 | Два нові Devices офлайн відновилися одним комплектом і змінили controller та Notes | Сумісні дані об'єднуються; несумісні authority-дії призупинені до перевірного reconciliation, локальні Notes не губляться. Немає clock/Device winner. |
| REC-F14 | Після provisional роботи peer доводить попереднє відкликання комплекту | Старий grant не публікує чи не керує Persona; незасинхронізований локальний текст можна скопіювати або перенести лише після нової авторизації. |
| REC-F15 | Звичайна ротація material і перезапуск до/після re-export | Створюється новий комплект і явний запит зберегти його; звичайні редагування не блокуються. UI не обіцяє restore зі старим комплектом; crash-atomicity лишається `OQ-0024/0036`. |
| REC-F16 | Підозрюваний витік обох частин комплекту | Термінова дія відкликає старий комплект, створює новий і запускає переузгодження Devices; ефективний controller cut та crypto proof лишаються `OQ-0022/0024`. |

## Необрані design decisions

- `OQ-0022/0024`: byte encoding, secret representation/entropy confirmation, KDF, AEAD/key wrapping and nonce policy, bundle portability and atomic custody, proof-of-restore, integrity/version/ownership validation and manifest visibility, verified current-controller/concurrent-restore reconciliation, effective compromise cut and crash-safe re-export. Product behavior above is approved; these mechanisms are not.
- `OQ-0049`: корпоративний controller, account recovery, portability і точна залежність grants від node account.
- До закриття цих питань не обіцяти відновлення після будь-якої втрати пристроїв. [OWASP Key Management](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html) вимагає планувати key lifecycle, backup і compromise response; [OWASP MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) — захищати локальні чутливі дані. Це вимоги до перевірки, не готовий recovery-протокол.
