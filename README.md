# TransCrab
Discord transcription bot written in Rust, using Serenity and Poise
Transcription using Whisper-rs

# Running dev build
Download a Whisper.cpp model from [here](https://huggingface.co/ggerganov/whisper.cpp/tree/main) to the root directory and name the file `model.bin`. I suggest one of the `tiny.en` models, since they use very little memory.
Add your discord token to `.env`
run `cargo make start-dev` or `cargo run`

## Running production build
same process as a dev build, except run `cargo make start-prod` or `cargo run -r`

# Building
It should work on multiple platforms
It will work on NixOS (and probably anything using nix) using `nix-direnv`. You're welcome.

Need assistance? Open an issue and I might be able to help