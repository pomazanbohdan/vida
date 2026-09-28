# Citation audit — offline-read freshness report

Audit date: 2026-09-20. Scope: only `../research.md` and its four cited official URLs. All four URLs opened directly; the NIST final-publication metadata was also checked against the official [CSRC record](https://csrc.nist.gov/pubs/sp/800/63/b/4/final). No project specifications were inspected.

## Verdict

**Conditional pass.** The main separation between session timeout, Space-membership freshness and secure local storage is evidence-aligned. The report should fix one citation mismatch and label the OWASP-to-`MemberRemoved` transfer as an inference. NIST AAL figures are correct if `SHOULD` remains a recommendation, not a mandatory maximum.

## Source-by-source checks

| Ref | Direct official support | Boundary |
|---|---|---|
| [1] [OWASP MASWE-0024](https://mas.owasp.org/MASWE/MASVS-AUTH/MASWE-0024/) | States sensitive data must be inaccessible after **session termination**; mitigations include server/client invalidation, clearing session tokens/cached personal data/in-memory/on-screen content, and risk-appropriate inactivity/absolute timeouts (page's Overview/Mitigations). | Does **not** mention Space membership, `MemberRemoved`, future key epochs or a universal offline-read age. Mapping membership removal to a local session/context termination is a reasonable VIDA design inference, not a direct OWASP rule. |
| [2] [OWASP Session Management Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html) | Session Expiration section says web-session idle/absolute timeouts depend on application/data criticality; server-side expiration is mandatory, and client-controlled time references can be manipulated. | Applies to web-session management; its example minute/hour values are not a local-first Space-membership TTL. It does not itself prove `MemberRemoved` semantics or new-key refusal. |
| [3] [NIST SP 800-63B-4 AAL](https://pages.nist.gov/800-63-4/sp800-63b/aal/) | Normative AAL section: periodic subscriber-session reauthentication **SHALL** occur and a definite overall timeout **SHALL** be established; the AAL1 limit **SHOULD** be ≤30 days, AAL2 **SHOULD** be ≤24 hours, AAL2 inactivity **SHOULD** be ≤1 hour. The [final CSRC record](https://csrc.nist.gov/pubs/sp/800/63/b/4/final) dates the final publication July 2025. | These are authenticated-session reauthentication targets, not a Space-membership freshness or offline-read TTL. The report's `рекомендує` wording is accurate; never present the numeric `SHOULD` figures as unconditional `SHALL` ceilings. |
| [4] [OWASP MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) | Directly states that the app securely stores sensitive data regardless of storage location. | No offline-read expiration. The page does not prescribe a specific cache-encryption/key-management implementation; the stated benefit of encryption is an engineering inference, not a verbatim control. |

## Findings to fix in `research.md`

1. **High — unsupported citations in the first guarantees row (`research.md:35`).** `[1][2]` support ending a session and server-side timeout enforcement, but not the stronger VIDA-specific promise that Space authority immediately rejects operations under revoked grants and issues no new keys. Attribute that row to the project's proposed/accepted protocol contract without OWASP citation, or add a directly relevant source; keep the OWASP links only for analogous session-handling behavior.
2. **Medium — transfer from session termination to membership removal is stated as direct support (`research.md:26`).** Replace “Це прямо підтримує” with “Це узгоджується за аналогією; застосування до `MemberRemoved` — рішення VIDA.” OWASP [1] is not a Space-membership specification.
3. **Medium — absence of universal TTL is a bounded inference (`research.md:20,26,29`).** The current opening “у перевірених матеріалах” is appropriately scoped. Retain that qualifier; do not generalize to “OWASP/NIST nowhere specify an offline-read TTL” without a broader standards review. NIST [3] is about AAL sessions, so non-applicability to Space membership is an inference from its stated scope.
4. **Low — NIST review date arithmetic (`research.md:61`).** The final CSRC history says 2025-07-31. If the stated 24-month window is literal, it reaches 2027-07-31, not 2027-07-01. Keep 2027-07-01 only if explicitly called an intentionally early, conservative checkpoint.

## Positive checks

- The candidate VIDA durations (7 days / 24 hours / online validation) are explicitly labeled product risk-budget, not OWASP or NIST mandates (`research.md:43`).
- The report correctly says biometric local unlock cannot prove current remote Space membership without receiving new control state; it labels this as a VIDA inference (`research.md:28`).
- The four cited links are official, live pages. The NIST July 2025 date and AAL1/AAL2 figures match the final NIST text.
