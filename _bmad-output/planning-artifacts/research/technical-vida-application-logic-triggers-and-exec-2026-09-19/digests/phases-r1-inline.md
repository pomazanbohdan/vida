# Phases and effects — round 1 (inline)

Accessed 2026-09-19. Primary sources only; docs undated/current unless noted.

| Claim | Source | Publisher | Date | Confidence/class |
|---|---|---|---|---|
| PostgreSQL distinguishes BEFORE/AFTER/INSTEAD OF trigger timing; its AFTER trigger remains in the same transaction, so it is not synonymous with after-commit delivery. | https://www.postgresql.org/docs/18/trigger-definition.html | PostgreSQL Global Development Group | undated | medium / pattern |
| AWS outbox writes data/event atomically and dispatches only committed work; duplicates remain possible, so consumers need idempotency. | https://docs.aws.amazon.com/prescriptive-guidance/latest/cloud-design-patterns/transactional-outbox.html | AWS | undated | medium / pattern |
| Supabase database webhooks use asynchronous pg_net and fire after INSERT/UPDATE/DELETE row changes. | https://supabase.com/docs/guides/database/webhooks | Supabase | undated | medium / pattern |
| Wasmtime offers fuel/epoch interruption of guest execution, but the host is responsible for what its functions can do. | https://docs.wasmtime.dev/examples-interrupting-wasm.html | Bytecode Alliance / Wasmtime | undated | medium / executor |
| Rhai operation count can cap scripts, but a counted external Rust function call may consume arbitrary work. | https://rhai.rs/book/safety/max-operations.html | Rhai | undated | medium / executor |

Lead: distinguish pre-commit pure decision/validation, in-transaction host acceptance, and committed-event durable effects; do not infer direct I/O safety from scripting budget.
