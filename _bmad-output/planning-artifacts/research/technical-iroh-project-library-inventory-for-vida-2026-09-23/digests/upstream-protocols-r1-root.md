# Upstream Iroh protocol libraries: dependency and maturity digest

Accessed 2026-09-23. Publisher: n0-computer. Evidence is repository-owned manifests/README at pinned commit, official changelogs and docs.rs release listing. A manifest range is not a tested VIDA version tuple.

| Library | Current evidence | Role and limitation | VIDA decision signal |
|---|---|---|---|
| `iroh` | [docs.rs 1.2.0](https://docs.rs/crate/iroh/1.2.0) lists release 2026-09-09. | Authenticated direct/relay QUIC endpoint foundation; not an application schema, authority, offline mailbox or conflict engine. | Already adopted by VIDA as transport foundation; adapters still need conformance. |
| `iroh-gossip` | [manifest at `f128b987`](https://github.com/n0-computer/iroh-gossip/blob/f128b98705f5ad3a28a92f1d2b483686348b6ae8/Cargo.toml) says crate `0.101.0`, optional `iroh = "1"`; [README](https://github.com/n0-computer/iroh-gossip/blob/f128b98705f5ad3a28a92f1d2b483686348b6ae8/README.md) describes epidemic broadcast trees, optional pure protocol state machine and Iroh network module. [Changelog](https://github.com/n0-computer/iroh-gossip/blob/f128b98705f5ad3a28a92f1d2b483686348b6ae8/CHANGELOG.md) records Iroh 1.0 migration on 2026-06-15. | Dissemination of transient topic messages, not proof of durable recipient apply. | Prototype for presence/fanout only; retain VIDA outbox/receipts for durable delivery. |
| `iroh-blobs` | [manifest at `e82cbdcb`](https://github.com/n0-computer/iroh-blobs/blob/e82cbdcbdac9a78033174aad55e3199b2cf4c0dc/Cargo.toml) says crate `0.103.0`, `iroh = "1.0.0"`. [README](https://github.com/n0-computer/iroh-blobs/blob/e82cbdcbdac9a78033174aad55e3199b2cf4c0dc/README.md) describes BLAKE3-verified range/sequence transfer and explicitly says current version is **not production quality**, recommending 0.35 for production. [Changelog](https://github.com/n0-computer/iroh-blobs/blob/e82cbdcbdac9a78033174aad55e3199b2cf4c0dc/CHANGELOG.md) records Iroh 1.0 migration on 2026-06-15. | Verified byte transport/store; not VIDA blob authorization, encrypted manifest, quota or GC policy. The recommended stable [0.35.0 manifest](https://github.com/n0-computer/iroh-blobs/blob/v0.35.0/Cargo.toml) depends on `iroh = "0.35"`, so it is not a direct Iroh-1.2 companion. | Critical prototype/risk gate: either demonstrate 0.103 safely under VIDA fixtures or use a different transfer implementation; do not silently combine old 0.35 with current core. |
| `iroh-docs` | [manifest at `8cfeacb0`](https://github.com/n0-computer/iroh-docs/blob/8cfeacb087b4b195b1930683aa4448e990da0659/Cargo.toml) says crate `0.101.0`, `iroh = "1"`, `iroh-blobs = "0.103"`, `redb = "4.1"`; [README](https://github.com/n0-computer/iroh-docs/blob/8cfeacb087b4b195b1930683aa4448e990da0659/README.md) describes namespace/author signed key-value entries with content hashes/timestamps, range-based set reconciliation, redb file store and integration with blobs+gossip. | Useful sync/reference layer, but its namespace write capability and timestamp entry model do not establish VIDA roles, owner approval, exclusive operations or transparent conflict resolution. It inherits the `iroh-blobs` production-quality caveat for production use. | Prototype only for reconciliation and integration; VIDA signed operation/authority log remains separate. |

## Cross-source insight and limitations

- The current upstream trio `gossip 0.101` + `blobs 0.103` + `docs 0.101` advertises Iroh-1.x manifest ranges, but no retrieved evidence here proves the exact tuple builds with VIDA-pinned Iroh 1.2 on Android/iOS/Windows. Release-build matrix remains required.
- `iroh-blobs` current README's own production warning is stronger negative evidence than star counts or examples in other projects. The older recommended 0.35 belongs to the pre-1.x transport generation; choosing it would need a distinct compatibility design, not a quiet version substitution.
- `iroh-docs` can be a useful reference without becoming the domain source of truth: its entry metadata and capability model differ from VIDA's already adopted authority/receipt semantics. This is an architectural inference from the cited README and VIDA requirements, not an upstream claim about VIDA.
- Default-branch commit dates are reported in the activity snapshot below. Exact resolved dependency graph from lockfiles and physical-device behavior are not established by this digest.

## Development momentum snapshot (GitHub API, 2026-09-23)

Numbers below describe activity, not quality or adoption suitability. `main` commits count the last 30 days (since 2026-08-24 UTC); merged PRs and closed issues count the last 90 days (since 2026-06-25 UTC). Open issues and PRs are separated because GitHub's `open_issues_count` combines them. Repository `pushed_at` is **not** the date of the latest default-branch commit.

| Repo | Latest `main` commit | `main` commits / 30d | Merged PR / 90d | Closed issues / 90d | Open issues / PR |
|---|---|---:|---:|---:|---:|
| [`iroh`](https://github.com/n0-computer/iroh/commits/main/) | 2026-09-21 | 27 | 80 | 28 | 145 / 28 |
| [`iroh-blobs`](https://github.com/n0-computer/iroh-blobs/commits/main/) | 2026-06-15 | 0 | 0 | 0 | 61 / 17 |
| [`iroh-docs`](https://github.com/n0-computer/iroh-docs/commits/main/) | 2026-08-19 | 0 | 6 | 0 | 25 / 14 |
| [`iroh-gossip`](https://github.com/n0-computer/iroh-gossip/commits/main/) | 2026-07-30 | 0 | 1 | 1 | 19 / 19 |
| [`iroh-ffi`](https://github.com/n0-computer/iroh-ffi/commits/main/) | 2026-07-28 | 0 | 18 | 10 | 11 / 3 |

API evidence: GitHub REST [repository](https://docs.github.com/en/rest/repos/repos#get-a-repository), [commits](https://docs.github.com/en/rest/commits/commits#list-commits) and [issue search](https://docs.github.com/en/rest/search/search#search-issues-and-pull-requests), queried for each repository by `gh api` on 2026-09-23. The count is a point-in-time sample; it does not prove maintainer response time or feature completeness. Review concrete PR/issues before treating a backlog as an adoption risk.
