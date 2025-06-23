/// Structs + logic for RPC messages (RequestVote, AppendEntries)

#[derive(Serialize, Deserialize)]
struct RequestVote {
    term: u64,
    candidate_id: String,
    last_log_index: u64,
    last_log_term: u64,
}

#[derive(Serialize, Deserialize)]
struct VoteResponse {
    term: u64,
    vote_granted: bool,
}

#[derive(Serialize, Deserialize)]
struct AppendEntries {
    term: u64,
    leader_id: String,
    prev_log_index: u64,
    prev_log_term: u64,
    entries: Vec<LogEntry>,
    leader_commit: u64,
}

#[derive(Serialize, Deserialize)]
struct AppendEntriesResponse {
    term: u64,
    success: bool,
    match_index: u64,
}