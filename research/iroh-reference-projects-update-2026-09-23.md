---
title: "Актуалізація Iroh reference projects — UniClipboard organization"
type: research-addendum
status: readme-and-manifest-checked
snapshot: "2026-09-23 Europe/Kyiv"
source: "UniClipboard organization page, Engine README/ARCHITECTURE/Cargo manifests, rc.18 release and storage policy"
---

# UniClipboard organization: окремий Engine reference

Цей follow-up доповнює [основний каталог](iroh-local-first-research-index-2026-09-18.md), [addendum 21–22.09](iroh-reference-projects-update-2026-09-22.md) і [бібліотечний inventory 23.09](iroh-project-library-inventory-2026-09-23.md). Організаційна сторінка GitHub переглянута 23.09.2026; README, `ARCHITECTURE.md` і manifests `UniClipboard/Engine` прочитані окремо. Це структурний/reference review, не code/security audit.

## Новий окремий reference repository

| Проєкт | Що підтверджено | Reference для VIDA | Межі доказу |
|---|---|---|---|
| [UniClipboard/Engine](https://github.com/UniClipboard/Engine) | Shared Rust core для macOS, Windows, Linux, iOS, Android і HarmonyOS; `uc-engine` як єдиний стабільний Rust entry point; `uc-core`/`uc-application`/`uc-infra`; device identity, spaces/pairing, clipboard/file sync, encrypted history/search/persistence, migrations, lifecycle, UniFFI/N-API bindings і release provenance. `uc-infra` manifest декларує Iroh network stack, `iroh-blobs`, mDNS/address lookup, relay/tickets і discovery components. | Сильний reference для host ↔ portable core boundary, mobile lifecycle, encrypted local persistence, binding ownership, release-manifest/SHA-256 provenance та окремого LAN compatibility channel. | Pre-release/активна розробка; exact Iroh dependency tuple і release artifacts треба перевірити в lockfile/CI. Workspace manifest вказує `1.1.0-rc.18` і Apache-2.0, тоді як README описує pre-1.0 surface — version/status треба вважати перевірюваним фактом, не готовою production-гарантією. Не переносити UniClipboard domain semantics або dependency forks у VIDA автоматично. |

## Organization map і deduplication

- [UniClipboard/UniClipboard](https://github.com/UniClipboard/UniClipboard) уже був у попередньому каталозі як end-user clipboard product; цей follow-up додає саме `Engine`, не дублює продукт.
- [UniClipboard/UniClip](https://github.com/UniClipboard/UniClip) позначений організацією як fork mobile client; не рахувати новою Iroh implementation без окремої перевірки.
- `relay`, `uc-rendezvous`, `uc-status`, `uc-website`, `feedlog`, `homebrew-*` — support/deployment/monitoring/packaging repos; вони не додаються до Iroh implementation count на підставі org page.

## Наступний gate

Для VIDA варто порівняти `UniClipboard/Engine` з `iroh-ffi`, Acerola, Nexa mobile і Arachne за однаковими fixtures: process kill/resume, cancellation/error ownership, secure-storage boundary, binding version skew, relay/direct path, encrypted persistence and migration. Engine є reference для протоколів і меж відповідальності, не рекомендованою залежністю VIDA.

Повторна перевірка наданого посилання не змінила цей висновок. [Реліз `v1.1.0-rc.18`](https://github.com/UniClipboard/Engine/releases/tag/v1.1.0-rc.18) містить конкретні lifecycle і durable-delivery виправлення, але це ще не результати VIDA fixtures. [Політика зберігання Engine](https://github.com/UniClipboard/Engine/blob/main/docs/security/encrypted-persistence.md) явно допускає сирі байти файлів у керованому blob store/import directory; для приватних файлів VIDA потрібен окремий file-at-rest gate, не припущення про повне шифрування. [Маніфест](https://github.com/UniClipboard/Engine/blob/main/crates/uc-infra/Cargo.toml) використовує Iroh `1.0.0-rc.1` та `iroh-blobs 0.102` із [закріпленим fork](https://github.com/UniClipboard/Engine/blob/main/Cargo.toml), а затверджена транспортна основа VIDA — Iroh 1.2: пряма залежність потребувала б суміснісного прототипу. Повний [BMad evidence і відкриті перевірки](../_bmad-output/planning-artifacts/research/technical-iroh-project-library-inventory-for-vida-2026-09-23/research.md) вже існують; нового вибору provider або Clipboard App не зроблено.

Облік: +1 distinct repository URL (`UniClipboard/Engine`); поточний орієнтир каталогу — 79 distinct URLs разом із Annoda як adjacent lead.
