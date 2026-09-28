# Delta Chat × Iroh — agent digest

Accessed: 2026-09-18. Source class: primary code, specifications, issues, and releases.

- Current Chatmail core pins `iroh` and `iroh-gossip` 0.35. Its Iroh 1.0 upgrade PR is still draft and blocked by relay deployment and backup compatibility. Sources: https://github.com/chatmail/core/blob/main/Cargo.toml, https://github.com/chatmail/core/pull/8182, https://github.com/chatmail/relay/issues/1010 (high confidence).
- `peer_channels.rs` lazily creates an ephemeral Iroh endpoint and gossip topic. A random 32-byte topic is transported inside an encrypted WebXDC MIME message; peers advertise `NodeId` and relay data through ordinary system email rather than IP addresses. Source: https://rs.delta.chat/src/deltachat/peer_channels.rs.html (high).
- The Iroh peer channel currently carries up to 128 KiB WebXDC real-time payloads; WebXDC itself is transport-independent. Source: https://delta.chat/en/2024-11-20-webxdc-realtime (high).
- Iroh also supports QR backup/device transfer via a serialized provider ticket. Source: https://github.com/deltachat/interface/blob/main/uri-schemes.md (high). Ongoing multi-device synchronization remains mail-message-based. Source: https://github.com/chatmail/core/blob/main/spec.md (high).
- SecureJoin v3 QR data combines inviter identity, OpenPGP fingerprint, invite/auth tokens, and group/channel identifiers. It verifies tokens and a freshness window; tokens are not consumed, so VIDA must design replay handling separately. Portable ideas are staged verification and verified-contact propagation; the implementation is email/OpenPGP-coupled. Source: https://rs.delta.chat/src/deltachat/securejoin.rs.html (high).
- Broadcast channels use an owner-only model and a shared channel secret to hide the recipient graph and avoid per-recipient ciphertext. No evidence supports claiming generic MLS-style group key rotation. Source: https://github.com/chatmail/core/issues/6884 (high).
- Delta Chat core is MPL-2.0 file-level copyleft; the relay is MIT; Iroh is MIT OR Apache-2.0. Sources: https://github.com/chatmail/core/blob/main/LICENSE, https://github.com/chatmail/relay, https://github.com/n0-computer/iroh/blob/main/README.md (high).
- Delta Chat has shipped an Iroh path since 1.48; its current dependency remains 0.35 and the 1.x migration is incomplete. Sources: https://delta.chat/en/2024-11-20-webxdc-realtime, https://github.com/chatmail/core/blob/main/Cargo.toml, https://github.com/chatmail/core/pull/8182 (high). Its deny configuration contains a separate transitive `lru` advisory exception; this is not evidence for the other claims. Source: https://github.com/chatmail/core/blob/main/deny.toml (high).

Portable patterns:

- Ephemeral transport identities distinct from user identity.
- QR trust bootstrap and lazy peer-channel creation.
- App-instance-scoped random topics and gossip broadcast.
- Relay metadata lookup without exposing direct addresses in invitations.
- Event/JSON-RPC boundary between core and clients.

Rejected coupling for VIDA: RFC5322 identity, SMTP, IMAP, MIME, Autocrypt/rPGP mail flows, mail-header group membership, and email-based offline delivery.

Evidence gaps: partition delivery guarantees, churn benchmarks, formal group convergence, and completed group-key rotation.
