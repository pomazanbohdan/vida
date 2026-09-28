# Local-first executor and offline behavior — round 1

| Claim | Source | Publisher | Pub date | Accessed | Confidence / class |
|---|---|---|---|---|---|
| Automerge stores documents locally and later syncs offline changes; synchronization is about convergent document state, not a job-executor guarantee. | https://automerge.org/docs/tutorial/network-sync/ | Automerge | undated | 2026-09-19 | high for sync behavior; pattern |
| Apple background push is low-priority, may be throttled, and delivery is not guaranteed; a mobile client cannot be assumed to wake on schedule for every effect. | https://developer.apple.com/documentation/usernotifications/pushing-background-updates-to-your-app | Apple | undated | 2026-09-19 | high for iOS limitation; platform |
| Signal keeps personal message content/keys on devices and sends E2EE archives through servers that cannot read them; an encrypted relay is not automatically a plaintext automation worker. | https://signal.org/blog/a-synchronized-start-for-linked-devices/ | Signal | 2025-01-27 | 2026-09-19 | medium; privacy pattern |

Leads: distinguish originating device ownership from replicated pending intent; test lost-originator handoff. Android constraints can be checked if implementation timing requires platform-specific background SLA.

Not found: a local-first library guarantee of exactly-once external effects from state sync alone.
