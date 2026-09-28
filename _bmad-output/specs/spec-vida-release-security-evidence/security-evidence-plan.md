# Release security evidence plan — not executed

Це похідний від [SPEC](SPEC.md) план доказів, не результат оцінювання й не нова політика безпеки. Для кожної release-версії треба зафіксувати активні компоненти й довірчі межі; неактивний майбутній Hosted Space або browser flow не позначати `pass` за результатами native-клієнта. [Privacy evidence](../../../docs/04-specifications/privacy-release-evidence.md) окремо веде `REQ-PRIV-001–009` і legal launch blockers.

## Мінімальна межа threat model

| Boundary | Активи й запитання до моделі | Затверджена основа / gap |
|---|---|---|
| Persona ↔ Device ↔ Space authority | Ключі, `DeviceGrant`, recovery, membership, revocation; чи може втрачений Device, старий grant або чужий вузол видати новий доступ? | [AD-37 та gates](../../planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md); recovery `OQ-0024/0049`, rights proof `OQ-0053` ще відкриті. |
| UI/OS ↔ Rust facade ↔ Core | DTO, помилки, callbacks, lifecycle, platform secure store; чи може shell/plugin обійти Core-авторизацію або видати stale result за актуальний? | [Binding draft](../spec-vida-platform-bindings/SPEC.md); production bridge `OQ-0035` відкритий. |
| Local storage ↔ backup/export | Payload, pending work, snapshots, logs, recovery bundle; де лишається plaintext після crash, GC, restore чи відкликання? | `NFR-SEC-002/005`, [storage candidate](../spec-vida-storage-provider/SPEC.md), [MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/). |
| Device ↔ Iroh direct/relay/mailbox ↔ peer | E2EE payload, metadata, tickets, replay, retry/ACK; що бачить посередник і чи повтор змінює бізнес-стан вдруге? | `NFR-SEC-001/003/004`, [operation-finality contract](../../../docs/04-specifications/operation-finality-contract.md); authority `OQ-0033` відкритий. |
| AppPackage/repository ↔ AppInstance ↔ інший App | Підпис, оновлення, sandbox, dependency grant; чи пакет отримує чужі дані або виконує новий код без затвердженої межі? | [App package draft](../spec-vida-app-packages/SPEC.md), `NFR-COMP-004`; trust/update gates відкриті. |
| Messages/files/calls ↔ recipients | Вміст, групові epochs, файлові bytes, media keys і signaling; чи обходить інший шлях E2EE, право або revocation? | [Calls draft](../../../docs/04-specifications/e2ee-calls-conformance.md), [BlobStore](../../../docs/04-specifications/blob-store-contract.md); випробування ще не доводять production. |
| Optional node/hosted/public API | Account, relay, replica, mailbox, decrypt authority, storage/retention; які функції реально активні та хто бачить plaintext? | [OQ-0077](../../../docs/00-governance/open-questions.md) затвердив zero-knowledge default; лише явне рішення Owner для конкретного Space вмикає managed replica з ключами, а terminal authority делегується окремо. `OQ-0026` і hosted lifecycle лишаються відкритими; [privacy gate](../../../docs/04-specifications/privacy-release-evidence.md) окремий. |

Для кожного рядка зберігати: версійну схему потоків, припущення, актуальні asset/actor/entry point, загрози, обраний контроль, негативний тест, результат і residual-risk disposition. Це застосування [OWASP Threat Modeling](https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html), перевіреного 2026-09-23; OWASP не обирає VIDA crypto або release verdict.

## Evidence gate: що саме потрібно показати

| ID | Область доказу | Мінімальний артефакт і негативний кейс | Статус зараз |
|---|---|---|---|
| SEC-G01 | Threat model coverage | Versioned flow/trust-boundary model, загрози, припущення, mitigations, residual risks; позначити активні й неактивні flows. | planned; не виконано |
| SEC-G02 | Identity, authorization, recovery | Release-build результати lost-device/replay/stale-grant, current-rights і cross-Persona isolation; recovery та rights-proof рішення/gates показати окремо. | planned; `OQ-0024/0049/0053` відкриті |
| SEC-G03 | Local secrets and data | Android/iOS/Windows disk, backup, log, notification, error-path inspection для ключів, pending data, conflict variants і plaintext; перевірка після restart/restore. | planned; платформи не тестовані |
| SEC-G04 | Transport, lookup, sync, authority | Direct/relay/mailbox duplicate, replay, ticket expiry, metadata-correlation, revoked Device, old epoch та незалежність transport ACK від domain acceptance; Address Lookup outage, stale/rollback record, provider compromise і relay migration за `NFR-OPS-003`. | planned; частина semantic fixtures описана, не виконана |
| SEC-G05 | App supply chain and extension | Provenance/signature/anti-rollback, hostile package, unauthorized cross-App read, unknown mandatory feature, mixed-version activation. | planned; exact trust/profile відкритий |
| SEC-G06 | Communication and blobs | E2EE message/call/file-path перевірки, unauthorized fetch, media/key epoch change, loss/reconnect; не стверджувати E2EE calls за одним signaling test. | planned; calls/blob gates відкриті |
| SEC-G07 | FFI and platform release | Installed release build на кожній ОС: stale handle, error leakage, app kill/resume, OS permission denial, storage and background-policy checks. | planned; bridge/OS fixtures не виконані |
| SEC-G08 | Privacy/legal | Посилання на окрему [privacy release evidence matrix](../../../docs/04-specifications/privacy-release-evidence.md) для кожного активного processing flow. | draft; legal owner і докази відсутні |
| SEC-G09 | Independent review | Якщо `OQ-0081` затвердить: scope, незалежність, звіт, remediation і retest для mobile pentest та окремого crypto-review. | decision pending; не gate за замовчуванням |

Статуси `planned`, `executed-pass`, `executed-fail`, `not-applicable` та `decision-pending` у цій таблиці — кандидатний evidence vocabulary. `executed-pass` потребує immutable build/profile, exact fixture version, tool/operator, дату, результат і локатор артефакту; `not-applicable` — доказ неактивності або обґрунтування поверхні. Нинішні рядки не є результатом тесту.

## OWASP applicability check (перевірено 2026-09-23)

| Джерело | Наслідок для VIDA та тестова межа |
|---|---|
| [Threat Modeling Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html) | Моделювати flows/trust boundaries, загрози, mitigations і перевіряти їх; сам список загроз без validation не завершує модель. |
| [MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) | Для Android/iOS перевірити sensitive local data на диску/у backup/logs; не переносити mobile-control label як Windows certification. |
| [MASVS-PRIVACY-1](https://mas.owasp.org/MASVS/controls/MASVS-PRIVACY-1/) | Для мобільних permission/identifier/diagnostics flows перевірити мінімізацію та відсутність небажаної кореляції Personas. |
| [Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) | Перевіряти поточні права на кожну дію й ресурс; негативні тести stale/revoked grant, plugin і cross-Space доступу. |
| [Software Supply Chain Security Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Software_Supply_Chain_Security_Cheat_Sheet.html) | Для host/AppPackage оцінити provenance, dependencies та update integrity; це не обирає TUF чи repository format за VIDA. |
| [ASVS project](https://owasp.org/projects/asvs) | Застосовувати версійні ASVS controls лише до фактично активної web/API поверхні; він не є посвідченням native Windows або Mobile Apps. |

## Невирішена release boundary

- Architecture spine [implementation gates](../../planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md) забороняють production claims без common conformance evidence. Відкриті `OQ-0024/0033/0053` та інші listed gates не стають зеленими через цю матрицю.
- [Independent-review research](../../planning-artifacts/research/security-independent-review-for-vida-2026-09-22/research.md) пояснює різницю pentest і crypto-review, але обов'язковість до публічного Release 1 потребує відповіді на `OQ-0081`.
- Невизначені legal controller/operator/store accounts з `REQ-PRIV-009` окремо блокують public publication/production processing; security evidence не замінює юридичне рішення.
