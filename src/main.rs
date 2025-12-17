mod audio_engine;
mod audio_source;
mod mixer;
mod note;
mod waves;

use std::sync::{Arc, Mutex};

use crate::{audio_engine::AudioEngine, mixer::Mixer, note::Note};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_engine = AudioEngine::new()?;
    let sample_rate = audio_engine.sample_rate;
    let mixer_arc = Arc::new(Mutex::new(Mixer::new(0.3, sample_rate)));

    mixer_arc.lock().unwrap().play_chord(&[
        Note::B3.to_frequency(),
        Note::A3.to_frequency(),
        Note::E3.to_frequency(),
        Note::F3.to_frequency(),
    ]);

    let _stream = audio_engine.start_with_mixer(Arc::clone(&mixer_arc))?;

    std::thread::sleep(std::time::Duration::from_secs(2));

    Ok(())
}
