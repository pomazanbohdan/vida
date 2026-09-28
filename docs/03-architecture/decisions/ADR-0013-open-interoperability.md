---
id: ADR-0013
status: accepted
last_updated: 2026-09-20
source_refs:
  - ../../../_bmad-output/planning-artifacts/research/technical-vida-open-protocol-specifications-2026-09-20/research.md
  - ADR-0005-iroh-transport-foundation.md
decision_refs:
  - ADR-0005
  - ADR-0007
  - ADR-0008
supersedes: []
superseded_by: []
---

# ADR-0013: Відкриті контракти для незалежних клієнтів, вузлів і застосунків

## Контекст

VIDA є open-core супер-апом із власними протоколами поверх Iroh, декларативними AppPackages і зовнішніми репозиторіями. Відкритий код ядра сам по собі не гарантує, що сторонній розробник зможе створити сумісний клієнт, вузол або пакет. Користувач підтвердив обидва напрями незалежної розробки та повну прозорість контрактів.

## Рішення

1. VIDA `MUST` публікувати нормативні версійні специфікації достатні для незалежної реалізації сумісних клієнтів **і** вузлів, включно з поведінкою, wire-форматом, identity/authority, синхронізацією, помилками та правилами сумісності. Незалежна реалізація не потребує внутрішнього VIDA waiver для альтернативного first-party core; її перевіряють відкриті conformance-тести. Той самий принцип `MUST` охоплювати schema-driven Apps, керовані extension packages (не довільний host code), repository metadata і дозволені runtime/host APIs.
2. Кожна заява про сумісність `MUST` називати версійний profile із замкненим списком обов'язкових capabilities і залежностей. Нормативні специфікації, машиночитані схеми, позитивні та негативні тестові вектори, набір conformance-тестів і результати його виконання для офіційних релізів `MUST` бути доступні публічно. Жоден прихований контракт або платний обов'язковий extension `MUST NOT` бути умовою відкритого baseline. Стороння реалізація `MUST` мати документований шлях пройти той самий conformance-набір без платного VIDA tenant/marketplace account; тестування `MUST` охопити пари незалежний клієнт↔незалежний вузол, а не тільки пари з офіційним ПЗ.
3. Зміни контрактів `MUST` мати публічну історію, статус (proposal/experimental/accepted/deprecated), версіонування та правила міграції. Процес внесків `MUST` дозволяти зовнішні пропозиції та пояснювати їх прийняття/відхилення. Специфікація App-розширень `MUST` охоплювати hook/effect/entitlement semantics, що впливають на спостережувану поведінку. Сумісний package не отримує автоматично publisher trust, Space grants/consent, network egress чи місце в керованому marketplace: це окремі security/policy перевірки.
4. Платні модулі, hosting і керований marketplace `MAY` існувати поруч з відкритим ядром; їхні реалізації не проголошено open source цим рішенням. Однак інтерфейси, необхідні для заявленої міжоперабельності та безпеки, `MUST` бути описані публічно. Незалежна реалізація не означає безкоштовний доступ до чужої hosted-інфраструктури.
5. Це рішення **не** обирає новий wire codec, ліцензію, орган стандартизації або Nostr як транспорт. Прийняті `vida/<capability>/<major>` та VIDA envelope з ADR-0005 залишаються чинними. Можливий Nostr bridge — окремий optional adapter після сценарію й privacy review.

## Наслідки та перевірка

- Сторонній клієнт і сторонній вузол можуть без приватної документації або платного VIDA акаунта реалізувати один опублікований VIDA protocol profile та пройти його публічні byte-exact, behavior і negative-security fixtures одне з одним.
- Сторонній App/плагін із зовнішнього репозиторію може реалізувати опублікований package/runtime contract і пройти ті самі тести, що bundled package; підключення не надає доступу до Space автоматично.
- Офіційний release manifest посилається на immutable версії специфікацій і conformance suite; функція без відкритого нормативного контракту не може заявляти незалежну сумісність.
- Ліцензія для `vida-core` та іншого open-source коду, текстів, схем, fixtures і reference code, patent/copyright IPR, правила внесків і governance-процедура — `OQ-0055`; до вирішення ліцензії «open-core» є ціллю, а не підтвердженим правовим статусом. Canonical codec і низькорівневі wire деталі — чинний `OQ-0028`.

## Джерела для форми процесу (не автоматично прийняті протоколи)

[Nostr NIPs](https://github.com/nostr-protocol/nips), [Matrix MSC process](https://spec.matrix.org/proposals/), [AT Protocol Lexicon](https://atproto.com/specs/lexicon). Порівняння й ризики — у [дослідженні](../../../_bmad-output/planning-artifacts/research/technical-vida-open-protocol-specifications-2026-09-20/research.md).
