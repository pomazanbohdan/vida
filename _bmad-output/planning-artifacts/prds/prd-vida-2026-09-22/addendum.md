---
title: "VIDA Release 1 — Technical and Decision Addendum"
status: final
created: 2026-09-22
updated: 2026-09-25
---

# VIDA Release 1 — Technical and Decision Addendum

Цей файл зберігає погоджені технічні рішення й невирішені механізми, які обмежують PRD, але не є продуктовими вимогами.

## A. Platform baseline

- Спільний VIDA Core: Rust.
- Release 1 UI: Flutter для Android, iOS, Windows і статичного Web. Web потребує Rust/Wasm boundary до спільної Core-семантики; native FFI не переноситься в браузер механічно.
- Native WinUI 3: після Release 1.
- Tauri не є Release 1 client.
- Web у Release 1 є local-first авторизованим peer із browser storage та direct-first sync з VIDA-operated Iroh relay fallback; може розміщуватися на статичному host. Paid Hosted Space/server runtime, mailbox і server-rendered private web client залишаються окремим post-v1 контуром. Stock Web Iroh/Wasm на перевірці 2026-09-25 не має direct path, тому browser-compatible WebRTC/custom transport потребує окремого prototype/conformance proof.
- Rust↔Flutter boundary має окремо зафіксувати ABI/codegen, ownership, threading, cancellation, error mapping і compatibility window.

## B. Transport і sync

- Затверджений [ADR-0005](../../../../docs/03-architecture/decisions/ADR-0005-iroh-transport-foundation.md) обирає **Iroh core 1.2** як primary transport foundation. Exact patch/build і транзитивні версії/checksums фіксує release lockfile/SBOM після compatibility proof; перехід на іншу core series потребує зміни ADR. Iroh не замінює VIDA operation, authorization, merge і product semantics.
- Devices однієї Persona рівнозначні; device priority за платформою або роллю заборонено.
- Усі peer-data profiles пробують прямий шлях першими; VIDA-operated protocol-compatible encrypted relay є fallback, не бізнесовим authority чи durable mailbox. Native Iroh має окремий LAN/known-address profile без relay. Web direct path потребує окремо доведеного browser-compatible adapter, бо stock Iroh/Wasm transport є relay-only; коли обидва шляхи недоступні, remote operations лишаються pending.
- Device clocks використовуються для UX metadata, але не є єдиним arbiter причинності або finality.
- Першою прийнятою є operation, яка пройшла protocol validation/acceptance; пізня несумісна operation створює Conflict.
- Доставка й remote apply не повинні повторно запускати бізнес-команду.
- Потрібні versioned contracts: operation envelope, causal metadata, receipt, idempotency key, sync cursor/frontier, Conflict record і terminal outcome.
- Approved `docs/04-specifications/operation-finality-contract.md` визначає п'ять незалежних evidence axes: default sync після першої незалежної authorized durable application replica; 1:1 Delivered після першого qualifying recipient Device; group delivery `N/M` per Persona; exact Device topology не розкривається sender-у; mailbox означає лише stored for delivery. OQ-1 закрито semantic fixtures `fixtures/operation-finality-v1.yaml`.

## C. Data, merge і storage

- CRDT/reference libraries використовуються лише для collaborative documents. Loro проходить перший запуск, Automerge — обов'язковий control, Yrs — ecosystem control; остаточний engine/editor pair обирається лише за `SPEC-COLLABORATIVE-DOCUMENT-CONFORMANCE-DRAFT` і чинними F01–F14 у `crdt-editor-v1.yaml`.
- Automatic merge застосовується лише до сумісних changes.
- Conflict variants не видаляються лише через локальне resolution.
- Files/blobs мають content identity, availability state, configurable auto-download threshold і safe-eviction rule.
- Garbage collection не може видаляти останню відновлювану копію.
- Schema evolution підтримує lazy read, migration on write і background migration.
- Field rename/removal потребує versioned migration; silent data loss заборонено.

## D. Identity, keys і access

- Persona isolation є Core invariant.
- Autonomous Persona recovery контролює користувач через recovery material.
- Організаційний/federated account є окремою Persona/account context; блокування доменної адреси не блокує приватні Personas.
- Owner може призначати/видаляти Owner; Admin не може керувати Owner.
- Access revocation застосовується після sync; shared-rights revalidation window — 7 діб.
- Security requirements звіряються з актуальними OWASP MASVS/ASVS і platform guidance.

## E. AppPackage і logic runtime

- AppPackage містить schemas, forms/screens, workflows, metadata, dependencies і дозволену logic representation.
- AppInstance володіє власним schema/config namespace, але кожна операція проходить чинну Space/instance/container/Resource authorization.
- Cross-App access відбувається через оголошені versioned dependencies/contracts.
- Lifecycle hook усередині AppInstance може розширити або замінити default hook цього AppInstance; Core governance не перевизначається.
- Від одного accepted source fact пристрої мають зійтися на одному logical derived Resource зі стабільною identity. Детерміноване виведення ID чи authority-issued ID, executor і поведінка для ще не обробленого fact після оновлення логіки лишаються `OQ-0048`.
- iOS Release 1 виконує сумісні bundled і зовнішні declarative packages, які викликають вбудовані VIDA capabilities; завантажуваний код і окремий Apple 4.7 mini-app runtime потребують нового post-v1 контуру.
- Update modes: compatible-auto, security-auto, manual, pinned. `security-auto` requires verified compatible security change; publisher label alone is insufficient. Expanded capabilities, data scope or dependency access require authorized Owner/Admin confirmation; exact signed proof profile remains `OQ-0043`.
- Release-1 Marketplace/repository trust потребує publisher keys, signed metadata, anti-downgrade protection і dependency resolution; exact format/profile лишається `OQ-0043`.
- Developer є publisher role; вона не тотожна Owner або Admin конкретного Space.
- Release 1 включає вбудований керований Marketplace, external repository connection і сумісні зовнішні AppPackages за тим самим conformance contract, що й bundled Core Apps; розширені комерційні інструменти видавця не входять у baseline.

## F. Approval і effects

- Core approval modes: single approver, sequential stages, M-of-N; `all-of` є M=N.
- AppPackage задає оптимальний default для named process; Admin налаштовує конкретний process.
- Exclusive claim є generic operation family для ресурсу, де збережений request ще не дорівнює confirmed outcome.
- External effects використовують outbox/intention record, stable ID, idempotency і status verification where supported.
- Якщо outcome зовнішнього сервісу неможливо перевірити, стан лишається unknown і потребує ручної дії.

## G. Calls і background availability

- 1:1 і group audio/video calls мусять мати E2EE.
- LiveKit є first measured baseline; direct 1:1 + LiveKit — mandatory control; фінальний media profile обирається лише після чинних F01–F17 у `e2ee-calls-v1.yaml` без hard-gate failure.
- Small-team group-call contract затверджено: 8 total participants, UX optimized for 4–6; public support claim лишається залежним від physical-device proof.
- `Autonomous anonymous` Persona не реєструє external push/wake binding. Push/reconnect/background scheduling дозволені лише явно consented `Public` profile; registrations не повторно використовуються між Personas.
- iOS Device є Online лише за фактичного контрольованого application connection; push reachability не рахується Online.
- Reconnect TTL визначається platform experiments, не довільним числом у PRD.

## H. Localization і diagnostics

- 18 мов / 21 profiles: `uk`, `en`, `es-419`, `es-ES`, `pt-BR`, `pt-PT`, `hi-IN`, `id-ID`, `ar`, `de-DE`, `fr-FR`, `ja-JP`, `ko-KR`, `tr-TR`, `zh-Hans`, `zh-Hant`, `pl-PL`, `it-IT`, `ro-RO`, `cs-CZ`, `nl-NL`.
- Російська не входить у Release 1.
- Diagnostics є local-first: preview, redaction, explicit send; optional future consent не може корелювати Personas.

## I. Deferred decisions

- Exact CRDT/editor pair після cross-platform prototype; presence boundary і live-cursor gate вже визначені draft conformance spec.
- Canonical binary encoding/signature suite для approved finality receipts у межах protocol-major architecture contour.
- Rust↔Flutter native binding і Rust/Wasm↔Flutter Web boundary, browser storage/key custody, supported-browser/lifecycle profile та static-host release packaging.
- Exact E2EE media profile після чинних F01–F18 у `e2ee-calls-v1.yaml`, включно з Web browser/mixed-client fixtures: LiveKit first measured baseline; direct-1:1 + LiveKit mandatory architectural control; `iroh-live` R&D control. Native pass не доводить Web E2EE calls.
- AppPackage format, trust/update specification and schema DSL.
- Numeric performance/resource budgets after the approved G0 method executes a physical-device G1 baseline.
- Independent security review scope.
- Public-release operational ownership.
- Узгодження решти похідних UX/epic/fixture документів із ADR-0021; старе browser post-v1/free-local-only формулювання ADR-0018 superseded.

### R-2 implementation fan-out inventory

`OQ-2` і `OQ-3` є необхідними, але не єдиними передумовами масової реалізації. Перед fan-out кожний нижчий контракт має мати затверджений версійований profile, negative/golden fixtures і сумісний release tuple; наявність draft kernel не дорівнює проходженню gate. Bounded prototypes можуть тривати для отримання цих доказів.

| Контракт | Наявний артефакт | Що ще не доведено |
|---|---|---|
| Operation envelope/serialization | [BMad kernel](../../../specs/spec-vida-operation-envelope/SPEC.md), [accepted finality semantics](../../../../docs/04-specifications/operation-finality-contract.md) | `OQ-0028`: exact codec/signature bytes, public vectors, independent reader. |
| Serverless domain authority і SyncLog acceptance | [accepted SyncLog contract](../../../../docs/04-specifications/sync-log-contract.md), [BMad signed-log kernel](../../../specs/spec-vida-signed-log-engine/SPEC.md), [authority option research](../../research/technical-vida-multi-device-federation-ownership-c-2026-09-19/research.md) | `OQ-0033`/`OQ-0034`: serialized logical authority topology, equivalence/transition policy, current/frontier and snapshot/pruning semantics, shared convergence fixtures. `SPEC-OPERATION-FINALITY-001` не закриває ці питання. |
| Local storage, recovery і safe GC | [BMad storage kernel](../../../specs/spec-vida-storage-provider/SPEC.md), [blob contract](../../../../docs/04-specifications/blob-store-contract.md) | `OQ-0036`/`OQ-0034`: provider tuple, fault fixtures та retention frontier. |
| Collaborative document/presence | [F01–F14 contract](../../../../docs/04-specifications/crdt-editor-conformance.md) | `OQ-2`/`OQ-0073`: pair selection і reproducible cross-platform evidence. |
| Rust↔Flutter boundary | [BMad binding kernel](../../../specs/spec-vida-platform-bindings/SPEC.md) | `OQ-0035`/`OQ-0037`: ABI/codegen, ownership і mixed-version contract. |
| E2EE media/signaling/keying | [F01–F18 contract](../../../../docs/04-specifications/e2ee-calls-conformance.md) | `OQ-3`/`OQ-0072`: measured Android/iOS/Windows/Web profile, browser lifecycle/key/permission and mixed-client fixtures, answer authority і security proof. |
| Bundled/external AppPackage і Marketplace compatibility | [runtime baseline](../../../../docs/04-specifications/app-package-runtime-baseline.md), [approved requirements](../../../../docs/02-requirements/app-package-requirements.md), [BMad package kernel](../../../specs/spec-vida-app-packages/SPEC.md) | До fan-out: `OQ-0039`/`OQ-0040`/`OQ-0070` для activation authority, schema/dependency/host compatibility, migration й authorization fixtures. Для зовнішніх repositories у Release 1: `OQ-0043` exact manifest/trust/freshness/anti-downgrade profile та conformance; `security-auto` product rule затверджено, але його signed eligibility proof і позитивні/негативні fixtures лишаються release gate. |

## J. Official comparator evidence

- Signal: mandatory E2EE messaging/calls and privacy-first positioning — <https://signal.org/>.
- Delta Chat: decentralized multi-profile messenger and independent multi-device clients — <https://delta.chat/en/help>.
- webxdc: sandboxed, self-contained Apps distributed through chats — <https://delta.chat/en/2022-06-14-webxdcintro>.
- Delta Chat + Iroh: realtime P2P channels complement durable host-message updates — <https://delta.chat/en/2024-11-20-webxdc-realtime>.
- Notion: connected workspace and native offline limitations — <https://www.notion.com/product/features>, <https://www.notion.com/en-gb/help/use-pages-offline>.
- Linear: opinionated project system whose offline mode is a failsafe — <https://linear.app/docs/conceptual-model>, <https://linear.app/docs/get-the-app>.
- Weixin/WeChat Mini Programs: centralized chat-led super-app ecosystem — <https://www.tencent.com/products/weixin-mini-programs/>.
- Iroh: encrypted QUIC connectivity, NAT traversal and relay fallback; it is a protocol toolkit, not VIDA product semantics — <https://www.iroh.computer/>.

**Positioning inference:** VIDA's credible wedge is the shared typed Resource graph plus first-class Knowledge/Projects under one Space/access/sync model. Claims such as «fully serverless», «zero metadata», «exactly once» or «first decentralized super-app» are prohibited without separate proof.

## K. 2026-09-24 product-scope reconciliation

The user's later decisions supersede the older PRD/PRFAQ calendar non-goal: Release 1 includes Personal and Shared Space Calendar Events, time zones, one-off and simple daily/weekly recurrence, reminders, invitations and RSVP. City Portal/booking remain out. Calendar is mandatory Core capability, not a dated Task or a Project dependency; its visible Space menu entry can be hidden. Only an existing VIDA Persona can be invited. An external invitee sees title, time, time zone and place; description, files and linked tasks require explicit scoped sharing. Organizer and Space Owner/Admin may reschedule; an ordinary invitee may only propose another time, and a changed time requires new RSVP.

ContactCard and Space membership are separate. A Space member appears in membership automatically, but a ContactCard is optional and is not a CRM client record. Work-Space cards and other shared data are team-visible under current App/Project access rules; Shared Space has no per-author private Notes by default. App-defined per-user/manager visibility for special resources remains possible through Core ACL. Explicit cross-Space copy creates a separate card with field preview and no live linkage. Release 1 includes user-selected system-contact import into Personal Space; `.vcf` import/export, `.ics` file exchange, provider export/two-way contact sync and external calendar sync are deferred.

The selected-Space-only Search decision supersedes older PRD/product/native-client global-search wording. FR-21, `REQ-CLIENT-022` and the product bundle are reconciled; cross-Space aggregate Search is not a Release-1 capability. This dated decision overrides historical PRFAQ language without rewriting the completed PRFAQ.
