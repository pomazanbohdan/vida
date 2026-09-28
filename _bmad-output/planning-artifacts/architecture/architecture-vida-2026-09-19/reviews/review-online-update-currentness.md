# Technology/currentness review — online-first/update batch (2026-09-21)

Verdict: PASS; no critical/high finding remains.

- Android official docs support user-visible foreground services but Android 15 limits `dataSync` foreground-service runtime to six hours per 24 hours for target API 35; the spine therefore does not promise permanent Android execution.
- FCM high-priority delivery is bounded and policy-sensitive; it is documented as a wake hint, not a liveness receipt.
- Apple official docs state background apps are typically suspended and background tasks/push are system-controlled; the spine correctly requires reconciliation instead of 100% availability.
- Lamport and Matrix support causal/partial ordering plus deterministic linearization; millisecond wall-clock order is not treated as proof.
- TUF supports signed freshness/version metadata and rollback/freeze/mix-and-match checks; OCI Distribution supports content-addressed manifest/blob retrieval. Neither is misrepresented as a migration engine.
- Apple App Review Guideline 2.5.2 remains an explicit production gate for downloaded executable AppPackage logic.

Sources: Android foreground services and Android 15 behavior changes; Firebase message priority; Apple BackgroundTasks/background execution/App Review; Lamport 1978; Matrix specification; TUF specification; OCI Distribution specification.

