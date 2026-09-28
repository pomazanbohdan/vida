# Delta Chat / Chatmail identity — evidence digest

Accessed: 2026-09-25. Independent research track. Official sources only.

| Claim | Evidence | Confidence |
| --- | --- | --- |
| A Delta Chat profile can be created without a phone number, prior email or name; relay address is randomly assigned and device stores profile keys. | [Delta Chat privacy policy](https://delta.chat/en/privacy), published September 2026 | High |
| Chatmail relay provisions an address at first login; Core exposes JSON-RPC account and transport operations. This supports an integration hypothesis, not a completed VIDA flow. | [Chatmail setup](https://chatmail.at/doc/relay/getting_started.html); [Core JSON-RPC API](https://github.com/chatmail/core/blob/main/deltachat-jsonrpc/src/api.rs), undated/current | High for APIs; medium for integration |
| Multi-device QR profile transfer and backup restore exist; current Delta Chat UI does not offer arbitrary manual key reuse. | [Delta Chat FAQ](https://delta.chat/en/help), undated/current | High |
| Delta Chat relay address is a transport address, not a suitable permanent VIDA Persona identifier. | [Delta Chat privacy policy](https://delta.chat/en/privacy); [Delta Chat FAQ](https://delta.chat/en/help) | Architectural inference, medium |
| Relay sees routing/network metadata; anonymous signup does not guarantee network anonymity. | [Delta Chat privacy policy](https://delta.chat/en/privacy) | High |
| `nine.testrun.org` public relay publishes 60 messages/min, 700 MB and 20-day message retention; its older privacy text is not fully consistent on retention. | [Operator information](https://nine.testrun.org/info.html); [privacy policy](https://nine.testrun.org/privacy.html) | Medium; recheck before use |

Open verification: account/key continuity across relay migration, backup/recovery after lost devices, public-relay acceptable-use and capacity for third-party clients, separate VIDA Persona identity binding, pairwise anonymity/correlation.
