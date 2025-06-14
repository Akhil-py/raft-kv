use clap::Parser;
use std::net::SocketAddr;
use warp::Filter;

mod config;
use config::Config;

async fn start_http_server(addr: SocketAddr) {
    let health = warp::path!("health").map(|| warp::reply::json(&"ok"));

    warp::serve(health).run(addr).await;
}

#[tokio::main]
async fn main() {
    env_logger::init(); // enables log output

    let config = Config::parse();
    log::info!("Starting node: {:?}", config);

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    log::info!("Binding HTTP server to {}", addr);

    start_http_server(addr).await;
}

