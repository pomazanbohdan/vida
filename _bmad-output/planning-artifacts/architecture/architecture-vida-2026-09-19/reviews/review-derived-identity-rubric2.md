# Independent rubric review — derived resource and corporate identity

**Verdict:** Conditional pass for the architecture spine as a prototype-gated build substrate. The two newly accepted product invariants are represented and neither directly contradicts ADR-0004, ADR-0012 or REQ-EFFECT-007. One high and one medium semantic ambiguity should be tightened before the relevant independent implementations begin; the existing OQ-0048/OQ-0049 gates correctly prevent silently filling in their mechanisms.

## Scope and source check

- Reviewed `ARCHITECTURE-SPINE.md`, especially Inherited Invariants, AD-3–5, AD-13, Consistency Conventions, Implementation Gates and Deferred.
- Compared with accepted `docs/03-architecture/decisions/ADR-0004-multi-axis-identity-model.md`, `docs/03-architecture/decisions/ADR-0012-command-event-sync-boundary.md`, and approved `docs/02-requirements/effect-execution-constraints.md` (`REQ-EFFECT-007`).
- Supporting identity contract `docs/04-specifications/identity-domain-contract.md` (`SPEC-ID-007`–`009`) confirms the work/private isolation interpretation. This review does not select OQ-0048's algorithm or OQ-0049's controller.

## Findings

### R1 — High — AD-13 does not define which acceptance boundary makes a source fact eligible to create a shared durable resource

**Evidence.** REQ-EFFECT-007 explicitly says one *confirmed* source fact leads to one resource. AD-13 says “one committed fact leads an AppInstance rule to create a durable Space resource”, while AD-4 treats the origin's atomic log+outbox commit as `delivery.accepted` and AD-5 correctly says this is **not** `authority.accepted`. ADR-0012 separates committed-fact reactions from receive/apply and prohibits creating a new business intent merely because state arrived.

**Divergence.** A client may treat origin-local commit as the confirmed source and generate a resource while another waits for Space authority acceptance. If the source is rejected or its causal prerequisites fail, the first implementation has an orphan resource and the second does not. That cannot be repaired merely by converging on one resource ID.

**Disposition: discuss / then autofix.** Make the AD-13 eligibility rule refer to source-fact confirmation under the selected Space authority policy, distinguish it from `delivery.accepted` and local projection visibility, and require the OQ-0048 gate to test rejected/pending source facts as well as duplicate receive and mixed versions. Keep the acceptance protocol in OQ-0033; do not invent it here.

### R2 — Medium — AD-3's persistent federated endpoint rule needs an explicit cross-account unlinkability qualification

**Evidence.** ADR-0004 and `SPEC-ID-007` prohibit disclosing a link between a separate corporate Persona/account and the user's private/anonymous Persona; `SPEC-ID-009` isolates visible Spaces by active account and grants. The spine's inherited table says the work context does not reuse unrelated private Personas, but AD-3 permits persistent ordinary/federated device endpoints and calls out isolation only for anonymous/session contexts. The `Conformance Plane` has broad privacy fixtures, but no specific work/private co-location fixture is named.

**Divergence.** A transport implementer could reuse one persistent endpoint or observable binding for a private and corporate Persona in the same `LocalVault`, while UI/account implementers correctly isolate their Space lists. That leaks the account relationship on wire despite apparently correct local account switching.

**Disposition: autofix.** Clarify in AD-3 that persistent endpoint reuse across otherwise-unlinked Personas/account contexts is prohibited whenever it would disclose co-location, and add a conformance fixture proving corporate suspension does not affect unrelated grants or reveal their shared vault/network identity. Do not decide corporate Persona controller or recovery; OQ-0049 remains open.

## Good-spine checklist

| Check | Result |
|---|---|
| Real initiative-level divergence points fixed | Pass, subject to R1–R2. AD-13 fixes the one-resource outcome and AD-11–14 retain one core/runtime path. |
| Rules enforce their stated prevention | Mostly pass. OQ-0048 properly blocks a guessed derived-resource algorithm; R1 identifies the missing eligibility predicate. |
| Deferred items cannot leak silent incompatible choices | Pass for the new mechanisms because OQ-0048 and OQ-0049 are paired with explicit implementation gates. |
| Inherited accepted decisions preserved | Pass. AD-13 does not authorize `sync.apply` to rerun commands; the spine inherits ADR-0004's separate Persona/account and limited suspension. |
| Capability/spec coverage | Pass for REQ-EFFECT-007's single logical resource; corporate account isolation represented, with the transport privacy loophole in R2. |
| Platform/operations envelope | Pass at this altitude: AD-9 and AD-15 plus prototype gates make runtime asymmetry and operational ownership explicit. |
| Technology version/currentness | Outside this focused delta review; the spine pins Iroh 1.2.0 as a decision baseline and blocks production without exact release pins. |

## Non-findings / boundaries

- The spine correctly does **not** turn delivery ACK or federation hosting into domain authority.
- It correctly does **not** pick a deterministic ID, claim/lease, single executor, or merge algorithm for OQ-0048; choosing one in this review would exceed the accepted requirement.
- It correctly leaves controller and recovery of the separate corporate Persona to OQ-0049 while already fixing the blast radius of corporate account suspension.
