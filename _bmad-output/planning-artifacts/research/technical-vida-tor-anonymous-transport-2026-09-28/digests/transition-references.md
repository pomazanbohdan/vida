# Tor transition reference digest — 2026-09-28

Scope: existing Space policy transition, stale/offline devices, route leakage, and offline toggle ordering. Sources below are primary project, standards, research, or OWASP sources; accessed 2026-09-28. Their publication dates are not inferred from access date.

| ID | Source / publication | Direct evidence | VIDA inference | Confidence |
|---|---|---|---|---|
| R1 | Briar, [How it works](https://briarproject.org/how-it-works/), n.d. | Tor internet sync; Bluetooth/Wi-Fi/memory-card offline sync. | Data model can have multiple transports; no proof of route-policy migration. | high |
| R2 | SimpleX, [App settings](https://simplex.chat/docs/guide/app-settings.html), n.d. | `.onion required` fails if unavailable; `when available` is distinct. | Strict VIDA Space must fail closed. | high |
| R3 | Iroh, [Tor custom transport](https://www.iroh.computer/blog/tor-custom-transport), 2026-01-27 | Tor custom transport; author calls API experimental and says privacy requires disabling ordinary transports/lookup; sample disables Tor control auth. | Separate Tor-only endpoint and authenticated control, subject to prototype. | high |
| R4 | Iroh, [Builder 1.2](https://docs.rs/iroh/latest/iroh/endpoint/struct.Builder.html), n.d. | `clear_ip_transports`, `clear_relay_transports`, `clear_address_lookup`; custom transport/path selector marked unstable. | Builder may support isolation but must be tested in release profile. | high |
| R5 | Tor Project, [Stream isolation](https://spec.torproject.org/path-spec/stream-isolation.html), n.d. | Separate accounts/sessions should not share circuits; application-provided isolation token is strong. | Isolate VIDA Personas and route contexts. | high |
| R6 | Tor Project, [SOCKS extensions](https://spec.torproject.org/socks-extensions), n.d. | SOCKS is TCP; no UDP ASSOCIATE; local DNS can leak. | Generic SOCKS-wrapped QUIC/WebRTC is not a Tor proof. | high |
| R7 | Tor Project, [Arti TorClient](https://docs.rs/arti-client/latest/arti_client/struct.TorClient.html), n.d. | Rust TorClient supports bootstrap, onion service methods and isolated client. | Runtime candidate only; mobile/FFI/lifecycle unproven. | high |
| R8 | OWASP, [MASVS-PRIVACY-1](https://mas.owasp.org/MASVS/controls/MASVS-PRIVACY-1/), n.d. | Minimize sensitive data and third-party access. | Control-only bootstrap must be minimal; network contact still has metadata. | high |
| R9 | Lamport, [Time, Clocks, and the Ordering of Events](https://lamport.azurewebsites.net/pubs/time-clocks.pdf), 1978; Google, [Spanner TrueTime](https://docs.cloud.google.com/spanner/docs/true-time-external-consistency), n.d. | Causal partial order; external physical-time order needs bounded uncertainty and protocol. | Exact real-time order of arbitrary disconnected device actions cannot be inferred from millisecond display precision. | high for limitation; proposed handling unverified |

No source directly establishes VIDA's proposed policy-only handshake, receipt semantics, or seamless in-place Space transition. Those are original design proposals and require fault-injection plus network-capture proof.
