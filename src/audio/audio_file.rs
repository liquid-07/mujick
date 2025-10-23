use crate::audio::audio_type::AudioType;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AudioDetails {
    pub path: PathBuf,
    pub audio_type: AudioType,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub comment: String,

    pub duration: f64,
    pub sample_rate: Option<u32>,
    pub bit_depth: Option<u8>,
    pub channels: Option<u8>,
    pub year: u32,
}

impl AudioDetails {
    pub fn new(
        path: PathBuf,
        audio_type: AudioType,
        title: String,
        artist: String,
        album: String,
        genre: String,
        comment: String,
        duration: f64,
        sample_rate: Option<u32>,
        bit_depth: Option<u8>,
        channels: Option<u8>,
        year: u32,
    ) -> Self {
        Self {
            path,
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
        }
    }
}

impl Default for AudioDetails {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            audio_type: AudioType::Unknown,
            title: "Unknown".into(),
            artist: "Unknown".into(),
            album: "Unknown".into(),
            genre: "Unknown".into(),
            comment: "Unknown".into(),
            duration: -1.0,
            sample_rate: None,
            bit_depth: None,
            channels: None,
            year: 0,
        }
    }
}
