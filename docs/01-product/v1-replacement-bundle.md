---
id: PROD-V1-BUNDLE-001
status: approved
last_updated: 2026-09-18
source_refs:
  - ../../research/Новий Text Document (6).txt
  - ../../research/Новий Text Document (8).txt
  - ../../research/Новий Text Document (14).txt
  - ../../research/Новий Text Document (15).txt
  - ../../_bmad-output/planning-artifacts/prfaq-vida.md
decision_refs: []
---

# Функціональний bundle першої версії Vida

## Прийняте рішення

Vida першої версії має об'єднати три категорії щоденної функціональності. Це не три ізольовані застосунки: їх композиційна модель визначена в `composable-workspace-model.md`.

### 1. Communications

- персональні повідомлення;
- групові чати;
- структуровані форуми та теми обговорень.

### 2. Knowledge

- персональні нотатки;
- пов'язані документи;
- персональна й спільна база знань.

### 3. Work management

- персональні задачі та проєкти;
- командні задачі та проєкти;
- гнучкі типи, поля, views і workflows;
- явне керування приватністю та спільним доступом.

CRM не входить до цього рішення: під час обговорення його замінено на personal/team project and work management.

## Принцип цілісності

Ці категорії `MUST NOT` бути трьома ізольованими продуктами або лише лінійним переходом «чат → документ → задача».

Messenger, knowledge і work management є capabilities, що комбінуються всередині персональних і спільних контекстів. Проєкт може одночасно містити чати, knowledge, документи й задачі; документ і задача можуть мати власні контекстні обговорення.

Вони `MUST` використовувати спільні:

- identity та Spaces;
- ресурси, relations і attachments;
- permissions і sharing;
- global search і notifications;
- local-first persistence та sync;
- cross-app links: повідомлення ↔ документ ↔ задача ↔ проєкт ↔ forum topic.

City Portal App є окремим інтегрованим застосунком Vida й використовує ці самі платформні механізми.

## Межа рішення

Рішення визначає категорії, але не означає feature parity із Telegram, Discord, Notion, Google Keep, Jira або іншими зрілими продуктами у першому релізі.

Мінімальний корисний capability set, rollout order та acceptance metrics кожної категорії потребують окремого decision gate.

## Відкриті питання

1. Який baseline access отримує новий учасник shared Space від своєї ролі?
2. Які можливості кожної категорії є mandatory для v1, а які переходять у later?
3. Які ресурси можуть бути personal, shared, public або city-visible?
4. Як forum topics співвідносяться з group chats, documents і projects?
5. Чи City Portal App входить у стандартну поставку Vida або встановлюється за міським контекстом?
