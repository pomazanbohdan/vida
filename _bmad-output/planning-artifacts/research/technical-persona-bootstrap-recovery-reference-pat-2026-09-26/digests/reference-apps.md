# Reference apps: identity bootstrap and recovery

Accessed 2026-09-26. Primary project documentation; publication dates are not stated unless noted.

| Project | Account usable before recovery setup? | Key/data recovery pattern | Boundary of evidence |
|---|---|---|---|
| Delta Chat | FAQ says the first profile is created on first installation [1]. Manual backup export is later under Settings → Chats, and second-device QR transfer is a separate flow [2]. | Existing device transfers profile by QR on the same network; a manual backup can be exported/restored [2]. | FAQ does not document crash/restart behavior during incomplete export, nor require a recovery phrase at profile creation. Current ordinary messaging uses chatmail relay; Delta is a UX reference, not VIDA's transport [1]. |
| Signal | Secure Backups are opt-in from Settings after registration; enablement requires recording and confirming a 64-character key [3]. | Backup cannot be restored without the key [3]. | No evidence that Signal blocks ordinary messaging pending backup setup. This is backup confirmation, not account creation. |
| SimpleX | Default local DB uses a random passphrase; user manually sets one for export or system backup [4]. | App data export is explicitly separate from the default local store [4]. | No documented kill-during-export state. Local encryption is not proof of recoverability. |
| Element | Element X/Pro require device verification at login, but UI can still show “Get recovery key” later; Element Web verification is not yet mandatory [5]. | A verified logged-in device can create/rotate recovery key later [5]. | Verification and recovery-key setup are different gates; this does not determine VIDA's product policy. |

Cross-project conclusion: none of these official sources supports the claim that an unfinished recovery confirmation must block all local content creation. The direct crash/restart UX for an interrupted recovery prompt is not documented; resuming one stable pending Persona is a VIDA design inference, not a copied reference behavior.

Sources:

[1] Delta Chat, FAQ, n.d., https://delta.chat/en/help
[2] Delta Chat, FAQ “Multi-client / Manual Transfer”, n.d., https://delta.chat/en/help
[3] Signal Support, “Signal Secure Backups”, n.d., https://support.signal.org/hc/en-us/articles/9708267671322-Signal-Secure-Backups
[4] SimpleX Chat, “Managing Your Data”, n.d., https://simplex.chat/docs/guide/managing-data.html
[5] Element, “How to Ensure You Have a Recovery Key”, n.d., https://docs.element.io/latest/element-support/device-verification/how-to-ensure-you-have-a-recovery-key/
