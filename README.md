# TransCrab
Discord transcription bot written in Rust, using Serenity and Poise
Transcription using Vosk

# Running
note: you will have issues on macOS because I couldn't easily find a vosk model for macOS

Download a Vosk model from [here](https://alphacephei.com/vosk/models) and extract the contents into the root folder. Name the resulting folder `model`
Add your discord token to `.env`
run `cargo make start-dev`

## Running production build
Download a Vosk model from [here](https://alphacephei.com/vosk/models) and extract the contents into the root folder. Name the resulting folder `model`
Add your discord token to `.env`
run `cargo make start-prod`

# Building
It should work on multiple platforms
It should work on NixOS (and probably anything using nix) using `nix-direnv`. You're welcome.

Need assistance? Open an issue and I might be able to help