# VIDA post-commit effect execution authority — research brief

- Decision served: choose who may execute a committed-fact reaction exactly once in practice (idempotent external outcomes), separately for autonomous personal and federated Spaces.
- Scope: notification/message/webhook/booking-style external effects; local UI projection is excluded.
- Dimensions: (1) local-first/offline executor ownership; (2) federated/server/keeper roles and E2EE trust; (3) durable workflow/outbox, dedupe, failover, fencing and external API limits; (4) option trade-offs and minimum prototype tests.
- Research shape: explore and compare candidate models; no candidate is approved before user discussion.
- Sources: primary official docs and papers; project documents set constraints but do not count as external evidence.
- Validation: normal, cross-check load-bearing claims; no new technology version selected.
- User clarification: in a fully autonomous personal Space, external effects may wait until an owner device is online; an always-on helper is not required by default.
- User clarification: in a private shared project the federated node sends only an opaque signal; the recipient device decrypts task details and composes the user-facing notification. Do not extrapolate this answer to every public/business Space.
