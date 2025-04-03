use std::fmt;

use ab_glyph::InvalidFont;

/// Represents errors that can occur in the image generation module.
#[derive(Debug)]
pub enum Error {
    /// Error that occurs during image processing.
    ///
    /// This error is typically caused by issues with the `image` crate.
    ImageError(image::ImageError),

    /// Error indicating that the specified font could not be found.
    ///
    /// # Fields
    /// - `String`: The name or path of the font that was not found.
    FontNotFound(String),

    /// Error that occurs when a font is invalid or cannot be loaded.
    ///
    /// This error is typically caused by issues with the `ab_glyph` crate.
    InvalidFont(InvalidFont),
}

impl fmt::Display for Error {
    /// Formats the error for display purposes.
    ///
    /// # Arguments
    /// * `f` - The formatter used to format the error.
    ///
    /// # Returns
    /// Returns a `fmt::Result` indicating success or failure.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImageError(why) => write!(f, "{why}"),
            Self::FontNotFound(why) => write!(f, "{why}"),
            Self::InvalidFont(why) => write!(f, "{why}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<image::ImageError> for Error {
    /// Converts an `image::ImageError` into an `Error`.
    ///
    /// # Arguments
    /// * `value` - The `ImageError` to convert.
    ///
    /// # Returns
    /// Returns an `Error::ImageError` variant.
    fn from(value: image::ImageError) -> Self {
        Self::ImageError(value)
    }
}

impl From<InvalidFont> for Error {
    /// Converts an `InvalidFont` into an `Error`.
    ///
    /// # Arguments
    /// * `value` - The `InvalidFont` to convert.
    ///
    /// # Returns
    /// Returns an `Error::InvalidFont` variant.
    fn from(value: InvalidFont) -> Self {
        Self::InvalidFont(value)
    }
}
