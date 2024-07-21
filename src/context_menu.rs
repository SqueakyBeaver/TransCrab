use crate::{stt, Context, Error};
use poise::serenity_prelude as serenity;
use std::fs;
use std::fs::File;
use std::io::{copy, Cursor};

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

    let reply = ctx.reply("Downloading file...").await?;
    let target = &msg.attachments[0].url;
    let response = reqwest::get(target).await?;
    let name = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");

    let binding = fs::canonicalize("temp-files/").unwrap();
    let fname = binding.to_str().unwrap().to_owned() + &format!("/{}", name);

    let mut dest = File::create(&fname).unwrap();
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut dest)?;

    reply
        .edit(
            ctx,
            poise::CreateReply {
                content: Some(String::from("Transcribing file...")),
                ..Default::default()
            },
        )
        .await?;
    
    reply.edit(
            ctx,
            poise::CreateReply {
                content: Some(format!(
                    "Transcript for {}:\n>>> {}",
                    msg.link(),
                    stt::transcribe(&fname).unwrap()
                )),
                ..Default::default()
            },
        )
        .await?;

    Ok(())
}
