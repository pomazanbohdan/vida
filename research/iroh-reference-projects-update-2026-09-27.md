---
title: "Актуалізація Iroh reference projects — зріз 25 → 27 вересня 2026"
type: research-addendum
status: user-report-with-live-readme-and-commit-spot-checks
snapshot: "2026-09-27 Europe/Kyiv"
source: "Користувацький GitHub-звіт; README і linked commits вибірково перевірені 28.09; manifest details retained from report"
---

# Актуалізація Iroh reference-проєктів: 25–27.09.2026

Цей addendum продовжує [основний індекс](iroh-local-first-research-index-2026-09-18.md) і [зріз 25.09](iroh-reference-projects-update-2026-09-25.md). Попередній орієнтир становив близько 91 distinct repository URLs разом із Annoda як adjacent lead. Новий звіт додає **5 нових project groups / 6 distinct repository URLs**; окремо фіксує сім суттєвих оновлень уже відомих репозиторіїв.

Це reference review, не повний code/security audit. Нові README та linked commits перевірені точково 28.09; manifest details, commit cutoff і пошукові exclusions походять із наданого звіту.

## Нові references

| Категорія | Проєкт | Що підтверджено | Reference для VIDA | Межі доказу |
|---|---|---|---|---|
| Agents / ACP | [iroh-acp-go](https://github.com/carsonfarmer/iroh-acp-go) | Go ACP client/server над `go-iroh v0.2.1`: Ed25519 EndpointId, tickets, direct/relay, EndpointId allow-list, один bi-stream, ALPN `acp/1`, ACP JSON-RPC без перетворень. | Дуже малий reference для мережевого ACP: editor → remote agent без окремого application RPC шару. | Go port і process ownership потребують compatibility, authorization, reconnect та agent-lifecycle fixtures; README не є security audit. |
| Messenger / files | [Doot](https://github.com/Brajesh3/doot) | Desktop + Android P2P messenger/transfer: Iroh Router, `iroh-blobs`, `iroh-gossip`, `iroh-ping`, direct/relay telemetry, path migration, persistent contacts/messages, JNI і headless bot. | Приклад Iroh Endpoint як application bus із кількома офіційними protocols над одним QUIC endpoint. | E2EE, mobile background, storage recovery, abuse limits і release conformance VIDA не доведені. |
| Collaboration | [acidtrip](https://github.com/jondot/acidtrip) | `acidtrip-net` на Iroh 1.2 + tickets + mDNS; host є canonical source of truth, guests надсилають versioned transactions; tickets, reconnect, cursor presence, peer list і versioned ALPN. | Простий host-ordered state replication без CRDT; корисний для малих collaborative surfaces. | Host availability/order semantics не є durable SyncLog; потрібні conflict, reconnect, permissions і multi-host fixtures. |
| Multi-agent workspace | [blirp](https://github.com/backyarddd/blirp) | Iroh 1.2 + mDNS + SPAKE2; окремі ALPN `blirp/pair/1`, `/sync/1`, `/proxy/1`, `/files/1`; sessions, project memory, outbox, hub log, presence, project files. | Найближчий новий reference для agent hub: один identity/transport layer і розділені application protocols. | Secrets, summarizer boundaries, revocation, durable hub authority та cross-OS lifecycle потребують перевірки; README не доводить VIDA semantics. |
| Data/RPC SDK | [Grainlift TypeScript](https://github.com/Query-farm/grainlift-typescript) | Node toolkit для typed Grainlift ADBC workers; raw Iroh QUIC через `vgi-iroh-bridge` 0.27.3, verified EndpointId authorization, canonical contract parity, resource/session limits. | Multi-language application RPC із transport-specific identity та fail-closed bridge boundary. | Це SDK над окремим bridge, не самостійна Iroh transport implementation; package/release and VGI compatibility gates лишаються. |
| Data/RPC SDK | [Grainlift Go](https://github.com/Query-farm/grainlift-go) | Go Grainlift workers із Iroh bridge, allowlisted authenticated EndpointId, published `vgi-rpc-go v0.30.0`, raw adapters fail closed зі старими VGI версіями. | Go-side reference для typed Arrow/ADBC RPC, contract fixtures і transport compatibility policy. | Bridge/process boundary, version pin, conformance fixture та production release parity потрібно перевірити окремо. |

## Суттєві оновлення існуючих references

| Проєкт | Оновлення | Практичний висновок для VIDA |
|---|---|---|
| [Antgrid](https://github.com/antgrid-ai/antgrid/commit/36ec4afa6def10bba497b69ff74b9e5a8b7d6926) | Remote payloads перенесені на native Iroh streams; WebSocket лишився control-plane only; stock relay і Flutter/Node bindings. | Розділяти control plane та native data plane; transport migration треба перевіряти на backpressure, auth і replay. |
| [Codevisor](https://github.com/851-labs/codevisor/commit/a07e3eefd5e1faa208b7916bb2ccdc92a50f9156) | Новий `codevisor-net` на Iroh 1.2; NAPI-RS для Node, UniFFI для Apple; bi-stream control і QUIC datagrams для screen sharing. | Один Rust networking core може мати різні host bindings і stream/datagram media paths. |
| [rho](https://github.com/casonadams/rho/commit/2ec3b8b62eb3c5868896ed76cd13832f74dd90c6) | Повний Collab mode: host/guest, encrypted handshake, capability levels, spectator/co-pilot, ticket join. | Capability-scoped collaboration треба відділяти від endpoint identity та session admission. |
| [remote-device-sync](https://github.com/NDDev-OpenNetwork/remote-device-sync) | Security/audit pass: bounded tickets, sanitized errors, hardened file opens, relay allowlist fail-closed, rate limits, typed lifecycle events, migration benchmarks. | Корисний security-hardening checklist для remote device transport; не замінює VIDA conformance. |
| [NexaPipe](https://github.com/open-nexa/nexapipe/commit/b45324aa58cd95d1426294a028d7b56f8f3ddf42) | Graceful drain, фактичний Iroh path logging, configurable health checks і CodeQL. | Shutdown/path observability мають бути частиною transport lifecycle evidence. |
| [Audia](https://github.com/liquidiert/audia) | Активний 27.09; CSP дозволяє Iroh relay connections, додані distributed voting modes. | Browser relay policy та application voting state — окремі security і consistency gates. |
| [Godot Iroh](https://github.com/tipragot/godot-iroh/commit/6c85a4e33580f1bdae839836b12222a7b1294625) | Оновлення до Iroh 1.2. | Підтверджує ще один engine binding, але не доводить VIDA mobile/desktop lifecycle. |

## Що додати до VIDA comparison map

Новий кластер зміщує акцент до **native application bus для agents, collaboration і typed RPC**: ACP transport (`iroh-acp-go`), multi-protocol runtime (Doot), host-ordered collaboration (acidtrip), multi-machine agent hub (blirp) і bridge-backed Grainlift SDKs. Не змішувати SDK/bridge references із самим Iroh transport provider та не рахувати design/application claims як доказ VIDA authority, receipt або E2EE.

## Відсіяні та нераховані записи

У звіті не рахувалися форки, copies, Dependabot і записи без нової independent Iroh history. Уже відомі проєкти з оновленнями залишені в окремій таблиці, а не пораховані як нові.

Облік: **+6 distinct repository URLs / 5 project groups**; поточний орієнтир каталогу — приблизно **97 distinct repository URLs**, включно з Annoda як adjacent lead.
