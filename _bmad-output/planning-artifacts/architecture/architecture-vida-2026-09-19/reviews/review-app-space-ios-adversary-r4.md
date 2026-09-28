# Reviewer gate — App Space and iOS adversary r4

Verdict: PASS with dependency grants still open.

- A malicious publisher cannot gain ownership by declaring a dedicated Space because genesis requires the activating user's signature.
- A complex App cannot replace the standard permission/key/sync model; its dedicated Space is an ordinary VIDA Space.
- A simple extension is not forced into a separate Space, avoiding unnecessary identity and key boundaries.
- An iOS package cannot hide executable logic under the declarative profile without failing capability/content validation.
- A client cannot silently fall back from unsupported handler execution to partial behavior; it returns compatibility failure.
- Remaining open item is narrow and explicit: default grants and manifest semantics for bundled versus external App dependencies.

