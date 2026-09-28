# Identity axes — round 1

- claim: Stable cryptographic identity, human-readable alias, service location and hosted account are separate concepts. AT Protocol uses a persistent DID, mutable handle and PDS service location; the handle must resolve bidirectionally to the DID.
  source: https://atproto.com/specs/did and https://atproto.com/specs/handle
  publisher: AT Protocol
  pub_date: n.d.
  accessed: 2026-09-18
  confidence: high
  class: architecture-pattern
- claim: A global identifier creates correlation risk; pairwise identifiers with unique verification material reduce it, while public identity should be an explicit choice.
  source: https://www.w3.org/TR/did/#did-correlation-risks
  publisher: W3C
  pub_date: 2022-07-19
  accessed: 2026-09-18
  confidence: high
  class: security-pattern
- claim: Human-readable discovery addresses must not replace the stable cryptographic reference. NIP-05 explicitly requires clients to follow the public key rather than the address.
  source: https://github.com/nostr-protocol/nips/blob/master/05.md
  publisher: Nostr protocol
  pub_date: n.d.
  accessed: 2026-09-18
  confidence: medium
  class: architecture-pattern

Leads: model Vida identity, hosting, discovery, verification and authorization as orthogonal records.

