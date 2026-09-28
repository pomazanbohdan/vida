# Composition — round 1 (inline)

Accessed 2026-09-19. Primary sources only; WordPress handbook updated 2024-01-29; API pages and Shopify docs undated/current.

| Claim | Source | Publisher | Date | Confidence/class |
|---|---|---|---|---|
| WordPress distinguishes filters that transform and return a value from actions that perform tasks without returning a value. | https://developer.wordpress.org/plugins/hooks/ | WordPress | 2024-01-29 | medium / pattern |
| WordPress filter callbacks have priority order and may be removed/replaced by exact hook/callback/priority identity; this is an explicit registration contract, not automatic conflict arbitration. | https://developer.wordpress.org/reference/functions/add_filter/ ; https://developer.wordpress.org/reference/functions/remove_filter/ | WordPress | undated | medium / pattern |
| Shopify discount functions run concurrently without knowledge of each other; their outputs combine under separately defined combination/stacking rules. | https://shopify.dev/docs/api/functions/unstable/discount | Shopify | undated (unstable API) | low / pattern |
| Shopify mediates permitted external network requests for a Function through a declared fetch target and provides the response to its run target. | https://shopify.dev/docs/apps/build/functions/network-access | Shopify | undated | medium / capability |

Lead: a single number named `priority` does not define whether a base action is skipped, composed, or resumed. Each extension slot needs typed semantics; Shopify unstable page is illustrative, not a target API choice.
