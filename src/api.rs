use crate::error::IndexerError;
use anyhow::Context;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use sqlx::PgPool;

pub fn app(db: PgPool) -> Router {
    Router::new().merge(router()).with_state(db)
}

pub async fn serve(db: PgPool) -> anyhow::Result<()> {
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app(db).into_make_service())
        .await
        .context("failed to start server")
}

pub fn router() -> Router<PgPool, axum::body::Body> {
    Router::new()
        .route("/blocks/:validator", get(validator_blocks))
        .route("/transactions/:last_blocks", get(transactions))
}

async fn validator_blocks(
    State(db): State<PgPool>,
    Path(proposer_address): Path<String>,
) -> Result<Json<Vec<i64>>, IndexerError> {
    let blocks: Vec<i64> = sqlx::query!(
        "SELECT block_height FROM blocks WHERE proposer_address = $1",
        proposer_address
    )
    .fetch_all(&db)
    .await?
    .into_iter()
    .map(|row| row.block_height)
    .collect();

    Ok(Json(blocks))
}

/// Get the number of transactions in the last `last_blocks` blocks.
async fn transactions(
    State(db): State<PgPool>,
    Path(last_blocks): Path<i64>,
) -> Result<Json<i64>, IndexerError> {
    let txs_nums: Vec<i64> = sqlx::query!(
        "SELECT num_txs FROM blocks ORDER BY block_height DESC LIMIT $1",
        last_blocks
    )
    .fetch_all(&db)
    .await?
    .into_iter()
    .map(|row| row.num_txs.unwrap_or(0))
    .collect();

    Ok(Json(txs_nums.iter().sum()))
}
