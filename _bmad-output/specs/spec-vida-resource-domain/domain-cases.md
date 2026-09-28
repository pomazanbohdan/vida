# Resource-domain conformance cases

| ID | Case | Required observation |
|---|---|---|
| RES-01 | Create Note and Task offline, restart after local commit, then synchronize twice | Stable IDs/history on both devices; no duplicate Resources or second originating command. |
| RES-02 | One File is attached to a message, note and task | Three authorized attachment Relations point to one File ID and payload, not three File copies. |
| RES-03 | Viewer can read a Task but not its linked Note | Task remains visible; Note title, snippet, backlink target and search hit remain hidden. |
| RES-04 | Revocation arrives while a Resource is shown in search or a link preview | Current read check removes managed access/projections; old exports/screenshots are not claimed erased. |
| RES-05 | Lower Resource rule says allow, upper Container/Space rule says hard deny | Effective action is denied and UI can identify its controlling rule without revealing hidden data. |
| RES-06 | AppPackage v2 adds optional field with safe default to an old record | Read uses current projection without eager history rewrite; next edit stores current schema with materialized default. |
| RES-07 | AppPackage v2 adds required field without safe default | Old record remains readable through converter but cannot be saved as current until a value is supplied. |
| RES-08 | Old client receives a Resource with mandatory unknown schema feature | Affected AppInstance is read-only/update-required rather than dropping unknown fields; other Apps continue. |
| RES-09 | File payload cannot be downloaded to this Device | File metadata/Relations persist; local state says not downloaded with cause; remote copy unchanged. |
| RES-10 | Export selected accessible Resources and Relations | Export includes allowed endpoints and files with schema/provenance; inaccessible target metadata is not serialized. |
| RES-11 | Actor supplies a known or guessed ID of a Resource outside their grant in a relation, deep link or export request | Every path rejects unauthorized access; no title, snippet, existence hint or payload is returned through that path. |

Decision-dependent future fixtures: Container move/multi-parent behavior; ContactCard owning scope; cross-Space Relation edge authority and backlink visibility; copy/revision metadata/ACL inheritance. These cannot be specified as passing outcomes until their OQ-0003/OQ-0034 decisions are made.

Security reference: [OWASP IDOR Prevention](https://cheatsheetseries.owasp.org/cheatsheets/Insecure_Direct_Object_Reference_Prevention_Cheat_Sheet.html) calls for object-level checks even with hard-to-guess identifiers; the [OWASP Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) calls for deny-by-default and permission validation on every request. These support RES-03/04/10/11; they do not define VIDA's Relation ownership or cross-Space wire format.
