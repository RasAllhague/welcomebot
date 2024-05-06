use ab_glyph::{FontVec, PxScale};
use image::Rgba;
use img_gen::{error::Error, ImageBuilder, ImageGenerator};

pub static FIRA_SANS_BOLD: &str = "fsb";
pub static FIRA_MONO_MEDIUM: &str = "fmm";

fn main() -> Result<(), Error> {
    let img_generator = setup_image_generator()?;

    let scale = PxScale { x: 48.0, y: 20.0 };

    let image_builder = ImageBuilder::new("assets/testsprite1.png")
        .add_image("assets/testsprite2.png", 0, 0)
        .add_text(
            "text",
            32,
            32,
            scale,
            FIRA_SANS_BOLD,
            Rgba([111u8, 123u8, 143u8, 255u8]),
        );

    let output_image = img_generator.generate(image_builder)?;
    output_image.save("output.png")?;

    Ok(())
}

fn setup_image_generator() -> Result<ImageGenerator, Error> {
    let fira_sans_bold =
        FontVec::try_from_vec(include_bytes!("../assets/FiraSans-Bold.ttf").to_vec())?;
    let fira_mono_medium =
        FontVec::try_from_vec(include_bytes!("../assets/FiraMono-Medium.ttf").to_vec())?;

    let mut img_generator = ImageGenerator::new();
    img_generator.add_font(FIRA_SANS_BOLD, fira_sans_bold);
    img_generator.add_font(FIRA_MONO_MEDIUM, fira_mono_medium);
    
    Ok(img_generator)
}
