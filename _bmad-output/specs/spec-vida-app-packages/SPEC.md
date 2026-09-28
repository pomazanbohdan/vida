---
id: SPEC-vida-app-packages
companions:
  - app-package-model.md
  - repository-trust-options.md
  - cross-app-dependency-conformance.md
  - ../../../docs/01-product/composable-workspace-model.md
  - ../../planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md
sources: []
---

> Canonical contract for the accepted app-package, distribution and application-logic intent. The companion and adopted documents are part of this contract; exact executor, hook protocol, manifest, trust and update policy are not implied.

# VIDA app packages and instances

## Why

VIDA is a super-app whose built-in and later downloaded applications must compose from the same capabilities without fragmenting data, permissions or client behavior. Authors need schema-driven applications and extensions with their own business behavior without a new VIDA client release for every package.

## Capabilities

- **CAP-1**
  - **intent:** A compatible external app package can be obtained and connected to VIDA without updating the client.
  - **success:** An unchanged client loads a new package's declared resource types, forms and supported workflows using existing runtime capabilities; unsupported declarations fail explicitly.
- **CAP-2**
  - **intent:** VIDA includes foundational Messenger, Knowledge/Notes and Projects/Tasks applications from the start.
  - **success:** A new standard Space contains three provisioned AppInstances, each using the same package/instance contract as an external app. In Personal Space onboarding, the user chooses which to enable/show and can enable the others later; provisioning alone does not activate handlers or grant resource access.
- **CAP-3**
  - **intent:** A package can provide separate application instances in different Spaces.
  - **success:** Two Spaces activate the same package without merging resources, ownership, grants or synchronization state.
- **CAP-4**
  - **intent:** Users can connect repositories outside the VIDA marketplace and discover compatible applications there.
  - **success:** An external repository lists an application package with its source provenance preserved; discovery does not activate it in a Space.
- **CAP-5**
  - **intent:** VIDA includes its own governed marketplace for application and extension discovery.
  - **success:** The client presents the VIDA marketplace alongside, not in place of, connected external repositories.
- **CAP-6**
  - **intent:** Repositories can distribute extensions to foundational apps and expose newer package releases.
  - **success:** VIDA distinguishes an app from an extension, identifies the intended host, and shows an available update from repository metadata without assuming automatic install or execution.
- **CAP-7**
  - **intent:** An application or extension can define business logic over schema-defined records, including additions, replacements or suppression of selected base-app behavior.
  - **success:** An instance-defined Notes save handler has precedence over the base handler only in its target AppInstance/Space; the developer can compose an independent post-save action without Owner arbitration; trusted authorization still rejects a forbidden command.
- **CAP-8**
  - **intent:** A complex application can present one dedicated product Space while reusing VIDA's foundational capabilities.
  - **success:** An authorized user creates and owns a dedicated Space offered by a test application, which activates Messenger, Knowledge/Notes and Projects/Tasks as dependent standard AppInstances under the same Space governance and sync contracts.

## Constraints

- An `AppPackage` describes schemas, relations, commands, workflows, permission declarations and UI/data ports; it may also carry managed application-logic handlers that use only supported runtime/host capabilities. The exact executor is open.
- Each `AppInstance` belongs to a Space. Existing Space authorization, ownership, identity and sync rules govern its data and commands; a link or package installation grants no access by itself.
- A dependency names a versioned API/schema contract and compatible capability set, never a membership, data/effect grant or network egress. Every dependency call rechecks the initiating actor/delegation and current Space, AppInstance, container and Resource permissions; see `cross-app-dependency-conformance.md` for candidate checks, not an approved manifest.
- Built-in and downloaded packages must not require incompatible domain or protocol implementations in the shells. Shared Rust semantics and the Conformance Plane remain authoritative.
- Client platforms may render different layouts, but a declared capability must have equivalent authorized outcomes or be reported as unsupported by that platform profile.
- Repository metadata is a discovery and update signal, not permission to install, execute, activate or access Space resources. The package's source must remain identifiable.
- A standard Space provisions all three bundled AppInstances; in Personal Space the user chooses which to enable/show. Provisioning alone runs no handlers and grants no resource access. Shared-Space activation actor remains open.
- App-level behavior may be extended or replaced only through an explicit runtime contract; application logic cannot replace trusted authorization, storage integrity, key handling, sync acceptance or Iroh transport.
- An extension's effective behavior is bound to a target `AppInstance` in one Space; installing or activating the same package elsewhere does not globally rewrite the base application or another instance.
- The application developer owns composition of business handlers within one `AppInstance`. On the same trigger, the instance-defined process takes precedence over the base-app handler; VIDA does not add an Owner conflict picker or a separate business-conflict watcher.
- iOS v1 executes declarative package content through built-in VIDA capabilities and rejects downloadable Rhai/Wasm/JavaScript handlers; an Apple 4.7 mini-app runtime is a separate future contour.
- An AppPackage/schema supplies an optimal default confirmation policy per named process/operation family; Space Admin may independently select another Core-supported profile for each process. The default is not a non-configurable minimum, and neither package nor Admin may replace Core resolver or governance rules.

## Non-goals

- Unrestricted third-party native/Dart/JavaScript/Wasm code with direct OS access or authority to replace VIDA's trusted platform invariants. Controlled application logic is not excluded by this non-goal.
- An Apple Guideline 4.7 HTML5/JavaScript mini-app catalog in iOS v1.
- Choosing the concrete repository/manifest format, publisher trust policy, signature format, auto-update policy, executable extension language or mobile/desktop UI toolkit here.
- Moving City Portal's backend or its booking authority into a client package.

## Success signal

On an unchanged VIDA client, a standard Space starts with three provisioned built-in AppInstances; Personal Space onboarding enables/shows the user's selection without reinstalling the client. A complex App can create a dedicated Space and reuse those capabilities as dependent AppInstances; a separately obtained package is connected and activated in two Spaces with isolated resources and permissions. A Notes instance handler takes precedence over its base save trigger only in that instance, while other instances stay unchanged and forbidden operations fail. iOS v1 executes the declarative tier and rejects code-bearing package payloads.

## Open Questions

- What trust, review and risk-acceptance policy applies to packages from each connected source and publisher?
- At what scope is a package obtained and cached, and who may activate an AppInstance in a Space?
- Which repository and package manifest, versioning, signature, update, migration and rollback contract is required for production?
- Which executor and hook contract supports authored logic (Rhai, Wasm/Wasmtime or another option), with what phases, limits and side-effect rules?
- Does an overridden base handler run automatically after the higher-priority instance handler, or only when the developer explicitly composes it?
- Which dependency manifest and grant UX safely connect AppInstances, including cross-Space scope, optional dependencies, delegated service principals and update-expanded data access (OQ-0070)?
