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
    pub kv_store: HashMap<String, String>, // The actual key-value store (state machine)
}

// ---- New Raft logic helpers ----
impl RaftState {
    /// Returns the index (1-based) of the last log entry; 0 if empty.
    pub fn last_log_index(&self) -> u64 { self.log.len() as u64 }
    /// Returns the term of the last log entry; 0 if none.
    pub fn last_log_term(&self) -> u64 { self.log.last().map(|e| e.term).unwrap_or(0) }

    /// Process an incoming RequestVote RPC and return a VoteResponse.
    /// Implements Raft paper rules for voting (term checks and log up-to-dateness).
    pub fn process_request_vote(&mut self, req: &crate::rpc::RequestVote) -> crate::rpc::VoteResponse {
        // 1. Reject if request term is stale.
        if req.term < self.current_term {
            return crate::rpc::VoteResponse { term: self.current_term, vote_granted: false };
        }
        // 2. Update our term & convert to follower if term is newer.
        if req.term > self.current_term {
            self.current_term = req.term;
            self.role = Role::Follower;
            self.voted_for = None;
            self.votes_received = 0;
        }
        // 3. If we've already voted for someone else this term, reject.
        if let Some(existing) = &self.voted_for {
            if existing != &req.candidate_id {
                return crate::rpc::VoteResponse { term: self.current_term, vote_granted: false };
            }
        }
        // 4. Check candidate log is at least as up-to-date.
        let up_to_date = (req.last_log_term > self.last_log_term()) ||
            (req.last_log_term == self.last_log_term() && req.last_log_index >= self.last_log_index());
        if !up_to_date {
            return crate::rpc::VoteResponse { term: self.current_term, vote_granted: false };
        }
        // 5. Grant vote (record if first time this term).
        if self.voted_for.is_none() { self.voted_for = Some(req.candidate_id.clone()); }
        crate::rpc::VoteResponse { term: self.current_term, vote_granted: true }
    }
}
