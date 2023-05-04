use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Config file not found")]
    ConfigFileNotFound,
}