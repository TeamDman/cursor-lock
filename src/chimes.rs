// src/chimes.rs

use std::io::{BufReader, Cursor};
use rodio::{Decoder, OutputStream, Sink};
use eyre::Result;

// Embed the MP3 files directly into the binary.
const ACTIVATE_MP3: &[u8] = include_bytes!("assets/activate.mp3");
const DEACTIVATE_MP3: &[u8] = include_bytes!("assets/deactivate.mp3");

/// Plays the provided audio data.
/// This function creates an output stream, decodes the audio from memory,
/// and plays it synchronously (waiting until playback completes).
pub fn play_sound_from_bytes(audio_data: &'static [u8]) -> Result<()> {
    // Create an output stream.
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    
    // Create a Cursor so we can use the audio data as a stream.
    let cursor = Cursor::new(audio_data);
    let reader = BufReader::new(cursor);
    
    // Decode the audio.
    let source = Decoder::new(reader)?;
    
    // Append the source to the sink.
    sink.append(source);
    
    // Wait until the sound finishes playing.
    sink.sleep_until_end();
    
    Ok(())
}

/// Plays the activation chime.
pub fn play_activation() -> Result<()> {
    play_sound_from_bytes(ACTIVATE_MP3)
}

/// Plays the deactivation chime.
pub fn play_deactivation() -> Result<()> {
    play_sound_from_bytes(DEACTIVATE_MP3)
}
