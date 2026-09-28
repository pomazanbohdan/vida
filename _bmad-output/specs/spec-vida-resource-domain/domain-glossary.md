# Domain glossary

`Approved` means the meaning is already supported by an approved product, requirement or ADR. `Open` means the term is useful but its exact domain shape still needs a decision.

| Term | Meaning and boundary | Status |
|---|---|---|
| Persona | Separate cryptographic user identity with its own profile, devices and Spaces. No automatic linkage to another Persona. | Approved |
| Space | Top-level ownership, membership, keys, policy, App and sync boundary. Personal Space has one Owner; Shared Space may have several. | Approved |
| AppPackage / AppInstance | Signed package definition / one activation of it in a Space. Bundled and external Apps use the same Core contracts; Developer is not Space Owner. | Approved |
| Resource | Stable addressed schema record. PRD examples: Message, Forum Topic, Note, Task and File; ContactCard is a Core Resource. Exact ContactCard owning scope is open. | Approved core; placement open |
| Author / Owner | Author created a Resource; Owner is the reserved governance role of a Space. Authorship alone does not convey Owner powers. | Approved |
| ResourceType / Schema | Declared record kind and versioned field/action contract. A write names its schema identity/version; retained history keeps writer version. | Approved |
| Container | Organizing and permission-inheritance scope within an AppInstance, e.g. Project X or Note Section. Whether it is a Resource and its parent cardinality are not fixed. | Open shape |
| Relation | Typed edge between Resources, not a copied payload and not a grant to inspect either target. Edge identity, authority and cross-Space storage are open. | Approved purpose; mechanics open |
| Attachment | Relation to a File Resource. Reusing a File does not duplicate its identity or payload. | Approved |
| Backlink | Reverse navigation/projection from a Relation; only endpoints independently readable to the viewer may be shown. Whether stored or derived is open. | Approved visibility; representation open |
| File / Blob | File is a versioned Resource with stable ID and Relations; blob is verified immutable payload under BlobStore. Local payload availability may differ by Device. | Approved distinction |
| Conflict | Incompatible causally concurrent versions that cannot safely merge; a link or projection cannot silently select a business winner. | Approved |

`ContactCard` and `File` are Core-level records, so this glossary does **not** assume every Resource is owned by an AppInstance. A record's exact owner scope must be explicit in its contract.
