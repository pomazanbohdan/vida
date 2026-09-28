# Federation: primary-source digest

- Iroh shared relay forwards end-to-end encrypted traffic and is stateless; it does not store or read message content. It is not a mailbox, account manager, or VIDA domain authority. Publisher Iroh; 2026-09-08; accessed 2026-09-20. [5]
- AT Protocol distinguishes PDS account/data hosting, relay distribution, and application-specific App Views; roles can be deployed separately. Its public-social trust model is not automatically a private E2EE model for VIDA. Publisher AT Protocol; publication date not stated; accessed 2026-09-20. [6]
- Matrix federation synchronizes room events between homeservers with separate event authorization/state resolution. This shows why a federated server's visibility and authority must be explicit, not inferred from being a relay. Publisher Matrix.org Foundation; publication date not stated; accessed 2026-09-20. [7]
- Inference for VIDA: describe deployable node capabilities independently; co-location does not implicitly confer Space authority or plaintext keys. Exact managed-mode policy remains open.

[5]: https://www.iroh.computer/blog/shared-relays
[6]: https://atproto.com/guides/the-at-stack
[7]: https://spec.matrix.org/latest/server-server-api/
