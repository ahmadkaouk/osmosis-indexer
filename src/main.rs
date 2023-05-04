//! An Indexer for the Osmosis Blockchain
use anyhow::Result;
use clap::Parser;
use std::fs;
use toml;

mod db;
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
    #[arg(long, default_value = "0")]
    height: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config: Config = toml::from_str(
        &fs::read_to_string(args.config_path).map_err(|_| IndexerError::ConfigFileNotFound)?,
    )?;

    let db = db::DB::new(&config.db_url).await?;
    Ok(())
}
