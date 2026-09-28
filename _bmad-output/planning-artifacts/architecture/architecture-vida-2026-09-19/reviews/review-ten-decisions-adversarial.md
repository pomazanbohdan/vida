---
review: ten-decisions-adversarial
artifact: ../ARCHITECTURE-SPINE.md
focus: AD-13, AD-16, AD-22–25 and related requirements/specifications
date: 2026-09-20
verdict: intent-coherent-one-text-conflict-and-six-gated-risks
---

# Adversarial gate — ten device/core decisions

## Verdict

The adopted direction is mostly coherent and implementation gates are appropriately explicit. One textual issue should be fixed before claiming the spine internally consistent: the late-branch rule does not distinguish comparable acceptance from genuinely incomparable acceptance. Presence-count privacy has no corresponding release gate; effect, equivalence, retention, agent and rule-version mechanisms remain correctly unproven. This is **not** a green production/conformance verdict. AD-16/AD-18 now consistently apply the seven-day shared-Owner read lock.

## Findings

### H1 — Late branch can override the comparable-first rule [fix now]

`AD-23` says a late valid incompatible branch outside a resolution's causal base reopens conflict (`ARCHITECTURE-SPINE.md:230`); `REQ-SYNC-011` says the same without qualifying the **acceptance relation** (`transport-sync-requirements.md:54`). Suppose a shared authority verifiably accepts resolution R at sequence 10, then receives/accepts old-base C at 11. R and C are causally unrelated as operations, but their acceptance receipts are comparable. AD-19/AD-22 and `REQ-SYNC-004/007` retain the first accepted result in that case (`ARCHITECTURE-SPINE.md:206,224`; `transport-sync-requirements.md:47,50`), whereas a literal AD-23 reader reopens no-winner conflict. If C cannot validly be accepted after R, that rejection rule must be explicit rather than inferred from “late.” Test C accepted before R, after R, and incomparable with R; distinguish delivery lateness from authority-order incomparability.

### H2 — Space presence count is an identity/correlation oracle [add explicit gate]

The draft exposes a count of “known available” devices but leaves audience, TTL, scope and cross-context aggregation open (`sync-presence-status-model.md:14,22,28–31`; `open-questions.md:74`). Two otherwise unlinkable private/corporate or anonymous contexts can be correlated by synchronized count changes even without exposing raw device IDs. A removed participant might continue polling count after the AD-16 shared-read lock, learning member activity. AD-3 forbids observable co-location of otherwise unlinked contexts (`ARCHITECTURE-SPINE.md:108`), and AD-16 gates shared reads (`:188`); neither states whether presence metadata uses that gate. Do not release a Space/device count by default while `OQ-0064` is open. Require audience and per-Space privacy policy, freshness/partition labeling, revocation/read-lock behavior and count-differencing negative tests.

### H3 — “No known conflict” is not proof an irreversible effect is safe [production gate]

Partitioned peer A has locally authority-confirmed `Done`, knows no conflict and sends a client email. Peer B later brings a valid incompatible accepted branch, reopening conflict under AD-23. The email cannot be rolled back. AD-24/`REQ-EFFECT-008` explicitly admit unknown late branches and defer sufficient proof/compensation (`ARCHITECTURE-SPINE.md:236`; `effect-execution-constraints.md:27`), while the gate blocks production execution until `OQ-0033/0045` (`ARCHITECTURE-SPINE.md:273`). Keep that gate hard: define what confirmation/causal cut entitles an effect to fire, whether residual late-branch risk is accepted, a logical effect idempotency key and compensation/notification semantics. “No known conflict” alone is not a safety proof.

### H4 — Equivalent visible outcome need not mean one logical effect or head [convergence gate]

Two offline resolutions choose the same task status; one also schedules a customer email, or both schedule the same email under different operation IDs. AD-23 only coalesces when canonical business outcome **and effects** are provably identical, retaining both audit operations (`ARCHITECTURE-SPINE.md:230`; `REQ-SYNC-010`, `transport-sync-requirements.md:53`). What counts as the same effect is not pinned: payload equality, intended logical effect ID and third-party execution receipt can differ. A client that coalesces by status alone loses an effect; one that executes by operation ID sends twice. Also, one visible result must retain the **union of both causal heads** in subsequent bases/snapshots, not silently choose a representative. Require versioned canonical outcome/effect-intent equivalence, effect dedupe identity and replay/frontier fixtures under `OQ-0034/0045`; unknown equivalence must stay unresolved.

### H5 — Safe blob-GC frontier may not advance under unbounded offline branches [data-loss/retention gate]

AD-22/23 require retaining old file payloads until a safe frontier (`ARCHITECTURE-SPINE.md:224,230`), while `BlobStore` still says pins last “until authorized conflict resolution” in one acceptance criterion (`blob-store-contract.md:56`) and says local resolution is insufficient in the next (`:57–58`). With a peer offline indefinitely, a previously accepted incompatible branch can arrive long after local resolution; early GC loses a variant, but never collecting every possible branch makes storage unbounded. The warning alone does not settle quota/exhaustion behavior. Align the shorter BlobStore criterion with the safe-frontier rule; `OQ-0034/0036` must define evidence for collectability, unavailable-payload UX, quota failure without silent loss and replay after snapshot/prune. Test local resolution → GC pressure → late accepted branch → restart/rekey.

### M2 — Agent opt-in scope is ambiguous; stale delegation can become authority [security gate]

The user decision authorizes an agent after explicit permission for a **type of action** (`decision-batch-device-core-2026-09-20.md:47`), while AD-25 and the implementation gate say “per-action opt-in” (`ARCHITECTURE-SPINE.md:242,273`). That may mean each operation or a durable permission for an entire action class. A broad stored opt-in, old read grant or changed variant set could let an agent resolve a different conflict later. AD-25 does require current read/action checks, but principal, scope, expiry, revocation and execution limits are `OQ-0045`. Define the consent tuple and authority-time recheck, including Space/resource/action, conflict heads, delegator, expiry and audit; test revoked grant and prompt-supplied fake authorization. Do not infer auto-resolution from package conformance or an AI suggestion.

### H6 — “Effective immutable rule version” lacks an authority binding point [mixed-version gate]

Phone with AppInstance rule v1 and laptop with v2 both receive source fact F. Each can call its locally installed version “effective” and derive different resources; deriving ID from F alone yields one ID with divergent content, while including version yields two resources. AD-13 and `REQ-EFFECT-010` demand an immutable source-fact/version bond and one logical resource (`ARCHITECTURE-SPINE.md:170`; `effect-execution-constraints.md:29`), but binding point, canonical ID, old-version availability and executor are deferred and explicitly blocked by `OQ-0048` (`ARCHITECTURE-SPINE.md:274`; `open-questions.md:58`). The proof needs a content-addressed/immutable rule identity at an authoritative cut, deterministic replay after upgrade/rollback, and fail-closed handling when the pinned rule is unavailable or revoked for security. A mutable package version string is not sufficient proof.

## Handoff

Fix H1 text now; leave production gates closed for H2–H6. Conformance should compare two independent peers across partitions, reordered control/data, late branches, quota/GC, read-lock/revocation, agent consent expiry, effect retries and mixed rule versions—not merely one reference implementation replaying itself.
