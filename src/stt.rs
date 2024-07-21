use hound::WavReader;
use std::fs;
use std::process::Command;
use vosk::{Model, Recognizer};

pub fn transcribe(path: &std::path::Path) -> String {
    let model_path = "model";
    // Best way I could think of to get good path and also have the "temp.wav" at the end
    // bc canonicalize errors if the file doesn't exist ;-;
    let binding = fs::canonicalize("temp-files/").unwrap();
    let output_path = binding.to_str().unwrap().to_owned() + "temp.wav";

    Command::new("ffmpeg")
        .args(["-y", "-i", path.to_str().unwrap(), &output_path])
        .spawn()
        .expect("Failed to run the ffmpeg command whoops")
        .wait()
        .expect("Failed to run the ffmpeg command whoops"); // It yelled at me about an unused result, so I got pissy

    let mut reader =
        WavReader::open(&output_path).expect("Could not create the WAV reader");

    let samples = reader
        .samples()
        .collect::<hound::Result<Vec<i16>>>()
        .expect("Could not read WAV file");

    let model = Model::new(model_path).expect("Could not create the model");
    let mut recognizer = Recognizer::new(&model, reader.spec().sample_rate as f32)
        .expect("Could not create the recognizer");

    recognizer.set_max_alternatives(0);
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    for sample in samples.chunks(100) {
        recognizer.accept_waveform(sample);
    }

    recognizer.final_result().single().unwrap().text.to_string()
}
