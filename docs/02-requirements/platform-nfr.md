---
id: NFR-PLATFORM-001
status: approved
last_updated: 2026-09-25
requirement_refs:
  - transport-sync-requirements.md
  - access-control-requirements.md
  - native-client-requirements.md
decision_refs:
  - ../03-architecture/decisions/ADR-0003-offline-revocation.md
  - ../03-architecture/decisions/ADR-0005-iroh-transport-foundation.md
  - ../03-architecture/decisions/ADR-0006-durable-delivery-and-operation-envelope.md
  - ../03-architecture/decisions/ADR-0014-native-only-vida-clients.md
  - ../03-architecture/decisions/ADR-0020-flutter-windows-in-release-1.md
  - ../03-architecture/decisions/ADR-0021-static-web-client-in-release-1.md
---

# Platform NFR: transport and replicated state

## Reliability and durability

- `NFR-REL-001`: after any injected crash boundary in `log → outbox → stored → applied`, every operation acknowledged as `delivery.accepted` `MUST` remain recoverable or produce an explicit integrity failure.
- `NFR-REL-002`: duplicate delivery over direct and mailbox paths `MUST` result in exactly one domain apply.
- `NFR-REL-003`: reconnect after missed transient events `MUST` converge through pull/reconcile.
- `NFR-REL-004`: delivery topology `MUST` pass node-loss and restore drills before an availability SLO is approved.

## Security and privacy

- `NFR-SEC-001`: relay and durable-delivery operators `MUST NOT` read E2E payloads.
- `NFR-SEC-002`: secrets, recovery material and hidden cross-context links `MUST NOT` appear in logs, metrics or public projections.
- `NFR-SEC-003`: leaked/expired capability tickets, removed devices and stale epochs `MUST` have explicit negative conformance tests.
- `NFR-SEC-004`: anonymous endpoint profiles `MUST` pass metadata-correlation review against persistent profiles.
- `NFR-SEC-005`: effective revocation received for a shared Space, or expiry of 7 days without authority-confirmed rights reconciliation, `MUST` deny further managed UI, search, export, runtime/API and plugin/automation reads on every supported client for **всі shared-Space roles, включно з Owner** до чинного нового proof. Negative conformance tests `MUST` cover stale control-head replay, restart/clock rollback masquerading as rights reconciliation, read bypass after received revocation or interval expiry, and crash/restart without network (older persisted head cannot reopen access). Personal Space Owner is separate. No class- or role-dependent shared offline-read timeout is specified; proof/anti-rollback remains `OQ-0053`.

## Compatibility and migration

- `NFR-COMP-001`: conformance matrix `MUST` test independently Iroh core, adapter version, VIDA ALPN major, schema/features, persisted state and FFI artifact.
- `NFR-COMP-002`: every release that changes persisted or wire state `MUST` test mixed-version rolling upgrade, atomic activation, stale-downgrade rejection and backup restore before release. Post-activation package/schema rollback or runtime fallback is not a supported recovery flow; an observed defect fails release conformance.
- `NFR-COMP-003`: unknown mandatory protocol features `MUST` fail explicitly; they `MUST NOT` be silently ignored.
- `NFR-COMP-004`: native host-binary updates and VIDA `AppPackage` updates `MUST` be separate lifecycles. A package update `MUST` declare host/runtime compatibility and migration requirements; it `MUST NOT` silently install executable behavior forbidden by the target platform or store policy.

## Platform behavior

- `NFR-PLAT-001`: each supported installed mobile or desktop OS client і Release-1 static Web client `MUST` publish separate conformance results. Browser `MUST NOT` успадковувати mobile/desktop guarantees без browser-specific доказів; майбутній paid Hosted Space потребує окремого профілю.
- `NFR-PLAT-002`: mobile tests `MUST` cover network switch, suspend, process death, key-store restore and reconnect.
- `NFR-PLAT-003`: Messenger, Knowledge/Notes and Projects/Tasks `MUST` each pass offline local-read, permitted local-action, pending-state, process-restart and reconnect tests on every supported client OS. Network delivery or remote authority acceptance `MUST NOT` be presented as completed while pending.
- `NFR-PLAT-004`: each supported client `MUST` test uniform offline-read availability before seven-day expiry for `standard`, `protected` and `critical` (including already-open content) and for shared Owner as well as other roles; at expiry without fresh rights proof all shared-Space read surfaces `MUST` lock, while Personal Space remains governed separately. Tests `MUST` cover immediate received revocation, restart, clock rollback, old-head replay and unlock only after valid reconciliation. Cross-platform proof design remains `OQ-0053`.
- `NFR-PLAT-005`: offline mutation candidates `MUST` survive process death, backup/restore and long disconnected periods without age-only rejection or silent loss; reconnect tests `MUST` distinguish a still-authorized candidate, a revoked actor and a stale grant/epoch. Production rebase/re-sign and conflict fixtures await `OQ-0033`/`OQ-0034`.
- `NFR-PLAT-006`: Windows release build `MUST` pass keyboard-only journeys, system menu/tray actions, screen-reader inspection and equivalent Messenger, Notes/Knowledge and Projects/Tasks workflows. `ADR-0020` selects Flutter Windows for Release 1; framework capability alone is not acceptance evidence.
- `NFR-PLAT-007`: після семиденного shared-read lock встановлювані клієнти `MUST` дозволяти створити новий локальний draft без читання заблокованих даних; він переживає restart, не стає спільним до rights reconciliation і не обходить чинну заборону. Conflict tests `MUST` перевіряти перший acceptance лише за спільного перевірного порядку, рівноправність пристроїв, однаковий явний невирішений status/file conflict без winner після reconciliation непорівнюваних гілок, збереження variants, нову resolution operation на основі обох гілок, повторну перевірку прав і незалежне збереження неконфліктних текстових правок. Два несумісні авторизовані офлайн-рішення того самого conflict з непорівнюваними прийняттями `MUST` дати новий conflict; доведено еквівалентні рішення — один видимий результат із двома audit records і об'єднаними causal heads; пізня прийнята несумісна гілка з непорівнюваним щодо рішення acceptance — відновлений conflict, а за порівнюваного acceptance діє правило першого прийняття. Тести також перевіряють заборону resolution без read до всіх variants і дві окремі дії revision/copy; точна wire/merge модель лишається `OQ-0033`/`OQ-0034`.
- `NFR-PLAT-008`: для irreversible effect, agent auto-resolution і derived resource mixed-version scenario клієнти `MUST` перевіряти відповідно domain confirmation перед effect, явну згоду для відповідного типу дії/поточні права та одну логічну похідну сутність за pinned rule version. Proof/executor/ID contract лишаються `OQ-0033`/`OQ-0045`/`OQ-0048`.
- `NFR-PLAT-009`: installed clients `MUST` be continuous-sync/online-first: while the OS permits execution, the client maintains or promptly restores an authenticated VIDA/Iroh path, drains durable pending work and reports degradation explicitly. A mobile client `MUST NOT` claim guaranteed 100% background reachability because platform suspension, force-stop, power policy and network loss remain outside VIDA control.
- `NFR-PLAT-010`: Android conformance `MUST` cover foreground, process/background restriction, an optional per-device user-visible high-availability mode configurable during onboarding and later settings, FCM wake, WorkManager reconciliation, force-stop and Android 15 foreground-service limits. iOS conformance `MUST` cover foreground connection, suspension, APNs/background push, scheduled background task, force-quit and foreground reconciliation; suspended/push-reachable iOS is not counted online without an active controlled VIDA connection. A floating/overlay window `MUST NOT` be required for baseline sync or treated as a portable liveness proof.
- `NFR-PLAT-011`: network-change, wake, foreground and explicit retry `MUST` trigger prompt path re-evaluation; Iroh direct/relay migration precedes application reconnect. A true disconnect uses bounded exponential retry with jitter/cap; a new network or proven connection resets backoff. Exact timings are profile-test parameters under `OQ-0067`. Retry `MUST NOT` duplicate a business operation or discard the durable outbox.
- `NFR-PLAT-012`: mobile release `MUST` pass current Apple App Review and Google Play background/resource policies. iOS `MUST NOT` depend on permanent arbitrary background execution; Android foreground services `MUST` declare a supported type, be user-visible where required and pass Play declaration/review. Policy is re-checked before each store submission.
- `NFR-PLAT-013`: release evidence `MUST` report per-platform delivered download/install size, cold/warm startup, memory tiers, battery/background activity, wake locks, crash/ANR and local-data growth on representative devices. Store maxima are compliance ceilings, not product targets.
- `NFR-PLAT-014`: numeric VIDA budgets `MUST` be approved only after a representative Messenger+Notes+Project+Files+Calls vertical slice measured under `SPEC-PLATFORM-RESOURCE-CONFORMANCE-001`. CI `MUST` then reject regressions over the approved budgets; until then the values remain `OQ-0076`, not implied guarantees.
- `NFR-PLAT-015`: contact connectors `MUST` pass privacy/permission tests for full, limited and denied access; provider token expiry/full reconcile; concurrent external edit; field-level export; Persona isolation; and absence of private/anonymous VIDA bindings from platform logs, metrics and default exports.
- `NFR-PLAT-016`: Release-1 Web `MUST` pass browser-specific tests for static hosting, Rust/Wasm boundary, authenticated peer enrollment, browser-storage durability/quota/eviction, key custody/recovery, origin/XSS isolation, direct-path proof, encrypted relay fallback, route migration, both-path outage/reconnect, offline pending work, accessibility and all applicable Core-App/call flows. Browser background execution `MUST NOT` be inferred from native clients; storage eviction or path loss `MUST NOT` be reported as synchronized or delivered.

## Operations and observability

- `NFR-OPS-001`: telemetry `MUST` distinguish transport, lookup, authorization, validation, durability and apply failures.
- `NFR-OPS-002`: metrics `MUST` expose queue age, retry count, reconciliation lag, blob verification failures and relay/direct path selection without payload or secret identifiers.
- `NFR-OPS-003`: Address Lookup tests `MUST` cover outage, stale/rollback record, provider compromise and relay migration.

## Deferred quantitative SLOs

Latency, availability, queue-retention, storage, startup, delivered package size, memory, battery and relay-egress targets remain `Deferred` until representative installed-client/multi-node prototypes provide baselines. No implementation may advertise an unstated target as guaranteed.

### Приклади baseline-кейсів до встановлення числових budgets

Це test scenarios, а не вже затверджені thresholds:

- **Install size:** фактичний per-device download/install для Android ARM64, iPhone variant і Windows x64 із bundled Messenger, Notes, Project, Contacts та calls dependencies.
- **Cold start:** запуск після reboot/process death на mid-tier device з порожнім профілем і з великим локальним Space; окремо час до shell і до першого usable view.
- **Memory:** idle shell; selected-Space search; великий rich-text document; board із великою кількістю задач; 1:1 і group video call.
- **Battery/network:** 8 годин background із push/reconnect; network switching; активна година messaging/sync; audio/video call; Android optional high-availability mode окремо від baseline.
- **Storage growth:** текстові ресурси, thumbnails, downloaded files і незавантажені blobs; low-disk failure; browser quota/eviction; cleanup без втрати останньої відновлюваної копії.
- **Reliability:** crash/ANR, wake-lock, failed background task, process kill during local commit/sync і recovery після restart.

Після vertical slice для кожного кейсу треба зафіксувати device/profile, dataset, percentile, допустимий budget і CI/store evidence; «менше store maximum» саме по собі не є достатнім NFR.
