use crate::rpc::{RequestVote, VoteResponse, AppendEntries, AppendEntriesResponse};
use crate::raft::RaftState;
use std::sync::{Arc, Mutex};
use warp::Filter;

/// HTTP endpoints for Raft RPCs using Warp

// POST /raft/request_vote
pub fn request_vote_filter(state: Arc<Mutex<RaftState>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let state_filter = warp::any().map(move || Arc::clone(&state));
    warp::path!("raft" / "request_vote")
        .and(warp::post())
        .and(warp::body::json())
        .and(state_filter)
        .and_then(handle_request_vote)
}

async fn handle_request_vote(req: RequestVote, state: Arc<Mutex<RaftState>>) -> Result<impl warp::Reply, warp::Rejection> {
    // Example: lock state and access fields (replace with real Raft logic)
    let mut raft = state.lock().unwrap();
    // TODO: Implement real vote logic here
    let response = VoteResponse {
        term: raft.current_term,
        vote_granted: true, // Dummy: always grant vote
    };
    Ok(warp::reply::json(&response))
}

/// POST /raft/append_entries
pub fn append_entries_filter(state: Arc<Mutex<RaftState>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let state_filter = warp::any().map(move || Arc::clone(&state));
    warp::path!("raft" / "append_entries")
        .and(warp::post())
        .and(warp::body::json())
        .and(state_filter)
        .and_then(handle_append_entries)
}

/// Handle AppendEntries RPC
async fn handle_append_entries(req: AppendEntries, state: Arc<Mutex<RaftState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut raft = state.lock().unwrap();
    // TODO: Implement real append logic here
    let response = AppendEntriesResponse {
        term: raft.current_term,
        success: true, // Dummy: always succeed
        match_index: 0,
    };
    Ok(warp::reply::json(&response))
}

// Outbound RPC: send RequestVote to a peer
pub async fn send_request_vote(peer_addr: &str, req: &RequestVote) -> Result<VoteResponse, reqwest::Error> {
    let url = format!("http://{}/raft/request_vote", peer_addr);
    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .json(req)
        .send()
        .await?
        .json::<VoteResponse>()
        .await?;
    Ok(resp)
}