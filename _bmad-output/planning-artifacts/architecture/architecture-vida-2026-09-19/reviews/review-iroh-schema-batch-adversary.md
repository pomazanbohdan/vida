# Adversarial review — Iroh/schema batch (2026-09-21)

Verdict: PASS after corrections; no remaining critical/high findings.

Production gates or normative rules now cover:

- request-ID-bound terminal outcome resolver and concurrent outcome conflict;
- schema activation/migration/rollback and late old-version writes;
- origin attachment atomicity and inbound failure without remote GC;
- profile-count correlation, enumeration and replayed/revoked lease tests;
- agent conflict-context consent without a universal conflict permission;
- chat publication proof under skew/replay/reordering;
- causal-frontier correction assignment and multi-branch fallback.

