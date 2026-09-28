# Anonymous and autonomous operation — round 1

- claim: A communications system can operate without a network-wide user identifier by using per-connection queues and invitation links; SimpleX also supports locally stored profiles.
  source: https://simplex.chat/docs/simplex.html and https://simplex.chat/docs/guide/chat-profiles.html
  publisher: SimpleX Chat
  pub_date: n.d.
  accessed: 2026-09-18
  confidence: high
  class: architecture-pattern
- claim: Context-specific anonymous personas are practical: SimpleX incognito mode generates a different random profile for each new connection, independently of the user's ordinary local profiles.
  source: https://simplex.chat/docs/guide/chat-profiles.html
  publisher: SimpleX Chat
  pub_date: n.d.
  accessed: 2026-09-18
  confidence: medium
  class: product-pattern
- claim: Public distribution does not require exposing a subscriber's network-wide identity. SimpleX Channels use independent queues and describe public content separately from participation privacy.
  source: https://simplex.chat/docs/protocol/channels-overview.html
  publisher: SimpleX Chat
  pub_date: 2026-04-28
  accessed: 2026-09-18
  confidence: high
  class: security-pattern
- claim: Iroh can use direct connections, shared/dedicated relays or self-hosted relays; therefore relay choice is a connectivity property, not a user-identity definition.
  source: https://www.iroh.computer/services/hosting
  publisher: Iroh / n0
  pub_date: n.d.
  accessed: 2026-09-18
  confidence: medium
  class: integration-pattern

Leads: autonomous mode needs local keys, encrypted vault, invitation/contact cards and optional opaque availability infrastructure, but no registration or public profile.

