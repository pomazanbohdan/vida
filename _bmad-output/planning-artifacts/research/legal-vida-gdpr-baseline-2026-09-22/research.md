---
title: "VIDA GDPR baseline for EU availability"
date: 2026-09-22
status: researched
---

# Висновок

Мова UI або country selector не визначають compliance. GDPR застосовується до EU establishment, а також до non-EU organisation, яка цілеспрямовано пропонує товари/послуги, зокрема безплатні, людям у ЄС або моніторить їхню поведінку. Тому EU launch потребує процесу compliance незалежно від того, де розташована команда VIDA.

# Мінімальний baseline

1. Для кожного processing flow визначити controller, joint controller, processor і sub-processor; E2EE або zero-knowledge не скасовує аналіз ролей.
2. Вести data inventory/record of processing: purpose, data categories, recipients, retention, legal basis і transfers.
3. Реалізувати privacy by design/default: minimisation, найприватніші defaults, encryption/pseudonymisation, обмежений доступ і retention.
4. Підтримати rights flows: information, access, rectification, erasure, restriction, portability, objection та review automated decisions, де застосовно.
5. Застосовувати risk-based technical/organisational security і documented incident response; notification authority may be required within 72 hours after awareness of a qualifying breach.
6. Проводити DPIA до high-risk processing; якщо residual high risk не знижено, потрібна prior consultation із supervisory authority.
7. Для передач із EEA назовні визначити applicable mechanism: adequacy, SCC або інший дозволений safeguard.
8. Окремо аналізувати local-only, direct peer sync, push, crash reporting, store analytics і future Hosted Space: у них різні recipients, purposes і roles.

# Офіційні джерела

- European Commission, [Who the GDPR applies to](https://commission.europa.eu/law/law-topic/data-protection/reform/rules-business-and-organisations/application-regulation/who-does-data-protection-law-apply_en)
- European Commission, [Principles of the GDPR](https://commission.europa.eu/law/law-topic/data-protection/information-business-and-organisations/principles-gdpr_en)
- European Commission, [Obligations](https://commission.europa.eu/law/law-topic/data-protection/information-business-and-organisations/obligations_en)
- EDPB, [Controller or processor](https://www.edpb.europa.eu/sme/learn-the-basics/data-controller-or-data-processor_en)
- EDPB, [Be compliant](https://www.edpb.europa.eu/sme/be-compliant/be-compliant_en)
- European Commission, [International data transfers](https://commission.europa.eu/law/law-topic/data-protection/international-dimension-data-protection/rules-international-data-transfers_en)

# Не вирішено

- legal entity та establishment VIDA;
- точні launch countries і top-10 locales;
- processing inventory і controller/processor matrix;
- telemetry/crash-reporting profile;
- DPO/representative necessity;
- DPIA scope;
- transfer mechanism і hosting regions;
- age/children policy та local national requirements.
