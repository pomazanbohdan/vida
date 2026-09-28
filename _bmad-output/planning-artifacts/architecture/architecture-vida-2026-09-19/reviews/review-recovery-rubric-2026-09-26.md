# Recovery rubric review — 2026-09-26

## Verdict

**Needs one high-priority reconciliation before handoff.** AD-37 fixes the approved two-part owner-held recovery boundary without choosing a cryptographic profile. Its production enrollment gate correctly leaves byte format, freshness, concurrent restore, rotation and restore evidence in OQ-0024. The linked SPEC and companions add an approved setup lifecycle invariant that the spine does not yet bind.

## Findings

### High — Approved Persona bootstrap lifecycle is absent from the spine

- **Evidence:** `SPEC.md` CAP-1 and Constraints approve durable `setup_pending`, restoration of the same pending Persona/Space/Owner authority after restart, confirmation of separate secret and bundle custody, and no `PersonaCreated` or protected Space work until durable confirmation. `recovery-cases.md` REC-F01 makes this a shared crash/restart fixture. `ARCHITECTURE-SPINE.md:325-329` governs new-Device enrollment and recovery, but does not state the initial Persona activation boundary. AD-13 (`:180-184`) covers generic transaction replay, not this Persona-specific transition.
- **Divergence:** independent shells/runtime implementations could activate a Persona or Owner authority before confirmation, create new IDs after restart, or treat a custody checkbox as a proven backup.
- **Disposition:** **Autofix in spine.** Add the approved staged-to-active invariant to AD-37 or a separately numbered AD, binding `vida-core`, `vida-runtime`, shells and storage. Require same IDs/material across restart, no protected work or `PersonaCreated` before durable confirmation, and one activation after confirmation. Keep the physical atomic-persistence mechanism in OQ-0024/OQ-0036.

### Medium — Recovery spec is not named as a spine companion

- **Evidence:** `SPEC.md` links the spine, but the spine's `companions` list (`:46-62`) omits `spec-vida-persona-recovery/SPEC.md` and its two recovery companions. The approved lifecycle therefore has no explicit upstream trace from the spine.
- **Disposition:** **Autofix** by adding the SPEC link; avoid copying candidate fixture details into the spine.

## Checks passed

- **Enforceable recovery boundary:** AD-37 requires a current `ControllerState` transition and signed `DeviceGrant` before protected reads or accepted operations, fresh replacement Device keys, a separately retained random secret and encrypted versioned bundle, and available ciphertext for history. These match SPEC CAP-2/3 and `recovery-bundle-contract.md`.
- **No hidden crypto decision:** AD-37 names no KDF, AEAD, nonce, encoding, key-store vendor or physical provider. OQ-0024 explicitly gates these choices and restore proof; OQ-0049 separately gates corporate Persona policy.
- **Deferred boundary:** `recovery-cases.md` labels fixtures as candidates, and the spine's implementation gate (`:367`) requires shared positive/negative conformance. Exact current-controller and concurrent-restore proofs remain blocking rather than assumed complete.
