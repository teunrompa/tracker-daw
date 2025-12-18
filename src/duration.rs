pub enum Duration {
    Samples(u32),
    Seconds(f32),
}

impl Duration {
    pub fn to_samples(&self, sample_rate: f32) -> u32 {
        match self {
            Duration::Samples(s) => *s,
            Duration::Seconds(sec) => (sec * sample_rate) as u32,
        }
    }
}
