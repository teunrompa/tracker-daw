use crate::audio_source::AudioSource;

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
        }
    }

    fn next_amplitude(&mut self) -> f32 {
        let amplitude = match self.current_phase {
            EnvelopePhase::Attack => self.samples_in_phase as f32 / self.attack_samples as f32,
            EnvelopePhase::Decay => {
                let progress = self.samples_in_phase as f32 / self.decay_samples as f32;
                1.0 - (progress * (1.0 - self.sustain_level))
            }
            EnvelopePhase::Sustain => self.sustain_level,
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

        println!(
            "Current amplitude {}, Current Phase {:#?}",
            amplitude, self.current_phase
        );

        amplitude
    }

    fn release(&mut self) {
        self.current_phase = EnvelopePhase::Release;
        self.samples_in_phase = 0;
    }

    fn is_finished(&self) -> bool {
        self.current_phase == EnvelopePhase::Finished
    }
}

#[derive(Debug)]
pub struct EnvelopeSource {
    source: Box<dyn AudioSource>,
    envelope: Envelope,
}

impl EnvelopeSource {
    pub fn new(source: Box<dyn AudioSource>, envelope: Envelope) -> Self {
        EnvelopeSource { source, envelope }
    }
}

impl AudioSource for EnvelopeSource {
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
