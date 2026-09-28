# Recovery: primary-source digest

- Delta Chat FAQ: same-profile transfer via QR from an existing device or manual export/restore from an existing backup; no mandatory recovery phrase documented. Publisher Delta Chat; publication date not stated; accessed 2026-09-20. [1]
- Signal Secure Backups: opt-in encrypted archive unlocked by a user-held 64-character recovery key; loss of key prevents Signal recovery. Publisher Signal, 2025-09-08; accessed 2026-09-20. [2]
- Android Auto Backup: user/device conditions, 25 MB cap, and periodic execution mean it is not guaranteed to contain the VIDA recovery bundle. Publisher Android Developers; publication date not stated; accessed 2026-09-20. [3]
- OWASP Key Management: separate key recovery/backup and compromise planning is required; key classes have different escrow needs. Publisher OWASP; publication date not stated; accessed 2026-09-20. [4]
- Windows DPAPI-protected data can become unrecoverable when the user profile is deleted or machine reimaged. Publisher Microsoft Learn; last updated 2026-07-17; accessed 2026-09-20. [11]
- Android's `EncryptedFile` documentation cautions that restored ciphertext may lack the former device key; this specific API is deprecated, so the warning illustrates a class of restore hazard, not a recommended VIDA API. Publisher Android Developers; last updated 2026-06-24; accessed 2026-09-20. [12]
- Inference for VIDA: user-held recovery secret plus accessible encrypted data bundle; OS backup is optional redundant storage, not the only route. The chosen bundle cryptography and restore proof remain open.

[1]: https://delta.chat/en/help
[2]: https://signal.org/blog/introducing-secure-backups/
[3]: https://developer.android.com/identity/data/autobackup
[4]: https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html
[11]: https://learn.microsoft.com/en-us/windows/apps/develop/security/data-protection
[12]: https://developer.android.com/reference/androidx/security/crypto/EncryptedFile
