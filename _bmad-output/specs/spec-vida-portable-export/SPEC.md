---
id: SPEC-vida-portable-export
status: draft
companions:
  - portable-export-cases.md
  - ../../planning-artifacts/prds/prd-vida-2026-09-22/prd.md
  - ../../../docs/02-requirements/open-interoperability-requirements.md
  - ../../../docs/04-specifications/identity-domain-contract.md
  - ../../../docs/02-requirements/platform-nfr.md
  - ../spec-vida-resource-domain/SPEC.md
  - ../spec-vida-persona-recovery/SPEC.md
sources: []
---

> **Decision-gated Release-1 kernel.** FR-36 and REQ-OPEN-007 require public schemas, portable encrypted data and an independent reader. The archive encoding, cryptographic profile and recovery mechanism remain open; the cases are plans, not executed evidence.

# VIDA portable encrypted export

## Why

A person must be able to continue using their VIDA data with an independent compatible implementation if VIDA services or the original application are unavailable. The Release-1 open-baseline promise requires more than a documented format: an authorized encrypted export and a working independent reader must be demonstrable without a paid VIDA service.

## Capabilities

- **CAP-1**
  - **intent:** A person can carry the data they are authorized to export across VIDA Apps.
  - **success:** A complete package includes the selected accessible versioned schemas, Resources, Relations, Files/blobs and necessary identity/key-recovery metadata; missing required content never receives a complete claim. A relation never grants export access to an unreadable target.
- **CAP-2**
  - **intent:** A person can keep the portable copy confidential under their own authorized recovery material.
  - **success:** The package is encrypted; an unauthorized reader cannot expose its protected content or metadata, while an authorized reader can open it without access to a live VIDA account or commercial service.
- **CAP-3**
  - **intent:** An independent implementation can understand a VIDA export from the published contract.
  - **success:** Public schemas, a versioned export specification and a distributable independent reference reader are Release-1 gates; in a clean environment the reader consumes a public fixture and reconstructs the included schema/resource/relation/file links without private SDKs or credentials.
- **CAP-4**
  - **intent:** A person can tell whether their export is complete and usable.
  - **success:** Missing bytes, interrupted output, failed authorization, corruption or unsupported versions never produce a false complete/success state; the reader reports the affected portion without silently discarding retained data.

## Constraints

- Export uses the active account context and current Resource/Relation/File permissions. Effective revocation or expiry of the shared-Space rights-reconciliation interval blocks managed export reads; Personal Space remains a separate case.
- Export must not reveal another Persona's data merely because several accounts share a local vault. A known Resource ID, backlink, attachment or App dependency does not bypass the target's read/export gate.
- The public specification, fixtures, independent reader and open baseline must work without paid relay, mailbox, backup or hosted-replica credentials.
- Recovery material alone cannot restore unavailable history; an encrypted package alone cannot prove Persona controller authority. The reader's authorization to decrypt, controller continuity and availability of data bytes are separate checks under SPEC-ID-011 and the recovery kernel.
- Recovery metadata is not permission to embed plaintext secrets in the archive. Exact secret representation, key custody and encryption profile await OQ-0024; local staging, logs and platform share flows remain security test surfaces.

## Non-goals

- Selecting a ZIP/container encoding, cipher, KDF, key-wrap, signature domain or recovery UX in this kernel.
- Treating an encrypted export as proof that an independent client can already write, synchronize or import every VIDA operation; those are separate conformance claims.
- Making a hosted backup or a remote VIDA account mandatory for baseline portability.

## Success signal

On Android, iOS and Windows, a user exports an authorized Persona/Space fixture, transfers the encrypted package to a clean environment without VIDA credentials, and opens it with the published independent reader and proper recovery material. Schemas, objects, relations and file bytes remain linked; a revoked or cross-Persona target is absent; corruption or a missing blob is reported instead of a false success.

## Open Questions

- What public archive grammar, canonical bytes, integrity/signature and encryption/key-wrap profile will implement the package? OQ-0024 governs recovery; OQ-0028 applies separately if the package carries signed operation envelopes.
- How will a package represent remotely unavailable blobs or unsupported schema extensions, and when may an intentionally partial export be marked usable rather than complete?
- What minimum independent *import/write* behavior, if any, must Release 1 prove beyond the approved independent-reader gate?
- Which licenses and contribution rules cover the normative format, schemas, fixtures and reference assets (`OQ-0055`)?
