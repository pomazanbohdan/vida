---
review: peer-equality-adversarial
artifact: ../ARCHITECTURE-SPINE.md
focus: AD-19, AD-21, ADR-0016, partition, revocation, Owner controls, provisional UX
date: 2026-09-20
verdict: invariant-sound-but-independent-implementation-gated
---

# Adversarial review — equal device peers and incomparable acceptance

## Verdict

**No device-master regression found.** AD-19/AD-21 and ADR-0016 correctly reject transport-first, clock-first and device-priority arbitration. They do **not yet define an interoperable acceptance/convergence function** for two disconnected, independently implemented peers. That is an acknowledged gap, not permission to ship: the spine explicitly blocks domain acceptance (`OQ-0033`), independent `SyncLog` (`OQ-0034`) and membership/control-plane implementation (`OQ-0031`). Thus the constructions below satisfy the adopted *principles* but neither may claim production conformance before those gates close. Evidence: spine lines 202, 214, 237, 241–243; ADR-0016 lines 24–27.

## Two peer implementations that diverge

### H1 — Same Persona, two offline devices, incompatible status [OQ-0033/0034]

Start with accepted task `Open@R` in an Owner's Personal Space. Laptop L signs `Done` from R; phone P signs `Cancelled` from R. They are disconnected. AD-18 permits local Personal-Space Owner-policy acceptance, while AD-19 says neither incomparable acceptance establishes a global first (spine lines 196, 202). On reunion, implementation M uses a multi-value register: no singular current status, both alternatives visible. Implementation W uses a device-neutral operation-digest tie-break: one displayed current status plus the other as a recoverable conflict. Both preserve signatures and alternatives, converge independent of delivery order *within their own implementation*, and never privilege L or P. Yet the same operation set yields different task state and UX across M/W. The session spec itself lists both as undecided candidates (`device-sync-session.md` lines 35–42). AD-21's “same schema-level rule” is therefore not executable across vendors until a versioned rule and fixtures close `OQ-0034`.

**Required gate proof:** canonical operation family/base identity, whether both local acceptances remain authority-accepted branches or are provisional, conflict-state shape, deterministic projection, explicit human resolution command, and replay permutations across two vendors. Do not quietly redefine local Personal acceptance as globally final.

### H2 — Partitioned Owners can remove each other without a common control order [OQ-0031/0033]

Shared Space has Owners A and B. Partition A signs removal of B; partition B signs removal of A. Each is a valid Owner action against its observed control head and needs no co-owner quorum. Peer M serializes incomparable control operations by ascending operation digest; peer W by descending digest. Neither uses device identity or connection order. Each then rejects the removal initiated by the Owner it removed first, preserving one Owner and applying atomic grant revocation/key-epoch advance. But M retains A and W retains B from the same signed inputs. The phrase “Space authority serializes” in AD-6 (spine line 124) and the equal-peer rule do not specify the order or authority proof. `space-membership-contract.md` lines 67–69 explicitly defer cross-removal ordering and block independent implementations. AD-21's “same valid operation set” is too weak by itself: peers disagree first on which signed control operation is *valid at acceptance*.

**Required gate proof:** common control-order/cut rule, receipts/proof and deterministic validity for both cross-removal orders; tests must verify identical surviving Owner, grants, epochs, audit trail and no ownerless intermediate/accepted state. A designated device cannot be the tiebreak.

### H3 — Concurrent revocation versus data edit changes the valid set [OQ-0031/0033]

Member B creates an offline task edit from control frontier C while Owner A removes B in another partition. No receipt proves the two operations comparable. Peer M orders the edit before the removal cut and includes it; peer W orders removal first and rejects the edit while retaining it as a recoverable candidate. Both can revalidate against their chosen *actual* authority-acceptance state, without timestamps or device rank, yet they derive different data history and post-revocation projections. The same issue arises when one DeviceGrant of a Persona is revoked while another device remains authorized: Persona membership alone must not rescue the revoked device's operation (ADR-0016 line 25; `space-membership-contract.md` lines 48–50). The causal cut, signed authority evidence and operation-family ordering must be shared, not inferred from local arrival.

**Required gate proof:** one verifiable rule for incomparable data/control proposals, including device-grant revocation, epoch rotation and offline revalidation. Assert that a merely origin-durable candidate has no pre-cut entitlement; only a demonstrably authority-accepted pre-cut operation survives. After effective removal, protected data and conflict UI remain inaccessible to the former member.

### M1 — Provisional UI can overstate local finality [OQ-0033/0034]

L and P in H1 may each truthfully know “saved locally”; AD-18 may also let each know “accepted by local Personal policy.” Neither knows a globally comparable first acceptance. One shell may show `Done` as final with an ordinary success check; another may label it pending/conflicted. AD-13 distinguishes tentative projections from accepted state (spine line 166), and the session spec separates local save from accepted-by-all (`device-sync-session.md` lines 22–29), but no normative UI state machine says how two *locally accepted but incomparable* variants are represented or relabeled after merge. The result is cross-platform behavioral divergence even if storage eventually converges.

**Required gate proof:** versioned `origin-durable` / `authority-accepted-local` / `authority-accepted-comparable` / `incomparable-conflict` semantics (or an equivalent minimal state model), receipt-to-label mapping, restart/partition/reunion transitions, and an explicit prohibition on presenting local ACK as global finality. Conflict recovery UI must enforce current rights.

## Disposition

- **Keep gates closed:** do not mark independent domain, `SyncLog` or membership implementations conformant based only on AD-19/AD-21.
- **Close `OQ-0031` first or jointly with `OQ-0033`:** control serialization and causal cut determine whose rights exist for data acceptance.
- **Close `OQ-0033/0034` with cross-vendor fixtures:** same Persona on two devices, two partitions, reordered delivery, cross-removal, grant revocation, conflicting status, restart and provisional UX; compare authority receipts, valid-set classification, canonical state, conflict variants and rights-filtered views.
