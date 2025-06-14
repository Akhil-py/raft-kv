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
