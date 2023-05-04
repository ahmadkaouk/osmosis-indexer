//! An Indexer for the Osmosis Blockchain
//!
//!

use clap::Parser;
use std::fs;

/// An Indexer for the Osmosis Blockchain
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the config file
    #[arg(short, long, default_value = "Config.toml")]
    config_path: String,
    /// The height to start indexing from
    #[arg(long, default_value = "0")]
    height: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config =
        fs::read_to_string(args.config_path).expect("Something went wrong reading the file");
}
