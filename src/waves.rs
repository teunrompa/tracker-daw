use crate::audio_source::PlayableSource;

#[derive(Debug, Clone)]
pub enum Waveform {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

#[derive(Debug)]
pub struct Oscillator {
    frequency: f32,
    phase: f32,
    sample_rate: f32,
    waveform: Waveform,
}

impl Oscillator {
    pub fn new(frequency: f32, sample_rate: f32, waveform: Waveform) -> Self {
        Self {
            frequency,
            phase: 0.0,
            sample_rate,
            waveform,
        }
    }

    fn advance_phase(&mut self) {
        self.phase += self.frequency / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
    }
}

impl PlayableSource for Oscillator {
    fn next_sample(&mut self) -> f32 {
        let output = match self.waveform {
            Waveform::Sine => (self.phase * 2.0 * std::f32::consts::PI).sin(),
            Waveform::Square => {
                if self.phase < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Sawtooth => (self.phase * 2.0) - 1.0,
            Waveform::Triangle => {
                if self.phase < 0.5 {
                    self.phase * 4.0 - 1.0
                } else {
                    3.0 - self.phase * 4.0
                }
            }
        };
        self.advance_phase();
        output
    }

    fn is_finished(&self) -> bool {
        false
    }
    fn release(&mut self) {}
}
