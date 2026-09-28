# Technology/currentness review — Iroh/schema batch (2026-09-21)

Verdict: PASS after corrections; no remaining high findings.

- Iroh 1.2 `Endpoint::online()` is cited as bootstrap/ever-online convenience, not peer freshness.
- Only `finish()` plus `stopped().await == None` proves QUIC receipt of all stream bytes; it does not prove VIDA processing.
- Protobuf field/enum numbers are never reused; deleted names are reserved where name-based JSON/TextProto compatibility matters.
- Avro reference is pinned to 1.12.0. Kubernetes, Protobuf and Avro are pattern evidence, not technology mandates for VIDA.

