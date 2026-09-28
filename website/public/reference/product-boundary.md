---
id: PROD-BOUNDARY-001
status: approved
last_updated: 2026-09-21
source_refs:
  - ../../research/Новий Text Document (2).txt
  - https://github.com/pomazanbohdan/city-portal
  - ../../_bmad-output/planning-artifacts/prfaq-vida.md
decision_refs:
  - ADR-0013
  - ADR-0014
  - ADR-0018
---

# Межа Vida та City Portal

## Прийняте рішення

Vida і City Portal є пов'язаними, але окремими продуктами та codebases.

## Продуктове позиціонування

- Vida `MUST` позиціонуватися для кінцевого користувача як **супер-ап**.
- Головна consumer promise: Vida замінює кілька розрізнених щоденних застосунків одним узгодженим середовищем.
- «Платформа» описує архітектуру, модель розширення та екосистему розробки, але не є головною consumer-facing обіцянкою. Її відкритість охоплює незалежні реалізації клієнтів і вузлів, а також сторонні Apps і плагіни за публічними контрактами та conformance-наборами ([ADR-0013](../03-architecture/decisions/ADR-0013-open-interoperability.md)); платні модулі не обов'язково мають відкриту реалізацію.
- City Portal App лишається великим майбутнім інтегрованим застосунком і потенційним каналом залучення, але не входить до першого публічного релізу VIDA.
- Booking у локального бізнесу є майбутнім proof scenario інтеграції, а не критерієм готовності core v1.
- Перший functional bundle визначено в `v1-replacement-bundle.md`: communications, knowledge і personal/team work management.
- Ці capabilities композиційно формують персональні та спільні середовища згідно з `composable-workspace-model.md`.

| Контур | Відповідальність |
|---|---|
| **Vida** | Open-core платформа: повноцінні встановлювані mobile/desktop clients і статичний local-first Web-клієнт у Release 1 з direct-first peer sync та encrypted Iroh relay fallback без paid Hosted Space; optional paid hosted browser/server-backed mode — пізніший контур; спільне ядро, personal workspace, app runtime, локальні дані, синхронізація та платформні сервіси |
| **City Portal** | Окрема централізована SaaS-платформа місцевих порталів у `pomazanbohdan/city-portal`: public web, partner/tenant administration, content, catalogs і server-side portal capabilities |
| **City Portal App for Vida** | Застосунок, розгорнутий усередині Vida; показує міський контекст і дає користувачу доступ до можливостей порталу без дублювання портального продукту |
| **Integration service** | Окремий сервіс або sidecar, який з'єднує City Portal App із головним City Portal backend; точний deployment та protocol contract ще не затверджені |

## Наслідки

- Vida `MUST NOT` повторно реалізовувати весь City Portal backend.
- City Portal `MUST` залишатися самостійно розгортаним і корисним без клієнтів Vida.
- Публічний web City Portal лишається окремим продуктом, а не browser mode VIDA. Release-1 Web VIDA визначає [ADR-0021](../03-architecture/decisions/ADR-0021-static-web-client-in-release-1.md), який у цій частині supersedes історичний [ADR-0018](../03-architecture/decisions/ADR-0018-browser-local-and-hosted-modes.md); встановлювані клієнти зберігають offline scope ADR-0014.
- City Portal App `MUST` використовувати стабільний інтеграційний контракт, а не напряму залежати від внутрішніх таблиць порталу.
- Технологічні рішення репозиторію City Portal не стають автоматично рішеннями ядра Vida.
- Користувацькі, бізнесові, міські та платформні ідентичності мають бути явно зіставлені; tenant/domain не визначає ownership даних Vida автоматично.
- Перший реліз перевіряє core-сценарій: Persona/Profile → активація Messenger і Notes/Knowledge → підключення пристрою та sync → створення Project App/групи → спільна робота із задачами, нотатками, чатами, форумами, файлами та дзвінками.
- City Portal і booking не повинні затримувати готовність цього core-релізу.

## Відкриті питання

1. Чи integration service розгортається поруч із кожним City Portal deployment, централізовано у Vida cloud або підтримує обидва профілі?
2. Яка система є source of truth для business profile, availability, booking і notification state?
3. Як зіставляються City Portal account, Vida identity, business membership і city partner roles?
4. Які операції працюють offline, а які потребують server authority?
5. Чи sidecar є окремим deployable service, модулем City Portal backend або protocol adapter у Vida service layer?

## Evidence required

- Context diagram двох систем і trust boundaries.
- Sequence diagram resident → Vida App → City Portal App → integration service → City Portal backend.
- Data ownership matrix.
- API/event contract і versioning policy.
- Failure, retry, idempotency та offline behavior для booking-сценарію.
