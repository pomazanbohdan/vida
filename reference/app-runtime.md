---
id: SPEC-APP-PACKAGE-001
status: review
implementation_status: unplanned
last_updated: 2026-09-23
requirement_refs:
  - ../02-requirements/app-package-requirements.md
decision_refs:
  - ADR-0007
  - ADR-0008
  - ADR-0009
  - ADR-0010
  - ADR-0011
  - ADR-0012
source_refs:
  - ../../_bmad-output/specs/spec-vida-app-packages/SPEC.md
---

# SPEC-APP-PACKAGE-001: Package/runtime baseline

## Purpose

Визначити вже погоджену межу між переносним пакетом застосунку, керованою прикладною логікою, каналами discovery, спільним runtime та Space-scoped `AppInstance`. Це baseline, а не повний формат маніфесту, hook API чи production package manager.

## Scope і non-goals

У scope: bundled і external packages зі схемами та керованою прикладною логікою; вбудований marketplace, зовнішні репозиторії та update discovery; підтримувані runtime capabilities; окремі AppInstances; однакові permission/domain outcomes; поділ host-binary і package update lifecycle. Поза scope: довільний код із прямим OS/core-доступом, вибір Rhai/Wasmtime, точні hook/repository/catalog формати й trust policy, остаточний manifest/signing/auto-update policy, migration ABI, конкретний Flutter/Tauri renderer або FFI.

## Terminology

| Термін | Значення |
|---|---|
| `AppPackage` | Версійний переносний опис schemas, relations, commands, workflows, permission declarations, UI/data ports і, за потреби, керованої прикладної логіки. |
| Runtime capability | Реалізована VIDA primitive, на яку може посилатися package; сама declaration не додає executable semantics. |
| `AppInstance` | Space-scoped instance пакета з власною identity/configuration, ресурсами, grants і sync boundary; може бути підготовленим, але ще не активним для користування. |
| Bundled / external | Спосіб доставлення пакета, не окрема domain/security модель. |
| VIDA marketplace | Вбудований VIDA-керований канал пошуку пакетів і розширень. |
| External repository | Підключене зовнішнє джерело пакетів з метаданими для discovery та update awareness. |
| `Developer` | Marketplace-level роль автора/видавця, що може публікувати й супроводжувати package. Це не Space membership role і не доступ до customer data. |
| Extension package | Пакет, який ідентифікує базовий застосунок-ціль та може додати, замінити або вимкнути названу прикладну поведінку через контрольований runtime; точний extension-point контракт відкритий. |
| Managed app handler | Авторська логіка над schema-defined ресурсами, яку викликає runtime через дозволені host API; результат не обходить trusted core. |

## Requirements

Нормативні вимоги `REQ-APP-001`–`REQ-APP-021` наведені в [app-package-requirements.md](../02-requirements/app-package-requirements.md). Package `MUST` перевіряти підтримку потрібних runtime/UI capabilities до виконання; непідтримане `MUST` повертати явний типізований результат. Command authorization `MUST` повторно перевірятися trusted runtime, незалежно від UI state або результату handler.

## Data and state model

Source identity, package identity/version та `AppInstance` identity `MUST` бути різними поняттями. Один package `MAY` породжувати instances у кількох Spaces. Ресурси instance успадковують `OwnerSpaceId` свого Space; package origin не змінює authority або ownership. Точний маніфест, version range та installation cache state ще не затверджені.

Ефективна логіка розширення `MUST` мати binding до цільового `AppInstance` і його Space. Package cache/installation не є таким binding. Зміна конфігурації або версії handler у цьому binding не змінює інші instances чи базовий пакет глобально; точний формат binding і lifecycle його даних відкриті.

## Interfaces and protocols

Пакет описує декларації для спільного runtime та shell renderer і може прив'язувати прикладну логіку до визначеної поведінки host app; він не викликає core/runtime через приватний альтернативний API. Усі читання, команди й запропоновані зміни проходять host API і повторну trusted перевірку. На одному оголошеному прикладному тригері процес цільового `AppInstance` має пріоритет над базовим handler; власні незалежні процеси компонує розробник без Owner-level arbitration. Приклади тригерів — створення, зміна, збереження; точні назви/фази, продовження базового handler, помилки та side effects не затверджені. Межа з клієнтами узгоджується з чинною architecture spine й майбутнім `OQ-0035`. City Portal App використовує platform integration contract; пакет не є реалізацією портального backend.

За [ADR-0012](../03-architecture/decisions/ADR-0012-command-event-sync-boundary.md) нова бізнес-команда, реакція на committed fact, timer/external signal та пасивна UI projection є різними типами входу. Отримання синхронізованої операції й replay не викликають повторно command handler або новий бізнес-намір. Точний package API і виконавець реакцій із зовнішніми ефектами не визначені цим baseline.

## Lifecycle and failure behavior

Підтримані стани baseline розділені за рівнями: **джерело/пакет** — source підключено → release знайдено → package отримано → декларації перевірено; **Space** — `AppInstance` підготовлено → активовано для користування або отримано явну compatibility/authorization failure. Три bundled instances стандартного Space підготовлені від створення Space; у Personal Space onboarding користувач обирає, які вмикати та показувати, і може ввімкнути решту згодом. Підготовка не є активацією, не вмикає бізнес-обробники та не створює нових grants. Цей state split не визначає, чи саме Owner або Admin може активувати instance у shared Space (`OQ-0039`). Repository metadata може показати доступну нову версію, але не змінює активний instance до застосування його update policy. Несумісність одного `AppInstance` робить update-required/read-only або блокує лише цей instance; інші Apps і VIDA shell продовжують працювати.

Update lifecycle: discover signed metadata → перевірити publisher/channel/version/expiry/host compatibility/dependencies → отримати content-addressed artifacts → перевірити digest/signatures → виконати migration preflight на ізольованій копії/checkpoint → атомарно активувати package + current schema + converter set → синхронізувати activation operation. Failure **до** atomic commit залишає попередню активну версію без часткової міграції; це aborted activation, не rollback. Після успішної активації baseline не має downgrade/rollback чи runtime fallback flow. Post-activation defect вважається порушенням release gate; окрему recovery-поведінку користувач не затвердив. Це не скасовує TUF-style **anti-rollback** захист від старої/відкликаної версії, freeze чи mix-and-match metadata.

Native host binary (`Flutter`/platform shell, Rust core/runtime, Windows shell) оновлюється окремим OS/store/desktop channel. `AppPackage` policy на рівні AppInstance має чотири режими: `compatible-auto`, `security-auto`, `manual`, `pinned`. Bundled first-party Apps за замовчуванням використовують compatible/security auto; external repository — `manual`, доки publisher не отримав явну довіру. Owner або Admin із загальним правом керування Apps змінює режим. Погоджений `compatible-auto` допускає UI fixes, optional/defaulted fields і оптимізацію managed logic лише за незмінної сумісності старих даних, workflows, dependency contracts, supported host API та authorized outcomes. Нова mandatory capability, breaking migration/converter, розширення dependency contract або новий data scope виводять release з `compatible-auto`. Точна класифікація `security-auto` ще відкрита. Package `MUST NOT` активувати capability, якої не підтримує host, а security update не обходить Space activation authority чи migration gate.

## Security and privacy

Жоден package/handler не обходить Space grants, hard deny, resource ownership, приймання операцій, ключі або sync. Відображення UI чи повернення handler `allowed` не є доказом права на command. Роль `Developer`, publisher/source trust, signing, revocation і sandbox policy визначають supply-chain повноваження, але не Space authorization. Developer може окремо бути Owner/Admin конкретного Space за звичайними membership rules; публікація package не є авторизацією на читання чи зміну customer resources.

Фраза «App має всі права у своєму Space» безпечно трактується лише як **повний оголошений domain API власного AppInstance**, викликаний від імені чинного актора. Handler може читати/змінювати власні типи ресурсів настільки, наскільки це дозволяють роль актора, hard deny та Core-governance. Він не отримує ключі, Owner-права або дані іншого AppInstance лише тому, що package активовано.

Залежність від Messenger, Notes, Projects або «Міста України» є нормальною first-class можливістю, але описується versioned typed contract: package ID/capability, дозволені operations/data views, version range і direction. Власник Space може схвалити bundled preset під час активації; зовнішня залежність потребує явного grant. Link або dependency не перетворюється на необмежений raw-data access. Це відповідає OWASP least privilege/deny-by-default і зберігає можливість композиції Apps. Чи один Space містить кілька AppInstances, чи кожен App має окремий Space, лишається `OQ-0070` і не може бути змінено приховано через update policy.

Apple App Review Guideline 2.5.2 вимагає self-contained App Store app і забороняє завантажувати/встановлювати/виконувати код, що додає або змінює features/functionality. Інтерпретатор не обходить правило: downloadable Rhai/Wasm handler, який створює нову поведінку, має високий review risk. Безпечний baseline iOS profile тому приймає package як підписані **дані**: schemas, forms, layouts, workflows і посилання лише на вже вбудовані VIDA runtime capabilities; довільний native/JIT code виключений.

Guideline 4.7 окремо допускає певні HTML5/JavaScript mini apps, streaming games, chatbots і plug-ins, але host VIDA тоді відповідає за весь вміст і мусить забезпечити moderation/report/block abuse, privacy та явну згоду на data/permission sharing для кожного mini app, індекс і metadata/universal links, age-rating controls та правила digital-goods commerce. 4.7.2 також не дозволяє без дозволу Apple віддавати такому software доступ до native platform APIs. Apple Mini Apps Partner Program додає manifest і commerce/age APIs. Для iOS v1 затверджено декларативний profile: package містить schemas, forms, layouts, workflows і посилання на вже вбудовані VIDA capabilities, але не завантажуваний Rhai/Wasm/JavaScript executor. Окремий Apple 4.7 HTML5/JavaScript mini-app catalog відкладено в майбутній контур; він не є прихованою частиною v1.

Референс WeChat/Tencent показує архітектурний патерн, а не виняток із правил Apple: mini-program завантажується й кешується host-застосунком; WXML/WXSS описують view, JavaScript працює в обмеженому logic layer без browser `window/document`, а native можливості доступні лише через host-provided open APIs. Host SDK, каталог/backend, AppID, review/update lifecycle і sandbox разом створюють контрольовану платформу. Отже VIDA може проєктувати аналогічний профіль, але це означає окремий iOS JavaScript/HTML5 execution target або Apple-approved language, а не автоматичне виконання довільного Rhai/Wasm payload.

Google дає два інші патерни. Android Play Feature Delivery доставляє on-demand code modules, але вони підписані й збираються як частина одного Android App Bundle; це не third-party mini-app marketplace і не рішення для iOS. Firebase Remote Config змінює параметри та вмикає вже вбудовані можливості, але не доставляє новий executable functionality. Тому для VIDA є три різні механізми, які не слід називати однаково: `declarative package` на built-in capabilities; `iOS 4.7 mini app` у sandboxed HTML5/JS runtime; `native host feature` через store release/dynamic platform delivery.

## Compatibility and migration

Для кожного підтримуваного client profile потрібне versioned capability negotiation та conformance сценарії. Старий клієнт `MUST NOT` мовчки тлумачити невідому declaration як відому або виконувати write зі втратою unknown fields. Кожен resource/operation має schema version; нові writes використовують активну current schema, а старі дані читаються через versioned conversion/migration без переписування immutable history. Під час редагування legacy record його write `MUST` відповідати current schema: defaults materialize, required-without-default блокує save до введення значення. Rename/deprecation preserves stable identity/history. Детальна пропозиція — [schema evolution contract](schema-evolution-contract.md); migration authority, mixed-version window і forward-repair behavior залишаються `OQ-0040`/`OQ-0037`.

## Acceptance criteria

1. Стандартний Space запускає три bundled AppInstances через загальний contract.
2. Незмінений клієнт підключає сумісний зовнішній декларативний пакет і виконує supported workflow.
3. Один package створює два ізольовані Space instances.
4. Denied command відхиляється навіть при прямому виклику повз UI.
5. Невідома capability повертає типізовану compatibility failure.
6. VIDA marketplace та тестовий зовнішній репозиторій окремо показують застосунки й розширення з provenance та target host.
7. Нова версія в repository metadata відображається як доступне оновлення, не змінюючи активний instance без окремої дії/політики.
8. Тестовий Notes extension змінює названу прикладну поведінку через підтримуваний handler; спроба handler виконати заборонену команду відхиляється trusted core.
9. Зміна поведінки Notes A у Space X не змінює Notes B у Space X або Notes у Space Y; встановлення package без активації не змінює жодної дії.
10. На спільному тригері збереження instance handler має пріоритет над базовим; незалежна post-save дія, скомпонована розробником, не потребує окремого вибору Owner-а.
11. Дубль синхронізованої операції та replay після reconnect відновлюють той самий стан без нового виклику command handler чи зовнішнього бізнес-ефекту.
12. Старий record із defaulted field відкривається без eager rewrite; перший edit записує current schema й materialized default.
13. Старий record із новим required field без default не зберігається, доки користувач не заповнить поле; початкові дані не губляться.
14. Package update із невідомою mandatory capability не активується; чинний instance і дані лишаються відновлюваними.
15. Stale-version downgrade, freeze і mix-and-match package metadata fixture відхиляється до migration/activation; після activation немає downgrade flow.
16. iOS release fixture не завантажує code-bearing package tier, доки platform policy profile явно не доведе його допустимість.
17. `compatible-auto` fixture відхиляє release з новою mandatory capability, breaking converter, розширеним dependency contract або новим data scope.
18. Cross-App dependency fixture надає лише оголошений typed view/operation і не обходить права актора чи hard deny цільового AppInstance.
19. Складний App створює dedicated Space й активує Messenger, Notes і Projects як залежні AppInstances без окремих реалізацій їхніх протоколів.
20. iOS v1 приймає декларативний package на built-in capabilities і відхиляє code-bearing Rhai/Wasm/JavaScript payload.

## Open questions

- source/publisher trust policy package;
- cache/install scope і повноваження на activation у Space;
- repository/package manifest, TUF-style trust/signing roles, OCI-style чи власний content-addressed distribution profile та повний migration/release validation contract;
- межа декларативного package content і downloaded executable logic для Android/iOS/Windows release profiles;
- майбутній Apple 4.7 HTML5/JavaScript mini-app catalog, його review/consent/commerce runtime і portable Logic IR; це не scope iOS v1;
- default grants для bundled і external cross-App dependencies у dedicated App Space;
- конкретні hook phases, продовження/приглушення базового handler, помилки, executor (Rhai/Wasm), lifecycle binding та resource/side-effect limits.

Reference check 2026-09-21: [TUF specification](https://theupdateframework.github.io/specification/) defines signed root/targets/snapshot/timestamp metadata and downgrade/freeze/mix-and-match checks; [OCI Distribution](https://github.com/opencontainers/distribution-spec/blob/main/spec.md) is content-type agnostic and addresses manifests/blobs by digest. Apple [App Review Guidelines 2.5.2 and 4.7](https://developer.apple.com/app-store/review/guidelines/) and [Mini Apps Partner Program](https://developer.apple.com/programs/mini-apps-partner/) define the iOS constraints summarized above. Tencent documents the [Mini Program JavaScript logic layer](https://intl.cloud.tencent.com/document/product/1219/61743), [runtime download/cache/update lifecycle](https://intl.cloud.tencent.com/document/product/1219/62838?lang=en) and [host open APIs](https://intl.cloud.tencent.com/document/product/1219/62850?lang=en). Google [Play Feature Delivery](https://developer.android.com/guide/playcore/feature-delivery) and [Firebase Remote Config](https://firebase.google.com/docs/remote-config) demonstrate native dynamic modules versus configuration of prebuilt behavior. OWASP [Authorization](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html), [Third Party JavaScript Management](https://cheatsheetseries.owasp.org/cheatsheets/Third_Party_Javascript_Management_Cheat_Sheet.html) and [MASVS-PRIVACY-1](https://mas.owasp.org/MASVS/controls/MASVS-PRIVACY-1/) support explicit least-privilege cross-App grants and sandboxed data access. None of these sources pre-approve VIDA, Rhai or Wasm.

Додаткове джерельне порівняння 2026-09-23: [BMad repository-trust options](../../_bmad-output/specs/spec-vida-app-packages/repository-trust-options.md) містить TUF+OCI, TUF+digest-blobs і custom-index кандидати, `PKG-T01–T07` negative fixtures та OWASP dynamic-code/supply-chain орієнтири. Воно не закриває `OQ-0041`/`OQ-0043` і не дозволяє `security-auto` за самою міткою видавця.
