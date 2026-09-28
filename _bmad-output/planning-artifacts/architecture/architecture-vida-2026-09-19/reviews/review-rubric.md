# Architecture Spine review — good-spine rubric + accepted research reconciliation

**Initial verdict:** **CHANGES REQUIRED.** See the recheck at the end for the current disposition.

## Scope and evidence

- Reviewed: `ARCHITECTURE-SPINE.md` (2026-09-19).
- Reconciled input: `technical-spotify-react-native-to-native-shared-lo-2026-09-19/research.md`.
- Checked companion constraints: `platform-nfr.md`, `transport-sync-requirements.md`, `transport-sync-architecture.md`, and the run `.memlog.md`.
- Deterministic BMad lint: **PASS**, 0 findings.
- Repository state: planning/documentation repository; no product implementation exists to contradict. Brownfield ratification therefore means consistency with accepted ADR/REQ/NFR/architecture documents, which is mostly satisfied.

## Critical finding

### R-01 — The declared dependency direction compromises the headless-core invariant

**Evidence**

- Research says `vida-core` must run without a UI, simulator, network connection, or specific database (`research.md:91-105`).
- Research assigns external-capability ports to `vida-runtime` (`research.md:107-118`).
- The spine diagram declares `Core --> Runtime` (`ARCHITECTURE-SPINE.md:51-55`).
- The consistency table says “core may call runtime ports,” and the structural seed places those ports inside `vida-runtime/ports` (`ARCHITECTURE-SPINE.md:146, 169-177`).

**Why this fails the rubric**

Two teams can now implement incompatible interpretations: one makes the product kernel independently executable; another links it to runtime-owned types and lifecycle. The latter makes headless execution contingent on infrastructure and reverses the dependency-inversion boundary implied by the accepted paradigm. AD-11 therefore does not fully prevent the divergence it names.

**Required fix**

Make the invariant explicit: `vida-core` has no compile-time dependency on `vida-runtime`, network, OS lifecycle, or a concrete database. Core-owned commands/events and, where effects are unavoidable, core- or contract-owned interfaces are injected by the composition layer. `vida-runtime` depends on and drives `vida-core`, not vice versa. Update the diagram, dependency convention, and seed consistently. If a bidirectional orchestration model is intended, define the neutral contract crate and prohibit runtime implementation types from crossing it.

## High findings

### R-02 — “Headless” lacks the accepted executable CLI/test contract

**Evidence**

- The accepted research makes the headless API/CLI for humans, tests, and agents a governing rule (`research.md:34-43`) and includes test clock, deterministic randomness, and replay harness in `vida-core` (`research.md:91-103`).
- AD-11 lists domain functions but no headless entry point, deterministic effect controls, or CLI/test harness (`ARCHITECTURE-SPINE.md:130-134`).

**Impact**

A compliant implementation may technically be UI-free yet still require a simulator, live network, wall clock, random source, or platform database. That loses the key Shopify-derived development property and makes parity failures expensive to isolate.

**Required fix**

Extend AD-11 or add a compact AD: all normative commands and observable transitions must be exercisable through a deterministic headless API and reference CLI/test harness without UI, network, simulator, or concrete storage. Time/randomness/effects must be injectable and replayable.

### R-03 — The Conformance Plane is wire-heavy and does not bind product-behavior parity

**Evidence**

- Research makes Messenger/Notes/Projects parity scenarios and cross-platform visual/accessibility expectations part of the normative Conformance Plane (`research.md:78-89`), derived from Shopify’s shared behavior specifications and parity gates (`research.md:48-58, 146-157`).
- The spine names schemas, wire vectors, replay fixtures, and negative security cases (`ARCHITECTURE-SPINE.md:39-50, 76-80`). AD-11 only requires alternative implementations to pass language-neutral specifications, golden vectors, and replay fixtures (`ARCHITECTURE-SPINE.md:130-134`).
- Accessibility is assigned to each shell, but no common behavior/accessibility acceptance contract or release gate is stated (`ARCHITECTURE-SPINE.md:132-147`).

**Impact**

Swift, Kotlin, desktop, and browser teams can all pass protocol conformance while shipping different command availability, state transitions visible to users, accessibility semantics, or failure behavior.

**Required fix**

Bind the Conformance Plane to versioned product-behavior scenarios for every shared capability, plus platform-appropriate accessibility acceptance criteria. Require platform releases to publish those results. Avoid pixel identity; require semantic/behavioral parity where specified.

### R-04 — FFI and storage seams are deferred without enough guardrail to prevent incompatible implementations

**Evidence**

- The research permits provider replacement only behind stable transaction, migration, snapshot, encryption, recovery, and conformance contracts (`research.md:134-144`), and leaves the canonical binding strategy open (`research.md:169-176`).
- The spine defers the FFI strategy and concrete persistence engine (`ARCHITECTURE-SPINE.md:200-207`) but its implementation gates do not block independent shell bindings or storage-provider interfaces (`ARCHITECTURE-SPINE.md:149-159`).
- AD-8 says to test an “FFI artifact” independently, but does not define ownership/versioning of the ABI/API, error model, memory, cancellation, threading, or upgrade window (`ARCHITECTURE-SPINE.md:112-116`).

**Impact**

The iOS, Android, browser, and storage workstreams can choose incompatible FFI conventions or provider contracts before the prototype decision lands. This is exactly the level-below divergence the spine must prevent.

**Required fix**

Keep the tools open, but add explicit gates: no production platform binding until one versioned FFI contract and compatibility window are accepted; no replaceable storage provider until the stable transaction/migration/snapshot/encryption/recovery contract and conformance suite are accepted. Link each to a named OQ/ADR owner and revisit condition.

### R-05 — The initiative-level operational/environmental envelope is not closed in the spine

**Evidence**

- The spine defers the service deployment provider, durable-delivery topology, and quantitative SLOs, but does not state the invariant operational ownership, supported environments, config/secret boundary, backup/restore responsibility, observability contract, or production-readiness gate (`ARCHITECTURE-SPINE.md:149-159, 200-207`).
- The companion architecture already contains a useful ownership invariant: client/runtime teams own endpoint lifecycle/local recovery; delivery operators own ciphertext durability/quota/abuse/deletion evidence; Space-authority operators own control ordering/authorization receipts, and every deployment must name owners (`transport-sync-architecture.md:68-76`).
- The companion NFR already requires typed telemetry, queue/reconciliation/blob/path metrics, rolling-upgrade/rollback, and backup-restore tests (`platform-nfr.md:28-44`).

**Why this is a spine finding**

At initiative altitude this is a whole owned dimension. Merely listing companion documents does not make their load-bearing operational split visible as an inherited invariant. Independently built hosted, federated, and self-hosted units can otherwise assign responsibility differently.

**Required fix**

Either inherit the companion operational-ownership rule explicitly or add one compact AD. Keep providers and numeric SLOs deferred, but require named ownership, secret-safe observability, upgrade/rollback/restore evidence, and environment-specific readiness gates before production enablement.

## Medium findings

### R-06 — Platform primitive ownership is ambiguous at the shell/runtime boundary

Research assigns OS secure storage plus filesystem/database primitives to shells exposed through runtime ports (`research.md:120-132`). The spine assigns secure-store handles to shells, storage coordination/providers to runtime, and allows core to call runtime ports (`ARCHITECTURE-SPINE.md:130-146, 169-177`). It never says who owns the platform adapter, transaction boundary, key-handle lifetime, or recovery callback. Clarify: shell/platform adapter owns OS handles and primitives; runtime owns portable orchestration/contracts; core sees neither implementation type.

### R-07 — Final status is premature while load-bearing review findings remain

Frontmatter is already `status: final` (`ARCHITECTURE-SPINE.md:8`) although the dependency-direction conflict and missing gates remain. Return to draft during remediation; set final only after fixes, reviewer rerun, and a `spine finalized` memlog event.

## Checklist disposition

| Good-spine criterion | Result | Notes |
|---|---|---|
| Fixes real divergence points; misses none | **Fail** | Headless dependency, behavior parity, FFI/storage gates, operations need closure. |
| Every AD is enforceable and prevents its divergence | **Fail** | AD-11 is weakened by `Core --> Runtime`; AD-8 lacks canonical FFI gate. |
| Deferred cannot permit incompatible work | **Fail** | FFI and storage contract work may diverge before selection. |
| Named technology verified-current | **Pass** | Iroh core 1.2.0 is pinned and memlog records verification on 2026-09-18. |
| Ratifies brownfield reality | **Pass with note** | No product code exists; accepted docs align. Operational companion invariant should be inherited explicitly. |
| Covers input capabilities | **Pass with gaps** | Core/runtime/shell/Iroh pattern landed; CLI/testability and parity contract did not. |
| Preserves inherited parent decisions | **Pass** | ADR-0001–0004 are represented without visible weakening. |
| Every initiative dimension decided/deferred/open | **Fail** | Operational/environmental envelope is only partially present in companions. |
| Minimal seed; decisions over rationale | **Pass** | Structure and stack seed are appropriately compact for platform altitude. |
| Mechanics | **Pass** | BMad lint reports zero findings; AD IDs and required fields are valid. |

## Accepted research reconciliation

### Landed faithfully

- The Shopify/Spotify distinction is not misrepresented; the spine uses the architectural pattern rather than claiming a Spotify React Native migration.
- The named paradigm is preserved verbatim.
- `vida-core`, `vida-runtime`, native/platform shells, and Conformance Plane are explicit.
- Iroh core is strategic/mandatory, while version integration and optional pre-1.0 higher-level libraries are gated.
- UI, lifecycle, permissions, accessibility, notifications, secure-store handles, diagnostics, and packaging remain platform-owned.
- Provider replacement is constrained conceptually to delivery/storage/discovery seams and compatibility tests.
- FFI/browser/storage choices remain open rather than invented.

### Missing or distorted

- **Distorted:** `Core --> Runtime` and core-to-runtime-port dependency weaken the accepted independently executable headless core.
- **Missing:** reference headless API/CLI and deterministic agent/test harness.
- **Missing/underspecified:** product-behavior and accessibility parity scenarios in the Conformance Plane.
- **Underspecified:** stable storage-provider contract and production FFI gate.
- **Partially landed:** runtime telemetry, typed failure mapping, and platform-neutral background-job contract appear only indirectly in conventions/NFR, not as a clear runtime boundary.

## Recommended remediation order

1. Correct the core/runtime dependency direction and port ownership.
2. Add the headless CLI/deterministic-harness invariant.
3. Expand Conformance Plane from wire parity to specified product-behavior/accessibility parity.
4. Add FFI and storage implementation gates with named open questions.
5. Inherit the operational ownership/readiness invariant and rerun all reviewer lenses.

## Recheck — updated spine

**Current verdict:** **PASS.** BMad lint passes with 0 findings. All prior semantic blockers are resolved; the accepted research is faithfully represented. Only the normal finalization close step remains.

| Finding | Recheck |
|---|---|
| R-01 dependency direction | **Resolved.** Diagram and AD-11 establish `runtime → core`; core has no runtime/network/OS/database dependency; production shells use one SDK facade. Normative DTOs and port traits live only in `vida-contracts`; `vida-runtime/provider-host` only orchestrates implementations, and providers implement those SPIs. |
| R-02 headless CLI/test contract | **Resolved.** AD-11 requires a deterministic headless API and reference CLI/test harness with injected time/randomness/effects/replay; diagram and seed include it. |
| R-03 behavior/accessibility parity | **Resolved.** AD-2 adds versioned product-behavior scenarios, platform-appropriate accessibility criteria, immutable suite digests, and machine-readable release evidence. |
| R-04 FFI/storage guardrails | **Resolved.** AD-8/12/14 define compatibility and provider contracts; OQ-0035–0037 explicitly block production binding, storage substitution, and mixed-version rollout. Registry entries exist. |
| R-05 operational/environmental envelope | **Resolved.** AD-15 fixes ownership and production-readiness evidence across development/staging/production while preserving provider/SLO deferral. |
| R-06 platform primitive ownership | **Resolved except for the R-01 naming contradiction.** AD-11/13/14 now separate shell handles, runtime orchestration, and provider mechanisms. |
| R-07 final status | **Procedural close only.** `decision_status: accepted` plus `implementation_status: prototype-gated` removes the semantic ambiguity. Append the required `spine finalized` memlog event only after the remaining blocker and all reviewer fixes close. |

### Final disposition

**PASS — no remaining rubric or accepted-research reconciliation blocker.** The port-contract owner is singular and consistent across the diagram, AD-12/AD-14, dependency convention, and structural seed.
