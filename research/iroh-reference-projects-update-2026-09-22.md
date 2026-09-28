---
title: "Актуалізація Iroh reference projects — 22 вересня"
type: research-addendum
status: user-supplied-research-verified-readme
snapshot: "2026-09-21 19:50–2026-09-22 18:37 Europe/Kyiv"
source: "Користувацький GitHub-звіт; repository READMEs перевірені 2026-09-22"
---

# Актуалізація Iroh reference-проєктів: 21–22 вересня 2026

Цей зріз продовжує [каталог станом на 18.09](iroh-local-first-research-index-2026-09-18.md) і [addendum 19–21.09](iroh-reference-projects-update-2026-09-21.md). Період нового пошуку за наданим звітом: 21.09 19:50 — 22.09 18:37, Europe/Kyiv. Стан і технічні характеристики нижче звірені з GitHub README 22.09; це README-level review, не code/security audit.

## Нові репозиторії з реальною Iroh-реалізацією

| Проєкт | Що підтверджує README | Reference для VIDA | Межі доказу |
|---|---|---|---|
| [echo](https://github.com/sergey-melnychuk/echo) | Serverless folder-sync PoC на Iroh 1.x, `iroh-docs`, `iroh-blobs`, Automerge; text merge, content-defined chunks для інших файлів, Biscuit admission tokens з TTL і optional `EndpointId` binding. README описує один Router для кількох протоколів, deterministic winner для binary conflicts зі збереженням losing copy. | Найближчий новий file/workspace sync reference: один Endpoint із кількома Iroh protocols, domain merge + blob transfer + capability admission. | PoC; README прямо застерігає про `iroh-blobs` maturity, Automerge 0.12 pin, macOS-only development, поки неперевірений multi-NAT, не-revocable write-token до expiry та незашифровані дані at rest. Не переносити ці trade-offs як готові VIDA рішення. |
| [arachne-core](https://github.com/arachne-systems/arachne-core) | Portable Rust library/core, відокремлений від ATAK/Android adapters; Iroh connectivity; MLS workspace membership/admin, invitations, topic-scoped routing/delivery, encrypted local records і recovery. README вказує на 8 crate releases на crates.io та pre-release статус. | Reference для Workspace API над membership/security/routing/delivery/storage та opaque application payloads; доповнює, але не дублює [Arachne ATAK plugin](https://github.com/arachne-systems/arachne-atak). | Pre-release; API і persisted formats можуть змінюватися. Авторський код MPL-2.0, vendored components мають окремі ліцензії; ліцензійні межі дивитися до будь-якого reuse. README не є криптографічним аудитом. |
| [chess-p2p](https://github.com/mr-nitesh-poudel/chess-p2p) | Мінімальна terminal chess app: Iroh 1.2 endpoint, direct/relay QUIC, custom ALPN, один bidirectional stream для newline-delimited moves; README описує test між двома endpoints в одному процесі. | Невеликий reference для custom protocol поверх Iroh 1.2 та локального integration test. | Однокомітний demo; не має durable delivery, account/membership або production security model. |

## Суміжний transport-abstraction lead — не Iroh implementation

| Проєкт | Поточний стан | Як індексувати |
|---|---|---|
| [Annoda](https://github.com/Metalymph/annoda) | MIT, experimental 0.0.x messaging contract та adapter SPI; README називає Iroh одним із можливих adapter-ів, але repository tree показує loopback adapter, не Iroh adapter. Peer/application API не є cross-protocol gateway: peers і далі потребують спільного wire stack. | Watchlist/adjacent transport abstraction. Не включати до числа реалізованих Iroh-проєктів; повернутися, якщо з'явиться Iroh adapter і приклади interoperability. |

## Відсіяні як дублікати або не нові реалізації

- `yixinin/nexapipe`, `nexa-android`, `nexa-desktop` — копії `open-nexa` репозиторіїв, уже внесених у попередній addendum.
- `TheKnarf/subduction` — копія наявного `inkandswitch/subduction`.
- `andriyDev/aeronet` — нова публікація з історією/Iroh transport від 10.08 за наданим звітом.
- `e4mc-minecraft-architectury` — копія старого e4mc.
- `mimapp` — commit history починається щонайменше 07.09, тому не входить до нового delta.
- Нові копії Dashbeam/Rayfish не рахуються окремими проєктами.

## Пріоритет подальшого code review

1. **echo** — звірити протоколи, token verification, файл reconciliation, conflict retention та платформні межі з власним code; найрелевантніший file/sync PoC.
2. **arachne-core** — оглянути межі workspace/security/delivery API, MLS integration, persistence/recovery та vendored dependency/license map.
3. **chess-p2p** — компактний приклад Iroh 1.2 custom ALPN і bidirectional stream; допоміжний protocol primer.
4. **Annoda** — спостерігати як transport-neutral API proposal, не як підтверджений Iroh adapter.

Ці записи є архітектурними references, не рішеннями стека VIDA і не прийняттям сторонніх схем membership, revocation, offline delivery чи conflict resolution.
