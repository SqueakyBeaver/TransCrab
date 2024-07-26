use crate::{Context, Error};
use hound::WavReader;
use poise::serenity_prelude as serenity;
use std::fs;
use std::fs::File;
use std::io::{copy, Cursor};
use std::process::Command;
use vosk::{Model, Recognizer};

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
                        Some(String::from("tmp.") + name.split_terminator(".").last().unwrap())
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

fn convert(path: &String) -> Result<String, Error> {
    // Best way I could think of to get good path and also have the "temp.wav" at the end
    // bc canonicalize errors if the file doesn't exist ;-;
    let binding = fs::canonicalize("temp-files/").unwrap();
    let output_path = binding.to_str().unwrap().to_owned() + "/temp.wav";

    let output = Command::new("ffmpeg")
        .args(["-y", "-i", path, &output_path])
        .output()
        .expect("ffmpeg error");

    // There's an unstable feature to make things more consistent here,
    // but I don't feel like switching to experimental toolchain
    if output.status.success() {
        return Ok(output_path);
    }

    println!(
        "FFMPEG ERROR: {:?}\nStdout: {:?}",
        output.stderr, output.stdout
    );
    Err(Error::from("ffmpeg error"))
}

fn transcribe(path: &String) -> Result<String, Error> {
    let output_path = convert(path);

    match output_path {
        Err(err) => Err(err),
        Ok(path) => {
            // The compiler told me to do it this way
            // I don't want to, but I will
            let binding = fs::canonicalize("model").expect("Model path doesn't exist");
            let model_path = binding.to_str().expect("Model not found");

            let mut reader = WavReader::open(&path).expect("Could not create the WAV reader");

            let samples = reader
                .samples()
                .collect::<hound::Result<Vec<i16>>>()
                .expect("Could not read WAV file");

            let model = Model::new(model_path).expect("Could not create the model");
            let mut recognizer = Recognizer::new(&model, reader.spec().sample_rate as f32)
                .expect("Could not create the recognizer");

            for sample in samples.chunks(100) {
                recognizer.accept_waveform(sample);
            }

            let res = String::from(recognizer.final_result().single().unwrap().text);
            Ok(res)
        }
    }
}

#[poise::command(context_menu_command = "Transcribe audio in message")]
pub async fn exec(
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

    let transcription = transcribe(&fname);

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
                            "Transcript for {}:\n>>> {}\n-# Note: There may be errors. I am sorry.",
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
