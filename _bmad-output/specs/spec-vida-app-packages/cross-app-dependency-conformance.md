# Cross-App dependency conformance — OQ-0070 draft

This companion separates the **adopted no-grant boundary** from unselected dependency-manifest and grant mechanics. It neither activates an AppInstance nor defines a production capability token.

## Accepted boundary

- A complex App may create a user-owned dedicated Space and reuse Messenger, Notes and Projects as dependent AppInstances. One Space can therefore contain multiple AppInstances; a simple extension can target an existing AppInstance. The final cardinality rules beyond these cases remain OQ-0070.
- Under [AD-39](../../planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md#ad-39--app-dependencies-carry-contracts-not-grants-adopted), a dependency declares a versioned API/schema contract and compatible capabilities. Declaration, download, provisioning and activation confer no membership, read/write/effect grant or network egress.
- Core evaluates every call against the initiating actor or explicitly delegated principal and current Space, AppInstance, container and Resource policy. Hard deny, revocation and [AD-36 relation checks](../../planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md#ad-36--shared-resourcerelation-graph-is-core-owned-adopted) still apply. A link to a Note or File cannot reveal its title, preview, bytes or backlinks without target access.
- First-party and external Apps use the same authorization semantics. Publisher `Developer` status does not authorize customer data. An update that expands a dependency contract or data scope is not `compatible-auto` under [REQ-APP-018](../../../docs/02-requirements/app-package-requirements.md).

## Candidate contract to evaluate, not approved fields

| Stage | Candidate evidence | Failure boundary |
|---|---|---|
| Declaration | Source package/instance, target package/capability, contract version range, required/optional status, named operations/views and direction. | Unknown mandatory capability or incompatible range fails explicitly; a declaration is not a grant. |
| Resolution | Bind a compatible target AppInstance and version in the activation record; present the actual data scope to the authorized Space actor. | Missing required target blocks only affected activation; an optional target may degrade explicitly, never silently expose a different target. |
| Authorization | Separate approved grant or ordinary actor permission for the requested operation; delegated service access requires its own explicit capability. | Neither bundled status nor shared Space alone bypasses target ACL, hard deny or effective revocation. |
| Call | Core checks actor/delegation, source and target instance, operation, Resource/container scope, current control frontier and schema compatibility. | A cached UI decision, opaque handle or previously valid grant is insufficient after rights or version change. |
| Update | Re-evaluate target version and effective contract before activation; surface added operations/views/data scope. | Expanded scope cannot inherit `compatible-auto` merely because package identity is unchanged. |

This is one possible manifest shape. Exact field names, target selection, grant issuer/UX, delegation proof and cross-Space support are not decided. [OWASP Authorization](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) supports least privilege, deny-by-default and per-request checks; it does not prescribe VIDA's manifest or Space topology.

## Candidate fixtures — not executed

| ID | Input | Required observable result |
|---|---|---|
| DEP-F01 | Project App depends on Notes and Messenger in one user-owned dedicated Space; actor has relevant grants. | Named Notes/Messenger operations succeed through Core; no duplicate private data store or second app-specific authorization path. |
| DEP-F02 | Same dependency declared and activated, but actor lacks access to one private Note. | Note read, search hit, title, preview and backlink are denied/filtered; other authorized Project data remains usable. |
| DEP-F03 | Package publisher and Space member have different identities; publisher asks to read a customer's Note. | Publisher status alone grants nothing; request fails without that actor's independent Space/Resource permission. |
| DEP-F04 | Source instance in Space A references target in Space B, or a copied Resource ID from another Persona. | No access follows from ID, relation or same package; an unsupported cross-Space contract fails explicitly. |
| DEP-F05 | Target grant is revoked or seven-day shared-read reconciliation limit expires while an App view/handler remains open. | Every subsequent dependency read/write is rechecked; no cached handle, projection or error DTO exposes forbidden content. |
| DEP-F06 | Dependency version changes or mandatory capability disappears between discovery and activation/call. | Activation/call fails with typed compatibility state; no wrong-target substitution or lossy fallback. |
| DEP-F07 | Update adds a new Notes view, effect or broader data scope. | Release is not classified `compatible-auto`; previous approval is not silently widened. |
| DEP-F08 | ServicePrincipal or automation invokes a dependency without explicit delegated capability, or retries after revocation. | Core denies the operation regardless of handler result; duplicate/replay cannot restore the old grant. |
| DEP-F09 | Dependency is optional and absent; another required dependency is absent. | Optional feature shows explicit unavailable/degraded state; required dependency prevents only affected AppInstance activation, not the entire VIDA shell. |

## Still open

1. Exact versioned manifest schema, target-selection scope and whether cross-Space calls are supported at all in Release 1.
2. Which authorized Space actor approves each dependency's requested operation/data scope, and how bundled presets are reviewed without turning them into implicit grants.
3. Proof and lifecycle for service delegation, grant revocation, optional dependency degradation and update-expanded scope.
4. Executable positive/negative fixtures across Android, iOS and Windows; these table rows are plans, not conformance evidence.
