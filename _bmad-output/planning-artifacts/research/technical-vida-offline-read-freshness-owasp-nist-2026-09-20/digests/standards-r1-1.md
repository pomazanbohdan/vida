# Digest — official security guidance

Accessed: 2026-09-20. Search surface: official OWASP and NIST web documentation. Publisher dates are not displayed for OWASP web pages; NIST SP 800-63B-4 was published July 2025.

| Claim | Source | Publisher | Published | Confidence | Class |
|---|---|---|---|---|---|
| After session termination a mobile app should make sensitive data inaccessible, invalidate sessions, and clear tokens, cached personal data, memory and screen content. | [1] https://mas.owasp.org/MASWE/MASVS-AUTH/MASWE-0024/ | OWASP MAS | undated | medium: direct primary source | mobile-session |
| OWASP web-session expiry uses idle and absolute timeouts based on application/data risk, enforced server-side; client-only time checks are manipulable. | [2] https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html | OWASP Cheat Sheet Series | undated | medium: direct primary source | web-session |
| NIST SP 800-63B-4 recommends authenticated-session overall timeout no more than 30 days at AAL1 and 24 hours at AAL2, with AAL2 inactivity timeout no more than one hour. These are reauthentication/session limits. | [3] https://pages.nist.gov/800-63-4/sp800-63b/aal/ | NIST | 2025-07 | high: final normative primary source | auth-session |
| MASVS-STORAGE-1 requires secure storage of sensitive data but does not itself prescribe an offline-read expiry duration. | [4] https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/ | OWASP MAS | undated | medium: direct primary source | mobile-storage |

Inference from [1]–[4], not a quotation from any standard: none of the reviewed official controls states a universal `N hours/days` at which an offline device must stop reading already-decrypted, locally synchronized Space data. NIST authentication-session numbers must not be copied into VIDA membership-freshness policy as if they were the same control.

Negative search: official OWASP pages reviewed did not contain a prescriptive timeout for remote membership revocation reaching a device that never reconnects. Such a device cannot receive an updated control state while disconnected; a local freshness timer can restrict a compliant client but cannot be enforced by the remote authority against a modified client or an old copy.
