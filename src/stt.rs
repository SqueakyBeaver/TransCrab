use std::ffi::OsStr;
use std::fs::File;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use vosk::{Model, Recognizer};

pub fn transcribe(path: &std::path::Path) -> String {
    // This bit is taken from the symphonia basic-interleaved example

    // Create a media source. Note that the MediaSource trait is automatically implemented for File,
    // among other types.
    let file = Box::new(File::open(path).unwrap());

    // Create the media source stream using the boxed media source from above.
    let mss = MediaSourceStream::new(file, Default::default());

    // Create a hint to help the format registry guess what format reader is appropriate.
    let mut hint = Hint::new();
    hint.with_extension(path.extension().and_then(OsStr::to_str).unwrap());

    // Use the default options when reading and decoding.
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    let decoder_opts: DecoderOptions = Default::default();

    // Probe the media source stream for a format.
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .unwrap();

    // Get the format reader yielded by the probe operation.
    let mut format = probed.format;

    // Get the default track.
    let track = format.default_track().unwrap();

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_opts)
        .unwrap();

    // Store the track identifier, we'll use it to filter packets.
    let track_id = track.id;

    let mut sample_count = 0;
    let mut sample_buf: Option<SampleBuffer<i16>> = None;
    let mut sample_rate = 0;

    loop {
        // Get the next packet from the format reader.
        let packet = format.next_packet().unwrap();

        // If the packet does not belong to the selected track, skip it.
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the packet into audio samples, ignoring any decode errors.
        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                // The decoded audio samples may now be accessed via the audio buffer if per-channel
                // slices of samples in their native decoded format is desired. Use-cases where
                // the samples need to be accessed in an interleaved order or converted into
                // another sample format, or a byte buffer is required, are covered by copying the
                // audio buffer into a sample buffer or raw sample buffer, respectively. In the
                // example below, we will copy the audio buffer into a sample buffer in an
                // interleaved order while also converting to a f32 sample format.

                // If this is the *first* decoded packet, create a sample buffer matching the
                // decoded audio buffer format.
                if sample_buf.is_none() {
                    // Get the audio buffer specification.
                    let spec = *audio_buf.spec();

                    // Get the capacity of the decoded buffer. Note: This is capacity, not length!
                    let duration = audio_buf.capacity() as u64;

                    // // Create a raw sample buffer that matches the parameters of the decoded audio buffer.
                    // let mut byte_buf =
                    //     RawSampleBuffer::<f32>::new(audio_buf.capacity() as u64, *audio_buf.spec());

                    // // Copy the contents of the decoded audio buffer into the sample buffer whilst performing
                    // // any required conversions.
                    // byte_buf.copy_interleaved_ref(audio_buf);

                    // // The interleaved f32 samples can be accessed as a slice of bytes as follows.
                    // let bytes = byte_buf.as_bytes();


                    // Create the f32 sample buffer.
                    sample_buf = Some(SampleBuffer::<i16>::new(duration, spec));

                    sample_rate = audio_buf.spec().rate;
                }

                // Copy the decoded audio buffer into the sample buffer in an interleaved format.
                if let Some(buf) = &mut sample_buf {
                    buf.copy_interleaved_ref(audio_buf);

                    // The samples may now be access via the `samples()` function.
                    sample_count += buf.len();
                    print!("\rDecoded {} samples", sample_count);
                }
            }
            Err(Error::DecodeError(_)) => (),
            Err(_) => break,
        }
    }

    let model_path = "/model";

    let model = Model::new(model_path).expect("Could not create the model");
    let mut recognizer =
        Recognizer::new(&model, sample_rate as f32).expect("Could not create the recognizer");

    recognizer.set_max_alternatives(0);
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    recognizer.accept_waveform(sample_buf.unwrap().samples());

    recognizer.final_result().single().unwrap().text.to_string()
}
