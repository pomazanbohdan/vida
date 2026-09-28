---
title: 'Technical research: Iroh ecosystem and project patterns for VIDA'
type: technical
topic: 'Iroh ecosystem and project patterns for VIDA'
decision: 'Which Iroh-based projects and protocol patterns VIDA should adopt, adapt, use as reference, or reject'
source: native-web-research
status: complete
preset: standard
validation: normal
created: 2026-09-18
updated: 2026-09-19
---

# Технічне дослідження: екосистема Iroh і патерни для VIDA

**Рішення, якому служить дослідження:** що саме VIDA бере з Iroh та наявних Iroh-проєктів, що прототипує, а що свідомо не переносить у свою архітектуру.

## Виконавче резюме

VIDA варто прийняти Iroh 1.2.0 як основну транспортну платформу. Нативний node host забезпечує автентифіковані наскрізно зашифровані QUIC-з'єднання, Address Lookup, NAT traversal, прямий маршрут із резервним relay-маршрутом і маршрутизацію прикладних протоколів через ALPN. Сам Iroh не є месенджером, базою даних, системою ролей або offline mailbox. Він з'єднує endpoint-и; семантику продукту повинна визначити VIDA.

Рекомендована межа: `VidaNodeHost` володіє `Router`, endpoint pool і життєвим циклом мережі. Постійний device endpoint обслуговує звичайний/federated profile; anonymous/session flows можуть отримувати окремі ephemeral endpoints для unlinkability. Messenger, Notes/Knowledge, Projects, Spaces, attachments і sync взаємодіють із node host через стабільні VIDA-інтерфейси та окремі версіоновані ALPN. Жоден доменний модуль не повинен напряму залежати від конкретного 0.x `iroh-*` протоколу.

`iroh-blobs`, `iroh-gossip` та `iroh-docs` корисні для прототипів, але не мають стабільності core 1.x: це окремі pre-1.0 компоненти з недавніми breaking-міграціями. Особливо важливо не прийняти старе уявлення, що «Iroh Docs — стабільне ядро синхронізації». Для VIDA вони мають бути замінними adapter-ами за власними межами протоколу, storage і compatibility tests.

Екосистема дає сильні патерни, але не готову платформу:

- **Rayfish** і **iroh-lan** показують control plane, admission і transient no-account spaces.
- **Kith**, **Vellum**, **Synesis** показують local-first notes, QR pairing, CRDT/replica UX і проблеми recovery/authority.
- **p2panda** та **GuardianDB** показують поділ discovery/gossip/reconciliation/blobs/store/auth.
- **Delta Chat** показує QR trust bootstrap, lazy P2P channel і WebXDC real-time, але його email/IMAP/SMTP/MIME core несумісний із напрямом VIDA.
- **Proscenium** показує поділ account/signing/transport keys, ratcheted DM та outbox/ACK, але не вирішує гарантовану доставку отримувачу offline.
- **Sendme**, **Dropwire** і **git-remote-iroh** показують capability tickets, resumable content transfer і чистий adapter boundary.

Ключове архітектурне рішення: пряме Iroh-з'єднання є швидким шляхом, а окремий сервіс наскрізно зашифрованої надійної доставки — шляхом для адресатів не в мережі. Relay-сервери Iroh не зберігають повідомлення. Обидва шляхи мають переносити той самий signed operation envelope з атомарним `local log + outbox`, idempotency та ACK state machine. Для адресата offline VIDA потребує власних federated store-and-forward вузлів, TTL/retention, quota, retry і pull-after-reconnect.

Загальна впевненість: **висока** щодо core Iroh, relay semantics і меж Delta Chat; **середня** щодо конкретних 0.x higher protocols; **низька–середня** щодо production readiness більшості сторонніх застосунків.

## 1. Метод і межі

Дослідження почалося з локального індексу 54 Iroh/local-first репозиторіїв і попередніх VIDA research notes. Назви з індексу не вважалися доказом: ключові кандидати перевірено за first-party документацією, README, releases, source та issue/PR. Офіційні Iroh examples [12] і Awesome Iroh [13] використовувалися як discovery catalog, а не як знак зрілості.

Обмеження дослідження:

- Не проводився незалежний cryptographic audit сторонніх проєктів.
- Не запускалися їхні застосунки та cross-version interoperability suites.
- Для Hubris доступний лише опис з офіційного каталогу; код/ліцензію не підтверджено.
- Публічні README можуть описувати намір точніше, ніж фактично завершену реалізацію; alpha/prototype claims позначені відповідно.

## 2. Поточний Iroh: що стабільне

Стабільний core — Iroh 1.2.0, released 2026-09-11 [1]. За current docs:

- `EndpointId` — Ed25519 public key; QUIC connection взаємно автентифікований та E2E encrypted. Один застосунок зазвичай використовує один спільний `Endpoint` [2][40].
- Прикладні протоколи визначаються ALPN; `iroh::protocol::Router` dispatch-ить accepted connection до handler-а. Кілька ALPN дозволяють паралельні версії протоколу [3].
- Поточний термін — Address Lookup, а не старе `discovery`. Endpoint ID резолвиться до relay/direct candidates через signed DNS/Pkarr; mDNS-like local lookup і mainline DHT — opt-in [4].
- NAT traversal може почати connection через relay, обміняти candidates, виконати hole punching і мігрувати на direct path. Якщо direct path неможливий, relay лише forward-ить E2E-encrypted traffic [5].
- Release policy гарантує core wire compatibility в межах major, але не стабілізує явно experimental API [6].

### Платформні обмеження

- Browser WASM є relay-only: sandbox не дає Iroh довільний UDP; офіційного npm bundle немає. Нативний Node binding зберігає direct path [7].
- Iroh FFI охоплює Python, Swift/iOS/macOS, Kotlin/JVM/Android і Node.js, але стабільна поверхня стосується core endpoint/connection/path/ticket/relay. Blobs/docs/gossip явно виключені; custom lookup/transports не перелічені в стабілізованій binding surface і потребують prototype verification [8].
- Mobile background execution, iOS local-network permissions і exact FFI↔core 1.2 artifact matrix залишаються окремими prototype gates.

## 3. Протоколи вищого рівня: корисні, але не частина ядра

Станом на зріз:

- `iroh-blobs` 0.103.0 і `iroh-gossip` 0.101.0 released 2026-06-15; переходи на Iroh 1.0 позначені breaking [9][10].
- `iroh-docs` 0.100.0 released 2026-05-27 та був побудований на Iroh 1.0 release candidate. Це meta-protocol над blobs + gossip, а не частина стабільного transport core [11][39].
- README `iroh-blobs` рекомендує для production старішу гілку 0.35, хоча доступна версія 0.103 [38]. Поки цю розбіжність не пояснено й не виконано cross-version tests, компонент не можна приймати безумовно.

Висновок: VIDA може прототипувати BLAKE3 content addressing, verified ranges, gossip hot path і docs/reconciliation, але має:

1. Закріпити точні версії.
2. Загорнути кожен компонент у VIDA-owned adapter.
3. Зберігати власний protocol envelope/version.
4. Тестувати N−1/N/N+1 wire combinations і migration/recovery.
5. Мати replacement plan без зміни доменної моделі.

## 4. Каталог проєктів і цінність для VIDA

| Проєкт | Категорія / зрілість | Що підтверджено | Рішення для VIDA |
|---|---|---|---|
| Rayfish [14] | Mesh VPN; experimental/pre-1.0; MPL-2.0 | TUN, identity-derived network, DHT control plane, direct/relay, admission, firewall, exit nodes, multi-device | **Reference** daemon/control plane, network diagnostics, invite/admission; не data sync dependency |
| iroh-lan [15] | Ephemeral L3 LAN; prototype; MIT | No-account transient overlay, name/password, distributed lookup, TUN | **Adapt concept** для temporary anonymous spaces; не durable identity/state |
| Kith [16] | Notes/tabs/files; alpha; MIT OR Apache-2.0 | Iroh + Automerge, spaces, SPAKE2 pairing, signed membership chain, roles, epoch rekey | **Prototype** membership/revocation and pairing; require security review |
| Vellum [17] | Markdown notes; early; MIT | Tauri/Svelte, native+WASM shared Rust core, QR, full replica, Iroh Docs, MCP | **Reference** notes UX, browser/native core, MCP sync path; reject full-write-ticket authority model |
| Synesis [18] | Obsidian-compatible notes; early; MIT | Plain Markdown, QR pairing, direct Iroh sync while both peers online, alternative folder sync | **Reference** human-readable materialization and sync-provider UX; no durable delivery |
| Teamtype [19] | Collaborative local files; maintained; AGPL-3.0 | Automerge + Iroh, short join codes, local disk truth, JSON-RPC editor plugins | **Reference** editor/core boundary; licensing gate before code reuse |
| p2panda [20] | Local-first toolkit; active pre-1.0; MIT OR Apache-2.0 | Separate net/lookup/sync/blob/store/space/encryption/auth layers | **Prototype candidate** for architecture and protocols; do not adopt wholesale before API stabilization |
| GuardianDB [21] | Local-first DB 0.20.x; active; MIT OR Apache-2.0 | Full replicas, blobs/docs/gossip multiplex, LWW stores, causal event DAG, reconciliation, ACL | **Prototype candidate** for convergence/event-log tests; production maturity independently unvalidated |
| Proscenium [22] | Social/messaging; alpha; MIT | Account/signing/transport key split, gossip/blobs/direct ALPN, Noise IK + Double Ratchet, local outbox/ACK | **Reference** identity hierarchy and DM state machine; does not provide recipient-offline mailbox |
| Delta Chat / Chatmail core [23][34] | Mature messenger, email-centric; MPL-2.0 | Iroh peer channel/WebXDC, QR backup, SecureJoin; current core still pins Iroh 0.35 | **Adapt ideas only**; reject embedding email core and direct dependency |
| Sendme [27] | First-party transfer example; MIT OR Apache-2.0 | Capability ticket, resumable/verified BLAKE3 blobs, direct/relay | **Adopt pattern**, prototype attachment protocol |
| Dropwire [28] | Desktop file transfer; early; MIT OR Apache-2.0 | Dedicated Iroh engine boundary, pair confirmation, manifest-before-accept, resumable transfer | **Reference** adapter boundary and transfer UX |
| Knot Git prototype [29] | Decentralized Git lead; upstream verification blocked | Local research described CAS packfiles/manifests plus signed canonical pointer in ATProto | **Unverified lead**; do not use as architecture evidence until an accessible immutable source is verified |
| git-remote-iroh [30] | Small protocol bridge; Apache-2.0 | Git smart protocol over Iroh; stable or ephemeral endpoint; capability URL | **Reference** tunnelling mature protocols behind ALPN |
| iroh-live [31] | MoQ media preview; experimental | Live media over Iroh with browser relay and explicit limitations | **Watch/reference** for calls/live media, not v1 dependency |
| Obsiroh [32] | Tiny Obsidian prototype; MIT | Minimal stated Iroh sync | **Reject** as architecture evidence; too thin |

Vellum і Synesis відсутні в офіційних каталогах Iroh, але їхні актуальні репозиторії прямо підтверджують використання Iroh [17][18]. Тому це сторонні референсні проєкти, а не компоненти, схвалені екосистемою. `iroh-socks-proxy` відхилено як хибний збіг: README згадує інтеграцію Iroh лише як майбутній етап.

## 5. Delta Chat: що переносимо, а що ні

Поточний Chatmail core pins `iroh`/`iroh-gossip` 0.35 [23]; Iroh 1.0 upgrade PR лишається draft і залежить від relay deployment та backup compatibility [24]. Це сильний сигнал, що навіть production messenger не ізольований від Iroh protocol/version transitions.

У `peer_channels.rs` Delta Chat lazy-створює ephemeral endpoint і gossip topic для WebXDC real-time [25]. Random topic та endpoint/relay metadata передаються через encrypted email message. WebXDC real-time API не прив'язаний концептуально до email, але конкретний bootstrap — прив'язаний [26].

Переносимо як патерни:

- Ephemeral transport identity окремо від user/persona identity.
- QR-based trust bootstrap з invite/auth tokens, token validation і freshness window; replay handling має окремо визначити VIDA [35].
- Lazy P2P channel creation після явного контексту/consent.
- Random topic scoped до app/space/document session.
- Gossip broadcast для transient low-latency events.
- Event/JSON-RPC boundary між core і clients [37].
- Owner-only broadcast semantics як один із policy templates [36].

Не переносимо:

- RFC5322 email identity, SMTP, IMAP, MIME, mail headers.
- Autocrypt/rPGP mail flow як основу VIDA identity.
- Email як offline queue або membership channel.
- Delta Chat core як embedded messaging engine: email coupling, MPL-2.0 file-level copyleft і застарілий Iroh dependency створюють зайві обмеження.

Немає достатніх доказів, що Delta Chat підтримує загальну ротацію групових ключів на зразок MLS або формально доведену збіжність групового стану. Тому не вказуйте це у вимогах VIDA як готову запозичену властивість.

## 6. Цільовий патерн VIDA

```text
Persona / Space / Roles / Policies / Domain state
                    │
        VIDA protocol interfaces
    ┌───────────────┼────────────────┐
    │               │                │
event log + sync  durable queue   blob transfer
pull/reconcile     outbox/ACK     CAS/manifests
    │               │                │
    └───────────────┼────────────────┘
            VidaNodeHost
 endpoint pool + Router + Address Lookup
        direct QUIC ↔ relay forwarding
```

### `VidaNodeHost`

Повинен:

- Керувати endpoint pool: persisted device endpoint для звичайного/federated profile, ephemeral/session endpoints для anonymous flows та окремі service endpoints за потреби.
- Визначити rotation, recovery, unlinkability і compromise blast radius для кожного endpoint profile.
- Не використовувати `EndpointId` як global user/persona identity.
- Публікувати versioned protocol handlers, наприклад `vida/msg/1`, `vida/sync/1`, `vida/blob/1`, `vida/presence/1`.
- Динамічно визначати relay/direct candidates; не зберігати IP/relay candidate як довгоживучу identity.
- Давати clients стабільний command/event API; FFI details не повинні протікати в domain code.
- Підтримувати relay policy, network change, diagnostics, quotas і graceful shutdown.

### Розділення типів даних

| Тип | Primary path | Recovery/durability | Приклади VIDA |
|---|---|---|---|
| Transient event | Gossip або direct datagram/stream | Не гарантується; наступний state sync виправляє | presence, typing, cursor, live reaction |
| Durable operation | Signed log over direct Iroh | Local log + durable encrypted queue + pull/reconcile | message, note edit, task mutation, membership change |
| Immutable large content | CAS/blob transfer | Manifest, chunk/range verify, resumable fetch, replica/pin policy | attachments, media, document assets |
| Canonical pointer/control | Signed small operation | Causal log, epochs, quorum/policy as defined by Space | current doc head, membership, role grant, key epoch |

## 7. Відкладена доставка — окремий системний контур

Iroh relays are stateless: вони не зберігають payload і лише допомагають hole punching або forward-ять traffic connected clients [33]. Отже, direct connection і relay connection працюють тільки коли endpoint доступний.

Для вимоги «синхронізовані дані доступні локально; нові дані доходять після reconnect» VIDA потребує:

- Local-first replica на кожному пристрої.
- Один signed operation envelope для direct і mailbox paths.
- Атомарна transaction `local log + outbox` до підтвердження durable acceptance.
- E2E-encrypted envelope, адресований device/space epoch keys; mailbox зберігає transport cache, а не authoritative state.
- Federated durable delivery node або обрані replicas, які зберігають ciphertext, не plaintext.
- Stable operation IDs, idempotent apply, deduplication.
- Формальний ACK state machine: `accepted → stored → applied → read`; multi-device fan-out не змінює operation ID.
- Causal ordering policy для операцій, що приходять direct і mailbox paths у різному порядку.
- TTL, retention, quota, backpressure, retry schedule і dead-letter diagnostics.
- Pull/reconcile після reconnect, щоб втрата transient event не ламала state.
- Explicit deletion/expiry semantics і audit events.

Сервіс надійної доставки може використовувати Iroh як транспорт між client↔node та node↔node. Проте цей сервіс не є relay-сервером Iroh і не повинен подаватися як такий. Окремий ADR має вибрати trust/availability model: single mailbox, replicated mailboxes, keeper peers або authoritative service; рішення фіксує replication factor, metadata exposure, deletion jurisdiction і SLO.

## 8. Режими ідентичності та довіра

Попередньо визначені VIDA modes сумісні з Iroh, якщо transport identity відділена від Persona:

| Mode | Identity root | Iroh role | Service dependency |
|---|---|---|---|
| Anonymous / autonomous | Local device/persona root; optional ephemeral session keys | Direct/relay connectivity; QR/capability bootstrap | None until user opts into sharing or durable delivery |
| Federated | Persona + devices anchored to chosen federated node | Client↔node and P2P fast path | Directory, encrypted mailbox, multi-device sync, recovery policy |
| Public | Public persona/profile and signed discoverability records | Connection path only | Public directory/profile services; privacy controls remain separate |

Membership, roles і permissions належать Space policy. Endpoint key лише доводить володіння transport key. Device admission має прив'язувати device key до Persona через signed certificate/grant; removal має створювати новий membership/key epoch, а не лише видаляти endpoint зі списку. Rekey гарантує припинення майбутнього доступу, але не може відкликати plaintext, який пристрій уже отримав. Space security design повинен визначити root of trust, admin quorum/threshold, concurrent grant/revoke ordering, invite audience/expiry, recovery і partition behavior.

## 9. Матриця рішення

### Adopt

- Iroh core 1.2 як primary transport.
- Один node host/router на application process із policy-managed endpoint pool.
- Versioned VIDA ALPN protocols.
- Transport identity separate from Persona/device authorization.
- Direct-first + relay fallback.
- Local replica, durable operation log, outbox/ACK, pull/reconcile.
- Content-addressed verified blob transfer як attachment foundation.

### Adapt and prototype

- p2panda layering: net/lookup/sync/blob/store/space/auth.
- Kith: signed membership log, roles, epoch rotation, QR pairing.
- GuardianDB: reconciliation and causal event-log approaches.
- Proscenium: account/signing/transport/session key hierarchy and DM state machine.
- Vellum/Synesis: human-readable notes, native/browser core, QR/device UX.
- Delta Chat: SecureJoin-style bootstrap, lazy peer channels, scoped gossip topic.

### Reference only

- Rayfish daemon, admission, relay diagnostics, OS lifecycle.
- iroh-lan transient no-account spaces.
- Teamtype local-disk truth and editor adapter.
- Sendme/Dropwire transfer protocol and UX.
- Knot/git-remote-iroh CAS pointer and byte-stream bridge.
- iroh-live media evolution.

### Reject for the foundation

- Email/IMAP/SMTP/MIME as VIDA messaging architecture.
- Treating Iroh relay as store-and-forward.
- Treating EndpointId as user identity.
- Direct domain dependency on pre-1.0 Iroh higher protocols.
- A single shared write ticket as complete roles/permission model.
- Copying alpha cryptographic designs without audit/threat model.
- Embedding copyleft code without component-specific licensing analysis: MPL file-level і AGPL network copyleft мають різні obligations.
- Trusting project names, catalog inclusion, stars, or recent pushes as production evidence.

## 10. Обов'язкові прототипи перед architecture lock

1. **Core node prototype:** Iroh 1.2, Router, persistent + ephemeral endpoint profiles, two versioned ALPN handlers, rotation/recovery/unlinkability tests.
2. **Compatibility matrix:** окремі axes для Iroh core, кожного 0.x adapter-а, VIDA ALPN major, envelope/schema, persisted state та FFI artifact; mixed-version rolling upgrade, rollback і backup restore.
3. **Offline delivery prototype:** два clients + щонайменше два delivery nodes; recipient offline, atomic log/outbox, encrypted enqueue, direct/mailbox race, reconnect, dedupe, causal order, ACK states, TTL і failover.
4. **Space security prototype:** QR device admission, root of trust, admin policy, signed membership log, concurrent grant/revoke, epoch rekey, malicious removed device, partition/recovery; acceptance стосується лише future access.
5. **Local-first sync prototype:** concurrent notes/tasks edits, causal repair, snapshot/bootstrap, tombstones, attachment reference and garbage collection.
6. **Blob security prototype:** hash-over-ciphertext/plaintext decision, per-object keys, encrypted manifests, scoped expiring tickets, pin/GC ownership, leaked-ticket and delete/rekey tests.
7. **Platform prototypes:** окремо browser topology/cost; Android key storage, suspend/process death, push wakeup and battery; iOS permissions, background execution and recovery.
8. **Address Lookup prototype:** provider interface, signed-record TTL/rotation, cache and relay allowlist; DNS/Pkarr outage, stale/rollback record, relay migration and provider compromise.
9. **Threat-model review:** malicious member, replayed invite, stolen endpoint secret, compromised federated node, metadata leakage, spam/DoS and blob abuse.

Exit criteria: deterministic tests, documented wire envelope, migration strategy, measured latency/storage/relay egress, recovery drill, and explicit license/security review.

## 11. Ризики, суперечності та невідоме

| Питання | Стан | Наслідок |
|---|---|---|
| Core 1.2 vs higher 0.x protocols | Core stable; higher layers separate/breaking | Pin adapters and test interoperability |
| `iroh-blobs` current vs recommended production line | README/release state contradictory | No production selection until resolved empirically/upstream |
| Delta Chat migration | Draft Iroh 1.0 migration; current 0.35 | Use patterns only, not dependency |
| Durable offline delivery | Not supplied by stateless Iroh relay | VIDA federated mailbox/replica service required |
| Browser | Relay-only | Capacity, cost, privacy and UX plan required |
| Native FFI | Stable core only | Higher protocols may require Rust core integration |
| Mobile background | Not established here | Prototype before sync SLOs are promised |
| Group security | Several alpha implementations; no common proven standard found | Separate protocol/security design and audit |
| Third-party maturity | Many alpha/prototype/pre-1.0 | Reference, not foundation |
| Endpoint privacy | One persistent endpoint correlates modes/spaces | Policy-managed persistent and ephemeral endpoint profiles |
| Direct + mailbox race | Duplicate/reordered or crash-lost operations | One signed envelope, atomic log/outbox, idempotency and ACK state machine |
| Delivery topology | Single vs replicated node changes trust and SLO | Separate ADR and multi-node failover test |
| Blob confidentiality/revocation | Plaintext hashes or bearer tickets may leak/overlive policy | Encrypted manifests, scoped tickets, key and GC contract |
| Address Lookup dependency | DNS/Pkarr outage or stale record can block/re-route connectivity | Provider abstraction, cache/TTL policy and compromise tests |
| License reuse | MPL and AGPL obligations differ by integration boundary | SPDX BOM, reuse-mode classification, legal owner and clean-room provenance |

## 12. Реєстр тверджень

| # | Load-bearing claim | Evidence | Confidence |
|---|---|---|---|
| C1 | Current stable Iroh core is 1.2.0 | [1] | High |
| C2 | EndpointId is transport public-key identity; QUIC is mutually authenticated/E2E encrypted | [2][40] | High |
| C3 | Router/ALPN is the intended application-protocol boundary | [3] | High |
| C4 | Direct/relay paths are connectivity choices, not application durability | [5][33] | High |
| C5 | Iroh relays do not store messages | [33] | High |
| C6 | Browser participation is relay-only | [7] | High |
| C7 | Blobs/gossip/docs are separate pre-1.0 components | [9][10][11][38][39] | High |
| C8 | Delta Chat current Iroh integration is email-bootstrapped and still on 0.35 | [23][25] | High |
| C9 | Existing Iroh apps independently implement identity/membership/sync | Verified project repositories in section 4 | Medium-high |
| C10 | No reviewed project supplies VIDA's complete product/security/durability model | Cross-project comparison and explicit gaps | Medium-high |

## 13. Підсумкове рішення

Зафіксувати Iroh 1.2 як transport foundation, але **не** зафіксовувати Iroh Docs, Delta Chat core, p2panda або GuardianDB як готове data/application core. Створити VIDA-owned protocol and state architecture, використовуючи перевірені патерни проєктів як inputs до прототипів.

Наступний етап ухвалення рішення: затвердити логічні контракти `VidaNodeHost`, `DurableDelivery`, `SpaceMembership`, `SyncLog` і `BlobStore`, а потім оформити ADR та NFR/SLO для відкладеної доставки, security epochs, compatibility і platform support.

## Джерела

| Ref | Claim/finding supported | Publisher | Published | Accessed | Confidence |
|---|---|---|---|---|---|
| [1] | Iroh 1.2.0 current stable release | [n0-computer](https://github.com/n0-computer/iroh/releases/tag/v1.2.0) | 2026-09-11 | 2026-09-18 | high |
| [2] | Endpoint identity and authenticated encrypted QUIC | [Iroh docs](https://docs.iroh.computer/concepts/endpoints) | live | 2026-09-18 | high |
| [3] | ALPN protocols and Router dispatch | [Iroh docs](https://docs.iroh.computer/concepts/protocols) | live | 2026-09-18 | high |
| [4] | Address Lookup model and providers | [Iroh docs](https://docs.iroh.computer/concepts/address-lookup) | live | 2026-09-18 | high |
| [5] | Relay/direct NAT traversal and path migration | [Iroh docs](https://docs.iroh.computer/concepts/nat-traversal) | live | 2026-09-18 | high |
| [6] | Core release and compatibility policy | [Iroh docs](https://docs.iroh.computer/about/release-policy) | live | 2026-09-18 | high |
| [7] | Browser WASM relay-only constraints | [Iroh docs](https://docs.iroh.computer/languages/wasm-browser) | live | 2026-09-18 | high |
| [8] | First-party FFI scope and languages | [n0-computer](https://github.com/n0-computer/iroh-ffi) | live repo | 2026-09-18 | high |
| [9] | iroh-blobs version and maturity | [n0-computer](https://github.com/n0-computer/iroh-blobs/releases) | 2026-06-15 | 2026-09-18 | high |
| [10] | iroh-gossip version and breaking transition | [n0-computer](https://github.com/n0-computer/iroh-gossip/releases) | 2026-06-15 | 2026-09-18 | high |
| [11] | iroh-docs release and meta-protocol status | [n0-computer](https://github.com/n0-computer/iroh-docs/releases) | 2026-05-27 | 2026-09-18 | high |
| [12] | Official Iroh application examples | [Iroh docs](https://docs.iroh.computer/examples) | live | 2026-09-18 | high |
| [13] | Curated ecosystem discovery list | [n0-computer](https://github.com/n0-computer/awesome-iroh) | live repo | 2026-09-18 | medium |
| [14] | Rayfish architecture, status, and license | [Rayfish](https://github.com/rayfish/rayfish) | live repo | 2026-09-18 | medium-high |
| [15] | iroh-lan transient overlay design | [rustonbsd](https://github.com/rustonbsd/iroh-lan) | live repo | 2026-09-18 | medium |
| [16] | Kith sync, pairing, membership, and epoch design | [Kith](https://github.com/muhamadjawdatsalemalakoum/kith) | live repo | 2026-09-18 | medium |
| [17] | Vellum notes, native/WASM sync, tickets, MCP | [Vellum](https://github.com/andymitch/vellum) | live repo | 2026-09-18 | medium |
| [18] | Synesis local files and online direct sync | [Synesis](https://github.com/grimfeld/synesis) | live repo | 2026-09-18 | medium |
| [19] | Teamtype collaboration model and license | [Teamtype](https://github.com/teamtype/teamtype) | live repo | 2026-09-18 | medium-high |
| [20] | p2panda modular local-first architecture | [p2panda](https://github.com/p2panda/p2panda) | live repo | 2026-09-18 | medium-high |
| [21] | GuardianDB replica, reconciliation, stores, and license | [GuardianDB](https://github.com/wmaslonek/guardian-db) | live repo | 2026-09-18 | medium |
| [22] | Proscenium messaging/key hierarchy and alpha status | [Proscenium](https://github.com/iohzrd/proscenium) | live repo | 2026-09-18 | medium |
| [23] | Chatmail core Iroh 0.35 dependency | [Chatmail](https://github.com/chatmail/core/blob/main/Cargo.toml) | live source | 2026-09-18 | high |
| [24] | Draft Iroh 1.x migration and blockers | [Chatmail](https://github.com/chatmail/core/pull/8182) | live PR | 2026-09-18 | high |
| [25] | Delta Chat peer-channel implementation | [Delta Chat source](https://rs.delta.chat/src/deltachat/peer_channels.rs.html) | live source | 2026-09-18 | high |
| [26] | WebXDC real-time transport abstraction | [Delta Chat](https://delta.chat/en/2024-11-20-webxdc-realtime) | 2024-11-20 | 2026-09-18 | high |
| [27] | Sendme capability and verified blob transfer | [n0-computer](https://github.com/n0-computer/sendme) | live repo | 2026-09-18 | high |
| [28] | Dropwire transfer boundary and workflow | [Dropwire](https://github.com/muhamadjawdatsalemalakoum/dropwire) | live repo | 2026-09-18 | medium |
| [29] | Knot Git lead; upstream access returned 403, details unverified | [Knot](https://tangled.org/lmao.bsky.social/knot-iroh/) | inaccessible live repo | 2026-09-18 | unverified |
| [30] | Git smart protocol bridge over Iroh | [git-remote-iroh](https://github.com/magik6k/git-remote-iroh) | live repo | 2026-09-18 | medium |
| [31] | Experimental live media project | [n0-computer](https://github.com/n0-computer/iroh-live) | live repo | 2026-09-18 | medium |
| [32] | Minimal Obsiroh prototype | [Obsiroh](https://github.com/DrHongos/obsiroh) | live repo | 2026-09-18 | medium-low |
| [33] | Stateless relay and no stored data | [Iroh](https://www.iroh.computer/blog/shared-relays) | 2026-09 | 2026-09-18 | high |
| [34] | Chatmail core MPL-2.0 license | [Chatmail](https://github.com/chatmail/core/blob/main/LICENSE) | live source | 2026-09-18 | high |
| [35] | SecureJoin QR token validation and freshness behavior | [Delta Chat source](https://rs.delta.chat/src/deltachat/securejoin.rs.html) | live source | 2026-09-18 | high |
| [36] | Owner-only broadcast/channel secret design | [Chatmail](https://github.com/chatmail/core/issues/6884) | live issue | 2026-09-18 | medium-high |
| [37] | JSON-RPC API boundary | [Chatmail](https://github.com/chatmail/core/blob/main/deltachat-jsonrpc/src/api.rs) | live source | 2026-09-18 | high |
| [38] | iroh-blobs production-line maturity warning | [n0-computer](https://github.com/n0-computer/iroh-blobs) | live README | 2026-09-18 | high |
| [39] | iroh-docs meta-protocol over blobs and gossip | [n0-computer](https://github.com/n0-computer/iroh-docs) | live README | 2026-09-18 | high |
| [40] | Mutual authentication of Iroh connections | [docs.rs](https://docs.rs/iroh/1.2.0/iroh/) | 1.2.0 docs | 2026-09-18 | high |

## Робочі матеріали

Основні first-party джерела наведено inline. Робочі source digests:

- `digests/iroh-core-r1-agent.md`
- `digests/deltachat-iroh-r1-agent.md`
- `digests/iroh-projects-r1-root.md`
- `digests/iroh-projects-r2-agent.md`

**Staleness map:** release versions, dependency pins, project activity and open migration PRs перевіряти перед кожним architecture lock; core concepts і licensing — щоквартально; mobile/browser constraints — під кожен supported release target.
