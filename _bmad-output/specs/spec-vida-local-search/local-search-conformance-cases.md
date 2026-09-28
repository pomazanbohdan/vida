# Local Search conformance cases

These are Release-1 acceptance scenarios derived from PRD FR-11/FR-21, `REQ-CLIENT-022`, `SPEC-ID-009`, `REQ-ACL-020`, shared-Space rights rules and the UX search primitive. They are not evidence of an implemented index. Run on Android, iOS and Windows against the same authorized Resource fixture set.

| ID | Scenario and required observation | Source |
|---|---|---|
| SEARCH-F01 | Synchronize one readable Message, Forum Topic, Note, Task, File entry and ContactCard to the device; search while offline. Applicable entries appear, while content never synchronized to this device is absent and the UI states the local coverage limit. | PRD FR-11/FR-21; REQ-CLIENT-022; EXPERIENCE Search |
| SEARCH-F02 | Apply Persona/actor, Space, AppInstance, author and Resource-type filters to mixed authorized results. Every filter and result count reflects only visible entries. | PRD FR-21; REQ-CLIENT-022 |
| SEARCH-F03 | Keep two unlinked Personas in one LocalVault. Switch active account context and repeat query. No title, snippet, count, recent search suggestion or resource navigation reveals the other Persona; an authorized explicit cross-account mode, if implemented, is tested separately. | SPEC-ID-005/009/010 |
| SEARCH-F04 | Grant a Guest one Note in Personal Space containing a backlink to a private Note and an inaccessible File. Guest search finds only granted content; filter values, counts, snippets and backlink titles do not disclose the private targets. | REQ-ACL-020; EXPERIENCE Search |
| SEARCH-F05 | Query a shared Space, then receive effective revocation or reach seven days without authority-confirmed rights reconciliation. Search, cached results, suggestions and open actions deny shared content; a stale control head, restart or local clock rollback cannot restore access. | Space membership offline-read freshness; NFR-SEC-005 |
| SEARCH-F06 | Revoke read between rendering a result and activating it. The open action repeats authorization and shows a safe unavailable state, not the old title/snippet or Resource body. | EXPERIENCE Search; SPEC-ID-009 |
| SEARCH-F07 | Edit a permitted Resource, then apply a tombstone or delete under its domain rule. After local projection reconciliation, old text and excerpts disappear; rebuilding from canonical authorized state yields the same visible result set. | PRD FR-21; SPEC-SYNC-LOG-001 derived projections |
| SEARCH-F08 | Crash during indexing, restart without network and rebuild from durable local state. Search never presents an uncommitted change as saved or resurrects a locally applied revocation, expired rights proof or deleted result. An unseen remote revocation cannot be inferred while offline. | REQ-CLIENT-009/022; SPEC-SYNC-LOG-001 |
| SEARCH-F09 | Capture network requests during local query, filtering and result opening in default mode. No private query or Resource text is sent to an external search service. | PRD FR-21; REQ-CLIENT-022 |
| SEARCH-F10 | Exercise Arabic RTL input and Japanese, Korean, Simplified and Traditional Chinese search terms on all Release-1 clients; record tokenization, glyph/input and accessible result behavior. | REQ-L10N-004; REQ-CLIENT-001 |
| SEARCH-F11 | Search a File whose metadata is local but bytes are not downloaded. The UI does not claim body-content coverage or permit an inaccessible attachment through its parent relation; exact indexed File fields remain a recorded scope decision. | PRD FR-19/FR-21; OQ-0083; local-search open question |

## Evidence gate

- For each client record fixture, active Persona, grant/control epoch, locally available bytes, query/filter input, result IDs and snippets, open-action outcome, network trace and post-restart result set.
- Compare baseline search and the selected index provider using the same fixtures. An index benchmark or vendor documentation does not replace permission, revocation and cross-Persona negative proof.
- Treat field extraction depth, tokenization implementation and index-at-rest protection as prototype decisions, not as an implicit approval of a particular database or search library.

## Preservation boundary

The approved product promise is local search over accessible synchronized Spaces with filters. This suite adds observable security and rebuild checks derived from existing identity, ACL, rights-freshness and sync contracts; it does not choose full-file/OCR scope, a provider, a latency target or new cross-account access semantics.
