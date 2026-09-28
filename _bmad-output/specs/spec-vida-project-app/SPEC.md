---
id: SPEC-vida-project-app
status: draft
companions:
  - project-conformance-cases.md
  - ../../../docs/01-product/v1-replacement-bundle.md
  - ../../../docs/01-product/composable-workspace-model.md
  - ../../../docs/02-requirements/native-client-requirements.md
  - ../../../docs/03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md
  - ../../../docs/02-requirements/transport-sync-requirements.md
sources: []
---

> **Decision-gated contract.** The approved Project App behavior is assembled here. This draft does not invent the unresolved project-container layout, serverless authority proof or dependency manifest.

# VIDA Project App

## Why

A person should manage private work and collaborate with a team in one connected context: tasks, knowledge, discussions and files. Project is a composed App, not a separate task silo or CRM.

## Capabilities

- **CAP-1**
  - **intent:** A user can start a personal project and explicitly create or choose a separate Shared Space for team work.
  - **success:** A personal project stays in Personal Space; inviting a team never silently shares that Space, and a team Project is accessible only under its Shared Space membership and grants.
- **CAP-2**
  - **intent:** A user can manage tasks and subtasks with statuses, assignees and dates in list or board form.
  - **success:** Authorized participants see the same task identity and state in both views; a change made in one view appears in the other after local apply/sync as appropriate.
- **CAP-3**
  - **intent:** A user can relate a task to a note, document, chat, forum topic or file.
  - **success:** The link opens an authorized target in context without copying its content or granting access merely because the relation exists.
- **CAP-4**
  - **intent:** A team can discuss the project in a chat stream, durable forum topics and task/document-specific conversations.
  - **success:** Participants use the standard Messenger/Forum capability inside Project; a discussion and its related resource remain addressable from either context.
- **CAP-5**
  - **intent:** A user can read locally available project data and prepare permitted task/link/file changes without a network.
  - **success:** A confirmed local save survives restart; pending, synchronized and conflicted outcomes remain distinguishable, and reconnect does not silently discard committed intent.
- **CAP-6**
  - **intent:** An authorized participant can understand and resolve incompatible task-status changes.
  - **success:** Comparable domain acceptances keep the first accepted value; incomparable accepted branches show both variants with no implicit winner, and a resolution is a new authorized action that preserves history.

## Constraints

- Project is a bundled schema-driven `AppInstance` composed with standard Notes/Knowledge, Messenger/Forum and Files in a Space. It does not define a second identity, ACL, storage or sync plane.
- Space membership and action grants govern every task and linked resource. Neither dependency activation nor a resource relation grants access to another resource.
- Offline local commit is not domain acceptance. Shared read follows the approved seven-day rights-proof rule; equal devices and client timestamps have no authority priority.
- Task-status conflict behavior follows [ADR-0017](../../../docs/03-architecture/decisions/ADR-0017-concurrent-status-file-conflict.md); text/document merge and file alternatives retain their own domain policies.
- Android, iOS and full Flutter Windows Release-1 clients must cover the same Project journey, including offline/restart, keyboard and accessibility conformance.

## Non-goals

- CRM, City Portal, business booking, payments, Gantt/timeline, time tracking, budgeting and advanced analytics in Release 1.
- An isolated Project messenger or knowledge database that duplicates the bundled Apps.
- Selecting a task-authority implementation from an Iroh or CRDT library without accepted protocol fixtures.

## Success signal

One user creates a private project, makes a task and linked note offline, restarts and synchronizes without losing either. They create a separate Shared Space for a team project, link its tasks to chat, forum and files, and resolve an accepted status conflict without exposing the Personal Space. The [conformance cases](project-conformance-cases.md) make these behaviors testable.

## Open Questions

- What Project Container hierarchy, move/copy behavior and relation ownership follow from the unresolved resource-domain choices (`OQ-0003`)?
- What verifiable authority frontier and acceptance proof govern serverless exclusive task-status operations (`OQ-0033/0034`)?
- What exact dependent-App manifest and default cross-App grants does Project activation use (`OQ-0070`)?
