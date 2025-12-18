mod audio_engine;
mod audio_source;
mod duration;
mod envelope;
mod mixer;
mod note;
mod waves;

use std::sync::{Arc, Mutex};

use crate::{
    audio_engine::AudioEngine, duration::Duration, envelope::EnvelopeBuilder, mixer::Mixer,
    note::Note,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_engine = AudioEngine::new()?;
    let sample_rate = audio_engine.sample_rate;
    let mixer_arc = Arc::new(Mutex::new(Mixer::new(0.3, sample_rate)));

    let envelope = EnvelopeBuilder::new()
        .attack(Duration::Seconds(3.05))
        .decay(Duration::Seconds(4.0))
        .sustain_level(0.3)
        .release(Duration::Seconds(5.0));

    mixer_arc
        .lock()
        .unwrap()
        .play_note(Note::C3, Duration::Seconds(1.0), &envelope);

    let _stream = audio_engine.start_with_mixer(Arc::clone(&mixer_arc))?;

    std::thread::sleep(std::time::Duration::from_secs(10));
    Ok(())
}
