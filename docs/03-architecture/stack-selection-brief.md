---
id: ARCH-STACK-DISCUSSION-001
status: draft
last_updated: 2026-09-23
purpose: Обрати реалізаційний стек VIDA за продуктовими сценаріями та вимірюваними capability gates, не підміняючи продуктову вимогу можливістю бібліотеки.
---

# Стек VIDA: кандидати, межі та критерії рішення

Це **документ для обговорення**, не ADR і не дозвіл розпочати реалізацію за неперевіреними припущеннями. Джерела `research/` містять і рішення користувача, і історичні пропозиції. Прийняті інваріанти наведено окремо від кандидатів.

## 1. Що вже визначено

- Парадигма: headless local-first Rust `vida-core` + `vida-runtime`, нейтральний `vida-sdk`, platform shells і конформансні сценарії; одна прикладна семантика без копій авторизації у Dart/C#/TypeScript ([spine](../../_bmad-output/planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md)).
- Основний транспорт — Iroh core 1.2; VIDA визначає власні версійовані ALPN/envelopes. Iroh не визначає ролі, конфлікти, durable offline mailbox чи прийняття бізнес-операції ([ADR-0005](decisions/ADR-0005-iroh-transport-foundation.md), [Iroh Rust docs](https://docs.rs/crate/iroh/latest)).
- Signed operation log і його authority-accepted frontier — доменна істина; SQL/пошук/UI — відтворювані проєкції. Direct та durable delivery несуть один operation ID, але ACK доставки не дорівнює прийняттю ([ADR-0006](decisions/ADR-0006-durable-delivery-and-operation-envelope.md), [SyncLog](../04-specifications/sync-log-contract.md)).
- Пакет App містить схеми, інтерфейс і керовану логіку в конкретному AppInstance; «плагін» не означає виконуваний Flutter/Tauri модуль із прямим доступом до ОС ([ADR-0007](decisions/ADR-0007-declarative-app-packages.md), [ADR-0009](decisions/ADR-0009-managed-application-logic.md)).
- Для двох конкуруючих статусів задачі продуктове правило — перший успішний authority-accepted sync визначає спільний стан; другий несумісний намір лишається pending до явного вибору автора, а не перезаписує стан (`REQ-SYNC-004`). Незалежні offline acceptance, точний receipt/ordering і granular text/code conflict UX — `OQ-0033`/`OQ-0034`.
- Першосторонні клієнти Release 1 — встановлювані Flutter Android/iOS/Windows і повноцінний статичний Flutter Web із Rust/Wasm Core. Messenger, Knowledge/Notes і Projects/Tasks працюють локально офлайн; віддалена доставка/acceptance лишається pending. Browser exclusion з [ADR-0014](decisions/ADR-0014-native-only-vida-clients.md) скасовано [ADR-0021](decisions/ADR-0021-static-web-client-in-release-1.md); Web bridge, storage і direct transport мають окремі доказові gates.
- Windows-клієнт має бути повноцінним: три базові Apps, клавіатура, системні меню, трей і screen reader — перевірювані вимоги, але **не** рішення на користь WinUI 3 чи іншого toolkit ([REQ-CLIENT-006](../02-requirements/native-client-requirements.md)).

## 2. Рекомендований експериментальний стек, не фінальний lock

| Шар | Робочий вибір | Чому для VIDA | Що ще довести |
|---|---|---|---|
| Семантика / runtime | Rust core + runtime + один application facade | Один код прав, команд, idempotency, schema і подій для різних оболонок | Threading, cancellation, DTO, error/compatibility contract (`OQ-0035`) |
| P2P transport | Iroh 1.2 у `VidaNodeHost`; `iroh-blobs/docs/gossip` лише за адаптерами | Direct+relay, authenticated QUIC, VIDA-owned protocol; вищі бібліотеки не тотожні стабільному core | Mobile lifecycle, mixed-version протокол, durable delivery topology |
| Mobile | Flutter Android/iOS + пробний `flutter_rust_bridge` | Один мобільний UI-код, Rust interop і нативні escape hatches | Release-build FFI, фоновий режим, secure storage, push, IME/a11y, процесне відновлення |
| Windows Release 1 | Flutter Windows | Спільний Flutter UI та Rust core, окреме Windows packaging/platform profile | Rust FFI, keyboard/menu/tray, screen reader, startup/memory і Windows distribution |
| Windows post-v1 | WinUI 3 + C# за окремим доказом потреби | Windows-native XAML controls без WebView для основного UI | C#↔Rust FFI, окремий UI-стек і AppPackage renderer; UX/a11y/performance gain має виправдати divergence |
| Локальні дані | Signed op log за replaceable storage contract; SQLite/FTS як початкова проєкція | Пошук/запити без перетворення SQL на канонічну істину | Durable pending queue, encryption, one writer, snapshot/rebuild/backup, provider swap (`OQ-0036`) |
| Binary/wire | PDM + deterministic CBOR profile — кандидат, JSON — авторинг/діагностика | Стандартна компактна основа без прив'язки до UI/БД | Canonical bytes, schema registry, unknown fields, signatures і golden vectors (`OQ-0028`) |
| Спільне редагування | Loro first-run; Automerge mandatory control; Yrs ecosystem control; editor adapters оцінюються разом | Усі кандидати мають Rust CRDT і stable-position/presence building blocks, але різні rich-text, conflict, persistence та Flutter integration trade-offs | F01–F13 на Android/iOS/Windows за [draft conformance spec](../04-specifications/crdt-editor-conformance.md); фінального lock немає |
| Calls/media | LiveKit first measured baseline; direct `flutter_webrtc` 1:1 + LiveKit groups as control; `iroh-live` R&D only | VIDA Core/Iroh retains signed call intent, membership and key epochs; media plane remains replaceable WebRTC/SFU adapter | F01–F15 E2EE, lifecycle, NAT/TURN, multi-device, metadata and resources by [draft conformance spec](../04-specifications/e2ee-calls-conformance.md); no final lock |
| Логіка пакетів | Декларативний baseline; Rhai/Wasmtime — окреме порівняння sandbox runtime | Можна обмежити guest API і бюджет без прямого OS/core-доступу | Hook phases, determinism, fuel/limits, side effects, permissions, package compatibility (`OQ-0045`) |

Основні перевірені upstream-можливості: [Flutter platforms](https://docs.flutter.dev/reference/supported-platforms), [Flutter native interop](https://docs.flutter.dev/platform-integration/bind-native-code), [FRB](https://cjycode.com/flutter_rust_bridge/), [WinUI 3](https://learn.microsoft.com/en-us/windows/apps/get-started/winui-get-started-overview), [.NET native interop](https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke-source-generation), [Tauri architecture](https://v2.tauri.app/concept/architecture/), [Tauri permissions](https://v2.tauri.app/security/permissions/), [Loro Rust API](https://docs.rs/crate/loro/latest), [Automerge conflicts](https://automerge.org/docs/reference/documents/conflicts/). [Windows-аналіз](windows-client-options.md) деталізує переваги, ризики та однаковий gate. Можливість бібліотеки **не доводить**, що VIDA-продукт задовольнив власну вимогу; це роблять fixtures, security review і прототип.

## 3. Вимоги, які стек мусить витримати

| ID | Перевірка в одному vertical slice | Gate |
|---|---|---|
| STACK-G1 | Та сама команда, право й відповідь на Android/iOS та Windows-кандидатах, допущених `OQ-0008`; Dart/C#/JS shell не реалізує доменний перехід повторно | Без semantic fork |
| STACK-G2 | Два offline статуси від однієї ревізії: лише перший у спільно порівнюваному authority-порядку є чинним; другий доступний тільки авторизованому актору; intentional next-base update можливий | Receipt/order і два Personal devices за `OQ-0033`/`OQ-0034` |
| STACK-G3 | Kill/restart у кожній точці local log+outbox/ACK/apply: немає втраченої операції, дубля ресурсу чи повторного зовнішнього ефекту | Rust runtime + storage recovery |
| STACK-G4 | Відкликання права діє на API, UI, пошук, preview, notifications і plugins; спроба команди напряму з shell відхиляється | Поточний Space/object/action authorization |
| STACK-G5 | Якщо Tauri буде допущено, hostile document/forum content не може викликати привілейовану команду або читати інший Space | Недовірений вміст не виконується в привілейованому WebView-контексті; мінімальні IPC capabilities, Rust-side authorization, CSP як додатковий захист і same-context XSS→IPC fixture. Сам per-webview allowlist не зупиняє injected script у вже привілейованому WebView |
| STACK-G6 | Фонове повідомлення та відновлення після OS kill перевірені на фізичних Android/iOS; телефон не оголошується постійно онлайн | Honest durable-delivery fallback |
| STACK-G7 | Rich document, task grid, global search, перемикання Space/Persona і скрінрідер проходять повний workflow | Installed Flutter Windows Release-1 build; offline command matrix `OQ-0012` |
| STACK-G8 | AppPackage із різних репозиторіїв відкривається за однаковою схемою/правами та UI-port semantics; неподтримувана вимога manifest відхиляється явно | Версійований cross-shell UI-port contract і fixtures `OQ-0056`, незалежний клієнт |
| STACK-G9 | Безстроковий за віком pending candidate переживає restart/backup; нестача місця дає явну помилку, а не тиху втрату | Storage quota/restore contract |

Це критерії до **вибору**, а не твердження, що наведений стек уже їх виконує. Кількісні SLO, supported OS floor, моделі загроз і rollout order мають бути зафіксовані до порівняння release builds.

## 4. Чому не закрити стек однією бібліотекою

- `LoroMap`/Automerge можуть детерміновано показати «winner», але це не квитанція authority й не продуктове правило «перша прийнята синхронізація». Status transition і бронювання лишаються доменними командами; CRDT застосовується лише до придатних даних ([Loro docs](https://www.loro.dev/docs/tutorial/tree), [Automerge conflicts](https://automerge.org/docs/reference/documents/conflicts/)).
- Flutter підтримує Windows і може повторно використати UI, але не гарантує нативні фонові задачі чи бездоганну доступність без перевірки. WinUI 3 дає Windows-native UI, але вводить C#/XAML renderer і окремий C#↔Rust binding; офіційний UniFFI не обіцяє готового C# target. Tauri дає DOM/IPC, але ризик непривілейованого контенту, WebView2 і другого UI-стека потребує реального тесту. Зокрема Tauri за замовчуванням дозволяє зареєстровані `invoke_handler` app-команди всім webview: для VIDA їх треба явно обмежити через `AppManifest::commands` і capability allowlist; самого CSP або обмеження plugin permissions недостатньо ([Flutter platforms](https://docs.flutter.dev/reference/supported-platforms), [WinUI 3](https://learn.microsoft.com/en-us/windows/apps/get-started/winui-get-started-overview), [UniFFI](https://mozilla.github.io/uniffi-rs/next/), [Tauri capabilities](https://v2.tauri.app/security/capabilities/), [Tauri runtime authority](https://v2.tauri.app/security/runtime-authority/)).
- Iroh вирішує транспорт, не retention чи business acceptance; relay не є authority. `iroh-docs` не підміняє VIDA signed log ([Iroh research](../../_bmad-output/planning-artifacts/research/technical-iroh-ecosystem-and-project-patterns-for-2026-09-18/research.md)).
- [Перевірка залежностей 36 Iroh-прикладів і чотирьох upstream бібліотек](../../_bmad-output/planning-artifacts/research/technical-iroh-project-library-inventory-for-vida-2026-09-23/research.md) уточнює кандидати для prototype gates. Вона не відсіює `<1.0` за номером версії; development momentum, breadth, issues, конкретні upstream caveats та conformance мають оцінюватися разом. `iroh-blobs 0.103` має окремий upstream production-quality warning, тому його не можна обрати тільки за поширеністю в інших проєктах. [UniClipboard/Engine](../../_bmad-output/planning-artifacts/research/technical-iroh-project-library-inventory-for-vida-2026-09-23/digests/uniclipboard-engine-r6-luna.md) — корисний Rust host/core/lifecycle reference, але не доведений VIDA SyncLog provider; перед reuse потрібні Iroh 1.2/fork, Flutter/OS, file-at-rest і license gates.
- Наявність Web client не усуває потреби тестувати кожну встановлювану ОС окремо: offline persistence, kill/restart, secure storage, background і reconnect відрізняються; Web також має окремі browser-origin, storage, key і lifecycle gates за ADR-0021.
- Історичні згадки Dioxus та .NET у raw `research/` не були чинним вибором. Нову пропозицію WinUI 3 + C# розглянуто окремо в [Windows-аналізі](windows-client-options.md). Висновок Flutter/Rust/Tauri з [попереднього UI research](../../_bmad-output/planning-artifacts/research/technical-vida-ui-platform-and-rust-binding-strate-2026-09-19/research.md) — історична гіпотеза; він не включав WinUI 3 і ще припускав browser client, тому не є Windows ADR.
- Безпекові gates звірено з [OWASP Authorization](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) та [MASVS](https://mas.owasp.org/MASVS/): поточні права, локальне сховище/ключі, мережа, платформа, приватність. OWASP не обирає UI toolkit або CRDT за VIDA.

## 5. Питання для наступного рішення

`ADR-0020` обрав Flutter Windows для Release 1, тому UI-toolkit bake-off більше не є release decision. Vertical slice має довести Windows packaging, FFI, accessibility і desktop workflows. WinUI 3 + C# можна оцінювати після Release 1 лише проти вимірюваної проблеми Flutter Windows; Tauri не входить до затвердженого шляху. CRDT/editor pair окремо проходить [research](../../_bmad-output/planning-artifacts/research/technical-vida-crdt-editor-stack-2026-09-22/research.md) і однаковий conformance gate; жоден кандидат ще не затверджений.
