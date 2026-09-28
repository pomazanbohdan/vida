---
id: SPEC-VIDA-PERSONA-RECOVERY
status: draft-decision-gated
companions:
  - recovery-cases.md
  - recovery-bundle-contract.md
  - ../../../docs/02-requirements/identity-requirements.md
  - ../../../docs/04-specifications/identity-domain-contract.md
  - ../../planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md
sources:
  - ../../planning-artifacts/research/technical-vida-persona-recovery-crypto-and-bundle-2026-09-26/research.md
---

# Persona recovery and new-Device enrollment

## Why

Власник автономної Persona має зберігати контроль після втрати пристрою, не передаючи приватну ідентичність федеративному вузлу. Відновлення права керувати Persona та відновлення історії — різні результати: для другого потрібні ще доступні зашифровані дані.

## Capabilities

- **CAP-1**
  - **intent:** Власник створює і зберігає recovery material окремо від пристрою під час створення автономної Persona.
  - **success:** Setup показує вже створений секрет і надає зашифрований bundle для окремого збереження, відновлює незавершений крок після restart, вимагає підтвердження збереження обох частин і не відправляє секрет федеративному вузлу; лише після підтвердження активує ту саму Persona. UI попереджає, що втрата всіх Devices разом із комплектом може бути незворотною, а підтвердження не доводить існування копії.
- **CAP-2**
  - **intent:** Власник додає новий Device через чинний довірений Device або власний recovery authority, не змінюючи Persona.
  - **success:** Новий Device має окремі ID і ключі; trusted enrollment перевіряє чинний `ControllerState` і видає підписаний `DeviceGrant`. Коли після втрати всіх Devices свіжий frontier недоступний, recovery authority може видати лише явно provisional локальний grant до подальшої звірки.
- **CAP-3**
  - **intent:** Власник відновлює контроль і лише ту історію, для якої залишилися доступні зашифровані дані.
  - **success:** Restore окремо показує результат відновлення authority, data keys і content; секрет без bundle не видається за відновлення authority, а комплект без доступної зашифрованої копії ресурсів не видається за відновлення історії. Якщо всі старі Devices недоступні й актуальність controller history неможливо перевірити, новий Device може отримати необхідний локальний recovery grant і працювати в явно provisional гілці до звірки, не видаючи її за перевірену спільну authority.
- **CAP-4**
  - **intent:** Власник відкликає втрачений або скомпрометований Device без втрати своєї Persona.
  - **success:** Старий `DeviceGrant` перестає діяти, affected envelopes/epochs змінюються, а stale/replayed transition не повертає доступ. При підозрі на витік recovery secret і bundle власник запускає окрему термінову ротацію комплекту й переузгодження Devices; вже скопійований стороннім plaintext не вважається відкликаним.
- **CAP-5**
  - **intent:** Власник за бажанням додає зашифровану OS/cloud копію recovery bundle.
  - **success:** Залишається документований незалежний шлях збереження; відсутня, застаріла чи недоступна копія показує явний результат, а не хибну гарантію відновлення.

## Constraints

- `PersonaId`, `DeviceId`, `EndpointId` і `ServiceAccountId` різні; кожний Device має окремі ключі та відкличний grant.
- Для спільно визнаних `DeviceGrant`/revoke потрібна перевірна controller history; recovery authority може видати необхідний локальний provisional grant без заяви про актуальність для інших peers. Вузол не стає controller приватної Persona через attestation чи відновлення власного акаунта.
- **[APPROVED — product owner, 2026-09-26]** Власник окремо зберігає випадковий recovery secret і версійний зашифрований recovery bundle; для історії потрібна ще доступна копія encrypted resources. Bundle не містить звичайних Device private signing keys; новий Device створює власні ключі. Деталі у [recovery-bundle-contract.md](recovery-bundle-contract.md).
- Корпоративне відновлення не відкриває анонімну чи приватну Persona; секрети й між-Persona linkage не потрапляють у logs, telemetry або публічні проєкції.
- Key lifecycle і захист exported material перевіряються за [OWASP Key Management](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html); локальне зберігання чутливих даних у мобільному клієнті — за [MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/). UX і фактичне відновлення перевіряють власні fixtures VIDA; жодне з цих джерел не затверджує її конкретний формат.
- **[APPROVED — product owner, 2026-09-26]** Незавершений локальний bootstrap не дорівнює `PersonaCreated`: staged Persona/Space/Owner authority не відкривають protected Space work до durable confirmation; після restart відновлюється той самий pending стан. Це правило VIDA, а не вимога Delta Chat чи OWASP; перевірки наведені в [recovery-cases.md](recovery-cases.md).
- **[APPROVED — product owner, 2026-09-28]** Два незалежні відновлення тим самим комплектом можуть зберігати локальні дані. Після зустрічі сумісні гілки об'єднуються, а несумісні controller/grant transitions не мають переможця за часом чи Device; спірні authority-дії чекають перевірного reconciliation. Доведено відкликаний комплект не публікує локальних операцій; автор може врятувати текст після нової авторизації.
- **[APPROVED — product owner, 2026-09-28]** Звичайна ротація вимагає нового комплекту й наполегливого запиту зберегти його без блокування щоденних редагувань. Підозрюваний витік обох частин вимагає негайного відкликання старого комплекту; жодна ротація не стирає вже викрадених копій.

## Non-goals

- Ця чернетка не затверджує байтовий encoding, конкретні KDF/AEAD/nonce, конкретне сховище, криптографічний механізм ротації чи production enrollment proof; обов'язкового server escrow немає.
- Корпоративну controller/recovery policy не переносимо на приватну Persona і не закриваємо тут.

## Success signal

Сценарії [REC-F01–F16 та REC-F09a](recovery-cases.md) мають демонструвати staged/restart/finalize lifecycle, окреме збереження двох частин, provisional restore, concurrent restore, stale-kit rejection, ротацію, нові ключі Device, відмову replay і чесну межу відновленої історії; перевірка чужого чи пошкодженого bundle лишається кандидатним oracle до остаточного профілю `OQ-0024`. Це план випробувань, а не production-ready протокол.

## Open Questions

- `OQ-0022/0024`: який перевірний protocol reconcile provisional/concurrent controller branches, proof відкликання старого комплекту, byte format і представлення secret, KDF/AEAD/nonce profile, bundle-to-controller/key-epoch binding і atomic re-export, integrity/version/ownership validation, manifest visibility та cross-platform restore fixtures реалізують погоджену модель?
- `OQ-0049`: хто контролює окрему корпоративну Persona і які саме grants може відновити або припинити компанія?
