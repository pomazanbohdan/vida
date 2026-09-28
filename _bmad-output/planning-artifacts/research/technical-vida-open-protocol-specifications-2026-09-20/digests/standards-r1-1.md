# Standards and format digest (2026-09-20)

| Claim | Source/publisher | Date | Confidence | Class |
|---|---|---|---|---|
| NIPs are optional implementation possibilities, not a checklist; acceptance suggests two clients and a relay when applicable, optional backwards compatibility, and one way per feature. | https://github.com/nostr-protocol/nips / Nostr maintainers | accessed 2026-09-20 | high | primary/live |
| NIP-01 defines signed JSON event and client/relay baseline; `kind` interprets an event and is not its NIP number. | https://github.com/nostr-protocol/nips/blob/master/01.md / Nostr maintainers | accessed 2026-09-20 | high | primary/live |
| NIP-29 group access is enforced by a relay and its role details are relay-specific; this does not directly implement VIDA distributed Space authority. | https://github.com/nostr-protocol/nips/blob/master/29.md / Nostr maintainers | accessed 2026-09-20 | high for source, medium for architectural comparison | primary/inference |
| NIP-78 application data permits arbitrary `content` and tags; NIP-42 auth is recommended for private reads. | https://github.com/nostr-protocol/nips/blob/master/78.md / Nostr maintainers | accessed 2026-09-20 | high | primary/live |
| Matrix separates MSC proposal/implementation evidence from integration in published spec; unstable vendor-prefixed surface exists before stabilization. | https://spec.matrix.org/proposals/ / Matrix Foundation | accessed 2026-09-20 | high | primary/live |
| AT Lexicon describes records, HTTP endpoints and event streams, namespaced by NSID; schema evolution preserves old/new mutual validity. | https://atproto.com/specs/lexicon / AT Protocol | accessed 2026-09-20 | high | primary/live |
| CDDL describes CBOR/JSON structures, not permission semantics; protobuf describes binary message compatibility, not authority protocol. | https://www.rfc-editor.org/rfc/rfc8610.html ; https://protobuf.dev/programming-guides/proto3/ | accessed 2026-09-20 | high | primary/live |

Leads: governance/IPR/license, interoperability test harness, stable vs experimental namespace, security/replay/authority semantics, Nostr bridge limited to explicitly chosen public capabilities. No evidence that adoption of NIPs alone would make VIDA's Iroh ALPN protocol interoperable.
