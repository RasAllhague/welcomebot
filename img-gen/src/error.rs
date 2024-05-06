use ab_glyph::InvalidFont;


#[derive(Debug)]
pub enum Error {
    ImageError(image::ImageError),
    FontNotFound(String),
    InvalidFont(InvalidFont),
}

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