pub struct SineWave {
    frequency: f32,
    phase: f32,
    sample_rate: f32,
}

impl SineWave {
    pub fn new(frequency: f32, sample_rate: f32) -> Self {
        SineWave {
            frequency,
            phase: 0.0,
            sample_rate,
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let value =
            (self.phase * self.frequency * 2.0 * std::f32::consts::PI / self.sample_rate).sin();

        self.phase += 1.0;

        value
    }
}
