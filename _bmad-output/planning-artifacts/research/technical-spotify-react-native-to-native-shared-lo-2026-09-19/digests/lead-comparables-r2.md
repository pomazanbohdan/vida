# Lead digest — comparable shared-core/native-shell architectures

Accessed: 2026-09-19

## Findings

- **Claim:** 1Password uses a shared Rust Core across macOS, iOS, Windows, Android, Linux, browser and web, placing server communication, database handling, permissions enforcement, cryptography and other feasible non-UI behavior in the core while stopping short of UI. Android uses a native frontend. **Source:** https://1password.com/blog/1password-8-the-story-so-far — 1Password, 2021. **Confidence:** high. **Class:** architecture pattern.
- **Claim:** 1Password treats generated cross-language types as part of its core/frontend boundary and compiles the Rust core to WASM for browser use. **Source:** https://1password.com/blog/passkey-crates — 1Password, 2023. **Confidence:** high. **Class:** integration.
- **Claim:** Mozilla Application Services separates reusable lower-level Rust components from platform-specific high-level orchestration written in Swift/Kotlin; the platform layer owns embedding-application policy while Rust components remain independently consumable. **Source:** https://mozilla.github.io/application-services/book/design/sync-manager.html — Mozilla Application Services. **Confidence:** high. **Class:** architecture pattern.
- **Claim:** Mozilla packages the same Rust components for iOS as XCFramework/Swift Package artifacts with generated Swift bindings, illustrating a concrete native-shell/shared-Rust distribution boundary. **Source:** https://mozilla.github.io/application-services/book/design/swift-package-manager.html — Mozilla Application Services. **Confidence:** high. **Class:** integration.

## Implication under test

For a Rust/Iroh product, the strongest evidence supports a three-part split: shared headless domain/protocol core; narrow generated platform bindings; native UI and platform orchestration. The boundary should not force UI, background lifecycle, permissions UX or OS integrations into the shared core.

## Caveat

1Password and Mozilla validate the technical pattern, not Shopify's claim that coding agents make two native implementations cheaper. That economic claim remains Shopify-specific and newly published.
