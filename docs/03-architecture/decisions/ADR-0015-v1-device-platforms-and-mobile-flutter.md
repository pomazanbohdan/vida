---
id: ADR-0015
status: accepted
last_updated: 2026-09-20
source_refs:
  - ../stack-selection-brief.md
  - ../windows-client-options.md
decision_refs:
  - ADR-0005
  - ADR-0014
supersedes: []
superseded_by: []
---

# ADR-0015: Android, iOS і Windows у v1; Flutter для мобільного UI

> Refined by `ADR-0020`: Android, iOS і Windows входять у Release 1 як Flutter targets; окремий WinUI 3/C# client можливий після Release 1.

## Контекст

ADR-0014 встановив installed-only клієнти, не обираючи перелік ОС або UI toolkit. Користувач 2026-09-20 окремо підтвердив Android та iOS на Flutter і повний Windows-клієнт для першого релізу. Частину технічного стека користувач планує надати з іншого репозиторію для подальшої оцінки.

## Рішення

- Публічний v1 `MUST` включати встановлювані Android, iOS і Windows-клієнти з усіма трьома базовими Apps та погодженою offline-поведінкою.
- Android/iOS presentation layer `MUST` використовувати Flutter. Спільна доменна авторизація, синхронізація й обробка операцій залишаються у VIDA core, а не дублюються в Dart UI.
- Windows `MUST` бути повноцінним клієнтом за `REQ-CLIENT-006`. WinUI 3 + C#, Flutter Windows і Tauri лишаються кандидатами до окремого порівняння; це ADR не обирає Windows toolkit, WebView policy, FFI binding або конкретний повторно використаний репозиторій.

## Наслідки й перевірка

- Release conformance для кожної ОС перевіряє Messenger, Notes/Knowledge, Projects/Tasks, offline durability, conflict UX, rights reconciliation та accessibility. Beta можуть виходити поетапно, але не замінюють повний публічний v1.
- Flutter mobile binding до Rust core має пройти prototype/release tests для cancellation, background lifecycle, process restart і secure local storage; вибір binding — `OQ-0035`.
- Перевірено 2026-09-20: [OWASP MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) вимагає захисту sensitive data на пристрої. Для VIDA це стосується pending operations, альтернативних версій і ключів; acceptance test має шукати plaintext/ключі в незахищеному mobile storage після restart. OWASP не визначає UI framework чи набір ОС.
