# Незалежний adversarial review: recovery AD-37/40, 2026-09-29

**Вердикт:** продуктові рішення узгоджені, але незалежні production-реалізації recovery/backup ще не сумісні. AD-37/40 правильно залишають byte/proof/rotation protocol у `OQ-0022/0024`; ці питання — справжній блокер реалізації, а не привід назвати погоджену поведінку незатвердженою. Фізичний ноутбук із Personal і Work уже однозначно потребує окремих логічних Device IDs/ключів; прогалина лишається в backup/custody domain separation.

## 1. Покриття перевіреної копії не має спільного значення — high

**Два формально сумісні виконавці:** Android зберігає у manifest загальний `SyncLog` frontier і після authenticated readback пише «перевірено». Web зберігає пооб'єктні heads, controller frontier, key epoch і blob closure; той самий файл Android відхиляє як неповний. Обидва здійснили write → readback → authenticated open → власну coverage validation, як вимагає AD-37, але незгодні, чи конкретна Note, її вкладення та післяротаційні ключі реально відновлювані. Можлива й гонка: snapshot містить Note до операції X, а manifest записав frontier після X.

**Наслідок:** фальшива гарантія restore або несумісність Android/Web. **Потрібно в `OQ-0024`:** нормативне визначення coverage як атомарного причинного зрізу; manifest має доводити замкненість потрібної історії, resource/blob bytes, data-key envelopes та відповідного controller/key-epoch стану. Визначити, чи частковий backup допустимий і як саме він маркується. Golden fixture: concurrent write під час export, відсутній blob, старий key epoch, backup з frontier X без операції X. Посилання: [AD-37](../ARCHITECTURE-SPINE.md), [CAP-3/5](../../../../specs/spec-vida-persona-recovery/SPEC.md), [REC-F22–F24](../../../../specs/spec-vida-persona-recovery/recovery-cases.md).

## 2. Crash під час звичайної ротації допускає втрату останнього робочого комплекту — high

**Два формально сумісні виконавці:** A підписує controller rotation і припиняє приймати старий комплект до завершення нового export/custody; B залишає старий комплект чинним, доки новий файл не записаний, прочитаний і власник не підтвердив окреме зберігання. AD-37 каже «ordinary rotation creates a new kit and requests custody without blocking normal edits», а SPEC не визначає момент зміни authority. Crash після підпису, але до durable bundle, залишає A без перевірено доступного комплекту; B може відновити попереднім. У сценарії компрометації, навпаки, чекати custody може залишити старий комплект небезпечно чинним.

**Потрібно в `OQ-0024`:** дві окремі state machines для ordinary rotation і suspected compromise; crash-atomic переходи `prepared → exported/verified → custody-confirmed → effective`, чітка дія для старого комплекту на кожному кроці; протокол restart/rollback або forward completion без відновлення відкликаного secret; binding до controller frontier і epoch. Фікстури crash на кожному кроці та паралельного offline Device. Посилання: [AD-37](../ARCHITECTURE-SPINE.md), [CAP-4](../../../../specs/spec-vida-persona-recovery/SPEC.md), [REC-F15/F16](../../../../specs/spec-vida-persona-recovery/recovery-cases.md).

## 3. Розділені Device keys не гарантують розділені recovery kits — medium/high

**Два формально сумісні виконавці:** обидва створюють окремі Personal/Work `DeviceId` і keypair та не об'єднують grants. A експортує один installation-level encrypted archive/secret з material обох Personas; B — два незалежні per-Persona kits. AD-40 refinement забороняє union authority, але не називає cryptographic/export namespace; у A втрата одного секрету розкриває обидва контексти або створює спільний linkage/custody failure. Поведінка реєстрації однакова, recovery/privacy — ні.

**Потрібно в `OQ-0022/0024`:** визначити kit, recovery-authority credential, manifest/AAD та backup index per `PersonaId`; заборонити installation-global recovery authority і неявне об'єднання envelopes; окремо вирішити, чи явний multi-Persona export як зручний контейнер допустимий без спільного ключа/права. Фікстура: Personal kit не відкриває Work bundle чи key envelopes навіть на тому самому ноутбуці; Work node не отримує Personal linkage через export. Посилання: [AD-37/40](../ARCHITECTURE-SPINE.md), [CAP-2/5](../../../../specs/spec-vida-persona-recovery/SPEC.md), [REC-F21](../../../../specs/spec-vida-persona-recovery/recovery-cases.md).

## Межа огляду

Не пропонується змінювати погоджені правила: локальна робота триває без backup; неперевірений export не називається перевіреною копією; fresh-profile restore проводиться після першої копії з даними й ротації; grants різних Personas на одній інсталяції співіснують без union. Перевірявся саме ризик різних реалізацій нижчого рівня. AD-40 явно лишає same-logical-Device/multiple-Space-scope grant merge відкритим; це окремий `OQ-0022`, не заперечення вже погодженого Personal/Work рішення.
