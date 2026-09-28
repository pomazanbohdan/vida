---
title: 'Technical research: direct-first Iroh connectivity and UniClipboard patterns for VIDA'
type: technical
topic: 'Iroh direct connectivity, UniClipboard, VIDA-owned relay fallback'
decision: 'Which peer discovery and transport profiles should VIDA use for direct-first connectivity with its own relay fallback?'
source: 'User request and primary-source research, 2026-09-25'
status: complete
preset: standard
validation: normal
verified_claims: 2
unverified_claims: 3
verification_basis: 'Five load-bearing claims in memlog; external source review completed, no VIDA runtime or device conformance test.'
created: '2026-09-25'
updated: '2026-09-25'
---

# Technical research: direct-first Iroh connectivity and UniClipboard patterns for VIDA

**Decision this research serves:** Define a direct-first VIDA connection architecture independent of Delta Chat Relay and UniClipboard services, with VIDA-operated Iroh relay fallback.

## Висновок

**Так: нативні Iroh-пристрої можуть з'єднуватися без relay**, якщо знають актуальну адресу й можуть досягти один одного: LAN через opt-in mDNS, публічний IPv6/IPv4 або налаштований прямий маршрут. Це transport capability, не автоматична гарантія для двох телефонів у різних мобільних мережах. [1][2]

**Для типового cross-NAT шляху Iroh використовує relay двічі:** як місце початкового контакту для обміну адресами й спроби UDP hole punching; якщо прямий шлях не відкрився — для передачі зашифрованих пакетів. Після успішного direct дані йдуть повз relay. Це не Delta Chat Relay і не поштовий сервіс. [3][4]

**Рекомендація VIDA:** власна Persona/Space/authorization/sync-семантика над Iroh 1.2; `direct-first` для нативних клієнтів; локальний `LAN-only` профіль без relay; VIDA-operated Iroh-compatible relays як fallback; окремо керована address lookup. **Статичний браузерний клієнт є винятком:** поточний Iroh Wasm завжди передає трафік через relay. [5][6][7] Отже, для Web фраза «direct-first» зараз не є правдивою.

Це **дослідницька рекомендація**, не зміна вже прийнятих ADR. [ADR-0005](../../../../docs/03-architecture/decisions/ADR-0005-iroh-transport-foundation.md) уже погоджує native direct + relay fallback і VIDA-owned ALPN; окремий [ADR-0014](../../../../docs/03-architecture/decisions/ADR-0014-native-only-vida-clients.md) поки виключає браузер із Release 1, хоча в пізнішому обговоренні його запитали. Релізні документи треба узгодити окремо.

## 1. Як виникає з'єднання

| Крок | Хто і що робить | Якщо немає relay |
| --- | --- | --- |
| 1. Довіра | VIDA створює окремі Persona, Device і Space ключі; запрошення/підтвердження додає Device або людину за правилами Space. `EndpointId` Iroh не є правом доступу VIDA. | Iroh ticket у QR може передати адресу без сервера, але він сам по собі повторно використовуваний і може розкрити IP; одноразовість, строк і права мають бути окремою VIDA invitation. [14] |
| 2. Пошук адреси | За відомим `EndpointId` потрібні поточні direct IP/порт або relay URL. Iroh має окремі DNS/Pkarr, mDNS і opt-in DHT lookup. [8][9] | LAN mDNS або попередньо передана актуальна адреса; DHT може прибрати hosted lookup, але не створює мережевої досяжності. |
| 3. З'єднання | Native Iroh пробує відомі прямі адреси; у cross-NAT relay допомагає координувати hole punching. [1][3] | Пряме native з'єднання можливе лише за реально досяжної адреси/маршруту. |
| 4. Fallback | Якщо NAT/firewall блокує direct, Iroh передає E2EE трафік через relay. [4] | Якщо direct неможливий, peer недоступний: зберігаємо операцію локально і повторюємо після появи шляху; не називаємо її доставленою. |
| 5. Sync | Після з'єднання VIDA обмінюється власними versioned ALPN/envelopes, перевіряє членство та application-level receipts. | Шлях не змінює семантику операцій, конфліктів і підтверджень. |

Критична відмінність: **relay ≠ discovery ≠ mailbox ≠ authority**. Транспортний relay не зберігає історію й не вирішує бізнес-конфлікт. Address Lookup лише показує, куди пробувати dial. DHT/публічний DNS не замінюють relay для складного NAT. [4][8][9]

## 2. Що насправді показує UniClipboard

UniClipboard явно розділяє **trust** (одноразове запрошення + passphrase/PAKE), **discovery** (mDNS, власний HTTPS rendezvous, кешовані адреси) та **transport** (Iroh QUIC direct, за потреби encrypted relay). Це корисний архітектурний патерн, але не готовий VIDA account або sync protocol. [10]

Його `LAN-only` вимикає Iroh relay і публічний n0 DNS: уже спарені desktop peers можуть синхронізуватися на одному LAN навіть без WAN. Проте опублікована інструкція зазначає, що **первинне pairing коротким кодом усе ще використовує сервіс rendezvous UniClipboard**. Для повністю автономної VIDA треба власний QR/локальний bootstrap, а не копіювати цей прихований сервісний крок. [11]

Важлива межа застосовності: продуктова документація UniClipboard описує поточний мобільний companion як HTTP до desktop, без Iroh P2P і без phone↔phone sync; Engine README водночас описує мобільні bindings та P2P-архітектуру. Це різні рівні доказу. Тому UniClipboard не доводить, що Android↔Android↔Web VIDA уже працюватиме: потрібен власний міжплатформний prototype. [10][12]

## 3. Рекомендовані transport profiles VIDA

| Профіль | Discovery | Шлях даних | Призначення |
| --- | --- | --- | --- |
| `LAN-only` | Локальний mDNS і явні QR/known-address hints; без default n0 lookup | Native direct; relay disabled | Офлайн-зустріч двох пристроїв на досяжному LAN; невдачу показувати чесно. [2][11] |
| `Direct-first` | Локальний mDNS + затверджений lookup + кешовані короткоживучі адреси | Native direct; **власний** Iroh relay для NAT coordination/fallback | Звичайний online-режим Android/iOS/desktop. [3][4][8] |
| `Browser` | Профіль Web окремо від native | За поточним Iroh Wasm **relay-only**, навіть якщо Android поруч у тому самому Wi-Fi | Статичний Flutter Web, якщо його повернути в scope; потрібен доступний relay. [5] |
| `IP-private` (окреме рішення) | Вимагає явного privacy profile | Direct-first **не** обіцяє приховати IP від peer | Не називати звичайний native direct «повністю анонімним». [13] |

Для власної інфраструктури варто **експлуатувати protocol-compatible open-source `iroh-relay`**, а не писати новий wire relay. Iroh дозволяє custom relay URLs; власний оператор прибирає залежність від Delta/Chatmail і публічних n0 relay, але не прибирає залежності від Iroh як узгодженого transport substrate. Окремо треба вибрати власний або незалежний Address Lookup: заміна relay URL сама по собі не вимикає default n0 DNS/Pkarr. Висновок — інференція з конфігурації Iroh та його deployment docs. [6][7][8]

## 4. Ризики та перевірка

1. **Доступність:** одна LAN може блокувати mDNS/P2P через ізоляцію клієнтів; два різні NAT можуть не пропустити hole punch; браузер без relay не працює. Тому в UI показувати фактичний шлях `direct / relay / недоступний`, а не один маркетинговий статус «P2P». [2][3][5]
2. **Метадані:** native direct розкриває IP адреси сторонам; власний relay бачить час, обсяг і учасників з'єднання, хоча не вміст. «Повна анонімність» потребує окремого threat model і, можливо, іншого транспортного профілю. [13]
3. **Сервісна незалежність:** для релізу перевірити, що custom relay + chosen lookup не роблять прихованих запитів до n0/UniClipboard/Delta; relay не є durable mailbox. [7][8][10]
4. **Конформанс:** одна й та сама VIDA operation має application-level результат на direct та relayed path; transport ACK не є `synced` чи business-approved receipt.

**Proof matrix до остаточного ADR:** Android↔Android на одному LAN із відключеним WAN і relay; два native peers з різних мереж, де direct вдався; мережа з примусовим relay fallback; Web↔Android через власний relay; reconnect після sleep/network switch; outage одного relay; перевірка DNS/relay egress; packet/receipt trace без приватного вмісту. Для всіх сценаріїв збирати факт обраного шляху, а не виводити його з очікування. [1][2][3][5][7]

## Відкриті рішення

- Чи підтримувати `LAN-only` як явний user-facing режим уже в першому релізі, чи лише як conformance/profile для автономної Persona? Технічно можливий, але не гарантує кросмережеву доставку.
- Чи повертаємо static Web в Release 1, змінивши ADR-0014/PRD? Якщо так, relay стає обов'язковою частиною Web availability, попри direct-first native.
- Яким буде VIDA-owned lookup у production і хто оперує щонайменше двома relay instances? Не треба змішувати lookup, relay, mailbox і федеративний вузол.

## Джерела

| № | Опорне твердження | Первинне джерело | Опубліковано | Перевірено | Впевненість |
| --- | --- | --- | --- | --- | --- |
| [1] | Direct dial та disabled relay native API | [Iroh 1.2 endpoint Builder](https://docs.rs/iroh/latest/iroh/endpoint/struct.Builder.html) | 1.2.0 | 2026-09-25 | high |
| [2] | mDNS LAN direct, opt-in | [Iroh mDNS](https://docs.iroh.computer/connecting/local-address-lookup) | living docs | 2026-09-25 | high |
| [3] | NAT coordination і direct attempt | [Iroh NAT traversal](https://docs.iroh.computer/concepts/nat-traversal) | living docs | 2026-09-25 | high mechanism |
| [4] | Relay fallback, no durable app state | [Iroh relays](https://docs.iroh.computer/concepts/relays) | living docs | 2026-09-25 | high |
| [5] | Browser Iroh relay-only | [Iroh WebAssembly/browser](https://docs.iroh.computer/languages/wasm-browser) | living docs | 2026-09-25 | high |
| [6] | Open-source self-hosted Iroh relay | [Iroh self-hosted relay](https://docs.iroh.computer/iroh-services/relays/self-hosted) | living docs | 2026-09-25 | high capability; config guide caveat |
| [7] | Custom relays vs default public infra | [Iroh dedicated infrastructure](https://docs.iroh.computer/deployment/dedicated-infrastructure) | living docs | 2026-09-25 | high |
| [8] | Address lookup providers and defaults | [Iroh Address Lookup](https://docs.iroh.computer/concepts/address-lookup) | living docs | 2026-09-25 | high |
| [9] | DHT removes hosted lookup, not reachability | [Iroh DHT](https://docs.iroh.computer/connecting/dht-address-lookup) | living docs | 2026-09-25 | high |
| [10] | UniClipboard trust/discovery/transport and mobile boundary | [UniClipboard pairing](https://docs.uniclipboard.app/guides/pairing) | living docs | 2026-09-25 | high documented; runtime untested |
| [11] | UniClipboard LAN-only still uses rendezvous for initial code pairing | [UniClipboard FAQ](https://docs.uniclipboard.app/help/faq) | living docs | 2026-09-25 | high documented; runtime untested |
| [12] | Engine architecture vs released mobile behavior | [UniClipboard Engine README](https://github.com/UniClipboard/Engine) | main | 2026-09-25 | medium for release comparison |
| [13] | Direct IP exposure; relay metadata | [Iroh security/privacy](https://docs.iroh.computer/concepts/security-privacy) | living docs | 2026-09-25 | high |
| [14] | QR endpoint ticket без сервера, але reusable/stale/IP exposure | [Iroh tickets](https://docs.iroh.computer/concepts/tickets) | living docs | 2026-09-25 | high |

## Актуальність

Iroh version/API та браузерні можливості перевіряти перед implementation pin (ціль ≤1 місяць). UniClipboard product/mobile режим і власні relay/discovery налаштування — перед використанням як release evidence (ціль ≤1 місяць). Джерела — живі документи без стабільної дати публікації, тому висновки чинні для перевірки 2026-09-25, не безстроково.
