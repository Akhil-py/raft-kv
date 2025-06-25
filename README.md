# raft-kv: Distributed Key-Value Store (Rust, Raft Consensus)

## Executive Summary
A robust, distributed key-value store built in Rust, implementing the Raft consensus algorithm from scratch. Designed for strong consistency, high availability, and operational transparency—mirroring the reliability and scalability standards of FAANG-scale systems.

---

## Problem Statement
Modern distributed systems require consistent, fault-tolerant state replication. Existing solutions (etcd, Consul, TiKV) are complex or opaque. This project demonstrates a clear, correct, and extensible Raft-based KV store, suitable for production and educational use.

---

## Solution Overview
- **Raft Consensus**: Implements leader election, log replication, and command commitment for strong consistency.
- **Clustered KV Store**: Exposes a simple HTTP API for `GET`/`PUT` operations, replicated across nodes.
- **Operational Transparency**: Structured logging, health checks, and metrics-ready design.
- **Extensible Architecture**: Modular codebase for easy feature addition (snapshots, persistence, metrics).

---

## Architecture & Design
- **Roles**: Follower, Candidate, Leader (Raft roles)
- **Communication**: Warp-based HTTP endpoints for Raft RPCs and client API
- **Concurrency**: Tokio async runtime for scalable, non-blocking IO
- **State Machine**: In-memory KV store, updated via Raft log application (in progress)
- **Persistence**: (Planned) Write-ahead log and snapshotting for durability

---

## Current Progress
- [x] Project setup with Rust, Tokio, Warp, Serde, and modular structure
- [x] Config parsing and node startup
- [x] Raft state struct and log entry struct (with serialization)
- [x] RPC message structs for Raft (RequestVote, AppendEntries, etc.)
- [x] HTTP endpoints for Raft RPCs (RequestVote, AppendEntries)
- [x] Outbound RPC logic for sending RequestVote
- [x] Shared state management with Arc<Mutex<RaftState>>
- [x] Health check endpoint
- [x] Basic cluster bootstrapping
- [ ] Real Raft voting logic in handle_request_vote (in progress)
- [ ] Log replication/consistency in handle_append_entries
- [ ] Leader election, term management, state transitions
- [ ] Heartbeat and election timers (Tokio tasks)
- [ ] Apply committed log entries to the key-value store
- [ ] Client API for KV operations (set/get/delete)
- [ ] Forwarding client requests to the leader
- [ ] Persistence (WAL, snapshots) for durability
- [ ] Integration/fault tolerance tests, partition simulation

---

## Engineering & Leadership Principles
- **Customer Obsession:** Delivering correctness, reliability, and clear, intuitive APIs for end users and developers.
- **Ownership:** Built entirely in-house, with no reliance on third-party Raft libraries, ensuring full control and deep understanding.
- **Highest Standards:** Emphasizing rigorous testing, modular design, and code clarity to enable maintainability and scalability.
- **Bias for Action:** Rapid iteration and delivery of a minimal, testable, and extensible MVP.
- **Learn and Be Curious:** Continuously inspired by industry leaders, with a focus on learning, innovation, and sharing knowledge.

---

## Getting Started

### Build
```
cargo build
```

### Run 3 Nodes (separate terminals)
```
cargo run -- --id node1 --port 8001 --peers localhost:8002,localhost:8003
cargo run -- --id node2 --port 8002 --peers localhost:8001,localhost:8003
cargo run -- --id node3 --port 8003 --peers localhost:8001,localhost:8002
```

### Health Check
```
curl localhost:8001/health
```

---

## Roadmap / Next Steps
- [ ] Complete real Raft voting logic and log consistency checks
- [ ] Implement leader election and heartbeat timers
- [ ] Apply committed log entries to the in-memory key-value store
- [ ] Expose client API for KV operations (set/get/delete)
- [ ] Add persistence (WAL, snapshots) for durability
- [ ] Add integration and fault tolerance tests
- [ ] Implement cluster membership changes and metrics (optional)

---

## Technologies Used
- **Rust**: Safety, performance, and concurrency
- **Tokio**: Asynchronous runtime
- **Warp**: HTTP server for API and Raft RPCs
- **Serde**: JSON serialization
- **Clap**: CLI argument parsing
- **env_logger**: Structured log output

---

## Resources
- https://raft.github.io/
- https://raft.github.io/raft.pdf
- https://www.youtube.com/watch?v=ro2fU8_mr2w&t=654s

---

## License
MIT
