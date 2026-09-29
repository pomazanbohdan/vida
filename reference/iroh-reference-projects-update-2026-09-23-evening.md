---
title: "Актуалізація Iroh reference projects — зріз 22.09 18:37 → 23.09 22:20"
type: research-addendum
status: user-report-with-readme-and-manifest-spot-checks
snapshot: "2026-09-23 22:20 Europe/Kyiv"
source: "Користувацький GitHub-звіт; repository READMEs і вибрані manifests перевірені 23.09"
---

# Актуалізація Iroh reference-проєктів: вечірній зріз 23.09.2026

Цей addendum продовжує [основний індекс](iroh-local-first-research-index-2026-09-18.md), [зріз 21–22.09](iroh-reference-projects-update-2026-09-22.md), [UniClipboard follow-up](iroh-reference-projects-update-2026-09-23.md) і [library inventory](iroh-project-library-inventory-2026-09-23.md). Попередній орієнтир становив близько 79 distinct repository URLs разом із Annoda як adjacent lead. Новий звіт додає **5 справді нових проєктів** у часовому вікні та окремо **AgnView**, який став видимим у вибірці, але має старішу внутрішню історію.

Це reference review, не повний code/security audit. Старт історії та функціональні акценти нижче взяті зі звіту; README/маніфести перевірені точково 23.09.

## П'ять нових проєктів

| Категорія | Проєкт | Що підтверджено | Reference для VIDA | Межі доказу |
|---|---|---|---|---|
| Coding agents | [MainListActivity/maplayerdev](https://github.com/MainListActivity/maplayerdev) | Host daemon спавнить Codex/Cursor через ACP і переносить stdio JSON-RPC через Iroh QUIC, ALPN `maplayer/1`; README підтверджує Android `computer.iroh:iroh-android`, Tauri desktop, pairing PIN/ticket, allowlist, direct hole punching і n0 relay fallback. `server/Cargo.toml` має `iroh = 1.2.0`; reported history start — 23.09 01:10. | Найближчий reference для local agent service + mobile/desktop control plane; server owns credentials and agent processes. | Linux/macOS host only, managed sessions не переживають restart, лише офіційний n0 relay у v1; exact Android lifecycle/permission/error fixtures VIDA ще не проходив. |
| Data / RPC | [Query-farm/adbc-proxy](https://github.com/Query-farm/adbc-proxy) | Stateful ADBC/Arrow RPC proxy над VGI-RPC; transport options HTTP(S), TCP/mTLS і Iroh; `iroh://` endpoint, EndpointId identity, principal/target authorization, pooling, Arrow batch streaming. Manifest pins `iroh = 1.1.0`; reported history start — 23.09. | Reference для application RPC над raw Iroh QUIC, endpoint-to-principal mapping, quotas/deadlines/cancellation та durable server-side state. | Pre-release, binaries/Windows CI ще не готові; Iroh tuple 1.1 не є прямою сумісністю з VIDA Iroh 1.2; database authority і VIDA operation semantics не успадковуються. |
| Browser / Gossip | [liquidiert/audia](https://github.com/liquidiert/audia) | Browser collaborative playlist: phone peers голосують, display peer агрегує; Rust `audia-gossip` компілюється у WASM з `iroh = 1.2`, `iroh-gossip = 0.101`, `wasm-bindgen`; підписані endpoint events і relay-only шлях через відсутність UDP у browser. | Реальний reference для Iroh protocol у browser/WASM UI, gossip event log, session owner і signed finalization. | Relay-only не доводить browser availability/security для VIDA; gossip не є durable application ACK, а playlist/session semantics не є generic workspace model. |
| Messenger / game | [Spaceghost/xivlantern-dalamud](https://github.com/Spaceghost/xivlantern-dalamud) | FFXIV Dalamud plugin: friends/presence, 1:1 chat, channels, offline queued delivery, signed announcements, block/mute, file transfer, custom relay/LAN-only і SQLite persistence; README підтверджує P2P over Iroh, C ABI (`lantern.dll`) і окремий C# binding. Звіт фіксує Iroh 1.2 + gossip + blobs; history start — 23.09. | Strong reference для стабільного C ABI між Rust P2P core і C# host, presence/offline delivery, relay opt-out та messaging/file boundary. | Plugin компілюється проти Dalamud 15, але ще не завантажувався ним; exact dependency tuple і media/MoQ plan потребують окремої перевірки; FFXIV domain не переноситься у VIDA. |
| Workspace mesh | [bobishh/mesh-lighthouse](https://github.com/bobishh/mesh-lighthouse) | Standalone Rust MetaMesh participant; `Cargo.toml` має `iroh = 1.2.0`, MetaMesh/Match vendored submodules і Automerge 0.12; owner-issued invite, signed workspace state, restart replication, verified peers, HTTP intake у durable inbox ще до pairing. Reported history start — 23.09. | Reference для gateway/lighthouse node, який входить у Workspace як peer: public HTTP edge → durable inbox → native mesh node → workspace replication. | Перший consumer — Match; shared authority rules приходять із submodules; не доводить VIDA membership, inbox finality або multi-owner semantics без власних fixtures. |

## Окремо: AgnView із старішою історією

[tlaskar-git/AgnView](https://github.com/tlaskar-git/AgnView) має dashboard/coordination hub для Claude Code, Codex, AntiGravity, DeepSeek і custom agents. `pyproject.toml` pins `iroh==1.1.0`; README описує LAN-first pairing, bundled Iroh fallback, QR/node-ticket remote access, persistent secret і можливість власного relay. За наданим звітом внутрішня commit history починається 17.09, тому AgnView **не входить до п'ятірки нових проєктів цього вікна**, але додається як окремий agent-runtime reference. Це не доказ mobile/release readiness: mobile companions ще in development, а Iroh 1.1 потребує compatibility gate.

## Відсіяні записи

- `n0-grookie/iroh-services` — свіжа копія старішого `n0-computer/iroh-services`, history з 2025 року.
- `AkaShark/alleycat`, `flexaccessdev/net-tools`, `random-ml/iroh`, `ryanpetris/iroh` — копії/форки або стара upstream history.
- `bollfile` — новий проєкт, але поточний transport Hyperswarm; Iroh лише майбутній relay fallback.
- `wattetheria`, Dashbeam/mesh-llm/awesome-rust entries — старий код або копії без окремого нового Iroh implementation доказу.

## Що додати до VIDA comparison map

Новий кластер розширює Iroh як універсальний application transport: **agents** (`maplayerdev`), **database/RPC** (`adbc-proxy`), **browser gossip** (`audia`), **messenger** (`xivlantern`) і **workspace mesh** (`mesh-lighthouse`). Наступний gate — однакові fixtures для agent process ownership, RPC receipt/authorization, browser relay-only recovery, C ABI lifecycle, durable inbox/restart та Iroh 1.2 compatibility.

Облік: **+5 нових implementation repositories + 1 окремий earlier-history reference**; поточний орієнтир каталогу — приблизно **85 distinct repository URLs**, включно з Annoda як adjacent lead.
