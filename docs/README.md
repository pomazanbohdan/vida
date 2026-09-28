---
id: DOCS-INDEX
status: review
last_updated: 2026-09-25
---

# Vida documentation

Цей каталог є майбутньою нормативною документацією проєкту Vida.

Поточну карту опрацьованих і відкритих контурів див. у [contour-status.md](00-governance/contour-status.md).

Контактна модель і platform address-book connectors описані у [contact-card-requirements.md](02-requirements/contact-card-requirements.md). Release-1 набір 18 мов/21 locale profiles — у [localization-requirements.md](02-requirements/localization-requirements.md). Privacy/GDPR baseline — у [privacy-compliance-requirements.md](02-requirements/privacy-compliance-requirements.md). Local-first crash/feedback contract — у [diagnostics-feedback-requirements.md](02-requirements/diagnostics-feedback-requirements.md). Опційна платна always-online replica/storage модель — у [hosted-space-service-model.md](01-product/hosted-space-service-model.md). Статичний синхронізований Web включено до Release 1 за [ADR-0021](03-architecture/decisions/ADR-0021-static-web-client-in-release-1.md); browser security/implementation лишаються `OQ-0075`.

Попередні хвилі запитань та відповіді зібрано в [decision-batch-2026-09-20.md](00-governance/decision-batch-2026-09-20.md) і [decision-batch-app-center-2026-09-20.md](00-governance/decision-batch-app-center-2026-09-20.md). AppCenter/платний Hosted Space відкладені, але статичний Web-клієнт тепер у Release 1; історичні відповіді у [decision-batch-device-core-2026-09-20.md](00-governance/decision-batch-device-core-2026-09-20.md) читаються з урахуванням ADR-0021.

[Контур прямої device sync-сесії](04-specifications/device-sync-session.md) описує дослідницьку послідовність і відкрите schema-level правило для конкурентних serverless сесій. Продуктова поведінка конфліктів зафіксована в [transport/sync requirements](02-requirements/transport-sync-requirements.md); контур сесії ще не є фінальним wire protocol.
Пристрої однієї Persona рівноправні за [ADR-0016](03-architecture/decisions/ADR-0016-equal-device-peers.md): жоден не є Owner-арбітром. Для роз'єднаних конкурентних змін глобального «першого sync» не визначено; schema-level правило збіжності лишається відкритим.

Порівняння кандидатів реалізаційного стека та його перевірочні сценарії — у [stack-selection-brief.md](03-architecture/stack-selection-brief.md); це `draft`, а не затверджений UI/CRDT/runtime вибір.

CRDT/editor contour має окремі [research](../_bmad-output/planning-artifacts/research/technical-vida-crdt-editor-stack-2026-09-22/research.md), [draft conformance spec](04-specifications/crdt-editor-conformance.md) і [machine-readable fixtures](04-specifications/fixtures/crdt-editor-v1.yaml). Live cursors є Release-1 gate; конкретний engine/editor pair ще не обрано.

E2EE calls contour має окремі [research](../_bmad-output/planning-artifacts/research/technical-vida-e2ee-media-stack-2026-09-22/research.md), [draft conformance spec](04-specifications/e2ee-calls-conformance.md) і [F01–F17 candidate fixtures](04-specifications/fixtures/e2ee-calls-v1.yaml). Iroh лишається control/key plane; media profile ще не обрано, fixtures не є виконаним proof.

За [ADR-0020](03-architecture/decisions/ADR-0020-flutter-windows-in-release-1.md) і [ADR-0021](03-architecture/decisions/ADR-0021-static-web-client-in-release-1.md) Release 1 охоплює Android/iOS/Windows Flutter і статичний Flutter Web. Окремий WinUI 3/C# client можливий після Release 1.

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
- Identity multi-axis model: `approved`; зафіксована в `ADR-0004`, `REQ-IDENTITY-001` і `SPEC-IDENTITY-001`.
- Iroh transport і durable-delivery boundary: `approved`; зафіксовані в `ADR-0005`, `ADR-0006`, `REQ-TRANSPORT-SYNC-001`, `NFR-PLATFORM-001` та п'яти component contracts.
- App-package baseline: `approved` у `ADR-0007`–`ADR-0012`, `REQ-APP-PACKAGE-001` і `REQ-EFFECT-001`–`REQ-EFFECT-011`; технічний `SPEC-APP-PACKAGE-001` у review до рішень про executor/hook continuation, distribution trust та update lifecycle. Підтверджені обмеження: [effect-execution-constraints.md](02-requirements/effect-execution-constraints.md). Межі майбутнього довіреного центру застосунку — [app-center-boundaries.md](02-requirements/app-center-boundaries.md), `review`.
- Client release boundary: [ADR-0020](03-architecture/decisions/ADR-0020-flutter-windows-in-release-1.md), [ADR-0021](03-architecture/decisions/ADR-0021-static-web-client-in-release-1.md), [REQ-CLIENT-001–029](02-requirements/native-client-requirements.md) і [REQ-BROWSER-001–008](02-requirements/browser-client-requirements.md) фіксують Android/iOS/Windows та статичний Web у Release 1; WinUI 3 і paid Hosted Space після нього.
- [PRFAQ](../_bmad-output/planning-artifacts/prfaq-vida.md): Stage 5, `complete`; це завершена продуктова основа, а не дозвіл на реалізацію без наступних контрактів.
- [PRD Release 1](../_bmad-output/planning-artifacts/prds/prd-vida-2026-09-22/prd.md): `final` як продуктовий контракт; UX є наступним документом для завершення, а implementation fan-out окремо залежить від відкритих R-2 контрактів і доказів.
- [UX DESIGN.md](../_bmad-output/planning-artifacts/ux-designs/ux-vida-2026-09-22/DESIGN.md) і [EXPERIENCE.md](../_bmad-output/planning-artifacts/ux-designs/ux-vida-2026-09-22/EXPERIENCE.md): `discovery`; затверджено щільність/порядок блоків трьох мобільних концептів, але не глобальні візуальні токени та навігацію.
- [Architecture Spine](../_bmad-output/planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md): `final`, межі архітектури accepted; implementation і interoperability залишаються prototype/conformance-gated.
