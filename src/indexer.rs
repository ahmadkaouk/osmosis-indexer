use crate::{data::BlockInfo, db::DB, utils::IndexerConfig};
use anyhow::Result;
use futures::{future::{join_all, try_join_all}, stream::FuturesUnordered};
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
        self.current_height = max(self.db.get_latest_block_height().await?, height);

        loop {
            self.fetch_interval.tick().await;
            self.fetch_data().await?;
        }
    }

    /// Fetch data from the blockchain and store it in the database
    pub async fn fetch_data(&mut self) -> Result<()> {
        let latest_height: i64 = self.client.latest_block().await?.block.header.height.into();

        let header_futures = (self.current_height..=latest_height)
            .step_by(20)
            .map(|height| {
                let end_height = std::cmp::min(height + 19, latest_height);
                let rpc_client = self.client.clone();
                let db = self.db.clone();
                tokio::spawn(async move {
                    let blocks = rpc_client
                        .blockchain(Height::try_from(height)?, end_height.try_into()?)
                        .await
                        .map_err(|e| {
                            println!("Error fetching blocks: {:?}", e);
                            e
                        })?
                        .block_metas
                        .into_iter()
                        .map(|meta| BlockInfo::from(meta))
                        .collect::<Vec<BlockInfo>>();

                    // for block in &blocks {
                    //     let rpc_client = rpc_client.clone();
                    //     let db = db.clone();
                    //     let height = block.height;
                    //     tokio::spawn(async move {
                    //         let height = height;

                    //         let validators = rpc_client
                    //             .validators(Height::try_from(height)?, tendermint_rpc::Paging::All)
                    //             .await
                    //             .map_err(|e| {
                    //                 println!("Error fetching validators: {:?}", e);
                    //                 e
                    //             })?
                    //             .validators
                    //             .into_iter()
                    //             .map(|validator| ValidatorInfo::from_info(validator, height))
                    //             .collect::<Vec<_>>();

                    //         db.store_validators(&validators[..]).await?;
                    //         Result::<()>::Ok(())
                    //     });
                    // }

                    db.store_blocks(&blocks).await?;
                    Result::<()>::Ok(())
                })
            })
            .collect::<FuturesUnordered<_>>();

        // Wait for all futures to complete and handle any errors
        try_join_all(header_futures).await?;

        Ok(())
    }
}
