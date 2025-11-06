use crate::audio::audio_error::AudioError;
use crate::audio::audio_service::refresh;
use crate::audio::scanner::extract_metadata;
use crate::audio::{AudioDetails, get_all_audio_path};
use crate::database::repository;
use std::path::{Path, PathBuf};

pub(crate) async fn load_audio_on_startup() {
    let path = Path::new("main.db");
    if !path.exists() {
        repository::create_tables()
            .await
            .expect("TODO: panic message");

        let audio_list = get_all_audio_path().await;

        match audio_list {
            Ok(audio_list) => match extract_metadata(&audio_list).await {
                Ok(audio_details) => {
                    repository::insert_audio_details(&audio_details)
                        .await
                        .expect("TODO: panic message");
                }
                Err(_) => {
                    log::warn!("Failed to extract audio metadata");
                }
            },
            Err(_) => {
                panic!("cannot start music because there is no audio")
            }
        }
    } else {
        let audio_details_list = repository::get_all_audios().await.unwrap();
    }
}

pub(crate) async fn refresh_audio(
    audio_path: Vec<&PathBuf>,
) -> Result<Vec<AudioDetails>, AudioError> {
    refresh(audio_path).await
}
