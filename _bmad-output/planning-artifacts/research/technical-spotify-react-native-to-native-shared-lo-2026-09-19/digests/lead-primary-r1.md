# Lead digest — primary-source pass

Accessed: 2026-09-19

## Findings

- **Claim:** The referenced September 2026 migration is Shopify, not Spotify. Shopify says it is moving its mobile apps from React Native to native Swift and Kotlin. **Source:** https://shopify.engineering/back-to-native — Shopify Engineering, 2026-09-10. **Confidence:** high. **Class:** landscape.
- **Claim:** Shopify does not describe React Native as a technical failure: it says React Native was successful and fast, while coding agents changed the economics of maintaining two native implementations. **Source:** https://shopify.engineering/back-to-native — Shopify Engineering, 2026-09-10. **Confidence:** high. **Class:** architecture rationale.
- **Claim:** Shopify's replacement for a shared mobile implementation is not simply duplicated code. It uses native Swift/Kotlin implementations while maintaining parity through shared specifications, tests, review checkpoints, and release process. **Source:** https://shopify.engineering/back-to-native and https://shopify.engineering/shop-app-migration — Shopify Engineering, 2026-09-10. **Confidence:** high. **Class:** architecture pattern.
- **Claim:** Shopify's stated agent-addressable architecture principle is to decouple business logic completely from UI, run it headlessly on desktop, and expose it through a CLI for millisecond feedback without a simulator. **Source:** https://shopify.engineering/back-to-native — Shopify Engineering, 2026-09-10. **Confidence:** high. **Class:** integration.
- **Claim:** Migration is checkpoint-driven: agents inspect the source, document behavior, prepare per-platform plans, implement, and undergo tests, visual/event parity checks, adversarial reviews, and human approval. Plan acceptance is bound to the plan content hash. **Source:** https://shopify.engineering/back-to-native and https://shopify.engineering/shop-app-migration — Shopify Engineering, 2026-09-10. **Confidence:** high. **Class:** implementation reality.
- **Claim:** Shopify previously used a separate Kotlin Multiplatform library for background sync: common OS-agnostic logic, narrow platform wrappers, and sandbox apps. This is evidence that selective shared executable cores and native/platform shells can coexist, but it predates the 2026 native rewrite and does not prove that the new Shop app uses KMP. **Source:** https://shopify.engineering/managing-native-code-react-native — Shopify Engineering, 2021-04-16. **Confidence:** high for the historical pattern; unverified for the 2026 app. **Class:** architecture precedent.
- **Claim:** A 2025 Shopify pattern kept some business logic on the web and rendered/invoked native UI through Mobile Bridge for non-critical surfaces. This demonstrates tiered implementation strategies, not one universal client architecture. **Source:** https://shopify.engineering/mobilebridge-native-webviews — Shopify Engineering, 2025-04-25. **Confidence:** high. **Class:** contrary/adjacent evidence.

## Leads and gaps

- The public 2026 articles do not identify the language or binary boundary used by the headless business-logic core.
- They do not say that Swift and Kotlin share one executable business-logic implementation; the explicit shared artifacts are specifications, tests, checkpoints, behavior and release gates.
- Helix and Tardis implementation details are not yet public enough to treat as reusable components.
