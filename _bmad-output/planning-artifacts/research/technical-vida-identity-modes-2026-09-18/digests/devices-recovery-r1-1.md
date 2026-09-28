# Devices, recovery and revocation — round 1

- claim: Multi-device E2EE requires explicit user/device separation and per-device session state. Signal Sesame defines UserID, DeviceID and per-device identity-key variants separately.
  source: https://signal.org/docs/specifications/sesame/
  publisher: Signal
  pub_date: 2017
  accessed: 2026-09-18
  confidence: high
  class: security-pattern
- claim: Cross-signing can make a user-level master key authorize device keys without treating every device as the user identity.
  source: https://github.com/matrix-org/matrix-spec/blob/main/content/client-server-api/modules/end_to_end_encryption.md#cross-signing
  publisher: Matrix.org Foundation
  pub_date: n.d.
  accessed: 2026-09-18
  confidence: high
  class: security-pattern
- claim: Existing trusted devices or offline recovery material can authorize a new device; device revocation should rotate per-user encryption material for remaining devices.
  source: https://book.keybase.io/account and https://book.keybase.io/docs/teams/puk
  publisher: Keybase
  pub_date: 2022 / n.d.
  accessed: 2026-09-18
  confidence: high
  class: recovery-pattern
- claim: A raw local keypair without backup makes identity loss irreversible. Automerge Repo Keyhive currently warns that clearing its key storage destroys the identity because it has no backup or recovery mechanism.
  source: https://automerge.org/docs/keyhive/ark-api-guide/
  publisher: Automerge
  pub_date: 2026
  accessed: 2026-09-18
  confidence: high
  class: implementation-risk

Leads: Vida needs a controller/recovery layer above device keys, explicit device certificates, and recovery that never silently gives a provider plaintext access.
