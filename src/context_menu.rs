use crate::{stt, Context, Error};
use poise::serenity_prelude as serenity;
use std::path::Path;

/// Query information about a Discord profile
#[poise::command(context_menu_command = "Transcribe audio in message", slash_command)]
pub async fn transcribe(
    ctx: Context<'_>,
    #[description = "Voice Message to transcribe"] msg: serenity::Message,
) -> Result<(), Error> {
    ctx.defer().await?;

    if msg.attachments.is_empty() {
        ctx.reply("No message attachments found. :p").await?;
        return Ok(());
    }

    // Is ./voice-message atm bc it is in relation to the binary
    // Not the source file
    // I am smort
    let path = Path::new("./voice-message.ogg");

    ctx.reply(stt::transcribe(path)).await?;

    Ok(())
}
