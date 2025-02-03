use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use eyre::Result;

pub fn play_sound(path: &str) -> Result<()> {
    // Try to get a default output stream.
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    
    // Open the file and decode it.
    let file = File::open(path)?;
    let source = Decoder::new(BufReader::new(file))?;
    
    sink.append(source);
    // Wait until the sound finishes playing.
    sink.sleep_until_end();
    
    Ok(())
}
