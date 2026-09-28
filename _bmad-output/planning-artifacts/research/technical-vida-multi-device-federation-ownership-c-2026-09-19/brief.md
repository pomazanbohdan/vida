# Research brief — VIDA multi-device, federation, authority

## Decision boundary

Design separate contracts for (a) applying the same event on multiple devices, (b) deriving new shared resources versus local projections, (c) federated-node service roles, and (d) acceptance/receipts for a resource with one or multiple owners. Do not promote a research option into an accepted ADR before the user's decision.

## Questions and evidence plan

1. When 2–3 devices receive the same signed event, what can safely run on each replica, and when must a new domain operation have a single stable identity or authority? How should unsynced edits by the same person reconcile?
2. What do primary federated systems assign to account, handle, identity proof, relay, storage, app hosting and automation? Is federation itself an execution role?
3. How do authorization signatures, authority acceptance, storage consensus/quorum, and delivery receipts differ? What constitutes a confirmed booking after a lost response?

Primary sources only: official protocol/project documentation and original papers. No current VIDA artifact is external technical evidence; accepted VIDA docs are used only for compatibility/impact analysis.

## Existing VIDA decisions to reconcile

- `ADR-0012`: `sync.apply` updates local state/projections and must not auto-issue a new business command; user's newly described app-logic behavior may require narrowing or revising this rule.
- `ADR-0004`: a federated account is a revocable ServiceBinding, not the root Persona; user clarifies the node can attest a domain handle and separately host proxy, storage, marketplace or app services.
- `ADR-0006`: ciphertext durable delivery is not domain authority; a City Portal/business server with booking authority is a different role.
- `ADR-0003`: ordinary membership changes do not require multiple Owner votes by default; critical scopes may use M-of-N. The user asks whether ordinary resource acceptance should have broader 1+1/2+1 confirmation.

## Output

Decision-grade research report, compatibility/contradiction map, proposed authority profiles and concrete questions for user approval. No implementation or normative ADR in this run.
