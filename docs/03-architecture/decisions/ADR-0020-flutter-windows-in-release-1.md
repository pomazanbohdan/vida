---
id: ADR-0020
status: accepted
last_updated: 2026-09-22
source_refs:
  - ../../../_bmad-output/planning-artifacts/prfaq-vida.md
  - ../windows-client-options.md
decision_refs:
  - ADR-0014
  - ADR-0015
  - ADR-0018
  - ADR-0019
supersedes:
  - ADR-0019
superseded_by:
  - ADR-0021-browser-post-v1-consequence
---

# ADR-0020: Flutter Windows у Release 1; WinUI 3 після Release 1

## Контекст

Попередню інтерпретацію ADR-0019 уточнено користувачем: повноцінний Flutter Windows client входить у Release 1 разом з Android та iOS. Після Release 1 може з'явитися окрема нативна Windows-реалізація на WinUI 3/C#, якщо її цінність буде доведена. Windows-поставка Release 1 не є браузерним companion і не використовує WinUI 3 як обов'язковий shell.

## Рішення

- Public Release 1 `MUST` включати встановлювані Flutter clients для Android, iOS і Windows зі спільним Rust core.
- Кожна з трьох ОС `MUST` мати окремі platform specifications, lifecycle/permission adapters, packaging/signing/distribution і conformance profiles у монорепозиторії.
- Flutter Windows `MUST` пройти той самий core functional gate, що Android та iOS: Messenger, Notes/Knowledge, Projects/Tasks, files, forums, calls, offline durability, multi-device sync і conflict UX.
- Windows profile додатково `MUST` довести keyboard-first workflows, system menus, tray integration і screen-reader accessibility.
- WinUI 3/C# `MAY` бути окремою нативною Windows-реалізацією після Release 1; вона не входить у Release-1 critical path.
- Tauri не є обраним Release-1 Windows stack. Його можна повернути до розгляду лише новим рішенням із конкретною потребою.

## Наслідки

- ADR-0019 superseded повністю.
- ADR-0015 зберігає правильний перелік платформ, але вибір Windows toolkit тепер уточнено цим ADR.
- Історичне post-Release-1 відкладення browser superseded by [ADR-0021](ADR-0021-static-web-client-in-release-1.md); Windows toolkit і native-client рішення цього ADR лишаються чинними.
- Числові Windows performance budgets визначаються representative vertical slice, але відсутність наперед вигаданих чисел не скасовує Windows із Release 1.
