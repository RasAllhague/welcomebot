use std::path::Path;

use ab_glyph::{FontRef, PxScale};
use image::{imageops, DynamicImage, Rgb, Rgba};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

fn overlay_images(base: &mut DynamicImage, top: &DynamicImage, x: i64, y: i64) {
    imageops::overlay(base, top, x, y);
}

fn main() -> Result<(), image::ImageError> {
    // Specify the path to your image
    let base_img_path = Path::new("assets/testsprite1.png");
    let top_image_path = Path::new("assets/testsprite2.png");
    let font = FontRef::try_from_slice(include_bytes!("../assets/FiraSans-Bold.ttf")).unwrap();

    // Use the open function to load the image
    let mut base: DynamicImage = image::open(&base_img_path)?;
    let top: DynamicImage = image::open(&top_image_path)?;

    overlay_images(&mut base, &top, 0, 0);
    let white = Rgba([255u8, 255u8, 255u8, 255u8]);

    let height = 12.4;
    let scale = PxScale {
        x: height * 2.0,
        y: height,
    };

    draw_text_mut(&mut base, white, 64, 64, scale, &font, "Test");
    let (w, h) = text_size(scale, &font, "Test");
    println!("Text size: {}x{}", w, h);

    base.save("output.png")?;

    Ok(())
}
