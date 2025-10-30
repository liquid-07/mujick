use std::fmt::Display;

#[derive(Debug)]
pub struct AudioError {
    pub error: String,
}

impl Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl std::error::Error for AudioError {}
