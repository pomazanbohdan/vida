---
id: GOV-OPEN-QUESTIONS
status: review
last_updated: 2026-09-18
---

# Відкриті питання

| ID | Контур | Питання | Статус |
|---|---|---|---|
| OQ-0001 | Governance | Чи вважаємо `vida-current-architecture.md` baseline, а інші research-файли — evidence/candidates? | open |
| OQ-0002 | Product | Який мінімальний v1: platform core чи також системні Apps? | open |
| OQ-0003 | Domain | Чи затверджуємо `Space`, `Resource`, `Container` і app model як канонічні терміни? | open |
| OQ-0004 | Network | Iroh уже accepted foundation чи ще потребує decision gate? | open |
| OQ-0005 | Data | CBOR Profile є рішенням, prototype candidate чи research recommendation? | open |
| OQ-0006 | Identity | Яка identity model входить у v1? | open |
| OQ-0007 | Messaging | Інтегруємо Chatmail чи будуємо native Vida protocol? | open |
| OQ-0008 | Client | Які client platforms і технологічні обмеження входять у v1? | open |
| OQ-0009 | Runtime | Wasmtime/WIT і Rhai приймаємо чи спершу прототипуємо? | open |
| OQ-0010 | Workflow | Durable jobs у v1 local-only чи distributed? | open |
| OQ-0011 | Integration | Де розгортається City Portal integration service і як розподіляється source of truth між Vida та City Portal? | open |
| OQ-0012 | Product | Який мінімальний capability set і rollout order для communications, knowledge та work management у v1? | open |
| OQ-0013 | Product architecture | Чи Space є верхнім контейнером для projects, chats, knowledge і tasks, чи Project є типом Space? | resolved: [PROD-COMPOSITION-001](../01-product/composable-workspace-model.md) |
| OQ-0014 | Authorization | Який рівень є типовою одиницею налаштування permissions для Owner: app, container, schema/resource type чи конкретний resource? | resolved: [ADR-0001](../03-architecture/decisions/ADR-0001-layered-access-control.md) |
| OQ-0015 | Authorization | Чи приймаємо шість role presets (Owner, Admin, Manager, Contributor, Commenter, Viewer) та окремі membership classes (Member, Guest, ServicePrincipal)? | resolved: [ADR-0002](../03-architecture/decisions/ADR-0002-default-role-presets.md) |
| OQ-0016 | Authorization | Чи Contributor може за замовчуванням видаляти власні draft resources? | resolved: [ADR-0002](../03-architecture/decisions/ADR-0002-default-role-presets.md) |
| OQ-0017 | Ownership | Чи Space підтримує кількох Owner одночасно? | resolved: [ADR-0002](../03-architecture/decisions/ADR-0002-default-role-presets.md) |
| OQ-0018 | Authorization | Чи приймаємо authority-accepted revocation, control-before-data sync, scope key epochs і quarantine rejected offline operations? | resolved: [ADR-0003](../03-architecture/decisions/ADR-0003-offline-revocation.md) |
| OQ-0019 | Authorization | Які maximum offline mutation acceptance windows застосовуємо для різних risk classes? | open |
| OQ-0020 | Authority | Який authority/quorum приймає revocation у різних типах Space? | resolved: [ADR-0003](../03-architecture/decisions/ADR-0003-offline-revocation.md) |

Питання закривається лише посиланням на accepted ADR або approved normative document.
