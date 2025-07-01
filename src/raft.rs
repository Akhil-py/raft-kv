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
    pub current_term: u64, // Current term number
    pub voted_for: Option<String>, // ID of the candidate voted for in this term, None if no vote
    pub votes_received: u64, // Number of votes received in the current term
    pub peers: HashMap<String, String>, // Map of peer IDs to addresses (e.g. "node1" -> "localhost:8001")
    pub role: Role, // Current role: Follower, Candidate, or Leader
    pub commit_index: u64, // Index of the highest log entry known to be committed
    pub last_applied: u64, // Index of the highest log entry applied to state machine
    pub log: Vec<LogEntry>, // Log entries for this node, ordered by index
}
