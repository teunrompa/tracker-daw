use crate::{
    audio_source::AudioSource,
    duration::Duration,
    envelope::{EnvelopeBuilder, EnvelopeSource},
    note::Note,
    waves::SineWave,
};

enum VoiceStealingStrategy {
    Oldest,
    Quietest,
    RejectNew,
}

pub struct Mixer {
    sources: Vec<Box<dyn AudioSource>>,
    max_voices: Option<usize>,
    volume: f32,
    sample_rate: f32,
}

impl Mixer {
    pub fn new(volume: f32, sample_rate: f32) -> Self {
        Self {
            sources: Vec::new(),
            max_voices: None,
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
        self.add_source(Box::new(enveloped_source));
        id
    }

    pub fn add_source(&mut self, source: Box<dyn AudioSource>) {
        self.check_max_sources();

        //Add new source
        self.sources.push(source);
    }

    pub fn add_sources(&mut self, sources: Vec<Box<dyn AudioSource>>) {
        for source in sources {
            self.check_max_sources();
            self.sources.push(source);
        }
    }

    //Remove oldest voice if max voice is reached
    fn check_max_sources(&mut self) {
        if let Some(max) = self.max_voices
            && self.sources.len() >= max
        {
            self.sources.remove(0);
        }
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
