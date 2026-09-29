---
title: "Актуалізація Iroh reference projects — зріз 27 → 29 вересня 2026"
type: research-addendum
status: user-report-with-live-readme-spot-checks
snapshot: "2026-09-29 Europe/Kyiv"
source: "Користувацький GitHub-звіт; README та вибрані facts перевірені 29.09"
---

# Актуалізація Iroh reference-проєктів: 27–29.09.2026

Цей addendum продовжує [основний індекс](iroh-local-first-research-index-2026-09-18.md) і [зріз 27.09](iroh-reference-projects-update-2026-09-27.md). Попередній орієнтир становив близько 97 distinct repository URLs разом із Annoda як adjacent lead. Новий звіт додає **5 нових репозиторіїв** і окремо фіксує hardening/release updates для вже відомих references.

Це reference review, не повний code/security audit. README/facts нових репозиторіїв перевірені точково 29.09; commit cutoff і exclusions походять із наданого звіту.

## Нові references

| Категорія | Проєкт | Що підтверджено | Reference для VIDA | Межі доказу |
|---|---|---|---|---|
| Agent mesh / workspace | [Walkie](https://github.com/alexcarney460-hue/walkie) | P2P agent workspace: signed team log, projects/Data Rooms, seats, remote agent actions, phone pairing, Walkie Direct, encrypted relay fallback і pooled local LLM через llama.cpp RPC; daemon має SQLite log та Ed25519 node key. | Найцікавіший новий reference для distributed agent workspace, signed replicated log і human kill-switch над remote agent operations. | FSL-1.1-ALv2; agent authorization, log finality, multi-owner recovery, pooled-model isolation і VIDA receipts не доведені. |
| Remote desktop | [FarSail](https://github.com/wanghao9103/farsail) | Rust/Tauri Windows remote desktop/file-transfer design із Iroh data channel, local direct path і self-hosted TLS relay; device binding, explicit host approval, JPEG capture, mouse/keyboard control та local qualification. | Reference для account/device/auth separation, explicit approval і desktop stream lifecycle. | README прямо позначає проєкт як in design; public cross-NAT, file content, mobile control, second Windows і unsigned installer лишаються unverified. |
| Remote execution / MCP | [Portal](https://github.com/lab47/portal) | Go `go-iroh v0.2.1` transport для remote commands; Iroh EndpointId як transport identity, SSH user certificate як authorization, argv-bound challenge, coordinator inventory без payload proxy, MCP interface. | Чіткий приклад transport identity ≠ application authorization і мінімального control plane для remote execution. | SSH CA/user policy, OS command sandbox, certificate lifecycle, relay exposure і MCP tool authorization потребують VIDA-specific tests. |
| File transfer | [ahole](https://github.com/ast/ahole) | Magic-Wormhole-like CLI на stock `iroh-blobs`: collections, BLAKE3, resumable/content-addressed transfer, relay→direct upgrade, self-hosted relay, path-traversal protection і sender exit після completion. | Мінімальний reference для `iroh-blobs 1.x`, collections і verified file transfer без custom application protocol. | Temporary store/GC, receiver authorization, relay token custody, crash recovery і durable File/manifest commit не доведені. |
| Pairing / permissions | [ErisAuth](https://github.com/Eriskii/ErisAuth) | Reusable Iroh pairing layer: QR ticket, one-time redemption, fingerprint ceremony, `pending → requested → approved → collected`, permission patterns, signed tokens і derived/stable keys. | Сильний building block для device enrollment, workspace join, permission grant і explicit human approval. | Crate `v0.2.0` і protocol semantics потребують compatibility, replay/race, revocation, key custody та multi-device fixtures; pairing code alone grants no permission. |

## Значні оновлення існуючих references

| Проєкт | Оновлення | Практичний висновок для VIDA |
|---|---|---|
| [Antgrid](https://github.com/antgrid-ai/antgrid/commit/1b35ff9156189f280b69273bb3a633dbf3708897) + [reconnect](https://github.com/antgrid-ai/antgrid/commit/f315beb2c15db87d4e4dbd059fa7b00c20fc0912) | Iroh leases переживають self-push/desktop focus; після reconnect телефон отримує пропущені events; mobile remote-agent UI. | Native data-plane migration перейшла до reconnect/resume hardening. |
| [Codevisor](https://github.com/851-labs/codevisor) | Path + RTT visibility, Linux native tunnel packaging і fleet/cloud-machine management поверх спільного Iroh core. | Network observability і fleet identity мають бути частиною daemon contract. |
| [rho 0.11.0](https://github.com/casonadams/rho) | Peer identity/hostname handshake, prompt + turn-start sync, footer metrics/session activity, E2E parity tests і release 0.11.0. | Collab наближається до shared agent session, а не лише remote terminal. |
| [NexaPipe 0.2.0](https://github.com/open-nexa/nexapipe) | Request/config/lifecycle hardening; Android credential key не regenerates при keystore error. | Device identity key continuity є окремим correctness/security gate. |
| [blirp 0.3.0](https://github.com/backyarddd/blirp) | Multi-machine merge-following fixes, distributed memory/distillation fixes, project-file stability через hub. | Чотири application ALPN вже мають реальні convergence scenarios; hub authority все ще треба перевірити. |
| [Legato](https://github.com/tvolk131/legato) | Working pointer sharing/yield, cross-monitor cursor transition між Mac і PC. | Iroh використовується для low-latency input/display session, не лише pairing. |
| [AfuDesk](https://github.com/pirncedark/afudesk) | Decentralization audit E1–E9 і transport-layer fixes після Iroh 1.2/trusted devices/WAN/relay/reconnect. | Remote desktop reference перейшов до окремого hardening evidence, але не є VIDA conformance. |
| [remote-device-sync](https://github.com/NDDev-OpenNetwork/remote-device-sync) | Hardened shared control reader, cancel/teardown і terminal-cause propagation; native macOS qualification lanes. | Teardown semantics і platform qualification важливіші за сам факт relay connectivity. |

## Що додати до VIDA comparison map

Новий кластер посилює три пов'язані шари: **agent workspace/log** (Walkie), **enrollment/permissions** (ErisAuth) і **remote execution/device control** (Portal/FarSail). [ahole](https://github.com/ast/ahole) додає мінімальний stock-blobs baseline. Не змішувати transport identity, user authorization, pairing approval і application receipt в один Iroh contract.

## Відсіяні та нераховані записи

`NanoMesh`, `Agentaps`, `Aster`, `NFX`, `Kith` та інші записи зі старшою commit history не рахуються новими; форки `iroh`, `iroh-blobs`, `iroh-gossip`, Alleycat, Dashbeam і `awesome-*` також відсіяні.

Облік: **+5 distinct repository URLs**; поточний орієнтир каталогу — приблизно **102 distinct repository URLs**, включно з Annoda як adjacent lead.
