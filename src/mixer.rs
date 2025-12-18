use crate::{
    audio_source::AudioSource,
    duration::Duration,
    envelope::{Envelope, EnvelopeBuilder, EnvelopeSource},
    note::Note,
    waves::SineWave,
};

pub struct Mixer {
    sources: Vec<Box<dyn AudioSource>>,
    volume: f32,
    sample_rate: f32,
}

impl Mixer {
    pub fn new(volume: f32, sample_rate: f32) -> Self {
        Self {
            sources: Vec::new(),
            volume,
            sample_rate,
        }
    }

    pub fn play_note(
        &mut self,
        note: Note,
        duration: Duration,
        envelope: &EnvelopeBuilder,
    ) -> usize {
        let mut env = envelope.build(self.sample_rate);

        env.set_max_sustain(duration.to_samples(self.sample_rate));

        let enveloped_source = EnvelopeSource::new(
            Box::new(SineWave::new(note.to_frequency(), self.sample_rate)),
            env,
        );

        let id = self.sources.len();
        self.sources.push(Box::new(enveloped_source));
        id
    }

    pub fn add_source(&mut self, source: Box<dyn AudioSource>) {
        self.sources.push(source);
    }

    pub fn add_sources(&mut self, sources: Vec<Box<dyn AudioSource>>) {
        self.sources.extend(sources);
    }

    pub fn next_sample(&mut self) -> f32 {
        self.sources.retain(|s| !s.is_finished());

        let mixed: f32 = self.sources.iter_mut().map(|s| s.next_sample()).sum();

        (mixed * self.volume).clamp(-1.0, 1.0)
    }

    //Helper methods
    pub fn add_sine_wave(&mut self, frequency: f32) {
        self.add_source(Box::new(SineWave::new(frequency, self.sample_rate)));
    }

    pub fn play_chord(&mut self, frequencies: &[f32]) {
        for &freq in frequencies {
            self.add_sine_wave(freq);
        }
    }
}
