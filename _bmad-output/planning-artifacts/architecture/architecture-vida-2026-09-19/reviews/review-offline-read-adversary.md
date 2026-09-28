# Adversarial review — shared Space offline-read freshness

Reviewed 2026-09-20. Scope: Architecture Spine AD-16, ADR-0003 §6–7, REQ-ACL-016/018, SpaceMembership offline-read contract, OQ-0053. Fixed product outcome: shared `standard` has a strict `< 7 days` offline-read maximum, `protected` strict `< 24 hours`, `critical` needs online validation for a *new open*; received effective revocation locks immediately; Personal Space has a separate default. Verdict: **the numerical ceilings and revocation precedence agree, but risk-class inheritance and the effect of a read lock on still-valid offline mutations need normative treatment; proof/restart and already-open critical behavior are correctly marked as OQ-0053 gates.**

## Finding 1 — effective risk class for nested and linked data is unspecified (high; product/security divergence)

- Evidence: AD-16 and ADR-0003 require one Space/scope profile across Apps and allow lower-level policy to tighten, but do not define how to derive the effective class for a resource, linked item, attachment, search hit or composite screen when parent Space is `standard` and a nested policy or schema marks data `protected`/`critical`.
- Two implementations: A uses the enclosing Space's `standard` timer for a note and its attachment, allowing both after 25 offline hours. B applies `protected` to the attachment and hides it at 24 hours while keeping the standard note visible. Both can cite “corresponding Space/scope” if the attachment-to-scope/class mapping is not canonical. A may expose data intended to be protected.
- Required resolution: define canonical risk-class derivation and inheritance (likely most restrictive applicable Space/scope/resource policy), independent authorization for linked content, and how a mixed screen is redacted rather than wholly opened by its least restrictive component. Fixtures must cover nested scopes, cross-App links, attachments, search/recents and policy changes while the client is offline.

## Finding 2 — read expiry versus offline mutation lease has an observable gap (medium; product decision)

- Evidence: ADR-0003 and REQ-ACL-016 explicitly separate read limits (`7d/24h/online-open`) from mutation acceptance windows (`30d/7d/24h`). At read expiry the client must block *reading* on all managed surfaces, but neither source says whether it can create/queue a new mutation without viewing existing Space data.
- Two implementations: A blocks all Space interactions after a standard read gate expires on day 7. B hides prior messages/notes but still lets a user compose a new message or create a new note and queues the operation under the 30-day standard mutation lease. Both preserve the two separate maximum windows, yet users and downstream accepted operations differ.
- Required resolution: explicitly select whether read-gate expiry blocks only disclosure or also new local commands; if blind creation is allowed, define minimal UI/projection that cannot reveal stale Space state and require execution-time authority acceptance on reconnect. Never silently use the mutation lease to extend reading.

## Finding 3 — restart/clock proof can produce early lock or apparent extension (OQ-0053 implementation gate)

- Evidence: ADR-0003 forbids client-clock rollback, restart, local activity and old-head replay from *renewing* freshness, but leaves the freshness proof and time source open. AD-16 blocks production enforcement until OQ-0053 is closed.
- Two candidate implementations at offline restart after 2 hours: A fails closed because it cannot prove elapsed time across reboot; B resumes access using a protected persisted authority receipt plus rollback-resistant elapsed-time source. Both stay within the 24-hour protected ceiling, but availability differs. A naive third implementation resetting the timer on restart or trusting a replayed head would be **noncompliant**, not an allowed option.
- Gate requirements: define the proof tuple (authority identity, Space/scope, control frontier, attested freshness time/age, policy version), trusted time strategy per platform, recovery when trusted time is unavailable, and crash/replay fixtures. State whether early fail-closed behavior is acceptable or whether the promised offline window requires availability when age can be established.

## Finding 4 — already-open `critical` content is deliberately unresolved (OQ-0053 product gate)

- Evidence: ADR-0003 explicitly limits online validation to a *new opening* and says an already-open critical screen after link loss is undecided. AD-16 repeats that gate.
- Two implementations: A immediately obscures an already-open critical document when connectivity drops. B allows the existing in-memory view until navigation/timeout but refuses any new open. Both satisfy the current new-open rule. OQ-0053 must decide background/foreground, scrolling to uncached content, embedded links, export and screen capture behavior; neither option should be marketed as accepted yet.

## No divergent compliant implementation on these points

- A verified, effective `MemberRemoved` delivered to a compliant client overrides any remaining 7-day/24-hour window and locks all managed reads immediately. Continuing to show data until lease expiry would violate ADR-0003 §6–7 and REQ-ACL-016. The implementation gate should require durable control-head/revocation persistence before later reads, including after a crash, so a restart cannot resurrect a previously received grant.
- The strict boundary is `< 7 days` and `< 24 hours`; an implementation that allows reading at exactly 7 days/24 hours is noncompliant. The 30-day/7-day/24-hour *mutation* windows are distinct and cannot reset or extend those read ceilings.
- A disconnected device that has not received the revocation cannot be remotely wiped; this is an explicit security limitation, not a reason to loosen the local gate once the event arrives.
