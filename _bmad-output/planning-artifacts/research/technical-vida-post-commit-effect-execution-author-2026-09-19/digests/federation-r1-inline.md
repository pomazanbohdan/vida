# Federated executor and privacy — round 1

| Claim | Source | Publisher | Pub date | Accessed | Confidence / class |
|---|---|---|---|---|---|
| Matrix separates homeserver event receipt from Push Gateway and push provider; event-ID-only format minimizes content but still routes per-device notification metadata. The gateway deduplicates on event ID. | https://spec.matrix.org/v1.19/push-gateway-api/ | Matrix Specification | v1.19, 2026-07 | 2026-09-19 | high; protocol pattern |
| Signal servers temporarily queue E2EE messages for delivery while lacking decryption keys; content-dependent automation cannot be inferred from ciphertext alone. | https://signal.org/blog/signal-is-expensive/ | Signal | 2023 (historical) | 2026-09-19 | medium; privacy pattern |
| Cloudflare Durable Object alarms provide at-least-once execution, retries and a single scheduled alarm per object, demonstrating a hosted worker pattern but not exactly-once external API results. | https://developers.cloudflare.com/durable-objects/api/alarms/ | Cloudflare | 2026-04-21 | 2026-09-19 | high; hosted execution pattern |

Leads: separate opaque wake-up/push, domain notification intent, and content-dependent business automation; inspect a user-granted service principal rather than silently granting node plaintext.

Not found: evidence that an E2EE-blind federation node can evaluate arbitrary private-content business rules.
