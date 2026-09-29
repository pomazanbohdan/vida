# Recovery/currentness gate — 2026-09-29

**Verdict: conditional pass.** AD-37/AD-40 contain no unverified library-version or protocol-compatibility claim. The reference behaviors support the direction, but the backup claim needs one explicit distinction before implementation.

## Finding (high): verified bytes are not an independently recoverable copy

AD-37 line 331 requires destination readback, authenticated open and coverage validation before a new backup supersedes the last known-good one. Those tests establish integrity and recorded coverage; they do **not** establish off-device custody, continuing destination availability, or the ability to restore after loss of that same Device. A valid archive kept only on the lost phone is still unavailable. Keep `backup verified` distinct from `recoverable from an independent copy`; make independent-copy evidence and the failure/atomic-replacement fixture explicit under OQ-0024. This is a proposed clarification to VIDA, not a behavior guaranteed by the cited reference apps. [andOTP README](https://github.com/andOTP/andOTP), [SQLite Backup API](https://sqlite.org/backup.html).

## Source support and limits

| Spine refinement | Official-source check | Limit |
|---|---|---|
| AD-40 line 350: Personal/Work Personas on one installation with distinct logical Device identities | [Tailscale identity](https://tailscale.com/docs/concepts/tailscale-identity) describes one physical device in multiple tailnets as separate logical nodes with distinct node keys; [1Password](https://support.1password.com/multiple-accounts/) supports separate personal/work accounts in one app. | Neither specifies VIDA `DeviceGrant` or Space ACL. Standard Tailscale client may have only one active tailnet; simultaneous VIDA sync is VIDA's design, not a Tailscale precedent. |
| AD-37 line 331: later local edits are not covered by an earlier backup | [SQLite Backup API](https://sqlite.org/backup.html) defines completed online backup as a snapshot. | Reference establishes snapshot boundary, not VIDA's frontier format. |
| AD-37 line 331: readback/authenticated parse before replacing known-good backup | [andOTP](https://github.com/andOTP/andOTP) provides internal encrypted exports and warns that external app-data backup can lack Android Keystore material; its [changelog](https://github.com/andOTP/andOTP/blob/master/CHANGELOG.md) records background backups and correction of a falsely reported failure. [Chrome File System Access](https://developer.chrome.com/docs/capabilities/web-apis/file-system-access) permits readback through a retained file handle. | No cited app documents VIDA's full write-readback-decrypt-manifest-restore pipeline. Treat that pipeline as an adopted VIDA safety requirement, not a reference fact. Browser download alone does not prove readback permission. |

**Currentness:** checked official primary pages on 2026-09-29. andOTP is archived/unmaintained (2022); use it as a historical UX/failure reference, not a maintained dependency recommendation. No version pin is introduced by these refinements.
