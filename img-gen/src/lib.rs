pub mod error;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use ab_glyph::{FontVec, PxScale};
use error::Error;
use image::{imageops, DynamicImage, Rgba};
use imageproc::drawing::draw_text_mut;

pub struct ImageGenerator {
    fonts: HashMap<String, FontVec>,
}

impl ImageGenerator {
    pub fn new() -> Self {
        ImageGenerator {
            fonts: HashMap::new(),
        }
    }

    pub fn add_font(&mut self, name: &str, font: FontVec) {
        self.fonts.insert(name.to_string(), font);
    }

    pub fn generate(&self, builder: ImageBuilder) -> Result<DynamicImage, Error> {
        let mut base_image = image::open(builder.base_image)?;

        for element in &builder.elements {
            match element {
                ImageElement::Picture { x, y, path } => {
                    self.overlay_image(&mut base_image, &path, *x, *y)?
                }
                ImageElement::Text {
                    x,
                    y,
                    scale,
                    text,
                    font_name,
                    color,
                } => self.overlay_text(&mut base_image, &text, *x, *y, scale, &font_name, color)?,
            }
        }

        Ok(base_image)
    }

    fn overlay_image<T: AsRef<Path>>(
        &self,
        base_image: &mut DynamicImage,
        top_image_path: T,
        x: i64,
        y: i64,
    ) -> Result<(), Error> {
        let top_image = image::open(top_image_path)?;
        imageops::overlay(base_image, &top_image, x, y);

        Ok(())
    }

    fn overlay_text(
        &self,
        base_image: &mut DynamicImage,
        text: &str,
        x: i32,
        y: i32,
        scale: &PxScale,
        font_name: &str,
        color: &Rgba<u8>,
    ) -> Result<(), Error> {
        if let Some(font) = self.fonts.get(font_name) {
            draw_text_mut(base_image, *color, x, y, *scale, font, text);

            return Ok(());
        }

        Err(Error::FontNotFound(format!(
            "The font '{}' was not found. Please make sure it is loaded.",
            font_name
        )))
    }
}

pub enum ImageElement {
    Picture {
        x: i64,
        y: i64,
        path: PathBuf,
    },
    Text {
        x: i32,
        y: i32,
        scale: PxScale,
        text: String,
        font_name: String,
        color: Rgba<u8>,
    },
}

pub struct ImageBuilder {
    pub base_image: PathBuf,
    pub elements: Vec<ImageElement>,
}

impl ImageBuilder {
    pub fn new<T: AsRef<Path>>(base_image: T) -> Self {
        Self {
            base_image: base_image.as_ref().to_path_buf(),
            elements: Vec::new(),
        }
    }

    pub fn with_base_image<T: AsRef<Path>>(mut self, path: T) -> Self {
        self.base_image = path.as_ref().to_path_buf();

        self
    }

    pub fn add_image<T: AsRef<Path>>(mut self, path: T, x: i64, y: i64) -> Self {
        self.elements.push(ImageElement::Picture {
            x,
            y,
            path: path.as_ref().to_path_buf(),
        });

        self
    }

    pub fn add_text(
        mut self,
        text: &str,
        x: i32,
        y: i32,
        scale: PxScale,
        font_name: &str,
        color: Rgba<u8>,
    ) -> Self {
        self.elements.push(ImageElement::Text {
            x,
            y,
            scale,
            text: text.to_string(),
            font_name: font_name.to_string(),
            color: color,
        });

        self
    }
}
