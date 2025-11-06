use crate::audio::audio_error::AudioError;
use crate::audio::scanner::extract_metadata;
use crate::audio::{AudioDetails, get_all_audio_path};
use crate::database::repository;
use crate::database::repository::insert_audio_details;
use crate::gui::CRITERIA;
use std::collections::HashSet;
use std::hash::Hash;
use std::path::PathBuf;

/*https://github.com/k8sgpt-ai/k8sgpt
*/

pub(crate) fn init() {}

pub fn search_audios_ref<'a>(
    audio_name: &String,
    audio_details: &'a [AudioDetails],
) -> Vec<&'a AudioDetails> {
    audio_details
        .iter()
        .filter(|audio| {
            audio
                .title
                .to_lowercase()
                .contains(&audio_name.to_lowercase())
        })
        .collect()
}

///this method will take Criteria, and then it will return all sub criteria present
///  like for artist: Tony Kakkar, Dinchak pooja, Neha Kakkar, Gyaan
pub fn filter_by_criteria(category: &CRITERIA, audio_details: &Vec<AudioDetails>) -> Vec<String> {
    if audio_details.is_empty() {
        return vec![];
    }
    match category {
        CRITERIA::ALL | CRITERIA::UNKNOWN | CRITERIA::PLAYLIST(_) => {
            // never get called
            //todo implement later for playlist
            //add db support
            vec![]
        }
        _ => audio_details
            .into_iter()
            .map(|audio| match category {
                CRITERIA::GENRE(_) => audio.genre.clone(),
                CRITERIA::ARTIST(_) => audio.artist.clone(),
                CRITERIA::YEAR(_) => audio.year.to_string(),
                _ => String::new(),
            })
            .collect(),
    }
}

pub fn filter_by_criteria_value(
    category: CRITERIA,
    audio_details: &[AudioDetails],
) -> Vec<&AudioDetails> {
    if audio_details.is_empty() {
        return Vec::new();
    }
    match category {
        CRITERIA::ALL | CRITERIA::UNKNOWN | CRITERIA::PLAYLIST(_) => audio_details.iter().collect(),

        CRITERIA::GENRE(Some(ref genre)) => audio_details
            .iter()
            .filter(|audio| audio.genre.eq_ignore_ascii_case(genre))
            .collect(),

        CRITERIA::ARTIST(Some(ref artis_opt)) => audio_details
            .iter()
            .filter(|audio| audio.artist.eq_ignore_ascii_case(artis_opt))
            .collect(),
        CRITERIA::YEAR(Some(year_opt)) => audio_details
            .iter()
            .filter(|audio| audio.year == year_opt as u32)
            .collect(),
        _ => audio_details.iter().collect(),
    }
}

pub async fn refresh(audio_path_list: Vec<&PathBuf>) -> Result<Vec<AudioDetails>, AudioError> {
    let latest_audio = get_all_audio_path().await?;
    let unique_paths: HashSet<&PathBuf> = audio_path_list.into_iter().collect();

    let new_audio: Vec<PathBuf> = latest_audio
        .into_iter()
        .filter(|path| !unique_paths.contains(path))
        .collect();

    //new latest details
    let details = extract_metadata(&new_audio).await?;
    let _ = insert_audio_details(&details);
    Ok(details)
}
