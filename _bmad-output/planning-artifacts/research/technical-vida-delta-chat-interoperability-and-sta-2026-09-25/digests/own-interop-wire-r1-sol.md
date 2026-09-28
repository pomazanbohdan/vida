# Independent Delta Chat-compatible wire protocol — digest

Accessed: 2026-09-25. Official Chatmail/Delta/RFC sources; no stock-client test executed.

| Claim | Source (publisher/date) | Confidence/class |
| --- | --- | --- |
| Public Chatmail relay filters cleartext mail; `Chat-Version`+MIME without encryption is insufficient. | [Relay overview](https://chatmail.at/doc/relay/overview.html) (Chatmail, undated/current) | high, operational |
| `DCACCOUNT:` transports use generated 9-character lowercase-alphanumeric username and ≥30-character password; first IMAP login creates the account; autoconfig supplies server endpoints. | [URI schemes](https://github.com/deltachat/interface/blob/main/uri-schemes.md) (Delta Chat, current); [relay overview](https://chatmail.at/doc/relay/overview.html) | high, onboarding |
| Wire baseline is RFC 5322 mail plus MIME RFC 2045/2046 over SMTP submission and IMAP, with `Chat-Version: 1.0` and text/plain. | [Standards list](https://github.com/chatmail/core/blob/main/standards.md) and [spec](https://github.com/chatmail/core/blob/main/spec.md) (Chatmail, current); [RFC 5322](https://www.rfc-editor.org/info/rfc5322/) (2008); [RFC 2045](https://www.rfc-editor.org/info/rfc2045/), [RFC 2046](https://www.rfc-editor.org/info/rfc2046/) (1996) | high, normative |
| Autocrypt peer-key discovery and PGP/MIME sign/encrypt are necessary for encrypted exchange. Exact current algorithm interoperability remains to verify against stock clients. | [Autocrypt Level 1](https://docs.autocrypt.org/level1.html) (Autocrypt v1.1); [Chatmail spec](https://github.com/chatmail/core/blob/main/spec.md) | high mechanism; medium current profile, crypto |
| Group messages carry group ID/name, all members in From/To and a group-related Message-ID; membership changes require explicit add/remove headers, not inference from ordinary recipient changes. | [Chatmail spec](https://github.com/chatmail/core/blob/main/spec.md) (current, marked in-progress) | high documented format; medium completeness, protocol |
| The spec also defines name/avatar actions, edit/delete by Message-ID and reactions using RFC 9078; these are later profile candidates, not needed for first minimal group-text proof. | [Chatmail spec](https://github.com/chatmail/core/blob/main/spec.md); [RFC 9078](https://www.rfc-editor.org/info/rfc9078/) (2021) | high, extensions |
| `Chat-Version: 1.0` is a wire header; no retrieved evidence of negotiated versioning or general forward-compatibility guarantee. | [Chatmail spec](https://github.com/chatmail/core/blob/main/spec.md) | medium, compatibility |

Engineering inferences, not normative claims: durable outbox, stable Message-ID on retry, inbound idempotency by Message-ID, causal group-action replay, unknown-header safe handling. Need conformance tests rather than treating these as stock behavior guarantees.

Golden test set: account provisioning/relogin; stock↔VIDA encrypted 1:1 text and media; two groups with same roster retain distinct IDs; explicit add/remove while offline/reordered; ordinary To change never changes roster; duplicate redelivery yields one message; edit/delete/reaction as later profile; unknown extensions preserved/ignored safely.

Open: exact SecureJoin handshake and peer verification; current stock media MIME samples; group key availability after membership changes; spec in-progress divergences; exact delivery/duplicate behavior. No end-to-end implementation proof yet.
