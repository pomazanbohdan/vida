---
id: DOCS-INDEX
status: review
last_updated: 2026-09-18
---

# Vida documentation

Цей каталог є майбутньою нормативною документацією проєкту Vida.

`../research/` містить дослідження, стенограми, гіпотези та архітектурні пропозиції. Матеріал із `research/` не стає вимогою або рішенням автоматично.

## Принцип роботи

1. Обираємо один архітектурний або продуктовий контур.
2. Вичитуємо пов'язані джерела й фіксуємо факти, кандидати, суперечності та прогалини.
3. Codex ставить користувачу обов'язкові decision questions із варіантами, рекомендацією та наслідками.
4. За потреби виконуємо додаткове дослідження або прототип.
5. Рішення набуває нормативної сили лише після явного підтвердження користувача.
6. У тому самому циклі оновлюємо вимоги, ADR, специфікації, traceability та відкриті питання.

Мовчання, відсутність заперечень або текст у `research/` не вважаються прийняттям рішення.

## Структура

| Каталог | Призначення |
|---|---|
| `00-governance/` | Процес, порядок обговорень, джерела й відкриті питання |
| `01-product/` | Vision, scope, non-goals, use cases; створюється після відповідного контуру |
| `02-requirements/` | Функціональні та нефункціональні вимоги |
| `03-architecture/` | Огляд архітектури, domain model та ADR |
| `04-specifications/` | Реалізаційні контракти й технічні специфікації |
| `05-verification/` | Acceptance, conformance, security і performance evidence |
| `_templates/` | Шаблони робочих документів |

Каталоги `01`–`05` наповнюються лише після проходження відповідних decision gates.

## Пріоритет джерел

1. Явно підтверджене рішення користувача й accepted ADR.
2. Approved specification.
3. Approved architecture/requirements document.
4. Research evidence та чернетки.

У разі суперечності нижчий рівень не переписує вищий. Зміна прийнятого рішення оформлюється новим ADR, а попередній документ отримує статус `superseded`.

## Статуси

- Документ: `draft | review | approved | superseded`.
- Реалізація: `unplanned | planned | partial | implemented | verified`.
- ADR: `proposed | accepted | rejected | superseded`.
- Контур: `backlog | scoped | researched | decision-ready | accepted | materialized | reviewed | blocked | deferred`.

## Поточний стан

- Governance scaffold: `review`.
- Product boundary і композиційна Space-модель: `approved`.
- Access-control requirements: `approved`; layered model зафіксована в `ADR-0001`.
- PRFAQ: Stage 2, press-release draft.
