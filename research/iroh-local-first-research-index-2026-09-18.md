# Індекс досліджених проєктів Iroh та local first

Комплексний research index для пошуку архітектурних і продуктових референсів

**Зріз станом на 18 вересня 2026 року**

Каталог охоплює 54 окремих GitHub-репозиторіїв, знайдених у пов’язаному дослідженні та під час актуалізації. Кожна картка має категорію, архітектурний шар, stack, патерн, зони коду для огляду, активність, сильні сторони, обмеження і рівень доказовості.

Це нейтральний дослідницький індекс. Він фіксує відмінності між реалізаціями та не переносить жодне рішення автоматично на VIDA.

З них 51 — Iroh або Iroh-oriented references; суміжні lead-и та недоступні репозиторії позначені окремо. Кількість не є оцінкою якості чи зрілості.

## Як користуватися індексом

Почніть зі швидкого індексу тем: він веде до репозиторіїв і зон коду. Картки далі пояснюють роль, технології та межі висновків. Глибоко досліджені записи називають конкретні протоколи, типи чи модулі; короткі профілі прямо позначені як leads для самостійного огляду.

«Детальний code review» означає, що попередній аналіз охоплював реалізацію, tests або ADR. «README» — перевірку опису/дерева. «Repo pointer» — стек чи поведінку ще треба підтвердити у коді.

GitHub push dates — знімок metadata перевірок 18.09.2026. Push date не є release date і сама по собі не доводить зміну функціоналу. Коли дата не була записана, це вказано прямо.

## Швидкий індекс де шукати референс

| **Тема** | **Репозиторії** | **Що відкрити** |
| --- | --- | --- |
| CRDT merge | Synesis, Kukuri, iroh-db, iroh-beekem, Knot | Loro/Automerge, snapshots, merge tests, revisions. |
| Operation log і Merkle DAG | Irokle, Knot, Yaiba, DStore | Signed operations, causality, immutable revisions, tree history. |
| Blobs і великі файли | Acerola, Kukuri, iroh-db, OCID, DStore | Chunking, range reads, digest checks, manifests, cache, GC. |
| Membership | Irokle, Kukuri, iroh-db, iroh-beekem, Kith | Control operations, participants, grants, join/leave. |
| Roles і capabilities | iroh-db, Kukuri, iroh-beekem, Kith, Peerline Host | Domain scope, operation certificates, Space roles, mount admission. |
| Revocation та epoch rotation | iroh-db, Kukuri, iroh-beekem, Kith | Causal cut/rebase, key rotation, grant reissue, removed-device access. |
| Discovery DHT mDNS | Gossip Rendezvous, DStore, Kith, Kukuri, Acerola | Mainline DHT/pkarr, mDNS, bootstrap, discovery vs authorization. |
| Durable replicas | DStore, Guardian DB, iroh-db, Knot | Persistence, recovery, revision durability, cluster rebalancing. |
| Android та iOS | Acerola, Synesis, Knot, Android Native, PocketHound, DSH Tether, FlexTunnel | JNI/AAR, shared Rust core, lifecycle, background sync, tunnel. |
| Browser та WASM | Vellum, Arena Zero, Yaiba, ESP32 examples | WASM core/OPFS, sandbox, embedded UI, browser client. |
| Visualization Gantt timeline graph canvas map | Yaiba, Synesis, Knot | Critical-path scheduler, Timeline culling, JSON Canvas, graph/map. |
| Messaging і DM | Kukuri, Proscenium, HoloChat, M2M | Pairwise keys, outbox/ACK, ratchets, channels, agent messages. |
| Live media | Proscenium, Openstream, Bevy Iroh, WebRTC adapters | QUIC streams/datagrams, media backpressure, capture/encode. |
| Package/marketplace distribution | OCID, DStore | Signed releases, content-addressed blobs, seed/follow/pin, retention/GC. |
| Daemon і control plane | Rayfish, Datum, Peerline Host, Quix, Flexaccess Iroh | Lifecycle, admission, coordinator, relay diagnostics, OS networking. |
| Embedded ESP32 | CO2 Monitor, ESP32 Examples | Memory budgets, board targets, PSRAM, relay/discovery limits. |
| Agent і MCP | Vellum, Kith, PocketHound, AgentLink, Arena Zero, M2M | MCP boundary, tool permissions, remote control, WASM executor. |
| Cross-language bindings | Iroh FFI, Go Iroh, Android Native, Iroh HTTP | Ownership, cancellation, error mapping, runtime adapters. |

## Оновлення та останні знахідки

| **Період** | **Що зафіксовано** |
| --- | --- |
| 17–18 вересня | Synesis v0.0.9; Acerola Reader і Kukuri мали свіжу активність; Iroh v1.2.0 вийшов 11.09, upstream push — 17.09. |
| 18 вересня | OCID v0.5.1; Rayfish та інші app repos мали push у день зрізу. |
| 16 вересня | DStore: working-copy workflow і node-ID bootstrap за попереднім оглядом; Tincan CLI мав свіжий push. |
| 13–15 вересня | Knot: iOS/sync/storage updates; iroh-db і Resilum Core активні; Quix/Flexaccess оновлювали transport/runtime. |
| 8–12 вересня | Vellum, Android Native, Openstream, DSH Tether, PocketHound, HoloChat, M2M, Ingot Cluster і Zeiroh мали недавню активність. |
| Розширення огляду | iroh-db, iroh-beekem, Proscenium, Vellum, Kith і Rayfish додали database, group security, messaging, browser/WASM та daemon layers. |
| Знахідки цього індексу | Iroh HTTP, Tunnel RS та офіційні Iroh ESP32 examples додані як README-backed практичні references. |

## Картки проєктів

## Застосунки та робочі простори

### Synesis

**Repository:** [github.com/grimfeld/synesis](https://github.com/grimfeld/synesis)

**Архітектурний шар.** Локальний Markdown vault, index, sync engine та views.

**Stack.** React 19, TypeScript, Tauri 2, Rust, Loro, SQLite/FTS5, Iroh, CodeMirror, D3, Leaflet.

**Ключовий патерн.** Loro — mutable sync state; Markdown/YAML — читабельна матеріалізація; SQLite — rebuildable projection. JSON Canvas Boards зберігають nodes/edges окремо за ID.

**Що дивитися в коді.** Engine/Sync, Loro adapters, file reconciliation, SQLite index, board.ts geometry, Timeline filtering/culling, two-device/mobile tests.

**Остання знахідка.** Push 17.09.2026; v0.0.9 від 17.09. v0.0.7 додав Boards, JSON Canvas round-trip, geometry module і Timeline clustering/filters.

**Сильна сторона.** Цілісний приклад від local files до Graph/Map/Timeline.

**Межа або ризик.** Рання версія; попередній code review відзначив scalar membership version, немає rekey при removal, snapshot sync, whole-buffer files і потрібні online peers.

**Доказовість.** Детальний code review у попередньому дослідженні; push/release повторно перевірені.

### Acerola Reader

**Repository:** [github.com/vinicius-gpl/acerola-reader](https://github.com/vinicius-gpl/acerola-reader)

**Архітектурний шар.** Media app: desktop library обслуговує Android reader локально або через P2P.

**Stack.** Rust/Tauri 2, Svelte 5/TypeScript, Android, SQLite, axum/GraphQL/WebSocket, Iroh, iroh-blobs, mDNS; CBZ/CBR/PDF.

**Ключовий патерн.** mDNS + HTTP у LAN; Iroh QUIC/relay для віддаленого доступу; GraphQL subscriptions для progress/history; BLAKE3 для content identity.

**Що дивитися в коді.** Android↔desktop flows; mDNS; endpoint/relay config; subscriptions; blob streaming/cache; file import/export.

**Остання знахідка.** Push 18.09.2026; нову feature delta окремо не підтверджено.

**Сильна сторона.** Реальний mobile-to-desktop сценарій із великими файлами та LAN/fallback.

**Межа або ризик.** Відокремлювати Iroh data path від account/coordination service; перевіряти lifecycle desktop storage.

**Доказовість.** Попередній code review; README/URL перевірені.

### Kukuri

**Repository:** [github.com/kukuri-app/kukuri](https://github.com/kukuri-app/kukuri)

**Архітектурний шар.** Topic-first P2P social platform з кількома data planes.

**Stack.** Rust, Tauri 2, Svelte/TypeScript, Iroh Docs/Blobs/Gossip, DHT, SQLite, signed envelopes.

**Ключовий патерн.** Structured state → Docs; media → Blobs; transient notifications → Gossip. Private channels мають audience/policy, capabilities, epochs; DM має pairwise keys, outbox, encrypted attachments.

**Що дивитися в коді.** Workspace crates/ADR; межі docs/blob/gossip; channel rotation/grants; DM ACK/outbox; discovery.

**Остання знахідка.** Push 18.09.2026. Репозиторій переїхав зі старого KingYoSun/kukuri до kukuri-app/kukuri.

**Сильна сторона.** Розділяє типи state та delivery semantics в одному застосунку.

**Межа або ризик.** Багато компонентів; перевіряти актуальність ADR, security та maturity окремих платформ.

**Доказовість.** Детальний попередній огляд; новий URL перевірений.

### Irokle

**Repository:** [github.com/arunaengine/irokle](https://github.com/arunaengine/irokle)

**Архітектурний шар.** Rust library для operation log, membership і anti-entropy sync.

**Stack.** Rust, Ed25519, BLAKE3, postcard, optional Fjall та Iroh.

**Ключовий патерн.** Підписані operations утворюють Merkle DAG; membership/control — у тому самому log. SyncSummary/SyncPlan/SyncAck, actor clocks, deferred deps і bounded peer selection підтримують repair.

**Що дивитися в коді.** Operation/signature types; deterministic ControlKey merge; sync plan/ACK; missing-dependency repair; bounded replication.

**Остання знахідка.** Push 17.09.2026; попередній огляд зафіксував v0.1.4 та Iroh 1.2.0.

**Сильна сторона.** Компактний приклад causal history, anti-entropy та transport boundary.

**Межа або ризик.** Membership не є повною role model; не знайдено epoch rekey чи blob layer; discovery треба надбудовувати.

**Доказовість.** Детальний code review у попередньому дослідженні.

### Yaiba

**Repository:** [github.com/yukimemi/yaiba](https://github.com/yukimemi/yaiba)

**Архітектурний шар.** Standalone task/Gantt planner, один локальний binary з embedded UI і P2P sync.

**Stack.** Rust workspace, embedded web UI, SQLite, Iroh; cargo install yaiba.

**Ключовий патерн.** Task hierarchy; finish-to-start dependencies; scheduler обчислює slack і critical path; phone UI дивиться на ту саму replica.

**Що дивитися в коді.** Workspace crates; task/scheduler model; revisions/merge/sync; embedded UI; date editing; auth boundary у --host.

**Остання знахідка.** Push 17.09.2026; README позначає released і описує Gantt/dependency scheduling.

**Сильна сторона.** Keyboard-first Gantt, folding та single-binary distribution.

**Межа або ризик.** Hosted UI не має authentication за README; доступ відкривати лише в довіреній мережі.

**Доказовість.** README/tree перевірені 18.09.2026.

### Knot

**Repository:** [github.com/Fletcher-Alderton/knot](https://github.com/Fletcher-Alderton/knot)

**Архітектурний шар.** Local-first Markdown Kanban з revision/merge/sync modules та Tauri UI.

**Stack.** Rust, Markdown/YAML, Tauri, Iroh adapter.

**Ключовий патерн.** Core → store → immutable revision DAG/tombstones → deterministic three-way merge → transport-independent sync → Iroh.

**Що дивитися в коді.** Store atomic writes; revisions; merge; sync без Iroh; Iroh adapter; mobile/background acceptance tests.

**Остання знахідка.** Push 13.09.2026; iOS support, bounded pending revisions, per-board IO locking, storage hardening та watcher fixes.

**Сильна сторона.** Чітка межа domain state, merge protocol і transport.

**Межа або ризик.** Попередній звіт не довів повний acceptance mobile/background sync; перевіряти tests.

**Доказовість.** Попередній code/repo огляд.

### Proscenium

**Repository:** [github.com/iohzrd/proscenium](https://github.com/iohzrd/proscenium)

**Архітектурний шар.** Decentralized social network, messaging, calls і live audio.

**Stack.** Tauri 2, SvelteKit 5, Rust, SQLite, Iroh, Gossip, Blobs, QUIC.

**Ключовий патерн.** Master key делегує device transport і content-signing keys. Gossip для feed/control, Blobs для media, QUIC для DM/calls/stage; DM uses Noise IK + Double Ratchet + ChaCha20-Poly1305.

**Що дивитися в коді.** Identity delegation/rotation; DM outbox/ACK/ratchet state; stage topology; discovery; security tests.

**Остання знахідка.** Push 24.08.2026; README називає alpha.

**Сильна сторона.** Identity hierarchy, forward-secure DM і live audio paths в одному продукті.

**Межа або ризик.** Alpha: breaking changes/data loss можливі; це не anonymity network, peer бачить IP.

**Доказовість.** README та попередні архітектурні notes.

### Vellum

**Repository:** [github.com/andymitch/vellum](https://github.com/andymitch/vellum)

**Архітектурний шар.** Cross-platform Markdown vault зі спільним Rust core.

**Stack.** Rust, Tauri, WASM, Browser/PWA, OPFS, Iroh Docs, local MCP server.

**Ключовий патерн.** Один Rust core у native і WASM; browser worker працює з OPFS та P2P replica; MCP edits і UI проходять тим самим sync path.

**Що дивитися в коді.** vellum-vault API; wasm bindings; Worker↔OPFS; bootstrap; MCP edit propagation.

**Остання знахідка.** Push 08.09.2026; попередній зріз описує Android/macOS, Web/WASM та MCP.

**Сильна сторона.** Єдине доменне ядро для native і browser із local persistence.

**Межа або ризик.** Browser permissions/background limits і mobile release maturity перевіряти окремо.

**Доказовість.** Попередній repo/code report.

### Kith

**Repository:** [github.com/muhamadjawdatsalemalakoum/kith](https://github.com/muhamadjawdatsalemalakoum/kith)

**Архітектурний шар.** Mesh engine для Memory, Tabs, Files і MCP surfaces.

**Stack.** Rust, Automerge, Iroh, encrypted blobs, OS keychain, desktop/MCP.

**Ключовий патерн.** Один endpoint обслуговує незалежні encrypted Spaces; membership до EndpointId; signed hash-chain log, Admin/Writer/Reader roles, epoch rotation.

**Що дивитися в коді.** mesh-engine; SPAKE2 pairing; space keys; membership log; removal/rekey; blob path; MCP adapters.

**Остання знахідка.** Push 05.07.2026; попередній огляд позначає alpha.

**Сильна сторона.** Чітка межа engine/products та незалежні Spaces на одному mesh.

**Межа або ризик.** Звіти вказують: crypto audit не проводився, honest-peer assumptions, немає retroactive revocation, NAT tests переважно ручні.

**Доказовість.** Попередній детальний огляд.

### HoloChat

**Repository:** [github.com/dkl070418/HoloChat](https://github.com/dkl070418/HoloChat)

**Архітектурний шар.** P2P desktop messenger і file transfer.

**Stack.** Python desktop app, Iroh, local SQLite; точні залежності дивитися у manifests.

**Ключовий патерн.** Channels, peers і files в одному застосунку; локальне збереження розмов.

**Що дивитися в коді.** Iroh binding/process; message persistence/outbox; attachments; pairing; UI lifecycle.

**Остання знахідка.** Push 11.09.2026.

**Сильна сторона.** Компактний messaging приклад поза Rust/Tauri stack.

**Межа або ризик.** Product/security maturity окремо не оцінювали; перевірити threat model і залежності.

**Доказовість.** Попередній звіт і activity metadata.

### PocketHound

**Repository:** [github.com/arivsj/PocketHound](https://github.com/arivsj/PocketHound)

**Архітектурний шар.** Android client для віддаленого керування локальними AI agents.

**Stack.** Android/mobile UI та Iroh; точну мову перевірити у manifests.

**Ключовий патерн.** Phone↔desktop agent control; resume/cursor state та approvals у попередньому звіті.

**Що дивитися в коді.** Pairing; approval state machine; resume/cursor protocol; Android background lifecycle.

**Остання знахідка.** Push 12.09.2026.

**Сильна сторона.** Приклад mobile agent control через P2P link.

**Межа або ризик.** Authorization/reconnect behavior та maturity не пройшли повний code review.

**Доказовість.** Попередній звіт і activity metadata.

## Сховище історії та розповсюдження

### Iroh DB

**Repository:** [github.com/holon-technologies/iroh-db](https://github.com/holon-technologies/iroh-db)

**Архітектурний шар.** Rust-native local-first distributed database.

**Stack.** Rust, CRDTs, immutable signed commits, Iroh, encrypted chunked blobs.

**Ключовий патерн.** Typed records → CRDT → signed commits → snapshots/projections/indexes → Iroh sync. Domains: MultiWriter, SingleWriter, AppendOnly; sharing як typed projection.

**Що дивитися в коді.** Commit DAG/signatures; capabilities; causal cut/rebase; epoch rotation; encrypted chunk streaming; provider scheduler/prefetch.

**Остання знахідка.** Push 14.09.2026; створений 20.07.2026 за попереднім дослідженням.

**Сильна сторона.** Поєднує history, access domains та range-capable encrypted blob reads.

**Межа або ризик.** Молодий проєкт; перевірити fork/revocation/partial fetch/recovery tests.

**Доказовість.** Детальний code/repo огляд.

### Iroh Beekem

**Repository:** [github.com/unfoldml/iroh-beekem](https://github.com/unfoldml/iroh-beekem)

**Архітектурний шар.** Confidential collaborative workspace і group security layer.

**Stack.** Rust, Loro, iroh-docs, iroh-blobs, iroh-gossip, Beekem CGKA.

**Ключовий патерн.** Gossip control plane окремо від encrypted Docs/Blobs data plane; per-chunk causal secrets, blinded document keys, grant certificate з operation.

**Що дивитися в коді.** CGKA transitions; grant verification; key rotation/revocation; blinded IDs; concurrent membership tests.

**Остання знахідка.** Push 06.08.2026.

**Сильна сторона.** Приклад causal group keying та operation-level authorization evidence.

**Межа або ризик.** Crypto claims звіряти зі spec/tests; interoperability та production maturity тут не доведені.

**Доказовість.** Попередній detailed report і README.

### DStore

**Repository:** [github.com/amber-store/dstore](https://github.com/amber-store/dstore)

**Архітектурний шар.** Distributed content-addressed object store і working-copy workflow.

**Stack.** Rust, CAS, tree objects, replication, CASPaxos refs, mDNS, pkarr/Number0 DNS.

**Ключовий патерн.** Distributed CAS → replicated trees → clone/init/fetch/pull/push/status/diff; merge і GC. Node-ID bootstrap race-ить mDNS і DNS.

**Що дивитися в коді.** Placement/hash; cluster views; rebalance-before-publish; CASPaxos refs; GC; working-copy merge; discovery.

**Остання знахідка.** Push 16.09.2026; попередній огляд зафіксував Git-like working copy і node-ID bootstrap.

**Сильна сторона.** Покриває low-level storage та користувацький clone/pull workflow.

**Межа або ризик.** Cluster consistency, rebalance, GC safety і recovery потребують fault tests.

**Доказовість.** Попередній code/update report.

### OCID

**Repository:** [github.com/safonas/ocid](https://github.com/safonas/ocid)

**Архітектурний шар.** P2P OCI package/container registry та content distribution.

**Stack.** Rust, Iroh, signed releases, content-addressed blobs, gossip, Docker/Podman API.

**Ключовий патерн.** Publisher підписує release; receiver перевіряє signatures/digests; follow/seed/pin та latest/last:N/full керують replication/retention.

**Що дивитися в коді.** Release signature; registry API; digest verification; policy evaluator; GC; build provenance.

**Остання знахідка.** v0.5.1 released 18.09.2026, push 18.09; v0.5.0 посилив Android/Termux та supply-chain provenance.

**Сильна сторона.** Розділяє package metadata, publisher identity і blob distribution.

**Межа або ризик.** Перевіряти key recovery/rotation, multi-tenant trust, OCI compatibility, GC/pin edge cases.

**Доказовість.** Release і repo metadata перевірені.

### Guardian DB

**Repository:** [github.com/wmaslonek/guardian-db](https://github.com/wmaslonek/guardian-db)

**Архітектурний шар.** Local-first/distributed data або database exploration.

**Stack.** Stack не зафіксовано; перевірити manifests.

**Ключовий патерн.** Кандидат для database-layer порівняння; точний merge/log/transport protocol не верифікований.

**Що дивитися в коді.** README architecture; schema/persistence; operation/CRDT model; Iroh sync; restart/recovery tests.

**Остання знахідка.** Push 04.08.2026.

**Сильна сторона.** Кандидат для порівняння data layers.

**Межа або ризик.** Repo identity/activity не підтверджують production guarantees; висновків лише з назви не робити.

**Доказовість.** Repo pointer; окремий code review потрібен.

### Resilum Core

**Repository:** [github.com/Resilum/resilum-core](https://github.com/Resilum/resilum-core)

**Архітектурний шар.** Transport-resilient identity/network core.

**Stack.** Iroh поруч із Tor, I2P, Yggdrasil, BLE, TCP/UDP за попереднім описом.

**Ключовий патерн.** Одна identity та transport abstraction для різних network paths.

**Що дивитися в коді.** Identity↔transport mapping; capability/address discovery; failover; reachability tests per transport.

**Остання знахідка.** Push 15.09.2026.

**Сильна сторона.** Порівнює transport-independent identity та failover.

**Межа або ризик.** Adapters мають різні guarantees; перевіряти кожен окремо.

**Доказовість.** Попередні project notes; повний code path не виписаний.

### Iroh Topic Tracker

**Repository:** [github.com/rustonbsd/iroh-topic-tracker](https://github.com/rustonbsd/iroh-topic-tracker)

**Архітектурний шар.** Вузький topic tracking/membership helper.

**Stack.** Rust, Iroh; dependency surface не зафіксовано.

**Ключовий патерн.** Topic state/participant tracking; це не durable database.

**Що дивитися в коді.** Topic state; expiry/cleanup; peer identity; sync messages; persistence/tests.

**Остання знахідка.** Push 15.06.2026.

**Сильна сторона.** Може ізолювати discovery/tracking logic.

**Межа або ризик.** Активність старша; перевірити Iroh 1.x сумісність.

**Доказовість.** Repo pointer; limited audit.

### Iroh Gossip Rendezvous

**Repository:** [github.com/swaits/iroh-gossip-rendezvous](https://github.com/swaits/iroh-gossip-rendezvous)

**Архітектурний шар.** Decentralized discovery/rendezvous protocol.

**Stack.** Rust, iroh-gossip, Mainline DHT, encrypted rendezvous records.

**Ключовий патерн.** Gossip swarm bootstrap через DHT; попередній звіт описує protocol modeling/formal verification.

**Що дивитися в коді.** DHT bootstrap; record encryption; convergence; liveness/partition/property tests.

**Остання знахідка.** Push 13.05.2026.

**Сильна сторона.** Вузький алгоритмічний reference для discovery.

**Межа або ризик.** Старша активність; звіряти з поточними Iroh/gossip APIs.

**Доказовість.** Попередній report і metadata.

## Мережевий runtime та транспорт

### Iroh

**Repository:** [github.com/n0-computer/iroh](https://github.com/n0-computer/iroh)

**Архітектурний шар.** Upstream networking substrate: endpoint, QUIC connectivity, relay та discovery APIs.

**Stack.** Rust, QUIC, Iroh endpoint/relay and related crates.

**Ключовий патерн.** Один endpoint process може обслуговувати кілька ALPN; relay — connectivity fallback, не durable storage.

**Що дивитися в коді.** Endpoint creation; connection/discovery; relay auth/status; protocol compatibility; changelog/examples.

**Остання знахідка.** Push 17.09.2026; v1.2.0 released 11.09. RelayStatus отримав auth-denied reason і resolver fallback налаштування.

**Сильна сторона.** Першоджерело API, wire compatibility та transport behavior.

**Межа або ризик.** Бібліотека не задає application persistence, membership чи authorization.

**Доказовість.** Release/repo metadata перевірені 18.09.

### Rayfish

**Repository:** [github.com/rayfish/rayfish](https://github.com/rayfish/rayfish)

**Архітектурний шар.** Mesh VPN daemon і control plane.

**Stack.** Rust, Iroh, TUN, Magic DNS, firewall, invites, multi-device identity, exit nodes.

**Ключовий патерн.** Daemon керує endpoint/runtime; pairing/invites задають membership; exit nodes маршрутизують traffic.

**Що дивитися в коді.** Daemon lifecycle; TUN packet path; DNS; firewall; invite/device identity; exit-node tests.

**Остання знахідка.** Push 18.09.2026.

**Сильна сторона.** Повний node/runtime/control-plane reference, не storage.

**Межа або ризик.** VPN/TUN відрізняється від app-level protocol; перевірити privilege boundary та OS networking.

**Доказовість.** Попередні детальні notes й activity metadata.

### Quix

**Repository:** [github.com/quixvpn/quix](https://github.com/quixvpn/quix)

**Архітектурний шар.** Iroh mesh VPN з virtual IP/TUN.

**Stack.** Rust, Iroh QUIC datagrams, TUN, coordinator.

**Ключовий патерн.** IP packets через QUIC datagrams; coordinator роздає roster, data path лишається між peers.

**Що дивитися в коді.** Packet encapsulation/MTU; address derivation; roster integrity; loss handling; key authorization.

**Остання знахідка.** Push 15.09.2026.

**Сильна сторона.** Показує virtual network для legacy IP services.

**Межа або ризик.** Попередній огляд відзначив unsigned roster; EndpointId сам не є authorization.

**Доказовість.** Попередній report та activity metadata.

### Datum Connect Daemon

**Repository:** [github.com/datum-labs/datum-connect-daemon](https://github.com/datum-labs/datum-connect-daemon)

**Архітектурний шар.** Host daemon для P2P/cloud tunnels і local clients.

**Stack.** Iroh, daemon/local API, UI/CLI/agent clients; актуальні manifests уточнити.

**Ключовий патерн.** Long-running daemon відкриває local control API й керує tunnel connectivity.

**Що дивитися в коді.** Local API auth; config/state; tunnel lifecycle; client compatibility; privilege boundary.

**Остання знахідка.** Push 17.09.2026.

**Сильна сторона.** Runtime/control-plane приклад зі спільним daemon.

**Межа або ризик.** Розрізняти cloud/coordination path і direct peer data path.

**Доказовість.** Попередній report і metadata.

### DSH Tether

**Repository:** [github.com/zexadev/dsh-tether](https://github.com/zexadev/dsh-tether)

**Архітектурний шар.** Mobile access до developer agent на іншій машині через Rust sidecar.

**Stack.** Android/iOS clients, Rust sidecar, Iroh, DeepSeek Harness plugin.

**Ключовий патерн.** Direct hole-punch спершу; ciphertext-only relay fallback; Android arm64 має local mode.

**Що дивитися в коді.** Sidecar packaging; pairing/identity; relay fallback; local mode; Android/iOS lifecycle.

**Остання знахідка.** Push 09.09.2026; README каже, що iOS build beta і не тестований на real device.

**Сильна сторона.** Практичний mobile-to-dev-machine tunnel без обов’язкового central account.

**Межа або ризик.** iOS signing/device behavior потребують перевірки.

**Доказовість.** README повторно переглянуто.

### Peerline Host

**Repository:** [github.com/h-bar/peerline-host](https://github.com/h-bar/peerline-host)

**Архітектурний шар.** Один host process експонує кілька локальних і Iroh ALPN services.

**Stack.** Rust, Iroh, WebSocket/Unix socket interfaces.

**Ключовий патерн.** Mount-specific admission policy; remote identity може визначати, чи method реєструється взагалі.

**Що дивитися в коді.** Mount policies; service registration; caller identity; ALPN dispatch; method-not-found vs permission tests.

**Остання знахідка.** Push 05.09.2026; попередня зміна зробила per-mount policy обов’язковою.

**Сильна сторона.** Явна admission boundary до handler виклику.

**Межа або ризик.** Перевірити однаковість policy для WebSocket, Unix та Iroh inputs.

**Доказовість.** Попередні code-focused notes.

### Flexaccess Iroh

**Repository:** [github.com/flexaccessdev/flexaccess-iroh](https://github.com/flexaccessdev/flexaccess-iroh)

**Архітектурний шар.** Reusable Iroh connectivity і relay/failover diagnostics.

**Stack.** Rust, Iroh 1.2.

**Ключовий патерн.** Розділяє direct connectivity, relay auth status і failover diagnostics.

**Що дивитися в коді.** Endpoint builder; relay selection/fallback; status mapping; retry; integration API.

**Остання знахідка.** Push 15.09.2026; попередній report відзначив v0.0.9 та Iroh 1.2.

**Сильна сторона.** Діагностика розрізняє auth denial і network failure.

**Межа або ризик.** Transport helper не задає app authorization чи durable delivery.

**Доказовість.** Попередній report і metadata.

### WireHop

**Repository:** [github.com/Keikai-Inc/wirehop](https://github.com/Keikai-Inc/wirehop)

**Архітектурний шар.** Peer networking/tunnel utility за попереднім sweep.

**Stack.** Iroh-oriented; точна мова/service type перевірити у manifests.

**Ключовий патерн.** Можливий peer connection/service forwarding; exact protocol path не визначений.

**Що дивитися в коді.** README diagram; endpoint setup; service mapping; auth/config; stream/datagram handling.

**Остання знахідка.** Push 12.09.2026.

**Сильна сторона.** Кандидат на narrow tunnel integration.

**Межа або ризик.** Role і direct Iroh code path не пройшли детальний audit.

**Доказовість.** Попередній sweep; обмежена перевірка.

### Tincan CLI

**Repository:** [github.com/bilalyazicioglu/tincan-cli](https://github.com/bilalyazicioglu/tincan-cli)

**Архітектурний шар.** CLI tooling для peer connectivity або service access.

**Stack.** Iroh; CLI/runtime deps уточнити у Cargo.toml.

**Ключовий патерн.** CLI surface для endpoint setup/dial/service; точні commands звірити з README.

**Що дивитися в коді.** Command schema; key/config handling; connection lifecycle; error diagnostics; tests.

**Остання знахідка.** Push 16.09.2026.

**Сильна сторона.** Операторський CLI для peer tooling.

**Межа або ризик.** Security і maintenance не оцінювали без source review.

**Доказовість.** Попередній sweep і metadata.

### Quackhole

**Repository:** [github.com/smithclay/quackhole](https://github.com/smithclay/quackhole)

**Архітектурний шар.** Networking/DNS/proxy concept зі старого Iroh sweep.

**Stack.** Stack не записаний; перевірити Cargo.toml/README.

**Ключовий патерн.** Точна архітектура не встановлена; кандидат на DNS/proxy peer path.

**Що дивитися в коді.** DNS request path; endpoint addressing; policy; upstream errors; tests.

**Остання знахідка.** Push 04.09.2026.

**Сильна сторона.** Може порівняти DNS/proxy boundary.

**Межа або ризик.** Не покладатися на claims до підтвердження прямого Iroh use.

**Доказовість.** Попередній sweep; limited audit.

### Iroh SOCKS Proxy

**Repository:** [github.com/Creepybits/iroh-socks-proxy](https://github.com/Creepybits/iroh-socks-proxy)

**Архітектурний шар.** Adjacent SOCKS proxy prototype.

**Stack.** Iroh integration не підтверджена у попередньому sweep.

**Ключовий патерн.** Кандидат на SOCKS-over-peer; позначений як pre-integration prototype.

**Що дивитися в коді.** Branches/issues; socket-to-QUIC adapter; DNS/auth; relay path.

**Остання знахідка.** Push date не записана.

**Сильна сторона.** Показує межу між концептом та завершеною Iroh інтеграцією.

**Межа або ризик.** Не називати реалізованим Iroh transport без source evidence.

**Доказовість.** Prototype/adjacent.

### M2M

**Repository:** [github.com/unconfirmedlabs/m2m](https://github.com/unconfirmedlabs/m2m)

**Архітектурний шар.** Agent-to-agent protocol поверх Iroh.

**Stack.** Iroh; protocol/economics/payment stack перевірити у manifests.

**Ключовий патерн.** Agent identity, request/response та streaming payment concepts.

**Що дивитися в коді.** Identity; framing; payment accounting; replay protection; transport adapter.

**Остання знахідка.** Push 11.09.2026.

**Сильна сторона.** Поєднує agent protocol та economic flow як окрему область.

**Межа або ризик.** Payment/security claims не аудитили; перевірити protocol status.

**Доказовість.** Попередній report та metadata.

### Openstream

**Repository:** [github.com/gtkacz/openstream](https://github.com/gtkacz/openstream)

**Архітектурний шар.** P2P screen sharing/live collaboration.

**Stack.** Iroh transport; UI/runtime stack перевірити у manifests.

**Ключовий патерн.** Session transfer для screen/media поверх direct/relay connectivity.

**Що дивитися в коді.** Capture/encode; frame transport/backpressure; session setup; user permissions.

**Остання знахідка.** Push 12.09.2026.

**Сильна сторона.** Кандидат realtime media path на Iroh.

**Межа або ризик.** Codec, latency, platform support і security не перевірені end-to-end.

**Доказовість.** Repo summary і metadata.

### Bevy Iroh

**Repository:** [github.com/rvdende/bevy_iroh](https://github.com/rvdende/bevy_iroh)

**Архітектурний шар.** Bevy integration для peer networking/replication/media.

**Stack.** Rust, Bevy, Iroh.

**Ключовий патерн.** Iroh connectivity інтегрована у Bevy lifecycle; конкретний replicated state path перевірити.

**Що дивитися в коді.** Plugin lifecycle; entity replication; media/assets; reconnect; Bevy/Iroh version matrix.

**Остання знахідка.** Push 11.09.2026.

**Сильна сторона.** Вбудовування networking у ECS/game runtime.

**Межа або ризик.** Перевірити compile matrix і сумісність версій.

**Доказовість.** Попередній repo sweep; tests не перевірені тут.

### Iroh HTTP

**Repository:** [github.com/Momics/iroh-http](https://github.com/Momics/iroh-http)

**Архітектурний шар.** HTTP API поверх Iroh QUIC для native runtimes.

**Stack.** Node.js, Deno, Tauri adapters; Rust/Iroh core; Hyper/Tower за README.

**Ключовий патерн.** Fetch()-like client і serve()-like server; peer за ключем; спільний RequestTransport adapter.

**Що дивитися в коді.** Adapter boundary; request limits/timeouts; streaming bodies; runtime parity; auth/error paths.

**Остання знахідка.** README позначає pre-v1.0; push date не записана окремо.

**Сильна сторона.** Спільний API surface для native runtimes.

**Межа або ризик.** Не browser library і не public HTTP proxy; API може змінитися до 1.0.

**Доказовість.** README/architecture переглянуті.

### Tunnel RS

**Repository:** [github.com/flexaccessdev/tunnel-rs](https://github.com/flexaccessdev/tunnel-rs)

**Архітектурний шар.** Cross-platform encrypted TCP/UDP port forwarding через P2P.

**Stack.** Rust, Iroh QUIC, config/CLI, authorized keys.

**Ключовий патерн.** Server/client roles; path-based key config; невідомі config keys відхиляються; transport tuning.

**Що дивитися в коді.** Config validation; key files; allowed-source policy; forwarding; congestion/window params.

**Остання знахідка.** README оновлений у вересні 2026; точний push date не captured.

**Сильна сторона.** Практичний tunnel із явним config validation та allowlist.

**Межа або ризик.** Перевірити ключову ротацію та service isolation.

**Доказовість.** README перевірено.

## Платформні bindings та приклади

### Android Native

**Repository:** [github.com/1337farm/iroh-android-native](https://github.com/1337farm/iroh-android-native)

**Архітектурний шар.** Reusable Android bridge/runtime для Iroh.

**Stack.** Rust/JNI, Kotlin/Android, Android AAR, GitHub Packages.

**Ключовий патерн.** irohbridge exposes blobFetch/progress, stable engine secret, foreground service; USB reverse pairing.

**Що дивитися в коді.** JNI API; AAR publishing; foreground lifecycle; secret persistence; USB pairAccept/pairDial.

**Остання знахідка.** Push 11.09.2026; Iroh layer винесений у reusable AAR.

**Сильна сторона.** Приклад пакування Rust networking в Android app.

**Межа або ризик.** Перевірити ABI matrix, background limits і package versioning.

**Доказовість.** Попередній code/update report.

### Iroh FFI

**Repository:** [github.com/n0-computer/iroh-ffi](https://github.com/n0-computer/iroh-ffi)

**Архітектурний шар.** Foreign-function bindings для Iroh.

**Stack.** Rust FFI; target languages/targets уточнити у manifests.

**Ключовий патерн.** Експорт endpoint/network API у інший runtime.

**Що дивитися в коді.** C ABI/API; ownership/lifetimes; errors; threading; versioning; generated headers.

**Остання знахідка.** Push date не captured.

**Сильна сторона.** Upstream lead для non-Rust consumers.

**Межа або ризик.** FFI stability і supported targets треба підтвердити release/build matrix.

**Доказовість.** Supplemental discovery.

### Go Iroh

**Repository:** [github.com/tmc/go-iroh](https://github.com/tmc/go-iroh)

**Архітектурний шар.** Go integration/bindings around Iroh.

**Stack.** Go та Iroh bridge; конкретний bridge route перевірити.

**Ключовий патерн.** Potential endpoint/service API для Go workloads.

**Що дивитися в коді.** cgo/FFI boundary; context cancellation; callbacks/errors; release packaging.

**Остання знахідка.** Push date не captured.

**Сильна сторона.** Кандидат cross-language integration reference.

**Межа або ризик.** Не плутати з недоступним decentral1se/iroh-go чи forks.

**Доказовість.** Supplemental discovery.

### Iroh CO2 Monitor

**Repository:** [github.com/n0-computer/iroh-co2-monitor](https://github.com/n0-computer/iroh-co2-monitor)

**Архітектурний шар.** Embedded IoT app на ESP32-C61.

**Stack.** Rust, Iroh, ESP32-C61, sensor/telemetry.

**Ключовий патерн.** Resource-constrained endpoint надсилає environmental telemetry.

**Що дивитися в коді.** Target config/memory; sensor sampling; endpoint lifecycle; reconnect; buffers.

**Остання знахідка.** Push 10.09.2026.

**Сильна сторона.** Реальний embedded target.

**Межа або ризик.** Перевірити radio, RAM, relay та reconnect у довгому запуску.

**Доказовість.** Попередній sweep і metadata.

### Iroh ESP32 Examples

**Repository:** [github.com/n0-computer/iroh-esp32-examples](https://github.com/n0-computer/iroh-esp32-examples)

**Архітектурний шар.** Upstream examples для Iroh на ESP32 targets.

**Stack.** Rust, Iroh, Xtensa/RISC-V, ESP32/S3/C6/P4, optional WASM GUI.

**Ключовий патерн.** Target-specific configs; relay/pkarr discovery потребує RAM; constrained boards можуть бути LAN-only.

**Що дивитися в коді.** Per-target Cargo.toml; PSRAM flags; relay/pkarr; QUIC buffers; client examples.

**Остання знахідка.** README доступний 18.09.2026; push date не captured.

**Сильна сторона.** Фіксує embedded limits та target support.

**Межа або ризик.** Не кожна board підтримує relay/discovery; для частини потрібна PSRAM/reduced deps.

**Доказовість.** Upstream README.

### xk6 Iroh

**Repository:** [github.com/tmc/xk6-iroh](https://github.com/tmc/xk6-iroh)

**Архітектурний шар.** k6 extension для load-testing Iroh services.

**Stack.** Go/k6 extension та Iroh binding.

**Ключовий патерн.** Load scripts exercise endpoints/protocols; це testing tool, не app architecture.

**Що дивитися в коді.** JS↔Go boundary; endpoint per VU; metrics; independent peer setup; scenarios.

**Остання знахідка.** Push 20.08.2026.

**Сильна сторона.** Load/performance testing reference.

**Межа або ризик.** Перевірити версії k6/Iroh і незалежність тестових identities.

**Доказовість.** Попередній sweep і metadata.

### FlexTunnel iOS

**Repository:** [github.com/flexaccessdev/flextunnel-ios](https://github.com/flexaccessdev/flextunnel-ios)

**Архітектурний шар.** Native iOS client для private network access через Iroh tunnel.

**Stack.** SwiftUI/WebKit, Rust XCFramework, Iroh QUIC, local SOCKS5.

**Ключовий патерн.** Server підключається до private hosts; proxy може split-tunnel; без TUN/root.

**Що дивитися в коді.** WebKit proxyConfigurations; hostname vs CIDR; XCFramework; port forwards; iOS lifecycle.

**Остання знахідка.** README поточний у вересні 2026; push date не captured.

**Сильна сторона.** Детальний iOS reference для scoped proxy та private access.

**Межа або ризик.** Split tunnel змінюється для CIDR/full tunnel; врахувати iOS lifecycle.

**Доказовість.** README переглянуто.

### WebRTC Iroh Transport

**Repository:** [github.com/SuddenlyHazel/iroh-webrtc-transport](https://github.com/SuddenlyHazel/iroh-webrtc-transport)

**Архітектурний шар.** Experimental WebRTC/Iroh transport adapter.

**Stack.** WebRTC + Iroh; runtime/bridge уточнити.

**Ключовий патерн.** Potential adapter між WebRTC peer connections та Iroh.

**Що дивитися в коді.** Signaling; ICE/QUIC mapping; data channels; browser/native split; interop tests.

**Остання знахідка.** Push 02.05.2026.

**Сильна сторона.** Окремий experiment для mixed transport stacks.

**Межа або ризик.** Experimental; wire compatibility/production readiness не припускати.

**Доказовість.** Repo discovery/metadata.

### WebRTC Iroh Transport

**Repository:** [github.com/anchalshivank/iroh-webrtc-transport](https://github.com/anchalshivank/iroh-webrtc-transport)

**Архітектурний шар.** Окремий experimental WebRTC/Iroh project.

**Stack.** WebRTC та Iroh; stack verify у manifests.

**Ключовий патерн.** Схожа назва, інший owner; не вважати fork за замовчуванням.

**Що дивитися в коді.** Handshake/signaling; payload transport; platform deps; interoperability tests.

**Остання знахідка.** Push 10.05.2026.

**Сильна сторона.** Порівняння незалежних adapter designs.

**Межа або ризик.** Кодовий overlap/maturity потребують перевірки; активність старша.

**Доказовість.** Repo discovery/metadata.

## Агенти та обчислення

### Arena Zero

**Repository:** [github.com/0xff-ai/arena0](https://github.com/0xff-ai/arena0)

**Архітектурний шар.** P2P agent execution та verifiable WASM workload environment.

**Stack.** WASM, Iroh; host/runtime details verify у manifests.

**Ключовий патерн.** Agent workload execution поєднаний з peer coordination і verification.

**Що дивитися в коді.** WASM sandbox; workload signatures; executor identity; attestation; peer protocol.

**Остання знахідка.** Push 13.09.2026.

**Сильна сторона.** Portable execution і P2P coordination в одному reference.

**Межа або ризик.** Окремо оцінити sandbox, attestation та economics.

**Доказовість.** Попередній sweep; limited code review.

### Ingot Cluster

**Repository:** [github.com/niranjanaryan/ingot_cluster](https://github.com/niranjanaryan/ingot_cluster)

**Архітектурний шар.** Elixir distributed cluster over Iroh/Zenoh.

**Stack.** Elixir, Iroh, Zenoh.

**Ключовий патерн.** Cluster membership/messaging bridged across runtimes.

**Що дивитися в коді.** NIF/port boundary; naming; discovery; failure detection; ordering/backpressure.

**Остання знахідка.** Push 08.09.2026.

**Сильна сторона.** Cross-language reference поза Rust-only apps.

**Межа або ризик.** Розрізняти Iroh traffic і Zenoh traffic.

**Доказовість.** Попередній sweep.

### Zeiroh

**Repository:** [github.com/niranjanaryan/zeiroh](https://github.com/niranjanaryan/zeiroh)

**Архітектурний шар.** Phoenix FLAME distributed execution integration.

**Stack.** Elixir/Phoenix, FLAME, Iroh, Zenoh.

**Ключовий патерн.** Remote compute nodes через кілька network/messaging layers.

**Що дивитися в коді.** FLAME runner; node registration/discovery; dispatch/retry; Iroh vs Zenoh roles.

**Остання знахідка.** Push 08.09.2026.

**Сильна сторона.** Кандидат remote execution/elastic topology.

**Межа або ризик.** Experimental; звірити security, deployment, failure recovery.

**Доказовість.** Попередній sweep.

### AgentLink

**Repository:** [github.com/704986409/AgentLink](https://github.com/704986409/AgentLink)

**Архітектурний шар.** Historical C#/Avalonia agent desktop і MCP lead.

**Stack.** C#, Avalonia, Iroh, MCP за попереднім описом.

**Ключовий патерн.** Раніше описувався як agent connectivity surface; зараз недоступний.

**Що дивитися в коді.** Якщо відновиться: MCP boundary, endpoint auth, process lifecycle.

**Остання знахідка.** GitHub API повернув 404 18.09.2026; push unknown.

**Сильна сторона.** Зберігає історичний пошук desktop agent/MCP.

**Межа або ризик.** Недоступний implementation reference на дату зрізу.

**Доказовість.** Unavailable.

### ModelPipe

**Repository:** [github.com/mmogr/modelpipe](https://github.com/mmogr/modelpipe)

**Архітектурний шар.** Model/agent pipeline over distributed transport за попереднім discovery.

**Stack.** Iroh-oriented; точні мови дивитися у manifests.

**Ключовий патерн.** Можливе peer-connected model/pipeline orchestration.

**Що дивитися в коді.** Job graph; artifact transfer; discovery; retry/checkpoint; model-input security.

**Остання знахідка.** Push 14.09.2026.

**Сильна сторона.** Кандидат distributed agent/model workflow.

**Межа або ризик.** Не code-audited; durable semantics не виводити з назви.

**Доказовість.** Попередній sweep і metadata.

## Агенти та суміжні системи

### Company Mesh

**Repository:** [github.com/LifeAnalysis/company-mesh](https://github.com/LifeAnalysis/company-mesh)

**Архітектурний шар.** Organization/company collaboration mesh за попереднім sweep.

**Stack.** Мови/persistence у цьому індексі не зафіксовані.

**Ключовий патерн.** Можливий whole-product reference для shared workspace.

**Що дивитися в коді.** Org identity/membership; authorization; data model; offline behavior; service deps.

**Остання знахідка.** Push 13.09.2026.

**Сильна сторона.** Потенційний organization-level приклад.

**Межа або ризик.** Direct Iroh path і scope не пройшли code audit.

**Доказовість.** Попередній sweep; low confidence.

### Immich Shared Albums

**Repository:** [github.com/lukeet332/immich-shared-albums](https://github.com/lukeet332/immich-shared-albums)

**Архітектурний шар.** Adjacent sidecar для sharing між hosted Immich servers.

**Stack.** Container sidecar і signed protocol за README; Iroh use не встановлено.

**Ключовий патерн.** Оригінали лишаються на owner server; placeholder assets/live byte routes; per-person join/ejection.

**Що дивитися в коді.** Signed protocol; first-redemption key pinning; byte routes; proxy fail-open; membership/ejection.

**Остання знахідка.** Push date не captured.

**Сильна сторона.** Ілюструє sharing без копіювання оригіналів.

**Межа або ризик.** Не рахувати як Iroh project.

**Доказовість.** README перевірено supplemental search.

### Diaswarm

**Repository:** [github.com/alex-poor/diaswarm](https://github.com/alex-poor/diaswarm)

**Архітектурний шар.** Encrypted replicated data swarm за попереднім описом.

**Stack.** Точний stack не captured.

**Ключовий патерн.** Попередня розмова називала encrypted replicated data swarm.

**Що дивитися в коді.** Доступність; identity/encryption; replication; persistence; conflicts.

**Остання знахідка.** Push/status не captured.

**Сильна сторона.** Історичний lead для swarm replication.

**Межа або ризик.** Технічних claims недостатньо; repo/code recheck потрібен.

**Доказовість.** Історичний reference.

### Iroh-go legacy

**Repository:** [github.com/decentral1se/iroh-go](https://github.com/decentral1se/iroh-go)

**Архітектурний шар.** Історичний Go bindings lead; недоступний.

**Stack.** Stack/history потребують повторної перевірки.

**Ключовий патерн.** Не змішувати з tmc/go-iroh.

**Що дивитися в коді.** Якщо URL відновиться: binding lineage, supported targets, license.

**Остання знахідка.** GitHub API повернув 404 18.09.2026.

**Сильна сторона.** Зберігає provenance пошуку.

**Межа або ризик.** Не є доступним implementation reference.

**Доказовість.** Unavailable.

## Суміжні проєкти

### Lantunnel

**Repository:** [github.com/lantunnel/lantunnel](https://github.com/lantunnel/lantunnel)

**Архітектурний шар.** Виключено з Iroh catalog за попереднім sweep.

**Stack.** Не записано.

**Ключовий патерн.** Попередня перевірка класифікувала його як non-Iroh.

**Що дивитися в коді.** Повторно відкрити лише якщо потрібне порівняння LAN tunnel tools.

**Остання знахідка.** Не враховано у Iroh count.

**Сильна сторона.** Фіксує причину виключення.

**Межа або ризик.** Попередня класифікація потребує live recheck для іншого task.

**Доказовість.** Попередній sweep.

## Взаємозв’язки між проєктами

Порівнюйте репозиторії на рівні конкретної відповідальності. Iroh — connectivity substrate; Irokle, Loro, Automerge і Knot revisions описують history/merge; iroh-blobs, iroh-db, DStore й OCID працюють із payloads/storage/distribution; Kukuri, Proscenium та HoloChat — продукти поверх кількох primitives.

| **Зв’язок** | **Приклади** | **Межа порівняння** |
| --- | --- | --- |
| Connectivity | n0-computer/iroh; Gossip Rendezvous; DHT/mDNS у DStore та Kukuri | Endpoint/relay/discovery — транспортна основа; discovery не є authorization. |
| History and merge | Synesis/Loro; Irokle/signed DAG; Knot/revision DAG; Yaiba task history | Різні causal/revision моделі, не один узагальнений CRDT. |
| Blob/package data | Acerola; Kukuri; iroh-db; DStore; OCID | Media, domain blobs, object storage й signed release distribution — різні задачі. |
| Membership/security | Kukuri; iroh-db; iroh-beekem; Kith; Peerline Host | Розділяти membership, authorization, key rotation і transport identity. |
| Runtime/apps | Rayfish; Datum; Quix; Peerline Host; Acerola; Proscenium | Daemon/service host може відкривати багато protocols; mobile/UI clients споживають ці межі. |

## Уточнення та невирішені посилання

Kukuri переїхав із [KingYoSun/kukuri](https://github.com/KingYoSun/kukuri) до [kukuri-app/kukuri](https://github.com/kukuri-app/kukuri); це один проєкт. Старий URL потрібний лише для redirect/history.

704986409/AgentLink і decentral1se/iroh-go повертали 404 у перевірці 18.09.2026. AgentLink залишено як недоступний historical lead; iroh-go не вважається окремим доступним implementation. Не плутати з tmc/go-iroh.

Старі назви CommonsDB iroh service, p2prpc, ConduitMC, iruks, LinceBlob, n0-dns-resolver-android-demo та Sideband не мали однозначного GitHub URL у матеріалах. URL не вигадували. markqvist/Sideband використовує Reticulum, його не рахувати як Iroh.

Попередній sweep не рахував наведені нижче fork/noise, майже порожні або суміжні записи як незалежні Iroh implementations. Посилання залишені для provenance пошуку.

[webbeef/iroh-ble-transport](https://github.com/webbeef/iroh-ble-transport) — не підтверджено як окремий reference

[Mark1626/ringdrop](https://github.com/Mark1626/ringdrop) — шум у попередньому sweep

[snow2flying/iroh-live](https://github.com/snow2flying/iroh-live) — не підтверджено як незалежний project

[hanthor/neutrino-iroh](https://github.com/hanthor/neutrino-iroh) — fork

[domegle/iroh-rendezvous](https://github.com/domegle/iroh-rendezvous) — майже порожній репозиторій

[lantunnel/lantunnel](https://github.com/lantunnel/lantunnel) — попередньо класифікований non-Iroh

[markqvist/Sideband](https://github.com/markqvist/Sideband) — Reticulum, не Iroh

## Метод і свіжість

Індекс поєднує linked ChatGPT research thread, попередні code/repository reports та повторну перевірку GitHub repository/release pages станом на 18.09.2026. Детальні твердження позначені за evidence level; короткі repo pointers не означають, що код пройшов аудит.

Push dates взяті з GitHub metadata у перевірках 18.09.2026. Release dates наведені окремо, коли їх підтверджено. README описує заявлену поведінку; гарантії вимагали б актуальних tests, protocol code та issue history.

Картки нейтральні: вони описують implementation patterns, сильні сторони та межі, не формулюючи автоматичних рішень для VIDA.
