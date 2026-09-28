---
id: REQ-APP-CENTER-001
status: review
last_updated: 2026-09-20
decision_refs:
  - ../03-architecture/decisions/ADR-0004-multi-axis-identity-model.md
  - ../03-architecture/decisions/ADR-0014-native-only-vida-clients.md
---

# Межі довіреного центру застосунку

## Погоджена продуктова ціль

- Федеративний deployment може суміщати identity/account, relay, sync/replica, catalog і центр застосунку, але це різні capabilities. Саме суміщення не видає ключі Space та не призначає authority.
- Центр застосунку може бути **окремо довіреним пристроєм/учасником синхронізації** з доступом до дешифрування даних тих Spaces, для яких йому належно надано ключі. Це не загальне право будь-якого федеративного вузла на plaintext.
- Постійна доступність центру робить його бажаним онлайн peer для sync. Це не доводить, що він має остаточний пріоритет при прийнятті операцій, статусі задачі чи зміні прав; такі правила окремо визначає authority contract.
- Можливий вебінтерфейс, розміщений поруч із центром, є **майбутнім** контуром. Публічний v1 VIDA лишається без browser/PWA клієнта за ADR-0014.
- Кожна дія людини через майбутній вебінтерфейс має бути автентифікована та авторизована **як дія цієї людини** для конкретного Space/ресурсу/операції. Доступ центру до ключів не перетворює користувача на Owner і не дозволяє виконувати його команди з усіма правами центру. Аудит має відрізняти людину-ініціатора від сервера-виконавця.

## Не затверджено

Точний enrollment, суб'єкт/роль центру, розподіл ключів і історичного доступу, ізоляція між Spaces, повноваження автономних реакцій, пріоритет sync/authority, failover, web auth/session TTL, key rotation після компрометації та межі операторського доступу — відкриті питання `OQ-0026`, `OQ-0033`, `OQ-0045`, `OQ-0057`–`OQ-0062`. Жодні запропоновані числа для вебсесій (24 години / тиждень) поки не є вимогою.

## Референси для наступного рішення

- [Iroh](https://www.iroh.computer/) забезпечує зашифрований транспорт; application authorization лишається окремою межею.
- [AT Protocol: PDS, Relay, AppView](https://atproto.com/guides/the-at-stack) показує розділення функцій сервера; це не готова trust model VIDA.
- [OWASP Authorization](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) вимагає перевіряти права на кожну дію; [OWASP Key Management](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html) — інвентаризації та захисту ключів.
- [RFC 8693](https://www.rfc-editor.org/rfc/rfc8693.html) розрізняє суб'єкта, від імені якого діють, та виконавця; це reference pattern, не рішення про OAuth у VIDA.
- [OWASP Session Management](https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html) відокремлює idle й absolute timeout і не встановлює універсального 24h/7d для VIDA.
