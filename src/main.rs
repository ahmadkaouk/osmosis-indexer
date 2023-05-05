//! An Indexer for the Osmosis Blockchain
use anyhow::Result;
use clap::Parser;
use std::{fs, time::Duration};
use toml;

mod db;
mod indexer;
mod utils;
use utils::Config;
mod error;
use error::IndexerError;

/// An Indexer for the Osmosis Blockchain
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the config file
    #[arg(short, long, default_value = "Config.toml")]
    config_path: String,
    /// The height to start indexing from
    #[arg(long, default_value = "9479346")]
    height: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.height == 0 {
        return Err(IndexerError::InvalidHeight(args.height as i64).into());
    }

    let config: Config = toml::from_str(
        &fs::read_to_string(args.config_path).map_err(|_| IndexerError::ConfigFileNotFound)?,
    )?;

    let db = db::DB::new(&config.db_url).await?;

    let mut indexer = indexer::Indexer::new(config.rpc_url, db)?;
    indexer
        .run(args.height, Duration::from_secs(config.fetch_interval))
        .await?;
    Ok(())
}
