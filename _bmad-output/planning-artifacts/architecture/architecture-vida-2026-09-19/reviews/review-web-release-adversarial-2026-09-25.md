# Adversarial review — Release-1 static Web client

**Scope:** `ARCHITECTURE-SPINE.md` as updated 2026-09-25, checked against `ADR-0021-static-web-client-in-release-1.md`, the Release-1 PRD and the call-conformance draft. This is a semantic review, not a claim that any prototype passed.

**Verdict:** Web inclusion is product-consistent at the top level, but the Web implementation is **not ready for independent fan-out**. Four browser-specific contracts remain underdefined and one mandatory call proof still names only the three installed platforms. Keep the Release-1 commitment; close these seams before separate teams build clients, deployment, media and package execution.

## 1. High — Web call evidence is still a three-platform gate

**Evidence:** Spine AD-20 (lines 222–226) and ADR-0021 require Web E2EE calls and reject a reduced browser companion. The PRD requires the full call suite on Web (§6.2–6.3, lines 538–554). Yet Spine AD-35 (lines 313–317) selects media after *physical Android/iOS/Windows* evidence, and `docs/04-specifications/e2ee-calls-conformance.md` G2 (line 52) likewise covers only Android/iPhone/Windows.

**Two plausible builds:** Media team A completes G2 on the installed clients and ships a Web WebRTC adapter with a different browser E2EE/key path, judging the selected media profile proven. Web team B refuses profile selection until browser↔native and browser↔browser calls pass equivalent E2EE, key-rotation, device-transfer and lifecycle fixtures. Both can point to a current gate, but their Release-1 proof and interoperability differ.

**Required closure:** Amend AD-35 and the call-conformance gate so candidate selection/release explicitly includes a supported-browser matrix and Web↔native/Web↔Web media fixtures, or state that the existing G2 is only a partial gate. Define the browser cryptographic/media-capability floor before selecting a candidate. This is a contract correction, not permission to drop Web calls.

## 2. High — A static host is still an executable-code trust principal

**Evidence:** PRD FR-39 (lines 502–509) says static hosting does not confer access to open content, while ADR-0021 says the host does not become Persona/Space owner. Spine AD-9 (lines 156–160) lists origin/browser-security gates and AD-11 (lines 168–172) puts browser key custody in the shell, but no rule identifies which origins/release bundles may run trusted code or how a browser distinguishes an authentic update from a host/CDN injection. PRD OQ-19 acknowledges the missing browser contract.

**Two plausible builds:** Deployment A publishes mutable Flutter JS/Wasm under a convenient public Pages/CDN origin and treats TLS as sufficient; injected same-origin code can read plaintext at the UI/Core boundary or request operations as the enrolled Device. Deployment B verifies a versioned artifact and constrains origin, scripts and updates before key material is released. Both are static, use Rust/Wasm Core and an Iroh relay, yet have irreconcilable threat models. E2EE against the relay does not protect against code served by a compromised origin.

**Required closure:** Specify a trusted-origin/release-artifact model: who controls and may rotate the origin, integrity/authenticity of JS/Wasm/service-worker updates, CSP and dependency loading policy, key-unlock boundary, response to a compromised origin and independent self-hosting claims. Test malicious/mixed/rollback asset delivery. Reword any blanket claim that an arbitrary static host cannot access plaintext; the host lacks *domain authority*, but delivered code must be trusted or independently verified.

## 3. High — Browser storage eviction and DeviceGrant lifecycle can diverge

**Evidence:** PRD FR-2 (lines 128–135) and ADR-0021 make an enrolled browser an equal Device. PRD FR-39 (lines 502–509) requires local durability, honest single-copy status and verified recovery/export. Spine AD-37 (lines 325–329) requires ControllerState/DeviceGrant transitions, but the Web storage, key and recovery profile is deferred in ADR-0021 and PRD OQ-19.

**Two plausible builds:** Browser team A stores a long-lived Device signing key in origin storage and treats browser-data eviction as a lost Device, leaving its valid grant in ControllerState until manually revoked. Team B stores only session material and silently creates a new Device after eviction/re-enrollment. Both can present an authorized equal Web Device, but differ on grant accounting, last-copy loss, key recovery, replay and orphaned Device cleanup. A private window or changed origin adds a third incompatible interpretation of "same Device."

**Required closure:** Define Web Device identity persistence, key non-exportability/backup choice, orphan-grant detection and revocation, last-copy warnings, verified export/recovery and exact behavior after quota eviction, browser-profile clearing or origin migration. Require crash/eviction/reopen/new-origin fixtures that prove no false “saved/synchronized” or unauthorized re-enrollment. This may be resolved under OQ-19/OQ-0024, but is a hard pre-fan-out boundary.

## 4. Medium — Static Web bundle activation is not atomic across caches and tabs

**Evidence:** Spine AD-8 (lines 150–154) requires one compatibility tuple including Web Rust/Wasm bindings and supported mixed-version tests. AD-29 (lines 276–281) requires atomic AppPackage activation. ADR-0021 defers the precise Rust/Wasm bridge. Neither specifies the client-host artifact boundary when browser cache/service worker serves old Wasm with new Flutter JS, or when two tabs run different runtime versions against one origin database.

**Two plausible builds:** Web team A updates independent static asset URLs and lets browser caches mix generations; on startup it attempts migration immediately. Team B pins a complete build manifest, activates it atomically per origin database and makes incompatible old tabs read-only. Both can publish a compatible release tuple in CI, but a real partial rollout can produce divergent binding and persistence behavior.

**Required closure:** Give each deployable Web bundle an immutable compatibility manifest binding Flutter assets, Rust/Wasm, host API and database schema; specify cache/service-worker activation, old-tab handling, offline update and interrupted migration. Add mixed-assets/dual-tab/stale-worker fixtures to AD-8/OQ-19 before Web client and deployment work split.

## 5. Medium — Web AppPackage execution profile is narrower than “full parity” but not explicit

**Evidence:** PRD FR-32 (lines 428–437) requires bundled/external declarative parity and expressly bans downloaded handlers on iOS. Spine AD-33 (lines 301–305) limits only iOS, while AD-39 (lines 337–341) demands the same signed declarative-package semantics on all first-party Flutter shells. No Web rule decides whether downloaded Rhai/Wasm/JavaScript handlers can execute, which built-in capabilities Web must implement, and what happens when a package targets an installed-only capability.

**Two plausible builds:** Package team A marks Web as able to execute a signed downloaded Wasm handler; Web team B implements only declarative forms/workflows through built-in Core capabilities, mirroring iOS. Both can render signed declarative packages, but a shared Space may activate one AppPackage whose rules execute on one Device and stall or diverge on another.

**Required closure:** State the Release-1 Web execution tier and capability-negotiation/activation failure rule, with cross-platform fixtures for a package containing a handler or missing browser capability. Keep security review of any downloaded code separate from data/Space grants. This should be decided with OQ-0056/OQ-0068 before Marketplace fan-out.

## Acknowledged, not counted as a new defect

- Spine AD-1 (lines 108–112) correctly distinguishes Iroh relay from Address Lookup, mailbox and authority. Provider order/TTL/outage policy remains explicitly deferred as OQ-0030; it still must close before production discovery. A relay alone is not peer discovery or durable offline delivery.
- Spine AD-9 explicitly avoids native background guarantees on Web; Web being unavailable while a tab is closed is not a violation by itself. Product and UI must continue to mark undelivered operations pending.
- ADR-0021 accurately disclaims direct browser P2P with the current Iroh/Wasm profile. This review does not ask to invent a direct Web path or use Delta Chat Relay.
