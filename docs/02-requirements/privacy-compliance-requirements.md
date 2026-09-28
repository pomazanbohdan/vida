---
id: REQ-PRIV-001
status: approved-baseline
last_updated: 2026-09-22
source_refs:
  - ../../_bmad-output/planning-artifacts/research/legal-vida-gdpr-baseline-2026-09-22/research.md
---

# Privacy and GDPR baseline

| ID | Вимога |
|---|---|
| `REQ-PRIV-001` | Кожен processing flow `MUST` мати задокументовані purpose, data categories, legal basis, retention, recipients, transfers і controller/processor roles до production activation. |
| `REQ-PRIV-002` | VIDA `MUST` застосовувати privacy by design/default: data minimisation, найприватніші defaults, purpose limitation, storage limitation, encryption/pseudonymisation і least-privilege access. |
| `REQ-PRIV-003` | Product і operational procedures `MUST` підтримувати застосовні rights: notice, access, rectification, erasure, restriction, portability, objection та safeguards для automated decisions. |
| `REQ-PRIV-004` | Security controls `MUST` бути risk-based і доказовими; incident workflow `MUST` дозволяти оцінити breach, зафіксувати awareness time й виконати застосовне повідомлення authority не пізніше 72 годин. |
| `REQ-PRIV-005` | High-risk processing `MUST NOT` активуватися без DPIA; unresolved high residual risk `MUST` проходити prior consultation, якщо цього вимагає GDPR. |
| `REQ-PRIV-006` | Transfer EEA personal data за межі EEA `MUST` мати documented lawful safeguard, hosting/recipient inventory і transfer assessment. |
| `REQ-PRIV-007` | Local-only, direct peer, push, crash reporting, app-store analytics і Hosted Space `MUST` аналізуватися як окремі flows; E2EE/zero-knowledge `MUST NOT` використовуватися як заміна role/legal-basis analysis. |
| `REQ-PRIV-008` | Locale або country selector `MUST NOT` подаватися як GDPR compliance control. Targeting EU users `MUST` запускати EU applicability review незалежно від ціни застосунку чи місця розташування VIDA. |
| `REQ-PRIV-009` | На documentation stage legal entity/controller і store-account owner не призначені. Production processing, public store publication або paid service `MUST NOT` починатися, доки відповідальні controller/operator/store-account roles не визначені й не задокументовані. |

Точна legal entity, launch countries/locales, telemetry profile, DPO/EU representative, age policy, DPIA scope і transfer mechanism лишаються open legal decisions. Відкладення цих рішень на поточному етапі не скасовує їх як launch blockers.
