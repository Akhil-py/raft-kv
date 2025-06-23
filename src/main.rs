use clap::Parser;
use std::net::SocketAddr;
use warp::Filter;
use std::sync::{Arc, Mutex};

mod config;
mod raft;
mod rpc;
use config::Config;
use raft::RaftState;
mod network;

async fn start_http_server(addr: SocketAddr, raft_state: Arc<Mutex<RaftState>>) {
    let health = warp::path!("health").map(|| warp::reply::json(&"ok"));
    let raft_vote = network::request_vote_filter(Arc::clone(&raft_state));
    let raft_append = network::append_entries_filter(Arc::clone(&raft_state));
    let routes = health.or(raft_vote).or(raft_append);
    warp::serve(routes).run(addr).await;
}

#[tokio::main]
async fn main() {
    env_logger::init(); // enables log output

    let config = Config::parse();
    log::info!("Starting node: {:?}", config);

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    log::info!("Binding HTTP server to {}", addr);

    // Initialize Raft state and wrap in Arc<Mutex<>>
    let raft_state = Arc::new(Mutex::new(RaftState {
        current_term: 0,
        voted_for: None,
        votes_received: 0,
        peers: config.peer_list().into_iter().enumerate().map(|(i, p)| (format!("node{}", i+1), p)).collect(),
        role: raft::Role::Follower,
        commit_index: 0,
        last_applied: 0,
        log: Vec::new(),
    }));

    start_http_server(addr, raft_state).await;
}

