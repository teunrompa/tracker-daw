use crate::duration::Duration;
use crate::envelope::EnvelopeBuilder;
use crate::note::Note;
use crate::track::{Track, TriggerdEvent};
use crate::waves::Waveform;

#[derive(Clone, Debug)]
pub struct NoteEvent {
    pub note: Note,
    pub start_time: Duration,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct Instrument {
    pub waveform: Waveform,
    pub envelope: EnvelopeBuilder,
}

pub struct Sequencer {
    tracks: Vec<Track>,
    current_sample: u64,
    sample_rate: f32,
    is_playing: bool,
    is_looping: bool,
    loop_end_sample: Option<u64>,
}

impl Sequencer {
    pub fn new(sample_rate: f32) -> Self {
        Sequencer {
            tracks: Vec::new(),
            current_sample: 0,
            sample_rate,
            is_playing: false,
            is_looping: false,
            loop_end_sample: None,
        }
    }

    pub fn tick(&mut self) -> Vec<TriggerdEvent> {
        if !self.is_playing {
            return Vec::new();
        }

        // Check for loop
        if self.is_looping {
            if let Some(end) = self.loop_end_sample {
                if self.current_sample >= end {
                    self.current_sample = 0; // Loop back to start
                }
            }
        }

        let mut triggerd = Vec::new();

        for track in &self.tracks {
            for event in &track.events {
                let event_sample = event.start_time.to_samples(self.sample_rate) as u64;
                if event_sample == self.current_sample {
                    triggerd.push(TriggerdEvent {
                        note: event.note,
                        duration: event.duration.clone(),
                        instrument: track.instrument.clone(),
                    });
                }
            }
        }

        self.increment_time();
        triggerd
    }
    // Calculate the end time when tracks are added/changed
    fn calculate_loop_end(&mut self) {
        let mut max_sample = 0u64;

        for track in &self.tracks {
            for event in &track.events {
                let event_end = (event.start_time.to_samples(self.sample_rate)
                    + event.duration.to_samples(self.sample_rate))
                    as u64;
                max_sample = max_sample.max(event_end);
            }
        }

        self.loop_end_sample = if max_sample > 0 {
            Some(max_sample)
        } else {
            None
        };
    }

    pub fn set_looping(&mut self, looping: bool) {
        self.is_looping = looping;
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
        self.calculate_loop_end();
    }

    pub fn play(&mut self) {
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
    }

    pub fn reset(&mut self) {
        self.current_sample = 0;
    }

    pub fn increment_time(&mut self) {
        self.current_sample += 1;
    }
}
