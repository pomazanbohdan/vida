# Adversarial review — online-first/update batch (2026-09-21)

Verdict: PASS after two boundary fixes; unresolved mechanics remain explicit gates.

## Attacks tried

1. Android and iOS teams both obey “online-first” but one claims a suspended process is online forever. AD-28 + OQ-0064/0067 prevent this: public presence still requires fresh application proof and platform suspension semantics.
2. Two Messenger clients receive concurrent publications in opposite order. AD-27 requires causal/topological order and the same canonical operation-ID tie-break. Autofix: OQ-0065 now also depends on OQ-0028 canonical ID bytes.
3. A legacy record lacks a newly required field. One client invents a default; another refuses to load. Autofix: read/edit projection may expose a non-persistable `missing-required` state, and save is blocked until a value is supplied.
4. A bot updates current task state while a historical conflict exists. REQ-EFFECT-009 distinguishes an ordinary authorized update from a resolution operation that covers conflicting heads.
5. A second user “takes over” a correction and changes attribution. REQ-EFFECT-011 forbids handoff-by-relabeling but permits an ordinary new update under normal rights.
6. A malicious repository serves an old correctly signed package. AppPackage baseline requires version/expiry and rollback/freeze checks before activation; exact trust roles remain OQ-0043.
7. iOS downloads a Wasm/Rhai package that introduces new functionality. The release profile blocks code-bearing tier absent platform-policy proof.

No critical/high incompatibility remains in the updated slice.
