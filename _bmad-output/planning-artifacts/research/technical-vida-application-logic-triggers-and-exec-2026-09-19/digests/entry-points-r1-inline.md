# Entry points — round 1 (inline)

Accessed 2026-09-19. Primary sources only. Publisher dates: Airtable overview 2026-08-12; Airtable match-conditions 2026-09-18; Appwrite docs undated.

| Claim | Source | Publisher | Date | Confidence/class |
|---|---|---|---|---|
| Airtable offers record created/updated/condition/view, form, scheduled, webhook, button, and integrated-app triggers; an automation binds one trigger to actions. | https://support.airtable.com/articles/3669392397-getting-started-with-airtable-automations | Airtable | 2026-08-12 | medium / pattern |
| Airtable `updated` is not `created`; field edits may trigger before the user has finished filling a record. | https://support.airtable.com/articles/4192541920-airtable-automation-trigger-when-record-updated ; https://support.airtable.com/articles/3669392397-getting-started-with-airtable-automations | Airtable | 2026-08-12 | medium / pattern |
| Match-condition automations fire when crossing into a matching state, and can fire again after leaving/re-entering; existing matching records do not automatically trigger. | https://support.airtable.com/articles/7369519794-airtable-automation-trigger-when-record-matches-conditions | Airtable | 2026-09-18 | medium / pattern |
| Appwrite supports synchronous endpoint/SDK executions versus asynchronous event/scheduled executions. Event functions can recursively trigger themselves. | https://appwrite.io/docs/products/functions/execute | Appwrite | undated | medium / pattern |

Lead: separate *named intent/command* from broad record-change event and from schedule/external ingress; distinguish event occurrence from completion of user editing. No reference proves a VIDA-specific hook taxonomy.
