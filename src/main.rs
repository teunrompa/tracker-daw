mod audio_engine;
mod audio_source;
mod envelope;
mod mixer;
mod note;
mod waves;

use std::sync::{Arc, Mutex};

use crate::{
    audio_engine::AudioEngine,
    envelope::{Envelope, EnvelopeSource},
    mixer::Mixer,
    note::Note,
    waves::SineWave,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_engine = AudioEngine::new()?;
    let sample_rate = audio_engine.sample_rate;
    let mixer_arc = Arc::new(Mutex::new(Mixer::new(0.3, sample_rate)));

    let envelope = Envelope::new(300, 10000, 0.3, 40);

    let enveloped = EnvelopeSource::new(
        Box::new(SineWave::new(Note::C3.to_frequency(), sample_rate)),
        envelope,
    );

    mixer_arc.lock().unwrap().add_source(Box::new(enveloped));

    let _stream = audio_engine.start_with_mixer(Arc::clone(&mixer_arc))?;

    std::thread::sleep(std::time::Duration::from_secs(10));

    Ok(())
}
