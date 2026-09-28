# Static Web and relay feasibility — evidence digest

Accessed: 2026-09-25. Independent research track. Official/project-primary sources.

| Claim | Evidence | Confidence |
| --- | --- | --- |
| GitHub Pages hosts static Flutter build assets but no application backend. | [GitHub Pages](https://docs.github.com/en/pages/getting-started-with-github-pages/what-is-github-pages) | High |
| Flutter Web cannot use native `dart:ffi`; Rust needs separate Wasm build and JS/web bridge. Flutter's `--wasm` does not compile Rust automatically. | [Flutter architecture](https://docs.flutter.dev/resources/architectural-overview); [flutter_rust_bridge Wasm limits](https://cjycode.com/flutter_rust_bridge/manual/miscellaneous/wasm-limitations) | High |
| Browser Iroh Rust/Wasm examples exist; every browser Iroh link currently traverses an Iroh relay, while end-to-end encryption remains. | [Iroh browser guide](https://docs.iroh.computer/languages/wasm-browser) | High |
| Default public Iroh relays are development/test infrastructure, not recommended production dependencies; managed shared or dedicated relay is an alternative to self-hosting. | [Iroh relay guidance](https://docs.iroh.computer/add-a-relay) | High |
| Public Chatmail services use SMTP/IMAP mailbox transport. A Chatmail deployment may separately host an Iroh relay; the mail endpoint is not an Iroh relay and arbitrary VIDA traffic on that separate endpoint is not established. | [Chatmail relays](https://chatmail.at/relays); [architecture](https://chatmail.at/doc/relay/overview.html); [config](https://github.com/chatmail/relay/blob/main/chatmaild/src/chatmaild/ini/chatmail.ini.f) | High for protocol distinction; medium for usage policy |
| Delta Chat Desktop's browser edition is experimental and uses a webserver component; it does not prove a static Pages Delta Core client. | [Delta Chat Desktop](https://github.com/deltachat/deltachat-desktop) | High |
| IndexedDB/OPFS persistence is best-effort unless granted persistent storage, and user deletion/private mode still can lose data. | [MDN storage quotas](https://developer.mozilla.org/en-US/docs/Web/API/Storage_API/Storage_quotas_and_eviction_criteria) | High |
| Same-origin scripts can operate on WebCrypto-held keys and data. GitHub project Pages under one owner share an origin across paths; a dedicated domain is recommended for a vault. | [MDN WebCrypto](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto); [XSS](https://developer.mozilla.org/en-US/docs/Web/Security/Attacks/XSS); [same-origin policy](https://developer.mozilla.org/en-US/docs/Web/Security/Defenses/Same-origin_policy); [GitHub Pages URL layout](https://docs.github.com/en/pages/getting-started-with-github-pages/what-is-github-pages) | High, origin implication inferred |

Feasible prototype hypothesis: static Flutter Web UI → JS/Wasm boundary → VIDA Rust/Wasm core/browser Iroh endpoint → public test Iroh relay → Android Iroh endpoint. This is **not yet an implementation proof**. Pure static browser operation of Delta Chat Core mail transport remains unproven; do not conflate Chatmail with the Iroh relay.

Proof gaps: actual Rust `wasm32` dependency build; Android↔Web encrypted sync and reconnect; durable browser store/recovery; multi-tab; Flutter Wasm COOP/COEP hosting; production relay contract and costs.
