---
title: "Актуалізація Iroh reference projects — зріз 23 → 24 вересня 2026"
type: research-addendum
status: user-report-with-readme-and-manifest-spot-checks
snapshot: "2026-09-24 20:48 Europe/Kyiv"
source: "Користувацький GitHub-звіт; repository READMEs і вибрані manifests перевірені 24.09"
---

# Актуалізація Iroh reference-проєктів: 23–24.09.2026

Цей addendum продовжує [основний індекс](iroh-local-first-research-index-2026-09-18.md) та [вечірній зріз 23.09](iroh-reference-projects-update-2026-09-23-evening.md). Попередній орієнтир становив близько 85 distinct repository URLs разом із Annoda як adjacent lead. Звіт додає **3 справді нові Iroh-проєкти**; решта нових GitHub records мають старішу історію, є імпортами/форками або продовженнями вже зафіксованих проєктів.

Це reference review, не повний code/security audit. Дати старту й функціональні акценти нижче взяті зі звіту; README/маніфести трьох нових проєктів перевірені точково 24.09.

## Три нові проєкти

| Категорія | Проєкт | Що підтверджено | Reference для VIDA | Межі доказу |
|---|---|---|---|---|
| KVM / device sharing | [tvolk131/legato](https://github.com/tvolk131/legato) | Mac/Windows keyboard, mouse, clipboard і files over Iroh; README позначає pre-alpha/research spikes, Rust/iced UI, automatic LAN discovery і 6-digit pairing. Звіт фіксує spike з `iroh 1.2`, mDNS address lookup, tickets, Endpoint streams/datagrams, `EndpointHooks::after_handshake`, allow-list та ALPN `legato/1`/`legato/pair/1`; first commit — 24.09. | Reference для discovery → pair ALPN → SAS → persistent EndpointId allow-list → normal application ALPN; найближчий аналог seamless multi-device input/clipboard boundary. | Нічого usable ще немає; root workspace manifest окремо від spike dependencies; virtual monitor лише планується; OS input/clipboard permissions і reconnect fixtures VIDA не пройдені. |
| Social desktop P2P | [BloopPet/bloop](https://github.com/BloopPet/bloop) | Tauri/React desktop pet з peer pairing; `src-tauri/Cargo.toml` pins `iroh = 1.2` і `iroh-mdns-address-lookup = 0.5`. README описує one-time invite code з TTL 15 хвилин, paired EndpointId, direct/mDNS/relay paths, explicit message schema, privacy boundary, 16 KB/40 msg/s limits за звітом і MIT license; first commit — 24.09. | Малий чистий reference для transport security окремо від TypeScript application validation, EndpointId ACL, mDNS/direct/relay fallback і privacy-minimal protocol. | macOS desktop focus, young project, non-notarized release; pet state/messages не є generic workspace model, а paired-key admission потребує власних revocation/recovery fixtures. |
| ESP32 / Wi-Fi sensing | [Remade-With-Rust/rusty_esp_sense](https://github.com/Remade-With-Rust/rusty_esp_sense) | Host-only Rust reader для Janus ESP32 CSI: live `watch` path або recording, room-calibrated Candle model; README підтверджує raw CSI stream, `watch --model … <ticket>`/`--bridge <ticket>`, ~7 KB/s @ 50 Hz і MIT OR Apache-2.0. Manifest підключає `rusty_esp_iroh-host` git dependency на `w5/csi-stream`; first commit — 24.09. | Новий embedded chain reference: device transport → Iroh host bridge → downstream telemetry/model logic; корисний для ESP32 lifecycle, ticket handoff і host/device split. | Live feature залежить від Janus W5 branches; модель host-only і dataset-specific, device run ще не виконаний; sensing accuracy не є транспортною або VIDA presence гарантією. |

## Не рахуються як нові в цьому зрізі

- `mochar/iroh-c-ffi` — новий у пошуковій вибірці, але історія починається у 2024.
- `DavHau/landline` — walkie-talkie з історією від 02.09.
- `counterpunchtech/iroh`, `Ephemushroom/UniClipboard`, Dashbeam та інші копії — не незалежні нові implementation histories.
- `safety-net-flutter` — цікавий Flutter/DKG client, але історія почалася 23.09 до попереднього cutoff.
- `Nostos` — Iroh 1.1 як QUIC transport для local-first Postgres sync, але history починається 22.09.
- `Grainlift` — продовження/перейменування вчорашнього Query Farm ADBC/Iroh project, не другий новий reference.

## Що додати до VIDA comparison map

Новий зріз додає три окремі gates: **KVM/device sharing** (pairing/discovery/hooks/allow-list), **desktop social P2P** (minimal schema + EndpointId ACL) і **embedded telemetry** (ESP32 bridge + live stream + downstream model). Iroh тут є transport substrate; input authority, message validation, device identity, telemetry semantics і durable receipts залишаються application contracts.

Облік: **+3 нові implementation repositories**; поточний орієнтир каталогу — приблизно **88 distinct repository URLs**, включно з Annoda як adjacent lead.
