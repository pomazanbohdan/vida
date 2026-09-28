# Digest: direct transport and application receipts

Checked 2026-09-26. Scope: Stories 2.1–2.3, 2.10–2.11.

- Iroh 1.2 can keep direct and relay paths open simultaneously; a connection/QUIC ACK does not establish which path carried an app payload or whether it was durably applied. [Iroh Connection](https://docs.rs/iroh/1.2.0/iroh/endpoint/struct.Connection.html)
- Iroh builder supports relay transport configuration; custom transport/path-selection capability is unstable. [Iroh Builder](https://docs.rs/iroh/1.2.0/iroh/endpoint/struct.Builder.html)
- The Iroh team calls WebRTC custom transport significant work; an independent prototype exists, but not production validation for VIDA's Android↔Web profile. [Iroh discussion](https://github.com/n0-computer/iroh/discussions/4024), [prototype](https://github.com/anchalshivank/iroh-webrtc-transport/blob/main/README.md)
- Automerge sync trades heads and changes on an ordered reliable stream; it does not prove VIDA authorization, persistence or independent receipt. [Automerge Rust sync](https://automerge.org/automerge/automerge/sync/index.html)

Inference: VIDA needs its own signed application receipt after validated durable apply; direct-first must be enforced and packet/path traced, not assumed from Iroh's connection state.
