---
id: SPEC-BLOB-STORE-001
status: approved
implementation_status: unplanned
last_updated: 2026-09-21
requirement_refs:
  - ../02-requirements/transport-sync-requirements.md
  - ../02-requirements/platform-nfr.md
decision_refs:
  - ../03-architecture/decisions/ADR-0005-iroh-transport-foundation.md
---

# SPEC-BLOB-STORE-001: BlobStore contract

## Purpose

Відокремити великі immutable payloads від operation log та забезпечити verified, resumable і policy-controlled transfer.

## Manifest contract

Blob manifest `MUST` bind:

- content/blob identifier and digest semantics;
- byte size and chunk/range information;
- encryption/key-wrapping reference;
- media/type metadata required before acceptance;
- Space/resource authorization context;
- retention/pin class and owning references.

## Interfaces

```text
Offer(manifest) -> Capability
Accept(manifest, policy) -> TransferPlan | Rejected
Fetch(capability, ranges) -> VerifiedChunks
Pin(blobId, ownerRef)
Unpin(blobId, ownerRef)
Collect(policyFrontier)
```

Capabilities `MUST` be scoped and expiring. Possession of blob bytes or a stale ticket `MUST NOT` bypass Space authorization for future fetches.

## Device download admission

Клієнт `MUST` мати налаштування граничного розміру файла для автоматичного завантаження. Файл, що перевищує межу, `MUST NOT` завантажуватися автоматично: потрібна окрема дія підтвердження користувача. Manifest/metadata можуть синхронізуватися до payload, щоб людина бачила, що саме доступне для завантаження. Чи межа задається на пристрої або на рівні профілю, її стартове значення, поведінка на мобільному трафіку й облік доступного місця лишаються відкритими. Це правило керує **початком download**, а не дає дозволу видалити вже збережені conflict variants або останню відновлювану копію.

Якщо payload не вдалося записати на кінцевому пристрої через нестачу місця, quota, I/O або verification failure, shared resource/manifest `MUST NOT` змінюватися й файл `MUST NOT` позначатися доступним локально. UI показує локальний стан **«не завантажено»** та явну причину/помилку останньої спроби; metadata лишається доступною. Це локальний availability state, не conflict, не видалення віддаленої копії та не дозвіл на GC.

## Security and deletion

Convergent encryption is `MUST NOT` by default until leakage is explicitly accepted. Hash-over-plaintext versus hash-over-ciphertext, per-object keys and encrypted manifests require prototype selection. Deleting a reference `MUST NOT` promise immediate physical erasure across replicas/backups.

Конкурентна заміна файла `MUST NOT` видаляти жоден variant лише тому, що інший був прийнятий локально або прийшов через sync раніше. Blob bytes та resource-version metadata `MUST` залишатися recoverable у межах retention/rights policy, щоб уповноважена людина могла порівняти обидва й вирішити конфлікт. Два окремі варіанти збереження альтернативи затверджені: **ревізія того самого resource** або **копія як новий resource із власним ID**. Кожна дія перевіряє чинні права, не підміняє основний файл мовчки; копія, видима іншим учасникам, не є приватним обхідним каналом ACL. За порівнюваного acceptance перша версія може лишатися поточною; для непорівнюваних несумісних замін після reconciliation обидві версії утворюють явний невирішений conflict без автоматично поточної версії. Точні metadata/ACL inheritance, lifecycle та wire model лишаються `OQ-0034`.

## Acceptance criteria

- corrupted chunk/manifest is rejected;
- interrupted transfer resumes without reaccepting unverified bytes;
- файл понад налаштовану межу лишається з metadata без автоматичного payload-fetch; підтверджена користувачем дія запускає завантаження, але не обходить чинну Space authorization;
- запис payload при заповненому диску завершується локальною помилкою й станом «не завантажено»; shared manifest та наявні remote replicas не змінюються;
- origin attachment commit є атомарним щодо recoverable staged payload, manifest reference та outbox intent: full-disk/crash до recoverable payload `MUST NOT` опублікувати manifest/operation, яка посилається на відсутні локальні bytes;
- inbound download failure `MUST NOT` unpin/collect будь-яку remote/retained copy; retry або local state change не є BlobStore GC proof;
- expired/leaked ticket fails according to policy;
- removal/rekey blocks future authorized fetch while documenting old-copy limits;
- GC never removes a blob with a live owner/pin and eventually reports collectable orphan state.
- concurrent incomparable authority-accepted replacement keeps both verified payload variants and their pins until a safe retention frontier under explicit policy, not merely until one peer authorizes or records a conflict resolution; GC cannot collect one variant while its accepted conflict reference is live. All peers show unresolved conflict without a winner, while a distinct revision/copy cannot silently overwrite a version or gain access outside Space policy. Invalid/revoked pending payloads are recoverable only under their applicable rights/retention policy, not projected as accepted versions.
- якщо два authority-accepted офлайн-рішення file conflict з несумісними результатами залишаються непорівнюваними, новий conflict зберігає обидва resolution payloads/references і їхні живі pins; локальне рішення однієї peer-групи не є достатнім proof для unpin/GC до встановлення застосовної безпечної retention frontier (`OQ-0034`).
- пізня правомірно прийнята несумісна гілка з непорівнюваним щодо рішення acceptance повторно відкриває conflict; payload старої гілки не втрачено через ранній GC. Поки pins утримують місце, клієнт помітно показує використання сховища, не видаляючи дані мовчки; точний warning threshold і safe frontier — `OQ-0034`.
- ревізія лишається під тим самим resource ID, тоді як копія отримує новий ID і окрему перевірку прав. Тест перевіряє, що жодна дія не змінює primary file мовчки та не надає зайвого доступу.

## Deferred

Digest/encryption algorithms, chunk size, manifest encoding, ticket format, replication/pinning defaults and erasure SLO.

OWASP перевірено 2026-09-20: [MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) вимагає захисту sensitive data на пристрої, [Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) — перевірки доступу до resource/action. Для VIDA fixture перевіряє захищене зберігання retained conflict payloads та відмову у Fetch/revision/copy після відкликання прав. OWASP не задає строку pin або safe retention frontier.
