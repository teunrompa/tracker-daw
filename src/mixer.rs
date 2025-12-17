use crate::audio_source::AudioSource;

pub struct Mixer {
    sources: Vec<Box<dyn AudioSource>>,
    volume: f32,
}

impl Mixer {
    pub fn new(volume: f32) -> Self {
        Self {
            sources: Vec::new(),
            volume,
        }
    }

    pub fn add_source(&mut self, source: Box<dyn AudioSource>) {
        self.sources.push(source);
    }

    pub fn next_sample(&mut self) -> f32 {
        self.sources.retain(|s| !s.is_finished());

        let mixed: f32 = self.sources.iter_mut().map(|s| s.next_sample()).sum();

        (mixed * self.volume).clamp(-1.0, 1.0)
    }
}
