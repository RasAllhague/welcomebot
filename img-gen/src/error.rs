
#[derive(Debug)]
pub enum Error {
    ImageError(image::ImageError),
    FontNotFound(String),
}

impl From<image::ImageError> for Error {
    fn from(value: image::ImageError) -> Self {
        Self::ImageError(value)
    }
}