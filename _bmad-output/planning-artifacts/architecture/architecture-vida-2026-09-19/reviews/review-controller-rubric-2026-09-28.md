# Controller-history slice: good-spine review (2026-09-28)

**Scope:** AD-37 and AD-40 only, checked against `SPEC-VIDA-PERSONA-RECOVERY` and the BMad good-spine checklist. Source documents were read only. This is a semantic slice review, not a whole-spine lint or implementation audit.

**Verdict:** Conditionally passes. AD-40 fixes the principal cross-client divergence: canonical signed causal history, derived `ControllerState`, equal Device peers, preserved incompatible branches, and signed reconciliation before a disputed new grant gains shared access. AD-37 still contains one stale deferral, and the approved rotation split is not an enforceable shared rule.

## Findings

1. **High — Approved recovery behavior is presented as undecided.** AD-37's last sentence says “concurrent restore, rotation ... remain OQ-0024” ([spine, line 330](../ARCHITECTURE-SPINE.md)). The SPEC already approves two-restore branch preservation and convergence, no clock/Device winner, a nonblocking ordinary rotation, and urgent old-kit revocation on suspected compromise ([SPEC, lines 46–48](../../../../specs/spec-vida-persona-recovery/SPEC.md)). AD-40 covers much of the concurrent-history behavior, but the broad AD-37 deferral lets implementers treat the product behavior itself as optional. **Disposition: autofix** — narrow the deferral to signer/current-frontier proof, cryptographic rotation mechanism, wire format, crash safety and fixtures; retain the approved behavior as binding.

2. **High — Kit rotation has no common behavioral rule in this slice.** AD-40 binds “key rotation” but specifies no ordinary-versus-compromise policy ([spine, lines 346–348](../ARCHITECTURE-SPINE.md)). The SPEC requires a new kit and persistent save prompt without blocking ordinary edits; suspected exposure of both parts requires immediate old-kit revocation and Device renegotiation ([SPEC, lines 33, 48](../../../../specs/spec-vida-persona-recovery/SPEC.md)). Separate recovery clients could choose incompatible editing gates and compromise responses. **Disposition: autofix** — add these behavior invariants to AD-37 or AD-40; leave only the effective cut/proof and crypto mechanics under `OQ-0022/0024`.

3. **Medium — The recovery signer-to-history seam is still implicit.** AD-37 allows a user-held recovery authority to issue a provisional local grant after all Devices are lost ([spine, line 330](../ARCHITECTURE-SPINE.md)); AD-40 says each Device keeps signed event sequences, but does not state how a recovery-authority-signed transition enters that canonical history ([spine, line 348](../ARCHITECTURE-SPINE.md)). The SPEC leaves the exact signer/proof open ([SPEC, lines 47, 61](../../../../specs/spec-vida-persona-recovery/SPEC.md)). **Disposition: defer with explicit gate** — state that provisional recovery transitions are retained as signed causal events, while verification/acceptance semantics stay `OQ-0022/0024`; do not let implementations invent a privileged Device or mutable controller document.

## Rubric coverage

- **Pass:** AD-40's Rule is enforceable at the intended altitude and prevents its stated divergence; its undecided proof and format are explicitly gated.
- **Pass:** AD-37 retains separate identity/keys, owner-held two-part recovery, provisional restore, bootstrap finality and privacy boundaries from CAP-1–CAP-4.
- **Bounded gap:** CAP-3's separate authority/data-key/content result and CAP-5's optional OS/cloud backup behavior remain in the SPEC; this slice does not give those UX outcomes a shared conformance rule. If separate client teams own restore UX, add them to the AD-37 fixture gate rather than inventing a storage provider here.
- **Not assessed:** whole-spine dimensions, brownfield ratification, current named technology, mechanical lint and unrelated ADs.
