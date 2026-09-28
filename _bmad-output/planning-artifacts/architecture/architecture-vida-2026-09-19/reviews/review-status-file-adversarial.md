---
review: status-file-adversarial
artifact: ../ARCHITECTURE-SPINE.md
focus: AD-22, ADR-0017, REQ-SYNC-004/007/008, SyncLog, BlobStore
date: 2026-09-20
verdict: product-choice-sound-two-predicate-ambiguities-fix-before-lock
---

# Adversarial review — status/file multi-value conflict

## Verdict

AD-22 correctly rules out a hidden CRDT/device winner for genuinely incomparable, authority-valid status and file alternatives. Its current predicate risks **two different overclaims**: source-operation concurrency is not the same as incomparable *authority acceptance*, and a locally durable alternative is not necessarily authorized/accepted. Those should be clarified in the decision text before architecture lock. Resolution races, blob lifecycle and provisional UX are acknowledged but need explicit closure fixtures under `OQ-0033/0034` and `OQ-0029/0036`; current gates correctly bar independent production implementation. Evidence: spine lines 204, 216, 222, 246–252; ADR-0017 lines 24–27, 32.

## Counterexamples

### H1 — Concurrent authorship, comparable acceptance: conflict versus first winner [fix wording now]

Laptop and phone both author `Open@R → Done` and `Open@R → Cancelled` offline, so the **operations** remain causally incomparable. A shared authority then verifiably accepts `Done` at sequence 10 and evaluates `Cancelled` at sequence 11. AD-19 / `REQ-SYNC-004` give the first accepted value precedence and retain the stale intent without silent overwrite (spine line 204; requirements line 45). AD-22 / ADR-0017 can be read to mandate an unresolved multi-value conflict whenever the *changes* are causally incomparable (spine line 222; ADR line 24), despite the comparable acceptance evidence. The documents' predicates pull an implementer toward different projections from the same inputs.

**Required clarification:** distinguish source-operation causality from authority-acceptance order. State explicitly that a verifiable shared acceptance order invokes AD-19 even when the original proposals were authored concurrently; AD-22's no-winner rule applies only to incompatible alternatives whose valid acceptance histories remain incomparable. Add a 2×2 fixture (concurrent/sequential authorship × comparable/incomparable acceptance) for status and file replacement.

### H2 — Revoked or merely local-durable variant leaks into a shared conflict [fix wording now]

Peer A durably stages file replacement `F1`; before authority acceptance, its membership/device grant is removed and the file key epoch advances. Peer B's `F2` is accepted. A later sends its signed, causally incomparable `F1` envelope. If “both **durable** alternatives” (AD-22 line 222) or “both variants durable/recoverable” (ADR-0017 line 25) is treated as sufficient, one peer publishes `F1` as the second conflict variant and may fetch/display its blob; another rejects it as a recoverable private candidate. AD-18 and `SyncLog` explicitly require current-rights/epoch validation and keep invalid candidates out of accepted projections (spine line 198; `sync-log-contract.md` lines 42–44). Mere signature, delivery or origin commit cannot qualify the shared conflict.

**Required clarification:** the shared multi-value set contains only alternatives with verifiable authority-valid acceptance evidence evaluated against the agreed control history; rejected/pending candidates remain durable for authorized recovery but are not shared accepted variants. A prior *provable* pre-revocation acceptance is different from an alleged old client timestamp. Test member and single-DeviceGrant revocation, old epoch, late delivery, and post-removal UI/API/blob denial.

### H3 — Two resolutions, or a late third branch, can falsely close the conflict [gated]

After conflict `{A,B}`, two authorized actors independently issue `R1(A,B)` and `R2(A,B)` in partition. Each resolution references both original alternatives yet neither references the other. A third concurrent alternative `C` may also surface after `R1`. A peer that treats any operation citing `A,B` as final displays R1; another preserves an unresolved `{R1,R2}` or `{R1,C}` conflict. Both respect the simple “resolution operation causally based on both” sentence, but cannot converge on finality. ADR-0017 explicitly defers simultaneous resolution races (line 32), and `OQ-0034` retains them.

**Gate closure:** define a versioned conflict-set identity/frontier and authority-time precondition; specify how a resolution that omits a concurrent accepted branch is classified (stale, pending or a new conflict), with N-way and repeated-resolution fixtures. Do not make the first packet or a resolver's device the winner.

### H4 — Blob metadata converges while a payload disappears [gated]

Peer A persists a signed `replace → blob X` operation but crashes before its blob pin/reference becomes durable; GC collects X. Peer B persists Y and later reconciles both operations. Both peers derive `{X,Y}` as the unresolved file conflict, yet X is not retrievable for comparison. BlobStore currently requires both verified variants/pins until authorized resolution (`blob-store-contract.md` lines 47, 56), while the operation/outbox transaction does not explicitly include blob-pin lifecycle (spine line 114). Another failure: a provider unpins a losing branch on *local* acceptance or a premature resolution while an offline peer still needs it for replay. The phrase “both durable alternatives” must not imply that operation metadata alone proves payload durability.

**Gate closure:** require a recoverable operation↔manifest↔payload/pin invariant or explicit unavailable/pending-payload state; define publication/acceptance ordering, crash repair, GC frontier and retention after resolution. Test missing chunks, provider GC, restart and late peer repair. Revocation may block future fetch even while encrypted bytes remain; retention never overrides rights.

### M1 — Local success can masquerade as final status/file [gated]

During partition each peer may show its locally accepted branch as ordinary “Done” or the current file. After reunion AD-22 removes any canonical current value and shows conflict. A user may have acted on a success badge, shared a file link or triggered an external effect. The decision says “after peers reconcile” (spine line 222; ADR line 24), but gives no pre-reconciliation label or effect eligibility. `SyncLog.Accept` only means local validated/pending history, not `authority.accepted` (`sync-log-contract.md` line 42); the direct-session spec allows “saved locally,” not “accepted by all” (line 22). Preventing replay of a business command (ADR line 26) does not by itself prevent two independently committed, irreversible effects.

**Gate closure:** define UI and effect states for origin-durable, locally authority-accepted, comparable final, and conflict/unknown-global-finality; test partition/restart/reunion, no current winner after conflict, and external-effect idempotency/compensation policy. A former member must not see either variant after effective revocation.

## Disposition

- **Fix now:** align AD-22/ADR-0017's predicates with AD-19 and AD-18: acceptance comparability and authority-validity, not merely concurrent authorship or local durability, decide shared conflict membership.
- **Keep gates:** `OQ-0033/0034` for receipt/finality, N-way resolution and UX; `OQ-0029/0036` for payload/pin/GC crash safety. Cross-vendor fixtures must compare control evidence, accepted variant set, conflict state, blob availability and rights-filtered projections.
