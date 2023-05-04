use anyhow::{Context, Result};
use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions, PgPool, Postgres};

/// A wrapper around a [`PgPool`][sqlx::postgres::PgPool] that provides some
/// convenience methods.
pub struct DB {
    pool: PgPool,
}

impl DB {
    /// Create a new database connection pool.
    pub async fn new(url: &str) -> Result<DB> {
        // Ensure the database exists
        // Postgres::create_database(url).await?;

        let pool = PgPoolOptions::new()
            .max_connections(100)
            .connect(url)
            .await
            .context("Failed to connect to database")?;

        let db = DB { pool };
        db.run_migrations().await?;

        Ok(db)
    }

    /// Run the database migrations.
    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .context("Failed to run migrations")?;

        Ok(())
    }

    /// Get a reference to the database connection pool.
    pub async fn pool(&self) -> &PgPool {
        &self.pool
    }
}
