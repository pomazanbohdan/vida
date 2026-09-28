---
id: GOV-ROADMAP
status: review
last_updated: 2026-09-22
---

# Послідовність обговорень

| № | Контур | Ключовий результат |
|---:|---|---|
| 00 | Governance | Правила, статуси, шаблони, traceability |
| 01 | Product boundary і v1 | Vision, actors, use cases, MVP, non-goals |
| 02 | Glossary і domain | Space, Resource, Container, App та їхні інваріанти |
| 03 | Trust і quality attributes | Threat assumptions, offline, privacy, performance, availability |
| 04 | Identity і membership | Person, account, profile, device, keys, recovery, federation |
| 05 | Authorization і governance | Policies, capabilities, authority, offline acceptance |
| 06 | PDM і change protocol | Schemas, IDs, versions, operations, conflicts, migrations |
| 07 | Storage | Binary profile, files, projections, blobs, search, backup |
| 08 | Sync і network | Межа Iroh, node profiles, replication, recovery |
| 09 | App і workflow runtime | Packages, WIT/Wasm, scripts, sandbox, durable execution |
| 10 | Messaging і community | Chatmail/native boundary, groups, channels, forums, moderation |
| 11 | UX і client architecture | Shell, navigation, renderer/framework, accessibility, offline UX |
| 12 | System apps і edge scope | Tasks, notes, PM, portal, ESP32 та later scope |
| 13 | Deployment і operations | Environments, observability, upgrades, key/service operations |
| 14 | Verification | Acceptance, conformance, security, performance, compatibility |
| 15 | Vertical slice | Перший наскрізний increment і implementation roadmap |

Порядок може змінюватися лише після impact review. Контури з невирішеними залежностями не переводяться в `decision-ready`.

## Поточний стан контуру 04

Multi-axis identity model матеріалізовано в `ADR-0004`, `REQ-IDENTITY-001` і `SPEC-IDENTITY-001`. Контур лишається частково відкритим щодо controller method/history, anonymous granularity, recovery contract, alias governance і node-visible metadata (`OQ-0021`–`OQ-0026`).

## Поточний стан контуру 08

Iroh transport foundation, durable-delivery boundary і основну парадигму `Headless local-first shared core with native/platform shells and ports-and-adapters at external seams` матеріалізовано в `ADR-0005`, `ADR-0006`, `REQ-TRANSPORT-SYNC-001`, `NFR-PLATFORM-001`, component contracts та `ARCHITECTURE-SPINE.md`. Контур лишається частково відкритим щодо production mailbox topology, canonical operation envelope, blob encryption, platform-specific Address Lookup policy, membership ordering, delivery aggregation, domain authority, deterministic sync semantics, platform bindings, storage-provider contract і mixed-version rollout (`OQ-0027`–`OQ-0037`).

## Поточний стан контуру 09

Користувач підтвердив окремо завантажувані `AppPackage`, bundled Messenger, Knowledge/Notes і Projects/Tasks через спільний runtime (`ADR-0007`), а також VIDA marketplace, зовнішні репозиторії та update discovery (`ADR-0008`). Пакети можуть містити керовану прикладну логіку й розширювати, замінювати або вимикати визначену поведінку базового застосунку (`ADR-0009`), але лише в цільовому `AppInstance` конкретного Space (`ADR-0010`). Процес instance має пріоритет над базовим handler на тому самому тригері; композиція власних дій — відповідальність розробника, не Owner-level arbitration (`ADR-0011`). Новий command, committed fact, timer/external signal і UI projection розділено; sync receive/replay не повторює бізнес-команду (`ADR-0012`). Для автономного personal Space зовнішня дія може чекати онлайн-пристрою, а в приватному федеративному проєкті вузол дає лише непрозорий сигнал. Одне сповіщення доступне на всіх пристроях; наслідки дії й статус «прочитано» синхронізуються, а системні банери прибираються best-effort на підтримуваних ОС (`REQ-EFFECT-001`–`REQ-EFFECT-006`). Довільний код із прямим OS/core-доступом не входить до baseline; Iroh — транспорт, не виконавець плагінів. Installation vs Space activation, signing/rollback, source trust, repository format і детальна hook semantics, зокрема executor для post-commit effects, відкриті (`OQ-0039`–`OQ-0045`); вибір Rhai/Wasmtime/WIT лишається окремим (`OQ-0009`).

## BMad handoff після PRFAQ (2026-09-22)

| Артефакт | Поточний стан | Умова наступного переходу |
|---|---|---|
| [PRFAQ](../../_bmad-output/planning-artifacts/prfaq-vida.md) | Продуктове бачення й Release-1 boundary завершені | Є джерелом PRD, повторно не відкривати без нового продуктового рішення |
| [PRD](../../_bmad-output/planning-artifacts/prds/prd-vida-2026-09-22/prd.md) | Final як продуктовий контракт після рішень щодо Contacts, Calendar, Search і updates | OQ-2 CRDT/editor, OQ-3 E2EE media та решта R-2 contracts потребують відтворюваних доказів до implementation fan-out |
| [EXPERIENCE.md](../../_bmad-output/planning-artifacts/ux-designs/ux-vida-2026-09-22/EXPERIENCE.md) | Поведінковий draft: IA, UJ-1–UJ-5, offline/sync/conflict, sharing, calls, accessibility | Перегляд мобільної/Windows навігації та ключових екранів |
| [DESIGN.md](../../_bmad-output/planning-artifacts/ux-designs/ux-vida-2026-09-22/DESIGN.md) | Discovery: візуальні вимоги записані, brand/tokens ще не обрані | Вибір користувачем візуального орієнтира й перевірка контрасту, локалізації, світлої/темної тем |
| [Architecture Spine](../../_bmad-output/planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md) | Узгоджений із PRD/UX та Flutter Windows; три незалежні рецензії враховані, lint 0, [disposition](../../_bmad-output/planning-artifacts/architecture/architecture-vida-2026-09-19/reviews/review-disposition-2026-09-22.md) | OQ-2/OQ-3 і точний call-answer proof OQ-0072 залишаються prototype/specification gates; це не public-release approval |
| [Resource conformance](../04-specifications/platform-resource-conformance.md) | G0 метод і профілі затверджені | G1 фізичний baseline → G2 числові бюджети до feature-complete |

Наступна послідовність: переглянути UX choice/mock; специфікувати OQ-0072 Core call-answer proof; виконати OQ-2/OQ-3 відтворювані прототипи за fixtures; зафіксувати selection ADR; створити epics/stories і sprint status для implementation fan-out. G1 baseline збирається на репрезентативному vertical slice, а числові G2 бюджети затверджуються до feature-complete gate. Розбивку можна готувати раніше як чернетку з явними залежностями, але не оголошувати implementation-ready. BMad маршрут — PRFAQ → PRD → UX → Architecture → Epics/Stories → Sprint Planning; вже завершені етапи не повторюємо.
