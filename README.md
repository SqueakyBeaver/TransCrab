# TransCrab
Discord transcription bot written in Rust, using Serenity and Poise
Transcription using Vosk

# Running
Download a Vosk model from [here](https://alphacephei.com/vosk/models) and extract the contents into the root folder. Name the resulting folder `model`

There will eventually be a cargo-make script, but for now, move the contents of `lib` and the entire directory `model` into `target/<folder>` (`<folder>` being something like `debug` or `release`)
Add your discord token to `.env`

# Building
It might work idk
It should work on NixOS (and probably anything using nix) using `nix-direnv`. You're welcome.

Need assistance? Open an issue and I might be able to help