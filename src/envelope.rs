use crate::duration::Duration;

use crate::audio_source::PlayableSource;
#[derive(Clone, Debug)]
pub struct EnvelopeBuilder {
    attack: Duration,
    decay: Duration,
    sustain_level: f32,
    release: Duration,
}

impl EnvelopeBuilder {
    pub fn new() -> Self {
        EnvelopeBuilder {
            attack: Duration::Seconds(0.0),
            decay: Duration::Seconds(0.0),
            sustain_level: 0.0,
            release: Duration::Seconds(0.0),
        }
    }

    pub fn attack(mut self, d: Duration) -> Self {
        self.attack = d;
        self
    }
    pub fn decay(mut self, d: Duration) -> Self {
        self.decay = d;
        self
    }
    pub fn sustain_level(mut self, level: f32) -> Self {
        self.sustain_level = level;
        self
    }
    pub fn release(mut self, d: Duration) -> Self {
        self.release = d;
        self
    }

    pub fn build(&self, sample_rate: f32) -> Envelope {
        Envelope::new(
            self.attack.to_samples(sample_rate),
            self.decay.to_samples(sample_rate),
            self.sustain_level,
            self.release.to_samples(sample_rate),
        )
    }
}

#[derive(PartialEq, Debug)]
pub enum EnvelopePhase {
    Attack,
    Decay,
    Sustain,
    Release,
    Finished,
}

#[derive(Debug)]
pub struct Envelope {
    attack_samples: u32,
    decay_samples: u32,
    sustain_level: f32,
    release_samples: u32,
    current_phase: EnvelopePhase,
    samples_in_phase: u32,
    max_sustain_samples: Option<u32>,
}

impl Envelope {
    pub fn new(
        attack_samples: u32,
        decay_samples: u32,
        sustain_level: f32,
        release_samples: u32,
    ) -> Self {
        Envelope {
            attack_samples,
            decay_samples,
            sustain_level,
            release_samples,
            current_phase: EnvelopePhase::Attack,
            samples_in_phase: 0,
            max_sustain_samples: None,
        }
    }

    fn next_amplitude(&mut self) -> f32 {
        let amplitude = match self.current_phase {
            EnvelopePhase::Attack => self.samples_in_phase as f32 / self.attack_samples as f32,
            EnvelopePhase::Decay => {
                let progress = self.samples_in_phase as f32 / self.decay_samples as f32;
                1.0 - (progress * (1.0 - self.sustain_level))
            }
            EnvelopePhase::Sustain => {
                if let Some(max_sustain) = self.max_sustain_samples
                    && self.samples_in_phase >= max_sustain
                {
                    self.current_phase = EnvelopePhase::Release;
                    self.samples_in_phase = 0;
                }

                self.sustain_level
            }
            EnvelopePhase::Release => {
                let progress = self.samples_in_phase as f32 / self.release_samples as f32;
                self.sustain_level * (1.0 - progress)
            }
            EnvelopePhase::Finished => 0.0,
        };

        self.samples_in_phase += 1;

        if self.current_phase == EnvelopePhase::Attack
            && self.samples_in_phase >= self.attack_samples
        {
            self.current_phase = EnvelopePhase::Decay;
            self.samples_in_phase = 0;
        } else if self.current_phase == EnvelopePhase::Decay
            && self.samples_in_phase >= self.decay_samples
        {
            self.current_phase = EnvelopePhase::Sustain;
            self.samples_in_phase = 0;
        } else if self.current_phase == EnvelopePhase::Release
            && self.samples_in_phase >= self.release_samples
        {
            self.current_phase = EnvelopePhase::Finished;
            self.samples_in_phase = 0;
        }

        amplitude
    }

    pub fn release(&mut self) {
        self.current_phase = EnvelopePhase::Release;
        self.samples_in_phase = 0;
    }

    pub fn set_max_sustain(&mut self, samples: u32) {
        self.max_sustain_samples = Some(samples);
    }

    fn is_finished(&self) -> bool {
        self.current_phase == EnvelopePhase::Finished
    }
}

#[derive(Debug)]
pub struct EnvelopeSource {
    source: Box<dyn PlayableSource>,
    envelope: Envelope,
}

impl EnvelopeSource {
    pub fn new(source: Box<dyn PlayableSource>, envelope: Envelope) -> Self {
        EnvelopeSource { source, envelope }
    }
}

impl PlayableSource for EnvelopeSource {
    fn next_sample(&mut self) -> f32 {
        self.source.next_sample() * self.envelope.next_amplitude()
    }

    fn is_finished(&self) -> bool {
        self.envelope.is_finished()
    }

    fn release(&mut self) {
        self.envelope.release()
    }
}
