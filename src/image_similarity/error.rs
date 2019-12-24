use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::string::String;

#[derive(Debug)]
pub struct ImageSimilarityError {
    pub reason: String,
}

impl Display for ImageSimilarityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Error for ImageSimilarityError {
    fn description(&self) -> &str {
        &*self.reason
    }
}

impl std::convert::From<opencv::Error> for ImageSimilarityError {
    fn from(error: opencv::Error) -> Self {
        ImageSimilarityError {
            reason: format!("OpenCV error: {}", error)
        }
    }
}
