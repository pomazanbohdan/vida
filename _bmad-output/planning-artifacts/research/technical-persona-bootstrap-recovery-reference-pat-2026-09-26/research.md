---
title: 'Technical research: Persona bootstrap and recovery reference patterns'
type: technical
topic: 'Interrupted identity setup, key custody, local persistence and recovery in messaging/local-first apps'
decision: 'How VIDA should handle an unfinished autonomous Persona and what must be proven before Story 1.1 is release-ready'
source: 'Official project documentation, code and security standards checked 2026-09-26'
status: complete
preset: standard
validation: normal
verified_claims: 11
unverified_claims: 0
created: '2026-09-26'
updated: '2026-09-26'
---

# Technical research: Persona bootstrap and recovery reference patterns

**Decision:** Product owner confirmed on 2026-09-26 that recovery material must be generated and confirmed before onboarding is complete. Reopen the *same* durably staged Persona after an interrupted setup; do not silently create a replacement. Protected Notes remain blocked until confirmation. This is VIDA product policy, **not** a requirement shown by the reference apps or OWASP. No implementation or crash-safety tests have yet been verified.

## Executive summary

Delta Chat creates a profile on first installation and offers backup export or second-device transfer later [1][2]. Signal's recovery-key confirmation is part of an optional backup setup after account creation [3]. SimpleX separates encrypted local use from later export [4]. Element can create a recovery key after a verified login [5]. Thus these references separate **identity exists**, **local storage works**, **backup is configured**, and **data is recoverable**. They do not establish the strict VIDA gate, nor document their exact app-kill behavior mid-prompt. Confidence: high for documented flows; low for any claim about interrupted-flow internals.

For VIDA's offline autonomous Persona, use a durable, idempotent bootstrap transaction and a clear `setup_pending` state. Android Keystore protects a device-bound key [6]; it is not a portable recovery copy. Android Auto Backup is conditional and can be absent [7]. OWASP requires deliberate key management and secure storage but does not prescribe this onboarding gate [9][10]. A recovery secret can restore authority only if recoverable encrypted data actually exists elsewhere. Confidence: high for platform/standard constraints, medium for the proposed VIDA transaction design.

## Reference-project findings

| Reference | Documented sequence | VIDA takeaway |
|---|---|---|
| Delta Chat | First profile created at install; QR second-device transfer and manual backup export are separate, later actions [1][2]. | An account need not wait for a backup wizard. Delta's current ordinary messages use chatmail relays [1]; borrow the interaction pattern, not its transport. |
| Signal | Secure Backups are opt-in in Settings, with record-then-confirm recovery key; no key means no backup restore [3]. | Confirmation proves the user can reproduce a key at that moment, not that an independent backup exists. |
| SimpleX | Local database has a random default passphrase; a user-set passphrase is required for export/system backup [4]. | Local encryption, exportability and off-device survivability are separate facts. |
| Element | Element X/Pro require device verification on login; recovery key can still be obtained afterwards. Web verification may be skipped [5]. | Device trust and recovery-key readiness are distinct gates. |

Detailed source notes: [reference-apps.md](digests/reference-apps.md).

## Security and durability findings

- Android Keystore key material can remain non-exportable [6]. Do not make the device-bound wrapping key the only recovery path. The recovery design must specify what secret authorizes a new device, what encrypted bytes it unlocks, and what a lost key means. OWASP calls for explicit generation, storage, backup and recovery lifecycle [9].
- Android Auto Backup may not run, has a 25 MB per-app limit, and by default includes databases [7]. It must not be used as proof of a working self-held recovery path or allowed to copy encrypted records without an independently usable recovery secret.
- SQLite supports atomic commit [8]. VIDA should commit Persona ID, Space ID, Owner grant, key envelope and pending state together (or prove an equivalent atomic protocol). This is an implementation recommendation, not proof that VIDA has done so.
- For static Web, `StorageManager.persist()` may be refused and even persistent storage can be cleared explicitly [11]. Local browser storage alone is not a guaranteed second copy.
- OWASP MASVS-STORAGE-1 requires secure handling of sensitive local data [10]; neither OWASP source prescribes whether Notes must be blocked while recovery setup is pending.

Detailed source notes: [storage-security.md](digests/storage-security.md).

## Cross-dimension insight and contrary evidence

Strictly blocking Notes until recovery confirmation minimizes the period in which the user can create irreplaceable data without having acknowledged the secret. The cost is onboarding friction, and all four reference flows demonstrate that account or local-data use can be separated from recovery setup [1][3][4][5]. Therefore the strict gate must be described as VIDA's conscious guarantee, not “how Delta Chat does it.” A progressive alternative would allow local Notes in a visibly at-risk pending profile, but would require a deliberate revision to Story 1.1–1.3 and the current architecture/PRD wording; it cannot be adopted as a hidden implementation detail.

## Recommendations and downstream bindings

1. **Apply the approved VIDA gate:** on restart, resume one stable `setup_pending` Persona and recovery prompt; no `PersonaCreated` or protected Notes until durable confirmation. Source basis: product-owner approval on 2026-09-26 and VIDA's mandatory-confirmation requirement in `epics.md` Story 1.1/1.2 and the PRD, not an external standard. Confidence: high as project policy.
2. **Prove Story 1.1 durability:** crash before/after bootstrap commit, app kill during recovery prompt, disk-full, corrupted envelope, repeated Start/Resume; exactly one Persona and Space, no regenerated secret, no success claim before durable commit. Source basis: SQLite atomicity [8] and OWASP key lifecycle [9]. Confidence: medium pending implementation tests.
3. **Specify key custody by platform:** device-bound protection on Android [6], independently portable recovery material, explicit Auto Backup rules [7], browser eviction handling [11]. A completed recovery-screen confirmation must not be labelled “backup complete” unless a recoverable encrypted copy is also verified. Confidence: high for constraints, medium for VIDA design.
4. **Separate release proof:** Story 1.1 can prototype now but cannot be marked release-ready until key-envelope format, atomic persistence and restore tests pass. Story 1.2 needs a tested confirmation/resume flow; later recovery proof must show authority restoration with actual encrypted data availability. Confidence: medium pending tests.

## Resolved product question

Yes. On 2026-09-26 the product owner selected the stricter gate: no Notes or other protected Personal Space work before recovery confirmation; restart resumes the same pending Persona. This decision changes the status of the provisional Story 1.1/1.2 refinement to approved. The external sources informed the tradeoff but did not prescribe the answer.

## Sources

| Ref | Supports | Publisher / URL | Published | Accessed | Confidence |
|---|---|---|---|---|---|
| [1] | Delta profile creation, relay architecture | [Delta Chat FAQ](https://delta.chat/en/help) | n.d. | 2026-09-26 | high, primary |
| [2] | Delta multi-device/manual transfer | [Delta Chat FAQ](https://delta.chat/en/help) | n.d. | 2026-09-26 | high, primary |
| [3] | Opt-in backup and key confirmation | [Signal Support](https://support.signal.org/hc/en-us/articles/9708267671322-Signal-Secure-Backups) | n.d. | 2026-09-26 | high, primary |
| [4] | Default passphrase and export | [SimpleX Chat guide](https://simplex.chat/docs/guide/managing-data.html) | n.d. | 2026-09-26 | high, primary |
| [5] | Verification versus recovery key | [Element docs](https://docs.element.io/latest/element-support/device-verification/how-to-ensure-you-have-a-recovery-key/) | n.d. | 2026-09-26 | high, primary |
| [6] | Android key non-exportability | [Android Developers](https://developer.android.com/privacy-and-security/keystore) | n.d. | 2026-09-26 | high, primary |
| [7] | Android backup conditions and scope | [Android Developers](https://developer.android.com/identity/data/autobackup) | n.d. | 2026-09-26 | high, primary |
| [8] | Atomic transaction behavior | [SQLite](https://www.sqlite.org/atomiccommit.html) | n.d. | 2026-09-26 | high, primary |
| [9] | Key lifecycle and recoverability | [OWASP Key Management Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html) | n.d. | 2026-09-26 | high, primary |
| [10] | Sensitive local storage | [OWASP MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) | n.d. | 2026-09-26 | high, primary |
| [11] | Browser persistence is conditional | [MDN StorageManager.persist](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persist) | 2024-07-26 | 2026-09-26 | high, primary |

## Staleness map

The workflow staleness calculator uses 2026-09-26 as the **last-checked proxy** in `staleness-input.json`; it is **not** a claim that undated sources were published then. With 3-month windows for app/platform behavior and 12-month windows for storage/security principles, its earliest routine recheck is **2026-12-26**. Recheck sooner immediately before Story 1.1 implementation and release testing. This report is evidence current to 2026-09-26, not a test report for VIDA.
