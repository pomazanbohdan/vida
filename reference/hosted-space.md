---
id: PROD-HOSTED-SPACE-001
status: approved
last_updated: 2026-09-25
source_refs:
  - ../../_bmad-output/planning-artifacts/prfaq-vida.md
decision_refs:
  - ADR-0013
---

# Опційний платний VIDA Hosted Space

## Прийнята продуктова модель

- Open-source/open-spec VIDA core `MUST` залишатися працездатним без підписки й без зовнішнього сервера: локальні дані, direct device/peer sync і базові Apps не залежать від VIDA Cloud.
- Платний сервіс `MUST` дозволяти користувачу зареєструватися, активувати план і явно прив'язати обраний Space до постійно доступного hosted node.
- Hosted node `MAY` поєднувати durable mailbox/relay, постійно доступну replica, backup/recovery і blob storage на кшталт S3. Кожна capability `MUST` бути видима й окремо описана в trust/retention policy.
- Hosted node `MUST` давати availability, fault-tolerance і multi-device convenience; він `MUST NOT` ставати прихованою обов'язковою залежністю open core.
- Subscription entitlement `MUST NOT` змінювати відкриті формати, блокувати portable data або забороняти fork/сумісну незалежну реалізацію задекларованого baseline.
- Hosted node не стає Owner або бізнес-authority автоматично. Його actor/device/service role, доступ до plaintext/keys і право виконувати logic `MUST` випливати з явного Space trust profile.

## Затверджена trust-модель

- Default для кожного підключеного Space — `zero-knowledge host`: relay/mailbox/encrypted blob без plaintext або Space keys.
- `managed replica` вмикає лише Owner явною окремою дією для конкретного Space; node отримує Space keys, може розшифровувати, індексувати й виконувати дозволену logic, а оператор входить у trust boundary.
- `hosted authority` не виникає з managed replica автоматично: конкретний App/process має окремо делегувати terminal acceptance за Core policy.
- Вимкнення managed replica `MUST` відкликати майбутній доступ, rotate affected key epochs і запустити визначений retention/deletion flow; воно не може відкликати вже розкриті plaintext або exports.

Один тариф `MAY` комбінувати профілі для різних Spaces, але UI `MUST` показувати різницю до активації.

## Browser modes

`ADR-0021` включає статичний local-first Web із browser storage та авторизованою direct-first peer sync; VIDA-operated Iroh relay є зашифрованим fallback після невдалої обмеженої прямої спроби, а не платним Hosted Space, mailbox або application server. Прямий Web шлях через Iroh/WebRTC custom transport ще потребує Release-1 proof gate. `ADR-0018` зберігає історію попереднього free-local-only/post-v1 рішення, яке в цій частині superseded. Опційний paid hosted browser/Space mode із server-backed sync, billing, retention і видаленням hosted replica/blobs лишається post-v1. Browser encryption/key custody і persistence/security contract є окремими Release-1 implementation gates.
