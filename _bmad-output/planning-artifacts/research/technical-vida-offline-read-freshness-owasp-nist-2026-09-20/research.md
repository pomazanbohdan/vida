---
title: 'VIDA: offline-read freshness, OWASP and NIST'
type: technical
topic: 'Безпека читання синхронізованого Space без зв’язку після відкликання membership'
decision: 'Визначити, чи потрібен ліміт віку control head для offline-read та яким має бути його профіль'
source: 'official OWASP and NIST web documentation'
status: complete
preset: quick
validation: normal
created: '2026-09-20'
updated: '2026-09-20'
claims_verified: 2
claims_unverified: 2
---

# VIDA: offline-read freshness, OWASP and NIST

## Висновок для рішення

У перевірених матеріалах OWASP **немає універсального строку**, після якого офлайн-пристрій повинен втратити доступ до вже синхронізованих даних. OWASP розглядає доступність чутливих даних після завершення сесії як слабкість і рекомендує очищувати токени та керовані кеші; таймаути сесії добираються за ризиком [1][2]. NIST SP 800-63B-4 встановлює орієнтири для **повторної автентифікації активної сесії**, а не для свіжості Space membership на пристрої без мережі [3].

Отже, для VIDA потрібен окремий продуктово-безпековий параметр `maxOfflineReadControlAge` (назва кандидатна), а не перенесене з OWASP/NIST число. Його мета — обмежити час показу старих даних **у сумісному клієнті** без свіжого control head; це не гарантія дистанційного стирання чи захист від власника зміненого пристрою. На момент дослідження строки були `OQ-0051`; пізніше їх затверджено, див. post-decision update.

## Що кажуть джерела

1. OWASP MASWE-0024 описує як вразливість доступність чутливих даних після завершення сесії та рекомендує припиняти сесію на server і client, очищувати tokens, cached personal data, memory/on-screen content, встановлювати risk-appropriate inactivity/absolute timeouts [1]. Застосування цього патерна до `MemberRemoved` у VIDA є архітектурною аналогією: після отримання підтвердженого відкликання клієнт перестає показувати керовані дані Space. У самому контролі немає кількості годин/днів для раніше синхронізованого офлайн-Space [1].
2. OWASP Session Management Cheat Sheet вимагає server-side enforcement таймаутів web-сесії та попереджає, що client-only time reference можна змінити. Значення залежать від ризику застосунку; наведені там приклади для web-сесій не є нормативом для local-first offline-read [2].
3. Чинний NIST SP 800-63B-4 (липень 2025) рекомендує загальний строк до reauthentication не більше 30 днів для AAL1 і 24 годин для AAL2, для AAL2 — inactivity не більше години [3]. Це **session authentication**, не перевірка того, чи людина все ще є учасником конкретного Space. Біометричне розблокування локального застосунку теж не підтверджує поточний membership у відключеному Space — це висновок для VIDA, а не текст NIST.
4. MASVS-STORAGE-1 вимагає безпечно зберігати чутливі дані [4]; він не задає строку offline-read. Шифрування локального кешу й зберігання ключів зменшують ризик витоку з пристрою, але не передають йому новий `MemberRemoved` без зв’язку — висновок для VIDA з межі доступності мережі.

## Межі гарантій на прикладах

| Ситуація | Що VIDA може гарантувати | Чого не може гарантувати |
|---|---|---|
| Owner видалив учасника; authority прийняв операцію | За власним контрактом VIDA відразу відхиляти нові запити/операції за відкликаними grants і не видавати нові keys | Стерти вже відомий plaintext або копії на чужому пристрої |
| Видалений пристрій під’єднався й отримав control head | Сумісний клієнт закриває Space, прибирає керовані екрани/ключі й запускає очищення кешу [1] | Повернути зроблені раніше скриншоти/експорти |
| Видалений пристрій весь час офлайн | Майбутній локальний freshness limit може змусити **сумісний** клієнт заблокувати Space після строку | Authority не може доставити revocation офлайн; client-only таймер не є надійною межею проти модифікованого клієнта [2] |

## Рекомендація для VIDA — не стандарт OWASP

На момент початкового дослідження пропонувалося розрізняти (a) session inactivity/reauthentication; (b) authority revocation і key epoch для майбутнього доступу; (c) membership/control-head freshness для offline-read. Історичні `offline mutation acceptance windows` 30 днів/7 днів/24 години належали окремому механізму прийняття записів, не читання кешу; вони згодом скасовані (див. останнє оновлення нижче).

Кандидат на момент написання дослідження: Personal Space — локальне читання без мережі без membership-freshness limit; shared standard — до 7 днів від останнього підтвердженого control head; protected — до 24 годин; critical — online validation для відкриття. Це наш risk-budget, а не вимога OWASP/NIST. За закінчення строку сумісний клієнт блокує Space до синхронізації; після отримання revocation — блокує негайно. Подальше рішення зафіксовано в post-decision update нижче.

## Відкриті питання

- Строки shared `standard/protected/critical` — на момент дослідження `OQ-0051`; затверджено пізніше того ж дня, див. post-decision update.
- Яким способом конкретна client platform перевіряє давність підписаного control head і не відновлює доступ після перезапуску або зміни годинника? Це implementation/security test, не вирішується самим вибором строку; зараз `OQ-0053`.

## Джерела

| № | Що підтримує | Видавець | Опубліковано | Перевірено | Впевненість |
|---|---|---|---|---|---|
| [1] | Недоступність sensitive data після session termination, cache/token cleanup | [OWASP MASWE-0024](https://mas.owasp.org/MASWE/MASVS-AUTH/MASWE-0024/) | undated | 2026-09-20 | medium: direct primary source |
| [2] | Server-side session expiry, risk-based timeout, client-only timer weakness | [OWASP Session Management Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html) | undated | 2026-09-20 | medium: direct primary source |
| [3] | AAL1/AAL2 session reauthentication periods and their scope | [NIST SP 800-63B-4](https://pages.nist.gov/800-63-4/sp800-63b/aal/) | 2025-07 | 2026-09-20 | high: final primary standard |
| [4] | Secure storage of mobile sensitive data | [OWASP MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) | undated | 2026-09-20 | medium: direct primary source |

## Актуальність

Для сторінок OWASP без дати публікації повторно перевірити URL і поточний текст перед security design freeze; планова дата — 2027-03-20. NIST SP 800-63B-4 повторно перевірити перед призначенням AAL або власних session timeout; консервативний орієнтир за 24-місячним вікном від місяця публікації — 2027-07-01. Числовий профіль VIDA не походить із цих джерел; статус його затвердження наведено нижче.

## Post-decision update — 2026-09-20

Історичне рішення на цю дату: користувач затвердив для shared Space `standard` 7 діб, `protected` 24 години, `critical` online validation. **Цей абзац більше не є чинною вимогою:** пізніше того ж дня користувач скасував диференційовані строки offline-read (див. наступне оновлення). OWASP/NIST не є джерелом чисел VIDA.

## Superseding decision — 2026-09-20

Користувач скасував різні строки читання залежно від критичності: уже відкритий critical документ після втрати зв’язку поводиться як завжди. Для синхронізації/узгодження прав має бути один сталий інтервал. Його числове значення та поведінка, коли інтервал сплив, а authority недоступний, лишаються відкритими (`OQ-0051`). Отримане effective revocation застосовується негайно. Нормативне формулювання тепер містять [ADR-0003](../../../../docs/03-architecture/decisions/ADR-0003-offline-revocation.md), `REQ-ACL-016`, `SPEC-SPACE-MEMBERSHIP-001` та `AD-16`; окремі строки прийняття offline *змін* не скасовано без уточнення (`OQ-0054`).

## Subsequent offline-mutation decision — 2026-09-20

Після уточнення користувач визначив: строк очікування офлайн-зміни необмежений, якщо чинні права на відповідний Space/об'єкт/дію ще є, або це власний Personal Space. Отже попередні 30 днів / 7 днів / 24 години для **mutation candidates** скасовані; це не гарантія автоматичного прийняття чи безстрокової дії grant. Потрібні acceptance-time revalidation і перевірка конфліктів; нормативне рішення в [ADR-0003](../../../../docs/03-architecture/decisions/ADR-0003-offline-revocation.md), `REQ-ACL-018` і `AD-18`. Це уточнення **не** визначає числовий інтервал offline-read rights reconciliation `OQ-0051`.
