# Rubric review — static Web in Release 1

Reviewed: `ARCHITECTURE-SPINE.md` only, 2026-09-25. This is a semantic good-spine review, not a review of its referenced PRD, ADRs, code or external technology claims. The deterministic BMad spine linter passed with zero findings.

## Verdict

**Conditional pass.** The spine names the paradigm, assigns Core/runtime/shell authority, binds static Web to Release 1, separates native direct connectivity from Web relay connectivity, and preserves the main protocol, security and operational gates. Three Web-specific seams still permit independently built units to make incompatible release decisions; close them before accepting the Release-1 Web implementation plan or stories.

## Findings

### WEB-R1 — High: browser local durability has no explicit parity test for the mutation contract

- Evidence: AD-4 and AD-13 require the origin intent, outbox, dedupe and frontier to commit together and survive crash/replay (lines 126–136, 180–184). AD-20 promises offline use of locally available Web data/actions (lines 222–226). AD-9 names browser storage/quota/eviction gates, but specifies no observable outcome for a quota failure or eviction (lines 156–160). The implementation gates name a replaceable-storage contract in OQ-0036, but do not explicitly bind the browser persistence profile to it (line 385).
- Divergence: the Web shell could display “saved locally” after an in-memory or partially persisted write while Android waits for an atomic durable commit; either client would appear compliant with its own interpretation of the generic Web gate.
- Recommended disposition: **autofix in spine**. Bind the Web storage adapter to AD-4/AD-13 and the OQ-0036 crash/failure fixtures. State that a Web edit receives “saved locally” only after its browser-origin durable transaction succeeds; quota, eviction and unavailable persistence fail visibly without dropping pending operations. Include asset/Wasm availability in the offline fixture, or narrow the offline promise explicitly. Keep IndexedDB/OPFS/service-worker technology choice deferred.

### WEB-R2 — High: Release-1 call parity and the Web media gate are not expressed in one enforceable rule

- Evidence: AD-20 requires a full-featured Web client and says Core-App/call gates cannot be bypassed (lines 222–226). AD-35 names physical Android/iOS/Windows E2EE/lifecycle fixtures, but omits Web from that sentence (lines 313–317). A later Deferred item says Web binding/storage/security/media proof is a Release-1 gate (line 463), without naming Web-specific call conditions in AD-35 or the Implementation Gates.
- Divergence: the media team could pass installed-client call tests while a Web team treats a signaling-only or unencrypted-browser call as sufficient, or ships the browser without calls despite the full-featured rule.
- Recommended disposition: **autofix in spine**. Extend the Release-1 call gate to Web with browser E2EE, key custody, suspend/reload/reconnect, permission-denial, microphone/camera and mixed Web↔installed participant fixtures. If a browser media profile cannot pass, it blocks Release-1 Web acceptance rather than silently downgrading Web to a companion.

### WEB-R3 — Medium: static-origin release and update ownership are left implicit

- Evidence: AD-8 includes Web Rust/Wasm API and mixed-version compatibility (lines 150–154); AD-15 assigns hosted/federated/self-hosted operational ownership but does not name the static Web origin (lines 192–196); AD-20 requires static hosting (lines 222–226). The structural seed has `shells/web/`, but no explicit rule binds published HTML/JS/Wasm assets, browser cache and runtime compatibility to one release (lines 398–417).
- Divergence: a static site/CDN publisher could change Flutter assets independently of the Wasm bundle or conformance manifest, while another team assumes atomic version activation. Users may run stale cached assets against a newer runtime with no defined recovery signal.
- Recommended disposition: **autofix in spine**. Require an immutable signed/hashed Web release manifest tying shell assets, Wasm, Core/API compatibility tuple and cache activation together; name ownership of static origin, relay configuration and security headers; enforce mixed-version/rollback/stale-cache fixtures. Hosting vendor, CDN and exact cache strategy can remain deferred.

### WEB-R4 — Medium: online-device proof omits the new Web Device profile

- Evidence: AD-20 calls an authorized browser session a Web Device (lines 222–226). AD-26 makes online count and “synchronized” user-visible state depend on fresh application-level proof but says freshness/retry thresholds are measured for Android/iOS/Windows prototypes only (lines 258–262). The OQ-0064 implementation gate references platform suspension without a Web-specific inactive-tab, browser-close or evicted-storage case (line 378).
- Divergence: one client may count a stale Web tab as online while another drops it; operation status could also conflate relay reachability with application receipt.
- Recommended disposition: **autofix in spine**. Include Web in OQ-0064 presence/receipt fixtures, with tab suspension, browser close, relay-only reachability, profile isolation and stale/replayed proof. A Web Device follows the same application-proof rule, not a WebSocket/QUIC connectivity shortcut.

## Rubric summary

| Criterion | Result |
|---|---|
| Platform-level divergence coverage | Broad coverage; Web durability, media and origin-release seams need explicit closure |
| Enforceable AD rules | Most ADs have Binds/Prevents/Rule; findings above target remaining ambiguous Web gates |
| Deferred technology | Appropriate for concrete storage, media and hosting choices; their **required behavior** must not be deferred |
| Operational/environmental envelope | Present for nodes; static Web origin/update ownership needs addition |
| Named technology currency | Not independently verified in this single-file review; exact version/checksum is already required by the spine |
| Mechanical lint | Passed; 0 findings |

No changes to the spine are made by this review. The parent architecture update should triage and apply the clear fixes before handoff.
