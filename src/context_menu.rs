use crate::{stt, Context, Error};
use poise::serenity_prelude as serenity;
use std::fs;
use std::fs::File;
use std::io::{copy, Cursor};

async fn download_file(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;

    match response.error_for_status() {
        Err(err) => Err(Box::new(err)),
        Ok(res) => {
            let name = res
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| {
                    if name.is_empty() {
                        None
                    } else {
                        Some(String::from("tmp") + name.split_terminator(".").last().unwrap())
                    }
                })
                .unwrap_or(String::from("tmp.bin"));

            let binding = fs::canonicalize("temp-files/").unwrap();
            let fname = binding.to_str().unwrap().to_owned() + &format!("/{}", name);

            let mut dest = File::create(&fname).unwrap();
            let mut content = Cursor::new(res.bytes().await?);
            copy(&mut content, &mut dest)?;

            Ok(fname)
        }
    }
}

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

    let reply = ctx.reply("Please wait, this could take a while...").await?;

    let target = &msg.attachments[0].url;

    reply
        .edit(
            ctx,
            poise::CreateReply {
                content: Some(String::from("Downloading file...")),
                ..Default::default()
            },
        )
        .await?;

    let fname = download_file(target).await?;

    reply
        .edit(
            ctx,
            poise::CreateReply {
                content: Some(String::from("Transcribing file...")),
                ..Default::default()
            },
        )
        .await?;

    let transcription = stt::transcribe(&fname);

    match transcription {
        Err(err) => {
            reply
                .edit(
                    ctx,
                    poise::CreateReply {
                        content: Some(String::from(
                            "Error transcribing Audio. Please tell the devs.",
                        )),
                        ..Default::default()
                    },
                )
                .await?;
            return Err(err);
        }
        Ok(transcript) => {
            reply
                .edit(
                    ctx,
                    poise::CreateReply {
                        content: Some(format!(
                            "Transcript for {}:\n>>> {}",
                            msg.link(),
                            transcript
                        )),
                        ..Default::default()
                    },
                )
                .await?;
            return Ok(());
        }
    }
}
