use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Config file not found")]
    ConfigFileNotFound,
    #[error("Invalid height {0}")]
    InvalidHeight(i64),
}
