use cpal::{
    Device, Host, Stream, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

use crate::waves::SineWave;

pub struct AudioEngine {
    host: Host,
    device: Device,
    config: StreamConfig,
    sample_rate: f32,
    channels: u16,
    global_volume: f32,
}

impl AudioEngine {
    pub fn new(global_volume: f32) -> Result<Self, Box<dyn std::error::Error>> {
        let host = cpal::default_host();

        //Access output divice
        let device = host
            .default_output_device()
            .ok_or("No Audio device was avalable")?;

        //Find the format of the default_output_device
        let config_format = device.default_output_config()?;
        let sample_rate = config_format.sample_rate().0 as f32;
        let channels = config_format.channels();

        let config = config_format.into();

        Ok(AudioEngine {
            host,
            device,
            config,
            sample_rate,
            channels,
            global_volume,
        })
    }

    pub fn play_tone(&self, frequency: f32) -> Result<Stream, Box<dyn std::error::Error>> {
        let config = self.config.clone();
        let device = self.device.clone();
        let global_volume = self.global_volume;

        let mut sine = SineWave::new(frequency, self.sample_rate);

        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    let value = sine.next_sample();
                    *sample = value * global_volume;
                }
            },
            |err| eprintln!("Stream Err {}", err),
            None,
        )?;

        stream.play()?;

        Ok(stream)
    }
}
