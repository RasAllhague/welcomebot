use std::path::Path;

use ab_glyph::{FontVec, PxScale};
use image::{imageops::FilterType, Rgba};
use img_gen::{error::Error, ImageBuilder, ImageGenerator};
use tempfile::TempDir;
use tokio::{fs::File, io::AsyncWriteExt};

pub static FIRA_SANS_BOLD: &str = "fsb";
pub static FIRA_MONO_MEDIUM: &str = "fmm";

pub fn setup_image_generator() -> Result<ImageGenerator, Error> {
    let fira_sans_bold =
        FontVec::try_from_vec(include_bytes!("../../assets/FiraSans-Bold.ttf").to_vec())?;
    let fira_mono_medium =
        FontVec::try_from_vec(include_bytes!("../../assets/FiraMono-Medium.ttf").to_vec())?;

    let mut img_generator = ImageGenerator::new();
    img_generator.add_font(FIRA_SANS_BOLD, fira_sans_bold);
    img_generator.add_font(FIRA_MONO_MEDIUM, fira_mono_medium);

    Ok(img_generator)
}

pub fn create_image_builder(
    front_image_path: impl AsRef<Path>,
    back_image_path: impl AsRef<Path>,
    file_path: impl AsRef<Path>,
    headline_message: &str,
    subline_message: &str,
    x: i64,
    y: i64,
    display_name: &str,
    members: usize,
    big_scale: PxScale,
    small_scale: PxScale,
) -> ImageBuilder {
    let image_builder = ImageBuilder::new(back_image_path)
        .add_image(&file_path, x, y)
        .add_image(front_image_path, 0, 0)
        .add_text(
            &headline_message.replace("{name}", display_name),
            450,
            352,
            big_scale,
            FIRA_SANS_BOLD,
            Rgba([34, 34, 34, 255]),
            true,
        )
        .add_text(
            &subline_message.replace("{members}", &members.to_string()),
            450,
            400,
            small_scale,
            FIRA_MONO_MEDIUM,
            Rgba([59, 59, 59, 255]),
            true,
        );
    image_builder
}

pub async fn download_avatar(
    img_url: &str,
    temp_dir: &TempDir,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Sync + Send>> {
    let image_bytes = reqwest::get(img_url).await?.bytes().await?;

    let image_id = uuid::Uuid::new_v4();

    let file_path = if img_url.to_lowercase().contains(".webp") {
        temp_dir.path().join(format!("{}.webp", image_id))
    } else {
        temp_dir.path().join(format!("{}.png", image_id))
    };

    let mut tmp_file = File::create(&file_path).await?;
    tmp_file.write_all(&image_bytes).await?;
    tmp_file.flush().await?;

    drop(tmp_file);

    let image = image::open(&file_path)?;
    let image = image.resize(256, 256, FilterType::Nearest);
    image.save(&file_path)?;

    Ok(file_path)
}
