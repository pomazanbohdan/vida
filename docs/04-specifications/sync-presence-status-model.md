---
id: SPEC-SYNC-PRESENCE-STATUS-DRAFT
status: draft
last_updated: 2026-09-22
requirement_refs:
  - ../02-requirements/transport-sync-requirements.md
  - ../02-requirements/effect-execution-constraints.md
  - ../02-requirements/identity-requirements.md
  - ../02-requirements/platform-nfr.md
decision_refs:
  - ../03-architecture/decisions/ADR-0016-equal-device-peers.md
  - ../../_bmad-output/planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md
---

# Чернетка: позначки зв'язку, синхронізації та присутності профілю

Користувач попросив показувати «онлайн», «синхронізація», «завершено», «офлайн (один пристрій)». Базова парадигма — **continuous-sync/online-first**: встановлений клієнт підтримує або якнайшвидше відновлює VIDA/Iroh-з'єднання, а неможливість синхронізації є деградованим винятком. Це продуктова ціль, не неправдива гарантія 100% background reachability на mobile. Кількість онлайн-пристроїв належить **одному видимому профілю/Persona**, а не сумі пристроїв Space; її бачать усі, хто бачить цей профіль. Це правило однаково діє для анонімного профілю: його число не агрегується з приватною, публічною чи корпоративною Persona тієї самої людини. Пристрій «онлайн», коли він має свіжо підтверджену можливість синхронізуватися з іншим авторизованим VIDA endpoint, напряму або через робочий relay-шлях. Одна позначка не може чесно означати одночасно зв'язок, реплікацію, прийняття бізнес-зміни й виконання зовнішнього ефекту.

| Вимір | Що можна показати | Чого позначка не доводить |
|---|---|---|
| Зв'язок поточного пристрою | доступний шлях sync / зараз немає доступного шляху | чинність прав, прийняття операції або доступність усіх peers |
| Прогрес реплікації | очікує, синхронізується, наздогнав **відомі** frontiers | відсутність ще невідомої офлайн-гілки |
| Стан конкретної зміни | збережено локально, синхронізується, **синхронізовано**, конфлікт; delivery, read і domain acceptance обліковуються окремо | Iroh/QUIC ACK, mailbox storage, recipient delivery і `authority.accepted` не тотожні |
| Зовнішній наслідок | очікує підтвердження, виконується, підтверджено виконання або помилка | `Done` задачі не доводить, що лист уже надіслано |
| Presence профілю | число пристроїв **цього профілю**, які зараз здатні синхронізуватися, для всіх глядачів видимого профілю; актуальність потребує позначки | точну глобальну кількість під час partition або зв'язок різних Persona/профілів однієї людини |

Приклад: телефон офлайн змінює статус задачі. UI може одночасно показувати «немає шляху синхронізації» й «збережено локально»; після робочого Iroh-з'єднання — «синхронізація». Напис «синхронізовано» з'являється після signed VIDA application-level receipt від першої незалежної authorized durable application replica, яка validate/apply визначений frontier; сам Iroh/QUIC ACK або mailbox storage недостатній. Single-device Persona без replica лишається «збережено локально». Receipt не створює другого approval vote й не замінює Space authority acceptance. Канонічна семантика — [SPEC-OPERATION-FINALITY-001](operation-finality-contract.md).

## Iroh і рекомендований proof

- Iroh `Endpoint::online()` є bootstrap/convenience-сигналом, що endpoint принаймні один раз дійшов до relay handshake; після першого успіху він не є freshness lease конкретного peer-а і сам по собі не доводить поточну прикладну досяжність. Для single-device Persona точний public-online proof ще не визначений і не може бути вигаданий із самого `Endpoint::online()`.
- Лише `finish()` + `stopped().await == None` для Iroh `SendStream` доводить QUIC receipt усіх байтів peer-ом, але не VIDA validation, durable commit або apply; завершення звичайного `write`/`write_all` не доводить навіть remote receipt. Для `online` потрібен свіжий автентифікований VIDA sync-capability handshake/lease. Application receipt для `синхронізовано` bind-ить issuer, Space/control epoch, operation/frontier, policy version, durable validation/apply, result digest і signature за `SPEC-OPERATION-FINALITY-001`.
- Два endpoint однієї Persona після такого receipt доводять дві durable-копії. Це **replication acknowledgement**, не business quorum shared Space; authority proof визначає policy операції/ресурсу, а не кількість пристроїв одного користувача.

## Platform availability profile

- Desktop client підтримує довгоживуче з'єднання, коли процес або дозволений background component працює; reconnect і durable reconcile обов'язкові після sleep/network switch.
- Android має добровільний **per-device high-availability mode**. Його пропонують під час первинного налаштування і дозволяють змінити пізніше; відмова не вимикає VIDA. Для явно активованого й consented Public profile звичайний режим може використовувати FCM wake hint, reconnect і WorkManager reconcile; Autonomous anonymous Persona не реєструє зовнішній push/wake binding (`REQ-ID-018`). High-availability може використовувати user-visible foreground service лише для дозволеного сценарію; Android 15 обмежує `dataSync` foreground service сумарно шістьма годинами на 24 години для застосунків із target API 35.
- iOS зазвичай призупиняє background app. Для consented Public profile APNs/background push і BackgroundTasks дають обмежене, кероване ОС вікно роботи; анонімний автономний profile не реєструє APNs binding (`REQ-ID-018`). Клієнт відновлює sync у дозволене вікно та завжди при foreground. **Online** для iOS означає лише active VIDA client із чинним application-level connection proof; suspended або лише APNs-reachable device у count не входить.
- При network change, wake або foreground shell негайно сповіщає `VidaNodeHost`; Iroh спершу намагається зберегти або перенести direct/relay path. Якщо application connection справді втрачено, клієнт переходить у `reconnecting`, робить швидку першу спробу, далі bounded exponential backoff із jitter і cap; новий network-change/foreground/user retry скидає backoff. Outbox/reconcile забезпечують цілісність незалежно від retry timing.
- `Reconnecting` є приватним технічним станом, який бачить лише власник відповідного пристрою. Щойно чинний online proof відсутній, пристрій не входить у публічний count; іншим глядачам не показується внутрішня спроба перепідключення.
- Floating/overlay window не входить у baseline: Android overlay не скасовує загальні FGS/policy обмеження, а iOS не має еквівалентного універсального механізму. Це може бути окрема platform-specific UX capability, але не умова коректності sync.
- Публічний count включає поточний user-controlled device, якщо він має чинний proof. Bots, `ServicePrincipal` та майбутній AppCenter не збільшують число «пристроїв користувача» і показуються окремими типами присутності.
- Для Persona з одним пристроєм profile може бути «онлайн» лише за чинного application-level sync-capability proof; точний single-device proof ще відкритий. Нова operation однаково лишається «збережено локально», доки незалежна authorized durable application replica не видасть Application Receipt.

## Відкриті рішення

- Яку точну freshness/expiry policy має VIDA sync-capability proof і коли owner-visible `reconnecting` стає `unknown`, визначають вимірювані Android/iOS/Windows/Web prototype profiles; довільне універсальне число наперед не фіксується. Web browser/tab visibility або relay connection самі по собі не є доказом online. Public online count не містить пристрій без чинного controlled-connection proof.
- Яка update cadence/coalescing зберігає погоджений exact count, але зменшує timing-correlation oracle? Анонімний профіль успадковує загальну видимість свого профілю, але ніколи не агрегується з іншими Personas.
- Як уникнути витоку через зміну лічильника після відкликання прав або між контекстами? Поточний user-controlled device із чинним proof уже включено, а service principals показуються окремо за AD-26.

Перевірено 2026-09-21: актуальний [Iroh 1.2 `Endpoint::online`](https://docs.rs/iroh/latest/iroh/endpoint/struct.Endpoint.html#method.online) є transport bootstrap/convenience signal, а не peer-specific application freshness proof; [Iroh `SendStream::stopped`](https://docs.rs/iroh/latest/iroh/endpoint/struct.SendStream.html#method.stopped) прямо відрізняє receipt усіх stream bytes від processing і радить application-level response. [Matrix Client-Server API v1.19](https://spec.matrix.org/v1.19/client-server-api/) розрізняє presence, device-list updates і room events як типи даних; це референс розділення **семантики**, не обраний протокол VIDA. [OWASP Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) вимагає перевіряти доступ до даних на кожному запиті, а [MASVS-PRIVACY-1](https://mas.owasp.org/MASVS/controls/MASVS-PRIVACY-1/) — мінімізувати доступ до чутливих даних. Для VIDA потрібні негативні тести, що raw device list не розкривається без окремого права, а лічильник не пов'язує анонімну й корпоративну Persona. OWASP не встановлює heartbeat TTL.

Exact count є свідомо погодженим profile metadata для всіх глядачів видимого профілю, але не дозволом розкрити device IDs, endpoint addresses або зв'язок між Personas. Conformance `MUST` перевіряти count-differencing/timing correlation, replayed або revoked lease, enumeration/rate abuse і те, що один proof не оновлює лічильник іншої Persona. Залишковий timing fingerprint документується як privacy trade-off; exact coalescing/update cadence лишається `OQ-0064`.

Platform reality verified 2026-09-21: [Android foreground services](https://developer.android.com/develop/background-work/services/fgs) require user-noticeable work and a notification; [Android 15](https://developer.android.com/about/versions/15/behavior-changes-15) limits `dataSync` foreground-service runtime; [FCM priority](https://firebase.google.com/docs/cloud-messaging/android/message-priority) provides limited wake time and may be deprioritized; [Apple background strategies](https://developer.apple.com/documentation/backgroundtasks/choosing-background-strategies-for-your-app) are system-scheduled/bounded, and [Apple background execution modes](https://developer.apple.com/documentation/xcode/configuring-background-execution-modes) state that background apps are typically suspended. These sources support online-first recovery, not a 100% mobile-online claim.

Reconnect reference check 2026-09-21: Iroh [connection healing](https://www.iroh.computer/blog/healing-connections) and [network-change improvements](https://www.iroh.computer/blog/iroh-0-98-0-getting-back-to-traversing-nats) demonstrate path migration, relay fallback, network monitoring and bounded transport recovery; exact historical intervals are not VIDA guarantees. Android [`registerDefaultNetworkCallback`](https://developer.android.com/reference/android/net/ConnectivityManager) supplies default-network changes. [gRPC connection backoff](https://github.com/grpc/grpc/blob/master/doc/connection-backoff.md) supports exponential backoff, jitter, cap and reset after a working connection. Android [`VpnService`](https://developer.android.com/reference/android/net/VpnService) always-on and Apple [VPN On Demand](https://developer.apple.com/documentation/networkextension/vpn-on-demand-rules) are privileged VPN facilities; VIDA may copy their UX ideas but cannot claim their lifecycle privileges as an ordinary app.
