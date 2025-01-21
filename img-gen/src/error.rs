use std::fmt;

use ab_glyph::InvalidFont;

#[derive(Debug)]
pub enum Error {
    ImageError(image::ImageError),
    FontNotFound(String),
    InvalidFont(InvalidFont),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImageError(why) => write!(f, "{why}"),
            Self::FontNotFound(why) => write!(f, "{why}"),
            Self::InvalidFont(why) => write!(f, "{why}" ),
        }
    }
}

impl std::error::Error for Error {}

impl From<image::ImageError> for Error {
    fn from(value: image::ImageError) -> Self {
        Self::ImageError(value)
    }
}

impl From<InvalidFont> for Error {
    fn from(value: InvalidFont) -> Self {
        Self::InvalidFont(value)
    }
}
