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