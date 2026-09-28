# Offline conflict and effect execution: primary-source digest

- Automerge chooses a deterministic concurrent map-value winner by operation ID, not wall-clock time; losing values remain accessible through conflicts. Publisher Automerge; publication date not stated; accessed 2026-09-20. [8]
- Loro Map uses Lamport logical timestamps for LWW and offers a mergeable child by logical key. Neither mechanism proves which device acted first in physical time. Publisher Loro; publication date not stated; accessed 2026-09-20. [9]
- AWS transactional outbox explicitly warns that events can be duplicated; consumers need idempotency. Publisher AWS; publication date not stated; accessed 2026-09-20. [10]
- Matrix distinguishes a server-origin event timestamp and a client-generated transaction ID that makes send retries idempotent per device/endpoint. Publisher Matrix.org Foundation; publication date not stated; accessed 2026-09-20. [13]
- Nostr NIP-01 carries a signed client-created `created_at` timestamp and illustrates relay rejection when that timestamp is too far from current time. Publisher Nostr protocol contributors; publication date not stated; accessed 2026-09-20. [14]
- Inference for VIDA: preserve both task-status candidates and require domain authority/human confirmation for conflict semantics; one derived note needs stable logical key and dedupe; external effect needs durable intent plus provider-supported idempotency/status check. Iroh is transport, not an effect executor.

[8]: https://automerge.org/docs/reference/documents/conflicts/
[9]: https://www.loro.dev/docs/tutorial/map
[10]: https://docs.aws.amazon.com/prescriptive-guidance/latest/cloud-design-patterns/transactional-outbox.html
[13]: https://spec.matrix.org/latest/client-server-api/
[14]: https://github.com/nostr-protocol/nips/blob/master/01.md
