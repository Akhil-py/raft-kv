/// Core Raft state machine: terms, roles, votes, timers

use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub enum Role {
    Follower,
    Candidate,
    Leader,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    pub term: u64,
    pub command: String,
}

pub struct RaftState {
    pub current_term: u64,
    pub voted_for: Option<String>,
    pub votes_received: u64,
    pub peers: HashMap<String, String>,
    pub role: Role,
    pub commit_index: u64,
    pub last_applied: u64,
    pub log: Vec<LogEntry>,
}
