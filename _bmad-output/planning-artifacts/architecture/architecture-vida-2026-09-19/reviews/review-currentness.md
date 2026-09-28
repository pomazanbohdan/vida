# Currentness and Evidence Review

- Review date: 2026-09-19
- Artifact: `ARCHITECTURE-SPINE.md`
- Lens: configured reality/currentness — every committed technology, version, and pattern must be supported by current research or repository evidence
- Verdict: **PASS after re-review**
- Open findings: **0 P0 / 0 P1 / 0 P2**
- Prior findings: **2 P1 / 3 P2, resolved**

## Re-review — 2026-09-19

| Prior finding | Result | Updated evidence |
|---|---|---|
| P1-1 final status exceeded prototype evidence | **Resolved** | Frontmatter now separates `decision_status: accepted` from `implementation_status: prototype-gated`; line 187 blocks architecture lock and production claims until all research-mandated prototypes publish conformance evidence. |
| P1-2 absent lockfile/SBOM stated as present | **Resolved** | Stack now calls 1.2.0 a decision baseline and uses future normative wording: release implementations `MUST` create exact lockfile/SBOM pins. |
| P2-1 endpoint revocation conflation | **Resolved** | AD-3 now separates revocable `EndpointBinding`/`DeviceGrant`, endpoint-key rotation, transport-key lifecycle, and authorization revocation. |
| P2-2 browser scope stronger than evidence | **Resolved** | AD-1 now binds a logical transport contract, limits direct in-process hosting to validated native/service profiles, and leaves browser WASM versus companion/gateway explicitly gated. |
| P2-3 incomplete source manifest | **Resolved** | Sources now include identity research and ADR-0002, ADR-0005, and ADR-0006. |

### Re-review verdict

**PASS.** The spine now states the supported architectural decision without presenting unexecuted prototype or release artifacts as current reality. No currentness/evidence blocker remains. `status: final` is acceptably qualified by the explicit decision/implementation statuses and the architecture-lock gate.

## Executive verdict

The architectural direction is evidence-backed: Iroh 1.2.0, VIDA-owned ALPN protocols, separate transient/durable/blob planes, a shared headless Rust core, native/platform shells, and controlled provider seams all appear in the completed research. The current Iroh research was accessed one day before this spine and explicitly identifies 1.2.0 as the stable core release.

The artifact nevertheless presents an evidence-backed **direction** as a fully validated **final substrate**. The research requires platform, FFI, storage, endpoint, durable-delivery, membership, sync, blob, lookup, and threat-model prototypes before architecture lock. No implementation, lockfile, SBOM, prototype report, compatibility matrix, or measured platform result exists in this repository. Two present-tense claims therefore exceed the configured reality, and three traceability/semantic details should be tightened.

## Findings

### P1-1 — `status: final` exceeds the research confidence and missing prototype evidence

**Spine:** line 8 declares `status: final`; AD-1 through AD-11 are all `[ADOPTED]`.

**Evidence:**

- The shared-core research rates the Rust-kernel application only **medium-high**, pending successful FFI, browser/WASM, and storage prototypes (`technical-spotify-.../research.md`, lines 26–28 and 44).
- The Iroh research calls nine prototypes mandatory before architecture lock: core node, compatibility, offline delivery, Space security, local-first sync, blob security, platform, Address Lookup, and threat-model review (`technical-iroh-.../research.md`, lines 241–253).
- The repository currently contains planning/research documents but no product implementation or prototype evidence.

**Risk:** downstream teams can read “final” as proof that the selected boundary and platform coverage are implementation-validated, although the source explicitly says they are not. This is especially material for browser/WASM, native FFI, mobile lifecycle, and the single shared Rust implementation.

**Required correction:** distinguish decision acceptance from implementation validation. For example, retain `decision_status: accepted` but set `implementation_status: unvalidated` (or change the overall status to `provisional`) until named prototype evidence exists. Add a top-level architecture-lock gate that references the research exit criteria, not only the narrower OQ gates.

### P1-2 — The stack table claims a lockfile and release SBOM that do not exist

**Spine:** line 164 states: “exact version/checksum pinned in lockfile and release SBOM.”

**Evidence:** a repository-wide file inventory finds no `Cargo.lock`, SPDX inventory, or SBOM artifact. The research supports **Iroh 1.2.0 as the baseline** and recommends exact pins; it does not establish that the release artifacts already exist (`technical-iroh-.../research.md`, lines 279–285, 292–294, 300–307).

**Risk:** this is a false present-state assertion and can be cited as reproducibility evidence during review even though no reproducible dependency set exists.

**Required correction:** use future normative wording: “release implementations MUST pin the exact version/checksum in the lockfile and release SBOM.” Add the actual artifact paths and promotion evidence only after those files exist.

### P2-1 — “Revocable persistent endpoints” conflates endpoint rotation with authorization revocation

**Spine:** line 86 says ordinary/federated profiles use “revocable persistent endpoints.”

**Evidence:** the Iroh research supports a persisted device endpoint and requires rotation/recovery policy (`technical-iroh-.../research.md`, lines 151–157). Identity evidence makes `DeviceGrant` and `ServiceBinding` revocable; it does not establish a network-global revocation primitive for an Iroh `EndpointId` (`technical-vida-identity-modes-.../research.md`, lines 134–145 and 172–179).

**Risk:** implementers may treat revoking an application binding as revoking the transport key everywhere, or omit explicit key rotation and stale-binding rejection.

**Recommended correction:** say “persistent endpoints with revocable `EndpointBinding`/`DeviceGrant` and explicit endpoint-key rotation.” Keep authorization revocation and endpoint lifecycle as separate operations.

### P2-2 — “Every networked client” is stronger than the proven browser/platform scope

**Spine:** lines 72–74 bind Iroh/`VidaNodeHost` to every networked client and service; line 134 makes `vida-runtime` shared across browser and native shells.

**Evidence:** current research confirms browser relay-only behavior, but explicitly leaves browser/WASM runtime coverage and possible companion/gateway boundaries unresolved. It also requires separate platform prototypes before support claims (`technical-spotify-.../research.md`, unresolved decisions 3–4; `technical-iroh-.../research.md`, lines 241–250 and 263–265).

**Risk:** the text can be interpreted as a verified requirement that browser clients directly host the same Iroh runtime, although current evidence proves only the transport constraint, not the chosen browser integration.

**Recommended correction:** define `VidaNodeHost` as the logical transport contract for every networked profile, while stating that direct in-process Iroh hosting is mandatory only for validated native/service targets. Keep browser in-process WASM versus companion/gateway as an implementation-gated decision.

### P2-3 — The source manifest omits canonical evidence used by the spine

**Spine:** sources at lines 21–26 list ADR-0001, ADR-0003, and ADR-0004, but the inherited table also uses ADR-0002 and AD-1/AD-4/AD-5 rely directly on accepted ADR-0005 and ADR-0006.

**Evidence:** `ADR-0005-iroh-transport-foundation.md` and `ADR-0006-durable-delivery-and-operation-envelope.md` are accepted repository decisions dated 2026-09-19; ADR-0002 is explicitly inherited at line 64. The identity-mode research is the upstream evidence behind AD-3 but is only reachable indirectly through ADR-0004.

**Risk:** automated or human currentness review can miss the actual normative owners, and later research refreshes may not propagate cleanly.

**Recommended correction:** add ADR-0002, ADR-0005, ADR-0006, and the identity-mode research to `sources` or introduce separate `evidence_refs` and `decision_refs` fields. Avoid relying on companions that themselves cite the spine as their source.

## Claims supported by current evidence

| Spine commitment | Current support | Assessment |
|---|---|---|
| Iroh core 1.2.0 baseline | Iroh research claim C1 and first-party release source | Supported as a dated baseline; re-check before each architecture lock/release |
| Endpoint/Router/ALPN/Address Lookup/direct-relay model | Iroh research claims C2–C6 | Supported |
| Higher `iroh-*` protocols remain deferred adapters | Iroh research C7 and explicit maturity conflict | Supported and appropriately conservative |
| VIDA-owned versioned ALPN/envelope | Iroh research sections 6, 9, and final decision | Supported architectural decision; wire details remain gated |
| Direct path plus separate encrypted durable delivery | Iroh research sections 7 and 9 | Supported pattern; topology/SLO remain unvalidated |
| Transient/durable/blob plane split | Iroh research lines 159–166 | Supported |
| Shared Rust core plus native/platform shells | Shopify/Spotify/1Password/Mozilla synthesis | Supported direction with medium-high VIDA confidence, not yet prototype proof |
| Browser relay-only constraint | Current Iroh research C6 | Supported; runtime integration remains open |
| Copyleft/SPDX boundary | Iroh research line 273 and technology review | Supported process requirement; actual inventory not yet present |

## Staleness controls

The Iroh research requires release versions, dependency pins, project activity, and migration PRs to be re-checked before every architecture lock; mobile/browser constraints must be re-checked per supported release target. The spine should carry these triggers explicitly or link to the research staleness map. “Iroh 1.2.0 baseline” is current for the 2026-09-19 decision snapshot, not a permanent latest-version claim.

## Acceptance criteria for this review

1. Artifact metadata distinguishes accepted architecture direction from unvalidated implementation readiness.
2. Lockfile/SBOM wording becomes normative future tense until concrete artifacts exist.
3. Endpoint binding/grant revocation is separated from endpoint-key rotation.
4. Browser integration remains an explicit gated profile, not an already-proven same-runtime deployment.
5. Source metadata includes every inherited and directly governing ADR/research artifact.
