# AD-37 / AD-40 controller history — currentness and reality review

Reviewed: 2026-09-28. Scope: [AD-37 and AD-40](../ARCHITECTURE-SPINE.md) only. Lens: current primary sources for DID documents, causal event comparisons, device enrollment/recovery and OWASP. This is a source and architecture-boundary review, not a protocol implementation or approval of cryptography. No production document was changed.

## Verdict

**Conditional pass; 0 P1, 2 P2 source-boundary cautions.** Signed per-Device append-only causal events with a derived `ControllerState` are an internally coherent VIDA design choice. The primary sources support particular concepts and contrasts below; none specifies or validates VIDA's current-controller proof, reconciliation signer, branch acceptance, restore freshness or wire format. AD-37/AD-40 correctly retain those as `OQ-0022/0024/0049` production gates.

## Findings

### P2 — A DID document can expose controller state, but DID Core supplies no VIDA controller-history algorithm

AD-40's optional projection wording is sound. [DID Core v1.0](https://www.w3.org/TR/2022/REC-did-core-20220719/) is the W3C Recommendation: it defines DID document properties and verification relationships, while §8.2 assigns update authorization, authentic resolution, update and deactivation rules to a **DID method**. Its §9.9 discusses recovery without prescribing VIDA-style causal conflict handling. [DID v1.1](https://www.w3.org/TR/2026/CR-did-1.1-20260305/) was a Candidate Recommendation Snapshot on 2026-03-05, not a completed Recommendation at this review date. If VIDA later publishes a DID method, its method spec must define how a resolved document is computed from verified history, how stale/equivocating branches are handled, and how update/recovery authority is authenticated. Neither a document's `controller` field nor possession of a recovery key establishes the current head. This last sentence is an inference from the method-specific requirements and AD-37/AD-40, not a W3C-prescribed VIDA mechanism.

### P2 — Matrix's causal graph is precedent for preserving evidence, not for VIDA's no-winner access rule

[Matrix room version 12](https://spec.matrix.org/latest/rooms/v12/) represents room events with previous/authentication links and defines a conflicted event set; it then computes an effective room state through room-version-specific state resolution and iterative authorization checks. Its ordering can use power level, origin timestamp and event ID. VIDA AD-40 intentionally gives no shared access to a disputed new grant until signed reconciliation. Therefore cite Matrix only for the feasibility of DAG-based causal/auth evidence and explicit state resolution. Do not import its effective-state winner, power-level or timestamp rules into Persona authority. This is a comparison of distinct rules, not a claim that Matrix implements VIDA's model.

## Verified product and security boundaries

- [Delta Chat FAQ](https://delta.chat/en/help) says a second Device can be added from an existing Device through local QR transfer; after successful transfer both Devices work independently. Its manual path transfers a backup. This supports independent post-transfer operation, not VIDA's signed controller-event graph, recovery credential or conflict policy.
- [Signal Linked Devices](https://support.signal.org/hc/en-us/articles/360007320551-Linked-Devices) currently describes a registered primary Device that links other Devices; the primary must come online at least once per 30 days, and linked Devices eventually unlink after inactivity. Signal is a contrast for VIDA's equal Device peers, not evidence for them. [Signal Secure Backups troubleshooting](https://support.signal.org/hc/en-us/articles/10075139325850-Troubleshooting-Signal-Secure-Backups) says an enabled backup and its 64-character recovery key are both needed for history restore, and linked Devices cannot restore history to the phone. It does not validate VIDA's controller recovery.
- [OWASP Key Management Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html) covers key lifecycle, compromise/recovery and protected backup; it advises against escrowing signature keys. AD-37's fresh replacement Device signing keys and distinct recovery credential align with that guidance as a design direction. [MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) governs secure storage of sensitive data in mobile apps; it is a local Android/iOS storage gate, not approval of the bundle, browser storage, controller proof or cross-platform restore.

## Remaining proof, not a source finding

An offline recovery Device can hold a valid older checkpoint while an unseen Device has revoked a grant or another recovery has forked the controller history. Signatures authenticate the branches but cannot alone prove that the observed frontier is current. AD-37's provisional local grant and AD-40's disputed-grant denial preserve the safe boundary. A release claim still needs exact history availability/freshness and anti-equivocation rules, the authorized reconciliation signer and branch coverage, and cross-platform negative fixtures under the named OQ gates. No product comparison or general standard above closes those proofs.
