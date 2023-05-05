use crate::{
    data::{BlockInfo, ValidatorInfo},
    db::DB,
    utils::IndexerConfig,
};
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

    /// Fetch data from the blockchain and store it in the database
    pub async fn fetch_data(&mut self) -> Result<()> {
        let latest_height: i64 = self.client.latest_block().await?.block.header.height.into();
        let heights: Vec<i64> = (self.current_height..=latest_height).collect();
        let chunks = heights.chunks(20);
        let _ = chunks
            .map(|chunk| {
                let start_height = chunk[0];
                let end_height = *chunk.last().unwrap();
                self.fetch_blocks(start_height, end_height);
            })
            .collect::<FuturesUnordered<_>>();

        Ok(())
    }

    /// Fetch blocks from the blockchain
    fn fetch_blocks(&self, start_height: i64, end_height: i64) {
        let client = self.client.clone();
        let db = self.db.clone();
        tokio::spawn(async move {
            let blocks = client
                .blockchain(Height::try_from(start_height)?, end_height.try_into()?)
                .await?
                .block_metas
                .into_iter()
                .map(|meta| BlockInfo::from(meta))
                .collect::<Vec<BlockInfo>>();
            db.store_blocks(&blocks).await?;
            Result::<(), anyhow::Error>::Ok(())
        });
    }

    /// Fetch Validator set from the blockchain at the given height
    async fn fetch_validators(&self, height: i64) -> Result<Vec<ValidatorInfo>> {
        let validators = self
            .client
            .validators(Height::try_from(height)?, tendermint_rpc::Paging::All)
            .await?
            .validators
            .into_iter()
            .map(|validator| ValidatorInfo::from_info(validator, height))
            .collect::<Vec<ValidatorInfo>>();

        Ok(validators)
    }
}
