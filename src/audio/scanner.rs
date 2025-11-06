use crate::audio::audio_error::AudioError;
use crate::audio::audio_type::AudioType;
use crate::audio::AudioDetails;
use lofty::prelude::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::Accessor;
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;
use std::path::PathBuf;
use std::thread;
use walkdir::WalkDir;

pub async fn get_all_audio_path() -> Result<Vec<PathBuf>, AudioError> {
    let root_dir = "C:\\Users\\mechanic\\Downloads";

    let audio_files: Vec<PathBuf> = WalkDir::new(root_dir)
        .into_iter()
        .filter_entry(|e| !is_hidden_dir(e))
        .filter_map(Result::ok)
        .filter(|e| is_music_file(e))
        .map(|e| e.path().to_path_buf())
        .collect();
    // let audio_details_vec = extract_metadata(&audio_files).await?;
    Ok(audio_files)
}

pub async fn extract_metadata(audio_files: &Vec<PathBuf>) -> Result<Vec<AudioDetails>, AudioError> {
    let results: Vec<AudioDetails> = audio_files
        .into_par_iter()
        .filter_map(|audio_file| {
            log::info!(
                "extracting audio file {:?} , {:?} thread id",
                audio_file,
                thread::current().id()
            );
            let probe = Probe::open(&audio_file);
            if probe.is_err() {
                eprint!("Error: could not read the file");
                return None;
            }

            let tagged_file = match probe.unwrap().read() {
                Ok(file) => file,
                Err(_) => {
                    eprintln!("ERROR: Couldn't read file: {:?}", audio_file);
                    return None;
                }
            };

            let tag = match tagged_file.primary_tag() {
                Some(primary_tag) => primary_tag,
                None => match tagged_file.first_tag() {
                    Some(tag) => tag,
                    None => {
                        eprintln!("Warning: No tags found for file: {:?}", audio_file);
                        return None;
                    }
                },
            };

            let properties = tagged_file.properties();
            let path = audio_file.to_string_lossy().to_string();
            Some(AudioDetails::new(
                audio_file.clone(),
                AudioType::from_path(&path),
                tag.title().as_deref().unwrap_or("Unknown").to_string(),
                tag.artist().as_deref().unwrap_or("Unknown").to_string(),
                tag.album().as_deref().unwrap_or("Unknown").to_string(),
                tag.genre().as_deref().unwrap_or("Unknown").to_string(),
                tag.comment().as_deref().unwrap_or("Unknown").to_string(),
                properties.duration().as_secs_f64(),
                properties.sample_rate(),
                properties.bit_depth().map(|v| v as u8),
                properties.channels(),
                tag.year().unwrap_or(0) as u32,
            ))
        })
        .collect();
    if results.is_empty() {
        Err(AudioError {
            error: "Cannot process files".to_string(),
        })
    } else {
        Ok(results)
    }
}

#[inline]
fn total_number_of_cpu() -> usize {
    num_cpus::get_physical()
}

fn is_hidden_dir(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_dir() && entry.file_name().to_string_lossy().starts_with('.')
}

fn is_music_file(entry: &walkdir::DirEntry) -> bool {
    if !entry.file_type().is_file() {
        return false;
    }
    let path = entry.path().to_str().unwrap_or("unknown").to_string();
    match AudioType::from_path(&path) {
        AudioType::Unknown => false,
        _ => true,
    }
}
