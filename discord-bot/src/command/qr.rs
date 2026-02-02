use image::Luma;
use poise::{CreateReply, serenity_prelude::CreateAttachment};
use qrcode::QrCode;

use crate::{Context, PoiseError};

/// Creates a simple qr code from the given input text.
#[poise::command(slash_command)]
pub async fn qr_code(
    ctx: Context<'_>,
    #[description = "The text that should be rendered into an qr code."] input_text: String,
) -> Result<(), PoiseError> {
    let code = QrCode::new(input_text.as_bytes())?;
    let image = code.render::<Luma<u8>>().build();

    let outfile_id = uuid::Uuid::new_v4();
    let outfile_path = ctx.data().temp_dir.path().join(format!("{outfile_id}.png"));
    image.save(&outfile_path)?;

    let attachment = CreateAttachment::path(outfile_path).await?;

    ctx.send(
        CreateReply::default()
            .content("Here is your qr code:")
            .attachment(attachment),
    )
    .await?;

    Ok(())
}
