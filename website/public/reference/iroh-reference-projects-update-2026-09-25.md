---
title: "Актуалізація Iroh reference projects — зріз 24 → 25 вересня 2026"
type: research-addendum
status: user-report-with-readme-and-manifest-spot-checks
snapshot: "2026-09-25 Europe/Kyiv"
source: "Користувацький GitHub-звіт; repository READMEs і вибрані manifests перевірені 25.09"
---

# Актуалізація Iroh reference-проєктів: 25.09.2026

Цей addendum продовжує [основний індекс](iroh-local-first-research-index-2026-09-18.md) і [зріз 24.09](iroh-reference-projects-update-2026-09-24.md). Попередній орієнтир становив близько 88 distinct repository URLs разом із Annoda як adjacent lead. Новий звіт додає **3 записи, яких ще не було**: один повноцінний remote desktop, один distributed VCS backend і один design/research lead.

Це reference review, не повний code/security audit. README/маніфести перевірені точково 25.09; commit cutoff і пошукові exclusions походять із наданого звіту.

## Нові references

| Категорія | Проєкт | Що підтверджено | Reference для VIDA | Межі доказу |
|---|---|---|---|---|
| Remote desktop | [pirncedark/afudesk](https://github.com/pirncedark/afudesk) | Flutter/Rust remote desktop для Windows host + Android viewer: screen, mouse/keyboard, clipboard, files, gamepad, NAT hole punching, relay fallback, mDNS і trusted devices. README та `rust/Cargo.toml` підтверджують `iroh = 1.2`; protocol розділяє control/video/file streams. | Сильний application transport reference для persistent device identity, pairing/permissions, multiplexed QUIC streams, reconnect і host/mobile split. | README описує готові сценарії, але це не доказ VIDA security/revocation/conformance; Windows/Android permission and performance fixtures ще не перевірені. |
| Distributed VCS / Jujutsu | [amber-store/ajj](https://github.com/amber-store/ajj) | Jujutsu `jj` backend поверх amber-store objects; bookmarks — dstore references; `dstore-transport-iroh` у manifest, а `[patch.crates-io]` pins Iroh 1.2.0 compatibility patch. Fetch/push, CAS objects, tickets, prefix, relay/no-relay/no-discovery, compare-and-swap refs і tests описані в README. | Reference для application objects → content-addressed store → distributed references; корисний для file-first history, immutable revisions і remote bookmark semantics без традиційного Git server. | LGPL-3.0-only; dstore/amber authority model не є VIDA membership або operation receipt; Iroh patch compatibility потребує окремого build/prototype gate. |
| Distributed platform concept | [ThePyth0nKid/ultra-netz](https://github.com/ThePyth0nKid/ultra-netz) | Research/design repo для encrypted fragmented community storage: Nostr account/device identity, Iroh L0, blobs/gossip/docs/CRDT, community nodes, zero-knowledge і low-RAM operation. README прямо каже: researched, not built. | Architecture lead для identity/transport/storage separation, community-operated nodes і encrypted fragment placement; корисний як design comparison, не implementation. | Немає реалізованої Iroh системи або production code; концепт/тексти CC BY 4.0, майбутній code AGPL-3.0; не рахувати як provider або runnable VIDA dependency. |

## Відсіяні записи

- `oneiron-dev/iroh`, `ttizze/iroh` — копії самого Iroh; `dash-chat/iroh-gossip` — копія старого `iroh-gossip`.
- `SolutionsAsService/guardian-db` — копія GuardianDB; `signingup/tincan-cli` — копія/імпорт старого Tincan.
- `microgift/arion` — distributed storage на Iroh, але history починається у лютому.
- `retpel/flakes` і `rayfish.nix` — Nix packages для Rayfish, не нові Iroh application projects.

## Що додати до VIDA comparison map

Цей зріз додає два implementation gates і один design lead: **remote desktop** (persistent identity + permissioned multiplexed streams), **distributed VCS/storage** (CAS objects + distributed refs) та **encrypted community storage concept** (identity/transport/storage separation). Не змішувати design-only Ultra-Netz із runnable AfuDesk/ajj у maturity або provider ranking.

Облік: **+3 distinct repository URLs**; поточний орієнтир каталогу — приблизно **91 distinct repository URLs**, включно з Annoda як adjacent lead.
