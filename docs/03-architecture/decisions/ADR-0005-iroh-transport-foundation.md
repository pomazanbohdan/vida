---
id: ADR-0005
status: accepted
last_updated: 2026-09-25
source_refs:
  - ../../../_bmad-output/planning-artifacts/research/technical-iroh-ecosystem-and-project-patterns-for-2026-09-18/research.md
  - ../../../_bmad-output/planning-artifacts/research/technical-iroh-direct-connectivity-uniclipboard-re-2026-09-25/research.md
decision_refs:
  - ADR-0004-multi-axis-identity-model.md
supersedes: []
superseded_by: []
---

# ADR-0005: Iroh transport foundation and VIDA protocol boundary

## Context

VIDA потребує спільного transport substrate для mobile, desktop, nodes і сервісів. Iroh 1.2 стабілізує authenticated QUIC connectivity, NAT traversal, relays, Address Lookup і ALPN routing, але не визначає identity, authorization, durable history або offline mailbox.

## Decision drivers

- прямий P2P із relay fallback;
- незалежність Persona від transport endpoint;
- єдина transport boundary для Messenger, Notes, Projects і Apps;
- контроль VIDA над wire compatibility;
- можливість замінювати нестабільні higher protocols.

## Considered options

1. Delta Chat/Chatmail core разом з email transport.
2. Iroh core 1.2 плюс VIDA-owned application protocols.
3. Пряма залежність domain modules від `iroh-docs`, `iroh-gossip` і `iroh-blobs`.

## Decision

VIDA `MUST` використовувати Iroh core 1.2 як primary transport foundation.

`VidaNodeHost` `MUST` володіти Iroh Router, Address Lookup, relay policy, endpoint lifecycle та platform network lifecycle. Domain modules `MUST` працювати через VIDA ports і `MUST NOT` напряму імпортувати pre-1.0 higher-protocol crates.

Кожен VIDA protocol `MUST` мати versioned ALPN `vida/<capability>/<major>` і VIDA-owned envelope. Усі platform bindings `MUST` використовувати один normative codec/kernel або проходити ті самі byte-exact golden vectors. Persistent device endpoints та ephemeral anonymous/session endpoints `MUST` бути різними policy profiles. `EndpointId` `MUST NOT` бути Persona або application authorization.

Нативні VIDA peers `MUST` пробувати прямий досяжний Iroh шлях перед relay fallback. `VidaNodeHost` `MUST` підтримувати окремий relay-disabled LAN/known-address profile для автономного прямого з'єднання; відсутність досяжного шляху лишає outbound operation локально pending, а не «доставленою». Для cross-network NAT coordination і fallback VIDA `MUST` використовувати керовані VIDA protocol-compatible Iroh relay instances, а не Delta Chat/Chatmail relay чи власний несумісний relay wire. Точні relay deployment/availability і Address Lookup provider policy лишаються окремими implementation gates; налаштування власного relay `MUST NOT` мовчки залишати залежність від публічного lookup, якщо профіль обіцяє інфраструктурну автономність.

Transport relay `MUST NOT` трактувати як durable mailbox, Space authority або підтвердження синхронізації. Release-1 browser client має окремий direct-first target за ADR-0021: stock Iroh/Wasm на момент дослідження 2026-09-25 є relay-only, тому прямий Web шлях потребує перевіреного browser-compatible transport adapter; без такого доказу Web direct-first не вважається реалізованим. Relay лишається дозволеним зашифрованим fallback для Web і native, а не обов'язковим першим data path. Вибір browser release scope не визначається цим ADR.

## Rationale

Рішення приймає стабільну частину Iroh й ізолює domain model від окремо версійованих 0.x components та email coupling Delta Chat.

## Consequences

- потрібен shared `VidaNodeHost` і transport adapter;
- browser Release-1 scope і його direct-first/fallback gate визначає [ADR-0021](ADR-0021-static-web-client-in-release-1.md), а не сам transport relay;
- higher protocols дозволені лише як pinned replaceable adapters;
- compatibility тестується окремо для Iroh core, VIDA ALPN, schema/state та FFI;
- relay infrastructure не стає durable mailbox.
- release lockfile і SPDX SBOM фіксують exact Iroh/dependency versions та checksums; semver range не є release pin.

## Risks і mitigations

- endpoint correlation → endpoint profiles, contextual bindings і metadata tests;
- upstream breaking changes → exact pins, adapter boundary, mixed-version conformance;
- Address Lookup outage/compromise → provider abstraction, TTL/cache and failure tests;
- platform asymmetry → separate gates для кожної підтримуваної встановлюваної ОС.

## Acceptance evidence

- два VIDA ALPN handlers працюють через один `VidaNodeHost` Router;
- domain test suite запускається без Iroh dependency;
- persistent та ephemeral endpoint profiles не мають спільного network-visible parent ID;
- direct і relay paths доставляють той самий application envelope;
- native LAN/known-address peers синхронізують operation без будь-якого relay чи зовнішнього lookup; за недосяжності direct operation чесно лишається pending;
- cross-network тест доводить direct attempt та fallback через VIDA-operated relay без Delta/Chatmail сервісів; egress trace перевіряє фактичний Address Lookup і relay шлях;
- browser profile проходить окремі direct-path, relay-fallback, path-migration і storage/security conformance; stock Iroh/Wasm direct P2P не заявляється без доказу адаптера;
- кожен platform binding проходить спільні byte-exact codec vectors;
- кожна підтримувана платформа має окремий conformance result; перелік release targets визначається продуктовим baseline, не цим ADR.

## Open questions

- Address Lookup provider policy, cache/TTL, relay allowlist та availability/deployment profile;
- exact higher-protocol adapter versions;
- platform binding artifact matrix.
