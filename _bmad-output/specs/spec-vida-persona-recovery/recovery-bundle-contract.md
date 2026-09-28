# Owner-controlled recovery bundle — approved boundary, open byte profile

**Погоджено 2026-09-26:** комплект має дві незалежно збережені частини: випадковий recovery secret і зашифрований версійний bundle. Доступна копія зашифрованих ресурсів є третьою передумовою повернення історії, але не частиною секрету. Обов'язкового VIDA/федеративного вузла чи server escrow немає. [Дослідження і джерела](../../planning-artifacts/research/technical-vida-persona-recovery-crypto-and-bundle-2026-09-26/research.md).

| Компонент | Призначення | Заборона |
|---|---|---|
| Recovery secret | Власник зберігає поза VIDA; відкриває лише належний bundle. Кандидат: 256-бітний CSPRNG secret; людське представлення ще не затверджене. | Автоматично відправляти вузлу, включати в bundle, telemetry чи logs. |
| Encrypted bundle | Версійний portable export: окремий recovery-authority credential, необхідні data-key envelopes і signed controller checkpoint; authenticated metadata й покриття backup. | Копіювати приватні signing keys звичайних Devices або вважати зашифрований файл самодостатнім доказом актуальних прав. |
| Encrypted resource copy | Доступна репліка або власний export для повернення фактичних Note/File bytes. | Обіцяти відновлення історії за одним секретом чи за комплектом без ciphertext. |

Новий Device генерує новий keypair/`DeviceId`; тільки актуальний `ControllerState` приймає versioned signed `DeviceGrant`. Trusted-device add і lost-all-devices recovery — різні шляхи. Старий/stale checkpoint не доводить чинність authority; порядок конкуруючих переходів і ротація залишаються `OQ-0022/0024`.

UX відрізняє `secret_saved_claimed`, `bundle_available_verified`, `authority_restored`, `data_keys_restored` і `content_restored`. Позначка «історію можна відновити» потребує перевіреної доступної ciphertext-копії та достатнього coverage; окреме підтвердження користувача не є proof of storage. Кандидат production gate — fresh-profile restore, новий grant і звірка хоча б одного Note ID/content/attachment. Це не твердження, що тест уже пройшов.

**Відкрито до production:** canonical encoding/header/AAD, KDF/AEAD/nonce, human representation, bundle export/import atomicity, binding до controller/key epoch і обов'язковість нового export після ротації, manifest privacy, current-controller anti-replay, конвергенція двох одночасних restore, компрометація secret/bundle, cross-platform golden fixtures і точний restore proof. [OWASP Key Management](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html) є design/check gate, не готовою криптосхемою VIDA.
