# Rights proof options and candidate fixtures

Це похідна від [SPEC](SPEC.md) матриця **кандидатних, ще не виконаних** випробувань. `REQ-ACL-016/018/019`, `ADR-0003` та `AD-16` фіксують поведінку; `OQ-0053` лишає спосіб доказу відкритим.

## Що є доказом, а що ні

- Кандидатний receipt може зв'язувати version/profile, Space/scope, member Persona/Device/grant/epoch, authority identity, current control frontier/head, client challenge і час/межу дії. Це **поля для проєктування**, не затверджений wire profile. Receipt повинен походити від actor, авторизованого поточною Space authority policy, а не від relay чи випадкового peer.
- Підписаний старий receipt засвідчує колишню відповідь, але не поточну свіжість. Локальний UTC годинник можна змінити. Android `SystemClock.elapsedRealtime()` рахує час від boot **разом зі sleep**, Apple `ProcessInfo.systemUptime` описує час від restart (його sleep semantics ще перевірити на цільових ОС), Windows `GetTickCount64` рахує sleep/hibernate, а `QueryUnbiasedInterruptTime` їх виключає. Жоден із цих boot-scoped показників сам не доводить незворотний вік через reboot або restore старого стану ([Android](https://developer.android.com/reference/android/os/SystemClock), [Apple](https://developer.apple.com/documentation/foundation/processinfo/systemuptime), [Windows](https://learn.microsoft.com/en-us/windows/win32/sysinfo/interrupt-time)).
- Кандидатний monotonic anchor у захищеному platform storage та high-water control head треба тестувати на rollback. Device-bound key, AEAD чи підпис не роблять відновлену стару базу даних свіжою автоматично. Вимога безпеки стосується сумісних керованих клієнтів; rooted/compromised OS та вже скопійований plaintext не можна без доказів охоплювати гарантією.
- [OWASP MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) та [Session Management](https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html) обґрунтовують захист локального стану й перевірку expiry, але не задають VIDA строк 7 діб або універсальний offline clock.

## Потрібне продуктове рішення при unknown age

| Варіант | Приклад після offline reboot | Наслідок |
|---|---|---|
| A — fail closed до authority | Телефон перезавантажився на другий день, а доказ фактично минулого часу не збережено. Shared read блокується до зв'язку. | Не подовжує доступ відкатом; практично може скоротити офлайн-доступ до 7 діб. Потрібна явна згода користувача на цю поведінку. |
| B — гарантувати всі 7 діб попри reboot | Після перезапуску клієнт доводить elapsed age без network. | Потребує перевіреного non-rollback primitive на кожній підтриманій release-платформі; згадані стандартні timers такого доказу не дають. Без нього B не можна обіцяти. |
| C — platform capability profile з A fallback | На пристрої з доведеним primitive користувач продовжує офлайн; інші fail closed до authority. | Зберігає безпеку, але додає відмінності UX між платформами і потребує release-build conformance. Це кандидат, не затверджена policy. |

## Candidate fixtures — not executed

| ID | Перевірка | Очікуване спостереження |
|---|---|---|
| RRP-F01 | Валідний fresh proof від чинної Space authority | Лише він поновлює anchor і прив'язаний до конкретного Space/grant/head. |
| RRP-F02 | Той самий receipt із повторним nonce або для іншого Space/Persona/Device | Reuse відхилено, інтервал не поновлюється. |
| RRP-F03 | Є лише relay/peer connection або mailbox ACK | Жодного rights refresh. |
| RRP-F04 | Старий підписаний head після новішого control state | Відмова від downgrade/replay; high-water state не відкотити. |
| RRP-F05 | Зсув wall clock назад/вперед, зміна timezone/DST | Право не подовжується й не скорочується від зміни user clock. |
| RRP-F06 | App crash, force-stop і relaunch до/після межі | Read gate зберігає правильний стан, не поновлює 7 діб. |
| RRP-F07 | OS reboot або restore старої DB/backup/VM snapshot | Якщо trusted age не доведено, застосовується лише явно затверджена `OQ-0053` policy; старий proof не приймається як новий. |
| RRP-F08 | Sleep/hibernate довкола `7 діб − 1 ms`, `7 діб`, `7 діб + 1 ms` | На кожній release OS обраний clock/proof має рахувати фактичний elapsed age, включно зі sleep/hibernate; якщо не може — `unknown age` за ще не затвердженою `OQ-0053` policy, не фальшиве продовження інтервалу. За доведеного age до межі дозволено, на межі й після — lock. |
| RRP-F09 | Effective `MemberRemoved` отримано до семи діб, потім crash/reboot | Негайний lock зберігається; старий head не оживляє membership. |
| RRP-F10 | Ротація authority, key epoch, grant або policy version | Отриманий новий control state не дає старому receipt поновити інтервал; невідома офлайн ротація сама по собі не скасовує вже погоджені правила читання до proof expiry чи отриманого revocation. |
| RRP-F11 | Дубльовані, переплутані або несумісні receipts | Детермінований high-water outcome; немає випадкового reset interval. |
| RRP-F12 | Розрив зв'язку до спливу при вже відкритому документі | Вміст залишається видимим до доведеного expiry; `critical` не має окремого online-open gate. |
| RRP-F13 | Сплив інтервалу, поки документ відкритий або app у background | При поверненні/на межі всі керовані read surfaces блокують вміст. |
| RRP-F14 | Messenger, Notes, Projects, files, search, recents, notifications, export | Один Core read gate; немає обходу через secondary projection. |
| RRP-F15 | Plugin/automation/runtime/API намагається прочитати locked Space | Запит відхилений незалежно від UI state. |
| RRP-F16 | Shared Owner офлайн; інший Owner відкликав його membership | Немає Owner exemption; після revocation або доказаного expiry lock як для інших. |
| RRP-F17 | Personal Space Owner офлайн | Shared 7-day lock не застосовується; діють окремі local identity/lock rules. |
| RRP-F18 | Shared read locked, створення нового blind draft і старий pending work | Новий draft durable без читання кешу; старий pending збережений; acceptance лише після актуальної перевірки прав. |
| RRP-F19 | Receipt підписав неуповноважений actor або policy вимагає quorum, якого немає | Підпис/доставка не стають authority-confirmed reconciliation; інтервал не поновлюється. |

## Невирішений release gate

Перед production заявою про семиденний offline read потрібні: затверджений unknown-age UX, threat scope для compromised device, платформні proof-capability профілі, byte-level receipt/vector fixtures, фахова безпекова перевірка та результати RRP-F01–F19 на Android/iOS/Windows. Жоден пункт тут не є результатом виконаного тесту.
