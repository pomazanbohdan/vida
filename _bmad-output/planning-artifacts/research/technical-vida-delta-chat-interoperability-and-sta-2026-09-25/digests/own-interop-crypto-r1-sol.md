# Crypto, identity, SecureJoin and Rust libraries — digest

Accessed: 2026-09-25. Primary Delta/Autocrypt/current source and crate docs. No cross-client test.

| Claim | Source | Confidence/class |
| --- | --- | --- |
| Delta Chat uses a secure OpenPGP subset, Autocrypt 1.1 and SecureJoin; whole message/attachments encrypted and signed; Autocrypt v2/PFS/PQ remain future work. | [Delta FAQ](https://delta.chat/en/help) (living doc) | high, current profile |
| Chatmail spec says `Chat-Version: 1.0` MUST; Autocrypt encryption and protected headers RFC 9788 SHOULD. | [Chatmail spec](https://github.com/chatmail/core/blob/main/spec.md) (current in-progress) | high documented, protocol |
| Autocrypt L1 defines `addr`, `prefer-encrypt`, base64 `keydata`; Ed25519 signing and Curve25519 ECDH support MUST, RSA SHOULD. | [Autocrypt Level 1](https://docs.autocrypt.org/level1.html) (v1.1/2019) | high, normative |
| Current Core generates legacy Ed25519/Curve25519 ECDH keys and advertises SEIPD v2 plus AES256/192/128 preferences, while its current symmetric-key constant is AES128; a new peer needs packet-level fixture verification, not assumptions from advertised preferences. | [Core PGP source](https://github.com/chatmail/core/blob/main/src/pgp.rs) (main observed 2026-09-25) | high source; medium compatibility inference |
| Public SecureJoin prose is outdated for post-2025 key-contact identity; current Core source includes v3 `vc-request-pubkey`/`vc-pubkey`, auth exchange and legacy paths. | [SecureJoin docs](https://securejoin.delta.chat/); [Core securejoin source](https://github.com/chatmail/core/blob/main/src/securejoin.rs) (main 2026-09-25) | high warning/source, medium exact interpretation |
| QR URI may include fingerprint/address/invite/auth and group fields; never equate address to permanent identity. | [Delta URI schemes](https://github.com/deltachat/interface/blob/main/uri-schemes.md) | high format, medium identity inference |
| `pgp`/rPGP 0.20.0 (2026-06-23), MIT/Apache-2.0, is low-level and current Core itself uses it. It still needs VIDA application policy, certificate/key continuity and validation. | [rPGP docs](https://docs.rs/pgp/latest/pgp/); [Core Cargo.toml](https://github.com/chatmail/core/blob/main/Cargo.toml) | high version/dependency; medium suitability |
| `sequoia-openpgp` 2.4.1 (2026-07-09) is LGPL-2.0-or-later, with backend/platform/Wasm constraints; do not pick it from API breadth alone. | [Sequoia Cargo manifest](https://docs.rs/crate/sequoia-openpgp/latest/source/Cargo.toml), [README](https://docs.rs/crate/sequoia-openpgp/latest/source/README.md) | high version/license/platform |

Prototype gates: current stock-client 1:1/group encrypted text/media; key import/continuity; Autocrypt header state; verify/decrypt malformed or tampered MIME; v3 SecureJoin both directions, replay and QR tamper; old RSA sender and v6/AEAD inbound as compatibility probes. Do not implement from old SecureJoin prose alone; use current source as a test oracle without copying it blindly.

Open: exact outbound packet negotiation, protected-header behavior, group key discovery/rekey on membership changes, full v3 token/QR semantics, Sequoia Windows/Wasm production path. rPGP main mentions PQ support, but stock Delta Chat's current interoperability profile does not require it; assess released-version readiness separately.
