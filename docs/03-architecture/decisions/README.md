---
id: ADR-INDEX
status: review
last_updated: 2026-09-25
---

# Architecture Decision Records

| ADR | Рішення | Статус |
|---|---|---|
| [ADR-0001](ADR-0001-layered-access-control.md) | Багаторівнева модель доступу | accepted |
| [ADR-0002](ADR-0002-default-role-presets.md) | Стартовий каталог ролей | accepted |
| [ADR-0003](ADR-0003-offline-revocation.md) | Відкликання доступу offline | accepted |
| [ADR-0004](ADR-0004-multi-axis-identity-model.md) | Multi-axis identity model | accepted |
| [ADR-0005](ADR-0005-iroh-transport-foundation.md) | Iroh transport foundation і VIDA protocol boundary | accepted |
| [ADR-0006](ADR-0006-durable-delivery-and-operation-envelope.md) | Durable delivery і unified operation envelope | accepted |
| [ADR-0007](ADR-0007-declarative-app-packages.md) | Declarative AppPackages і Space-scoped AppInstances | accepted |
| [ADR-0008](ADR-0008-package-distribution-channels.md) | VIDA marketplace, зовнішні репозиторії та update discovery | accepted |
| [ADR-0009](ADR-0009-managed-application-logic.md) | Керована прикладна логіка та розширення базових застосунків | accepted |
| [ADR-0010](ADR-0010-instance-scoped-app-logic.md) | Логіка розширення лише в цільовому AppInstance/Space | accepted |
| [ADR-0011](ADR-0011-instance-trigger-precedence.md) | Пріоритет процесу AppInstance над базовим тригером | accepted |
| [ADR-0012](ADR-0012-command-event-sync-boundary.md) | Межа бізнес-команди, зафіксованого факту та sync apply | accepted |
| [ADR-0013](ADR-0013-open-interoperability.md) | Відкриті контракти для незалежних клієнтів, вузлів і застосунків | accepted |
| [ADR-0014](ADR-0014-native-only-vida-clients.md) | Встановлювані клієнти VIDA; browser exclusion superseded ADR-0018/0021 | accepted / partially superseded |
| [ADR-0015](ADR-0015-v1-device-platforms-and-mobile-flutter.md) | Android/iOS/Windows у v1; Flutter mobile UI | refined by ADR-0020 |
| [ADR-0016](ADR-0016-equal-device-peers.md) | Рівноправні пристрої користувача; немає device-арбітра для sync | accepted |
| [ADR-0017](ADR-0017-concurrent-status-file-conflict.md) | Явний multi-value conflict для конкурентних статусів задачі й замін файла | accepted |
| [ADR-0018](ADR-0018-browser-local-and-hosted-modes.md) | Історична free local-only/paid hosted browser модель | partially superseded by ADR-0021 |
| [ADR-0019](ADR-0019-mobile-release-1-and-post-release-windows.md) | Android/iOS у Release 1; Windows після Release 1 | superseded by ADR-0020 |
| [ADR-0020](ADR-0020-flutter-windows-in-release-1.md) | Flutter Android/iOS/Windows у Release 1; WinUI 3 після | accepted |
| [ADR-0021](ADR-0021-static-web-client-in-release-1.md) | Статичний синхронізований Flutter Web/Rust-Wasm у Release 1 | accepted |

Нумерація послідовна: `ADR-0001-short-title.md`. Рішення не створюється як `accepted` без явного підтвердження користувача.
