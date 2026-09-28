# Iroh application catalog — second-agent digest

Accessed: 2026-09-18. Baselines: https://github.com/n0-computer/awesome-iroh and https://raw.githubusercontent.com/n0-computer/docs.iroh.computer/main/examples.mdx.

## Reusable patterns

1. Iroh is dial-by-public-key QUIC/NAT/relay connectivity; the application owns trust, membership, authorization, and durable history.
2. Gossip is the low-latency hot path; pull/reconcile is the cold recovery path; content-addressed blobs carry large immutable payloads.
3. QR or short tickets bootstrap capabilities, but the application must bind a verified human/device before granting durable trust.
4. Account, signing, transport, and session keys should remain distinct; revocation needs explicit epochs or membership-log transitions.
5. Offline-first requires a local replica plus outbox/acknowledgement semantics; Iroh relay alone is not a durable queue.
6. A CAS data plane can be paired with a signed canonical-pointer/control plane.
7. Each VIDA protocol should have a distinct, versioned ALPN.

## Maturity and selection

- **Adapt/prototype:** p2panda layering, GuardianDB reconciliation/event-log ideas, Kith membership/epoch model.
- **Reference:** Rayfish control plane, iroh-lan transient spaces, Teamtype editor boundary, Proscenium key hierarchy/outbox, Sendme/Dropwire transfer, Knot pointer/data separation, git-remote-iroh protocol tunnelling.
- **Reject as direct dependency for now:** alpha/prototype apps without independent security evidence; AGPL/MPL components absent an explicit licensing decision; roadmap-only or thin/stale projects.

Catalogs are discovery aids, not proof. Every inclusion above was checked against its own repository; Hubris remains a lead only because the linked source was inaccessible.
