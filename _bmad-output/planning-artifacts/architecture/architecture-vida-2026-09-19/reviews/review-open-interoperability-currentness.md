# AD-17 / ADR-0013 currentness and evidence review

- Reviewed: 2026-09-20
- Scope: `ARCHITECTURE-SPINE.md` AD-17, `ADR-0013-open-interoperability.md`, `open-interoperability-requirements.md`, cited research, and accepted ADR-0005/0007/0008/0009 boundaries
- Verdict: **CONDITIONAL PASS** — 0 P0, 1 P1, 2 P2
- Review mode: documentary and primary-source verification; no interop prototype or legal opinion

## P1 — Core open-source license is missing from the explicit decision gate

**Location:** ADR-0013 Context and decision 4; AD-17; OQ-0055.

**Evidence:** ADR-0013 calls VIDA an *open-core* super-app and permits closed paid modules. OQ-0055 asks for licenses of normative texts, schemas/fixtures, conformance suite, and reference code, but does **not explicitly name `vida-core` or other promised open-source core modules**. The [Open Source Definition](https://opensource.org/osd) makes clear that source visibility is insufficient: distribution terms must grant redistribution, modification, and use without field-of-endeavor discrimination. An unspecified core license cannot support an unqualified “open-source core” release claim.

**Risk:** A release could satisfy the literal OQ-0055 list, publish implementation source, and still be only source-available; independent client/node/App builders could lack the intended rights to fork, redistribute, or embed the core. The license of specification materials is a separate question from the software license.

**Required disposition:** Extend OQ-0055 and the production-publication gate to name the exact *core code* boundary and its OSI-compatible software license(s), separately from spec/schema/test/reference-code licenses. Record contribution/IPR and patent-grant expectations for each normative surface. Until this is approved and applied to the repository, describe open-core as the accepted *product target*, not as a verified current legal status.

## P2 — IPR gate should distinguish patent permission from copyright permission

**Location:** OQ-0055, ADR-0013 consequences, REQ-OPEN-006.

**Evidence:** The documents correctly leave IPR open, but do not spell out whether essential-patent permissions are part of the independent-implementation claim. The [W3C Patent Policy](https://www.w3.org/policies/patent-policy/) treats royalty-free implementability and contributor patent commitments as distinct from publication of a specification. W3C is a **reference model**, not a governance authority VIDA has chosen.

**Risk:** Text and test licenses could be open while an essential patent commitment remains unresolved. This is particularly relevant if external proposals become normative.

**Recommendation:** In OQ-0055, separately require legal review of copyright licenses, patent rights/commitments, trademark and conformance-mark rules, and contributor declarations. Do not claim W3C-style RF status without adopting and executing such a policy.

## P2 — State the first interoperable profile and independent-implementation proof gate

**Location:** AD-17 and ADR-0013 acceptance consequences.

**Evidence:** The requirements ask for the same public conformance path and an independent client/node versus official client/node check. That is sound. The precise first profile, supported roles/capabilities, and release window remain unnamed; the document intentionally defers wire-level closure to OQ-0028 and related gates. Therefore current acceptance is a policy commitment, **not evidence that third-party interoperability exists today**.

**Risk:** “Open VIDA protocol” could be claimed for the whole platform when only one narrow profile has specifications and passing fixtures.

**Recommendation:** Tie every public compatibility claim to a named immutable profile/version and a test matrix: independent client ↔ official node, official client ↔ independent node, and independent ↔ independent where meaningful; include positive, negative-security, mixed-version, and refusal behavior. Publish suite version, result, and known exclusions in the release manifest. ADR-0013 already requires much of this; the profile-scoped claim rule should be explicit in implementation planning.

## Verified claims and boundaries

| Claim in new material | Primary evidence | Assessment |
|---|---|---|
| NIPs are optional, public implementation proposals; NIP number is not event `kind` | [NIPs repository](https://github.com/nostr-protocol/nips), [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md) | Supported. The repo says NIPs are not a checklist; NIP-01 defines JSON event `kind` and client–relay WebSocket messages. |
| NIP-29 does not supply VIDA-wide Space authority/role semantics | [NIP-29](https://github.com/nostr-protocol/nips/blob/master/29.md) | Supported as architectural inference. Current NIP-29 is draft/optional, relay-enforced; role semantics are relay policy and groups may fork. |
| NIP-78 is app-specific private data, not a generic shared-App interoperability model | [NIP-78](https://github.com/nostr-protocol/nips/blob/master/78.md) | Supported. Current NIP-78 is draft/optional and explicitly targets custom apps that do not require interoperability. |
| Matrix MSC is a public proposal/review path with implementation evidence and unstable vendor-prefixed features | [Matrix MSC process](https://spec.matrix.org/proposals/) | Supported. Do not describe VIDA as following Matrix governance; the ADR correctly uses it as process inspiration only. |
| AT Protocol Lexicon is schema language for records, XRPC, and streams, not VIDA's Iroh wire format | [Lexicon specification](https://atproto.com/specs/lexicon) | Supported. Lexicon is tied to the AT data model; the ADR does not adopt it. |
| Iroh remains the chosen VIDA transport with VIDA-owned ALPN/envelope | [iroh 1.2.0 docs](https://docs.rs/crate/iroh/latest), accepted [ADR-0005](../../../../../docs/03-architecture/decisions/ADR-0005-iroh-transport-foundation.md) | Supported. As of review, docs.rs shows 1.2.0 as latest; Iroh exposes Router/ALPN, direct QUIC and relay fallback but does not define VIDA identity/authorization/sync. |
| Independent external Apps and repositories are within existing VIDA scope | accepted [ADR-0007](../../../../../docs/03-architecture/decisions/ADR-0007-declarative-app-packages.md), [ADR-0008](../../../../../docs/03-architecture/decisions/ADR-0008-package-distribution-channels.md), [ADR-0009](../../../../../docs/03-architecture/decisions/ADR-0009-managed-application-logic.md) | Supported. ADR-0013 adds public contract/conformance obligations without weakening Space grants or trusted host API boundaries. |
| Public protocol contracts do not imply free hosted service, marketplace acceptance, or access to private Spaces | ADR-0013 decisions 3–4 and existing package/Space policy | Correctly distinguished. This is a VIDA policy boundary, not an externally sourced standard. |

## Currentness and non-adoption notes

- [NIPs](https://github.com/nostr-protocol/nips) currently describes acceptance criteria of at least two clients and one relay *where applicable*, optional/backwards-compatible changes, and no duplicate ways to perform the same action. The ADR does not import those criteria as VIDA requirements, which is appropriate.
- NIP-29 and NIP-78 are marked `draft`/`optional` as of this review; do not treat them as stable VIDA dependencies.
- The research cites current [OpenAPI 3.2.1](https://spec.openapis.org/oas/v3.2.1.html), [JSON Schema](https://json-schema.org/specification), and [RFC 8785 JCS](https://www.rfc-editor.org/rfc/rfc8785) only as possible tools. ADR-0013 expressly does not choose a codec or schema language; no version pin is required here.
- Nostr bridge remains an optional future adapter, not a second canonical VIDA transport. This is consistent with ADR-0005.
- No open specification, published conformance suite, implementation result, license, or independent interoperability test was verified as **already shipped** by VIDA. The new wording is future-tense obligations and an implementation gate, not proof of delivery.

## Gate recommendation

Accept AD-17/ADR-0013 as an architectural **target** after resolving the P1 wording/gate omission. Retain the current production-claim block until OQ-0055 and relevant protocol/package specs, licenses, external conformance tests, and immutable release evidence are complete. The P2 items may be closed during OQ-0055/profile planning; neither requires changing the Iroh decision.
