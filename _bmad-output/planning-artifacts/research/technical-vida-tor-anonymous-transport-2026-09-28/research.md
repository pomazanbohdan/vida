---
title: 'Technical research: Tor-isolated transport for VIDA Release 1'
type: technical
topic: 'Anonymous Persona network transport, Iroh, Delta Chat and Chromium Web'
date: 2026-09-28
status: researched-not-library-selected
---

# Tor-захищений транспорт VIDA: рішення для погодження

## Висновок

У Release 1 потрібен окремий **Tor-only мережевий профіль** для анонімної Persona. Він не замінює звичайний VIDA-профіль direct-first із дозволеним Iroh relay fallback. Профіль має приховувати IP учасників від одне одного і від звичайного VIDA relay, але не може гарантувати «повної анонімності»: саморозкриття, кореляція часу/обсягу, вміст вкладень та ідентифікатори інших сервісів лишаються ризиками. Tor Project прямо застерігає, що досконала анонімність неможлива і Tor захищає лише коректно сконфігурований трафік застосунку. [Tor safety](https://support.torproject.org/tor-browser/security/using-tb-safely/), [Tor stream isolation](https://spec.torproject.org/path-spec/stream-isolation.html).

## Що насправді показує Delta Chat

- Delta Chat V2 передає звичайні повідомлення через **chatmail relays**, які тимчасово тримають шифротекст; це не Iroh relay і не модель VIDA. [Delta Chat FAQ](https://delta.chat/en/help).
- У Delta Chat є історичний сценарій SOCKS5/Tor для частини мережевого трафіку. Публічне повідомлення розробника застерігало, що Iroh realtime не успадковував proxy settings навіть при relay; issue desktop фіксував окремі непрохідні через proxy WebRTC calls та Iroh realtime. Це докази **ризику**, а не твердження, що всі поточні версії Delta Chat досі мають ті самі дефекти. [Обговорення Iroh/proxy](https://support.delta.chat/t/alternative-p2p-realtime-chat-method-without-revealing-your-lan-wan-ip/3554), [Delta Chat desktop issue](https://github.com/deltachat/deltachat-desktop/issues/3093).
- У червні 2026 розробник Delta Chat писав, що calls використовують WebRTC, а перехід Iroh з 0.35 на 1.0 ще планувався; Tor/Nym для P2P згадували як надію після переходу. Це **не доказ** готового анонімного Iroh+Tor у Delta Chat. [Delta Chat forum](https://support.delta.chat/t/wider-usage-of-iroh/5353).

## Кандидати та межі доказу

1. Iroh показав Tor onion-service custom transport. Концептуально це найкращий референс для VIDA Iroh protocol over Tor, але автор називає custom transport експериментальним, а приклад запускає окремий C Tor daemon із вимкненою cookie authentication. Таку конфігурацію не можна переносити в production. Автор прямо вимагає вимкнути звичайні IP/relay transports і discovery для приватного профілю. [Iroh Tor transport write-up](https://www.iroh.computer/blog/tor-custom-transport), [репозиторій](https://github.com/n0-computer/iroh-tor-transport).
2. `arti-client` — офіційний вбудовуваний Rust-кандидат для Tor streams. [Головна сторінка Arti](https://arti.torproject.org/) називає proxy та onion-service support готовими до використання, але [сторінка обмежень](https://arti.torproject.org/guides/capability-limitations/) одночасно каже «не рекомендований для production» і попереджає, що її деталі частково застарілі. Це суперечливий стан документації, а не доказ непридатності. Версія `<1.0` сама по собі не дискваліфікує Arti; потрібні перевірка актуального release/feature profile, мобільних збірок, onion hosting, security review і conformance.
3. Альтернатива для першого prototype — ізольований Tor daemon/sidecar із автентифікованим control channel, але Android/iOS background і distribution профіль потребують окремих перевірок. Публічний Iroh example із `CookieAuthentication 0` є лише демонстрацією, не безпечним шаблоном.

## Рекомендований контракт VIDA

- Режим обирається на рівні **Persona/network context**, щоб чат, файли, запрошення, контакти/lookup, sync, receipts, presence, дзвінки, push, діагностика та відкриття зовнішніх URL не залишили непоміченого каналу повз Tor. Для операції в анонімному контексті усі мережеві підзапити йдуть через перевірений Tor path або fail-closed. Окремі Apps не можуть самі вимкнути це правило.
- Якщо Tor недоступний, локальний durable save дозволений, мережеві операції лишаються pending; **ніякого** fallback на звичайний direct, N0/public relay чи VIDA relay. Власний relay може бути доступним як `.onion` endpoint, але це все ще Tor path; звичайний relay fallback лишається тільки для звичайного профілю.
- Вхідний peer може бути onion service, а не відкритий IP endpoint. Ідентифікатори/ключі для Tor-контексту треба відокремити від публічної/робочої Persona; не копіювати референсне використання одного Iroh private key для onion-адреси без privacy review. Tor circuits між Personas ізолювати application-provided tokens. [Tor onion services](https://community.torproject.org/onion-services/overview/), [Tor stream isolation](https://spec.torproject.org/path-spec/stream-isolation.html).
- E2EE ресурсу лишається незалежним від Tor: Tor приховує маршрут, не замінює шифрування вмісту, права або acceptance receipts. UI називає режим «Tor-захищений», а не гарантує абсолютну анонімність.
- Для **звичайних** нотаток, задач, файлів і чатів зберігається direct-first + налаштований relay fallback. Але будь-який ресурс/чат, що належить анонімній Persona, включно з його вкладеннями та синхронізацією, не може непомітно вийти звичайним шляхом.
- Chromium-based static Web є окремим ризиком: [Chromium documentation](https://developer.chrome.com/docs/iwa/direct-sockets) прямо зазначає, що стандартна сторінка не має raw TCP/UDP; Direct Sockets доступні Isolated Web Apps, а не звичайній GitHub Pages/HTTPS-сторінці. Варіант із віддаленим WebSocket gateway технічно переносить транспорт, але цей gateway бачить IP клієнта до входу в Tor; він не виконує обіцянки «лише Tor». Локальний native helper або IWA змінюють модель самостійного статичного Web. Додатково Tor Project не рекомендує Tor в іншому браузері через WebRTC/DNS/fingerprint/cookies. Отже звичайний Web Release 1 підтримується, але анонімний Web-профіль потребує окремого продуктового рішення й proof; його не можна позначити готовим лише через Rust/Wasm. [Tor and other browsers](https://support.torproject.org/tor-browser/security/using-tor-with-other-browsers/).

## Release-1 proof gate, не виконаний дослідженням

Threat model визначає, від кого приховуємо IP та metadata. Integration prototype має довести Tor-only transport на кожній заявленій платформі, onion discovery/connection, E2EE Iroh payload і receipt, network-capture tests без DNS/STUN/WebRTC/direct/relay витоків, fail-closed після втрати Tor, cross-Persona isolation, recovery та revoked-device поведінку, медіа/вкладення, зовнішні URL/preview, push і diagnostics. Окремо перевіряються battery/background policy і latency. Ні бібліотеку, ні Web анонімний режим цим документом не затверджено.
