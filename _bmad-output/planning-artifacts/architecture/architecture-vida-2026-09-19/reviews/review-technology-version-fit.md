# Technology, Version, and Fit Review

- Review date: 2026-09-19
- Artifact: `ARCHITECTURE-SPINE.md`
- Lens: named technology, current version, platform fit, and license boundary
- Verdict: **PASS** after re-review
- Open findings: **0 P0 / 0 P1 / 0 P2**
- Prior findings: **2 P1 / 3 P2, resolved or controlled by explicit implementation gates**

## Re-review — 2026-09-19

| Prior finding | Result | Evidence in updated artifacts |
|---|---|---|
| P1-1 reproducible version pin | **Resolved** | Stack fixes the baseline at `iroh 1.2.0`; exact versions/checksums are mandatory in the lockfile and release SPDX SBOM; ADR-0005 explicitly rejects semver ranges as release pins. |
| P1-2 cross-platform VIDA protocol boundary | **Resolved** | AD-2 and ADR-0005 require one normative codec/kernel or the same byte-exact golden vectors on every platform. `vida-node-host-contract.md` explicitly states that FFI is transport-only. Implementation is blocked by OQ-0028 until the envelope, signature domain, framing, negotiation, and vectors are canonical. |
| P2-1 0.x maturity | **Controlled** | Higher protocols remain deferred, pinned, replaceable adapters that require compatibility prototypes; none is canonical VIDA state. |
| P2-2 `iroh-docs` authority mismatch | **Controlled** | AD-2 limits docs to an adapter; AD-6 reserves membership/authorization to signed Space policy; SyncLog remains the authority-accepted durable log. |
| P2-3 incomplete license rule | **Resolved** | AD-10 requires exact version/checksum and integration mode for every direct/transitive component in the release SPDX inventory and separately names MPL, GPL, and AGPL review. |

### Remaining user decisions

The technology/version/fit review has no remaining blocker. The architecture artifact still needs these product/architecture decisions:

1. **Immediate finalization decision:** approve or replace the `[ASSUMPTION — review]` design paradigm: hexagonal local-first architecture.
2. **OQ-0027:** durable-delivery production topology and trust model.
3. **OQ-0028:** canonical operation-envelope fields, signature domain, serialization, framing, negotiation, and golden-vector format.
4. **OQ-0029:** blob encryption, key wrapping, authorization, ticket, and deletion contract.
5. **OQ-0030:** Address Lookup provider order, privacy policy, cache TTL, and fallback per platform.
6. **OQ-0031:** membership authority ordering, causal cut, and concurrent-admin precedence.
7. **OQ-0032:** per-device ACK aggregation and sender-facing completion semantics.

## Executive verdict

The central choice is technically sound: `iroh` 1.2.0 is the current stable core release, and its `Endpoint`, `Router`, ALPN dispatch, Address Lookup, direct-QUIC, and encrypted-relay model fit AD-1 through AD-3. The spine is also appropriately conservative about the pre-1.0 higher protocols and browser asymmetry.

The earlier implementation-level compatibility gaps are now closed by normative release pinning, cross-platform codec/vector rules, and explicit implementation gates. The remaining open questions are intentionally deferred product/architecture decisions, not technology-fit defects.

## Verified technology baseline

| Technology / claim | Verified current state | Fit assessment |
|---|---|---|
| `iroh` core | `1.2.0`, published 2026-09-09; stable 1.x API; MIT OR Apache-2.0 | **Good fit.** The documented API contains Endpoint, Router/ProtocolHandler, ALPN routing, Address Lookup, direct QUIC, NAT traversal, and encrypted relay fallback. |
| `iroh-blobs` | Latest `0.103.0`; depends on `iroh ^1.0.0`; MIT OR Apache-2.0; upstream explicitly says it is not production quality and recommends `0.35` when production quality is required | **Prototype only.** The replaceable-adapter decision is correct; “latest” must not be selected automatically. |
| `iroh-gossip` | Latest `0.101.0`; pre-1.0; MIT OR Apache-2.0; implements HyParView/PlumTree topic swarms | **Good transient-plane candidate.** It is not durable storage, which matches AD-7. |
| `iroh-docs` | Latest `0.101.0`; pre-1.0; depends on `iroh-blobs 0.103` and `iroh-gossip 0.101`; MIT OR Apache-2.0 | **Reference/prototype only.** Its namespace-secret write-capability model does not directly implement VIDA Space membership or role policy. |
| `iroh-ffi` | First-party Swift, Kotlin, Python, and Node.js bindings mirror the stabilized iroh 1.0 transport surface; higher protocols are explicitly out of scope; MIT OR Apache-2.0 | **Core transport fit only.** It does not solve VIDA envelope, sync, blob, or gossip bindings. |
| Browser | Core can compile to browser WASM; upstream describes the browser profile as relay-only because browsers cannot open raw UDP/QUIC sockets; browser features differ from native | **Correctly constrained by AD-9.** A concrete browser runtime/binding choice is still required. |
| Mobile | First-party Swift and Kotlin bindings exist; iOS and Android are named supported platforms | **Promising, not proof of lifecycle fit.** Background execution, wake-up, key storage, and recovery remain VIDA conformance work as AD-9 states. |
| Iroh licensing | Core, relay, FFI, blobs, gossip, and docs publish MIT OR Apache-2.0 | **Compatible with an open-core product in principle**, subject to normal attribution/dependency review. |

## Original P0 review — Blocking

No P0 findings.

## Original P1 findings — Resolved in re-review

### P1-1 — “1.2.x pinned” is not a reproducible pin

**Location:** Stack table: `Iroh core | 1.2.x pinned per release`.

**Evidence:** The current release is exactly `1.2.0`, while the `1.2.x` notation is a version family, not an immutable dependency identity. Iroh releases also carry independently versioned transitive crates such as `iroh-relay`, `iroh-dns`, and `noq`.

**Risk:** Two builds that both satisfy the spine may resolve different patch versions or FFI artifacts. That breaks AD-8’s independent compatibility axes and weakens release reproducibility.

**Required change:** Define the release authority and artifact set: exact Cargo version plus lockfile checksum, exact FFI package/artifact versions and checksums, relay-server version, and the compatibility-test result that promotes the set. Keep `1.2.x` only as the allowed upgrade line, not as the release pin.

### P1-2 — The cross-platform implementation boundary stops below VIDA-owned protocols

**Location:** AD-2, AD-8, AD-9, and `platform-bindings/` in the structural seed.

**Evidence:** `iroh-ffi` explicitly mirrors only the stabilized iroh 1.0 transport surface and excludes `iroh-blobs`, `iroh-docs`, and `iroh-gossip`. Its JavaScript package is documented as Node.js; this is not evidence that the same binding is the browser runtime. Swift/Kotlin/Node/browser clients therefore receive byte streams, but the spine does not identify where the canonical VIDA envelope codec, ALPN handlers, signature verification, idempotency rules, or conformance vectors live.

**Risk:** Native, mobile, Node, and browser teams can all comply with the current text while producing incompatible envelope encodings or state transitions. That is a direct AD-8 interoperability failure.

**Required change:** Choose and record one strategy: (a) a shared Rust protocol kernel exported to every supported platform, including a separately proven browser/WASM build; or (b) language-native implementations generated from one normative wire schema plus canonical byte-level fixtures and cross-language conformance tests. State explicitly that Iroh FFI is transport-only.

## Original P2 findings — Resolved or controlled in re-review

### P2-1 — The higher-protocol maturity gate needs a “no latest by default” rule

**Location:** AD-2 and Stack table.

**Evidence:** Upstream labels `iroh-blobs 0.103.0` “not yet considered production quality” and directs production users to `0.35`; the newest `iroh-docs` transitively selects blobs/gossip versions. Version number recency is therefore not a maturity signal.

**Risk:** A prototype could silently become a production dependency or persisted-data format merely because it is the latest compatible crate.

**Recommended change:** Add an adoption gate per adapter: maturity statement, exact version, wire/storage migration test, fuzz/interoperability evidence, rollback path, and explicit prohibition on making a 0.x adapter’s persisted format canonical VIDA state.

### P2-2 — `iroh-docs` must be classified as reference/prototype, not an implicit SyncLog implementation

**Location:** AD-2, AD-6, AD-7, and Deferred.

**Evidence:** `iroh-docs` models entries using namespace and author keys; possession of the namespace secret is a write capability. VIDA instead makes signed Space policy, roles, capabilities, and key epochs authoritative.

**Risk:** An adapter could accidentally promote a transport/library capability key into application authorization, contradicting AD-6 and ADR-0001.

**Recommended change:** State that `iroh-docs` cannot be the authorization source or canonical operation schema. Any experiment must validate a VIDA operation before insertion and treat the replica as derived transport/storage state.

### P2-3 — The copyleft wording is not a complete license inventory rule

**Location:** AD-10.

**Evidence:** The Iroh stack is permissively licensed, while the referenced Delta Chat/Chatmail ecosystem is mixed: current Chatmail core declares MPL-2.0, Delta Chat Android declares GPL-3.0-or-later, and other ecosystem components may use AGPL-3.0. Naming only MPL and AGPL can cause GPL obligations to be missed.

**Risk:** “Patterns only” can drift into copied or linked implementation without the correct distribution/source obligations being recorded.

**Recommended change:** Replace the enumerated wording with an all-license gate: record component, exact revision, SPDX expression, provenance, integration mode (`idea`, `spec`, `source-copy`, `link`, `process`, `service`), modification status, and distribution/SaaS obligations. Name MPL/GPL/AGPL as examples, not the complete set. This is an engineering control, not a legal conclusion.

## Acceptance evidence recorded by re-review

The updated spine and companions now record:

1. An exact, reproducible Iroh release bill of materials and promotion owner.
2. The canonical cross-platform VIDA protocol implementation strategy and byte-level conformance suite.
3. Explicit prototype-only adoption gates for every 0.x Iroh protocol adapter.
4. A complete third-party SPDX/provenance/integration-mode register.

## Primary sources

- [iroh 1.2.0 package documentation and release history](https://docs.rs/crate/iroh/latest)
- [Iroh core repository and MIT/Apache-2.0 license](https://github.com/n0-computer/iroh)
- [Iroh FFI scope, languages, and license](https://github.com/n0-computer/iroh-ffi)
- [Iroh FFI support matrix](https://github.com/n0-computer/iroh-ffi/blob/main/support-matrix.yaml)
- [Iroh browser support: relay-only browser profile and feature limits](https://www.iroh.computer/blog/iroh-0-32-0-browser-alpha-qad-and-n0-future)
- [iroh-blobs 0.103.0 maturity warning, dependencies, and license](https://docs.rs/crate/iroh-blobs/latest)
- [iroh-gossip 0.101.0 protocol, dependencies, and license](https://docs.rs/crate/iroh-gossip/latest)
- [iroh-docs 0.101.0 data model, dependencies, and license](https://docs.rs/crate/iroh-docs/latest)
- [Current Chatmail core package metadata: MPL-2.0](https://raw.githubusercontent.com/chatmail/core/main/Cargo.toml)
- [Delta Chat Android: GPL-3.0-or-later](https://github.com/deltachat/deltachat-android)
