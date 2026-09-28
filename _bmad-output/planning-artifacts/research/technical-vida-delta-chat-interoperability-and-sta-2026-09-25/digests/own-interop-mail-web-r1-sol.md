# Chatmail account, mail transport, browser boundary — digest

Accessed: 2026-09-25. Official Chatmail, browser-platform and crate sources. No implementation test.

| Claim | Source | Confidence/class |
| --- | --- | --- |
| `DCACCOUNT:domain` generates a 9-character lowercase-alphanumeric username and password of ≥30 alphanumeric characters; `DCLOGIN` imports existing credentials. | [Delta Chat URI schemes](https://github.com/deltachat/interface/blob/main/uri-schemes.md) (current) | high, onboarding |
| Chatmail creates the address at first authenticated mail login; operator may refuse new signup when load/disk/policy requires. | [Relay getting started](https://chatmail.at/doc/relay/getting_started.html) (current) | high, operations |
| A Chatmail deployment exposes SMTP submission, IMAPS and HTTPS/autoconfig; `doveauth` implements create-on-login. A QR/page alone does not send or receive mail. | [Technical overview](https://chatmail.at/doc/relay/overview.html) (current) | high, architecture |
| Public independently run Chatmail relays are listed; list membership does not imply access/uptime or permission for arbitrary product-scale provisioning. | [Relay list](https://chatmail.at/relays); [relay setup](https://chatmail.at/doc/relay/getting_started.html) | high list, medium policy inference |
| Current Core uses native Rust Tokio/async-imap/async-smtp/SQLCipher, but importing Core is separate from implementing protocol independently. | [Core README](https://github.com/chatmail/core); [Cargo.toml](https://github.com/chatmail/core/blob/main/Cargo.toml) (current main) | high, implementation |
| Ordinary static browser pages cannot open arbitrary SMTP/IMAP TCP sockets; Chrome Direct Sockets applies to packaged Isolated Web Apps, not ordinary static Flutter Web. Browser Chatmail therefore needs provider HTTPS/WS API, a mail bridge or a native companion. | [Chrome Direct Sockets](https://developer.chrome.com/docs/iwa/direct-sockets) (current) | high platform fact, medium architecture inference |
| Candidate lower-level crates: `async-imap` 0.11.3 (2026-07-17), `lettre` 0.11.23 (2026-08-03). They do not implement Delta Chat encryption/group semantics. | [async-imap docs.rs](https://docs.rs/crate/async-imap/latest), [lettre docs.rs](https://docs.rs/crate/lettre/latest) | high versions, medium coverage inference |

Proof gates: consenting public relay account signup/relogin; native mail send/receive with stock Delta; secure credential custody; test browser API/bridge feasibility and CORS/auth; operator policy/abuse constraints. No retrieved evidence that listed public Chatmail relays expose supported general-purpose browser mail API. A pure static web page plus public Chatmail alone has no demonstrated full-messenger path.
