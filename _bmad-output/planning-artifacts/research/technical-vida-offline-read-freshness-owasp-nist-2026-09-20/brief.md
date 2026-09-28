---
topic: VIDA offline read freshness and revocation
type: technical
decision: Whether a shared Space should stop displaying synchronized data after a device remains offline without a fresh membership control head
status: researched
created: 2026-09-20
---

# Research brief

Question: Does OWASP prescribe a time limit for local reading of already-synchronized data after a Space Owner removes a member while that member's device is offline? Which controls govern session termination, cached data and server-side revocation? Compare current NIST authentication-session guidance without treating its session timeouts as offline-read expiry.

Scope: official OWASP MASWE/MASVS and Cheat Sheet Series; current NIST SP 800-63B-4. Decision shape: explore, focused one-pass research. Output distinguishes source requirements, engineering inference and proposed VIDA policy. Existing VIDA mutation-acceptance windows are not evidence for an offline-read timeout.
