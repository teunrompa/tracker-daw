mod audio_engine;
mod note;
mod waves;

use crate::note::Note;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "C0 enum={}, Midi{}, freq={:.2} hz",
        Note::C0 as u8,
        Note::C0.to_midi(),
        Note::C0.to_freqency()
    );
    Ok(())
}
