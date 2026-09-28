# Durable effects, retries and ownership — round 1

| Claim | Source | Publisher | Pub date | Accessed | Confidence / class |
|---|---|---|---|---|---|
| Transactional outbox couples domain commit with pending event, but downstream processing may deliver duplicates; consumer idempotency remains necessary. | https://docs.aws.amazon.com/prescriptive-guidance/latest/cloud-design-patterns/transactional-outbox.html | AWS | undated | 2026-09-19 | high; reliability pattern |
| Temporal Activity tasks can be retried after worker crash/timeout even if the function began; a workflow engine does not by itself prove an external effect happened only once. | https://github.com/temporalio/documentation/blob/main/docs/encyclopedia/activities/activity-execution.mdx | Temporal | current main, date not stated | 2026-09-19 | high; reliability pattern |
| Google Spanner explicitly notes external API calls cannot be atomically coupled with queue acknowledgement and require application idempotency/reconciliation. | https://docs.cloud.google.com/spanner/docs/queues/queues-at-most-once | Google Cloud | undated | 2026-09-19 | high; reliability pattern |
| Stripe persists the response for an idempotency key; keys may be pruned after 24h, so the external provider's retention window is part of the effect guarantee. | https://docs.stripe.com/api/idempotent_requests | Stripe | undated | 2026-09-19 | high; external API contract |
| Kubernetes client-go leader election does not guarantee fencing: more than one client may act as leader. A lease/election alone is insufficient to prevent duplicate effects. | https://pkg.go.dev/k8s.io/client-go/tools/leaderelection | Kubernetes client-go | current docs, date not stated | 2026-09-19 | high; coordination limitation |

Leads: derive stable effect ID from event + rule/version + recipient/channel, record intent before dispatch, require destination idempotency or reconcile unknown result. Exact key schema is a VIDA design proposal, not a fact supplied by a reference.

Not found: a general guarantee of exactly-once external effects across all third-party APIs.
