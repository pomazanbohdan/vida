---
title: 'Technical research: Vida identity modes'
type: technical
topic: 'Vida identity modes'
decision: 'Define autonomous anonymous, federated-node and public identity architecture'
source: native-web-research
status: complete
preset: standard
validation: normal
verified_claims: 8
unverified_claims: 0
created: 2026-09-18
updated: 2026-09-18
---

# Technical research: Vida identity modes

**Decision this research serves:** визначити identity-архітектуру Vida для повністю автономного anonymous mode, прив'язки до federated node, public identity, багатьох пристроїв, recovery та міграції.

## Executive summary

Vida не повинна мати поле `AccountType = Anonymous | Federated | Public`. Докази підтримують іншу модель: **ідентичність, хостинг, discoverability, verification, device trust і authorization є незалежними осями**.

Рекомендована основа — локальний `Vault`, який містить одну або кілька криптографічних `Persona`. Persona може не мати жодного сервера, мати один чи кілька `ServiceBinding` до federated nodes, а також за окремою згодою отримати `PublicProfile` та `AliasBinding`. Anonymous Persona використовує окремі pairwise/contextual keys без публічного батьківського ідентифікатора.

Ключовий наслідок: `Public` — не третій тип акаунта, а opt-in публічна проєкція Persona. `Federated` — не власник особи, а сервісна прив'язка для availability, mailbox, sync, address і policy. `Autonomous` — повноцінний режим без реєстрації, але з чесною відповідальністю користувача за backup/recovery.

Найбільший ризик — кореляція: після явного зв'язування anonymous Persona з public Persona неможливо гарантувати, що зовнішні спостерігачі забудуть цей зв'язок. Тому merge/link має бути окремою дією з незворотними наслідками, а не автоматичним upgrade.

## Recommended decision

| Decision | Required architectural action | Evidence |
|---|---|---|
| Identity modes are independent axes, not account types | Заборонити `AccountType = Anonymous/Federated/Public` як canonical domain field | §§1, 6; [1][2][4][7] |
| `Persona` is the stable application identity | Не використовувати phone/email/handle/node account/`EndpointId` як root ID | §§1, 3; [1][2][3][7][8] |
| Anonymous identity is contextual | Створювати pairwise/contextual Persona і не розкривати on-wire parent link | §2; [4][5][6] |
| Federation is a revocable service relationship | Моделювати node registration як `ServiceBinding` із contract for export/migration | §3; [7][8][9] |
| Publicity is explicit projection | Публікувати через `PublicProfile` + `AliasBinding`; попереджати про незворотну кореляцію | §§4, 7; [4][10][11][12] |
| Devices never equal the person | Використовувати per-device keys і signed `DeviceGrant`; revoke з ротацією affected keys | §5; [14][15][16][17][18] |

Вибір DID method, alias registry і конкретної криптосистеми відновлення слід винести в окремі ADR та перевірки прототипів після прийняття цієї моделі.

## 1. Незалежні шари identity

AT Protocol відокремлює persistent DID, mutable handle і PDS location; handle перевіряється двостороннім зв'язком із DID [1][2]. Nostr також вимагає зберігати первинне посилання на public key, а не на змінний NIP-05 address [3]. W3C DID Core прямо попереджає, що глобальні ідентифікатори створюють correlation risk, і рекомендує pairwise identifiers, коли публічна кореляція не потрібна [4].

Звідси випливає запропоноване розділення:

| Сутність | Призначення | Мережева видимість |
|---|---|---|
| `LocalVault` | Локальний контейнер персон, пристроїв і recovery material | Ніколи не передається як identity |
| `Persona` | Криптографічний суб'єкт взаємодії | Лише в обраному scope |
| `ControllerState` | Поточні controller/recovery keys і versioned history | За потреби перевірки control |
| `DeviceGrant` | Дозволений пристрій, його signing/key-agreement keys і статус | Лише учасникам відповідного trust context |
| `ServiceBinding` | Зв'язок Persona з node account, mailbox, quota та address | Відомий обраному node і peers за policy |
| `AliasBinding` | Людинозрозумілий змінний address/handle | За discoverability policy |
| `Credential` | Підтверджений телефон, email, організація або інша claim | Розкривається вибірково |
| `PublicProfile` | Публічна проєкція Persona для directory/web/search | Публічна за явною згодою |
| `EndpointBinding` | Зв'язок Device з Iroh `EndpointId` та transport metadata | Транспортний рівень |

`Phone`, `email`, username, domain і node-local account ID не повинні ставати кореневим `PersonaId`.

```text
LocalVault
  └─ Persona ─┬─ ControllerState ──authorizes──> DeviceGrant ──> EndpointBinding
              ├─ ServiceBinding[] ─────────────> federated nodes
              ├─ AliasBinding[] ───────────────> exact/public addresses
              ├─ Credential[]
              └─ PublicProfile? ───────────────> directory/web/search
```

### UX presets over independent axes

| Сценарій | Ідентичність | Hosting | Discoverability | Відновлення за замовчуванням |
|---|---|---|---|---|
| Autonomous anonymous | Pairwise/contextual Persona | none; optional opaque relay | invitation/exact link | trusted device або recovery package |
| Federated private | Stable Persona + `ServiceBinding` | provider/self-hosted node | invitation або exact address | device/recovery package; optional node assistance |
| Public | Stable Persona + `PublicProfile` | будь-який compatible hosting | directory/public/web | controller recovery + visible key history |

Presets спрощують onboarding, але не зливають identity, hosting і publicity в один тип акаунта.

## 2. Autonomous anonymous mode

SimpleX демонструє робочу модель без network-wide user ID: контакт встановлюється invitation link, а сервери обслуговують окремі queues без user authentication [5]. Його Incognito Mode приховує вибраний profile і генерує нове випадкове ім’я profile для кожного нового контакту [6]. W3C DID Core підтверджує, що pairwise identifiers мають бути унікальними для відносин і не повинні повторно використовувати ті самі verification methods [4].

Для Vida autonomous mode означає:

- створення encrypted `LocalVault` і Persona keys без телефону, email або node registration;
- локальну роботу Messenger, Notes/Knowledge і Projects/Tasks;
- контакт через одноразовий invite, QR або переданий contact card;
- direct/LAN/P2P sync, коли peers доступні;
- optional relay або availability node, який бачить лише transport/service capability, але не стає identity authority;
- відсутність public directory entry та network-wide handle;
- recovery лише через trusted device або encrypted recovery package.

Для найвищого рівня приватності використовуйте окремі ключі контролера й пристрою для кожного вибраного контексту: контакту, кімнати або простору. Спільний прихований root key не повинен бути видимим у wire data, signatures, aliases чи service endpoints.

Anonymous не означає unauthenticated: peer може криптографічно довести, що контролює ключ Persona, не розкриваючи legal identity, телефон або public profile.

## 3. Federated-node mode

AT Protocol показує сильний portability pattern: identity залишається DID, тоді як PDS location і signing configuration можуть змінюватися при міграції [7][8]. Matrix показує простішу, але жорсткішу альтернативу: Matrix user ID містить domain homeserver, який його видав [9]. Для Vida server-bound root ID створив би зайву залежність від оператора та ускладнив би міграцію.

Рекомендована модель federated node:

- Persona локально доводить control над своїм `PersonaId`;
- node створює власний `remoteAccountId` і підписаний `ServiceBinding`;
- binding може містити Vida address, mailbox, sync endpoint, quota, retention і federation policy;
- node `MAY` видавати attestation, але `MUST NOT` непомітно замінювати Persona controller;
- система `MUST` дозволяти експортувати й переносити ідентичність та зашифровані дані;
- зміна node створює новий binding і відкликає старий, не змінюючи Persona, якщо користувач свідомо зберігає continuity.

Federated Persona може залишатися private й доступною лише за exact address/invite. Наявність node не означає directory listing.

## 4. Public identity

Public identity має бути окремою opt-in projection. SimpleX Names прямо визначає name як discovery layer, а не account чи identity [10]. Signal usernames також є optional способом почати контакт, а не profile name чи незмінним public identity [11]. ActivityPub корисний для public federation surfaces, але його Actor та inbox/outbox є server URLs [12]; це не підходить як єдине приватне identity-ядро Vida.

`PublicProfile` має містити лише явно опубліковані поля:

- display name, avatar, description;
- public alias/handle;
- дозволені contact entry points;
- public channels, businesses, communities або pages;
- selected credentials/attestations;
- discoverability і message-request policy.

Public content і public identity — теж різні речі. SimpleX Channels показує, що public content можна поєднати з participation privacy; публічно joinable content не слід описувати як секретний лише через encryption [13].

Public Persona може бути hosted provider, self-hosted або використовувати resolver/registry. Public handle повинен резолвитися в stable Persona reference, але не замінювати його.

## 5. Пристрої, ключі та recovery

Signal Sesame окремо моделює UserID, DeviceID і session state та допускає per-device identity keys [14]. Matrix cross-signing використовує user-level master/self-signing keys для довіри до device keys [15]. Keybase додає нові пристрої через уже trusted device або paper key і після device revocation обертає per-user encryption material [16][17]. Automerge Repo Keyhive показує ризик протилежного підходу: якщо єдина локальна keypair втрачена без backup, identity зникає без відновлення [18].

Для Vida потрібні різні ключі та ідентифікатори:

```text
PersonaId != DeviceId != EndpointId != ServiceAccountId

Controller/Recovery keys
  └─ authorize DeviceGrant
       ├─ device signing key
       ├─ device key-agreement key
       └─ EndpointBinding → Iroh EndpointId
```

Базові recovery paths:

1. trusted-device approval із QR/short authentication string;
2. encrypted recovery package із recovery secret;
3. optional organization recovery/quorum для managed Persona;
4. node-assisted account recovery, яке може відновити service access, але не повинно саме розшифровувати приватні дані чи непомітно замінювати controller.

Якщо autonomous користувач втратив усі trusted devices і recovery package, Vida має чесно повідомити про безповоротну втрату зашифрованих даних.

## 6. Independent policy axes

Кожний UX preset компонується з таких незалежних axes:

```text
Identity scope:     pairwise | contact | room | space | public
Hosting:            none | provider | self-hosted | multiple bindings
Discoverability:    invitation | exact address | directory | public web
Verification:       none | phone | domain | organization | government | custom
Network privacy:    direct allowed | relay required | onion/privacy route
Authorization:      memberships + roles + resource grants
```

## 7. Переходи між режимами

| Перехід | Рекомендована семантика |
|---|---|
| Autonomous → Federated | Додати `ServiceBinding`; зберегти Persona лише після явного вибору continuity |
| Federated node A → node B | Перенести state, додати binding B, переключити address, відкликати binding A |
| Private → Public | Створити `PublicProfile` і alias після preview даних, що стануть публічними |
| Anonymous → Public | За замовчуванням створити нову Persona; link/merge дозволити лише з попередженням про незворотну кореляцію |
| Public → Private | Припинити майбутню directory/web видачу; не обіцяти стирання вже скопійованих profile/content/linkage |
| Device lost | Revoke `DeviceGrant`, rotate affected key envelopes/epochs, зберегти Persona continuity |

## 8. Cross-dimension insights

1. **Persona є одиницею identity, а Vault — одиницею локального володіння.** Один Vault може містити непов'язані Persona; ця спорідненість не повинна витікати в мережу.
2. **Federation і publicity ортогональні.** Можливі federated-private, self-hosted-public та anonymous Persona з opaque availability service.
3. **Portability потребує власного controller layer.** Якщо node одночасно є адресою, controller і recovery authority, міграція або стає неможливою, або повністю залежить від довіри до оператора.
4. **Transport identity не є user identity.** Iroh EndpointId потрібний для з'єднання; Iroh підтримує public, dedicated і self-hosted relay paths [19], але Vida окремо визначає Persona, devices, authorization і recovery.

## 9. Contrary evidence and trade-offs

- Server-bound identifiers, як у Matrix, простіші для routing і moderation [9]. Ціна — складніша portability та зміна адреси/identity при зміні namespace.
- Один глобальний public key, як базова Nostr identity, спрощує signatures і discovery [3], але збільшує cross-context correlation; Vida має дозволяти його лише як свідомо public Persona.
- Повна автономність прибирає provider dependency, але сама собою не забезпечує доступність або recovery. Без relay/node offline peers не отримають нові дані; без backup втрачені ключі не відновлюються.
- Public alias registry створює питання врядування, захоплення імен і протидії зловживанням незалежно від identity protocol.

## 10. Decisions still required

### Required ADRs and prototypes

| Question | Required output | Affects |
|---|---|---|
| Власний Vida `PersonaId` із DID-compatible document model чи standardized DID method? | ADR + interoperability prototype | Persona/controller model |
| Signed append-only identity log, DID document чи інша canonical controller history? | ADR + key-rotation prototype | Recovery, migration, revocation |
| Per-contact, per-room чи policy-selected per-Space anonymous Persona? | Privacy threat model + metadata test | Anonymous mode |
| Recovery package format, KDF і storage UX для v1? | Security ADR + recovery usability test | Device loss and onboarding |

### Policy and governance questions

| Question | Required output | Affects |
|---|---|---|
| Хто адмініструє public alias registry та вирішує conflicts/abuse? | Governance policy + abuse process | Public identity |
| Які дані federated node може бачити для moderation, quotas та legal compliance? | Data-classification and node-visibility policy | Federation and privacy |

## Appendix A. Source register

| Ref | Claim/finding | Publisher | Pub date | Accessed | Confidence |
|---|---|---|---|---|---|
| [1] | DID is stable identity; PDS is a service endpoint | [AT Protocol](https://atproto.com/specs/did) | n.d. | 2026-09-18 | high |
| [2] | Handle is mutable discovery alias resolved bidirectionally | [AT Protocol](https://atproto.com/specs/handle) | n.d. | 2026-09-18 | high |
| [3] | Public key remains primary over NIP-05 address | [Nostr protocol](https://github.com/nostr-protocol/nips/blob/master/05.md) | n.d. | 2026-09-18 | medium |
| [4] | Pairwise DIDs mitigate correlation | [W3C DID Core](https://www.w3.org/TR/did/#did-correlation-risks) | 2022-07-19 | 2026-09-18 | high |
| [5] | Messaging without network-wide user identifier | [SimpleX Chat](https://simplex.chat/docs/simplex.html) | n.d. | 2026-09-18 | high |
| [6] | Locally stored profiles and per-connection incognito profile | [SimpleX Chat](https://simplex.chat/docs/guide/chat-profiles.html) | n.d. | 2026-09-18 | medium |
| [7] | Identity, account hosting and lifecycle are distinct | [AT Protocol](https://atproto.com/specs/account) | n.d. | 2026-09-18 | high |
| [8] | Identity and content can migrate between PDS hosts | [AT Protocol](https://atproto.com/guides/account-migration) | n.d. | 2026-09-18 | high |
| [9] | Matrix user ID is namespaced to allocating homeserver | [Matrix.org Foundation](https://spec.matrix.org/v1.18/appendices/#user-identifiers) | v1.18 | 2026-09-18 | high |
| [10] | Public name is discovery layer, not account/identity | [SimpleX Chat](https://simplex.chat/docs/protocol/names-overview.html) | 2026-07-14 | 2026-09-18 | high |
| [11] | Username is optional contact initiation mechanism | [Signal](https://support.signal.org/hc/en-us/articles/6712070553754-Phone-Number-Privacy-and-Usernames) | n.d. | 2026-09-18 | medium |
| [12] | Public federation uses server-hosted Actor inbox/outbox | [W3C ActivityPub](https://www.w3.org/TR/activitypub/) | 2018-01-23 | 2026-09-18 | high |
| [13] | Public content can preserve participation privacy | [SimpleX Channels](https://simplex.chat/docs/protocol/channels-overview.html) | 2026-04-28 | 2026-09-18 | high |
| [14] | User/device/session separation for multi-device E2EE | [Signal Sesame](https://signal.org/docs/specifications/sesame/) | 2017 | 2026-09-18 | high |
| [15] | User-level cross-signing authorizes device keys | [Matrix.org Foundation](https://github.com/matrix-org/matrix-spec/blob/main/content/client-server-api/modules/end_to_end_encryption.md#cross-signing) | n.d. | 2026-09-18 | high |
| [16] | Trusted device or paper key provisions/revokes devices | [Keybase](https://book.keybase.io/account) | 2022 | 2026-09-18 | high |
| [17] | Device revocation rotates per-user encryption key | [Keybase](https://book.keybase.io/docs/teams/puk) | n.d. | 2026-09-18 | high |
| [18] | Local key without backup makes identity loss irreversible | [Automerge](https://automerge.org/docs/keyhive/ark-api-guide/) | 2026 | 2026-09-18 | high |
| [19] | Connectivity supports shared, dedicated and self-hosted relays | [Iroh](https://www.iroh.computer/services/hosting) | n.d. | 2026-09-18 | medium |

## Appendix B. Staleness map

| Claim class | Re-check by |
|---|---|
| Product patterns and integration assumptions | 2027-09-18 |
| Architecture, security, privacy and recovery patterns | 2028-09-18 |

Earliest refresh: **2027-09-18**. Refresh sooner if Vida selects a DID method, public alias registry, recovery implementation or a materially different Iroh release/API.
