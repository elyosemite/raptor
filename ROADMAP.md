# Implementation Roadmap

Ordered backlog for raptor, most important / blocking first. This is the
source of truth for "what's next" — `CLAUDE.md` points here instead of
duplicating it. Update this list as items are implemented or priorities
change.

## 1. Log backend selection

`proxy::handle` only logs on error. Add an info-level log on every
selection (`tracing::info!`) showing which backend a connection was routed
to — needed to manually observe round robin working, and useful once
health checks/failover exist.

## 2. Native integration test for the proxy

Spin up a few in-process `TcpListener`s as fake backends inside a `tests/`
integration test, point `server::run` at them, connect clients, and assert
the round-robin distribution. No Docker/testcontainers needed for this.

## 3. Health checks

Backends must be probed (actively or passively) so dead backends stop
receiving traffic. This is the single feature that most separates "TCP
forwarder" from "load balancer."

## 4. Failure handling / failover

If `TcpStream::connect(backend)` fails in `proxy::handle`, the connection
currently just fails and logs an error. Should retry against another
backend before giving up.

## 5. Timeouts

`copy_bidirectional` in `proxy.rs` has no timeout — a stalled client or
backend leaks the connection/task forever. Needs connect/read/write
timeouts.

## 6. Metrics / observability

Structured counters beyond logs: connections per backend, error rate,
latency. Needed to actually see if round robin is distributing correctly
and to catch degradation.

## 7. Graceful shutdown / connection draining

Allow removing a backend from rotation, or stopping raptor itself, without
abruptly killing in-flight connections.

## Deferred (not yet justified)

- **Testcontainers-based integration tests** — revisit once there are real
  HTTP backends, TLS, or behavior a Rust-native fake can't reproduce.
  Today's TCP proxy is fully testable in-process (see #3).
