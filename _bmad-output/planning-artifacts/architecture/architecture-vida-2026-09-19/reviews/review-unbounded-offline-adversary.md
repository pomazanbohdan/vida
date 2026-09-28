---
review: unbounded-offline-adversary
artifact: ../ARCHITECTURE-SPINE.md
focus: AD-18, ADR-0003, REQ-ACL-018, SyncLog
date: 2026-09-20
verdict: accepted-direction-with-blocked-implementation-seams
---

# Adversarial review — unbounded offline mutation candidates

## Verdict

AD-18 consistently expresses the accepted product rule: a locally durable offline mutation candidate has no age-only expiry, but acceptance rechecks current authority, rights and operation preconditions. It does **not** promise eventual shared acceptance, indefinite credential validity or remote erasure. The spine has an explicit `OQ-0033`/`OQ-0034` implementation gate for the central incompatibilities. Do not claim an interoperable acceptance/recovery policy until that gate closes. One stale sentence in ADR-0003 can be read as reinstating a risk-class maximum and should be corrected now.

## Scenario matrix

| Adversarial case | Accepted invariant | Remaining seam |
|---|---|
| Six-month offline edit; membership/grant/key epoch and object preconditions still valid | Age alone never rejects or quarantines. Authority evaluates at reconnect; delivery/local durability alone are not acceptance. | What exact success/failure predicate forces acceptance rather than discretionary `MAY`? Define in `OQ-0033`. |
| Six-month edit; original short-lived grant expired, but actor currently has equivalent rights via a new grant | Candidate and its content remain recoverable; old grant is not an unlimited credential. | Old signed `grantId` fails. Reauthorize/re-sign/rebase, new versus stable operation ID, audit link and dedupe need `OQ-0033`/`OQ-0034`. |
| Actor effectively revoked before reconnect | Current authority rejects old candidate regardless of client timestamp; old key/grant cannot authorize future write. Quarantine cannot expose former protected Space content to removed member. | Separate recoverable storage from user-visible recovery actions after full removal; define lawful export/reapply surface. |
| Owner works six months in own Personal Space | Owner may locally accept under own Space policy without server/member approval; read remains offline by default. | With multiple devices or device revocation, which local control head/device grant makes one acceptance final? Define in `OQ-0033`. |
| Offline task `status=Done`; another accepted writer sets `status=Cancelled` | Candidate age cannot choose a winner; both must apply same conflict policy and accepted frontier. | Transition/merge/conflict fixtures remain `OQ-0034`; no current interoperable result. |
| Membership unchanged but affected scope key epoch rotated | Old epoch cannot encrypt a new accepted operation. Candidate stays recoverable. | Re-encryption/signature preimage and identity/dedupe semantics are unresolved; old envelope cannot simply be replayed. |
| Client A retains an old candidate and offers rebase; client B marks it ineligible because its old grant/epoch cannot validate | Both can obey “no age-only rejection” yet show different states and converge differently if A silently rewrites it. | Require one versioned pending→revalidated→accepted/rejected/rebased state machine and conformance fixtures before independent implementation. |

## Critical/high findings

### C1 — ADR-0003 retains an obsolete maximum-lifetime sentence

**Evidence:** ADR-0003 §7 now explicitly removes `standard` 30 days / `protected` 7 days / `critical` 24 hours as age limits for offline candidates. Immediately afterward it still says, “Точний maximum lifetime визначається risk class scope; довгоживучі безстрокові write grants MUST NOT бути default.” `REQ-ACL-018`, AD-18 and `OQ-0019`/`OQ-0054` say candidate age alone never expires.

**Adversarial implementation:** client A reads “maximum lifetime” as applying to a pending operation and rejects the six-month edit. Client B reads it as applying only to write-grant lifetime and preserves the candidate. Both can cite the same accepted ADR; A defeats AD-18.

**Disposition — autofix:** explicitly label this sentence as **credential/grant lifetime only**, not offline-candidate lifetime; preferably move it to a grant-policy paragraph. No risk-class candidate timer may be inferred from it.

### H1 — A currently entitled actor with an expired embedded grant cannot reuse the old signed envelope as-is

**Evidence:** ADR-0003 §3 and `REQ-ACL-012` require `grantId`, `policyVersion`, `keyEpoch` and acceptance-time grant checks. AD-18 says grant expiry does not erase candidate and current rights are rechecked. `SyncLog` says re-sign/rebase is open. AD-4 expects a stable operation ID/envelope across direct and durable paths.

**Adversarial implementation:** after six months, authority grants the same user access again under new `grantId` and epoch. Client A changes grant/epoch and keeps operation ID; client B considers this a signature/identity violation or has already deduped the old ID, so rejects. Both comply with the current high-level rules yet cannot agree on the accepted operation.

**Disposition — gated:** `OQ-0033`/`OQ-0034` must define whether the candidate becomes a new signed operation linked to an immutable original intent, or an allowed re-signed revision of one logical operation; signature domain, authority receipt, dedupe, causal dependencies and audit history must be testable. No automatic regrant of an expired credential.

### H2 — Personal Space “local acceptance” is underdefined after another device rotates or revokes a device

**Evidence:** AD-18 and ADR-0003 §7 allow Owner local acceptance without server approval. ADR-0003 §§2–5 require current control head, device/grant validation and new key epoch after effective revocation. `SyncLog` distinguishes origin-durable from authority-accepted frontiers.

**Adversarial implementation:** Owner's laptop works offline for six months and marks notes accepted locally. On a second Owner device, the laptop's DeviceGrant was revoked and keys rotated. Laptop later syncs with old epoch. It cannot override the later control transition, but its UI/backup may already have treated its local frontier as final. Another client kept those notes tentative. Their state, receipts and user expectations diverge.

**Disposition — gated:** `OQ-0033` must specify Personal Space authority/control-head scope for multi-device offline work, when local acceptance is provisional relative to unseen control transitions, and what happens to previously labelled accepted work after conflict. Define receipt/frontier status so UI never equates origin durability with global acceptance.

### H3 — Retention promise needs explicit local storage/recovery failure behavior

**Evidence:** AD-18 says a candidate “can remain pending indefinitely” and age alone never hides it; ADR-0003 §4 requires rejected work remain recoverable. Neither defines behavior under quota, database compaction, key loss, logout, uninstall, backup restore or user-requested deletion. Durable-delivery TTL/quota is a separate ciphertext mailbox policy and cannot safely stand in for local candidate retention.

**Adversarial implementation:** client A preserves six-month local edits across compaction and backup restore; client B evicts oldest pending edits once a quota is reached and reports “storage pressure,” not “age.” Both avoid an explicit age-only TTL, but B violates the user-visible no-loss expectation implicit in AD-18's `Prevents` clause.

**Disposition — specify/gate:** define local candidate retention, encrypted backup/export, quota-pressure handling, explicit user-consented deletion, integrity failure, and visible unrecoverable-loss diagnostics. In particular, mailbox TTL expiry must not delete the sole local candidate/outbox copy. Reuse `OQ-0036` storage-provider conformance and add six-month/quota/crash/restore fixtures.

### H4 — Conflict resolution cannot be replaced by “current rights still valid”

**Evidence:** AD-18 rechecks schema, causal and business preconditions; `SyncLog` requires deterministic convergence but defers operation-family transitions and snapshots to `OQ-0034`.

**Adversarial implementation:** two valid writers change one task's status while one is offline. Client A accepts the late `Done` as last-arriving; client B rejects it because `Cancelled` is terminal. Neither uses candidate age as a rejection rule, yet shared state diverges.

**Disposition — correctly gated:** no independent acceptance claim until `OQ-0034` publishes operation-family transitions and conflict fixtures. If a valid but conflicting candidate is not applied, it remains recoverable with a conflict reason, not silently discarded or mislabelled as revoked.

## Contained/in-scope distinctions

- Revoked user: the rejection is mandatory and **not** an age-only rejection. A timestamp preceding revocation cannot rescue the operation.
- Key epoch: old ciphertext may remain in local candidate storage, but new accepted operations/snapshots/attachments cannot use the old epoch key.
- Personal Space: indefinite local work is accepted as a product direction; it does not mean a disconnected, later-revoked device can forge current authority.
- Offline read: `OQ-0051`/`OQ-0053` concern reconciliation timing and read behavior, not candidate mutation age. Do not reintroduce `standard/protected/critical` read or mutation timers by inference.
- Membership/role/policy/key changes may be staged offline but never take effect before applicable authority acceptance.

## Closure gate

Fix C1 in ADR-0003 now. Preserve `OQ-0033`/`OQ-0034` as blocking gates for revalidation, re-sign/rebase, Personal Space multi-device authority and conflicts; extend `OQ-0036` with local long-lived candidate retention/recovery evidence. A conformant release should test 6-month candidate + unchanged rights, expired grant with renewed rights, effective revocation, old epoch, Personal Space device revocation, concurrent status conflict, quota/crash/backup restore, and two independent client implementations against the same expected frontiers and outcomes.
