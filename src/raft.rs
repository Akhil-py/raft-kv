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

// ---- Raft logic helpers ----
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

    /// Apply all newly committed (but not yet applied) log entries to the state machine (kv_store).
    /// Simple command grammar (temporary):
    ///   set <key> <value>
    ///   del <key>
    /// Future: Replace String command with a strongly typed Command enum.
    pub fn apply_committed(&mut self) {
        while self.last_applied < self.commit_index {
            self.last_applied += 1; // Move to next entry (1-based indexing)
            if let Some(entry) = self.log.get(self.last_applied as usize - 1) {
                let parts: Vec<&str> = entry.command.split_whitespace().collect();
                if parts.is_empty() { continue; }
                match parts[0] {
                    "set" if parts.len() >= 3 => {
                        let key = parts[1].to_string();
                        // Re-join remaining tokens as value to allow spaces in value after second token.
                        let value = parts[2..].join(" ");
                        self.kv_store.insert(key, value);
                    }
                    "del" if parts.len() == 2 => {
                        self.kv_store.remove(parts[1]);
                    }
                    _ => {
                        // Unknown command – ignore for now (could log a warning)
                    }
                }
            }
        }
    }

    /// Process an incoming AppendEntries RPC (heartbeats + log replication).
    /// Implements the core safety properties: term checks, log consistency, conflict resolution, commit advancement.
    pub fn process_append_entries(&mut self, req: &crate::rpc::AppendEntries) -> crate::rpc::AppendEntriesResponse {
        // 1. Reply false if term < currentTerm (§5.1)
        if req.term < self.current_term {
            return crate::rpc::AppendEntriesResponse { term: self.current_term, success: false, match_index: 0 };
        }
        // 2. If term > currentTerm, update term and convert to follower.
        if req.term > self.current_term {
            self.current_term = req.term;
            self.role = Role::Follower;
            self.voted_for = None;
            self.votes_received = 0;
        }
        // (Timer reset would occur here in a full implementation.)

        // 3. Reply false if log doesn’t contain an entry at prevLogIndex whose term matches prevLogTerm (§5.3)
        if req.prev_log_index > self.last_log_index() { // leader references an index we don't have yet
            return crate::rpc::AppendEntriesResponse { term: self.current_term, success: false, match_index: self.last_log_index() };
        }
        if req.prev_log_index > 0 { // only check if not the virtual 0 index
            let local_term = self.log[req.prev_log_index as usize - 1].term; // 1-based -> vec index
            if local_term != req.prev_log_term {
                // Conflict: delete the entry and all that follow (§5.3) – we'll signal failure; leader will decrement nextIndex.
                return crate::rpc::AppendEntriesResponse { term: self.current_term, success: false, match_index: req.prev_log_index - 1 };
            }
        }

        // 4. Append any new entries not already present; first, remove conflicts after prev_log_index.
        // Determine the starting log position for new entries (1-based index where new entries begin)
        let mut expected_index = req.prev_log_index + 1;
        for entry in &req.entries {
            if expected_index <= self.last_log_index() { // Entry may already exist; check term for conflict
                let existing_term = self.log[expected_index as usize - 1].term;
                if existing_term != entry.term {
                    // Delete existing entry and all that follow, then append remainder of leader entries
                    self.log.truncate(expected_index as usize - 1); // keep entries before expected_index
                    // Append the rest of leader entries starting from current one
                    // We'll push current entry and rest after loop; clone needed since we iterate by ref.
                    // Collect remaining entries including current
                    let start_pos = (expected_index - (req.prev_log_index + 1)) as usize; // offset within req.entries
                    for e in req.entries[start_pos..].iter().cloned() { self.log.push(e); }
                    expected_index = self.last_log_index() + 1; // end loop
                    break;
                }
            } else {
                // Append new entry
                self.log.push(entry.clone());
            }
            expected_index += 1;
        }

        // 5. If leaderCommit > commitIndex, set commitIndex = min(leaderCommit, lastLogIndex) (§5.3)
        if req.leader_commit > self.commit_index {
            self.commit_index = std::cmp::min(req.leader_commit, self.last_log_index());
            self.apply_committed();
        }

        crate::rpc::AppendEntriesResponse { term: self.current_term, success: true, match_index: self.last_log_index() }
    }
}
