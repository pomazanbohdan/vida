# Adversarial review — Epic 1/2 approval batch (2026-09-29)

**Verdict:** Pass for documenting the ten approved product behaviors. This is not a production/security proof: controller reconciliation, rotation atomicity, recovery cryptography and independent-replica evidence remain explicit `OQ-0022/0024` and conformance gates.

## Rechecked findings

1. **Resolved — historical ciphertext versus revoked authority.** Retaining an older verified resource snapshot does not restore controller authority, but revocation cannot erase plaintext already obtained or make old ciphertext confidential if old data keys were compromised. This distinction is now explicit in `../../../../specs/spec-vida-persona-recovery/recovery-bundle-contract.md:23` and `../../../../specs/spec-vida-persona-recovery/recovery-cases.md:54`. No further product decision is needed.
2. **Resolved — automated restore scope.** `REC-F24` now names the release test suite as its trigger and expressly excludes mandatory restoration in each user's profile (`../../../../specs/spec-vida-persona-recovery/recovery-cases.md:50`). This matches the optional guided user test in `SPEC.md:66` and the release gate in `epics.md:362`.

## Remaining implementation gates, not new approval questions

- The frontier/data-key epoch check must be atomic with signed rotation commit and crash-safe repair; the documents correctly leave the proof in `OQ-0022/0024` (`ARCHITECTURE-SPINE.md:332`, `SPEC.md:72`).
- Same-host browser profiles do not satisfy the default independent-replica sync claim; privacy-preserving failure-domain evidence remains a conformance gate (`docs/04-specifications/operation-finality-contract.md:58`).
- Scoped grants for one logical Device/key compose only within their authorized Space scopes; canonical wire and merge proof remain open (`ARCHITECTURE-SPINE.md:354`, `docs/00-governance/open-questions.md:32`).
- Voluntary unlink warning must not delay urgent compromise revocation (`ARCHITECTURE-SPINE.md:225`, `epics.md:721`).
- Unknown mandatory recovery-bundle versions must be retained unchanged with no partial import (`SPEC.md:66`, `epics.md:364`). Exact import/version conformance remains part of `OQ-0024`.

**Scope:** read-only adversarial inspection of the updated approved documents; no code or cryptographic verification was performed.
