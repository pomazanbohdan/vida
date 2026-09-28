# Reviewer gate — Web direct-first transport currentness (2026-09-25)

**Verdict: conditional pass for architecture direction; not yet a verified Release-1 Web implementation.** AD-1 and AD-20 correctly separate stock browser Iroh relay behavior from a prospective direct WebRTC/custom transport and preserve encrypted relay fallback. Keep the direct path as an explicit prototype/release gate, not as a capability already delivered by Iroh 1.2 or the cited community crates.

## High — The chosen Iroh 1.2 and available WebRTC bridges are not demonstrated compatible

- `ARCHITECTURE-SPINE.md:112,226` binds Iroh core 1.2, but the [original `iroh-webrtc-transport` manifest](https://github.com/anchalshivank/iroh-webrtc-transport/blob/main/Cargo.toml) pins Iroh **0.97** and `unstable-custom-transports`; the [separate MIT-labeled repository's manifest](https://github.com/SuddenlyHazel/iroh-webrtc-transport/blob/main/crates/iroh-webrtc-transport/Cargo.toml) uses **0.98.2**, declares **0.1.0-alpha.2**, and likewise uses that unstable feature. The latter README explicitly calls its API experimental ([source](https://github.com/SuddenlyHazel/iroh-webrtc-transport)). Iroh's [current 1.2 API documentation](https://docs.rs/iroh/latest/src/iroh/socket/transports/custom.rs.html) still gates custom transport behind an unstable feature. No source here proves that either bridge builds and works with the mandated Iroh 1.2, VIDA's Flutter Web/Rust-Wasm shell, or Android.
- **Action:** maintain the current direct-first *policy* but make the Web direct path a blocking, version-pinned compatibility proof. Do not name either community crate as a production dependency until a pinned 1.2 build, Android↔Web and Web↔Web tests, browser matrix, security review, and maintenance/licensing assessment pass. The original repository also has an [open no-license issue](https://github.com/anchalshivank/iroh-webrtc-transport/issues/1); copying it is not a safe default. A low version number alone is not the objection; compatibility and proof are.

## High — Browser direct connectivity is conditional, not a stock Iroh property

- Iroh 1.2's [browser endpoint source](https://docs.rs/iroh/latest/src/iroh/endpoint.rs.html) says the browser advertises a relay URL and no direct address because browsers lack raw socket APIs. Iroh's [browser announcement](https://www.iroh.computer/blog/iroh-0-32-0-browser-alpha-qad-and-n0-future) explicitly describes browser Iroh as relay-only absent deep WebRTC integration. An Iroh maintainer's [WebRTC discussion](https://github.com/n0-computer/iroh/discussions/4024) treats WebRTC as an external/custom transport project, not an upstream production capability. Community demos demonstrate feasibility, not VIDA production conformance.
- **Action:** read `direct-first` in AD-1 as routing preference contingent on proven WebRTC support. In AD-20's release gate, explicitly fail Web direct evidence if the negotiated WebRTC candidate pair is relay/TURN rather than host/server-reflexive/peer-reflexive, and require a real Android↔static-Web transfer with application receipt. Relay fallback may still carry end-to-end encrypted sync when direct is unavailable, exactly as the user approved.

## Medium — Prove VIDA relay ownership and distinguish signaling from data forwarding

- The bridge's [browser/native demo](https://github.com/anchalshivank/iroh-webrtc-transport/blob/main/README.md) uses `presets::N0` and/or an optional separate WebSocket signaling server; neither is automatically VIDA-operated infrastructure. [Iroh Builder docs](https://docs.rs/iroh/latest/iroh/endpoint/struct.Builder.html) say the default relay set is N0 and support custom relay configuration. The direct WebRTC path may use relay-carried SDP/ICE signaling; that does **not** mean user sync payload traverses the relay. Conversely, fallback *does* forward encrypted payload through relay.
- **Action:** fixture-configure only explicitly allowed VIDA relay URLs; test no accidental N0 default or surprise TURN. Record path selection and relay byte classes (`lookup/signaling`, `encrypted application forwarding`) separately without collecting plaintext. Verify peer authentication, protocol envelope identity and idempotent receipt survive direct↔relay migration.

## Medium — Direct-first cannot mean direct-only or guaranteed direct on arbitrary networks

- Iroh's [WebRTC issue](https://github.com/n0-computer/iroh/issues/3250) reports limited exploratory WAN results and cautions about ICE complexity; those percentages are not universal forecasts. Restrictive NAT/firewall and browser lifecycle can defeat a direct path. Thus the authorized encrypted relay fallback is necessary for the promised Android/Web sync. The current AD-1/AD-20 wording is acceptable if no timing or universal direct-connect guarantee is inferred.
- **Action:** test LAN, separate household networks, mobile carrier NAT, restrictive Wi-Fi/UDP blocked, page reload/suspend, and direct↔relay reconnection. Assert one signed operation ID and one materialized note regardless of route; show pending when neither route works.

## Scope of this review

- Read-only review of `ARCHITECTURE-SPINE.md` AD-1/AD-20, with supporting source checks as linked. No dependency, implementation, test run, packet trace, or cross-platform prototype was produced. Other architecture decisions were not validated here.
