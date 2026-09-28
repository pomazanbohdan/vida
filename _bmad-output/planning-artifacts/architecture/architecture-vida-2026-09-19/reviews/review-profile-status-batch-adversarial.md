---
review: profile-status-batch-adversarial
artifact: ../ARCHITECTURE-SPINE.md
focus: AD-24–27, presence, quorum, publication, effects, agents, rules, downloads
date: 2026-09-20
verdict: adopted-boundaries-coherent-seven-independent-implementation-divergences-gated
---

# Adversarial review — profile, status, chat, effects and downloads

## Verdict

The amendment preserves the crucial separations: profile count ≠ Space total, two same-Persona copies ≠ automatic Space authority, transport ACK ≠ domain acceptance, and download admission ≠ retention/GC. No direct contradiction found in AD-24–27. **Seven independently reasonable implementations still diverge** because the evidence and policy behind those separations remain open. Existing implementation gates at `ARCHITECTURE-SPINE.md:282–288` must remain hard; the examples below are pre-gate candidates, not production-conformant peers.

## Two-implementation attacks

### H1 — Profile online count and privacy [OQ-0064]

One profile has two devices. Peer A counts a device online after an authenticated application-level exchange with another authorized device; peer B counts it once a relay path is reachable, even if no peer completed sync. Both interpret “can actually synchronize” and a “usable relay” differently (`ARCHITECTURE-SPINE.md:248`; `sync-presence-status-model.md:14,28`). During partition they show 1 versus 2. For privacy, A treats only an explicitly public profile as visible; B treats a contact-visible private/anonymous profile as visible and exposes the count to all its viewers. Both can claim “every viewer of the visible profile” because visibility is unresolved (`OQ-0064`, `open-questions.md:74`). Even with no cross-Persona sum, correlated count changes can link otherwise separated private/corporate contexts; a revoked viewer may keep polling stale presence. The gate must specify sync-capability proof, TTL/unknown state, profile audience, suppression/revocation and cross-context count-differencing fixtures—not just hide raw device IDs (`ARCHITECTURE-SPINE.md:285`).

### H2 — Same-Persona “quorum” can mean replication or authority [OQ-0033]

Bogdan's phone and laptop exchange one signed operation. Peer A marks “synchronized” after two durable copies; peer B waits for the applicable Space/object authority receipt. Both keep a separate `authority.accepted` field, but the identical user-facing word now means different things. More dangerously, a shared-Space policy could explicitly count the two DeviceGrants as its threshold while another counts independent Persona/member approvals; neither is fixed by saying devices are equal. The user called the two-copy event a “quorum,” yet expressly did **not** approve it as shared-Space authority (`decision-batch-device-core-2026-09-20.md:58–59`; `open-questions.md:43`; `ARCHITECTURE-SPINE.md:285`). Close `OQ-0033` with authority-policy proof and distinguish replication cardinality, member quorum and status-label scope. A second device is not implicitly a second Owner.

### H3 — Offline group-chat publication has no common order witness [OQ-0065]

Two isolated peer groups each receive an offline group message. Peer A treats each group's local acceptance as the publication event and orders by receipt time; peer B waits for common reconciliation and orders by a deterministic operation tie-break. Both put a 10:00 draft into the shared history when they consider it published around 13:00, never by written-at metadata, yet they disagree on message order and unread frontier. A late message can still insert before already-read messages if its 13:00 local receipt becomes visible after another group's 14:00 message. AD-27 fixes the product preference but leaves publication evidence, incomparable ties and clock semantics open (`ARCHITECTURE-SPINE.md:254`; `REQ-CLIENT-008`, `native-client-requirements.md:21`; `open-questions.md:75`). Define a verifiable publication receipt/order or explicit incomparable presentation, and test partition, rejoin, mixed clock skew, unread markers and Messenger/project-chat parity before release (`ARCHITECTURE-SPINE.md:287`).

### H4 — External correction and `outcome_unknown` diverge [OQ-0045]

An email saying `Done` is sent, then a valid late `Cancelled` branch appears. Peer A proposes a human-reviewed follow-up email; peer B proposes only an internal corrective task. Both preserve the first audit fact and offer a separate correction, satisfying AD-24/`REQ-EFFECT-011`, yet the customer's outcome differs (`ARCHITECTURE-SPINE.md:236`; `effect-execution-constraints.md:30`). If the provider timed out after the original request, A may stop in `outcome_unknown`; B may retry using an idempotency key whose provider retention window has expired. The references are explicitly **not** VIDA retry policy (`effect-execution-constraints.md:34–38`). `OQ-0045` must bind correction command/actor/rights, provider-aware result lookup, safe retry window, durable logical effect ID and terminal unknown/manual-review UX. Do not claim a sent email was “undone,” and do not auto-retry an unprovably idempotent request (`ARCHITECTURE-SPINE.md:286`).

### H5 — Generic agent ACL admits incompatible consent scopes [OQ-0045]

Peer A records action-type opt-in as a persistent `ServicePrincipal` capability for every `file conflict resolution` in a Space. Peer B requires a delegator-signed consent per conflict, still typed as `file conflict resolution`. Both use the general layered ACL, avoid a special Owner/Admin approval tier, and recheck current variant read/action rights (`ARCHITECTURE-SPINE.md:242`; `REQ-EFFECT-009`, `effect-execution-constraints.md:28,40`). Yet A can act on a later unseen conflict under an old broad consent while B cannot. The open mapping must specify issuer authority, Space/resource scope, lifetime, revocation, conflict-head binding, agent/delegator attribution and prompt-injection-resistant tool authorization. The user approved opt-in for a **type**, not an unbounded Space-wide grant (`decision-batch-device-core-2026-09-20.md:63`).

### H6 — Fact accepted under v1, first processed under v2 [OQ-0048]

AppInstance accepts source fact F while rule v1 is installed, but no executor runs before the package upgrades to v2. Peer A pins v1 at fact acceptance and later creates note format A; peer B pins v2 at first execution and creates format B. Each can claim F is immutably bound to its chosen “effective” rule version, but the peers derive different content or two IDs. If a third peer merely receives the already-created note, it correctly syncs data rather than rerunning logic; that does not settle the unprocessed-F case (`ARCHITECTURE-SPINE.md:170,288`; `REQ-EFFECT-010`, `effect-execution-constraints.md:29,42`; `open-questions.md:58`). `OQ-0048` must define the authority binding point, immutable content identity, availability/fail-closed behavior for old or withdrawn rule code, canonical resource ID and crash/rollback fixtures.

### H7 — Download threshold must not become blob-retention policy [OQ-0066/0034]

For a 30 MiB conflict variant, peer A uses a 20 MiB **per-device** threshold and keeps only its manifest until user confirmation; peer B uses a 50 MiB **per-profile** threshold and downloads bytes automatically. Both follow the stated behavior because threshold scope/default and exact size metric are open (`blob-store-contract.md:45,57`; `open-questions.md:76`). Their offline availability and bandwidth cost differ. A dangerous implementation might then count manifest-only as “no local owner” and GC the last payload on B; that crosses a separate boundary: accepted conflict references/pins survive until a safe retention frontier regardless of any device's download choice (`blob-store-contract.md:60–63`; `ARCHITECTURE-SPINE.md:287`). Close `OQ-0066` on setting scope, logical/encrypted size, cellular policy and explicit confirmation; close `OQ-0034/0036` on pin ownership, quota failure, minimum recoverable copies and payload-unavailable UX. Test manifest-only A + sole-payload B + storage pressure + late conflict/rejoin.

## Disposition

Keep AD-24–27 as product-level choices, but do not turn their open proof fields into implementation defaults. Require cross-vendor fixtures for each A/B pair and preserve the explicit release gates for `OQ-0033/0034/0045/0048/0064/0065/0066`; private/anonymous count exposure and provider retry deserve negative-security tests, not merely happy-path UI checks.
