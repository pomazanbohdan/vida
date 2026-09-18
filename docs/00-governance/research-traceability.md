---
id: GOV-TRACEABILITY
status: review
last_updated: 2026-09-18
---

# Трасування досліджень

`research/` є доказовою базою, але не нормативним контрактом.

| Джерело | Майбутні документи | Поточна оцінка |
|---|---|---|
| `vida-current-architecture.md` | Architecture overview, domain, authorization, storage, sync | Найсильніший baseline-кандидат; містить прийняті принципи й відкриті питання |
| `storage-format-architecture-research.md` | PDM, schema registry, binary profile, CAS, projections | ADR-кандидат; benchmark/conformance відсутні |
| `iroh-local-first-research-index-2026-09-18.md` | Network/sync evidence appendix | Evidence catalog; не специфікація |
| `Новий Text Document.txt` | Governance | Корисний процес статусів |
| `(2)` | Product/SaaS/portal | Product evidence; Dioxus відкликано |
| `(3)` | App/plugin runtime | Wasmtime/WIT, Rhai та Extism — кандидати |
| `(4)` | Jobs/workflows | Apalis, Duroxide, Restate, NATS — кандидати |
| `(5)` | Identity/federation | Candidate model; privacy/recovery unresolved |
| `(6)` | UX/client | UX evidence; framework не визначено |
| `(7)` | Messaging integration | Chatmail/Delta reuse — кандидат |
| `(8)` | Messaging/community | Scale/domain evidence; native accepted-log — кандидат |
| `(9)` | Platform architecture | Важливий precursor; частково superseded термінами |
| `(10)` | Local-first storage | Signed operations/files — кандидат; частково superseded |
| `(11)` | Edge profile | ESP32 feasibility; experimental/later candidate |
| `(12)` | Protocol patterns | Nostr/AT/W3C/IETF/OCI — evidence only |
| `(13)` | Iroh profile | Evidence; версії/API потрібно перевіряти live |
| `(14)` | PM requirements | Feature-discovery evidence; не v1 commitment |
| `(15)` | Productivity requirements | TickTick/Keep evidence; не v1 commitment |

## Відомі конфлікти

- UI: Dioxus відкликано; `.NET/Blazor Hybrid` — стара пропозиція; framework не обрано.
- Messaging: Chatmail/Delta core проти власного Vida protocol.
- Storage: signed operation files проти PDM + deterministic CBOR; фінальний contract не затверджено.
- Iroh: transport foundation прийнятий у research baseline, але components/API/FFI/node profiles лишаються prototype scope.
- CRDT: чинний baseline обмежує його workload-specific випадками.
- PostgreSQL: відхилений як єдина база, дозволений як server provider/projection.
- Naming: `OwnershipScope` → `Space`; `PersonalVida` → `Vida`.

