# Iroh core 1.x — agent digest

Accessed: 2026-09-18. Source class: first-party documentation, repositories, and releases.

- Current stable core is `iroh` 1.2.0, released 2026-09-11. Source: https://github.com/n0-computer/iroh/releases/tag/v1.2.0 (high confidence).
- `EndpointId` is an Ed25519 public key; QUIC connections are authenticated and end-to-end encrypted. One application normally shares one `Endpoint`. Source: https://docs.iroh.computer/concepts/endpoints (high).
- Application protocols are separate ALPN-identified handlers; `iroh::protocol::Router` dispatches accepted connections. Multiple ALPN values can represent protocol versions. Source: https://docs.iroh.computer/concepts/protocols (high).
- Current terminology is Address Lookup, not discovery. Default lookup uses signed DNS/Pkarr; local and mainline-DHT options are opt-in. Source: https://docs.iroh.computer/concepts/address-lookup (high).
- Connectivity may begin through an end-to-end-encrypted relay, attempt QUIC NAT traversal, and migrate to a direct path. Multipath keeps relay/direct/custom candidates and prefers low-latency paths. Source: https://docs.iroh.computer/concepts/nat-traversal (high).
- Core wire compatibility is promised within a major version, excluding explicitly unstable surfaces. Source: https://docs.iroh.computer/about/release-policy (high).
- Browser WASM is relay-only because browsers cannot send arbitrary UDP; there is no first-party browser npm bundle. Native Node retains direct connectivity. Source: https://docs.iroh.computer/languages/wasm-browser (high).
- First-party FFI covers Python, Swift, Kotlin/JVM, and Node.js for stabilized core concepts. Blobs/docs/gossip are explicitly excluded; custom lookup/transports are not listed in the stabilized binding surface and require prototype verification. Source: https://github.com/n0-computer/iroh-ffi (high for explicit scope, medium for absent surfaces).
- Higher protocols remain separately versioned pre-1.0 crates: `iroh-blobs` 0.103.0 and `iroh-gossip` 0.101.0 (2026-06-15); their Iroh 1.0 migrations were breaking. Sources: https://github.com/n0-computer/iroh-blobs/releases and https://github.com/n0-computer/iroh-gossip/releases (high).
- `iroh-docs` 0.100.0 (2026-05-27) is a meta-protocol over blobs and gossip and was released against an Iroh 1.0 release candidate. Source: https://github.com/n0-computer/iroh-docs/releases (high).

Contrary evidence and risks:

- Old `iroh-router` documentation and the term discovery are stale for current core.
- Core 1.x stability does not confer wire/API stability on blobs, gossip, or docs.
- The `iroh-blobs` main README calls the current line not production-quality and recommends 0.35 for production despite newer releases; this unresolved contradiction blocks unconditional adoption.
- Stable FFI exposes raw/core transport, while higher protocols and custom lookup remain Rust integration work.

Decision implication: adopt and pin Iroh core 1.2 behind a VIDA-owned node host and versioned ALPN boundary; evaluate every 0.x higher protocol as an independently pinned experimental adapter with cross-version tests.
