# AD-37 / Persona recovery — currentness and primary-source review

Reviewed: 2026-09-26. Scope: [AD-37](../ARCHITECTURE-SPINE.md), [recovery SPEC](../../../../specs/spec-vida-persona-recovery/SPEC.md), its [bundle contract](../../../../specs/spec-vida-persona-recovery/recovery-bundle-contract.md) and [candidate cases](../../../../specs/spec-vida-persona-recovery/recovery-cases.md), plus the [2026-09-26 research report](../../../research/technical-vida-persona-recovery-crypto-and-bundle-2026-09-26/research.md) and digests. Lens: current source status and the boundary between a product decision and validated cryptography. No source document was changed.

## Verdict

**Conditional pass for the architecture direction; 0 P1, 3 P2 documentation corrections.** The product owner approved separately retained random recovery secret and encrypted bundle, with an available encrypted resource copy additionally required for history. AD-37 and the SPEC correctly leave cipher/byte format, controller freshness, concurrent restore, rotation, durable persistence and cross-platform proof in `OQ-0024`; they do not claim a validated production protocol. The cited product examples are useful precedents, not evidence that VIDA's two-part construction is secure or interoperable.

## Findings

### P2 — Web Crypto source is a Working Draft, not a Recommendation

The research source table at `research.md:95` labels `[14] https://www.w3.org/TR/WebCryptoAPI/` a “W3C Recommendation”. That URL now redirects to **Web Cryptography Level 2, First Public Working Draft (22 April 2025)**, whose status expressly says it is work in progress. The [2017 Web Cryptography API Recommendation](https://www.w3.org/TR/2017/REC-WebCryptoAPI-20170126/) is a separate version. Correct the label to “Level 2 First Public Working Draft” or cite the frozen 2017 Recommendation for the exact feature used. Before choosing a Web recovery implementation, verify feature support in the targeted browsers; normative status alone does not establish conformance. [Current W3C document](https://www.w3.org/TR/WebCryptoAPI/).

### P2 — MASVS-STORAGE-1 is narrower than the SPEC's source attribution

`SPEC.md:44` says algorithm, backup/storage and UX choices are checked against both OWASP Key Management and MASVS-STORAGE-1. [MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) addresses secure local storage of sensitive data in **mobile apps**. It does not validate the VIDA bundle cipher, cross-platform restore, controller continuity or recovery UX. Narrow that citation to the Android/iOS local-storage gate; keep the broader [OWASP Key Management guidance](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html) as lifecycle/backup/compromise-review input, not as approval of a VIDA crypto construction. The SPEC's existing “sources do not approve the format” caveat is correct.

### P2 — Research approval request is stale after the 2026-09-26 decision

`research.md:20` records the product owner's approval of the two-part direction, but `research.md:70-76` still presents that same choice as a proposal and asks the owner whether it is acceptable. Mark that final section as a historical question now answered, or replace it with the remaining implementation decisions. Preserve the distinction: approval of user custody and separate encrypted data availability does **not** approve the report's candidate 256-bit representation, HKDF/AEAD profile, bundle fields or restore proof.

## Verified boundary and remaining proof

- [AD-37](../ARCHITECTURE-SPINE.md) requires a current `ControllerState` transition and new Device keys; it explicitly blocks production enrollment claims pending `OQ-0024/OQ-0049`. An encrypted checkpoint in a portable bundle cannot by itself establish the *current* controller head after offline revocation or concurrent restore. This is an open protocol proof, not an external-source claim.
- [Signal Secure Backups](https://support.signal.org/hc/en-us/articles/9708267671322-Signal-Secure-Backups) supports the narrower observation that an actual encrypted archive and its user-held recovery key are both needed. [Element's guide](https://docs.element.io/latest/element-support/device-verification/how-to-ensure-you-have-a-recovery-key/) describes checking its recovery key through a fresh Web sign-in. Neither validates VIDA's serverless controller recovery.
- [SimpleX's data guide](https://simplex.chat/docs/guide/managing-data.html) distinguishes database passphrase, backup and export/import; its [security guide](https://simplex.chat/docs/guide/privacy-security.html) names Keychain/Keystore for local passphrase custody. [Delta Chat's FAQ](https://delta.chat/en/help) distinguishes QR second-device setup from manual backup transfer. These support the local-custody versus portable-copy distinction only.
- [WHATWG Storage](https://storage.spec.whatwg.org/) makes persistent browser storage permission-dependent; a browser profile is not a guaranteed off-device backup. The Android/Windows platform references in the research likewise describe local key protection, not a tested VIDA restore path.

Re-review the mutable product/platform pages and browser support before implementation; retain cross-platform negative fixtures and an independent cryptographic review as production gates. No new library version is bound by AD-37 or this review.
