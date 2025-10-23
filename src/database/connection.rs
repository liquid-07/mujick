use sqlx::{
    Error, Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::{str::FromStr, time::Duration};
use tokio::sync::OnceCell;

static DB_CONN: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

pub async fn db_conn() -> Result<Pool<Sqlite>, Error> {
    let db_url = "sqlite://main.db";
    SqlitePoolOptions::new()
        .max_connections(20)
        .idle_timeout(Duration::from_secs(60))
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(
            SqliteConnectOptions::from_str(db_url)?
                .create_if_missing(true)
                .journal_mode(sqlx::sqlite::SqliteJournalMode::Delete),
        )
        .await
}
