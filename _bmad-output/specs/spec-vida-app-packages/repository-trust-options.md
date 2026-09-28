# Repository trust and update options (decision-gated)

This companion is source-backed comparison material for `OQ-0041`/`OQ-0043`, **not** an adopted VIDA manifest, trust root or `security-auto` policy. Accepted boundaries remain in [SPEC.md](SPEC.md), `REQ-APP-001–021` and ADR-0007/0008. Observation: 2026-09-23.

| Distribution profile | Useful standard behavior | VIDA-specific work / tradeoff |
|---|---|---|
| [TUF metadata](https://theupdateframework.github.io/specification/v1.0.26/) + [OCI Distribution artifacts](https://github.com/opencontainers/distribution-spec/blob/main/spec.md) | Signed root/targets/snapshot/timestamp roles and expiry protect against stale/mixed metadata; OCI names content by digest and supports artifact distribution. | VIDA still defines typed AppPackage manifest, Developer/namespace mapping, source trust UX, authorization, compatibility and activation. OCI digest alone is neither publisher identity nor freshness. Candidate for prototype, not selected. |
| TUF metadata + ordinary digest-addressed artifacts | Retains TUF trust/freshness control without OCI registry conventions. | VIDA must specify transport/discovery/resume and third-party repository interoperability itself. |
| Custom signed index and artifact format | Full distribution control. | VIDA would own root rotation, delegation, stale metadata defense and cross-vendor conformance; avoid inventing update-trust cryptography without a compelling requirement. |

TUF signature verification proves authorization under configured repository keys, **not** a publisher's legal identity. Developer verification, publisher organization membership, root-key bootstrap/rotation, risk acceptance for an external source, revocation and namespace delegation remain `OQ-0041`/`OQ-0043`. One candidate UX is explicit repository connection plus root-key verification, separate publisher trust and separate AppInstance update-mode selection; connecting or downloading never grants Space resources.

`security-auto` has no approved eligibility predicate. A candidate test profile requires authenticated fresh metadata, artifact digest/signature, explicitly trusted source for this mode, unchanged mandatory capabilities/data scope/dependency access, successful migration/conformance preflight and a signed security advisory. A publisher's free-text “security” label is insufficient. Whether such an update may automatically change managed business logic *inside already granted capabilities* remains a product decision; Core authorization and irreversible-effect confirmation remain mandatory. Anti-rollback protection against stale packages is distinct from the rejected post-activation downgrade flow.

## Negative and positive fixtures to prototype

| ID | Input | Expected boundary, independent of chosen format |
|---|---|---|
| PKG-T01 | Unknown root, invalid threshold signature, expired timestamp, stale root/targets or mixed snapshot | No activation or active-state mutation; typed reason. |
| PKG-T02 | Wrong digest, corrupt blob, missing dependency or unsupported media/capability | Explicit integrity/compatibility failure; no partial migration. |
| PKG-T03 | Publisher key for package A publishes package B; delegated key revoked | No unauthorized namespace release or automatic trust transfer. |
| PKG-T04 | Connect source → discover → download → provision → activate | Each transition separately observable; no transition implicitly grants Space access. |
| PKG-T05 | `compatible-auto`, `security-auto`, `manual`, `pinned` with deceptive security label, new data scope or breaking migration | Auto-ineligible cases remain on the active version; first activation failure remains inactive. |
| PKG-T06 | Trusted update changes managed logic within its current capability set but attempts a denied command or irreversible effect | Trusted runtime still denies forbidden command and applies separate effect confirmation. Eligibility for automatic activation remains open. |
| PKG-T07 | iOS declarative package vs downloaded Rhai/Wasm/JavaScript payload | Built-in-capability declaration may run; code-bearing payload fails Release-1 iOS profile. |

[Apple Guideline 2.5.2 and 4.7](https://developer.apple.com/app-store/review/guidelines/) distinguish downloaded functionality from a separately regulated HTML5/JavaScript mini-app profile. [Android dynamic-code guidance](https://developer.android.com/privacy-and-security/risks/dynamic-code-loading) and [OWASP MASWE-0049](https://mas.owasp.org/MASWE/MASVS-CODE/MASWE-0049/) reinforce verification of code origin/integrity and limiting executable downloads; [OWASP supply-chain guidance](https://cheatsheetseries.owasp.org/cheatsheets/Software_Supply_Chain_Security_Cheat_Sheet.html) supports signed components and provenance. None of these sources grants VIDA an App Store approval or selects a mobile code tier. Release-1 iOS stays declarative on built-in capabilities.
