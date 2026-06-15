# Project: nginxY

## Overview

raptor is an open-source load balancer written in Rust.

The goal of raptor is to provide a modern, simple, high-performance, and cloud-native load balancer focused on operational simplicity, observability, and reliability.

The project is inspired by traditional load balancers but aims to provide a better developer experience with a clean architecture and modern Rust ecosystem.

---

# Current Version

Current target: v0.0.1

The first version goal is:

- Accept TCP connections
- Select a backend using a load balancing strategy
- Forward traffic between client and backend
- Load configuration from a file

The project must remain small and focused.

---

# Core Principles

When modifying or adding code:

1. Prefer simplicity over abstraction.
2. Avoid premature architecture.
3. Do not add dependencies without a clear reason.
4. Keep modules focused on one responsibility.
5. Write production-oriented code even in early versions.
6. Favor readability over clever solutions.

---

# Technology Stack

Language:
- Rust

Runtime:
- Tokio async runtime

Configuration:
- TOML

Serialization:
- Serde

Logging:
- Tracing

Error handling:
- Anyhow for application-level errors

---

# Current Architecture

The project currently has three main modules:

```

src/

├── server/
│
├── balancer/
│
└── config/

```

---

# Module Responsibilities

## server

Responsible for networking.

Responsibilities:

- TCP listener
- Accept client connections
- Create backend connections
- Proxy traffic between client and backend
- Manage connection lifecycle

Do not place:
- Load balancing algorithms
- Configuration parsing
- Business rules


---

## balancer

Responsible for choosing the backend server.

Responsibilities:

- Maintain backend pool
- Apply balancing strategies
- Select destination backend

Current strategy:

- Round Robin

Future possible strategies:

- Least Connections
- Weighted Round Robin
- Latency Based

The server module should not know which algorithm is being used.

---

## config

Responsible for application configuration.

Responsibilities:

- Read configuration files
- Deserialize configuration
- Validate input

Example configuration:

```toml
listen = "0.0.0.0:8080"

backends = [
    "127.0.0.1:3001",
    "127.0.0.1:3002"
]
````

---

# Coding Guidelines

## Rust

Follow idiomatic Rust:

* Use ownership correctly
* Avoid unnecessary clones
* Prefer Result over panics
* Keep functions small
* Use meaningful names

Avoid:

```rust
unwrap()
```

in production paths.

---

# Dependency Policy

Before adding a crate:

Ask:

* Is this solving a real problem?
* Can this be implemented simply internally?
* Does this increase maintenance cost?

Avoid unnecessary frameworks.

---

# Version Roadmap

## v0.0.1

Implement:

* TCP load balancing
* Round Robin
* Static backend configuration
* Basic logging

## v0.1

Possible additions:

* Health checks
* Graceful shutdown
* Metrics

## v0.2+

Possible additions:

* HTTP routing
* TLS termination
* Kubernetes integration
* Observability stack

---

# Things NOT to build yet

Do not add:

* HTTP parsing
* Dashboard
* Distributed clustering
* Service discovery
* Authentication
* Complex configuration language

unless explicitly requested.

---

# Engineering Decisions

The project should evolve incrementally.

Every feature should answer:

"What user problem does this solve?"

Avoid implementing features only because competitors have them.

---

# AI Agent Behavior

When working on this repository:

1. First understand existing architecture.
2. Do not restructure modules without strong justification.
3. Explain architectural changes before applying them.
4. Prefer minimal changes.
5. Keep backwards compatibility when possible.
6. Add tests for important behavior.
7. Consider performance implications.

The priority order is:

1. Correctness
2. Simplicity
3. Performance
4. Features
