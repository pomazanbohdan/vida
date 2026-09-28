# Project App conformance cases

These cases preserve approved [Release-1](../../../docs/01-product/v1-replacement-bundle.md), [composition](../../../docs/01-product/composable-workspace-model.md), [client](../../../docs/02-requirements/native-client-requirements.md) and [conflict](../../../docs/03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md) behavior. They do not claim that the serverless authority implementation exists.

| ID | Scenario and observable result | Source |
|---|---|---|
| PROJECT-F01 | Create first personal Project. It belongs to Personal Space and is not shared merely because the user later creates a team Project. | REQ-CLIENT-017; PROD-COMPOSITION-001 |
| PROJECT-F02 | Invite another Persona to team work. VIDA requires a separate or existing Shared Space; the invitation never turns Personal Space into Shared Space. | REQ-CLIENT-017; PROD-V1-BUNDLE-001 |
| PROJECT-F03 | Create task and subtask with status, assignee and date; switch list ↔ board. Both views refer to the same authorized resources and current state. | REQ-CLIENT-015 |
| PROJECT-F04 | Link task ↔ note, document, chat, forum topic and File. Navigation works for authorized targets; an unreadable target is not exposed by link or search. | REQ-CLIENT-015/022; PROD-COMPOSITION-001 |
| PROJECT-F05 | Open project chat, forum topic and task-specific discussion. Standard communication capability is reused; topic and task remain addressable, without a duplicate Project-only messenger history. | REQ-CLIENT-018; PROD-COMPOSITION-001 |
| PROJECT-F06 | Offline, change task status, edit linked note, create task↔note relation and stage a photo/file. After confirmed local save and process death, all committed components reopen as local/pending. | REQ-CLIENT-004/007/009 |
| PROJECT-F07 | Device reconnects after rights proof expires or membership changes. Shared-read lock and pending acceptance follow current ACL; a revoked actor cannot make a local candidate accepted by delivery alone. | REQ-CLIENT-004; REQ-ACL-016; ADR-0017 |
| PROJECT-F08 | Two incompatible task-status transitions have verifiably ordered authority acceptance. First accepted value is current; the later author's intent remains visible/recoverable and can be resubmitted only as a new authorized operation. | REQ-SYNC-004; ADR-0017 |
| PROJECT-F09 | Two incompatible status transitions are accepted without comparable order. All authorized peers show one explicit unresolved conflict with both readable variants, no automatic winner or device priority. | REQ-SYNC-004/008; ADR-0017 |
| PROJECT-F10 | An authorized actor resolves both visible status variants. The resolution causally covers both and becomes a new operation; prior operations remain in history. An actor unable to read a variant sees a safe conflict signal only. | ADR-0017 |
| PROJECT-F11 | Two offline actors independently resolve one conflict incompatibly and neither acceptance dominates. Reconciliation opens a new explicit conflict; equivalent decisions with equal consequences show one result but retain both audits. | ADR-0017 |
| PROJECT-F12 | Activate Project later in Personal Space, then use it on Android, iOS and Windows. Other bundled Apps remain usable; Windows keyboard/screen-reader journeys and all three offline/restart paths pass. | REQ-CLIENT-001/005/006/016/024 |

## Evidence and preservation boundary

- Each case needs durable local state, signed actor/current-rights proof and cross-device application receipts. A transport ACK or CRDT materialization alone does not establish Project acceptance.
- The product sources approve tasks, subtasks, list/board, team Space separation and composition with Notes, Messenger/Forum and Files. They do **not** settle exact Project Container nesting, dependent-App manifest, cross-Space grants or serverless authority topology.
- CRM, City Portal/business booking, Gantt, time tracking, budgeting and advanced analytics are explicitly outside this Release-1 kernel.
