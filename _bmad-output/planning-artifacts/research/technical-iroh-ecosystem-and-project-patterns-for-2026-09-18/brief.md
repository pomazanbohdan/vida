# Research brief: Iroh ecosystem and project patterns for Vida

## Decision

Determine which Iroh-based projects, protocols and implementation patterns Vida should adopt directly, adapt behind its own contracts, use only as reference, or reject.

## Hard constraints

- Iroh remains the primary transport foundation.
- Vida is local-first, open-source at its core and supports direct P2P plus relay paths.
- The messaging core must not depend on SMTP, IMAP, MIME or global email identity.
- Architecture must support offline/asynchronous delivery, multi-device, privacy, groups, blobs and eventual node federation.
- Transport identity, Vida Persona, authorization and durable application state remain separate concerns.

## Dimensions

1. Current Iroh 1.x architecture, official protocol crates, deprecations and compatibility.
2. Delta Chat/Chatmail Iroh usage and reusable/non-reusable subsystems.
3. Other active projects using Iroh or implementing relevant local-first protocols.
4. Durable delivery, push/wakeup, membership/key rotation, blobs and large-group patterns.
5. Ecosystem health, licenses, operational burden and reuse cost.
6. Decision matrix: `adopt | adapt | reference | reject`.

## Method

Technical Deep Recon; breadth-first; standard preset; normal validation. Primary sources first: official docs, repositories, source code, releases and issue trackers. Local Vida research shapes the questions but is not external evidence.

