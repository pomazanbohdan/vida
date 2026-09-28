---
title: 'Technical research: VIDA recovery, federation and sync reference patterns'
type: technical
topic: 'VIDA recovery, federation and sync reference patterns'
decision: 'Separate confirmed VIDA requirements from open protocol choices'
source: 'primary official documentation'
status: complete
preset: standard
validation: normal
created: 2026-09-20
updated: 2026-09-20
verified_claims: 1
unverified_claims: 4
---

# VIDA: recovery, federation and sync reference patterns

## Executive summary

The references support three distinct contracts, not a single library solution: (1) recovery needs a user-controlled secret **and** available encrypted state, (2) federated servers may combine deployable roles but their visibility/authority must be declared per role, and (3) CRDT convergence does not settle business authority or exactly-once external effects. VIDA requirements already approved by the user are recorded under `docs/`; recommendations below are **not** accepted implementation choices.

## Recovery

Delta Chat documents moving a profile from a surviving device via QR or restoring from an earlier exported backup; its FAQ does **not** document a mandatory recovery phrase. It therefore is not a complete reference for all-devices-lost recovery [1]. Signal Secure Backups is a closer illustration: an encrypted backup archive is unlocked by a key held by the user, and Signal cannot recover it when the key is lost [2]. Android Auto Backup has size, schedule and user-setting constraints, so its availability cannot be assumed [3]. Windows DPAPI-protected data is tied to a protection context that can disappear after profile deletion or device reimaging [11]. Android cautions that restoring an encrypted file may leave it without the original key; the cited `EncryptedFile` API itself is deprecated and is not a VIDA technology recommendation [12]. OWASP requires explicit key recovery/backup planning and cautions against escrow of digital-signing keys [4].

VIDA implication: the user's approved requirement is to generate recovery material during autonomous Persona setup and require confirmation of separate storage. A recoverable encrypted state copy and a restore drill are still necessary to deliver full data recovery. Platform/cloud storage should be a redundant, user-selected route for an encrypted bundle, not the sole route. Cryptosystem, exportability of each key class, bundle location and recovery after compromise remain open.

## Federated node roles

Iroh's relay forwards encrypted traffic without storing it; that transport role alone neither hosts an offline mailbox nor accepts a business operation [5]. AT Protocol explicitly separates account/data PDS, relays and app-specific App Views, though its public-social model does not imply private VIDA content should be server-readable [6]. Matrix uses server-to-server event replication and explicit room authorization/state rules, demonstrating a different trust/visibility envelope [7].

VIDA implication: model identity/account, address lookup/relay, durable ciphertext mailbox, replica/blob hosting and catalog/AppHost as separate capabilities; Space authority and managed automation are optional **proposals**. One physical operator may co-host several, but deployment must disclose per capability its visible metadata/plaintext, key custody, retention, acceptance authority and side effects. A private Space remains E2EE without automatic node decryption; if a managed Space delegates decryption/automation, that requires an explicit policy and separate threat review.

## Offline conflicts and reactions

For group messaging, Matrix publishes an origin-server timestamp and uses a per-device transaction ID to distinguish retry from a new send [13]. Nostr NIP-01 signs a client-created time and illustrates that a relay can reject an event dated too far from current time [14]. These are alternative conventions, not a VIDA protocol choice. The user's requirement for a timeline consistent across Messenger and project discussions favors separating local draft creation time from the eventual shared publication order/time; the exact authority and timestamp source are still open.

Automerge preserves concurrent values but its chosen map winner is based on operation ID, not physical time [8]. Loro Map resolves by Lamport logical timestamp [9]. Thus neither establishes which disconnected device first changed a task by real-world time. The user's latest local-time preference conflicts with currently approved `REQ-SYNC-004` (first comparable authority acceptance), so no rule change is justified without an explicit choice and clock-skew/rollback semantics.

For one derived note per accepted fact, the proposed design is a stable logical effect key, version-pinned rule and uniqueness/dedupe at the applicable authority. For an external email, a durable intent/outbox can preserve the request **after a successful atomic durable commit and with recovery**, but may still duplicate delivery [10]. Downstream idempotency with an adequate retention window, or a provider status lookup backed by a stable external ID and safe retry protocol, must be established separately. A lookup alone does not eliminate a lookup/send race. Without those guarantees, an unknown result needs reconciliation, not an exactly-once promise. Iroh transports the envelope; Automerge/Loro do not provide this workflow contract.

## Cross-dimension insight

Do not grant a physical node recovery custody, content visibility or effect authority just because it hosts an identity, relay or replica role. Each grant changes the trust boundary independently. Likewise, a local timestamp is useful provenance/UI metadata, not a substitute for signed authority ordering.

## Recommendations for downstream docs

1. `docs/02-requirements/identity-requirements.md`: retain approved mandatory user-held recovery material; specify the encrypted bundle/restore protocol only after security design.
2. `docs/00-governance/open-questions.md`: expand `OQ-0026` into a capability/visibility/key-custody matrix and managed-mode decision.
3. `docs/02-requirements/transport-sync-requirements.md`: keep `REQ-SYNC-004` unchanged until the local-clock conflict is resolved; `OQ-0033/0034` need authority receipt/order fixtures.
4. `docs/02-requirements/effect-execution-constraints.md`: keep one logical derived resource as outcome; choose stable-key/outbox/executor details under `OQ-0045/0048` after provider guarantees are known.

## Open questions

- Is the public conflict rule first comparable authority acceptance, or earliest device-local timestamp after reconnect? What if clocks differ by hours or are rolled back?
- For a group message drafted offline, does the shared timeline use device creation time or Space-authority publication time/order after reconnect? Should the original creation time remain visible as provenance?
- Where is the encrypted recovery bundle stored when *all* devices are lost, and how is restoration verified without exposing signing keys to an operator?
- Which federation capabilities can see Space metadata/plaintext in private versus explicitly managed deployments?
- Which external effect providers support durable idempotency keys or status lookup, and what is the fallback when outcome is unknown?

## Sources

| Ref | Finding | Publisher / publication date / accessed | Confidence |
|---|---|---|---|
| [1] | Profile transfer and backup | [Delta Chat](https://delta.chat/en/help), n.d., 2026-09-20 | medium, single source |
| [2] | Encrypted archive and recovery key | [Signal](https://signal.org/blog/introducing-secure-backups/), 2025-09-08, 2026-09-20 | high for historical design |
| [3] | Auto Backup limits and conditions | [Android Developers](https://developer.android.com/identity/data/autobackup), n.d., 2026-09-20 | medium, single source |
| [4] | Key recovery/backup planning | [OWASP](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html), n.d., 2026-09-20 | medium, single source |
| [5] | Stateless encrypted relay | [Iroh](https://www.iroh.computer/blog/shared-relays), 2026-09-08, 2026-09-20 | medium, single source |
| [6] | PDS/relay/AppView separation | [AT Protocol](https://atproto.com/guides/the-at-stack), n.d., 2026-09-20 | medium, single source |
| [7] | Federated room events/state | [Matrix specification](https://spec.matrix.org/latest/server-server-api/), n.d., 2026-09-20 | medium, single source |
| [8] | Conflict winner and preserved values | [Automerge](https://automerge.org/docs/reference/documents/conflicts/), n.d., 2026-09-20 | medium, single source |
| [9] | Lamport map LWW | [Loro](https://www.loro.dev/docs/tutorial/map), n.d., 2026-09-20 | medium, single source |
| [10] | Outbox duplicates and idempotency | [AWS](https://docs.aws.amazon.com/prescriptive-guidance/latest/cloud-design-patterns/transactional-outbox.html), n.d., 2026-09-20 | medium, single source |
| [11] | DPAPI recovery boundary | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/apps/develop/security/data-protection), updated 2026-07-17, accessed 2026-09-20 | medium, single source |
| [12] | Restored encrypted file may lack key | [Android Developers](https://developer.android.com/reference/androidx/security/crypto/EncryptedFile), updated 2026-06-24, accessed 2026-09-20 | medium, single source |
| [13] | Event timestamp and retry transaction ID | [Matrix specification](https://spec.matrix.org/latest/client-server-api/), n.d., 2026-09-20 | medium, single source |
| [14] | Client-created signed timestamp | [Nostr NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md), n.d., 2026-09-20 | medium, single source |

## Staleness map

The protocol/library behavior claims [5], [8], [9], [13], [14] and platform-backup behavior [3], [11], [12] are version-sensitive and must be rechecked before binding a release stack or security contract. The mechanical two-year pattern window for the dated [2] and [5] claims gives the earliest recheck as **2027-09-08**. For undated live pages, no publication-age deadline can be calculated; recheck them at the implementation decision gate instead of inventing a publication date. Signal's 2025-09-08 announcement [2] is historical context, not a claim that its current platform rollout or pricing is unchanged. `verified_claims`/`unverified_claims` in frontmatter count the **five ledgered load-bearing claims**, not all fourteen source rows.
