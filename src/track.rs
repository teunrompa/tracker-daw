use crate::{
    duration::Duration,
    note::Note,
    sequencer::{Instrument, NoteEvent},
};

#[derive(Clone)]
pub struct Track {
    pub instrument: Instrument,
    pub events: Vec<NoteEvent>,
}

#[derive(Debug, Clone)]
pub struct TriggerdEvent {
    pub note: Note,
    pub duration: Duration,
    pub instrument: Instrument,
}
