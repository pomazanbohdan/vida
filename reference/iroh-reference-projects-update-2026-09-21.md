---
title: "Актуалізація Iroh reference projects"
type: research-addendum
status: user-supplied-research
snapshot: 2026-09-19/2026-09-21
source: "Користувацький GitHub-огляд, вставлений 2026-09-21"
---

# Актуалізація reference-проєктів Iroh: 19–21 вересня 2026

Цей addendum доповнює історичний каталог [`iroh-local-first-research-index-2026-09-18.md`](iroh-local-first-research-index-2026-09-18.md). Він не змінює висновки або evidence level базового зрізу.

Користувацький огляд повідомляє про перевірку README/description і свіжих GitHub-комітів у період 19–21.09.2026, з відсіканням fork-ів, масових копій та репозиторіїв із давнішою історією. Цей файл фіксує надані результати; незалежна live-перевірка GitHub для початкового огляду не виконувалась. Follow-up README для `iroh-rings` прочитано з GitHub під час цього оновлення.

## Нові reference-проєкти

| Категорія | Проєкт | Підтверджений у дослідженні патерн | Статус для VIDA |
|---|---|---|---|
| Тунелі / remote access | [iroh-tunnel](https://github.com/chethan62/iroh-tunnel) | Node.js TCP/UDP tunnel поверх QUIC, hole punching і relay fallback; native QUIC datagrams | Reference для простого port-sharing; не готовий VIDA transport |
| Тунелі / remote access | [remote-device-sync](https://github.com/NDDev-OpenNetwork/remote-device-sync) | Rust workspace з agent, CLI, transport, desktop streaming і self-hosted Iroh relay | Сильний reference для багатьох application protocols над одним transport |
| Тунелі / remote access | [NexaPipe](https://github.com/open-nexa/nexapipe) | HTTP/HTTPS/WebSocket/TCP/UDP через один Iroh endpoint; без центрального control plane; library API | Сильний reference для embed-able network/service layer |
| Mobile client | [nexa-android](https://github.com/open-nexa/nexa-android) | Android `VpnService`/TUN-клієнт NexaPipe; repo позначений як новий 21.09 | Reference для mobile tunnel shell; перевірити lifecycle/security окремо |
| Desktop client | [nexa-desktop](https://github.com/open-nexa/nexa-desktop) | Tauri 2 + Vue 3 клієнт NexaPipe; repo позначений як новий 21.09 | Reference для desktop shell; не вибір VIDA toolkit |
| Remote access / agents | [Portty](https://github.com/corvuxmindware/portty) | P2P control shell, reconnect, file transfer і approvals для coding agents; Tauri mobile app; Apache-2.0 | Reference для agent-oriented remote control; ліцензію та threat model перевірити перед reuse |
| Audio / WASM | [iroh-mic](https://github.com/TheNuclearNexus/iroh-mic) | Статичний web app; Rust→WASM; browser напряму піднімає Iroh endpoint і передає PCM без backend/signaling | Reference для browser/WASM experiment; не production browser baseline |
| Messenger / collaboration | [Totem](https://github.com/aurnik/totem) | AIM-подібний iOS/macOS messenger: presence, ephemeral conversations, P2P text, Opus voice; сервер переважно coordinator | Reference для identity/coordinator і P2P data-plane split |
| Group networking | [Arachne for ATAK](https://github.com/arachne-systems/arachne-atak) | Приватні групи без TAK Server через invite link/QR, membership і P2P networking | Reference для group bootstrap/membership; не переносити security claims без audit |
| Agents / AI | [a2a-codex](https://github.com/wayneColt/a2a-codex) | Repo позначений як новий 19.09; Agent2Agent поверх Iroh для ChatGPT Desktop/Codex CLI; localhost proxy, A2A tools, peer addressing без central queue | Один із найрелевантніших agent-to-agent references для JEV |
| Protocol runtime | [Protocolo](https://github.com/gsemyong/protocolo) | Typed participant programs; simulated/adversarial scheduler і реальний `protocolo::iroh` runtime; faults/replay/shrinking | Сильний reference для actor/workflow/distributed runtime; перший commit 20.09 |
| File transfer | [ghostdrop](https://github.com/Firefares2005/ghostdrop) | CLI file/directory transfer через code/QR; direct QUIC + relay fallback; Windows/macOS/Linux/Termux | Reference для portable transfer UX; не durable delivery |
| Distributed storage | [Wyrd](https://github.com/control-aesir/wyrd) | Append-only content-addressed drive; Syncthing replication + Git/Merkle objects + history; snapshots/chunks/mirrors | Reference для storage/history model; maturity окремо не доведена |
| Distributed compute | [OxideSwarm](https://github.com/DuongNAD/OxideSwarm) | Rust workload orchestrator для jobs, compilation, GPU, Map/Reduce; Iroh QUIC без Kafka/RabbitMQ/Redis/Zookeeper | Reference для transport у distributed compute fabric; заявлені можливості потребують code verification |
| Architecture experiment | [atproto-iroh](https://github.com/jedelman/atproto-iroh) | Створений 21.09; AT Protocol data model + Iroh; `did:iroh`, capability-scoped namespaces, P2P replication без global relay/index | Design sketch лише; README прямо вказує на відсутність фактичної реалізації |

## Проєкти, які тепер треба додати до каталогу як окремі записи

Ці репозиторії не були окремими доступними записами у baseline-індексі; вони не є «новими» за змістом огляду, але мають увійти до актуального списку:

| Проєкт | Що додано до reference map |
|---|---|
| [iroh-ble-transport](https://github.com/mcginty/iroh-ble-transport) | Custom BLE transport: advertising discovery, GATT→L2CAP upgrade, GATT fallback; iOS/macOS/Android/Linux |
| [UniClipboard](https://github.com/UniClipboard/UniClipboard) | Cross-device clipboard/file-sync engine; relay configuration у settings |
| [chatmail/core](https://github.com/chatmail/core) | Iroh-based ephemeral P2P networking, multi-device setup і realtime webxdc; multi-relay onboarding |
| [Ringdrop](https://github.com/rikettsie/ringdrop) | `iroh-blobs` file transfer із ring-based Read/Write/Delete access control |
| [iroh-rings](https://github.com/rikettsie/iroh-rings) | MIT library, extracted from Ringdrop: `Registry`/`RingGate`/`Transfer`, typed Read/Write/Delete permissions, `mem`/`redb`/`fs` backends, ALPN `/iroh-rings/2` |
| [freeq](https://github.com/freeq-irc/freeq) | P2P IRC, AT Protocol identity, E2EE, Iroh QUIC, federated clustering і MCP/API/skills для agents |

## Оновлення вже наявних у baseline проєктів

| Проєкт | Зміна за оглядом 19–21.09 | Значення для VIDA reference map |
|---|---|---|
| [n0-computer/iroh](https://github.com/n0-computer/iroh) | [Router ALPN ordering](https://github.com/n0-computer/iroh/commit/3e3ba06101f52f5ccb22dd6dcbec26988d3eb6d3); [rustls update](https://github.com/n0-computer/iroh/commit/78f66a22cb9481b1f1bed7f98c3053fd1ae7a089) через `RUSTSEC-2026-0285` + Android CI fix; [metrics change reverted](https://github.com/n0-computer/iroh/commit/2af28341e22ab58ac0168560641bf4c670e5da16) через semver issue | Router preference і versioned ALPN треба врахувати в одному Endpoint з кількома application protocols |
| [iroh-ble-transport](https://github.com/mcginty/iroh-ble-transport) | Release `0.5.0` 20.09; 21.09 peripheral recovery після power-on і automatic BLE scan restart після adapter power cycle | Reference для truly local/offline/mobile mesh; окремо перевірити platform lifecycle |
| [Rayfish](https://github.com/rayfish/rayfish) | [Shared TLS session ID](https://github.com/rayfish/rayfish/commit/280a64790a4afbe5f5c41c602ee71dca784298e8), [DNS forwarding](https://github.com/rayfish/rayfish/commit/103b040630fd8388523cedb24169a182cd9cd621), [reconnect/network-handle fix](https://github.com/rayfish/rayfish/commit/a52a0f6f5defca12d7e9202c2a35e4994e271d1e) | Оновлює reference для private mesh/control-plane без central control server |
| [UniClipboard](https://github.com/UniClipboard/UniClipboard) | [Engine relay configuration](https://github.com/UniClipboard/UniClipboard/commit/37ec07a9710b9cbf74b671d947beb9deda25a8d2) додано в settings 20.09 | Reference для relay-aware clipboard/file-sync engine |
| [chatmail/core](https://github.com/chatmail/core) | [multi-relay `init_transports()`](https://github.com/chatmail/core/commit/72e711e59a596f78ae826cc36913069c5a54b4a3), [fastest-relay preference](https://github.com/chatmail/core/commit/0eddb98354415c51fef3c701f2412fb109f294d6), [background onboarding progress fix](https://github.com/chatmail/core/commit/3d3a6352e9efc8cc57a7ab9f54c4324e622aadc5) 21.09 | Reference для multi-relay onboarding; email-centric core не переноситься як VIDA dependency |
| [Ringdrop](https://github.com/rikettsie/ringdrop) | Releases `v0.20.0` і `v0.20.1` 19.09; `iroh-blobs` + ring-based ACL | Reference для групових Read/Write/Delete permissions над P2P resources |
| [iroh-rings](https://github.com/rikettsie/iroh-rings) | Follow-up repo; README описує reusable ring-based authorization gate, а authentication делегує Iroh | Сильніший reusable ACL boundary reference; не замінює VIDA identity, membership epochs або policy engine |
| [freeq](https://github.com/freeq-irc/freeq) | 21.09 виправлено `sendMarkdown`, щоб довгі вставки chunk-илися замість обрізання; Iroh QUIC, E2EE, P2P DMs і MCP/API/skills | Reference для P2P messaging + agent integration; не доказ durable VIDA mailbox |

## Рекомендований короткий список для VIDA

Не рейтинг і не рішення стека; це shortlist для наступного code review/prototype:

1. **remote-device-sync** — багато application protocols + self-hosted relay.
2. **NexaPipe** — загальний embed-able network/service layer.
3. **Protocolo** — typed distributed protocol/runtime модель.
4. **a2a-codex** — agent-to-agent orchestration без central broker.
5. **Totem** і **Arachne** — coordinator/identity layer окремо від P2P data-plane.
6. **iroh-ble-transport** — local/offline/mobile mesh transport.
7. **atproto-iroh** — capability-scoped data graph як design lead, не implementation evidence.
8. **iroh-rings** — reusable authorization gate/registry для ресурсів; auth залишається відповідальністю Iroh, а product policy — application layer.

Загальна тенденція огляду: Iroh переходить від вузького file-transfer до універсального application transport для tunnel/SSH/remote desktop/VPN, messenger, agents, distributed compute, application protocols і mobile networking.

## Що не зараховано до нових проєктів

- `iroh-esp32-examples`, `guardian-db` і частина Dashbeam forks не рахуються новими проєктами лише через свіжу дату створення GitHub repo: у дослідженні вказано старішу commit history.
- `atproto-iroh` лишається design sketch і не може бути доказом готової ATProto/Iroh реалізації.
- Baseline-картка `Guardian DB` не видаляється: застереження стосується лише класифікації як «нового» repo у цьому зрізі.

## Облік змін

- Baseline: 54 repo станом на 18.09.2026.
- Addendum: 15 нових reference-проєктів + 5 інших окремих репозиторіїв, доданих через помітні оновлення/follow-up links + 2 оновлені baseline-картки. `iroh-ble-transport` зазначений у двох секціях, але рахується один раз.
- Орієнтовний union каталогу після de-duplication: 74 distinct repository URLs станом на 21.09; число не є оцінкою якості чи зрілості.
