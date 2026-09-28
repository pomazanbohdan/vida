# Rubric review — Web direct-first + encrypted relay fallback (2026-09-25)

## Verdict

**Needs two high-priority clarifications before Epic-2 implementation fan-out.** The spine correctly avoids claiming stock Iroh/Wasm supports browser P2P, separates route from operation/authority, and treats Web direct transport as prototype-gated. It does not yet force independent Web and native implementers to choose the same adapter boundary or the same direct→relay transition semantics.

## Findings

1. **High — WebRTC adapter boundary permits incompatible implementations** (`ARCHITECTURE-SPINE.md:108-112`, `:222-226`, `:355-356`). AD-1 mandates Iroh inside `vida-runtime`, but “WebRTC/custom adapter under the same VIDA Core/envelope contract” can mean either an Iroh custom transport carrying the Iroh session/ALPN or a parallel WebRTC DataChannel transport carrying only VIDA envelopes. Those have different discovery, authentication, path inspection and interoperability obligations. **Discuss/then fix AD-1:** name which boundary is normative for Release-1 Web direct sync; if both are permitted, define one versioned logical transport contract and cross-adapter conformance vectors before stories 2.2/2.3. Keep candidate libraries prototype-gated rather than binding an unproven crate.

2. **High — “direct-first” lacks enforceable path-selection evidence** (`:112`, `:226`, `:360-363`, `:461`). One implementation could send application bytes through relay immediately while racing direct establishment; another could require a direct attempt and only then fall back. Both can claim “direct-first” yet differ from the approved behavior. **Autofix in spine/gate:** state that route status is observed at the VIDA application boundary; define a verifiable attempt→failure/unreachability→relay-fallback state machine, path-switch idempotency and a conformance trace for same-LAN, WAN/NAT and direct-disabled conditions. Numeric timeout and provider choices may remain deferred.

3. **Medium — Release-1 consequence of failed Web direct prototype is implicit** (`:112`, `:226`, `:360-363`, `:463`). The spine says direct capability is a prototype gate and denies a non-sync companion, but the Implementation Gates do not explicitly say whether relay-only Web can satisfy Release 1 if direct proof fails. **Discuss/then fix:** if direct-first is a release requirement, make failed Web direct proof a blocker requiring a new architecture/product decision; do not silently relabel relay-only operation as compliant fallback.

4. **Medium — Relay confidentiality term could be overread** (`:112`, `:226`, `:354-355`). “Encrypted relay” does not specify whether relay operators are cryptographically unable to read Space payload, nor does it tie the route to the same signed/encrypted operation envelope. **Autofix:** cross-reference the common envelope and key/epoch contract; require relay-path negative fixtures for plaintext leakage and unauthorized replay. This is a security conformance detail, not a demand to choose cryptography here.

## Checklist result

| Good-spine criterion | Result |
|---|---|
| Real divergence points fixed for Epic 2 | Partial: route priority and adapter boundary remain ambiguous |
| `AD` rules enforce their stated prevention | Partial: AD-1 “direct-first” needs observable test semantics |
| Deferred items safe to defer | Mostly: timeout/provider selection can wait; Web direct release consequence cannot |
| Named technology not overclaimed | Pass for browser P2P: stock Iroh/Wasm is explicitly relay-only and WebRTC is proof-gated; current version verification is outside this focused rubric pass |
| Parent/source coherence | No parent spine indicated; ADR-0021 is cited, but this pass did not independently reconcile its text |
| Initiative dimensions covered | Pass in this scope: deployment/operations, security, compatibility, platform lifecycle and failure behavior are present |

## Positive contract already preserved

- AD-1, AD-20 and AD-26 distinguish Address Lookup, data relay, application receipt and domain acceptance.
- The same signed operation identity survives direct/relay paths, preventing route switching from creating duplicate business actions.
- Static Web is treated as a full client with browser-specific storage, security and lifecycle gates, not as a native-equivalent Iroh/Wasm endpoint.
