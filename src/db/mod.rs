use std::sync::Arc;

use anyhow::Result;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

pub type Pool = Arc<SqlitePool>;

pub async fn get_db_pool() -> Result<SqlitePool, sqlx::Error> {
    let db_file = dotenvy::var("DATABASE_FILE").unwrap();
    let options = SqliteConnectOptions::new()
        .filename(db_file)
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Delete); // Explicitly set to DELETE mode

    SqlitePool::connect_with(options).await
}
