---
id: ADR-0007
status: accepted
last_updated: 2026-09-19
source_refs:
  - ../../01-product/composable-workspace-model.md
  - ../../../research/vida-current-architecture.md
  - ../../../_bmad-output/planning-artifacts/research/technical-vida-ui-platform-and-rust-binding-strate-2026-09-19/research.md
decision_refs: []
supersedes: []
superseded_by: []
---

# ADR-0007: Declarative AppPackages and Space-scoped AppInstances

## Context

VIDA має постачати базові застосунки й дозволяти окремо завантажувати та підключати інші. Ухвалена композиційна модель уже розділяє пакет застосунку й `AppInstance` у Space. Потрібно зафіксувати, чи зовнішній пакет є описом можливостей спільного runtime, чи довільним виконуваним клієнтським плагіном.

## Decision drivers

- один domain/authorization/sync contract для базових і зовнішніх застосунків;
- встановлення нового декларативного застосунку без нового релізу клієнта;
- переносимість між клієнтськими профілями без дублювання бізнес-семантики;
- контроль прав і сумісності на межі пакета.

## Considered options

1. Кожен застосунок — окремий hard-coded модуль клієнта.
2. `AppPackage` описує схеми й поведінку, яку реалізує спільний runtime.
3. Зовнішній пакет приносить довільний виконуваний код на пристрій.

## Decision

VIDA v1 `MUST` використовувати модель 2: `AppPackage` є переносним декларативним описом schemas, relations, commands, workflows, permission declarations, UI/data ports і звернень до версійованих можливостей runtime. Пакет `MUST NOT` самовільно вводити нову domain, protocol або authorization semantics чи отримувати права лише через факт встановлення.

Messenger, Knowledge/Notes і Projects/Tasks `MUST` постачатися разом із VIDA як базові пакети й формувати відповідні `AppInstance` у стандартному Space. Зовнішній сумісний пакет `MUST` мати змогу завантажуватися й підключатися без перевипуску клієнта, якщо він спирається на вже наявні можливості runtime. Базові та зовнішні пакети `MUST` проходити один package/instance contract; окремої привілейованої моделі даних або доступу для базових застосунків немає.

`AppInstance` `MUST` бути окремою Space-scoped активацією пакета. Дані, ownership, grants і sync залишаються в межах відповідного Space; один пакет може мати різні instances у різних Spaces. Виконувана логіка спільних primitives належить Rust core/runtime, а клієнтські shells відповідають за представлення й OS-інтеграцію згідно з чинною architecture spine.

Це рішення `MUST NOT` трактувати як схвалення довільних завантажуваних Dart/native/JS/Wasm-плагінів у v1. Декларативні rules/workflows, які інтерпретує runtime, допускаються; вибір механізму executable extensions лишається відкритим окремим контуром.

## Rationale

Одна модель пакета зберігає композиційність супер-апу й дозволяє нові app experiences без розгалуження ядра. Розділення пакета та інстансу не змішує розповсюдження застосунку з владою над даними Space. Кодонесучі плагіни мають іншу модель довіри, сумісності й дистрибуції, тому не успадковуються з цього рішення.

## Consequences

- потрібні версійні package schema та capability negotiation для клієнтських профілів;
- built-in застосунки не повинні обходити package/instance, permission і sync contracts;
- зовнішній пакет може описувати UI й workflows, але не замінює City Portal backend або його authority;
- правила джерел, підписів, активації, оновлення та міграцій потребують окремих рішень.

## Risks і mitigations

- package посилається на недоступну capability → явна помилка сумісності, не тиха зміна поведінки;
- renderer по-різному тлумачить декларації → спільні conformance fixtures для кожного підтримуваного встановлюваного клієнта; first-party browser profile скасовано [ADR-0014](ADR-0014-native-only-vida-clients.md);
- package намагається обійти Space policy → runtime перевіряє кожну команду незалежно від стану UI;
- package update порушує старі дані → міграції й rollback лишаються production gate, не припущенням цього ADR.

## Acceptance evidence

- стандартний Space отримує три базові `AppInstance` через той самий contract, що й зовнішній пакет;
- незмінений клієнт завантажує сумісний тестовий package, показує його форму та виконує дозволений workflow;
- один package активовано у двох Spaces без змішування ресурсів, grants або sync;
- пряма спроба виконати заборонену команду відхиляється runtime, навіть якщо UI її показав;
- несумісна declaration отримує типізовану помилку.

## Open questions

- Керовану прикладну логіку й розширення поведінки базових застосунків уточнено в пізнішому [ADR-0009](ADR-0009-managed-application-logic.md); вибір виконавця відкритий.
- дозволені джерела визначено пізнішим [ADR-0008](ADR-0008-package-distribution-channels.md); publisher/source trust відкритий;
- хто і на якому рівні отримує package та активує `AppInstance`;
- package signatures, version pinning, update, migration і rollback;
- точний DSL/schema interpreter та майбутній sandbox для code-bearing extensions.
