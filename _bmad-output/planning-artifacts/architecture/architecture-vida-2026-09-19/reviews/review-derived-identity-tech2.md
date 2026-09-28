# Independent reviewer gate — technology currency and project-reality lens

Date: 2026-09-19  
Target: `ARCHITECTURE-SPINE.md`  
Verdict: **Changes required before an unqualified final handoff.** Iroh 1.2.0 and its named Endpoint/Router/Address Lookup concepts are current; the material problems are semantic/source alignment, not an obsolete transport version.

## High — origin commit is conflated with authority acceptance

**Location:** `ARCHITECTURE-SPINE.md:102–106, 156–160, 181` (AD-4, AD-13, state-mutation convention).

AD-4 says one local durability transaction records the operation, outbox, dedupe state **and authoritative frontier advancement**. AD-13 repeats this and calls the validated signed operation log the sole durable domain authority. Yet AD-5 correctly says `delivery.accepted` is only origin log+outbox commit, **not** `authority.accepted`, and ADR-0003 requires a separate Space-authority acceptance for revocation and authority-requiring writes (`ADR-0003:62–88, 130–138`). Under an authority-accepted Space, an offline origin cannot advance the authority-accepted frontier atomically with its local outbox transaction. An implementer could display a pending edit as final or accept a revoked grant without an authority receipt.

**Fix:** name and separate an `originDurableFrontier`/pending-intent ledger from the `authorityAcceptedFrontier` (exact names optional). The origin transaction advances only the former; the latter advances only after evidence satisfying the selected Space/object/operation acceptance policy. In peer-authoritative profiles, define which validated evidence makes the two coincide. Keep projections explicit about tentative versus authority-accepted state; leave the policy itself in OQ-0033. This is a wording/model correction, not a demand to choose a centralized server.

## High — corporate account isolation has weaker upstream normative language

**Location:** `ARCHITECTURE-SPINE.md:80`; `ADR-0004:72–76`; `REQ-ID-013` in `identity-requirements.md`.

The spine's inherited invariant says a corporate account/work context **does not reuse** unrelated private Personas. But the accepted ADR says the user **MAY** first create a separate Persona, and the requirement says the system **MUST allow** a separate account. Both still permit attaching a private Persona to a corporate node, possibly by an explicit path. The latest user clarification is stronger: the private identifier is not used for company accounts; the company-issued account is a separate switchable context, and company deactivation affects only its grants. The spine's statement is the safer interpretation but is not inherited verbatim from the currently accepted upstream rule.

**Fix:** reconcile ADR-0004/REQ-ID-013 with the user before marking this inherited invariant closed. If the intended rule is mandatory separation, require a fresh/unlinked corporate Persona or equivalent unlinkable account identity at enrollment, with no silent cross-context identifier/link leakage; keep corporate Persona controller and recovery in OQ-0049. If an explicit same-Persona opt-in is intended, the spine must say so and the privacy consequences need acceptance evidence. Do not silently strengthen the parent ADR only in the spine.

## High — diagram prematurely selects native Swift/Kotlin shells

**Location:** `ARCHITECTURE-SPINE.md:54, 224–228`, versus the same document's Deferred section at 250–258.

The diagram labels production shells `Swift, Kotlin, desktop, browser`, while the UI-platform research explicitly calls the user's Flutter mobile + Rust + Tauri Windows idea a **working hypothesis**, not an approved toolkit decision (`technical-vida-ui-platform-and-rust-binding-strate-2026-09-19/research.md:18–24`). The spine itself defers exact UI toolkits and desktop shell choice. This matters technically: Flutter's mobile shell uses Dart/native interop, whereas Tauri uses a WebView↔Rust message boundary; Swift/Kotlin are optional platform integrations or fallback native UI choices, not the chosen presentation stack. [Flutter official platform/interop guidance](https://docs.flutter.dev/platform-integration); [Tauri official architecture](https://v2.tauri.app/concept/architecture/).

**Fix:** label the diagram `platform-selected mobile, desktop, browser shells` and annotate toolkit selection as prototype-gated; keep `ios/` and `android/` as OS capability profiles rather than implying Swift/Kotlin UI. A later platform ADR can pin Flutter/Tauri or native alternatives after the already specified comparative prototype.

## Medium — one logical derived resource does not imply exactly one logged operation

**Location:** `ARCHITECTURE-SPINE.md:160, 198`; `REQ-EFFECT-007` in `effect-execution-constraints.md:26`.

AD-13 correctly binds the user-approved observable result: all devices converge on **one logical resource** after the same committed source fact. The gate then says OQ-0048 must define "one canonical operation/ID," which adds an unstated implementation constraint. Two devices can emit distinct candidate operations while deduplication/authorization causes only one resource to survive (or one candidate becomes a no-op), provided both converge and ADR-0012's replay boundary holds. Conversely, a deterministic shared resource ID alone is insufficient if two signed creation operations produce conflicting content under different AppInstance versions. The exact operation cardinality is algorithm-dependent and genuinely open.

**Fix:** gate on a canonical logical resource identity **and convergent accepted outcome**, including duplicate receive, offline evaluation, mixed AppInstance versions and crash recovery. Leave candidate-operation count, signer/claim protocol and tie-breaker to OQ-0048. Require fixtures that demonstrate one visible resource, no second external effect and deterministic conflict disposition.

## Current-technology check and no-finding notes

- [iroh crate 1.2.0](https://docs.rs/crate/iroh/1.2.0) is published; its first-party crate docs show `Endpoint`, `Router::builder(...).accept(ALPN, ...)` and relay fallback. [AddressLookup](https://docs.rs/iroh/1.2.0/iroh/address_lookup/trait.AddressLookup.html) is a real 1.2 API. The spine's 1.2.0 decision baseline is current as checked 2026-09-19. Exact release pin/checksum and conformance gates remain appropriate.
- The new separate-company-account rule is directionally consistent with the user clarification, but the *MAY/MUST* mismatch above prevents treating it as fully ratified downstream.
- AD-13's one-resource observable invariant is aligned with `REQ-EFFECT-007` and ADR-0012 **if** OQ-0048 remains a true implementation block. It does not grant `sync.apply` permission to rerun commands; the spine correctly preserves that boundary.
- Open questions for same-Persona concurrent field/status conflicts (OQ-0034), corporate Persona controller/recovery (OQ-0049), and derived-resource execution (OQ-0048) are correctly deferred rather than invented.
