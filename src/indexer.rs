use crate::{data::BlockInfo, db::DB, utils::IndexerConfig};
use anyhow::Result;
use futures::{future::try_join_all, stream::FuturesUnordered};
use std::{cmp::max, time::Duration};
use tendermint::block::Height;
use tendermint_rpc::{Client, HttpClient};
use tokio::time::Interval;

/// Indexer for the Osmosis Blockchain
pub struct Indexer {
    db: DB,
    client: HttpClient,
    current_height: i64,
    fetch_interval: Interval,
}

impl Indexer {
    /// Create a new Indexer instance with the given rpc_url and db connection.
    /// The indexer will start indexing from the given height
    pub fn new(config: IndexerConfig, db: DB) -> Result<Self> {
        let client = HttpClient::new(config.rpc_url.as_str())?;

        Ok(Self {
            db,
            client,
            current_height: 0,
            fetch_interval: tokio::time::interval(Duration::from_secs(config.fetch_interval)),
        })
    }

    /// Run the indexer, starting from the given height
    pub async fn run(&mut self, height: i64) -> Result<()> {
        // Set the current height to the max of the given height and the latest height in the database

        loop {
            self.fetch_interval.tick().await;
            self.current_height = max(self.db.get_latest_block_height().await? + 1, height);
            self.fetch_data().await?;
        }
    }

    async fn fetch_blocks(
        rpc_client: HttpClient,
        start_height: i64,
        end_height: i64,
    ) -> Result<Vec<BlockInfo>> {
        let blocks = rpc_client
            .blockchain(Height::try_from(start_height)?, end_height.try_into()?)
            .await?
            .block_metas
            .into_iter()
            .map(|meta| BlockInfo::from(meta))
            .collect::<Vec<BlockInfo>>();

        Ok(blocks)
    }

    /// Fetch data from the blockchain and store it in the database
    pub async fn fetch_data(&mut self) -> Result<()> {
        let latest_height: i64 = self.client.latest_block().await?.block.header.height.into();
        let heights: Vec<i64> = (self.current_height..=latest_height).collect();
        let chunks = heights.chunks(20);
        let _ = chunks
            .map(|chunk| {
                let start_height = chunk[0];
                let end_height = *chunk.last().unwrap();
                let rpc_client = self.client.clone();
                let db = self.db.clone();
                tokio::spawn(async move {
                    let blocks =
                        Self::fetch_blocks(rpc_client.clone(), start_height, end_height).await?;
                    db.store_blocks(&blocks).await?;
                    Result::<(), anyhow::Error>::Ok(())
                })
            })
            .collect::<FuturesUnordered<_>>();

        Ok(())
    }
}
