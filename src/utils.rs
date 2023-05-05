use serde::Deserialize;

/// Config struct for the application
#[derive(Deserialize, Debug)]
pub struct Config {
    pub db: DBConfig,
    pub indexer: IndexerConfig,
}

/// Database configurtion
#[derive(Deserialize, Debug)]
pub struct DBConfig {
    pub url: String,
}

/// Indexer configuration
#[derive(Deserialize, Debug)]
pub struct IndexerConfig {
    pub rpc_url: String,
    pub fetch_interval: u64,
}
