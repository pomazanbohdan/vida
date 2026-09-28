---
type: technical
topic: Iroh project library inventory for VIDA decisions
decision: Which existing Iroh-ecosystem libraries and integration patterns merit VIDA prototype gates, without selecting a production dependency from repository popularity alone?
date: 2026-09-23
status: approved-by-user-request
---

# Research plan

- **Intent:** Run a current, source-backed library/dependency audit of Iroh-based references in the existing VIDA catalog; update the catalog with a new dated addendum. This is evidence gathering for later selection, not a final library ADR.
- **Topology:** breadth-first, three parallel Luna agents. Round 1: (A) local-first documents/storage/merge, (B) messaging/workspaces/security, (C) VPN/mobile/transport/bindings. Further round follows material gaps, contradictions or missing decision-critical projects.
- **Evidence:** repository-owned manifests, lockfiles, code paths, tests, releases and licenses at identifiable revision; README claims marked separately. Record source URL, publisher, commit/release date if available, access date and confidence. No conclusion from prior VIDA research alone.
- **Questions:** Which crates/packages do projects actually use for Iroh, structured sync, CRDT, blob transfer, storage, cryptography, UI/FFI, media and diagnostics? Which are architectural dependencies versus incidental UI choices? Which versions and maintenance/security/licensing signals change VIDA feasibility? Where do examples fail VIDA requirements such as equal peers, offline durability, mobile lifecycle and transparent conflict handling?
- **Scope/effort:** deep technical survey, up to two bounded rounds per dimension, primary sources first, three agents at a time. Repository manifests are evidence of dependencies, not evidence that the dependency satisfies VIDA acceptance criteria.
- **Output:** per-agent source digests, decision-grade `research.md`, new dated `research/` addendum and links in existing index/traceability. Follow-up library choices remain prototype/ADR gates.
