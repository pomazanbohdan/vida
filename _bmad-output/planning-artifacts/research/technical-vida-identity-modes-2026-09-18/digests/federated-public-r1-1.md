# Federated hosting and public identity — round 1

- claim: AT Protocol demonstrates portable identity: DID remains the primary identifier while the PDS service location and signing keys can change during account migration.
  source: https://atproto.com/specs/account and https://atproto.com/guides/account-migration
  publisher: AT Protocol
  pub_date: n.d.
  accessed: 2026-09-18
  confidence: high
  class: architecture-pattern
- claim: Matrix user IDs contain the allocating homeserver domain. This is useful federation addressing but couples the durable user identifier to a server namespace.
  source: https://spec.matrix.org/v1.18/appendices/#user-identifiers
  publisher: Matrix.org Foundation
  pub_date: n.d.
  accessed: 2026-09-18
  confidence: high
  class: architecture-pattern
- claim: ActivityPub actors are URL-identified server resources with inbox/outbox endpoints; it is suitable for public federation surfaces but does not itself provide a host-independent private identity core.
  source: https://www.w3.org/TR/activitypub/
  publisher: W3C
  pub_date: 2018-01-23
  accessed: 2026-09-18
  confidence: high
  class: interoperability
- claim: Public names can be a separate discovery layer rather than an account or identity. SimpleX Names explicitly maps names to existing contact/channel links.
  source: https://simplex.chat/docs/protocol/names-overview.html
  publisher: SimpleX Chat
  pub_date: 2026-07-14
  accessed: 2026-09-18
  confidence: high
  class: product-pattern
- claim: Signal usernames are optional contact-discovery instruments rather than profile names or durable public identities.
  source: https://support.signal.org/hc/en-us/articles/6712070553754-Phone-Number-Privacy-and-Usernames
  publisher: Signal
  pub_date: n.d.
  accessed: 2026-09-18
  confidence: medium
  class: product-pattern

Leads: a Vida node should issue service bindings, addresses, mailboxes and quotas without becoming the root identity; public profile and directory publication must be opt-in.

