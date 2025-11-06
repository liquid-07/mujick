use crate::audio::{self, AudioDetails, AudioType};
use crate::database::connection::db_conn;
use sqlx::{Error, Row, query};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub async fn create_tables() -> Result<(), Error> {
    log::info!("Creating tables audio files...");
    let conn = db_conn().await?;
    query(
        "CREATE TABLE audio_files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                audio_type TEXT NOT NULL,
                title TEXT NOT NULL,
                artist TEXT,
                album TEXT,
                genre TEXT,
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

pub async fn insert_audio_details(meta_data_vec: &Vec<AudioDetails>) -> Result<(), Error> {
    // let conn = db_conn().await?;

    // todo add genre or comment
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

async fn get_path_of_audios() -> Result<HashSet<String>, Error> {
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

pub async fn get_all_audios() -> Result<Vec<AudioDetails>, Error> {
    let pool = db_conn().await?;
    let rows = query(
        r#"SELECT id, path, audio_type, title, artist, album, genre,
                    comment, sample_rate, bit_depth,channels, year,duration,year, is_deleted
                FROM audio_files
                WHERE is_deleted = 0"#,
    )
    .fetch_all(&pool)
    .await?;

    let mut audio_destails = Vec::new();
    for row in rows {
        let path: String = row.try_get("path")?;
        let audio_type = AudioType::from_path(&path);
        let title: String = row.try_get("title")?;
        let artist: String = row.try_get("artist")?;
        let album: String = row.try_get("album")?;
        let genre: String = row.try_get("genre")?;
        let comment: String = row.try_get("comment")?;
        let duration: f64 = row.try_get("duration")?;
        let sample_rate: Option<u32> = row.try_get("sample_rate")?;
        let bit_depth: Option<u8> = row.try_get("bit_depth")?;
        let channels: Option<u8> = row.try_get("channels")?;
        let year: u32 = row.try_get("year")?;
        let audio = AudioDetails::new(
            PathBuf::from(path),
            audio_type,
            title,
            artist,
            album,
            genre,
            comment,
            duration,
            sample_rate,
            bit_depth,
            channels,
            year,
        );
        audio_destails.push(audio);
    }
    Ok(audio_destails)
}
