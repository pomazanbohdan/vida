# Standards and platform custody — checked 2026-09-26

| Source | Primary observation | VIDA implication |
|---|---|---|
| [OWASP Key Management](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html) | Separate key purposes, protect exported key material, plan recovery and rotation; unrecoverable decryption keys lose data. | Device-signing keys must not be copied as recovery material; user-held recovery authority must be distinct and explicitly risk-reviewed. |
| [OWASP Cryptographic Storage](https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html) | Use CSPRNG, authenticated encryption, appropriate key lifecycle. | Random high-entropy recovery secret, authenticated portable capsule, no unauthenticated archive. |
| [RFC 5869 HKDF](https://www.rfc-editor.org/rfc/rfc5869) | `info` supports domain-separated derived keys. | Distinct derivation labels for capsule wrapping, recovery authorization and verification. |
| [RFC 9106 Argon2](https://www.rfc-editor.org/rfc/rfc9106) | Argon2id hardens low-entropy passwords against offline guessing. | An optional human passphrase path needs Argon2id and device benchmarks; it is not a substitute for random recovery material. |
| [RFC 8439 ChaCha20-Poly1305](https://www.rfc-editor.org/rfc/rfc8439) and [RFC 5116 AEAD](https://www.rfc-editor.org/rfc/rfc5116) | Standard authenticated-encryption constructions require unique nonces for a key. | Profile the concrete AEAD, nonce allocation and authenticated metadata before approval. |
| [Android Keystore](https://developer.android.com/privacy-and-security/keystore) and [Auto Backup](https://developer.android.com/identity/data/autobackup) | Android non-exportable keys protect local secrets; automatic backup has conditions and size limits. | Keystore wrapping helps device at-rest protection, not sole lost-device recovery. |
| [Web Crypto](https://www.w3.org/TR/WebCryptoAPI/) and [WHATWG Storage](https://storage.spec.whatwg.org/) | Non-extractable browser keys exist, but origin code can use them; persistent storage is conditional. | Static Web needs independent off-browser recovery kit and ciphertext copy. |
| [Windows DPAPI](https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata) | Protected data is tied to Windows account/machine context unless explicitly designed otherwise. | Local wrapping is not a portable cross-device recovery path. |

Confidence: high for standard/platform constraints; exact VIDA cipher suite and platform policy require implementation tests.
