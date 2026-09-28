# Adversarial independent-build review — derived resources and corporate identity

**Verdict:** Conditional pass for the existing prototype-gated spine; **not ready for independent production implementations** of fact-derived shared resources or corporate Persona lifecycle. The former is recognized by `OQ-0048`, but the gate wording can admit a premature single-device implementation; the latter is recognized by `OQ-0049` without an equivalent implementation gate. No finding below asks the spine to choose an algorithm while those questions remain open.

## High 1 — A single-device label can bypass the derived-resource gate

**Binding text:** `AD-13` requires one logical Space resource from one fact and leaves the canonical creation path to `OQ-0048`; the implementation gate blocks only “Multi-device rules that create durable shared resources.” `ADR-0012` separately forbids `sync.apply` from reissuing the originating command. `REQ-EFFECT-007` approves the one-resource result, not its algorithm.

**Two compliant units:** Unit A initially runs a rule on one device and deterministically materializes a note from `(source fact, AppInstance, rule)` as a rebuildable projection, with no second signed operation. Unit B initially runs the same class of rule on one elected device and commits a distinct signed note-creation operation with an authority-issued ID. Each can converge to one note in its homogeneous deployment, treats the committed-fact reaction separately from `sync.apply`, and does not violate an adopted creation algorithm because none exists. A mixed-version Space later combines the implementations: A displays its projected note plus B's operation-created note, or cannot edit B's note under the same resource semantics. The “single-device” initial deployment can be argued not to meet the gate's literal “Multi-device rules” condition even though the durable resource is shared and later replicated.

**Minimal fix:** Change the gate subject from “Multi-device rules” to **every AppInstance rule that creates a durable shared Space resource from a committed fact**, even if initially evaluated on one device. Close `OQ-0048` before that implementation with one normative creation model, resource/operation identity and provenance rules, authority/dedupe behavior, and mixed-AppInstance-version conformance fixtures. Do not solve it by allowing `sync.apply` to issue a fresh business command.

## High 2 — Corporate Persona controller and offboarding remain independently choosable without a gate

**Binding text:** `ADR-0004` permits a separate corporate Persona plus node `ServiceAccount`, forbids silent replacement of its controller, and protects unrelated Personas and Spaces. It expressly defers the corporate Persona controller, recovery and portability to `OQ-0049`. The spine inherits account isolation and lists `OQ-0049` under Deferred, but its implementation gates do not block corporate enrollment/controller or recovery implementations.

**Two compliant units:** Unit A explicitly creates a distinct company-managed Persona whose controller/recovery authority is the company node; blocking the company account terminates company grants, and separate policy may retire that corporate Persona. Unit B explicitly creates a distinct user-controlled Persona, binds it to the company `ServiceAccount`, and lets it remain cryptographically usable or migrate after company grants are revoked. Neither reuses a private Persona, revokes an unrelated Persona, or silently transfers control. Yet the same employee/offboarding flow yields incompatible signing, recovery, contact continuity and portability behavior. A device or service built against one cannot safely infer the other's authority from `ServiceAccountId` or `PersonaId` alone.

**Minimal fix:** Add an explicit production/independent-build gate for corporate Persona enrollment, controller changes, account suspension and recovery until `OQ-0049` specifies controller authority, key custody, transition evidence, recovery, portability and exact grant/session termination. Preserve the accepted invariant that blocking the corporate account does not affect unrelated Personas/Spaces; do not silently make the federated node root identity authority.

## High 3 — Account switching governs the Space list, but not every local read surface

**Binding text:** `ADR-0004` requires the *visible list of work Spaces* to follow the active account context and grants; `AD-11` leaves notifications/presentation to shells and `AD-13` leaves projections to runtime. No sentence makes the account context a mandatory selector for all read APIs and surfaced derived data.

**Two compliant units:** Unit A filters only the Space picker when switching from company to private Persona, while a shared local full-text index, recent-items list and notification previews can still surface company titles under the private context. Unit B scopes each read and presentation surface to the active Persona/binding/grants. Both keep distinct Persona IDs and can obey the literal Space-list rule. The result differs on confidentiality and offboarding visibility, especially on a shared device. This is not a claim that previously replicated plaintext can be remotely erased; `ADR-0003` already denies that guarantee.

**Minimal fix:** State one cross-layer invariant: every normal read/projection/search/recent/notification/export surface must evaluate the active Persona/account context and current grants before exposing resource data; any cross-account aggregate is an explicit, separately authorized mode. On account suspension, invalidate or lock that context's indexed/presented views, without promising physical erasure of old plaintext. Add a two-Persona account-switch/revocation fixture. The physical local cache/retention policy may remain separate.

## Scope and non-findings

- `OQ-0048` and `OQ-0049` are legitimate open decisions. The review does not recommend picking deterministic IDs, server execution, or company ownership by implication.
- `ADR-0012`'s receive/apply boundary is coherent; the incompatibility is in the still-open *committed-fact creation* contract, not permission to rerun a command on sync replay.
- Domain authority, same-Persona status conflicts and booking confirmation are already called out by `OQ-0033`/`OQ-0034` and existing implementation gates, so this review does not duplicate them.
