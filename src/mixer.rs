use crate::audio_source::{AudioSource, AudioSourceManager};

enum VoiceStealingStrategy {
    Oldest,
    Quietest,
    RejectNew,
}

pub struct Mixer {
    volume: f32,
    sample_rate: f32,
    pub source_manager: AudioSourceManager,
}

impl Mixer {
    pub fn new(volume: f32, sample_rate: f32) -> Self {
        Self {
            volume,
            sample_rate,
            source_manager: AudioSourceManager::new(Some(20), sample_rate),
        }
    }

    pub fn add_sources(&mut self, sources: Vec<Box<dyn AudioSource>>) {
        for source in sources {
            self.source_manager.add_source(source);
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        self.source_manager.remove_finished();
        let mixed = self.source_manager.mix_samples();
        (mixed * self.volume).clamp(-1.0, 1.0)
    }
}
