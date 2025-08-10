use crate::rpc::{RequestVote, VoteResponse, AppendEntries, AppendEntriesResponse};
use crate::raft::RaftState;
use std::sync::{Arc, Mutex};
use warp::Filter;

/// Network module: Defines HTTP endpoints and outbound RPCs for Raft consensus.
/// All networking logic for Raft protocol is centralized here for maintainability and clarity.

// POST /raft/request_vote
/// Returns a Warp filter for handling incoming RequestVote RPCs.
/// Injects shared Raft state into the handler for safe, concurrent access.
pub fn request_vote_filter(state: Arc<Mutex<RaftState>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Cloneable filter for sharing state across requests
    let state_filter = warp::any().map(move || Arc::clone(&state));
    warp::path!("raft" / "request_vote")
        .and(warp::post())
        .and(warp::body::json()) // Parse JSON body into RequestVote
        .and(state_filter)
        .and_then(handle_request_vote)
}

/// Handler for incoming RequestVote RPCs.
/// Locks Raft state, processes the vote request, and returns a VoteResponse.
async fn handle_request_vote(req: RequestVote, state: Arc<Mutex<RaftState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut raft = state.lock().unwrap();
    let resp = raft.process_request_vote(&req);
    Ok(warp::reply::json(&resp))
}

/// POST /raft/append_entries
/// Returns a Warp filter for handling incoming AppendEntries RPCs.
/// Injects shared Raft state into the handler for safe, concurrent access.
pub fn append_entries_filter(state: Arc<Mutex<RaftState>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let state_filter = warp::any().map(move || Arc::clone(&state));
    warp::path!("raft" / "append_entries")
        .and(warp::post())
        .and(warp::body::json()) // Parse JSON body into AppendEntries
        .and(state_filter)
        .and_then(handle_append_entries)
}

/// Handler for incoming AppendEntries RPCs.
/// Locks Raft state, processes the append request, and returns an AppendEntriesResponse.
/// TODO: Replace dummy logic with real Raft log replication and consistency checks.
async fn handle_append_entries(req: AppendEntries, state: Arc<Mutex<RaftState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut raft = state.lock().unwrap();
    let resp = raft.process_append_entries(&req);
    Ok(warp::reply::json(&resp))
}

/// Outbound RPC: Sends a RequestVote to a peer node and awaits a VoteResponse.
/// Used by candidates during leader election to solicit votes from other nodes.
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