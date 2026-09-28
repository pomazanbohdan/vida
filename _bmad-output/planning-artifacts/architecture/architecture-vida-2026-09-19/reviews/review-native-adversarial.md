---
review: native-adversarial
artifact: ../ARCHITECTURE-SPINE.md
focus: AD-20, ADR-0014, native-client requirements, platform NFR, stack brief
date: 2026-09-20
verdict: pass-with-one-claim-boundary-clarification
---

# Adversarial review — installed VIDA clients without a first-party browser client

## Verdict

AD-20 is consistent with ADR-0014, `REQ-CLIENT-001`–`005`, platform NFR and the updated stack-selection brief. It correctly removes a first-party browser/PWA/Wasm/gateway VIDA client and its release gate while preserving installed mobile/desktop offline behavior, City Portal public web, service nodes, open protocols and third-party implementations. No direct contradiction requiring reversal of the accepted decision was found. One conformance-claim boundary should be clarified before an independent browser client claims “VIDA-compatible”: protocol interoperability is not the same as certification of a first-party browser platform profile that VIDA intentionally does not provide.

## High finding

### H1 — Third-party browser clients have a public protocol path but no first-party platform conformance profile

**Evidence:** AD-20 says no browser/gateway/Wasm client is a VIDA product or release target, but independent implementations under public protocols remain in scope. ADR-0014 explicitly preserves the right to create another client. AD-17 grants independent clients a public conformance path. `NFR-PLAT-001` now requires OS-specific results only for supported installed clients; browser-specific lifecycle, storage, key custody and background gates were withdrawn by AD-20/AD-9.

**Two-compliant-unit construction:** an external team publishes a PWA implementing the VIDA wire profile and passes protocol fixtures. It markets itself as a “fully VIDA-compatible browser client.” An independent node accepts its messages but cannot promise its offline persistence, secure key storage, background delivery or AppPackage rendering; the PWA fails workflows official installed clients pass. Both parties correctly follow their documented protocol/platform obligations, but users hear one unqualified compatibility claim.

**Disposition — clarify claim scope, do not add an official browser product:** make release/conformance language distinguish `VIDA protocol-profile compatible` from `VIDA first-party supported platform/client profile`. Third-party browser clients may attempt public protocol conformance; VIDA does not provide, certify or promise browser platform behavior, feature parity or support. A third party may publish its own browser platform evidence, but it is not an official VIDA release gate. This preserves ADR-0013 openness without undoing ADR-0014.

## Medium findings and tested non-contradictions

### M1 — “Gateway client” versus service gateway/sidecar should not be conflated

AD-20/ADR-0014 excludes a first-party browser **or gateway client** as a product surface, while `product-boundary.md` still expects a City Portal integration service/sidecar and AD-20 retains VIDA services. A team could incorrectly delete a legitimate server-side adapter because it is called a gateway, or smuggle a first-party hosted VIDA UI behind an “integration gateway” label. Clarify in the next integration contract that a server-side protocol/booking adapter is permitted; a browser-facing VIDA super-app client delivered through it is not. `OQ-0011` still owns service placement and source-of-truth details.

### M2 — Embedded City Portal web content is not a VIDA browser client by itself

`product-boundary.md` keeps the public City Portal website separate and locates City Portal App inside installed VIDA. AD-20 does not forbid an installed shell from embedding or linking a City Portal web view, nor does it authorize replacing the installed Messenger/Knowledge/Projects experience with a thin web wrapper. A compliant integration must keep Portal identity/data/booking boundaries explicit and must not imply offline Portal business availability from VIDA's three base Apps. The exact City Portal App rendering/integration method belongs to `OQ-0011`, not to AD-20.

### M3 — “Native” remains intentionally unresolved for Tauri WebView

ADR-0014, AD-20, `OQ-0008` and the stack brief all say an installed Tauri desktop app may satisfy distribution while still using HTML in WebView, and the user has not selected whether that meets the desired “native” UI meaning. Flutter Windows is a hypothesis, Tauri a conditional challenger. No team should present Tauri as already approved or reject it merely because browser-client reuse is no longer a criterion. A Tauri spike must include privileged-command isolation, XSS→IPC, accessibility, offline/restart and installed-release evidence.

### M4 — Wasm plugin runtime is a separate axis from Wasm browser client

ADR-0009 still considers Wasm/Wasmtime as a managed AppPackage execution candidate inside an installed client. AD-20 bans a first-party browser/Wasm **client**, not a sandboxed Wasm extension executor. The current wording in ADR-0009 explicitly makes this distinction; no change to AD-20 is required.

## Conformance and offline checks

- Official release matrix: only installed OS profiles selected under `OQ-0008`; each must pass all three App scenarios under `REQ-CLIENT-005` and `NFR-PLAT-001`–`005`.
- Offline Messenger: local history and durable pending send are available; remote delivery is not falsely shown complete.
- Offline Knowledge/Projects: local read/action survive process restart; shared authority acceptance remains pending and conflict/revocation rules still apply.
- City Portal public web: separate product/repository and not a VIDA browser shell; integration service remains in scope.
- Independent clients/nodes: public protocol and App contracts remain open under AD-17, with claim scope and corresponding conformance evidence stated precisely.

## Closure recommendation

Treat AD-20 as accepted architecture direction. Add H1's precise claim language to the Conformance Plane or open-interoperability requirements, and add M1's service-versus-client distinction in the future City Portal integration contract. Keep OS targets and Tauri/WebView eligibility in `OQ-0008`; do not infer either from “installed app.”
