---
id: REQ-DIAGNOSTICS-001
status: review
last_updated: 2026-09-22
source_refs:
  - ../../_bmad-output/planning-artifacts/prfaq-vida.md
decision_refs: []
---

# Diagnostics і feedback без централізованого tracking

Це початковий контракт для майбутньої реалізації. Він не означає, що серверна інфраструктура вже існує.

| ID | Вимога |
|---|---|
| `REQ-DIAG-001` | VIDA `MUST` зберігати crash/diagnostic evidence спочатку локально, окремо за Persona, у захищеному bounded storage. |
| `REQ-DIAG-002` | Release 1 `MUST NOT` автоматично відправляти diagnostic bundle без явної згоди користувача. |
| `REQ-DIAG-003` | Після crash користувач `MUST` мати preview перед надсиланням: версія/build, ОС, device class, stack/panic, component state і технічні operation IDs; message/note/file content, contacts, keys, tokens та raw Persona IDs `MUST` бути виключені за замовчуванням. |
| `REQ-DIAG-004` | Користувач `MAY` явно додати опис, screenshot або вибраний контент після окремого попередження. |
| `REQ-DIAG-005` | Основний канал `SHOULD` використовувати вбудований Support Contact/Persona у Messenger: diagnostic bundle є зашифрованим вкладенням до звичайного support conversation і повертає receipt/ticket ID. |
| `REQ-DIAG-006` | Якщо Messenger/Core не може відправити report, client `MUST` дозволити encrypted export bundle через platform share/save flow. |
| `REQ-DIAG-007` | Support recipient `MUST` мати окремі access, retention, audit, key-rotation і deletion rules; support identity `MUST NOT` отримувати доступ до інших Spaces або Personas. |
| `REQ-DIAG-008` | Opt-in background crash upload, якщо буде додано, `MUST` бути окремим режимом від ручного support message, вимкненим за замовчуванням і некорельованим між Personas. |
| `REQ-DIAG-009` | Локальний report і серверна копія `MUST` мати видимі користувачу retention/deletion rules до активації каналу. |

## Відкриті рішення

- хто отримує Support Contact: лише maintainers чи спільнота triage;
- manual-only чи optional background opt-in у Release 1;
- schema diagnostic bundle, redaction tests і retention periods;
- self-hosted support routing та anti-spam/abuse controls.
