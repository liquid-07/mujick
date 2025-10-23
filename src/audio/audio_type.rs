use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum AudioType {
    Mp3,
    Flac,
    Wav,
    Ogg,
    Aac,
    Unknown,
}

impl AudioType {
    pub fn from_path(path: &Path) -> Self {
        match path
            .extension()
            .and_then(|e| e.to_str().map(|s| s.to_lowercase()))
        {
            Some(ext) => match ext.as_str() {
                "mp3" => AudioType::Mp3,
                "flac" => AudioType::Flac,
                "wav" => AudioType::Wav,
                "ogg" => AudioType::Ogg,
                "aac" => AudioType::Aac,
                _ => AudioType::Unknown,
            },
            None => AudioType::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AudioType::Mp3 => "mp3",
            AudioType::Flac => "flac",
            AudioType::Wav => "wav",
            AudioType::Ogg => "ogg",
            AudioType::Aac => "aac",
            AudioType::Unknown => "unknown",
        }
    }
}

impl Display for AudioType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
