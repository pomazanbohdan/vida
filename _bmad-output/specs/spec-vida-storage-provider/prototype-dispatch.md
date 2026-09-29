---
id: VIDA-STORAGE-PROTOTYPE-DISPATCH
status: planned-not-executed
updated: '2026-09-29'
---

# Epic 1–2: наступна робота замість повторного голосування

Derived from the storage memlog and approved behavior. This is engineering dispatch, not new epic numbering or proof that a story is implemented. Production promotion remains gated; candidate port names/provider profiles in [provider-contract-v1.md](provider-contract-v1.md) are reviewable prototype inputs.

## Current verified evidence

- `cargo test --workspace --all-targets`: 7 passed on 2026-09-29.
- `target/rec-case-matrix.json`: 14 passed, 16 unproven, 0 failed. REC-F01/F08 explicitly lack persistent bootstrap/restart adapter.
- Core provides `StateStore`/`MemoryStore`; no physical adapter or platform key-custody proof exists.
- Scoped source search found no `pubspec.yaml`, Dart source, Android manifest, APK or AAB. No Android/Web build is claimed.
- Closing/reopening an in-memory object is not a fresh-process restart test. The following SPV fixtures are **planned**, not included in the 7 passing tests.

## Work packages and exit evidence

| Order | Work | Story / existing fixtures | Exit evidence |
|---|---|---|---|
| P1 | Implement neutral persistent-adapter harness outside Core, using existing signed controller/recovery golden bytes; add native candidate adapter and redacted config capture | 1.1/1.2; REC-F01/F08, STO-01/02/13/20 | distinct child-process writer/reopen runner; before/after-cut authoritative state; atomic pending/active Persona under same IDs |
| P2 | Prove native encrypted commit, unknown-result lookup, key wrapping and independent recovery export | 1.1–1.4; STO-07/11/12/15/17 | measured DB/custody tuple; no plaintext artifact; fresh-profile decrypt/readback; wrong/tampered/stale inputs fail; H/I/P tiers distinguished |
| P3 | Implement Chromium provider and local writer coordination using the same semantic fixture data | 2.1/2.11; STO-01/02/13/16, BND-W02, D4–D6 | tab crash/offline reopen, quota, persistence denial, two-tab takeover/stale-resume; equal logical state/IDs; key-custody limitations recorded |
| P4 | Introduce shared facade + minimal Android/Web shells around one Rust domain implementation | 1.3/2.1; BND-W01/02 | create Persona and one Note, restart, reopen under same IDs; one artifact tuple per platform; build APK and static Web assets |
| P5 | Send that persisted Note between enrolled equal Devices; prove direct payload separately from signaling | 2.2; D9 | Android↔Web release-build route trace and authorized application receipt; forced relay prohibition proves no relay payload in direct run |
| P6 | Add explicit fallback/retry and three-peer repair, then conflict and Tor fixtures using their reviewed contracts | 2.3–2.15 | no false receipt, no duplicate effect, convergence/conflict proof, Tor fail-closed matrix; not implied by P1–P5 |

Direct Android↔Web feasibility is a parallel high-risk investigation from P1, **not** postponed until all UI is built. P5 nevertheless depends on validated enrollment/wire parity and P4 artifacts. Failure of the direct proof returns to transport design; it never silently changes the requirement to relay-only. P6 does not authorize inventing unresolved authority/merge/Tor contracts.

Generic Note wire, enrollment/receipt bytes, custody crypto profile and bridge tuple are independent contract gates. P1 may use existing controller wire without waiting for generic Note wire; P4/P5 cannot. Finish these engineering contracts before production code that depends on them. Already-approved product choices are not put back to a vote unless implementation evidence exposes a real trade-off or contradiction.

## First executable fixture backlog

Every runner records exact source/build/provider/browser/OS tuple, fixture seed, cut, effective settings and redacted durable observations. Run on isolated synthetic vaults; never kill the user's running app or fault their real data. A test file's presence is not execution evidence.

| ID | Cut / setup | Oracle | Maps to |
|---|---|---|---|
| SPV-01 | child writer dies before authoritative commit | fresh reader sees previous checkpoint, no partial action or saved result | STO-01/13 |
| SPV-02 | child writer commits then dies before callback | fresh reader resolves original ID to one full operation/outbox/dedupe/checkpoint; same-ID retry adds none | STO-02/13, D6 |
| SPV-03 | quota/IO failure during transaction, with ambiguous commit variant | recover to required checkpoint before absence assertion; result remains unknown until lookup; no fresh-ID retry | STO-13 |
| SPV-04 | staged Persona bootstrap dies at recovery/activation cuts | same Persona/Space/key identities reopen as complete pending or complete active, no usable partial grant | STO-20, REC-F01/F08 |
| SPV-05 | two writers contend; leader suspends, closes/dies, then stale leader resumes | exactly one effective writer; old-generation write denied; one Device and original outbox survive | STO-16, BND-16, D5 |
| SPV-06 | browser persistence denied, then tab closed/reopened; separately clear synthetic origin | denial does not block saves; reopen preserves state when bytes exist; clearing gives truthful restore/enrollment path, no invented replication | D4, BND-W02 |
| SPV-07 | wrong key/tampered ciphertext; inspect DB/WAL/temp/search/log/backup artifacts | safe typed failure and no unauthorized plaintext; no automatic empty-vault reset | STO-15 |
| SPV-08 | backup interrupted and fresh profile restored independently | last verified backup retained; IDs/frontier/pending jobs preserved within actual backup coverage; OS wrapper not assumed portable | STO-07/12/17 |

H = deterministic headless faults/process death; I = installed platform lifecycle; P = physical OS/power interruption. H does not close I/P. No test here proves unseen latest controller rights, global ordering, remote erasure or absence of host compromise.

## Completion labels

- `prototype-input-complete`: port semantics + seeded fixtures + recorded candidate assumptions; permits bounded implementation.
- `prototype-proven`: raw executions for exact tuples pass applicable SPV/STO/REC/BND subset; list omissions.
- `story-ready-for-production`: all story AC, dependency contracts and platform/security proof are satisfied; assessed per story, not by blanket approval of this file.
- `implemented/done`: working code, tests and review evidence; never set from a planning document alone.

Current status: prototype dispatch documented; provider adapters and SPV runs absent. Epic 1–2 remain production-gated. README files untouched.
