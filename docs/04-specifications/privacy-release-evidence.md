---
id: SPEC-PRIVACY-RELEASE-EVIDENCE-DRAFT
status: draft
implementation_status: evidence-not-produced
last_updated: 2026-09-23
requirement_refs:
  - ../02-requirements/privacy-compliance-requirements.md
source_refs:
  - ../../_bmad-output/planning-artifacts/prds/prd-vida-2026-09-22/prd.md
---

# Privacy release evidence matrix

Це трасування **які докази треба зібрати**, а не висновок про GDPR-compliance або дозвіл на публікацію. Жоден рядок не вважається `pass` без збереженого артефакту, відповідальної особи та перевірки застосовності. Конкретний legal controller/operator/store-account owner ще не визначений; це окремий launch blocker за `REQ-PRIV-009`.

| Requirement | Обов'язковий перевірний артефакт | Негативна перевірка / умова gate |
|---|---|---|
| `REQ-PRIV-001` | Інвентар кожного активованого processing flow: purpose, categories, legal basis, retention, recipients, transfers і roles; версія та відповідальний за запис. | Flow без будь-якого поля не переходить у production. |
| `REQ-PRIV-002` | Privacy-defaults matrix по Persona, Space, Contacts, diagnostics, push і hosted режиму; threat/data-flow review та докази мінімізації, шифрування і least privilege. | Новий optional sharing/telemetry не вмикається мовчки після update. |
| `REQ-PRIV-003` | Rights runbook з каналом запиту, ідентифікацією заявника, строками, межами E2EE/peer/offline, test cases для notice/access/rectification/erasure/restriction/portability/objection та applicable automated-decision safeguards. | Запит не оголошується виконаним, якщо ще існують недоставлені операції або винятки, які користувачеві не пояснили. |
| `REQ-PRIV-004` | Risk-based security assessment, incident register template й breach runbook із фіксацією awareness time, оцінкою ризику та умовним маршрутом повідомлення компетентному authority у застосовний строк. | Відсутність зафіксованого awareness time або рішення про застосовність 72-годинного повідомлення блокує gate. |
| `REQ-PRIV-005` | Скринінг high-risk flows; DPIA та, коли потрібно, запис prior consultation і статусу residual risk. | High-risk flow не активується за відсутності необхідної DPIA/consultation. |
| `REQ-PRIV-006` | Реєстр hosting/recipients/EEA transfers і для кожного застосовного transfer — lawful safeguard та transfer assessment. | Невідомий recipient/region або непідтверджений safeguard блокує відповідний flow. |
| `REQ-PRIV-007` | Окремі flow records для local-only, direct peer, push, crash reporting, app-store analytics і Hosted Space; для необраного майбутнього Hosted Space — `not active`, не `pass`. | E2EE/zero-knowledge не замінює аналіз purpose, roles, recipients та legal basis. |
| `REQ-PRIV-008` | Запис EU applicability review за фактичним targeting/distribution, незалежний від locale/country selector. | Вибір країни користувачем сам по собі не позначає продукт GDPR-compliant. |
| `REQ-PRIV-009` | Підписане призначення legal controller, оператора процесів і власника store accounts до production processing/public publication/paid service. | Порожня роль блокує відповідний launch event навіть якщо технічні тести зелені. |

## Flow-level gate

1. `not-in-scope` дозволено лише з мотивованим записом застосовності; `not-active` — лише для flow, який справді вимкнений і не отримує персональних даних.
2. `pass` потребує посилань на версійні артефакти, особу-рецензента, дату, область дії та відкриті винятки; цей документ таких доказів не створює.
3. Зміна package, host binary, backend, push/diagnostics setting або distribution region, яка змінює processing flow, повертає відповідні рядки до review перед активацією.
4. Публічний Release 1 не проходить privacy gate, доки всі фактично активні flows не мають застосовних доказів `REQ-PRIV-001–009` і не виконано launch-role gate.
