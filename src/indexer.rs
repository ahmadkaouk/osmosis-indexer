use crate::db::DB;
use anyhow::Result;
use futures::{future::try_join_all, stream::{FuturesOrdered, FuturesUnordered}};
use std::time::Duration;
use tendermint::block::Height;
use tendermint_rpc::{Client, HttpClient};

/// Indexer for the Osmosis Blockchain
pub struct Indexer {
    db: DB,
    client: HttpClient,
    current_height: u64,
}

impl Indexer {
    /// Create a new Indexer instance with the given rpc_url and db connection.
    /// The indexer will start indexing from the given height
    pub fn new(rpc_url: String, db: DB) -> Result<Self> {
        let client = HttpClient::new(rpc_url.as_str())?;

        Ok(Self {
            db,
            client,
            current_height: 0,
        })
    }

    /// Run the indexer, starting from the given height
    pub async fn run(&mut self, height: u64, duration: Duration) -> Result<()> {
        let mut interval = tokio::time::interval(duration);
        self.current_height = height;
        loop {
            interval.tick().await;
            println!("starting indexer");
            self.fetch_data().await?;
        }
    }

    pub async fn fetch_data(&mut self) -> Result<()> {
        let latest_height: u64 = self.client.latest_block().await?.block.header.height.into();

        let header_futures = (self.current_height..=latest_height)
            .step_by(20)
            .map(|height| {
                let end_height = std::cmp::min(height + 19, latest_height);
                let rpc_client = self.client.clone();
                tokio::spawn(async move {
                    let response = rpc_client
                        .blockchain(Height::try_from(height)?, end_height.try_into()?)
                        .await?;
                    println!("response: {:?}", response);
                    Result::<()>::Ok(())
                })
            })
            .collect::<FuturesUnordered<_>>();

        // Wait for all futures to complete and handle any errors
        try_join_all(header_futures).await?;
        Ok(())
    }
}
