# Iroh ecosystem projects — root digest

Accessed: 2026-09-18. Primary catalog: https://docs.iroh.computer/examples and https://github.com/n0-computer/awesome-iroh.

## Network and VPN references

- **Rayfish** — MPL-2.0 P2P mesh VPN with daemon/TUN, DHT lookup, direct-or-relay connectivity, invites, firewall, exit nodes, multi-device identity, and custom relay/discovery. Strong reference for node daemon and network control plane; not a data-sync layer. Source: https://github.com/rayfish/rayfish.
- **iroh-lan** — MIT ephemeral, no-account L3 LAN using a shared name/password, distributed discovery, virtual subnet, and TUN. Useful for temporary spaces; it intentionally lacks durable identity/state and requires same-version peers. Source: https://github.com/rustonbsd/iroh-lan.

## Notes and local-first references

- **Kith** — MIT OR Apache-2.0 no-account encrypted sync for notes/tabs/files using Iroh and Automerge. It demonstrates spaces, roles, signed membership log, epoch rekey, SPAKE2 pairing, DHT, relay, exports, and MCP. Alpha, unaudited, and partly honest-peer-dependent: reference/prototype, not production dependency. Source: https://github.com/muhamadjawdatsalemalakoum/kith.
- **Vellum** — MIT Tauri/Svelte Markdown notes, full-device replicas, QR sharing, vault documents, and MCP. It depends on the separately versioned `iroh-docs`; equal write-ticket access and endpoint-secret loss expose authority/recovery tradeoffs. Source: https://github.com/andymitch/vellum.
- **Synesis** — MIT local-first, Obsidian-compatible notes with QR pairing and direct Iroh synchronization while both peers are online. Good UX/reference for filesystem interoperability; it does not solve durable offline delivery. Source: https://github.com/grimfeld/synesis.
- **Obsiroh** — very small MIT Obsidian-sync prototype with only a few commits; insufficient evidence for an architectural dependency. Source: https://github.com/DrHongos/obsiroh.

## Framework and application references

- **p2panda** — MIT OR Apache-2.0 modular local-first stack with Iroh networking, lookup, sync, blobs, stores, spaces, encryption, and authorization. Strong architecture/protocol prototype candidate, but its pre-1.0 APIs are explicitly unstable. Source: https://github.com/p2panda/p2panda.
- **Teamtype** — AGPL-3.0 editor-agnostic collaboration over local files with Automerge, Iroh, short join codes, and a JSON-RPC editor boundary. It is used by its maintainers but maintained in free time with known bugs. Reference its local-disk truth and adapter boundary; do not embed without a licensing decision. Source: https://github.com/teamtype/teamtype.
- **GuardianDB** — MIT OR Apache-2.0 active but unstable local-first database that multiplexes blobs, docs, and gossip over Iroh and adds LWW stores, a causal event DAG, reconciliation, access control, and SQL-compatible surfaces. Valuable prototype target, not a proven VIDA data substrate. Source: https://github.com/wmaslonek/guardian-db.
- **Proscenium** — alpha P2P social/messaging app using Iroh gossip, blobs, and direct QUIC; separates master, signing, and transport keys and combines Noise IK with Double Ratchet. Direct messages queue only locally, so it does not provide recipient-offline delivery. Source: https://github.com/iohzrd/proscenium.
- **Dropwire** — file-transfer app with a dedicated `irohcore` engine as the only layer importing Iroh/iroh-blobs. Strong reference for a transport-adapter boundary and hermetic tests. Source: https://github.com/muhamadjawdatsalemalakoum/dropwire.
- **Sendme** — first-party transfer reference using Iroh connectivity and BLAKE3-addressed verified/resumable blobs. Its capability ticket and temporary provider model are useful for attachment transfer, but it is a protocol example rather than a complete product. Source: https://github.com/n0-computer/sendme.
- **Knot** — early decentralized Git prototype that places packfiles/manifests in Iroh blobs and a canonical head/peer hints in ATProto. Single-user and incomplete write-back constraints make it a reference for separating CAS data from signed canonical pointers only. Source: https://tangled.org/lmao.bsky.social/knot-iroh/.
- **git-remote-iroh** — small Apache-2.0 bridge of the existing Git smart protocol over Iroh, with stable or ephemeral endpoint identity and bearer-capability URLs. Useful proof that VIDA can tunnel a mature byte-stream protocol behind a versioned ALPN. Source: https://github.com/magik6k/git-remote-iroh.
- **iroh-live** — early MoQ media preview with platform and relay limitations and expected API change; useful only as a media experiment reference. Source: https://github.com/n0-computer/iroh-live.
- **Hubris** — listed by official Iroh examples as a collaborative application; evidence is currently limited to the official catalog description, so no deeper architectural claim is made. Source: https://docs.iroh.computer/examples.

Cross-project finding: Iroh supplies authenticated connectivity and path negotiation, while each project independently invents identity, membership, authorization, history, offline delivery, conflict handling, and recovery. VIDA must own those application semantics.

Verification note: Vellum and Synesis are real current repositories whose READMEs explicitly identify Iroh integration, but they are absent from the official Iroh examples/awesome catalogs. They remain third-party references, not ecosystem-endorsed components.

Rejected false positive: `iroh-socks-proxy` currently describes Iroh integration as a future roadmap phase, so its name is not evidence of present Iroh use.
