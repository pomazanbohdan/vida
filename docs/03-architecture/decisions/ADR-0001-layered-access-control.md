---
id: ADR-0001
status: accepted
last_updated: 2026-09-18
source_refs:
  - ../../01-product/composable-workspace-model.md
  - ../../02-requirements/access-control-requirements.md
  - ../../../research/vida-current-architecture.md
  - ../../../research/Новий Text Document (10).txt
supersedes: []
superseded_by: []
---

# ADR-0001: Багаторівнева модель доступу

## Context

Shared Space поєднує Messenger, Knowledge/Notes, Projects/Tasks та інші schema-driven apps. Один учасник повинен мати різні права в різних apps, projects, resource types і конкретних resources. Налаштування кожного нового ресурсу вручну не масштабується, а одна роль на весь Space є надто грубою.

## Decision drivers

- least privilege;
- зрозуміле керування для Owner;
- fine-grained доступ без сотень ручних правил;
- підтримка довільних app schemas і actions;
- однакова server/local перевірка;
- пояснювані effective permissions.

## Considered options

1. Одна роль на весь Space — відхилено як недостатньо granular.
2. Окремі права для кожного resource — відхилено як непридатне для щоденного UX.
3. Role presets, inheritance, permission matrix і resource overrides — прийнято.

## Decision

### 1. Role preset

Під час додавання учасника Owner `MUST` призначити готову або custom role. Role preset задає стартовий набір capabilities, але не замінює policy engine.

### 2. Основний рівень налаштування

Звичайний permission UX `MUST` відкриватися в контексті `AppInstance` або `Container`, наприклад `Knowledge у Project X`.

Матриця `MUST` мати:

- рядки — schema/resource types;
- колонки — actions, щонайменше `create/read/update/delete`;
- cells — `inherit/allow/deny` або еквівалентний стан із видимим джерелом правила.

### 3. Успадкування

Policy `MUST` успадковуватися за ланцюгом:

```text
Platform → Space → AppInstance → Container → ResourceType → Resource → Field → Action
```

Нижчий рівень `MUST NOT` розширювати доступ понад maximum capabilities, дозволені вищим рівнем. Явний hard deny `MUST` мати перевагу.

### 4. Resource override

Owner або уповноважений адміністратор `MAY` додати виняток для конкретної note, document, task, chat або іншого resource. Це винятковий механізм, а не стандартний спосіб налаштування кожного об'єкта.

### 5. Базова комунікація

Учасник `MUST` мати `read/send` у доступних йому чатах, якщо policy конкретного чату не звужує ці права. Доступ до чату не надає автоматичного доступу до пов'язаних notes, documents або tasks.

### 6. Enforcement

- app schema `MUST` декларувати resource types і domain actions;
- UI `MUST` показувати effective permission та джерело правила;
- backend/authority node `MUST` повторно перевіряти кожну command;
- приховування UI `MUST NOT` вважатися security control;
- зміни policy `MUST` бути auditable і revocable.

## Rationale

Модель поєднує швидкість готових ролей, керованість matrix на рівні app/project і точність окремих винятків. Вона ближча до багаторівневої granularity monday.com, але узагальнена для довільних Vida schemas і resources.

## Consequences

- permission editor потребує matrix UI та пояснення inheritance;
- policy engine має обчислювати effective permissions детерміновано;
- кеші й offline grants мають інвалідуватися після revocation;
- apps не реалізовують власні несумісні ACL-моделі.

## Risks і mitigations

- Надмірна складність → progressive disclosure та role presets.
- Приховане успадкування → показ джерела кожного effective rule.
- Витік через relation → незалежна перевірка доступу до кожного linked resource.
- Застарілі offline grants → expiry, key epochs, revocation sync і server recheck.

## Acceptance evidence

- тест role preset → inherited permissions;
- тест matrix override на app/container;
- тест resource-level exception;
- тест hard deny precedence;
- тест заборони linked-resource traversal;
- тест server rejection попри змінений клієнтський UI;
- audit trail для grant, change і revoke.

## Open questions

- Семантику offline revocation визначено в `ADR-0003`; maximum offline lease lifetime залишається відкритим у `OQ-0019`.

Каталог role presets визначено в `ADR-0002`.
