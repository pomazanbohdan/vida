# Platform and security constraints for Story 1.1

Accessed 2026-09-26. Primary platform and standards documentation; publication dates not stated unless noted.

- Android Keystore can keep a device-bound key non-exportable and restrict its use [6]. Therefore use a separate portable recovery mechanism; do not assume a Keystore key itself migrates to a replacement phone. This is an architecture inference, not an API guarantee.
- Android Auto Backup is conditional (user setting, device idle, elapsed time, connectivity), limited to 25 MB per app, and may never run; default inclusion covers app databases [7]. It cannot prove that a Persona or Note is recoverable. Backup inclusion/exclusion must be explicit so encrypted records are not restored without usable key material.
- SQLite documents atomic commits [8]. Persist Persona ID, key envelope, Space ID, Owner grant, and `setup_pending` state as one recoverable transaction, or prove equivalent atomicity; test process termination before/after commit. A successful UI step must follow the durable commit, not precede it.
- OWASP says cryptographic key lifecycle, storage, backup and recovery need an explicit design; data encrypted under lost keys cannot be recovered [9]. MASVS-STORAGE-1 requires secure storage of sensitive data [10]. Neither standard mandates a specific onboarding sequence or “block Notes until recovery confirmed” rule.
- Browser `StorageManager.persist()` may be denied; without it storage may be evicted under pressure [11]. Even granted persistence does not survive explicit user clearing. Web client cannot treat browser storage as the sole recovery copy.

Proposed verification fixture (not yet run): create pending Persona offline; kill before and after transaction commit; restart; verify exactly one stable Persona/Space/key envelope; fail closed on corrupt or partial state; simulate disk-full; confirm locally saved Note only after durable commit; restore from actual encrypted export/replica with recovery material; verify wrong material does not reveal content. This fixture is an engineering proposal, not evidence of existing implementation.

Sources:

[6] Android Developers, “Android Keystore system”, n.d., https://developer.android.com/privacy-and-security/keystore
[7] Android Developers, “Back up user data with Auto Backup”, n.d., https://developer.android.com/identity/data/autobackup
[8] SQLite, “Atomic Commit In SQLite”, n.d., https://www.sqlite.org/atomiccommit.html
[9] OWASP, “Key Management Cheat Sheet”, n.d., https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html
[10] OWASP, “MASVS-STORAGE-1”, n.d., https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/
[11] MDN, “StorageManager.persist()”, last modified 2024-07-26, https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persist
