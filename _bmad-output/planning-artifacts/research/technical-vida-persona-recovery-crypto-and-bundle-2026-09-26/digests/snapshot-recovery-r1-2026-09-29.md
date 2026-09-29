# Backup snapshot and later changes — source digest

Accessed: 2026-09-29. Source publication dates: Signal 2026-09-28; SimpleX 2024-03-23; other pages undated.

- **SQLite:** A completed online backup is a snapshot of the source at its start, not proof of later writes. Source: [SQLite Online Backup API](https://sqlite.org/backup.html). Confidence: high.
- **Signal:** Hosted Secure Backups create a new backup every 24 hours and remove the prior one. Restore scope is bounded; some expiring content is excluded. Source: [Signal backup improvements](https://signal.org/blog/backup-improvements/). Confidence: high.
- **Signal:** Secure Backup requires an available archive and recovery key; a linked device cannot restore the phone's history. Source: [Signal troubleshooting](https://support.signal.org/hc/en-us/articles/10075139325850-Troubleshooting-Signal-Secure-Backups). Confidence: high.
- **SimpleX:** Local database export/import is a separate migration path; a passphrase must be set for export. Source: [SimpleX managing data](https://simplex.chat/docs/guide/managing-data.html). Confidence: high.
- **Inference for VIDA, not a vendor promise:** Restoring a Persona and snapshot cannot establish that no later edits existed. If a later operation was durably accepted only on a lost Device and no other recoverable copy holds it, it is unrecoverable. A peer with that operation may fill the gap after reconciliation. User-facing coverage should be expressed by a verified operation frontier, with human-readable time as supplementary.
