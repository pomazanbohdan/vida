# Rubric-walker re-review — VIDA transport and replicated-state spine

## Gate verdict

**Two semantic decisions still block finalization; the prior interoperability, membership-ordering, blob, discovery, delivery-aggregation, and operational findings are now safely gated.** The updated documents are materially stronger and suitable for bounded prototypes, but not yet a final architecture spine while ordinary domain authority remains ambiguous and the paradigm remains an explicit assumption.

## Prior critical/high findings

| Prior finding | Status | Re-review |
|---|---|---|
| C1 — local `accepted` vs authority acceptance | **Partially fixed; still blocking** | `delivery.accepted` is now correctly namespaced and explicitly differs from `authority.accepted`. However, the state path still goes from origin commit directly to delivery/apply, while the spine calls the “authority-accepted domain/control log” authoritative. It never decides which ordinary domain mutations require authority acceptance, which scopes are peer-authoritative, or where authority receipt/sequence enters validation and projection. `SyncLog.Accept` remains ambiguous on this boundary. |
| H1 — deferred interoperability seams | **Fixed by safe gates** | AD-2 adds normative codec/golden-vector equivalence. `OQ-0028`–`OQ-0030` and implementation gates block independent/production implementations until canonical artifacts and conformance fixtures exist. |
| H2 — membership conflict ordering | **Safely gated** | `OQ-0031`, the spine gate, and `SpaceMembership` now block independent membership implementations until authority serialization, causal cut, precedence, quorum, and convergence fixtures are defined. |
| H3 — anonymous endpoint isolation | **Partially fixed; gate still missing** | AD-3 and `REQ-TR-006` now require isolated ephemeral endpoints, removing the unsafe `MAY`. Yet the canonical unlinkability scope remains open in `OQ-0023`; the spine does not block anonymous production/interoperability until that default and rotation/migration behavior are chosen. |
| H4 — operational/environmental ownership | **Substantially fixed and safely deferred** | Runtime, delivery, and Space-authority ownership are now named; production enablement requires owners; topology/trust, blob security, discovery, restore/conformance, and SLO claims are gated. Provider/persistence choices may remain outside this contour. |

## Remaining blockers

### B1 — Decide authority semantics for ordinary domain operations

- **Evidence:** AD-5 says only the “authority-accepted domain/control log” is authoritative, but the state path and contracts contain no authority-acceptance step for ordinary messages, notes, tasks, or document edits. `OQ-0031` gates membership/control ordering only.
- **Why blocking:** two implementations can both follow the documents while one applies a peer-signed content operation after local validation and the other requires a Space authority receipt/sequence.
- **Required decision:** specify per Space/operation class whether durable domain operations are (a) peer-authoritative after causal capability validation, (b) authority-sequenced/accepted, or (c) policy-selectable under a single explicit contract. Define when an operation may enter the authoritative log and become projectable. If not decided now, create and bind a blocking open question before finalizing.

### B2 — Ratify or replace the architecture paradigm

- **Evidence:** frontmatter and Design Paradigm still contain `[ASSUMPTION — review]`.
- **Why blocking:** a final spine cannot retain an unapproved structural premise that controls dependency direction and adapter ownership.
- **Required decision:** approve “hexagonal local-first architecture” as normative, or replace it.

## Non-blocking if one final gate is added

### G1 — Anonymous unlinkability granularity

`OQ-0023` still asks whether anonymous scope defaults to per-contact, per-room, or policy-selected per-Space. Add an explicit gate: anonymous production/interoperability claims are blocked until this question, endpoint rotation, persistence, and migration rules have conformance fixtures. With that gate, the choice may remain open without blocking architecture finalization.

## Earlier medium/low status

- Exact Iroh baseline is now `1.2.0` with lockfile/SBOM pinning: **fixed**.
- SPDX inventory/checksum/integration-mode rule: **fixed**.
- Transaction ownership remains somewhat abstract, but `delivery.accepted` and crash/recovery invariants now constrain behavior sufficiently for the current altitude: **acceptable; refine with persistence selection**.
- Source traceability still omits ADR-0002/0005/0006 in spine frontmatter: **autofix; not a semantic blocker**.
- Requirement-to-AD traceability remains implicit: **recommended autofix; not a blocker**.

## Finalization condition

Finalize after B1 and B2 are resolved, G1 is either decided or explicitly gated, and the two traceability autofixes are applied. Existing implementation gates for `OQ-0027`–`OQ-0032` may remain open in a final architecture because they prevent divergence rather than permit it.
