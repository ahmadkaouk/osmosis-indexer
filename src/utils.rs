use serde::Deserialize;

/// Config struct for the application
#[derive(Deserialize, Debug)]
pub struct Config {
    pub db_url: String,
    pub rpc_url: String,
    pub fetch_interval: u64,
}
