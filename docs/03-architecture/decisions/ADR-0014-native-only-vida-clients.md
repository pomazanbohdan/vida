---
id: ADR-0014
status: accepted
last_updated: 2026-09-20
source_refs:
  - ../stack-selection-brief.md
  - ../../../_bmad-output/planning-artifacts/research/technical-vida-ui-platform-and-rust-binding-strate-2026-09-19/research.md
decision_refs:
  - ADR-0005
  - ADR-0013
supersedes: []
superseded_by:
  - ADR-0018
  - ADR-0021
---

# ADR-0014: VIDA постачається як встановлювані клієнтські застосунки, без браузерного клієнта

> Історичне browser exclusion superseded by `ADR-0018`, а Release-1 Web scope — by [ADR-0021](ADR-0021-static-web-client-in-release-1.md). Вимоги цього ADR до повноцінних installed clients та їх offline behavior залишаються чинними.

## Контекст

Попередні документи розглядали browser/Wasm та gateway як можливий клієнтський профіль. Користувач уточнив 2026-09-20, що браузерної реалізації VIDA не повинно бути: потрібні встановлювані клієнтські застосунки, у яких чати, нотатки/база знань і проєктний застосунок працюють offline.

## Рішення

- Першосторонній клієнт VIDA `MUST` бути встановлюваним застосунком на підтримуваній ОС. VIDA `MUST NOT` планувати браузерний/PWA/Wasm/gateway клієнт як продуктову поверхню або критерій випуску. Конкретний перелік ОС і UI toolkit лишаються `OQ-0008`.
- Messenger, Knowledge/Notes і Projects/Tasks `MUST` мати локальне збереження/читання раніше синхронізованих даних і локальне створення дозволених дій без мережі. «Чат offline» означає доступ до локальної історії й чергу вихідного наміру, **не** доставку відключеному адресату до появи мережі. Зміни, що вимагають іншого authority, лишаються pending до його прийняття.
- Відсутність браузерного клієнта VIDA `MUST NOT` скасовувати публічний вебпортал у **окремому** City Portal, інтеграційні сервіси, вузли, відкриті протоколи або право незалежного розробника створити інший клієнт за публічними специфікаціями. Першосторонній conformance/release target для браузера не створюється.
- Незалежний браузерний клієнт може підтверджувати сумісність із **опублікованим протокольним профілем**, але це не означає офіційну підтримку браузерної платформи VIDA або гарантії offline storage, ключів, background lifecycle чи повноти AppPackage. Gateway-клієнт VIDA не планується; інтеграційний service/sidecar City Portal залишається допустимим.
- Цим рішенням **не обрано** Flutter або Tauri. Tauri створює встановлюваний desktop-застосунок, але відображає HTML у WebView; чи сумісне це з вимогою користувача «native», лишається точним `OQ-0008`, а не прихованим припущенням. Повторне використання браузерного клієнтського UI більше не є аргументом вибору Windows.

## Наслідки

- Видалити browser/Wasm client profile, browser conformance gate і relay-only browser обіцянки з поточних нормативних вимог; зберегти попереднє дослідження як історію, не як план реалізації.
- Кандидат Flutter Windows тепер оцінюється проти Tauri за Windows UX, доступністю, безпекою, підтримкою та вартістю, без бонусу за browser reuse.
- Offline-прототипи для Messenger, Knowledge і Projects перевіряють локальний read/write, pending UX, restart/recovery, права та синхронізацію після reconnect на кожній обраній ОС.

## Безпека та перевірка

[OWASP MASVS](https://mas.owasp.org/MASVS/) вимагає перевіряти mobile storage, crypto, authorization, network, platform та privacy; [Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) підтримує повторну перевірку прав на дію. Це не джерело рішення «без браузерного клієнта». Якщо буде обрано WebView shell, окремо перевірити [Tauri capabilities](https://v2.tauri.app/security/capabilities/) та OWASP MASWE щодо привілейованих native функцій у WebView.
