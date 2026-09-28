---
id: REQ-LOCALIZATION-001
status: approved
last_updated: 2026-09-22
source_refs:
  - ../../_bmad-output/planning-artifacts/prfaq-vida.md
  - ../../_bmad-output/planning-artifacts/research/market-vida-localization-selection-2026-09-22/research.md
decision_refs: []
---

# Localization requirements

Release 1 підтримує **18 мов** через **21 початковий locale profile**. Українська обов'язкова; російська не входить до підтримуваного набору.

| Мова | Release-1 locale profiles |
|---|---|
| Українська | `uk` |
| English | `en` |
| Español | `es-419`, `es-ES` |
| Português | `pt-BR`, `pt-PT` |
| हिन्दी | `hi-IN` |
| Bahasa Indonesia | `id-ID` |
| العربية | `ar` |
| Deutsch | `de-DE` |
| Français | `fr-FR` |
| 日本語 | `ja-JP` |
| 한국어 | `ko-KR` |
| Türkçe | `tr-TR` |
| 中文 | `zh-Hans`, `zh-Hant` |
| Polski | `pl-PL` |
| Italiano | `it-IT` |
| Română | `ro-RO` |
| Čeština | `cs-CZ` |
| Nederlands | `nl-NL` |

## Нормативні вимоги

| ID | Вимога |
|---|---|
| `REQ-L10N-001` | Android, iOS і Windows Release-1 clients `MUST` постачати всі 21 locale profiles для shell UI, bundled Messenger, Notes/Knowledge, Projects, Contacts, Files, Forums, Calls і system/error flows. |
| `REQ-L10N-002` | Language selection `MUST` бути окремим від country/region selection; country не може автоматично змінювати Persona, legal regime або data location. |
| `REQ-L10N-003` | Locale resolution `MUST` використовувати BCP 47 та deterministic fallback. Відсутній переклад `MUST NOT` fallback-итися на російську. |
| `REQ-L10N-004` | Arabic profile `MUST` пройти RTL layout/navigation/icon-direction tests. Japanese, Korean, Simplified Chinese і Traditional Chinese `MUST` пройти font, glyph, wrapping, input-method і search-tokenization tests. |
| `REQ-L10N-005` | Plurals, grammatical variables, dates, time, numbers, units і sorting `MUST` використовувати locale-aware data/ICU rules; UI strings `MUST NOT` складатися конкатенацією неперекладних фрагментів. |
| `REQ-L10N-006` | CI `MUST` перевіряти missing/obsolete keys, placeholder compatibility, plural completeness, overflow через pseudo-localization і representative screenshot/accessibility fixtures. |
| `REQ-L10N-007` | Store listing, onboarding, privacy/security notices, recovery, permissions і destructive-action warnings `MUST` мати перевірений переклад кожного supported locale до public release. |
| `REQ-L10N-008` | AppPackage `MAY` підтримувати підмножину мов, але `MUST` оголошувати locales і fallback у manifest; host `MUST` чітко показувати fallback language. |
| `REQ-L10N-009` | Додавання нової мови `MUST` бути data/config change без fork доменної логіки; переклад `MUST NOT` змінювати schema IDs, permission IDs, operation IDs або signatures. |

## Межі рішення

- `zh-Hans` і `zh-Hant` означають мовну готовність, але не автоматичний запуск у mainland China; distribution/compliance вирішується окремо.
- Regional English, Arabic, French та інші variants можуть додаватися через fallback без зміни переліку мов.
- Конкретна canonical source locale, translation workflow і review ownership визначаються implementation plan.
