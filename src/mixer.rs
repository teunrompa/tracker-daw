use crate::{
    audio_source::{AudioSourceManager, PlayableSource},
    envelope::EnvelopeSource,
    sequencer::{self, Sequencer},
    waves::Oscillator,
};

pub struct Mixer {
    volume: f32,
    sample_rate: f32,
    pub source_manager: AudioSourceManager,
    pub sequencer: Sequencer,
}

impl Mixer {
    pub fn new(volume: f32, sample_rate: f32, sequencer: Sequencer) -> Self {
        Self {
            volume,
            sample_rate,
            source_manager: AudioSourceManager::new(Some(20)),
            sequencer,
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let events = self.sequencer.tick();

        for event in events {
            // Create oscillator with the instrument's waveform
            let oscillator = Oscillator::new(
                event.note.to_frequency(),
                self.sample_rate,
                event.instrument.waveform,
            );

            // Wrap with envelope from the instrument
            let mut envelope = event.instrument.envelope.build(self.sample_rate);
            envelope.set_max_sustain(event.duration.to_samples(self.sample_rate));

            let enveloped_source = EnvelopeSource::new(Box::new(oscillator), envelope);

            self.source_manager.add_source(Box::new(enveloped_source));
        }

        self.source_manager.remove_finished();
        let mixed = self.source_manager.mix_samples();
        let num_sources = self.source_manager.sources.len().max(1) as f32;
        let normalized = mixed / num_sources;

        (normalized * self.volume).clamp(-1.0, 1.0)
    }
}
