# Independent rubric and input reconciliation — AD-17 open interoperability

## Gate verdict

**Revise before finalizing the AD-17 update.** The new rule captures the user's accepted scope—independent clients/nodes **and** Apps/extensions with public normative contracts, conformance evidence, and governance—and preserves Iroh/VIDA ALPN. One pre-existing spine rule still makes the promised independent path contingent on a VIDA waiver. A smaller terminology/traceability ambiguity should also be fixed.

## Critical

### C1 — AD-11's waiver conflicts with AD-17's equal independent implementation path

- **Evidence:** AD-11 says `vida-core` is the single Rust implementation and **“An alternative semantic implementation requires a time-bounded waiver ADR and differential property/fuzz/replay testing against the Rust reference”**. AD-17 says independent client/node and App/plugin implementers have the **same documented conformance path** as VIDA implementations. ADR-0013 §Decision 1–2 and `REQ-OPEN-001`, `REQ-OPEN-004` require independent compatible clients and nodes without hidden obligatory conditions; `REQ-OPEN-006` requires terms permitting independent implementation.
- **Divergence:** a third-party client or node can pass the published protocol suite yet be deemed noncompliant for lacking a project-issued waiver or Rust-core equivalence test. The waiver also makes the Rust implementation, rather than the public normative contract, the final source of compatibility truth. Two teams could therefore reach opposite conclusions about the same third-party implementation while obeying different ADs.
- **Disposition:** **Autofix by scoping AD-11.** Make the Rust-core mandate and waiver govern **VIDA-maintained/first-party release artifacts only**. External independently implemented clients/nodes should be governed by published normative profiles, shared conformance suites and security requirements, with no VIDA waiver or private Rust-reference dependency to claim baseline interoperability. Differential tests against Rust may be offered as optional/public supplementary evidence, not a private prerequisite. If the product intends mandatory waiver for outsiders, that requires an explicit new user decision because it narrows ADR-0013.

## High

### H1 — “Plugin” can be read as unrestricted executable extension

- **Evidence:** AD-17 says “App/plugin implementers” and binds `AppPackage`/runtime, but does not reference ADR-0007–ADR-0009. Those accepted ADRs distinguish declarative `AppPackage`, managed application logic through controlled host APIs, and **no arbitrary direct OS/storage/transport-access code in v1**. ADR-0008 permits external repositories, not automatic activation or grants.
- **Divergence:** a developer can take AD-17's unqualified “plugin” as a public arbitrary-code plugin ABI, while another follows the bounded AppPackage/extension model. Publishing a conformance path cannot silently expand the accepted execution/trust model.
- **Disposition:** **Autofix.** In AD-17, identify “plugin” as an **extension package under ADR-0007–ADR-0009 and the approved runtime capabilities**, expressly not a promise of unrestricted executable plugins. Add those ADRs as sources/inherited constraints; preserve separate discovery, package acquisition, activation, publisher trust and Space grants. Do not choose Rhai/Wasm or sandbox details here.

## Medium

### M1 — “Every claimed interoperable baseline” needs a public profile inventory

- **Evidence:** AD-17 requires publication for each **claimed** baseline, while the user accepted full transparency for the independent-client/node and App/package contours. ADR-0013 and `REQ-OPEN-001`–`006` bind claimed compatible profiles but do not name a v1 profile catalog yet.
- **Risk:** official releases could silently omit a critical protocol/package surface from the published baseline, making “full transparency” technically true only for a narrow advertised slice. This is not permission to force private implementation details or paid source code open; it is a boundary on interfaces required for the advertised independent use case.
- **Disposition:** **Autofix/explicit gate.** Require a public release inventory marking every externally observable protocol/package/host-API surface as normative interoperable, experimental, internal, or paid extension, with rationale and compatibility consequences. A feature that an independent compatible client/node/App must implement cannot be labelled internal to evade publication. Link to the release manifest/Conformance Plane.

### M2 — Public suite equivalence must be independent of proprietary fixtures

- **Evidence:** AD-2 and AD-17 promise public golden/negative-security vectors and release evidence; `REQ-OPEN-004` requires the same suite for independent teams. AD-17 does not explicitly ban proprietary test access or private reference-service accounts as required conditions.
- **Risk:** publication of a test catalog alone can leave the executable test harness or required endpoints unavailable, creating an unequal path.
- **Disposition:** **Autofix.** Specify that the mandatory baseline suite, fixtures and runnable harness are publicly obtainable without private credentials; hosted test infrastructure can be optional. Keep private Space data and managed-service access separate.

### M3 — Proposal governance is safely deferred, but normative status must stay clear

- **Evidence:** ADR-0013 and `REQ-OPEN-005` require public proposal/review/status history. AD-17 leaves license/IPR/governance to `OQ-0055`, and the Implementation Gate blocks production interoperability claims until it closes.
- **Disposition:** **Safely deferred.** Retain the gate. Ensure experimental proposals are not mistaken for accepted normative contracts; no need to select NIP numbering, schema language or licensing in this update.

## Good-spine checklist

| Criterion | Result | Evidence |
|---|---|---|
| Fixes real divergence for independent clients/nodes | **Fail pending C1** | AD-11 waiver/Rust-reference rule conflicts with open equal conformance. |
| Fixes real divergence for Apps/extensions | **Partial** | Common package path is accepted, but “plugin” needs bounded v1 meaning. |
| Rule is enforceable | **Mostly** | Public specs, vectors, suite and release evidence are verifiable; public inventory/runnable harness sharpen claims. |
| Deferred choices cannot silently diverge | **Pass with gate** | `OQ-0055` blocks production claims; `OQ-0028` still gates exact envelope. |
| Ratifies existing decisions | **Partial** | ADR-0005 is preserved; ADR-0007–0009 are not named; AD-11 conflicts. |
| Covers driving requirements | **Mostly** | `REQ-OPEN-001`–`006` are reflected, subject to C1. |
| Operational envelope | **Pass for this update** | Release manifest, immutable suite digests and public evidence already have owners in AD-2/AD-17. |

## Input reconciliation

- **User decision:** both independent compatible clients/nodes and third-party Apps/plugins, with full transparency — **captured by AD-17 and ADR-0013**, subject to C1/H1.
- **ADR-0013:** public normative contracts, no hidden mandatory extension, external proposal path, paid implementation may remain closed — **captured**.
- **`REQ-OPEN-001`–`006`:** protocol/profile specs, wire/security details, package/repository/host APIs, fixtures/evidence, public change history and implementable licensing — **captured or safely gated**, but AD-11 must not impose an external waiver.
- **ADR-0005:** Iroh primary transport, VIDA-owned versioned ALPN/envelope — **preserved**.
- **ADR-0007–0009 / package baseline:** common bundled/external package contract, controlled managed logic, no automatic grants, no unrestricted v1 code — **not contradicted explicitly, but should be inherited by name and clarified**.

## Handoff recommendation

Apply C1 and H1 before marking the update final. Apply M1/M2 as low-cost clarifications. Preserve `OQ-0055` and exact wire/package-format implementation gates; they do not require a new user decision now.
