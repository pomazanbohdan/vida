# Rubric review — shared Space offline-read freshness

Verdict: **product rule is coherently propagated; production enforcement remains correctly gated.** AD-16, ADR-0003 §7, REQ-ACL-016, SpaceMembership and resolved OQ-0051 agree on shared-Space maximums: `standard` `< 7 days`, `protected` `< 24 hours`, `critical` online validation before newly opening content. The clock starts at authority-confirmed control-state freshness, not app activity or an old peer head. Personal Space keeps its separate offline-read default; read freshness is distinct from offline mutation acceptance. OQ-0053 and spine implementation gate correctly leave proof/time mechanics and an already-open critical screen unresolved.

## Findings

1. **P1 — “New opening” may miss secondary critical-content reads.** ADR-0003 line 127 and REQ-ACL-016 line 107 require online validation for a new opening of `critical` content, while AD-16 says the same profile applies to every managed read surface. Search results, recents, notification previews, linked-resource snippets, export and AppInstance APIs can reveal critical fields without an explicit screen-open event. Define a new opening/read as any first materialization of critical managed content on a surface, or ensure those surfaces cannot display protected values offline. Keep *already-open screen after connectivity loss* as OQ-0053 rather than silently deciding it here. Add cross-app/API preview fixtures.

2. **P1 — Longer write leases create an internal-read boundary.** ADR-0003 keeps `standard` offline mutation acceptance at 30 days and `protected` at 7 days, while offline reading locks at 7 days/24 hours. This is explicitly intentional separation, not a numeric contradiction. But an AppInstance rule, plugin or background task could still inspect cached Space data after the local read gate closes and emit a new mutation or notification. Clarify whether the freshness gate applies to all consumer reads in `vida-core`/`vida-runtime`, with sync/maintenance processing separately classified, or whether some blind/background writes may continue. Acceptance evidence should assert that no app/API path reveals or derives inaccessible content after read expiry.

3. **P2 — Clock and freshness proof is a real platform gate, not an implementation detail.** ADR-0003 line 129 and REQ-ACL-016 line 109 say clock rollback/restart cannot extend the window, but the proof source and tamper-resistant elapsed-time handling are not yet defined for mobile, desktop and browser. OQ-0053 and spine line 209 correctly block production. The contract should require fail-closed behavior when a compliant client cannot establish trusted age, plus tests at `T−ε` and `T`, rollback, restart, restored app state, stale-head replay and online renewal on each supported platform. Do not imply a modified/offline device is remotely enforceable.

4. **P2 — Risk-class attribution needs a conformance case.** The documents refer to one Space/scope profile, but a shared Space can contain linked content in a stricter child scope. A standard list/search surface must not leak a protected or critical child item's title/snippet merely because the parent remains readable. The effective read limit should follow the displayed datum's scope/risk class, including derived projections and cross-App links. Add fixtures for mixed-class Space views; this tightens enforcement without changing the accepted 7-day/24-hour/online thresholds.

## Positive trace

- Exact `<` wording means access ends at the 7-day and 24-hour boundary, not after it; ADR and spec both include boundary acceptance evidence.
- Expiry suspends managed reads but does not delete membership, keys or data; received effective revocation still locks immediately.
- Space/scope policy may tighten but not extend the platform maximum, and cannot turn `critical` into offline-open.
- The documents do not falsely attribute the chosen numeric durations to OWASP or promise remote wipe of disconnected devices.
