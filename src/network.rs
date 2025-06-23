use crate::rpc::{RequestVote, VoteResponse};
use warp::Filter;

/// HTTP endpoints for Raft RPCs using Warp

// POST /raft/request_vote
pub fn request_vote_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("raft" / "request_vote")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_request_vote)
}

async fn handle_request_vote(req: RequestVote) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: Replace with real Raft vote logic
    let response = VoteResponse {
        term: req.term,
        vote_granted: true, // Dummy: always grant vote
    };
    Ok(warp::reply::json(&response))
}

/// POST /raft/append_entries
pub fn append_entries_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("raft" / "append_entries")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_append_entries)
}

/// Handle AppendEntries RPC
async fn handle_append_entries(req: AppendEntries) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: Replace with real Raft append logic
    let response = AppendEntriesResponse {
        term: req.term,
        success: true, // Dummy: always succeed
        match_index: 0,
    };
    Ok(warp::reply::json(&response))
}