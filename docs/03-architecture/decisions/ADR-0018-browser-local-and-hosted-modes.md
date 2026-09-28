---
id: ADR-0018
status: accepted
last_updated: 2026-09-22
decision_refs:
  - ADR-0013
  - ADR-0014
supersedes:
  - ADR-0014-browser-exclusion
superseded_by:
  - ADR-0021
---

# ADR-0018: browser має local-only та hosted режими

> Історичний baseline: [ADR-0021](ADR-0021-static-web-client-in-release-1.md) замінює заборону free peer sync та post-v1 відкладення браузера. Release 1 має статичний local-first Web із peer sync без платного Hosted Space; hosted service лишається окремою пізнішою можливістю.

## Контекст

ADR-0014 виключив browser/PWA як офіційну VIDA surface. 2026-09-22 користувач змінив цю продуктову межу: browser потрібен у двох режимах, причому free mode зберігає дані локально у browser storage, а paid mode працює із server-hosted synchronization.

## Рішення

- Free browser mode є local-only: дані належать поточному browser origin/profile і не синхронізуються із серверами, peers або іншими пристроями.
- Paid hosted browser mode працює через опційний Hosted Space service і server-backed sync.
- Installed Android/iOS/Windows clients лишаються повноцінними основними клієнтами. Прийняття browser product modes не означає автоматичної feature parity або включення обох режимів у перший release increment.
- Exact browser persistence, encryption/key custody, export/restore, quota/eviction, service-worker і hosted session contracts залишаються implementation/security gates.

## Наслідки

- Заборона browser product surface з ADR-0014 та `REQ-CLIENT-001` більше не чинна; решта installed-client/offline вимог ADR-0014 зберігається.
- UI повинен чітко відрізняти `локально в цьому браузері` від `синхронізовано через Hosted Space`.
- Paid plan variants і hosted retention є post-v1 business/deployment contours.

## Безпека

Browser storage не є platform keystore. [OWASP HTML5 Security](https://cheatsheetseries.owasp.org/cheatsheets/HTML5_Security_Cheat_Sheet.html) вимагає врахувати XSS, origin isolation, untrusted IndexedDB data й окремий захист secrets. Детальні controls фіксує майбутній browser security contract.
