use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::{ sync::{Arc, Mutex}};


#[derive(Clone, Debug)]
pub enum Note {
    Note(u8),
    NoteOff(),
    Empty,
}

impl Note {
    pub fn to_string(&self) -> String {
        match self {
            Note::Note(num) => {
                let note_names = ["C-", "C#", "D-", "D#", "E-", "F-", 
                                  "F#", "G-", "G#", "A-", "A#", "B-"];

                let octave = num / 12;
                let note = (num % 12) as usize;

                format!("{}{}", note_names[note], octave)
            }
            Note::NoteOff() => "OFF".to_string(),
            Note::Empty => "---".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sample {
    pub data: Vec<f32>,
    pub sample_data: u32,
    pub name: String,
}

impl Sample {
    pub fn sine_wave(freqency: f32, duration: f32, sample_rate: u32) -> Self {
        let num_samples = (duration * sample_rate as f32 ) as usize;
        let mut data = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let sample = (t * freqency * 2.0 * std::f32::consts::PI).sin();
            data.push(sample);
        }

        Sample {
            data: data,
            sample_data: sample_rate,
            name: format!("Sine {}Hz", freqency)
        }
    }

    pub fn from_wav(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        println!("Reading path: {}", path);

        let mut reader = hound::WavReader::open(path)?;
        let spec = reader.spec();

        println!("      sample rate:     {}", spec.sample_rate);
        println!("      Channels:        {}", spec.channels);
        println!("      Bits per sample: {}", spec.bits_per_sample);

        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => {
                reader.samples::<f32>().map(|s| s.unwrap()).collect()
            }
            hound::SampleFormat::Int => {
                let max_val = (1 << (spec.bits_per_sample - 1)) as f32;
                reader.samples::<i32>().map(|s| s.unwrap() as f32  / max_val).collect()
            }
        };


        let mono_samples = if spec.channels == 2 {
            samples.chunks(2).map(|chunk| (chunk[0] + chunk[1]) / 2.0).collect()
        } else {
            samples
        };

        println!("  Loaded {} samples", mono_samples.len());

        Ok(Sample { data: mono_samples, sample_data: spec.sample_rate, name: path.to_string() })
        
    }
}

pub struct Voice {
    sample: Option<Sample>,
    position: f32,
    speed: f32,
    volume: f32,
    active: bool
}

impl Voice {
    pub fn new() -> Self {
        Voice {
            sample: None,
            position: 0.0,
            speed: 1.0,
            volume: 1.0,
            active: false
        }
    }

    pub fn trigger(&mut self, sample: Sample, note: u8){
        let semitones_from_a4 = note as f32 - 69.0;
        let pitch_multiplier     = 2.0_f32.powf(semitones_from_a4 / 12.0);


        self.sample = Some(sample);
        self.position = 0.0;
        self.speed = pitch_multiplier;
        self.active = true;

        println!("  🎵 Voice triggered: note {} (pitch: {:.3}x)", note, pitch_multiplier);
    }

    pub fn process(&mut self) -> f32 {
        if !self.active {
            return 0.0
        }

        let sample = match &self.sample {
            Some(s) => s,
            None => return 0.0
        };

        if self.position >= sample.data.len() as f32 {
            self.active = false;
            return 0.0;
        }


        let index = self.position as usize;
        let fraction = self.position - index as f32;

        let sample1 = sample.data.get(index).copied().unwrap_or(0.0);
        let sample2 = sample.data.get(index + 1).copied().unwrap_or(sample1);

        let output = sample1 + (sample2 - sample1) * fraction;

        self.position += self.speed;
        output * self.volume
    }

    pub fn stop(&mut self) {
        self.active = false;
        println!("Voice Stopped");
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}



#[derive(Clone, Debug)]
pub struct PatternCell {
    ///What note to play
    pub note: Note,
    
    ///Witch instrument should be played
    pub instrument: Option<u8>,

    pub volume: Option<u8>,

    pub effect: Option<String>,
}

impl PatternCell {
    pub fn empty() -> Self {
        PatternCell { note: Note::Empty, instrument: None, volume: None, effect: None }
    }

    pub fn display(&self) -> String {
        let note_str = self.note.to_string();
        let inst_str = match self.instrument {
            Some(i) => format!("{0:02x}", i),
            None => "--".to_string(),
        };

        let vol_str = match self.volume {
            Some(v) => format!("{0:02x}", v),
            None => "--".to_string(),
        };

        format!("{} {} {}", note_str, inst_str, vol_str)
    }
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub rows: Vec<Vec<PatternCell>>,

    pub length: usize,

    pub channels: usize,
}

impl Pattern {
    pub fn new(length: usize, channels: usize) -> Self {
        let rows = vec![
            vec![PatternCell::empty(); channels];
            length
        ];
        

        Pattern { rows, length, channels }
    }

    pub fn set_note(&mut self, row: usize, channel: usize, note: u8, instrument: u8, volume: u8) {
        if row < self.length && channel < self.channels {
            self.rows[row][channel] = PatternCell { note: Note::Note(note), instrument: Some(instrument), volume: Some(volume), effect: None };
        }
    }

    pub fn display(&self) {
                println!("\n╔════════════════════════════════════════════╗");
        println!("║  ROW │ CH1      │ CH2      │ CH3      │ CH4      ║");
        println!("╠════════════════════════════════════════════╣");

        for (row_num, row) in self.rows.iter().enumerate() {
                 print!("║  {:02}  │", row_num);
            
            for cell in row.iter() {
                print!(" {} │", cell.display());
            }
            
            println!();
        }

                println!("╚════════════════════════════════════════════╝");
    }
}


pub fn start_audio_stream(voice: Arc<Mutex<Voice>>) -> Result<cpal::Stream, Box<dyn std::error::Error>> {
    println!("Initializing audio...\n");

    let host = cpal::default_host();
    let device = host.default_output_device().ok_or("No audio output device was found!")?;

    println!("  Device: {}", device.name()?);

    let config = device.default_output_config()?;

    println!("  Sample rate: {} Hz", config.sample_rate().0);
    println!("  Channels: {}", config.channels());
    println!("  Format: {:?}\n", config.sample_format());

    let stream = device.build_output_stream(&config.into(), 
        move |data: &mut [f32], _:&cpal::OutputCallbackInfo| {
            let mut voice = voice.lock().unwrap();

            for frame in data.chunks_mut(2) {
                let sample = voice.process();

                frame[0] = sample;

                if frame.len() > 1 {
                    frame[1] = sample;
                }
            }
        }, |err| eprintln!("Audio stream error {}", err), None)?;

        stream.play()?;

        println!("Audio stream started!!");

        Ok(stream)

}

fn main() -> Result<(), Box<dyn std::error::Error>>  {
   println!("🎵 TRACKER LESSON 3: Real Audio Output\n");
    println!("═══════════════════════════════════════\n");
    
    // Create a test sample (sine wave)
    let sample = Sample::sine_wave(440.0, 1.0, 44100);
    println!("📦 Created test sample: 440Hz sine wave\n");
    
    // If you have WAV files, you can load them like this:
    // let sample = Sample::from_wav("path/to/kick.wav")?;
    
    // Create a voice wrapped in Arc<Mutex<>> so we can share it
    // between the main thread and the audio thread
    let voice = Arc::new(Mutex::new(Voice::new()));
    
    // Start the audio stream
    let _stream = start_audio_stream(voice.clone())?;
    
    println!("🎹 Playing a melody...\n");
    println!("(Each note plays for 0.5 seconds)\n");
    
    // Play some notes!
    let melody = [
        (60, "C4"),  // Middle C
        (64, "E4"),  // E
        (67, "G4"),  // G
        (72, "C5"),  // High C
        (67, "G4"),  // G
        (64, "E4"),  // E
        (60, "C4"),  // C
    ];
    
    for (note, name) in melody.iter() {
        {
            let mut v = voice.lock().unwrap();
            v.trigger(sample.clone(), *note);
        }
        println!("  ♪ Playing: {} (MIDI note {})", name, note);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    

    Ok(())
 }

