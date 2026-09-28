---
id: SPEC-NODE-HOST-001
status: approved
implementation_status: unplanned
last_updated: 2026-09-19
requirement_refs:
  - ../02-requirements/transport-sync-requirements.md
  - ../02-requirements/platform-nfr.md
decision_refs:
  - ../03-architecture/decisions/ADR-0005-iroh-transport-foundation.md
---

# SPEC-NODE-HOST-001: VidaNodeHost contract

## Purpose

Забезпечити одну узгоджену Iroh transport boundary для process/platform profile.

## Contract

`VidaNodeHost` `MUST` own:

- Router та registry VIDA ALPN handlers;
- persistent/ephemeral endpoint profiles and bindings;
- Address Lookup providers/cache;
- relay configuration and path diagnostics;
- platform network-change, start, suspend, resume and shutdown lifecycle.

`VidaNodeHost` `MUST NOT` own Persona, Space authorization, domain log, message history or blob retention policy.

## Abstract interfaces

```text
BindProtocol(alpnMajor, handler)
OpenEndpoint(profile, deviceBinding?) -> EndpointHandle
Dial(endpointRef, alpnMajor) -> Connection
ObservePath(connection) -> direct | relay | unknown
RotateEndpoint(profile, reason)
Shutdown(gracefulDeadline)
```

Every command `MUST` be cancellable and return typed transport/lifecycle errors. Endpoint secret material `MUST NOT` cross the port boundary.

## Lifecycle and failure behavior

- persistent profile restart preserves its authorized binding;
- ephemeral profile restart MUST NOT silently reuse a previous unlinkability context;
- Address Lookup failure is distinct from relay, NAT and peer rejection;
- network change triggers path re-evaluation without mutating Persona or Space state;
- shutdown stops new dials, drains handlers to deadline and records no secret material.

## Compatibility

ALPN major selects breaking protocol version. Minor/optional features negotiate inside the selected protocol. Unknown ALPN or mandatory feature fails explicitly. FFI transport coverage does not define VIDA protocol semantics: every binding `MUST` call the normative codec/kernel or pass the same byte-exact golden vectors.

## Acceptance criteria

- two protocol handlers coexist on one Router;
- direct↔relay path changes preserve connection-level application semantics;
- persistent and ephemeral profiles pass correlation and recovery tests;
- injected lookup/relay/network failures map to distinct typed errors;
- domain contract tests run with an in-memory `NodeHost` substitute.
- release artifact records exact Iroh/dependency versions and checksums in lockfile plus SPDX SBOM.

## Deferred

Concrete Rust/FFI API, lookup provider, relay provider, endpoint-secret storage mechanism and background-runtime implementation.
