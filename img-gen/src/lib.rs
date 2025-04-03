pub mod error;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use ab_glyph::{FontVec, PxScale};
use error::Error;
use image::{imageops, DynamicImage, Rgba};
use imageproc::drawing::{draw_text_mut, text_size};

/// Represents a 2D vector with `x` and `y` coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2<T> {
    /// The x-coordinate of the vector.
    pub x: T,
    /// The y-coordinate of the vector.
    pub y: T,
}

impl Vec2<i32> {
    /// Creates a new `Vec2<i32>` instance.
    ///
    /// # Arguments
    /// * `x` - The x-coordinate.
    /// * `y` - The y-coordinate.
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Vec2<i64> {
    /// Creates a new `Vec2<i64>` instance.
    ///
    /// # Arguments
    /// * `x` - The x-coordinate.
    /// * `y` - The y-coordinate.
    #[must_use]
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

/// A generator for creating and manipulating images.
pub struct ImageGenerator {
    /// A collection of fonts available for use in the generator.
    fonts: HashMap<String, FontVec>,
}

impl ImageGenerator {
    /// Creates a new `ImageGenerator` instance.
    #[must_use]
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
        }
    }

    /// Adds a font to the generator.
    ///
    /// # Arguments
    /// * `name` - The name of the font.
    /// * `font` - The font data.
    pub fn add_font(&mut self, name: &str, font: FontVec) {
        self.fonts.insert(name.to_string(), font);
    }

    /// Generates an image based on the provided `ImageBuilder`.
    ///
    /// # Arguments
    /// * `builder` - The `ImageBuilder` containing the base image and elements to overlay.
    ///
    /// # Errors
    /// Returns an [`Error`] if the image generation fails.
    #[fastrace::trace]
    pub fn generate(&self, builder: ImageBuilder) -> Result<DynamicImage, Error> {
        let mut base_image = image::open(builder.base_image)?;

        for element in &builder.elements {
            match element {
                ImageElement::Picture { x, y, path } => {
                    Self::overlay_image(&mut base_image, path, *x, *y)?;
                }
                ImageElement::Text {
                    x,
                    y,
                    scale,
                    text,
                    font_name,
                    color,
                    center_pivot,
                } => self.overlay_text(
                    &mut base_image,
                    text,
                    Vec2::<i32>::new(*x, *y),
                    *scale,
                    font_name,
                    *color,
                    *center_pivot,
                )?,
            }
        }

        Ok(base_image)
    }

    /// Overlays an image onto the base image.
    ///
    /// # Arguments
    /// * `base_image` - The base image to overlay onto.
    /// * `top_image_path` - The path to the image to overlay.
    /// * `x` - The x-coordinate of the overlay position.
    /// * `y` - The y-coordinate of the overlay position.
    ///
    /// # Errors
    /// Returns an [`Error`] if the overlay operation fails.
    #[fastrace::trace]
    fn overlay_image<T: AsRef<Path>>(
        base_image: &mut DynamicImage,
        top_image_path: T,
        x: i64,
        y: i64,
    ) -> Result<(), Error> {
        let top_image = image::open(top_image_path)?;
        imageops::overlay(base_image, &top_image, x, y);

        Ok(())
    }

    /// Overlays text onto the base image.
    ///
    /// # Arguments
    /// * `base_image` - The base image to overlay text onto.
    /// * `text` - The text to overlay.
    /// * `position` - The position of the text.
    /// * `scale` - The scale of the text.
    /// * `font_name` - The name of the font to use.
    /// * `color` - The color of the text.
    /// * `center_pivot` - Whether to center the text around the position.
    ///
    /// # Errors
    /// Returns an [`Error`] if the font is not found or the overlay operation fails.
    #[fastrace::trace]
    fn overlay_text(
        &self,
        base_image: &mut DynamicImage,
        text: &str,
        position: Vec2<i32>,
        scale: PxScale,
        font_name: &str,
        color: Rgba<u8>,
        center_pivot: bool,
    ) -> Result<(), Error> {
        if let Some(font) = self.fonts.get(font_name) {
            let (text_x, _) = text_size(scale, font, text);

            if center_pivot {
                draw_text_mut(
                    base_image,
                    color,
                    position.x - text_x as i32 / 2,
                    position.y,
                    scale,
                    font,
                    text,
                );
            } else {
                draw_text_mut(base_image, color, position.x, position.y, scale, font, text);
            }

            return Ok(());
        }

        Err(Error::FontNotFound(format!(
            "The font '{font_name}' was not found. Please make sure it is loaded."
        )))
    }
}

impl Default for ImageGenerator {
    /// Creates a default `ImageGenerator` instance.
    fn default() -> Self {
        Self::new()
    }
}

/// Represents an element to overlay on an image.
pub enum ImageElement {
    /// Represents an image to overlay.
    Picture {
        /// The x-coordinate of the overlay position.
        x: i64,
        /// The y-coordinate of the overlay position.
        y: i64,
        /// The path to the image to overlay.
        path: PathBuf,
    },
    /// Represents text to overlay.
    Text {
        /// The x-coordinate of the text position.
        x: i32,
        /// The y-coordinate of the text position.
        y: i32,
        /// The scale of the text.
        scale: PxScale,
        /// The text content.
        text: String,
        /// The name of the font to use.
        font_name: String,
        /// The color of the text.
        color: Rgba<u8>,
        /// Whether to center the text around the position.
        center_pivot: bool,
    },
}

/// A builder for constructing images with overlays.
pub struct ImageBuilder {
    /// The path to the base image.
    pub base_image: PathBuf,
    /// The elements to overlay on the base image.
    pub elements: Vec<ImageElement>,
}

impl ImageBuilder {
    /// Creates a new `ImageBuilder` with the specified base image.
    ///
    /// # Arguments
    /// * `base_image` - The path to the base image.
    pub fn new<T: AsRef<Path>>(base_image: T) -> Self {
        Self {
            base_image: base_image.as_ref().to_path_buf(),
            elements: Vec::new(),
        }
    }

    /// Sets the base image for the builder.
    ///
    /// # Arguments
    /// * `path` - The path to the base image.
    #[must_use]
    pub fn with_base_image<T: AsRef<Path>>(mut self, path: T) -> Self {
        self.base_image = path.as_ref().to_path_buf();

        self
    }

    /// Adds an image overlay to the builder.
    ///
    /// # Arguments
    /// * `path` - The path to the image to overlay.
    /// * `x` - The x-coordinate of the overlay position.
    /// * `y` - The y-coordinate of the overlay position.
    #[must_use]
    pub fn add_image<T: AsRef<Path>>(mut self, path: T, x: i64, y: i64) -> Self {
        self.elements.push(ImageElement::Picture {
            x,
            y,
            path: path.as_ref().to_path_buf(),
        });

        self
    }

    /// Adds a text overlay to the builder.
    ///
    /// # Arguments
    /// * `text` - The text to overlay.
    /// * `x` - The x-coordinate of the text position.
    /// * `y` - The y-coordinate of the text position.
    /// * `scale` - The scale of the text.
    /// * `font_name` - The name of the font to use.
    /// * `color` - The color of the text.
    /// * `center_pivot` - Whether to center the text around the position.
    #[must_use]
    pub fn add_text(
        mut self,
        text: &str,
        x: i32,
        y: i32,
        scale: PxScale,
        font_name: &str,
        color: Rgba<u8>,
        center_pivot: bool,
    ) -> Self {
        self.elements.push(ImageElement::Text {
            x,
            y,
            scale,
            text: text.to_string(),
            font_name: font_name.to_string(),
            color,
            center_pivot,
        });

        self
    }
}
