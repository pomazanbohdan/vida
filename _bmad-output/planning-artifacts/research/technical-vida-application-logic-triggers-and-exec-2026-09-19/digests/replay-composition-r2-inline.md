# Local-first replay and override semantics — round 2 (inline)

Accessed 2026-09-19. Primary docs. This round followed the duplicate-effect and base-continuation leads.

| Claim | Source | Publisher | Date | Confidence/class |
|---|---|---|---|---|
| Automerge synchronizes offline changes on reconnect; a change observer also sees changes from other tabs/devices. This is a state notification, not evidence of exactly-once business-action delivery. | https://automerge.org/docs/tutorial/network-sync/ ; https://automerge.org/docs/tutorial/local-sync/ | Automerge | undated | medium / local-first |
| Replicache's optimistic mutator runs locally, server-side and may be re-run on rebase; subscriptions react to both local and sync changes. It is an illustrative historical sync pattern, not a recommended dependency. | https://doc.replicache.dev/concepts/how-it-works | Replicache/Rocicorp | undated | medium / local-first |
| Temporal records workflow history and retries Activities after loss/timeouts; durable history/replay is distinct from an external operation's idempotency. | https://github.com/temporalio/documentation/blob/main/docs/encyclopedia/workflow/workflow-execution/event.mdx ; https://github.com/temporalio/documentation/blob/main/docs/encyclopedia/activities/activity-execution.mdx | Temporal | undated | medium / workflow |
| Lexical dispatches command handlers in priority order until one explicitly returns handled; default behavior is lower priority. | https://lexical.dev/docs/concepts/commands | Meta / Lexical | undated | medium / composition |
| Tiptap's extension can override a method and explicitly spread `this.parent?.()` to retain existing behavior. | https://tiptap.dev/docs/editor/extensions/custom-extensions/extend-existing | Tiptap | undated | medium / composition |

Interpretation: these sources show multiple *different* composition policies, not a universal one. VIDA should define business command override and additive post-commit subscriptions separately. Remote state application, replay/rebase and UI rerender are not automatically new business commands; an executor authority and idempotent effect identity require a separate contract.
