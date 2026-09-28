# App-package model

| Element | Accepted boundary | Not yet decided |
|---|---|---|
| `AppPackage` | Portable app definition: schemas, relations, commands, workflows, permission declarations, UI/data ports, and controlled application logic over supported runtime capabilities. | Manifest format, executable representation, publisher identity, signature, versioning and update mechanics. |
| Bundled package | Messenger, Knowledge/Notes and Projects/Tasks ship with VIDA and form the standard Space baseline. All three AppInstances are provisioned; Personal Space onboarding lets the user enable/show a subset and enable the rest later. Provisioning does not run handlers or grant data access. | Which assets are embedded vs materialized on first run; exact shared-Space activation actor (`OQ-0039`). |
| External package | Can be obtained from a connected external repository as well as the VIDA marketplace without rebuilding the client when existing runtime capabilities can execute its declarations. Its source remains visible. | Publisher/source trust, install approval, cache scope, offline availability and compatibility negotiation. |
| VIDA marketplace | First-party, VIDA-governed discovery channel built into the client; it does not exclude external repositories. | Review criteria, moderation, payment and publisher onboarding. |
| External repository | Connectable package source whose metadata lets VIDA discover apps, extensions and available newer releases. | Repository format, trust bootstrap, metadata freshness, update cadence and removal behavior. |
| Extension package (`plugin`) | Targets a base `AppInstance` inside one Space; its process has precedence over base behavior on the same declared trigger. The app developer composes non-conflicting handlers without Owner-level arbitration. Core/security/sync invariants remain trusted. | Exact hook phases, base-handler continuation/fallback, error behavior and executor (Rhai, Wasm/Wasmtime or other). |
| `AppInstance` | Space-scoped identity/configuration of a package; the instance can be provisioned but not yet active. Resources retain one `OwnerSpaceId` and obey Space grants and sync. The same package can have distinct instances in different Spaces. | Who may activate, suspend, remove or transfer an instance and what removal retains. |
| Runtime and shells | Rust core/runtime owns product semantics and authorization; platform shells render declared UI and integrate OS facilities. | Exact schema interpreter, renderer profile, binding implementation and extensibility of executable primitives. |

The marketplace and external repositories differ by provenance and governance, not by domain semantics. Repository discovery, package retrieval and Space activation are distinct stages; an update indication alone changes no installed or active state. A City Portal App package can provide a VIDA entry point, while the City Portal backend and booking source of truth remain separate integration concerns.

Package-authored logic may operate on schema-defined records and app events through VIDA's mediated APIs. In the research example, a Notes/Tasks handler validates or transforms input, handles an event or replaces a named app action; the core still checks actual command authorization and commits durable operations. This does **not** approve unrestricted downloaded native/OS code or resolve `OQ-0009` (WIT/Wasm/Rhai). Unsupported runtime or presentation capabilities must be surfaced as compatibility failures, not silently approximated.

Activation of an extension's behavior is scoped to its target `AppInstance` in one Space. The underlying package can be reused, but installing it does not patch the bundled Notes package or other Notes instances. Access to referenced resources outside that instance remains subject to the existing Space grants; the scope rule is about where behavior changes, not a grant of cross-Space data access.

The app author may compose independent actions on creation, change or save. Where an instance-defined process and the base app handle the same trigger, the instance process takes precedence. This is business-behavior precedence, not an Owner selection step or a bypass of core authorization/audit. Whether the base handler continues automatically or only by an explicit call remains open.

## Evidence and inherited contracts

- User decision, 2026-09-19: choose the declarative package model; downloaded external packages and bundled foundational apps.
- User clarification, 2026-09-19: packages may carry business logic over schemas; extensions may expand, replace or suppress base-app behavior within their app context.
- User confirmation, 2026-09-19: this logic is effective only inside the application and its Workspace (VIDA Space).
- User correction, 2026-09-19: the developer composes non-conflicting logic inside the AppInstance; the instance process wins precedence over the base handler on the same trigger, without a platform conflict picker.
- [App/plugin research](../../../research/Новий Text Document (3).txt): Rhai event handlers and WIT/Wasmtime components are researched candidates, not an approved executor; Iroh remains network transport.
- [Approved composition model](../../../docs/01-product/composable-workspace-model.md): standard Space AppInstances, package declarations, Space ownership and access.
- [Architecture spine](../../planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md): headless Rust core/runtime, platform shells, versioned boundary contracts and conformance.
- [Raw research audit](../../planning-artifacts/research/technical-vida-ui-platform-and-rust-binding-strate-2026-09-19/digests/research-corpus-cases.md): package/UI schema proposals and historical alternatives are distinguished from approved decisions.
