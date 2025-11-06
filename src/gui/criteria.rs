use std::fmt::{Display, Formatter};

pub enum CRITERIA {
    ALL,
    PLAYLIST(Option<String>),
    GENRE(Option<String>),
    ARTIST(Option<String>),
    YEAR(Option<i32>),
    UNKNOWN,
}
impl CRITERIA {
    pub fn label(&self) -> &'static str {
        match self {
            CRITERIA::ALL => "All",
            CRITERIA::UNKNOWN => "Unknown",
            CRITERIA::PLAYLIST(_) => "Playlist",
            CRITERIA::GENRE(_) => "Genre",
            CRITERIA::ARTIST(_) => "Artist",
            CRITERIA::YEAR(_) => "Year",
        }
    }

    pub fn get_all_criteria() -> Vec<String> {
        vec![
            CRITERIA::ALL.label().to_string(),
            CRITERIA::PLAYLIST(None).label().to_string(),
            CRITERIA::GENRE(None).label().to_string(),
            CRITERIA::ARTIST(None).label().to_string(),
            CRITERIA::YEAR(None).label().to_string(),
            CRITERIA::UNKNOWN.label().to_string(),
        ]
    }

    pub fn value(&self) -> Option<String> {
        match self {
            CRITERIA::PLAYLIST(opt) => opt.clone(),
            CRITERIA::GENRE(opt) => opt.clone(),
            CRITERIA::ARTIST(opt) => opt.clone(),
            CRITERIA::YEAR(opt) => opt.map(|y| y.to_string()),
            _ => None,
        }
    }
}
impl Display for CRITERIA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.value() {
            Some(val) => write!(f, "{}: {}", self.label(), val),
            None => write!(f, "{}", self.label()),
        }
    }
}
