# OQ-0072 call state and proof plan — draft

This is a behavioral refinement and test plan, not an adopted wire format or media-library selection. Existing [call conformance](../../../docs/04-specifications/e2ee-calls-conformance.md), [operation finality](../../../docs/04-specifications/operation-finality-contract.md) and architecture AD-35 control any conflict.

## Distinct evidence and actor boundaries

| Fact | Source | What it does **not** prove |
|---|---|---|
| Caller-origin durable `CallIntent` | Signed Core operation, committed locally. | Recipient availability, ring, answer or media connection. |
| Recipient-applied invite | Authorized Device validates context, grant and operation. | Accepted answer by the Persona. |
| Ringing | OS UI or ephemeral signal derived from a validated invite. | Durable authority outcome; a push alone is only a wake signal. |
| `AnswerAttempt` | One Device's durable signed request at the current known frontier. | Permission to publish media before authority acceptance. |
| Accepted answer | Verifiable Core `AuthorityOutcome` for exact call revision and Persona/Device. | Connected tracks or continuing grant after revocation. |
| Media admission and connection | Replaceable adapter consumes Core proof and current key epoch, then reports track/session state. | Ownership of Space or retroactive Core acceptance. |
| Terminal outcome | Core-accepted decline/cancel/end/expiry for exact revision. | Deletion of earlier audit facts or packets already delivered. |

The answer is exclusive **per Persona**, not a global election of one call participant. Devices have no fixed rank. A group call may have several accepted Personas, each with one accepted Device until an explicit accepted transfer. Exact multi-Device listen-only behavior is outside this draft.

## Candidate lifecycle to prove

1. Caller creates a stable `CallId` and signed `CallIntent` with immutable owning context, audience revision and key-epoch lineage. Local commit gives only `origin.committed`. Iroh/mailbox may deliver the same envelope and ID.
2. Recipient Device checks signature, current control/grant, context and expiry proof before showing the OS ring. A push/notification cannot carry authority or a reusable media credential. Anonymous-no-server can ring only while directly reachable; consented Public push is a separate platform profile.
3. A tap on Answer creates a stable idempotent attempt. While authority is unreachable, display **answer pending** and do not publish microphone/camera or mint a publish-capable SFU credential. Another Device may make a competing attempt; neither gains priority from being first to contact an SFU.
4. A valid acceptance proof binds `CallId`, revision, Persona, selected Device, owning context, control frontier, key epoch and causal base. A losing attempt resolves to superseded/rejected and stops ringing after receipt; unknown/missing proof remains pending, never accepted by default.
5. Accepted transfer needs a new proof binding the new Device and invalidating old admission. Leave/revoke/end rotate relevant E2EE keys and eject/deny stale media identity. The media provider may enforce a narrower capability but cannot grant more than Core authorized.
6. After crash/reconnect, runtime replays durable control facts by stable IDs, reconciles authority outcome, then derives ring and media state. It neither repeats the originating command nor treats transient track loss as a new terminal decision.

Exact expiry, proof encoding, serialized authority topology and ordering of competing accepted transitions remain open. A local timeout may stop an obsolete ring **on that Device**, but cannot by itself issue a globally accepted `missed`/`expired` outcome.

## Provider admission hazard and selection consequence

The [LiveKit token/grant documentation](https://docs.livekit.io/frontends/reference/tokens-grants/) states that room/participant identities are logged and should be opaque, token expiration controls initial connection, and a self-hosted deployment does **not** revoke an existing token merely by removing a participant or changing permission. LiveKit Cloud's revocation behavior must not be assumed for the self-hosted baseline. Thus short token TTL plus refusal to reissue is only a partial mitigation: it does not by itself prove immediate rejoin denial or end an already connected participant. The prototype must demonstrate a Core-gated token issuer/admission path, active-participant eviction, key-epoch rotation and denial of cached-token replay within the required security boundary; otherwise that provider profile fails the call gate. The precise enforcement mechanism is still a decision, not an approved design.

[RFC 9605 SFrame](https://www.rfc-editor.org/rfc/rfc9605.html) leaves key management to the application; enabling a media SDK E2EE toggle is not evidence that VIDA rotates keys on join/leave or scopes epochs correctly. [OWASP MASVS](https://mas.owasp.org/MASVS/) and [MASWE-0003](https://mas.owasp.org/MASWE/MASVS-STORAGE/MASWE-0003/) frame the mobile authorization, crypto and protected-key checks; they do not select the provider or set the answer authority.

## Proposed conformance additions to existing F07/F16/F17

| ID | Schedule or attack | Required observation |
|---|---|---|
| CTRL-01 | Three Devices of one Persona answer simultaneously, direct and mailbox deliveries reorder. | At most one accepted proof and publish-capable admission; stable losing outcomes. |
| CTRL-02 | Two Devices answer while partitioned from each other and from logical authority. | Both may have durable pending attempts; neither claims acceptance or publishes. |
| CTRL-03 | Accepted answer arrives after caller cancel or accepted transfer. | Compare exact revisions/causal proofs; stale outcome cannot resurrect ring or old media access. |
| CTRL-04 | OS displays ring, app crashes before answer receipt, then restarts. | Reconcile durable facts; no second answer attempt without user action; no ghost participant. |
| CTRL-05 | Shared Resource guest follows cross-Space relation into call; grant later revoked. | Wrong-context join denied; reconnect rechecks immutable binding/current right. |
| CTRL-06 | Self-hosted participant removed; cached token used before expiry and after rejoin attempt; existing session persists. | Provider-specific enforcement plus epoch rotation denies publishing/decryption; a TTL-only demonstration fails. |
| CTRL-07 | Iroh invite duplicated through direct and mailbox paths; old invite replayed after terminal outcome. | One call identity, no new ring after proven terminal outcome, no duplicate media principal. |

## Conformance trace and result evidence

These CTRL schedules refine—not replace—the adopted [F01–F17 fixture set](../../../docs/04-specifications/fixtures/e2ee-calls-v1.yaml). Run each relevant schedule on the installed Android, iOS and Windows candidates where its platform path exists; a skipped path is reported as **not proven**, not passed.

| CTRL schedule | Adopted fixture coverage | Distinguishing evidence to retain |
|---|---|---|
| CTRL-01 | F07, F17 | Competing attempt IDs, their causal bases and one verifiable accepted `AuthorityOutcome`; admission/denial for all three Devices. |
| CTRL-02 | F07, F17 | Each partition's durable pending attempt, authority-proof absence and no publish-capable credential or media track. |
| CTRL-03 | F07, F17 | Answer/cancel/transfer revision and causal comparison; terminal projection and old-admission rejection after reconciliation. |
| CTRL-04 | F07, F11, F17 | Pre-crash commit/receipt boundary, recovered operation ID, post-restart projection and absence of a new attempt or ghost principal. |
| CTRL-05 | F05, F16 | Immutable owning-context binding, current grant/control frontier and denied wrong-context or revoked join/reconnect. |
| CTRL-06 | F04, F05, F16 | Accepted-to-revoked transition, cached-token replay result, active-participant eviction and post-removal key-epoch decryption failure. |
| CTRL-07 | F05, F17 | One `CallId` and invite operation ID across both delivery paths; proven terminal frontier and suppressed replay ring. |

For each run, the candidate harness records: fixture/schedule ID and pinned build/provider profile; opaque test references for `CallId`, intent/attempt operation IDs and exact revision; Persona/Device role labels; owning Space/AppInstance/conversation-or-Resource binding and grant/control-frontier reference; the observed origin, recipient-apply and authority evidence axes; `AuthorityOutcome` decision, issuer/proof reference, signature-validation result and causal base **or explicit proof absence**; key-epoch lineage reference; media-admission decision, credential scope and post-revoke/replay result; UI state before/after reconciliation; and restart/partition schedule with the asserted result. The harness may store raw sensitive inputs only in an access-controlled test environment; exported diagnostics use opaque references and omit media, keys, stable identities, private labels and SDP/IP data under REQ-CALL-011. This describes **evidence fields**, not a VIDA wire format or an authority-selection algorithm.

Gate rule: a Device claiming to have **answered or received an accepted transfer** without an exact, valid and current authority acceptance for that transition fails; a pending/losing answer attempt must not get publish-capable admission. Caller and other participants still need their separate current Core call/grant authorization, but this paragraph does not impose an answer proof on them. An unavailable answer authority can leave the attempt pending; it cannot be waived into acceptance. A provider that cannot demonstrate post-revocation denial and key-epoch isolation under CTRL-06 fails that provider profile. Numeric call quality/resource targets remain OQ-4; media profile remains OQ-3; OQ-0033/OQ-0072 still own the authority topology and expiry rule.
