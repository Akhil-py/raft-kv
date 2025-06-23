/// Core Raft state machine: terms, roles, votes, timers

use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use rand::Rng;
use std::collections::HashMap;

enum Role {
    Follower,
    Candidate,
    Leader,
}

struct LogEntry {
    term: u64,
    command: String,
}

struct RaftState {
    current_term: u64,
    voted_for: Option<String>,
    votes_received: u64,
    peers: HashMap<String, String>,
    role: Role,
    commit_index: u64,
    last_applied: u64,
    log: Vec<LogEntry>,
}
