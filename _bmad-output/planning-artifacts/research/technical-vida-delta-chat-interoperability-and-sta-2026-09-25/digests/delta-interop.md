# Delta Chat Core and stock-client interoperability — evidence digest

Accessed: 2026-09-25. Independent research track. Official/project-primary sources.

| Claim | Evidence | Confidence |
| --- | --- | --- |
| Delta Chat Core is Rust, used by stock Android/iOS/Desktop; new integrations are directed to stdio JSON-RPC; C FFI is deprecated. | [Core README](https://github.com/chatmail/core) | High |
| 1:1 messaging is mail-based Chat-Version; groups have group ID/name, member recipient list and explicit add/remove action messages. | [Core spec](https://github.com/chatmail/core/blob/main/spec.md) | High |
| JSON-RPC API exposes account lifecycle, events and SecureJoin QR for contacts/groups. | [Core JSON-RPC API](https://github.com/chatmail/core/blob/main/deltachat-jsonrpc/src/api.rs) | High |
| webxdc carries app state updates, not a predefined VIDA Notes/CRDT semantic model; payload/ordering/conflicts remain app responsibility. Stock clients can host a `.xdc` editor but do not natively render a VIDA Note schema. | [webxdc update API](https://webxdc.org/docs/spec/sendUpdate.html); [listener](https://webxdc.org/docs/spec/setUpdateListener.html); [shared-state conflicts](https://webxdc.org/docs/shared_state/conflicts.html) | High for API; medium for stock presentation inference |
| Current Core main declares MPL-2.0 and unconditional `iroh`/`iroh-gossip` 0.35 dependencies. This differs from VIDA's accepted Iroh 1.2 baseline. | [Core Cargo.toml](https://github.com/chatmail/core/blob/main/Cargo.toml) | High |
| MPL permits a larger work with separately licensed MIT files, but covered files/modifications retain MPL source/notice duties on distribution. | [Core LICENSE](https://github.com/chatmail/core/blob/main/LICENSE) | High; legal review remains needed |
| Separate Cargo versions may coexist, but this does not make 0.35 and 1.2 wire/type contracts compatible; a JSON-RPC process boundary is an integration option. | [Cargo resolver](https://doc.rust-lang.org/cargo/reference/resolver.html); [Core Cargo.toml](https://github.com/chatmail/core/blob/main/Cargo.toml) | Medium architectural inference |
| Core browser WASM build is not documented as a supported target; experimental desktop browser frontend is not proof of static Pages operation. | [Core README](https://github.com/chatmail/core); [Desktop development docs](https://github.com/deltachat/deltachat-desktop/blob/main/docs/DEVELOPMENT.md) | Medium: absence of documentation is not impossibility |

Proof gates: Android-to-stock Delta 1:1 and group send/receive/SecureJoin; group add/remove and offline replay; build/runtime conflict of Core 0.35 with VIDA 1.2; browser WASM/network/storage feasibility separately; note envelope/webxdc duplicate/reorder/conflict; MPL artifact compliance.
