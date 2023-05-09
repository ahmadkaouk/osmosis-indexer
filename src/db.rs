use crate::{
    data::{BlockInfo, PeerInfo, ValidatorInfo},
    utils::DBConfig,
};
use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, query_builder::QueryBuilder, PgPool, Postgres};

/// A wrapper around a [`PgPool`][sqlx::postgres::PgPool] that provides some
/// convenience methods.
#[derive(Clone)]
pub struct DB {
    pub pool: PgPool,
}

impl DB {
    /// Create a new database connection pool.
    pub async fn new(config: DBConfig) -> Result<DB> {
        // Ensure the database exists
        // Postgres::create_database(url).await?;

        let pool = PgPoolOptions::new()
            .max_connections(100)
            .connect(&config.url)
            .await
            .context("Failed to connect to database")?;

        let db = DB { pool };
        db.run_migrations().await?;

        Ok(db)
    }

    /// Store a set of blocks in the database.
    pub async fn store_blocks(&self, blocks: &[BlockInfo]) -> Result<()> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO blocks (block_height, block_id, time, num_txs, proposer_address)",
        );

        query_builder.push_values(blocks, |mut b, block| {
            b.push_bind(block.height as i64)
                .push_bind(block.block_id.as_str())
                .push_bind(block.time.as_str())
                .push_bind(block.num_txs as i64)
                .push_bind(block.proposer_address.as_str());
        });

        query_builder.build().execute(&self.pool).await?;

        Ok(())
    }

    /// Store a set of validators in the database.
    pub async fn store_validators(&self, validators: &[ValidatorInfo]) -> Result<()> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO validators (block_height, address, name, voting_power, proposer_priority)",
        );

        query_builder.push_values(validators, |mut b, validator| {
            let name = validator.name.to_owned().unwrap_or("".to_string());
            b.push_bind(validator.block_height)
                .push_bind(validator.address.as_str())
                .push_bind(name)
                .push_bind(validator.power as i64)
                .push_bind(validator.proposer_priority);
        });

        query_builder.build().execute(&self.pool).await?;

        Ok(())
    }

    /// Store network parameters in the database.
    pub async fn store_network_infos(&self, peers: &[PeerInfo]) -> Result<()> {
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO peers (block_height, node_id, remote_ip, peer_score)");

        query_builder.push_values(peers, |mut p, peer| {
            p.push_bind(peer.block_height)
                .push_bind(peer.node_id.as_str())
                .push_bind(peer.remote_ip.as_str())
                .push_bind(peer.peer_score);
        });

        query_builder.build().execute(&self.pool).await?;

        Ok(())
    }

    /// Get the latest block height from the database.
    pub async fn get_latest_block_height(&self) -> Result<i64> {
        let height = sqlx::query!("SELECT MAX(block_height) FROM blocks")
            .fetch_one(&self.pool)
            .await
            .context("Failed to get latest block height")?
            .max
            .unwrap_or(0);

        Ok(height)
    }

    /// Run the database migrations.
    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .context("Failed to run migrations")?;

        Ok(())
    }
}
