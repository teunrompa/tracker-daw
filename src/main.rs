mod audio_engine;
mod audio_source;
mod duration;
mod envelope;
mod mixer;
mod note;
mod sequencer;
mod track;
mod waves;

use std::sync::{Arc, Mutex};

use crate::{
    audio_engine::AudioEngine,
    duration::Duration,
    envelope::EnvelopeBuilder,
    mixer::Mixer,
    note::Note,
    sequencer::{Instrument, NoteEvent, Sequencer},
    track::Track,
    waves::Waveform,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_engine = AudioEngine::new()?;
    let sample_rate = audio_engine.sample_rate;

    // Define instruments
    let bass_instrument = Instrument {
        waveform: Waveform::Sine,
        envelope: EnvelopeBuilder::new()
            .attack(Duration::Seconds(0.01))
            .decay(Duration::Seconds(0.4))
            .sustain_level(1.0)
            .release(Duration::Seconds(0.2)),
    };

    let lead_instrument = Instrument {
        waveform: Waveform::Square,
        envelope: EnvelopeBuilder::new()
            .attack(Duration::Seconds(0.05))
            .decay(Duration::Seconds(0.1))
            .sustain_level(0.1)
            .release(Duration::Seconds(0.3)),
    };

    // Create sequencer and add tracks
    let mut sequencer = Sequencer::new(sample_rate);

    // Bass track
    sequencer.add_track(Track {
        instrument: bass_instrument,
        events: vec![
            NoteEvent {
                note: Note::C2,
                start_time: Duration::Seconds(0.0),
                duration: Duration::Seconds(0.5),
            },
            NoteEvent {
                note: Note::G1,
                start_time: Duration::Seconds(0.5),
                duration: Duration::Seconds(0.5),
            },
            NoteEvent {
                note: Note::C1,
                start_time: Duration::Seconds(1.0),
                duration: Duration::Seconds(0.5),
            },
            NoteEvent {
                note: Note::G1,
                start_time: Duration::Seconds(1.0),
                duration: Duration::Seconds(0.5),
            },
        ],
    });

    // Lead track
    sequencer.add_track(Track {
        instrument: lead_instrument,
        events: vec![
            NoteEvent {
                note: Note::C2,
                start_time: Duration::Seconds(0.0),
                duration: Duration::Seconds(0.3),
            },
            NoteEvent {
                note: Note::E3,
                start_time: Duration::Seconds(0.5),
                duration: Duration::Seconds(0.3),
            },
            NoteEvent {
                note: Note::G3,
                start_time: Duration::Seconds(1.0),
                duration: Duration::Seconds(0.5),
            },
        ],
    });

    sequencer.set_looping(true);
    sequencer.play();

    let mixer_arc = Arc::new(Mutex::new(Mixer::new(0.3, sample_rate, sequencer)));
    let _stream = audio_engine.start_with_mixer(Arc::clone(&mixer_arc))?;

    std::thread::sleep(std::time::Duration::from_secs(5));
    Ok(())
}
