mod audio_engine;
mod audio_source;
mod duration;
mod envelope;
mod mixer;
mod note;
mod waves;

use std::sync::{Arc, Mutex};

use crate::{
    audio_engine::AudioEngine,
    duration::Duration,
    envelope::{EnvelopeBuilder, EnvelopeSource},
    mixer::Mixer,
    note::Note,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_engine = AudioEngine::new()?;
    let sample_rate = audio_engine.sample_rate;
    let mixer_arc = Arc::new(Mutex::new(Mixer::new(0.3, sample_rate)));

    let envelope = EnvelopeBuilder::new()
        .attack(Duration::Seconds(3.05))
        .decay(Duration::Seconds(0.1))
        .sustain_level(0.7)
        .release(Duration::Seconds(3.0));

    {
        let mut mixer = mixer_arc.lock().unwrap();
        mixer
            .source_manager
            .play_note(Note::E3, Duration::Seconds(2.0), &envelope);
    }

    let _stream = audio_engine.start_with_mixer(Arc::clone(&mixer_arc))?;
    std::thread::sleep(std::time::Duration::from_secs(5));
    Ok(())
}
