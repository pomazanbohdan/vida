---
title: 'Technical research: VIDA Persona recovery crypto and bundle'
type: technical
topic: 'Owner-controlled recovery authority, portable encrypted bundle, platform key custody and restore proof'
decision: 'Which OQ-0024 design should VIDA approve for Release-1 autonomous Persona recovery?'
source: 'Primary standards and official project/platform documentation checked 2026-09-26'
status: direction-approved-details-open
preset: standard
validation: normal
created: '2026-09-26'
updated: '2026-09-26'
---

# Technical research: VIDA Persona recovery crypto and bundle

**Decision this research serves:** Select a candidate architecture for OQ-0024 without confusing controller continuity with restoration of Note data, or treating OS/browser storage as a guaranteed backup.

## Висновок для рішення

**Рішення власника продукту, 2026-09-26:** погоджено двокомпонентну owner-held модель із окремими secret та encrypted bundle; для повернення історії потрібна ще encrypted resource copy. Це затверджує напрям, але не точний криптографічний або байтовий профіль і не закриває production gate `OQ-0024`.

**Рекомендація — користувацький комплект із двох незалежно збережених частин:** (1) випадковий recovery secret з ентропією 256 біт; (2) версійний зашифрований recovery bundle. Bundle містить окремий recovery-authority credential, конверти ключів даних і підписаний checkpoint керування Persona; **не** містить приватних ключів її звичайних Device. Для повернення історії потрібна ще доступна копія зашифрованих ресурсів — у власному export, на іншому Device або в необов'язковій сховищній replica. Secret без bundle не доводить відновлюваність; secret і bundle без ресурсів повертають щонайбільше керування Persona, але не нотатки. Це *кандидат на затвердження*, а не вже прийнятий криптопротокол.

**Рішення зараз:** затвердити цей напрям для прототипу Story 1.1 та окремий restore-proof gate. **Не закривати OQ-0024 для production**, доки не визначені актуальність ControllerState, ротація скомпрометованого комплекту, конкретний формат/шифр, атомарне збереження та негативні тести.

### Межа з чинними рішеннями VIDA

Специфікація [Persona recovery](../../../specs/spec-vida-persona-recovery/SPEC.md) і AD-37 в [architecture spine](../../architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md) уже вимагають: та сама Persona після перерваного onboarding; підтвердження користувачем винесеного за межі Device матеріалу до завершення створення; новий Device отримує versioned signed `DeviceGrant` через trusted Device або recovery authority; попередні Device keys не копіюються; збережена десь зашифрована копія даних є окремою передумовою відновлення історії. Тут пропонується реалізація цієї межі, а не зміна її.

### Що роблять референси — і чого з них не випливає

| Референс | Встановлений факт | Застосування / межа для VIDA |
|---|---|---|
| [Signal Secure Backups][1], [troubleshooting][2] | Користувач володіє recovery key; зашифрований backup мусить існувати, а втрачений ключ Signal не відновлює. | Відокремити та перевірити ключ і ciphertext; Signal покладається на власний backup service, VIDA не мусить. |
| [Matrix secret storage][3], [server-side backup][4] | Метадані захищених секретів і версії encrypted backup явні. | Версіонувати capsule і key envelopes; не переносити точну схему шифрування Matrix без окремого вибору. |
| [Element recovery guide][5] | Практичну перевірку ключа роблять на іншому вході/Device. | Свіжий профіль, а не лише кнопка «я зберіг», дає restore proof. |
| [SimpleX local data][6], [security][7] | Platform Keychain/Keystore захищає локальну базу; export/import — інший механізм. | Device-local захист ≠ переносний backup. |
| [Delta Chat FAQ][8] | QR перенесення на інший Device і локальний export — різні шляхи. | Trusted-device enrollment ≠ lost-all-devices recovery; Chatmail/пошта не стає транспортом VIDA. |

### Варіанти дизайну

| Варіант | Вигода | Ризик / оцінка |
|---|---|---|
| **A. Окремий recovery authority у зашифрованому bundle + випадковий secret** | Device keys незалежні; recovery authority і data-key envelopes можна ротувати, bundle може лежати в недовіреному сховищі; не потрібен обов'язковий сервер. | Потрібні **обидві частини**; втрата або застарівання bundle блокує гарантоване відновлення. **Рекомендовано.** |
| B. Один детерміністичний seed для Persona, Device та даних | Один рядок для збереження. | Компрометація seed вражає всі ролі; незалежність Device keys і revocation слабшають. **Відхилити.** |
| C. Людський пароль + зашифрований bundle | Пароль легше запам'ятати. | Offline guessing; [Argon2id][11] та параметри на слабких телефонах і Web ускладнюють v1. Лише майбутній додатковий спосіб, не canonical recovery. |
| D. Лише trusted-device QR або OS/cloud backup | Мінімальний onboarding. | Втрата останнього Device / browser profile закриває шлях; OS backup не гарантований. **Недостатньо.** |

### Пропонований контракт A — на рівні полів, не байтів

1. `recovery-secret-v1`: 32 байти з CSPRNG; окремо показати для збереження поза VIDA, без автоматичного передавання вузлу. Людське представлення (hex/base32, групування, контроль помилок) обрати після UX і cross-platform тесту. Не використовувати людський пароль як цей secret. [OWASP][9] [RFC 5869][10]
2. `recovery-bundle-v1`: відкритий versioned header (`format`, `persona_id`, `controller_epoch`, `key_ids`, `kdf_id`, `aead_id`, `salt`, `nonce`, `ciphertext_length`, `snapshot_frontier`), authenticated encrypted body (окремий recovery-authority private credential, лише потрібні data-key envelopes, посилання на manifest копій, signed ControllerState checkpoint). Header, що впливає на значення, має бути authenticated associated data. Без plaintext Device signing keys. Конкретний binary encoding (наприклад deterministic CBOR), cipher suite і nonce strategy — **ще не затверджені**. [OWASP][9] [AEAD][12]
3. Secret → purpose-separated wrapping/check keys через HKDF; bundle шифрується AEAD. Новий Device генерує власний keypair, відновлений authority підписує **новий** `DeviceGrant`; старий keypair не імпортується. Це архітектурна пропозиція; криптографічні профілі слід незалежно перевірити. [RFC 5869][10] [OWASP][9]
4. `data-backup-manifest` окремо фіксує покриття (до якого frontier/часу), наявність key envelopes, blob hashes і місця отримання ciphertext. Повний offline-export може включати bundle та encrypted resources, але **ніколи** recovery secret. Інакше володіння одним файлом давало б повний доступ. [Signal][1] [Matrix][4]
5. Локально bundle/keys захищаються platform store: Android Keystore або Windows DPAPI; профіль iOS Keychain перевіряється окремо до його імплементації. Це додатковий захист, не гарантія переносу. Static Web має окремо запропонувати external export: браузерний storage може зникнути, а non-extractable WebCrypto key не є захистом від коду того ж origin. [Android][13] [Web][14] [Windows][15] [browser storage][16]

### Перевірюване відновлення та UX gate

Перед позначкою «відновлення готове» користувач повинен: зберегти secret окремо; зберегти/розмістити bundle; побачити, де є **принаймні одна доступна зашифрована копія** даних; пройти read-only пробу дешифрування bundle і manifest. Для сильнішого proof — відновити тестову Persona на ізольованому новому профілі/Device, видати новий `DeviceGrant`, прочитати контрольну нотатку/attachment та звірити ID, frontier і bytes. Власне підтвердження користувача доводить лише заяву, не наявність off-device копії. [Element][5] [Signal][2]

При створенні, якщо застосунок закрито до підтвердження, залишається той самий pending Persona/ID; protected Notes не відкриваються. Export/restore можна повторювати після crash. У розрахунок успіху входять **три** окремі результати: `authority_restored`, `data_keys_restored`, `data_content_restored`. Якщо ciphertext втрачено, показати «керування Persona відновлено; історію даних не знайдено», а не «відновлено все».

### Загрози, межі, тести до production

- Якщо secret і bundle потраплять до нападника, він може спробувати видати собі DeviceGrant; recovery credential має бути відкликаний через підписаний controller transition, а data-key envelopes ротовані. Потрібні точні правила для одночасного відновлення та застарілих checkpoint; без доступного актуального ControllerState сам файл не доводить, що власника не відкликали. Це залежність від OQ-0022 / controller ordering, а не властивість AEAD.
- Втрачений останній ciphertext не відновить жоден secret. Скомпрометований secret не можна «стерти» на офлайн-пристрої; після ротації старі копії, які нападник вже дістав, лишаються ризиком. Наявна в Device пам'ять, screenshots і exports поза контролем ретроактивного відкликання.
- Negative fixtures: неправильний secret; чужий Persona ID; пошкоджений/обрізаний body; заміна header; replay старого epoch; неправильний AEAD nonce; два одночасні відновлення; crash/disk-full між grant і записом; зміна схеми bundle; cleared browser storage; недоступна копія blob; відсутність secret у логах/telemetry. Тест має порівнювати однакові результати Android/Web, потім iOS/Windows.
- За [OWASP Key Management][9] exported secrets і їхній lifecycle — критичні. До release gate потрібні documented threat model, криптографічний review конкретного формату та автоматичні cross-platform restore fixtures; цей звіт не підтверджує їхнє виконання.

### Пропозиція для погодження

1. **Архітектура A:** secret і encrypted bundle зберігаються окремо; user-controlled export обов'язковий, OS/cloud copy опційний; encrypted data replica — окрема передумова відновлення історії.
2. **Видимі стани:** «ключ збережено» ≠ «комплект перевірено» ≠ «історію можна відновити»; показувати фактичне покриття backup.
3. **Story 1.1:** дозволити прототип формату й ізольованого restore proof, але production «готово» лише після controller-ordering, rotation, crypto profile, durable-write та negative-fixture gates.

Власник продукту **погодив** окреме збереження secret і зашифрованого bundle та додаткову потребу в ciphertext для повернення нотаток. Наступні рішення не стосуються повторного вибору цієї моделі: вони визначать точні криптографічні параметри, актуальність controller state, ротацію й перевірюваний restore.

### Джерела

| № | Первинне джерело | Тип / перевірка |
|---|---|---|
| [1] | [Signal Secure Backups][1] | Офіційна довідка, 2026-09-26 |
| [2] | [Signal backup troubleshooting][2] | Офіційна довідка, 2026-09-26 |
| [3] | [Matrix secret storage][3] | Відкрита специфікація, 2026-09-26 |
| [4] | [Matrix key backups][4] | Відкрита специфікація, 2026-09-26 |
| [5] | [Element recovery guide][5] | Офіційна довідка, 2026-09-26 |
| [6] | [SimpleX managing data][6] | Офіційна документація, 2026-09-26 |
| [7] | [SimpleX security][7] | Офіційна документація, 2026-09-26 |
| [8] | [Delta Chat FAQ][8] | Офіційна довідка, 2026-09-26 |
| [9] | [OWASP Key Management][9] | OWASP Cheat Sheet, 2026-09-26 |
| [10] | [RFC 5869 HKDF][10] | IETF RFC, 2010 |
| [11] | [RFC 9106 Argon2][11] | IETF RFC, 2021 |
| [12] | [RFC 8439 ChaCha20-Poly1305][12] | IETF RFC, 2018 |
| [13] | [Android Keystore][13] | Офіційна документація, 2026-09-26 |
| [14] | [Web Cryptography API][14] | W3C working draft/specification, 2026-09-26 |
| [15] | [Windows DPAPI][15] | Офіційна документація, 2026-09-26 |
| [16] | [WHATWG Storage Standard][16] | Відкритий стандарт, 2026-09-26 |

[1]: https://support.signal.org/hc/en-us/articles/9708267671322-Signal-Secure-Backups "Signal Secure Backups, official support"
[2]: https://support.signal.org/hc/en-us/articles/10075139325850-Troubleshooting-Signal-Secure-Backups "Signal backup troubleshooting, official support"
[3]: https://spec.matrix.org/latest/client-server-api/#secret-storage "Matrix Client-Server API: Secret storage"
[4]: https://spec.matrix.org/latest/client-server-api/#server-side-key-backups "Matrix Client-Server API: Server-side key backups"
[5]: https://docs.element.io/latest/element-support/device-verification/how-to-ensure-you-have-a-recovery-key/ "Element recovery key guide"
[6]: https://simplex.chat/docs/guide/managing-data.html "SimpleX managing data"
[7]: https://simplex.chat/docs/guide/privacy-security.html "SimpleX privacy and security"
[8]: https://delta.chat/en/help "Delta Chat FAQ"
[9]: https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html "OWASP Key Management Cheat Sheet"
[10]: https://www.rfc-editor.org/rfc/rfc5869.html "RFC 5869 HKDF"
[11]: https://www.rfc-editor.org/rfc/rfc9106.html "RFC 9106 Argon2"
[12]: https://www.rfc-editor.org/rfc/rfc8439.html "RFC 8439 ChaCha20-Poly1305"
[13]: https://developer.android.com/privacy-and-security/keystore "Android Keystore"
[14]: https://www.w3.org/TR/WebCryptoAPI/ "Web Cryptography API"
[15]: https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata "Windows DPAPI"
[16]: https://storage.spec.whatwg.org/ "WHATWG Storage Standard"

Дати публікації змінюваних сторінок не всюди вказані; у `staleness-input.json` дата перевірки 2026-09-26 використана як **proxy**, не як твердження про дату публікації. Найраніше планове переперевіряння змінюваних product/platform джерел — 2026-12-26, і обов'язково перед імплементацією. RFC є стабільними стандартами. Дані VIDA з репозиторію — продуктова вимога, не зовнішнє джерело.
