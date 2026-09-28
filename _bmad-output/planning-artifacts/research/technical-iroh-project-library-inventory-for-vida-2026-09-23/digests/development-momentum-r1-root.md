# Development momentum sample: application references

Snapshot 2026-09-23 UTC. Source: GitHub REST [list commits](https://docs.github.com/en/rest/commits/commits#list-commits), [repository](https://docs.github.com/en/rest/repos/repos#get-a-repository), [list pull requests](https://docs.github.com/en/rest/pulls/pulls#list-pull-requests), queried with `gh api`. Commit count is **default-branch commits since 2026-08-24T00:00:00Z**, not unique features; it includes merges, dependency bumps and automation. `open_issues_count` includes PRs; issues below are this count minus paginated open PRs. This sample is not a quality score or proof that maintainers respond to bugs. Features/test evidence lives in domain digests.

| Repository | `main` commits / 30d | Open issues | Open PR | Evidence / interpretation limit |
|---|---:|---:|---:|---|
| [Synesis](https://github.com/grimfeld/synesis/commits/main/) | 84 | 0 | 0 | Active code/release cycle; no open tracker items does not prove defect-free. |
| [Knot](https://github.com/Fletcher-Alderton/knot/commits/main/) | 27 | 0 | 0 | Sustained sync/storage work; issue reporting channel may differ. |
| [echo](https://github.com/sergey-melnychuk/echo/commits/main/) | 14 | 0 | 0 | Focused PoC activity, not production evidence. |
| [Irokle](https://github.com/arunaengine/irokle/commits/main/) | 481 | 0 | 0 | High commit velocity; inspect changes/test coverage, not count alone. |
| [iroh-db](https://github.com/holon-technologies/iroh-db/commits/main/) | 0 | 0 | 10 | Open PR work exists despite no merged default-branch commits in this window; inspect queue age/review before inferring stagnation. |
| [Kukuri](https://github.com/kukuri-app/kukuri/commits/main/) | 415 | 33 | 1 | Broad active monorepo; issue backlog needs severity/response sampling. |
| [arachne-core](https://github.com/arachne-systems/arachne-core/commits/main/) | 12 | 0 | 1 | Focused pre-release core; check CI/test evolution and specific security fixes. |
| [iroh-beekem](https://github.com/unfoldml/iroh-beekem/commits/main/) | 0 | 2 | 0 | Last recorded main commit preceded window; inspect security/test roadmap before concluding maintenance status. |
| [Proscenium](https://github.com/iohzrd/proscenium/commits/master/) | 1 | 1 | 0 | App's alpha caveats are concrete; one in-window commit alone says little about support. |
| [Kith](https://github.com/muhamadjawdatsalemalakoum/kith/commits/master/) | 0 | 0 | 10 | Open PR queue exists without merged main commits in window; review age and tests. |
| [HoloChat](https://github.com/dkl070418/HoloChat/commits/main/) | 7 | 0 | 0 | Repo activity is visible, but Rust dependency manifest remains unverified. |
| [m2m](https://github.com/unconfirmedlabs/m2m/commits/main/) | 17 | 1 | 0 | Active bounded paid-exchange experiment, not broad VIDA chat evidence. |

GitHub push date was deliberately not used: it may reflect non-default branches or tags. A 30-day window is too short for durable maintenance assessment; release/issue timelines and security response remain follow-up gates.
