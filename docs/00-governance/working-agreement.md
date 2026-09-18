---
id: GOV-WORKFLOW
status: review
last_updated: 2026-09-18
source_refs:
  - ../../research/Новий Text Document.txt
decision_refs: []
---

# Робочий процес документації

## Одиниця роботи

Один цикл охоплює один bounded contour: продукт, domain, identity, authorization, data model, sync, storage, runtime, messaging, UX або verification.

## Цикл контуру

1. **Scope card** — проблема, користувачі, межі, залежності, наявні твердження й суперечності.
2. **Evidence digest** — кожне твердження має статус `fact`, `accepted`, `candidate`, `rejected`, `stale` або `unknown`.
3. **Інтерв'ю** — 3–5 питань за раунд; кожне містить context, 2–4 options, recommendation і consequences.
4. **Research/prototype** — обов'язковий, якщо доказів недостатньо для незворотного або дорогого рішення.
5. **Decision brief** — варіанти, trade-offs, ризики, reversibility, evidence та відкриті unknowns.
6. **Decision gate** — лише явна відповідь користувача переводить рішення в `accepted`.
7. **Materialization** — одразу оновлюються canonical document, ADR, specification, traceability та open questions.
8. **Consistency review** — перевіряються ID, терміни, посилання, залежності й суперечності.
9. **User review** — контур стає `reviewed`; лише тоді переходимо до залежного контуру.

## Обов'язкові питання

Для кожного контуру необхідно визначити:

- яку проблему й для кого розв'язуємо;
- що входить і не входить у scope;
- інваріанти, обмеження та non-goals;
- MVP проти later;
- trust, authority й offline assumptions;
- ownership, lifecycle, consistency та recovery даних;
- security, privacy, interoperability й migration;
- вимірювані quality attributes;
- acceptance evidence;
- що свідомо лишається відкритим.

## Нормативний запис

Прийнятий пункт повинен мати стабільний ID, `MUST/SHOULD/MAY`, rationale, consequences, acceptance criteria, source links, дату та статус реалізації.

## Definition of Done

Контур завершено, коли:

- користувач явно прийняв рішення;
- суперечності вирішені або явно deferred із причиною й exit condition;
- вимоги вимірювані;
- ADR і специфікації пов'язані з джерелами та acceptance criteria;
- ризики й відкриті питання названі;
- залежності наступного контуру задоволені.

## Change control

Прийняту історію не стираємо. Зміна проходить impact analysis, новий decision gate і новий ADR, який supersede-ить попередній.

