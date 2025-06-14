# raft-kv (Rust Distributed Key-Value Store)

A distributed key-value store written in Rust, powered by the [Raft consensus algorithm](https://raft.github.io/).  
This project implements a crash-resilient, strongly consistent database that replicates state across multiple nodes using leader election, log replication, and command commitment.

---

## ⚙️ Features (WIP)

- 🧠 **Raft Consensus from Scratch** – Leader election, term tracking, log replication, commit index, heartbeats  
- 🗃️ **Key-Value Storage API** – HTTP endpoints for `GET` / `PUT` operations via JSON  
- 🔁 **Cluster Replication** – State synced across 3+ nodes with Raft log  
- 📦 **Written in Rust** – Safe, performant systems code using `tokio`, `warp`, `serde`, and `clap`  
- 🧪 **Testable & Extensible** – Modular architecture ready for snapshotting, persistence, and more  

---

## 🚀 Getting Started

### 1. Build
```
cargo build
```

### 2. Run 3 Nodes (each in a separate terminal)
```
cargo run -- --id node1 --port 8001 --peers localhost:8002,localhost:8003  
cargo run -- --id node2 --port 8002 --peers localhost:8001,localhost:8003  
cargo run -- --id node3 --port 8003 --peers localhost:8001,localhost:8002  
```

### 3. Health Check
```
curl localhost:8001/health
```

---

## 📚 Planned Architecture

- Raft role system: Follower, Candidate, Leader  
- Heartbeat + election timers with Tokio tasks  
- AppendEntries + RequestVote RPCs (Warp-based HTTP endpoints)  
- Log entries with term/index/command structure  
- In-memory key-value state machine  

---

## 🧠 Why This Project?

This project explores how distributed consensus and replication actually work — built entirely from scratch, without third-party Raft libraries.  
Inspired by etcd, Consul, and TiKV, this implementation prioritizes correctness, clarity, and educational value, while laying the groundwork for performance and productionization.

---

## 🛠️ Technologies Used

- **Rust**: Ownership, safety, and concurrency  
- **Tokio**: Asynchronous runtime  
- **Warp**: HTTP server for API and Raft RPCs  
- **Serde**: JSON serialization  
- **Clap**: CLI argument parsing  
- **env_logger**: Structured log output  

---

## 📈 Roadmap / Next Steps

- [ ] AppendEntries and RequestVote endpoints  
- [ ] Log persistence (file-based WAL)  
- [ ] Snapshotting and log compaction  
- [ ] Node join/leave API  
- [ ] Fault tolerance testing (partition simulation)  
- [ ] Prometheus-compatible metrics  

---

## Resources

https://raft.github.io/
https://www.youtube.com/watch?v=ro2fU8_mr2w&t=654s

---

## 📄 License

MIT
