---
id: SPEC-VIDA-RELEASE-SECURITY-EVIDENCE
status: draft-decision-gated
companions:
  - security-evidence-plan.md
  - ../../../docs/02-requirements/platform-nfr.md
  - ../../../docs/02-requirements/privacy-compliance-requirements.md
  - ../../../docs/04-specifications/privacy-release-evidence.md
  - ../../planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md
sources: []
---

# Release 1 security threat model and evidence

## Why

VIDA має окремі вимоги до E2EE, ключів, відкликання прав, синхронізації та платформ, але перелік вимог або невиконаних fixtures не доводить захищеність релізу. Для Android, iOS, Windows і кожного активованого сервісу потрібна спільна модель загроз та перевірні докази без змішування security assurance з окремою privacy/legal матрицею.

## Capabilities

- **CAP-1**
  - **intent:** Команда підтримує модель загроз для кожної активованої межі довіри Release 1.
  - **success:** Версійний запис пов'язує активи, акторів, потоки, припущення, сценарії зловживання, захист і залишковий ризик із перевірним доказом; відсутня межа позначена gap, а не `pass`.
- **CAP-2**
  - **intent:** Кожен підтримуваний клієнт і активний сервіс проходить застосовні негативні security-перевірки.
  - **success:** Затверджені `NFR-SEC-001–005` та пов'язані архітектурні gates мають прив'язані до release build/profile результати; нерелевантний контроль має пояснене `not applicable`.
- **CAP-3**
  - **intent:** Відповідальний за реліз відрізняє план, виконаний тест, невдачу й прийнятий залишковий ризик.
  - **success:** Спільний manifest пов'язує кожне твердження з перевіреним build/profile, fixture, результатом і артефактом; дата, перевіряльник, статус і винятки — запропоновані поля evidence schema, не вже затверджений формат. Немає security `pass` лише з документа або зеленого вузького тесту.
- **CAP-4**
  - **intent:** Зміни архітектури, платформи чи активованого потоку не успадковують застарілий висновок без перевірки.
  - **success:** Для змінених меж і controls записано повторну оцінку застосовності та оновлені докази до нового security claim.

## Constraints

- `NFR-SEC-001–005` і implementation gates у [плані доказів](security-evidence-plan.md) не послаблюються цим документом; невиконаний fixture не є доказом.
- Для Android/iOS застосовується релевантний OWASP MASVS/MASTG, для активних web/API поверхонь — ASVS; Windows має окремий platform profile та спільні VIDA fixtures, а не вигадану mobile-сертифікацію.
- Evidence не містить секретів, recovery material, прихованих зв'язків між Personas чи payload; privacy/legal launch gate лишається в adopted [privacy-матриці](../../../docs/04-specifications/privacy-release-evidence.md).
- Обов'язковість зовнішнього pentest і окремого crypto-review залишається `OQ-0081`; документ не затверджує vendor, алгоритм, severity threshold або hosted trust policy.

## Non-goals

- Ця чернетка не є завершеною threat model, виконаним pentest, crypto-review, доказом GDPR compliance чи дозволом публікувати Release 1.
- Не обирає криптографічний профіль, не закриває `OQ-0024`, `OQ-0033`, `OQ-0053`, `OQ-0078` або `OQ-0081`.

## Success signal

Перед security claim релізний manifest показує для кожної активної межі довіри актуальну threat model, застосовні контролі, виконані позитивні й негативні fixtures, артефакти та відкриті ризики. План і неперевірені сценарії в [companion](security-evidence-plan.md) — лише вхід до цієї перевірки, не її результат.

## Open Questions

- `OQ-0081`: чи є незалежний mobile pentest та окремий protocol/implementation crypto-review з remediation/retest обов'язковими до публічного Release 1?
- Хто може приймати залишковий security risk і за якою severity policy, якщо юридичний controller/operator ще не визначений (`OQ-0078`)?
