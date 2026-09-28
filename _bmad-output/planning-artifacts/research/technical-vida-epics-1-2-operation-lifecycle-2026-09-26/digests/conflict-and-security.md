# Digest: conflict and security

Checked 2026-09-26. Scope: Stories 2.4–2.9, 2.12.

- Automerge preserves concurrent values but exposes a deterministic arbitrary visible winner for conflicting same-property writes; getConflicts retrieves alternatives. [Automerge](https://automerge.org/docs/reference/documents/conflicts/)
- OWASP requires key backup, compromise/rotation planning and protected key storage; WebCrypto non-extractability does not defeat same-origin script compromise. [OWASP key](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html), [W3C](https://www.w3.org/TR/WebCryptoAPI/)
- DOM XSS is a real path by which browser code can obtain plaintext or invoke signing/decryption operations. [OWASP DOM XSS](https://cheatsheetseries.owasp.org/cheatsheets/DOM_based_XSS_Prevention_Cheat_Sheet.html)

Inference: VIDA must separate CRDT convergence from domain authority; an old recovery checkpoint cannot by itself prove that no newer revocation exists in disconnected replicas.
