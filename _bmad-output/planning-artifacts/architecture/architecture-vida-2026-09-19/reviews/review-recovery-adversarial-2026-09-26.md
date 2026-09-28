---
review: recovery-adversarial-two-compliant-implementations
artifact: ../ARCHITECTURE-SPINE.md
date: 2026-09-26
verdict: architecture-direction-consistent-production-interoperability-blocked
---

# Adversarial recovery review — 2026-09-26

## Verdict

AD-37, `REQ-ID-015/016`, the recovery spec and Stories 1.1–1.4 agree on the two-part owner-held kit, stable Persona, fresh Device keys, and the separation of authority from data recovery. The `setup_pending` gate closes an important partial-bootstrap failure. They do **not** yet determine when a kit remains usable after controller/key changes, how two concurrent restore transitions converge, or what evidence makes a restored authority claim final. These are explicitly open under `OQ-0022/0024`; neither independent implementation nor production recovery may claim conformance before a normative profile and shared fixtures exist. This review tests the gap, rather than treating the open questions as an adopted design.

## Counterexample 1 — both ship the approved kit; only one survives a later rotation

**Common input:** At bootstrap, both clients stage the same stable Persona/Space IDs, create a random secret plus portable encrypted bundle `B0`, ask for separate storage, and activate only after durable confirmation. This satisfies Story 1.1/1.2, `SPEC-ID-011`, and `REQ-ID-015/016`. Later a trusted Device is lost. The remaining trusted Device revokes its grant and rotates affected key envelopes/epochs as required by the recovery spec. The owner then loses that remaining Device and presents the original secret and `B0` on a replacement Device.

| Independent implementation | Locally compliant interpretation | Divergent result |
|---|---|---|
| A: one-time portable export | `B0` was current when exported; later controller/epoch changes do not force a new owner-held export or a new confirmation. Rotation invalidates the old recovery credential or removes envelopes needed after the rotation. | The owner followed the initial UX yet cannot restore current authority or some later content; Story 1.4's “current encrypted recovery bundle” is unavailable. |
| B: revision-bound portable export | A controller/recovery-key change creates `B1`; the client verifies the new export and shows `B0` as superseded before claiming recovery readiness. | The owner can restore from the secret plus `B1`, subject to current-controller proof and available encrypted resource bytes. |

No adopted rule states whether the recovery credential is stable across controller changes, whether every change invalidates old bundles, who must publish/export a replacement, or what happens when export fails after the control transition. Both units satisfy the *initial* two-part kit and revocation wording yet yield opposite recovery outcomes. `recovery-bundle-contract.md` names “current bundle” and atomic custody as open; Story 1.2 verifies a user claim, not retained bytes or future freshness.

**Required closure (`OQ-0024`):** define a versioned bundle-to-controller/key-epoch binding and update protocol. Specify whether rotation commits only after a usable replacement kit is durably exported, or whether the product explicitly enters a degraded, non-recoverable state. Cover crash between controller commit and `B1` export, stale OS/cloud `B0`, interrupted download, cross-platform import, and a lost-all-Devices restore using the latest independently retained kit. The UI must distinguish `confirmed_by_user`, `bundle_export_verified`, and `kit_current_for_controller_head`; none proves an encrypted resource copy exists.

## Counterexample 2 — two valid restores, incompatible controller histories

**Common input:** All prior Devices are unavailable. Two replacement Devices independently possess the same valid secret and bundle with signed controller checkpoint `H0`. During a partition, each generates fresh keys and proposes a different signed `DeviceGrant` transition from `H0`. Neither has seen the other's proposal; neither may read protected data before its transition is accepted.

| Independent implementation | Locally compliant interpretation | Divergent result after contact resumes |
|---|---|---|
| A: merge concurrent grants | Treat sibling transitions from `H0` as commuting additions, assign a deterministic merged controller state, and accept both new grants. | Both Devices become authorized peers. |
| B: serialize controller transitions | Choose one canonical successor of `H0`; reject or rebase the other proposal after the winner is accepted. | Only one Device obtains an accepted grant; the other remains pending or repeats enrollment. |

Both preserve `PersonaId`, use fresh Device keys, and require a versioned transition before protected access. `AD-37` says “current ControllerState” but neither the canonical history (`OQ-0022`) nor concurrent-restore ordering (`OQ-0024`) defines whether sibling transitions can merge. The same ambiguity becomes a security failure when one proposal comes from a stolen secret+bundle: unit A may admit the attacker alongside the owner, while unit B may choose the attacker as winner. A signed checkpoint alone cannot establish that `H0` is still current after a remote revocation; a locally maximal counter is not a freshness proof.

**Required closure (`OQ-0022/0024`):** define the accepted controller-head proof, branch ordering/merge rule, replay window, recovery-authority revocation, idempotent command ID, and behavior when no current-head witness is reachable without making a corporate/relay service the private controller. Specify `pending`, `accepted`, `rejected`, and `unknown` restore outcomes; no local proposal or transport receipt may be labeled `authority_restored`. Test two honest restores, honest versus stolen-kit restore, stale checkpoint after revocation, duplicate delivery, and crash/retry at the grant-commit boundary. A compromise response needs a defined way to invalidate the old recovery authority and provision a new owner-held kit; `REC-F09a` correctly says this is not yet guaranteed.

## Additional adversarial probes for the same gates

- **Browser profile loss/new Device:** A cleared Web profile generates new keys and `DeviceId`; it never inherits the old `DeviceGrant`. Verify that the old browser grant is shown for explicit revocation even if the old profile cannot participate. A new local keypair plus an old grant ID must be rejected. `AD-37` already requires this; `REC-F02/F07/F09` should include Web-to-Web and Web-to-native variants.
- **Compromised static kit importer:** A hostile or rolled-back JS/Wasm origin can observe both kit parts when the user imports them. Showing the origin and avoiding `localStorage` do not prevent that theft. The Web origin/code-delivery gate must exercise recovery import, replay with a stolen kit, and post-compromise rotation; success cannot be inferred from native key-store evidence (`REQ-BROWSER-005/009`, `AD-9`).
- **Partial recovery:** A valid kit may restore Persona authority but no data keys, or keys but only a subset of ciphertext. Record separately the accepted controller transition, successfully unwrapped key epochs, and verified Note/File IDs and bytes. Preserve missing-content and pending states across restart; do not turn a local grant proposal, bundle decryption, or a resource manifest into “identity and history restored.” Story 1.4 and `REC-F03/F04` require the distinction but do not yet define its exact proof/receipt contract.
- **Bootstrap crash:** Crash after staging and before confirmation must reopen the same `setup_pending` IDs/material. Crash after confirmation commit must emit/observe one `PersonaCreated` and enable protected work once. Story 1.1/1.2 and `REC-F01` already cover this; retain it as a mandatory negative fixture when storage providers vary.

## Evidence anchors

- `ARCHITECTURE-SPINE.md:325-329` (`AD-37`); `ARCHITECTURE-SPINE.md:360-367` (implementation gates).
- `docs/02-requirements/identity-requirements.md:58-60,74-82` (`REQ-ID-011/015/016`).
- `docs/04-specifications/identity-domain-contract.md` (`SPEC-ID-011`, atomic control-state failure and idempotent retry).
- `_bmad-output/specs/spec-vida-persona-recovery/SPEC.md` (`CAP-1`–`CAP-5`, open questions); `recovery-bundle-contract.md`; `recovery-cases.md` (`REC-F01`–`F12`, especially `F07/F09a`).
- `_bmad-output/planning-artifacts/epics.md:201-312` (Stories 1.1–1.4); `docs/02-requirements/browser-client-requirements.md:18-26` (Web custody/origin boundary); `docs/00-governance/open-questions.md:32-34` (`OQ-0022/0024`).
