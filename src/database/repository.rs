use crate::audio;
use crate::database::connection::db_conn;
use sqlx::{Error, Row, query};
use std::collections::HashSet;

pub async fn create_tables() -> Result<(), Error> {
    log::info!("Creating tables...");
    let conn = db_conn().await?;
    query(
        "CREATE TABLE audio_files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                audio_type TEXT NOT NULL,
                title TEXT NOT NULL,
                artist TEXT,
                album TEXT,
                duration REAL,
                year INTEGER,
                is_deleted INTEGER NOT NULL DEFAULT 0
        );",
    )
    .execute(&conn)
    .await?;

    log::info!("Tables created successfully.");
    Ok(())
}

pub async fn insert_audio_details(meta_data_vec: &Vec<audio::AudioDetails>) -> Result<(), Error> {
    // let conn = db_conn().await?;

    // for audio in meta_data_vec {
    let conn = db_conn().await?;

    for metadata in meta_data_vec {
        sqlx::query(
            r#"
            INSERT INTO audio_files (
                path, audio_type, title, artist, album, duration, year, is_deleted
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, 0
            ) ON CONFLICT(path) DO NOTHING;
            "#,
        )
        .bind(&metadata.path.to_str().unwrap())
        .bind(&metadata.audio_type.as_str())
        .bind(&metadata.title)
        .bind(&metadata.artist)
        .bind(&metadata.album)
        .bind(metadata.duration)
        .bind(metadata.year)
        .execute(&conn)
        .await?;
    }
    // }
    Ok(())
}

async fn get_existing_paths() -> Result<HashSet<String>, Error> {
    let mut paths = HashSet::new();
    let rows = query("SELECT path FROM audio_files WHERE path IS NOT NULL;")
        .fetch_all(&db_conn().await?)
        .await?;

    for row in rows {
        let path: String = row.try_get("path")?;
        paths.insert(path);
    }

    Ok(paths)
}

async fn get_new_files() {}
