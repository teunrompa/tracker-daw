use std::fmt::Debug;

use crate::{
    duration::Duration,
    envelope::{EnvelopeBuilder, EnvelopeSource},
};

pub trait PlayableSource: Send + Debug {
    fn next_sample(&mut self) -> f32;
    fn is_finished(&self) -> bool;
    fn release(&mut self);
}

pub struct AudioSourceManager {
    pub sources: Vec<Box<dyn PlayableSource>>,
    max_sources: Option<usize>,
}

impl AudioSourceManager {
    pub fn new(max_sources: Option<usize>) -> Self {
        AudioSourceManager {
            sources: Vec::new(),
            max_sources,
        }
    }

    pub fn add_source(&mut self, source: Box<dyn PlayableSource>) {
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
