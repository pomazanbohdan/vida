---
id: ADR-0019
status: superseded
last_updated: 2026-09-22
source_refs:
  - ../../../_bmad-output/planning-artifacts/prfaq-vida.md
  - ../windows-client-options.md
decision_refs:
  - ADR-0014
  - ADR-0015
  - ADR-0018
supersedes:
  - ADR-0015
superseded_by:
  - ADR-0020
---

# ADR-0019: Android/iOS у Release 1; Windows після Release 1

> Superseded by `ADR-0020`: Flutter Windows входить у Release 1; окрема WinUI 3/C# реалізація можлива після Release 1.

## Контекст

ADR-0015 вимагав Android, iOS і Windows одночасно у v1. Під час Internal FAQ користувач змінив sequencing: Android/iOS мають спільну Flutter-логіку й окремі platform targets; public Windows client планується після Release 1. Якщо Flutter Windows дозволить окреме пакування зі значним UI reuse, він є першим кандидатом; WinUI 3/C# відкладається далі.

## Рішення

- Public Release 1 `MUST` включати Android та iOS clients із Flutter presentation і спільним Rust core.
- Android та iOS `MUST` мати окремі specifications, permissions/lifecycle adapters, signing, store і conformance profiles у монорепозиторії.
- Public Windows client `MUST NOT` бути gate для Release 1 і планується наступним release increment.
- Перший Windows candidate `SHOULD` бути Flutter Windows, якщо prototype доведе окреме packaging, Rust binding, accessibility, keyboard/menu/tray, startup і memory без неприйнятної platform divergence.
- WinUI 3/C# `MAY` бути наступною native shell після Release 1; його не слід будувати паралельно без доказу потреби.
- Pre-v1 Windows work `MAY` бути лише bounded feasibility/protocol conformance і `MUST NOT` відбирати critical-path capacity Android/iOS.

## Наслідки

- ADR-0015 superseded щодо одночасної Windows-поставки; Flutter Android/iOS decision лишається чинним.
- Повноцінні Windows UX-вимоги не скасовані, а перенесені до Windows release gate.
- Browser modes лишаються post-v1 за `ADR-0018`; Windows postponement не робить browser заміною desktop client.
