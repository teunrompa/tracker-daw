use std::sync::{Arc, Mutex};

use cpal::{
    Device, Host, Stream, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

use crate::{mixer::Mixer, waves::SineWave};
//Sends audio to the output stream
pub struct AudioEngine {
    host: Host,
    device: Device,
    config: StreamConfig,
    pub sample_rate: f32,
    channels: u16,
}

impl AudioEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
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
        })
    }
    //main entry point for audio to come in
    pub fn start_with_mixer(
        &self,
        mixer: Arc<Mutex<Mixer>>,
    ) -> Result<Stream, Box<dyn std::error::Error>> {
        let config = self.config.clone();
        let device = self.device.clone();

        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut mixer = mixer.lock().unwrap();
                for sample in data.iter_mut() {
                    *sample = mixer.next_sample();
                }
            },
            |err| eprintln!("Stream Err {}", err),
            None,
        )?;

        stream.play()?;

        Ok(stream)
    }
}
