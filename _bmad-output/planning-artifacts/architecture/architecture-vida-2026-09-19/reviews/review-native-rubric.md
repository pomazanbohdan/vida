# Independent rubric review — AD-20 installed native/offline VIDA clients

## Gate verdict

**Pass with one high implementation-gate clarification.** AD-20 faithfully moves first-party VIDA distribution to installed mobile/desktop applications, removes a first-party browser/PWA/Wasm client, and keeps the three core Apps useful offline without confusing pending intent with delivery or authority acceptance. ADR-0014, `REQ-CLIENT-001`–`005`, and product-boundary documents are materially aligned. The unresolved meaning of “native” for a Tauri WebView and the supported OS matrix are correctly surfaced as `OQ-0008`, but the spine should explicitly block a production shell/platform lock until that decision is accepted.

## Critical findings

None within the reviewed contour.

## High findings

### H1 — `OQ-0008` is named but not a release/architecture lock gate

- **Evidence:** AD-20 says exact native OS targets and whether Tauri WebView qualifies as the desired Windows app remain `OQ-0008`. The Deferred section says to decide desktop shell choice and supported OS matrix under that question after prototypes. The Implementation Gates section has no `OQ-0008` gate, whereas other unresolved cross-team seams (`OQ-0033`–`0037`, `OQ-0051`) are explicitly blocked.
- **Divergence:** one team can proceed with a Tauri/WebView Windows shell because it is installed; another can treat “native” as prohibiting WebView and build a different UI stack. Both could cite AD-20, yet their packaging, security model, accessibility fixtures and UI architecture diverge materially.
- **Disposition:** **Autofix.** Add an implementation gate: platform prototypes may compare candidates, but first-party Windows shell selection, supported OS release matrix and production binding contracts cannot be locked until `OQ-0008` decides the accepted meaning of “native” and OS targets. Then require platform-specific offline/a11y/security/recovery conformance evidence. This does not choose Flutter or Tauri in the review.

## Medium findings

### M1 — AD-20 under-specifies durable pending offline action

- **Evidence:** AD-20 requires “offline local reading and permitted local actions” and says outbound delivery/authority acceptance stay pending. ADR-0014 and `REQ-CLIENT-002`–`005` further require locally persisted pending actions that survive restart/process death and clear undelivered/unaccepted UX for Messenger, Notes and Projects.
- **Divergence risk:** a client could advertise an offline compose/edit flow whose intent is only volatile memory. The spine's AD-4/AD-13 durability rules constrain durable mutations, but AD-20's standalone offline promise would be clearer if it named local durable pending state and restart recovery.
- **Disposition:** **Autofix.** Say each of the three bundled Apps can read *locally available/synchronized* content and durably stage permitted local actions across restart; pending UI must distinguish local persistence, delivery and authority acceptance. Keep exact offline command catalog and not-yet-downloaded resource behavior deferred as in `REQ-CLIENT`.

### M2 — “Native” is currently a distribution claim, not yet an OS-native UI claim

- **Evidence:** ADR-0014 and AD-20 require installed apps while explicitly leaving the acceptability of Tauri HTML-in-WebView to `OQ-0008`. The stack-selection brief lists Flutter Windows and Tauri 2 as experimental candidates, not accepted frameworks.
- **Disposition:** **Correctly deferred.** Do not describe the UI stack as already selected or claim that every shell uses platform-native widgets. The H1 gate is sufficient to prevent an unapproved interpretation from becoming production architecture.

### M3 — Public City Portal web and independent client protocols stay outside the first-party ban

- **Evidence:** AD-20 and ADR-0014 distinguish VIDA first-party client products from City Portal public web, VIDA services and independent clients under ADR-0013. `product-boundary.md` keeps City Portal in a separate codebase and City Portal App inside VIDA.
- **Disposition:** **Pass.** No contradiction. If the product later wants to restrict *independent* browser clients, that would conflict with the accepted open-interoperability goal and require a separate user decision; AD-20 does not silently do so.

### M4 — City Portal App offline behavior remains correctly unpromised

- **Evidence:** AD-20 names Messenger, Knowledge/Notes and Projects/Tasks as the three offline baseline Apps. Product docs call City Portal App a separate integrated app, while its offline operations and source of truth remain open.
- **Disposition:** **Pass.** Do not infer offline booking or portal data authority from the three-App offline guarantee. A later City Portal integration contract must decide this separately.

## Good-spine checklist

| Criterion | Result | Note |
|---|---|---|
| Fixes real cross-team divergence | **Mostly** | First-party browser target is excluded; Tauri/WebView/OS choice needs an explicit lock gate. |
| Rule enforceable and prevents stated divergence | **Pass with H1** | Release matrix can prove installed-only distribution and offline conformance; native UI meaning is still a choice. |
| Deferred items cannot cause divergence | **Partial** | `OQ-0008` is explicit but not yet an implementation gate. |
| Ratifies prior ADRs/product | **Pass** | ADR-0014, `REQ-CLIENT-001`–`005`, v1 bundle and product boundary align. |
| Covers driving capabilities | **Pass with M1** | Three offline Apps are named; durability/restart could be stated directly in AD-20. |
| Operational/platform envelope | **Pass** | AD-9 requires per-installed-OS lifecycle/key/wake/recovery gates; browser gates withdrawn. |

## Input reconciliation

- **User's no-browser VIDA request:** first-party browser/PWA/Wasm/gateway client removed; historical browser research remains non-normative.
- **User's native-client request:** installed mobile/desktop delivery accepted; whether an installed WebView shell counts as “native” remains a declared user choice, not a hidden selection.
- **User's offline request:** Messenger reads local history and queues messages; Notes and Projects read locally available content and stage edits; delivery or another authority's acceptance waits for connectivity/authorized actor. No impossible promise to fetch uncached remote content while disconnected.
- **Product boundary:** City Portal's public web is not a VIDA browser client, and its separate backend/sidecar remain in scope.
- **Open interoperability:** third-party clients and nodes can follow public protocols independently of the first-party release matrix.

## Recommended handoff

Add the `OQ-0008` production-lock gate and tighten AD-20's offline wording to durable pending actions and locally available content. No new architecture or user decision is required beyond the already-open `OQ-0008` choice.
