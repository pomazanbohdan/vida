# Reviewer gate — App Space and iOS rubric r4

Verdict: PASS.

- AD-32 preserves the existing Space security boundary while allowing a complex App to present one dedicated product context.
- `WorkspaceGenesis` remains user-signed; AppPackage and Developer do not become Owner, consistent with AD-30.
- Messenger, Notes and Projects are reused as standard dependent AppInstances rather than forked implementations.
- AD-33 gives iOS v1 a single enforceable capability profile: declarative package content on built-in runtime capabilities.
- Code-bearing Rhai/Wasm/JavaScript payload rejection is covered by REQ-APP-021 and an acceptance fixture.
- Apple 4.7 remains a named future contour instead of an ambiguous v1 fallback.

