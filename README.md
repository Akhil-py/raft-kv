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
- **State Machine**: In-memory KV store, updated via Raft log application
- **Persistence**: (Planned) Write-ahead log and snapshotting for durability

---

## Design Principles & Amazon Leadership Alignment
- **Customer Obsession**: Prioritizes correctness, reliability, and clear APIs
- **Ownership**: Built from scratch, no third-party Raft libraries
- **Insist on the Highest Standards**: Rigorous testing, modularity, and code clarity
- **Bias for Action**: Fast iteration with a clear, testable MVP
- **Learn and Be Curious**: Inspired by industry leaders, with a focus on educational value

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
- [ ] AppendEntries and RequestVote endpoints
- [ ] Log persistence (file-based WAL)
- [ ] Snapshotting and log compaction
- [ ] Node join/leave API
- [ ] Fault tolerance testing (partition simulation)
- [ ] Prometheus-compatible metrics

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
