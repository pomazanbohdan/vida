# Post-commit effect cases — planned, not executed

These cases preserve approved behavior from `REQ-EFFECT-001/007–011`, `ADR-0009–0012` and `REQ-FINALITY-001/007/010`. They do not choose an executor, rule ID derivation, provider retry API or code runtime.

| ID | Setup and action | Observable result |
|---|---|---|
| EFF-F01 | Device A submits an authorized named command; Device B receives the resulting operation twice and rebuilds its projection. | B applies one accepted fact but never invokes the originating command or handler again merely because of receive/rebuild. |
| EFF-F02 | A source candidate is saved locally, replicated and then rejected by its named authority. | No accepted derived resource or irreversible external effect is emitted from the pending/rejected candidate. |
| EFF-F03 | One accepted Task assignment fact is observed by three authorized Devices. | One logical derived Note appears in Space with one stable identity, not one Note per Device. |
| EFF-F04 | Device A has rule revision 1, Device B has revision 2; the accepted source fact is bound to one effective revision. | Both Devices converge on the same derived logical resource. Updating the AppPackage does not rerun the old fact under revision 2. |
| EFF-F05 | Origin crashes after durable source intent commit but before effect execution. | Pending intent is recoverable; restart does not silently mark the effect complete or create a second logical source operation. |
| EFF-F06 | Every Device of an autonomous Personal Space is off. | The external action may remain pending until an authorized Device returns; no permanent online server is presumed. |
| EFF-F07 | An identity/relay-only node routes an update for a private shared Space. | It can forward opaque update metadata/ciphertext but cannot read the private Task/Note or execute plaintext-dependent business logic by virtue of being a node. |
| EFF-F08 | A Device locally saves or merely delivers `Done` that would send a customer letter. | The letter does not leave until the source transition has the required AuthorityOutcome and no known unresolved conflict. |
| EFF-F09 | A confirmed transition has a known unresolved conflict at the executor. | The irreversible effect remains pending; UI does not label it sent. |
| EFF-F10 | Two Devices observe the same eligible fact; effect intent has one stable idempotency identity. | They retain one durable logical intent and attempt identity. At-most-one provider-visible effect requires a separately proven executor/provider idempotency contract; after an uncertain unsupported attempt, the status is `effect.unknown`, not an automatic second call. |
| EFF-F11 | Provider may have applied an external request, but its response is lost and it offers neither lookup nor safe same-key retry. | `effect.unknown` remains distinct from success/failure; no blind automatic duplicate request occurs. |
| EFF-F12 | Provider explicitly supports result lookup or a bounded idempotency contract for the same request key. | Recovery follows only that declared provider contract and records a separate effect receipt/status; a generic transport ACK does not stand in for provider result. |
| EFF-F13 | A letter was sent for accepted `Done` at frontier F; one later valid authority-accepted incomparable branch outside F creates conflict. | Sent fact stays in audit; branch actor gets `correction_needed`; VIDA does not invent an automatic cancellation. A pending or revoked candidate alone cannot start correction. |
| EFF-F14 | Several valid authority-accepted incomparable outside-F branches jointly create the late conflict. | System does not arbitrarily choose one correction actor; relevant actors see correction-needed until explicit conflict resolution. |
| EFF-F15 | Another user with ordinary update right changes the now-current Task status. | The change follows the normal domain flow and is not presented as transfer or completion of someone else's correction workflow. |
| EFF-F16 | A bot has normal `update task status` right and general automation opt-in, but no conflict-context opt-in or read access to both variants. | Ordinary update may proceed; conflict-covering resolution fails until the extra context and read checks pass. No special `resolve_conflict` ACL right is requested. |
| EFF-F17 | An instance handler extends Notes in AppInstance A; the same base App runs in AppInstance B; the handler attempts an unauthorized host call. | A's supported handler takes priority at its named point without changing B; B follows its own configured/base behavior, and Core rejects the unauthorized call. Base-handler continuation in A remains open. |

The result of a fixture is not proven by this table. Wire bytes, old-handler availability, exact executor/lease, API-specific retry and cross-platform runtime behavior require separate implementation and conformance evidence.
