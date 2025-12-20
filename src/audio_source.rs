use std::fmt::Debug;

use crate::{
    duration::Duration,
    envelope::{EnvelopeBuilder, EnvelopeSource},
    note::Note,
    waves::SineWave,
};

pub trait AudioSource: Send + Debug {
    fn next_sample(&mut self) -> f32;
    fn is_finished(&self) -> bool;
    fn release(&mut self);
}

pub struct AudioSourceManager {
    pub sources: Vec<Box<dyn AudioSource>>,
    max_sources: Option<usize>,
    sample_rate: f32,
}

impl AudioSourceManager {
    pub fn new(max_sources: Option<usize>, sample_rate: f32) -> Self {
        AudioSourceManager {
            sources: Vec::new(),
            max_sources,
            sample_rate,
        }
    }

    pub fn play_note(&mut self, note: Note, duration: Duration, envelope: &EnvelopeBuilder) {
        let mut env = envelope.build(self.sample_rate);

        env.set_max_sustain(duration.to_samples(self.sample_rate));

        let enveloped_source = EnvelopeSource::new(
            Box::new(SineWave::new(note.to_frequency(), self.sample_rate)),
            env,
        );

        self.add_source(Box::new(enveloped_source));
    }

    pub fn add_source(&mut self, source: Box<dyn AudioSource>) {
        self.check_max_sources();
        self.sources.push(source);
    }

    fn check_max_sources(&mut self) {
        if let Some(max) = self.max_sources {
            while self.sources.len() > max {
                // Changed >= to >
                self.sources.remove(0);
            }
        }
    }

    pub fn remove_finished(&mut self) {
        self.sources.retain(|s| !s.is_finished());
    }

    pub fn mix_samples(&mut self) -> f32 {
        self.sources.iter_mut().map(|s| s.next_sample()).sum()
    }
}
