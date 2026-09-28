---
review: two-compliant-units
artifact: ../ARCHITECTURE-SPINE.md
date: 2026-09-19
re_review: implementation-gates
verdict: conditional-pass-for-architecture-blocked-for-interoperability
---

# Adversarial compatibility re-review

## Verdict

The revised spine is safe to use as a bounded architecture direction, not yet as an interoperable implementation contract. Its new gates correctly make most unresolved choices future decisions rather than silent degrees of freedom: two prototypes may still diverge at those seams, but they cannot both legitimately claim VIDA interoperability or production readiness before the linked question is closed by a normative contract and shared fixtures.

One current architectural gap remains. `SyncLog` convergence, deterministic workload transitions, snapshot authority and safe pruning are deferred without a dedicated implementation gate or linked open question. Two independently built units can therefore both satisfy the written `SyncLog` contract yet derive different state or reject each other's snapshots. Close this gap before final handoff to independent implementers.

## Contained future-decision gates

| Previously divergent seam | Containment now present | Classification |
|---|---|---|
| Canonical operation envelope, signature domain, framing and feature negotiation | Spine gate + `OQ-0028`; normative codec/kernel or byte-exact golden vectors required | Contained future decision |
| Space authority ordering, causal cut and concurrent admin changes | Spine gate + membership contract + `OQ-0031`; shared convergence fixtures required | Contained future decision |
| Blob manifest, digest, encryption, ticket and deletion semantics | Spine gate + `OQ-0029`; external exchange blocked beyond internal prototype | Contained future decision |
| ACK aggregation and direct-versus-durable completion | Spine gate + delivery contract + `OQ-0032`; completion UX blocked | Contained future decision |
| Address Lookup provider, privacy, cache and fallback | Spine gate + `OQ-0030`; production discovery blocked | Contained future decision |
| Durable-delivery topology and trust model | Delivery contract + `OQ-0027`; production/independent adapters blocked | Contained future decision |
| Exact Iroh/dependency artifacts | Exact release pins/checksums delegated to lockfile and SPDX SBOM | Contained release decision |

These seams remain intentionally incompatible until decided. That is acceptable because the documents now prohibit a conformance or readiness claim during that interval.

## Current architectural blocker

### B1 — Deterministic state application and snapshot interoperability have no gate

**Evidence:** AD-4 and `REQ-SYNC-001` require idempotent apply and convergence. `SPEC-SYNC-LOG-001` nevertheless defers workload-specific CRDTs, snapshot format, and pruning/checkpoint policy. The spine's implementation gates cover protocol bytes, membership, blobs, discovery and delivery UX, but not independent `SyncLog` or workload state-machine implementations.

**Divergence:** two units can decode and authenticate the same canonical envelope while applying concurrent note/task/message operations differently. They can also choose incompatible snapshot roots/frontiers and prune dependencies the peer still needs. Both would meet the current semantic interface and local acceptance criteria.

**Required closure:** add a dedicated gate and open question for versioned operation-family transition rules, causal comparison, conflicts/tombstones, canonical state/root hashing, snapshot producer authority, frontier proof and safe pruning. Require replay fixtures that produce the same state hash across implementations. This may be a new question such as `OQ-0033`; it should not be hidden inside wire-envelope `OQ-0028` unless that question is explicitly expanded to include state semantics.

## Non-blocking follow-ups

- Stable error codes, retryability and wire mappings can be specified inside the protocol profile before `OQ-0028` closes.
- Named minimum browser/mobile/desktop capability profiles can remain a release gate as long as no platform claims feature parity without its conformance result.
- Concrete storage engines and product-specific CRDT choices may remain deferred; only their observable transition and convergence behavior must be normative for interoperability.

## Closure assessment

- **Architecture direction:** conditional pass after B1 receives an explicit gate/open question.
- **Single reference prototype:** permitted behind the documented provisional boundaries.
- **Independent interoperable implementations:** blocked until the relevant gates close and cross-implementation fixtures pass.
- **Production readiness:** blocked independently per seam; closing one gate does not waive the others.

## Next single decision area

Start with `OQ-0028`: define the canonical signed operation envelope, signature domain, framing, feature negotiation and golden vectors. It is the common substrate for durable delivery, membership operations, sync repair and subsequent state-transition fixtures. Record deterministic state application/snapshot semantics as a separate linked gate rather than treating envelope compatibility as sufficient for convergence.
