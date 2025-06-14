/// Core Raft state machine: terms, roles, votes, timers

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
    role: Role,
    commit_index: u64,
    last_applied: u64,
    log: Vec<LogEntry>,
}
