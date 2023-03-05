use std::error::Error;

use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup audio
    let mut audio = Audio::new();

    for filename in &["explode", "lose", "move", "pew", "startup", "win"] {
        audio.add(filename, format!("audio/{filename}.wav"));
    }

    audio.play("startup");

    // Cleanup
    audio.wait();
    Ok(())
}
