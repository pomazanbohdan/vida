---
id: SPEC-VIDA-RIGHTS-RECONCILIATION-PROOF
status: draft-decision-gated
companions:
  - proof-options-and-fixtures.md
  - ../../../docs/02-requirements/access-control-requirements.md
  - ../../../docs/04-specifications/space-membership-contract.md
  - ../../../docs/03-architecture/decisions/ADR-0003-offline-revocation.md
  - ../../planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md
sources: []
---

# Shared-Space rights reconciliation proof

## Why

Учасник shared Space має користуватися синхронізованими даними офлайн, але інший Owner може відкликати його доступ. VIDA вже затвердила єдиний семиденний строк узгодження прав; клієнтам потрібен міжплатформний доказ, який не перетворить перезапуск або відкат годинника на фальшиве продовження доступу.

## Capabilities

- **CAP-1**
  - **intent:** Клієнт shared Space розпізнає успішне узгодження актуального control state з чинною Space authority.
  - **success:** З'єднання з relay/peer, локальна активність і повтор старого підписаного head не поновлюють семиденний інтервал.
- **CAP-2**
  - **intent:** Учасник читає вже синхронізовані дані shared Space офлайн до доведеного спливу єдиного семиденного інтервалу.
  - **success:** Сама втрата зв'язку до спливу не приховує відкритий документ і не вимагає окремого online-open для нового синхронізованого `critical` вмісту.
- **CAP-3**
  - **intent:** Клієнт припиняє всі керовані shared reads після отриманого effective revocation або спливу інтервалу без нового proof.
  - **success:** UI, search, recents, notifications, export, runtime/API, local projections і plugins закриті, включно з уже відкритим вмістом; restart не відновлює читання.
- **CAP-4**
  - **intent:** Після shared-read lock користувач не втрачає pending work і може створити нову чернетку без читання заблокованого content.
  - **success:** Draft durable і pending; він не відкриває старих даних та не стає authority-accepted до перевірки прав; age alone не стирає candidate.
- **CAP-5**
  - **intent:** Власник Personal Space зберігає окреме правило офлайн-читання.
  - **success:** Його синхронізовані дані залишаються доступними без shared seven-day proof, доки local identity context активний, якщо немає окремого local lock/logout.

## Constraints

- `rightsReconciliationInterval = 7 діб` однаковий для всіх shared Apps, ролей включно з Owner та класів ризику. Старі 24-годинні/critical профілі скасовані.
- Лише actor, якого авторизує Space authority policy, може підтвердити поновлення control state. Отримане effective revocation блокує читання негайно, незалежно від залишку семи діб.
- Restart, wall-clock rollback, old-head replay, local activity та reachability вузла чи relay не є proof. Підпис підтверджує джерело повідомлення, але сам не доводить час, що минув після нього.
- Семиденний read lock не є строком життя offline mutation candidate; вже скопійовані plaintext, screenshots і exports не можна дистанційно відкликати.
- Без доведеного cross-reboot elapsed-age mechanism не заявляти точну семиденну гарантію на Android/iOS/Windows; рішення при невідомому віці потребує `OQ-0053`.

## Non-goals

- Ця чернетка не обирає receipt wire format, clock/anti-rollback primitive, unsupported-device policy чи cryptographic suite.
- Не змінює затверджені семиденний строк, offline candidate policy або Personal Space виняток.

## Success signal

Сценарії [RRP-F01–F19](proof-options-and-fixtures.md) мають однаково перевірити всі платформи й керовані read surfaces, межу рівно 7 діб, replay/restart/rollback та негайне revocation. Production proof лишається блокованим до рішення `OQ-0053` і виконання цих тестів.

## Open Questions

- `OQ-0053`: якщо пристрій перезапустився офлайн і не може надійно довести вік останнього rights proof, чи блокувати shared reads до зв'язку навіть тоді, коли реальні 7 діб могли ще не минути?
- `OQ-0053`: чи вважаємо скомпрометовану ОС/root/jailbreak/reversible VM snapshot поза гарантією сумісного клієнта, зберігаючи rollback-захист для звичайного немодифікованого пристрою?
