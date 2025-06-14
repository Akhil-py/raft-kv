use clap::Parser;

/// Configuration for a single Raft node
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    /// Unique ID of this node (e.g. "node1")
    #[arg(long)]
    pub id: String,

    /// Port to bind the HTTP server to (e.g. 8001)
    #[arg(long)]
    pub port: u16,

    /// Comma-separated list of peer addresses (e.g. localhost:8002,localhost:8003)
    #[arg(long)]
    pub peers: String,
}

impl Config {
    pub fn peer_list(&self) -> Vec<String> {
        self.peers
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }
}
