---
review: adversarial-independent-build
artifact: ../ARCHITECTURE-SPINE.md
date: 2026-09-19
verdict: pass-as-direction-fail-as-independent-build-contract
recheck_date: 2026-09-19
recheck_verdict: one-high-remains
final_recheck_verdict: pass
findings:
  critical: 3
  high: 4
---

# Adversarial review — independently built but formally compliant units

## Verdict

The spine is coherent as an architecture direction, but it is not yet a sufficient build contract for independently implemented units. Seven material degrees of freedom remain outside the deliberately gated OQs. Two teams can obey every adopted AD, pass locally selected tests, and still produce incompatible data shapes, dependency graphs, crash behavior, provider semantics, and release activation rules.

This review does **not** count the operation envelope, membership ordering, anonymous unlinkability, blob contract, discovery policy, delivery aggregation, authority-acceptance model, or `SyncLog` transition/snapshot rules as current holes: lines 149–158 explicitly block the relevant independent or production implementations until `OQ-0023` and `OQ-0028`–`OQ-0034` close. The findings below remain possible even after those gates are respected.

## Adversarial constructions

### C1 — The permitted dependency graph has two orchestration roots

**Evidence:** the diagram permits `Shells → Core`, `Shells → Runtime`, and `Core → Runtime`. The convention then says shells may call both packages and core may call runtime ports. At the same time, runtime owns sync, delivery, persistence coordination, and background jobs that necessarily invoke core validation and transitions.

**Two compliant units:**

- Unit A lets its iOS shell call `vida-core` for commands and projections, then separately asks `vida-runtime` to persist and deliver the returned operation.
- Unit B exposes a runtime application service; its Android shell calls runtime, which invokes core and commits the result.

Both follow AD-11 and the stated dependency convention. They nevertheless have different transaction owners, retry boundaries, error ordering, lifecycle behavior, and opportunities to bypass delivery or authorization. A future runtime→core call also creates an architectural cycle against the currently permitted core→runtime dependency.

**Required tightening:** replace the permissive arrows with one normative production call path and a cycle-free package graph:

```text
shell UI → vida-sdk/binding façade → vida-runtime application service → vida-core
vida-core → vida-contracts only
vida-runtime → vida-core + vida-contracts
provider/platform adapters → runtime SPI + vida-contracts
```

Define `vida-contracts` (name may change) as a semantics-free contract artifact containing boundary DTOs, IDs, errors, and port traits. `vida-core` MUST NOT import `vida-runtime`. Production shells MUST NOT call core and runtime independently; all stateful commands and authoritative reads cross one façade. Pure preview/formatting helpers, test harnesses, and developer tools require explicitly named read-only APIs. Platform services are injected into the host through adapters; runtime never calls UI modules.

### C2 — Cross-boundary and persisted data shape has no normative owner

**Evidence:** AD-2 governs protocol schemas, AD-11 names canonical serialization inside core, and AD-8 says an FFI artifact is tested independently. None states which artifact normatively defines FFI DTOs, persisted logical records, snapshots, projection rows, drafts, IDs in textual form, timestamps, errors, or feature-bearing enums. “Stable opaque IDs” does not define byte width, textual normalization, comparison, or invalid encodings. Golden examples do not by themselves close the full value domain.

**Two compliant units:**

- Unit A exposes hand-written JSON FFI DTOs, millisecond timestamps, absent optional values, lowercase textual IDs, and drops unknown enum cases. Its storage provider serializes private Rust structs.
- Unit B exposes generated binary DTOs, nanosecond timestamps, explicit nulls, case-preserving IDs, and retains unknown fields. Its provider persists normalized logical records.

Both can carry the same canonical signed operation bytes and satisfy every current AD. They diverge during binding upgrades, draft restore, import/export, unknown-feature forwarding, downgrade, and storage migration.

**Required tightening:** make the Conformance Plane the normative registry for every **observable boundary**: wire, signature input, FFI/API, persisted logical state, snapshot, projection checkpoint, and export. Every contract MUST declare a stable `ContractId`, version, field types, required/optional/null rules, defaults, bounds, Unicode/number/time/ID normalization, unknown-field/unknown-enum behavior, canonical bytes where equality/hash/signature matters, and migration behavior. Private in-memory layouts and physical database layout remain implementation details. Bindings SHOULD be generated from the registry; hand-written mappings MUST pass bidirectional and unknown-value preservation fixtures. No private Rust struct may become a persisted or FFI contract implicitly.

### C3 — The mutation convention does not define a crash-consistent state machine

**Evidence:** AD-4 requires only the local log and outbox to commit atomically. The convention lists `validate → sign → atomic log+outbox → deliver → idempotent apply → project`, but does not identify transaction ownership, the exact local/remote pipelines, when authorization is evaluated, which frontier/idempotency records join the commit, whether projections are synchronous, or which watermark a read observes.

**Two compliant units:**

- Unit A commits operation+outbox, returns `delivery.accepted`, and later applies canonical state and projections. After a crash it can expose an accepted operation absent from reads until replay completes.
- Unit B atomically commits operation, dedupe marker, causal frontier, materialized state, projection, and outbox, and does not return until all are visible.

Both satisfy atomic log+outbox and eventual idempotent apply. Their user-visible acknowledgement, read-your-writes, recovery, retry, and notification behavior differs.

**Required tightening:** define separate normative local-command and received-operation state machines. At minimum:

1. A single runtime application service owns the mutation transaction.
2. Core validates the command against a named base frontier and returns a deterministic canonical operation/effects; the signer signs exactly those canonical bytes.
3. One durability transaction records the operation, outbox intent, idempotency/dedupe state, and authoritative causal/frontier advancement. If canonical materialized state is stored, its advancement belongs to the same transaction.
4. A projection may lag only if it is explicitly non-authoritative, rebuildable, and exposes a source watermark; read-your-writes and completion behavior MUST be specified against that watermark.
5. The remote pipeline orders schema/signature verification, dedupe, causal availability, authorization at the operation’s causal cut, append/apply/frontier advancement, quarantine/rejection, and projection.
6. Shells and providers MUST NOT write log, frontier, authoritative state, projection checkpoints, or outbox directly.

Add crash-point replay fixtures at every boundary and require identical state/frontier/projection hashes after recovery.

### H1 — “Conformance Plane governs” is not an enforceable certification rule

**Evidence:** the spine lists specifications, vectors, replay fixtures, negative cases, and platform gates, but does not define which suites are mandatory for a given artifact, who may generate expected values, how fixtures are versioned, or what release evidence proves a target passed the same corpus.

**Two compliant units:**

- Unit A generates golden values from its reference serializer and runs the protocol subset relevant to its platform.
- Unit B generates its own expected values and treats unsupported negative or migration cases as non-applicable.

Each can truthfully say it used versioned specs and fixtures. Local green tests do not establish parity.

**Required tightening:** add a versioned conformance manifest that maps each supported release target and compatibility tuple to mandatory suite IDs and immutable fixture digests. Expected values MUST be spec-first or independently reviewed; an implementation cannot be the sole producer and certifier of its oracle. Mandatory suites include positive/negative codec, signature-domain, limits, unknown features, transitions, convergence, migration, crash recovery, FFI mapping, lifecycle, and provider failure semantics where applicable. CI emits machine-readable evidence keyed by source commit, artifact digest, platform, and suite-manifest digest. A release gate accepts only the common manifest; “not applicable” requires a documented capability-profile exclusion.

### H2 — Compatibility axes are listed, but their composition and rollout are undefined

**Evidence:** AD-8 correctly separates Iroh, adapter, ALPN, feature set, persisted state, and FFI compatibility. It does not define a release compatibility tuple, supported mixed-version window, reader/writer rules, feature activation, migration rollback barrier, or fleet rollout order. `OQ-0028` can close protocol negotiation while this cross-axis release problem remains.

**Two compliant units:**

- Unit A begins writing a newly supported optional field as soon as its local binary upgrades and assumes older readers ignore it.
- Unit B preserves but rejects the unknown semantic feature until every authority and client advertises support. One storage migration remains readable by the old binary; the other is immediately irreversible.

Both test every axis independently as AD-8 requires. A mixed fleet still loses data or becomes unrecoverable.

**Required tightening:** every release MUST publish a machine-readable compatibility manifest containing at least core/API version, FFI ABI/API version, operation/envelope feature set, ALPN major and negotiated features, persisted logical schema, snapshot schema, adapter versions, and platform capability profile. Before production, choose and record a supported mixed-version window. Writers MUST NOT activate a required feature until all required readers/authorities satisfy the activation rule; optional features need normative preserve/ignore behavior. Persisted changes use expand–migrate–contract or declare an irreversible rollback barrier that the rollout controller enforces. Conformance MUST test every supported tuple and the documented upgrade, rollback, and partial-rollout paths—not only each axis in isolation.

### H3 — External ports can absorb the semantics they are meant to isolate

**Evidence:** the structural seed names broad delivery, storage, discovery, and platform ports, but no rule distinguishes primitive provider capability from normative VIDA orchestration. AD-5 prevents mailbox authority; it does not prevent storage or delivery implementations from owning dedupe, ordering, retry, transaction, or acknowledgement aggregation.

**Two compliant units:**

- Unit A defines a domain-aware storage port such as `save_message`, with provider-specific upsert, ordering, and transaction rules.
- Unit B defines primitive atomic batch/CAS/scan capabilities and keeps domain ordering and idempotency in runtime.

Both place storage behind a port. Replacing Unit A’s provider changes product semantics; Unit B’s does not.

**Required tightening:** publish a port charter and versioned SPI for each external seam. A port contract states capabilities, consistency/atomicity guarantees, idempotency key behavior, retries, cancellation, timeout, error taxonomy, privacy/metadata exposure, and failure injection. Providers supply mechanisms only. Authority, causal ordering, conflict resolution, canonical serialization, idempotent apply, retry policy, and domain acknowledgement semantics remain in core/runtime and MUST NOT be implemented by providers. Domain-shaped repositories or callbacks require an architecture decision proving that no normative semantics crossed the boundary. Provider conformance is mandatory before substitution.

### H4 — “Unavoidable alternative implementation” is an unbounded escape hatch

**Evidence:** AD-11 allows an “unavoidable alternative implementation” if it conforms to specifications, golden vectors, and replay fixtures. It defines neither who decides necessity nor how finite example fixtures establish semantic equivalence outside their covered inputs.

**Two compliant units:**

- Native clients reuse the Rust core through bindings.
- A browser team declares Rust/WASM impractical, reimplements reducers and authorization in TypeScript, and passes all finite fixtures while diverging on an ungenerated concurrent edge case.

Both comply; only one shares the implementation that AD-11 calls single.

**Required tightening:** Rust core reuse through FFI/WASM is the default. Any alternative implementation requires a time-bounded waiver ADR naming the exact semantic surface, necessity, owner, expiry, threat model, compatibility tuple, and removal/renewal criteria. In addition to the common conformance manifest, the alternative MUST pass differential property/fuzz testing against the canonical Rust implementation, mutation/negative corpora, and supported-version replay. Its release needs an independent approval and a remotely operable disable/fallback path where the platform permits. A waiver cannot weaken signature, authorization, membership, or persistence semantics.

## Exact spine edits required before independent build handoff

1. Replace the layer-dependency convention and Mermaid arrows with the single façade and cycle-free dependency direction in C1.
2. Add a boundary-contract/data-ownership AD implementing C2; enumerate normative versus private shapes.
3. Add a mutation-transaction/recovery AD implementing C3 and bind it to `SyncLog`, projections, outbox, and providers.
4. Turn Conformance Plane from a noun into a release rule: manifest, immutable corpus, oracle independence, evidence, and target matrix.
5. Extend AD-8 with the compatibility tuple, activation policy, mixed-version window, and migration rollback barrier.
6. Add the provider port charter and explicitly prohibit semantic leakage.
7. Replace “unavoidable” with the waiver and differential-equivalence policy.

## Closure test

The review passes when two teams, given only the spine and referenced normative artifacts, are forced to produce:

- the same observable boundary values and canonical bytes;
- the same single production call/dependency path;
- the same local and remote crash outcomes at every transaction cut;
- providers whose substitution cannot change authority or domain semantics;
- independently reproducible conformance evidence against one immutable corpus; and
- a release pair that can prove compatibility for every supported mixed-version tuple.

Until then, the spine should be labeled **final architecture direction / independent-build contract blocked**, not an unconditional build substrate.

## Recheck against updated spine — 2026-09-19

### Verdict: one HIGH hole remains

The updated AD-2, AD-4, and AD-8 plus AD-11–AD-14 and gates `OQ-0035`–`OQ-0037` close six of the seven original attack classes. The single façade and cycle-free dependency direction close C1; AD-12 and the binding/storage gates close C2; the shared conformance manifest closes H1 at this altitude; the compatibility tuple and rollout gate close H2; AD-14 closes H3; and the waiver plus differential-test rule closes H4.

### H5 — The source of truth still splits between log and “authoritative materialized state”

**Conflicting rules:** AD-5 says only the validated domain/control log is authoritative. AD-13 binds “authoritative materialized state,” but its required atomic transaction includes only operation, outbox, dedupe, and frontier advancement. It never says whether materialized domain state is derived or co-authoritative, whether it joins that transaction, or what a read may return when its watermark lags.

**Two still-compliant units:**

- Unit A treats the validated log as the sole authority and rebuilds materialized domain state asynchronously after `delivery.accepted`.
- Unit B treats materialized domain state as co-authoritative and updates it inside, or immediately after, the log transaction.

Both satisfy AD-4 and the literal AD-13 transaction list. After a crash, Unit A can expose a committed operation through log-based reads while its domain view is stale; Unit B can expose only materialized state or can strand the log and materialization at different frontiers. Their read-your-writes, recovery, notification, and snapshot inputs still diverge.

**Exact tightening:** make one choice explicit. The direction already established by AD-5 suggests:

> The validated operation/control log is the sole authority. Every materialized domain state and projection is derived, rebuildable, and tagged with the exact source frontier. A release contract states whether an accepted read waits for that frontier or may return a lag watermark. If a materialization participates in acceptance/read-your-writes, it MUST advance atomically with operation, outbox, dedupe, and frontier; otherwise recovery MUST replay it before the target claims readiness.

Remove “authoritative” from “materialized state” in AD-13’s binds, or explicitly reverse AD-5 and include materialized state in the mandatory transaction. Add crash fixtures for the log-committed/materialization-not-advanced cut and assert the same read result and readiness state across providers.

### Recheck disposition

- **Closed:** C1, C2, H1, H2, H3, H4.
- **Partially closed:** C3; transaction ownership is fixed, but authoritative-state/read consistency remains ambiguous as H5.
- **Final result:** no CRITICAL findings; one HIGH finding blocks an unconditional independent-build PASS.

## Final blocker check — 2026-09-19

### Verdict: PASS

AD-13 now makes the validated signed operation log the sole durable domain authority, classifies every materialized state/projection as derived and rebuildable, requires exact source-frontier tags, defines minimum-frontier/read-your-writes versus explicit lag behavior, and blocks runtime readiness until replay reaches the required frontier. Its crash fixtures compare both authority (`log/frontier`) and derived-state hashes across providers.

This closes H5: the former Unit A and Unit B can no longer choose different authoritative sources or silently expose different post-crash readiness/read semantics while claiming conformance. No CRITICAL or HIGH independently-built-unit blocker remains in this review.
