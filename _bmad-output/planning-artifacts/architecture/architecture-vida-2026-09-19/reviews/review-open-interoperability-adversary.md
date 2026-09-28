---
review: open-interoperability-adversary
artifact: ../ARCHITECTURE-SPINE.md
focus: AD-17, ADR-0013, REQ-OPEN-001–006
date: 2026-09-20
verdict: architecture-direction-pass-production-interoperability-gated
---

# Adversarial review — two compliant independent implementations

## Gate verdict

The accepted direction is sound: open normative contracts for independent clients, nodes, Apps and bounded extensions, with paid implementation allowed behind public interoperability interfaces. The current text does **not** yet establish an enforceable, vendor-neutral production interoperability claim. The initially observed waiver/plugin contradictions were fixed during this review; the remaining claim-boundary gaps permit two ostensibly compliant implementations to fail together. The published-production claim is already gated by `OQ-0055` and concrete specs/fixtures; preserve that gate and resolve the high findings inside it. This review does not propose a new wire codec, license, federation admission policy, or unrestricted plugin ABI.

## Critical finding resolved during review

### C1 — AD-11 previously required a VIDA waiver for an alternative semantic implementation [RESOLVED]

**Original evidence:** AD-11 required an alternative semantic implementation to obtain a time-bounded waiver ADR and differential testing against the Rust reference. AD-17, ADR-0013 decision 1–2 and `REQ-OPEN-001/004/006` promise a documented independent path without private prerequisite.

**Counterexample:** team C builds a clean-room client in Kotlin; team N builds a clean-room node in Go. Both implement the public profile and pass every public vector. Neither has a VIDA-issued waiver or the Rust-reference differential harness. Under AD-11 they are noncompliant; under AD-17 they are compliant. A vendor decision, not the public contract, determines eligibility.

**Resolution verified:** AD-11 now scopes the Rust-core and waiver requirements to first-party releases and says independent third-party client/node implementations need no VIDA waiver. AD-17 now repeats the independent path. The contradiction is closed; the remaining public conformance details below still matter.

## High — incompatible but facially compliant builds

### H1 — No registered minimum baseline or exact claim subject

**Evidence:** AD-17 applies to every **claimed** baseline; ADR-0013 accepts one published protocol profile; `REQ-OPEN-001` says every claimed interoperable profile needs a spec. None names the required v1 client↔node profile set, mandatory operations/capabilities, role of optional/premium features, or whether a claim applies to a transport-only, read-only or full workflow profile. `OQ-0012`, `OQ-0028` and `OQ-0037` remain open.

**Counterexample:** client C advertises `vida/messages/1` send/read; node N advertises a `vida/messages/1` archive-only subset plus a proprietary send capability. Each publishes its own versioned profile and fixtures and honestly passes its own tests. Both can claim “VIDA interoperable” yet share no usable send path. Equally, a paid extension can carry the only implementation of a feature while the free profile is defined so narrowly that the advertised open-core workflow is not independently realizable.

**Disposition — discuss/gate:** define a public, immutable profile registry with exact `ProfileId`, revision/digest, required and optional capabilities, dependency closure, claim vocabulary and a minimum user-visible baseline per release. `OQ-0037` may own activation; `OQ-0055` may own publishing authority, but neither should leave baseline scope to the claimant. Explicitly mark premium-only features optional or a separate profile, and specify behavior when they are absent.

### H2 — Reference interoperability is weaker than independent↔independent interoperability

**Evidence:** `REQ-OPEN-001` verification asks an independent implementation to interact with the official client and node. ADR-0013 asks each external client/node to pass public fixtures. The text does not explicitly require a C×N cross-vendor pair or symmetric request/response/error behavior.

**Counterexample:** official node accepts both encodings of an optional field and official client tolerates both error styles. Independent client C emits only encoding A and rejects error B; independent node N emits only B and rejects A. Each passes against the permissive official counterpart and finite golden vectors, but C and N fail together. The same asymmetry can occur in retries, unknown mandatory features and mixed-version rollback.

**Disposition — autofix/fixture:** make the claim require pairwise client↔node and node↔node test scenarios across at least two genuinely independent implementations, including adverse/mixed-version cases, not only official-reference tests. Normative behavior, not whichever behavior the reference happens to accept, must decide failures. This supplements, not replaces, `OQ-0028` and `OQ-0037`.

### H3 — “App/plugin” previously risked implying an arbitrary executable ABI [RESOLVED IN SPINE]

**Original evidence:** AD-17 used unqualified “App/plugin implementers.” ADR-0007–0009 only accept declarative `AppPackage` plus managed logic through controlled host APIs; `SPEC-APP-PACKAGE-001` explicitly excludes unrestricted OS/core access and defers exact executor, hook phases and sandbox.

**Counterexample:** host H advertises conformance for schema packages and controlled hooks; plugin P claims conformance as a native/Dart/Wasm binary with direct filesystem/network access because “plugin” and “same documented path” appear unqualified in AD-17. Both cite accepted architecture but cannot execute together, and P violates the security model H assumed.

**Resolution verified:** AD-17 now explicitly defines the plugin as a managed extension under ADR-0007–0009, not arbitrary host code. Full executor, hook, effect and package trust contracts remain gated by `OQ-0040`, `OQ-0041`, `OQ-0043`, `OQ-0045` and `OQ-0048`.

### H4 — Equal API names do not define equal entitlement, execution or effect semantics

**Evidence:** `REQ-OPEN-003` says the format and allowed APIs are public and equal for bundled/external packages, while `SPEC-APP-PACKAGE-001` leaves hook phases, base-handler continuation, side effects, activation and update lifecycle open. AD-17's current production gate mentions “relevant package contracts” without naming these decisions.

**Counterexample:** third-party extension P passes its package schema suite and calls a published `notes.save` hook. VIDA host H1 executes P before the base handler; independent host H2 executes it after base commit. Both expose the same API name and enforce Space grants, yet notes persist differently. Another pair can treat a paid `automation.execute` capability as optional versus indispensable to a bundled package, so an independently built host cannot run the official default Space.

**Disposition — gate:** before a production App/plugin interoperability claim, require a versioned execution profile with hook phases, continuation/fallback, transaction/effect idempotency, activation permissions, supported capability/entitlement matrix and mixed-version behavior. Test the **same external package** on at least two independent conforming hosts and the same bundled package under the external package path. Where paid capability is absent, return an explicit portable unsupported/entitlement state without corrupting shared data.

## High — security, privacy and open-core boundary

### H5 — Package conformance must not be mistaken for publisher trust or Space consent

**Evidence:** ADR-0013 correctly says conformance does not grant Space rights or curated-marketplace admission; ADR-0008 separates discovery from activation; `SPEC-APP-PACKAGE-001` defers source signing, sandbox and trust. `REQ-OPEN-003/004` demand a public package suite but do not explicitly distinguish semantic conformance from safe installation/execution.

**Counterexample:** package P is byte-valid and passes API tests. An independent client installs it from an external repository and treats the conformance badge as permission to activate it in every Space. P then reads authorized but unrelated notes through a broad host API and sends contents through an allowed external-effect API. A second client requires per-Space consent and egress grants. Both can claim package conformance; only one preserves the intended privacy boundary.

**Disposition — security gate:** publish separate verdicts for `format/behavior conformance`, `publisher/provenance trust`, and `Space-scoped activation + least-privilege consent`. Before external-package production, specify network/side-effect egress capability, data minimization, consent and audit rules; negative tests must include cross-Space reads, post-revocation plugin reads, replayed effects and exfiltration attempts. Keep source identity distinct from authorization.

### H6 — Paid services may stay closed, but must not become an undeclared compulsory conformance dependency

**Evidence:** ADR-0013 permits closed paid modules and says free access to hosted infrastructure is not implied; AD-17 bans hidden mandatory extensions; `REQ-OPEN-006` requires terms permitting independent baseline implementation. There is no explicit self-hosted/offline test path or test credentials independent of the paid managed service.

**Counterexample:** an independent node correctly implements the public mailbox protocol but the only conformance runner requires a paid VIDA tenant or marketplace token to exercise federation. Or an official client sends mandatory premium-only anti-abuse attestation not in the declared base profile; the independent node cannot serve it. The wire spec is public, but a commercial gate controls the interoperability claim.

**Disposition — gate:** publish a runnable, vendor-neutral conformance harness and test peer with synthetic data, no paid account or marketplace listing required. Distinguish protocol compatibility from admission to a particular managed network. Any premium entitlement or optional extension must be discoverable in the public capability matrix and cannot be necessary for the declared base profile.

## Contained future decisions, not current contradictions

- Canonical bytes, signatures, version negotiation and mandatory-feature handling remain properly blocked by `OQ-0028`; no reviewer-selected codec is warranted.
- Deterministic state transitions, authority, snapshots, FFI, storage and release activation remain separately blocked by `OQ-0033`–`OQ-0037`; opening specs does not make those details decided.
- Repository manifest, publisher identity, hook executor/sandbox, updates and side effects remain open under `OQ-0040`–`OQ-0045`/`OQ-0048`; a prototype may exercise them but must not claim general App/plugin conformance.
- Exact spec/test/reference-code licenses, patent/IPR terms, proposal governance, stable releases and deprecation remain `OQ-0055`; the existing public-production claim gate is appropriate.
- Public security fixtures must contain synthetic keys/data and sanitized official release evidence; ADR-0013 already rules out treating transparency as publication of user data or private keys.

## Closure criterion

The architecture direction can proceed as **approved with explicit implementation gates**: C1 and H3 are resolved in the spine. A public independent-interoperability claim still requires: a registered mandatory profile, no paid test prerequisite, byte/behavior/security contracts and fixtures, cross-vendor pairwise results, a bounded App execution/security profile, and an unambiguous separation of free base interoperability from optional paid implementation/services.
