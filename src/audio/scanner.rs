//find all music in computer

use crate::audio::AudioDetails;
use crate::audio::audio_error::AudioError;
use crate::audio::audio_type::AudioType;
use lofty::prelude::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::Accessor;
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

pub fn scan_audio() -> Result<Vec<AudioDetails>, AudioError> {
    let root_dir = "C:\\Users\\mechanic\\Downloads";
    let start_time = Instant::now();
    let t_cpu = total_number_of_cpu();
    rayon::ThreadPoolBuilder::new()
        .num_threads(t_cpu)
        .build_global()
        .unwrap();

    log::info!(
        "search_in_drive():{:?} with total cpu {:?}",
        start_time,
        t_cpu
    );
    let audio_files: Vec<PathBuf> = WalkDir::new(root_dir)
        .into_iter()
        .filter_entry(|e| !is_hidden_dir(e))
        .filter_map(Result::ok)
        .filter(|e| is_music_file(e))
        .map(|e| e.path().to_path_buf())
        .collect();
    let elapsed = start_time.elapsed();
    let audio_details_vec = extract_metadata(&audio_files)?;
    println!("time taken to scan: {:.2?}", elapsed);
    Ok(audio_details_vec)
}

fn extract_metadata(audio_files: &Vec<PathBuf>) -> Result<Vec<AudioDetails>, AudioError> {
    let mut results = Vec::new();
    for audio_file in audio_files {
        let probe = Probe::open(&audio_file);
        if probe.is_err() {
            eprint!("Error: could not read the file");
            continue;
        }

        let tagged_file = match probe.unwrap().read() {
            Ok(file) => file,
            Err(_) => {
                eprintln!("ERROR: Couldn't read file: {:?}", audio_file);
                continue;
            }
        };

        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => match tagged_file.first_tag() {
                Some(tag) => tag,
                None => {
                    eprintln!("Warning: No tags found for file: {:?}", audio_file);
                    continue;
                }
            },
        };

        let properties = tagged_file.properties();

        let audio_details = AudioDetails::new(
            audio_file.to_path_buf(),
            AudioType::from_path(audio_file.as_path()),
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
        );
        results.push(audio_details);
    }
    if !results.is_empty() {
        return Ok(results);
    }
    Err(AudioError {
        error: "cannot process files".to_string(),
    })
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
    match AudioType::from_path(entry.path()) {
        AudioType::Unknown => false,
        _ => true,
    }
}
